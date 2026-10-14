//! Portable config: `rail.config.json` next to the exe (falls back to %APPDATA%\Rail).
//!
//! Persistence rules enforced here (W3B-FIX addendum A):
//!   * every step returns a `Result` — a silent `let _ =` on the write or the
//!     replace is what used to lose settings without any visible symptom;
//!   * the destination is replaced with `std::fs::rename`, which on Windows maps
//!     to `MoveFileExW(..., MOVEFILE_REPLACE_EXISTING)`. The old file is never
//!     deleted first, so a failed save always leaves the previous valid config
//!     intact;
//!   * the temporary file lives in the *same directory* as the destination (a
//!     cross-volume rename is not atomic and would fail with ERROR_NOT_SAME_DEVICE)
//!     and is removed on every failure path;
//!   * no BOM is ever written, and a stray BOM on read is tolerated.
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub theme: String,        // classic-light | classic-dark | porcelain | aurora | glass | studio
    pub side: String,         // right | left | bottom | top
    pub autostart: bool,
    pub edge_reveal: bool,
    pub hover_delay_ms: u32,  // hold time before info / metrics flyout
    pub hide_delay_ms: u32,   // auto-hide after mouse leaves
    pub icon_size: u32,       // 30 | 36 | 44
    pub pinned: bool,
    pub reduce_motion: bool,
    pub rail_scale: String,   // "mini" | "compact" | "normal"
    /// show the CPU/RAM footer at all
    pub show_meters: bool,
}

/// Upper bound for the flyout hold: 3s was requested as the *maximum*, so the
/// slider range and every migrated config are clamped to it.
pub const HOVER_MAX_MS: u32 = 3000;

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "glass".into(),
            side: "right".into(),
            autostart: false,
            edge_reveal: true,
            hover_delay_ms: 900,
            hide_delay_ms: 1200,
            icon_size: 36,
            pinned: false,
            reduce_motion: false,
            rail_scale: "compact".into(),
            show_meters: true,
        }
    }
}

impl Settings {
    /// Clamp everything that came from disk into the ranges the UI can express.
    pub fn sanitize(&mut self) {
        if !matches!(self.theme.as_str(), "classic-light" | "classic-dark" | "porcelain" | "aurora" | "glass" | "studio") {
            self.theme = "glass".into();
        }
        self.hover_delay_ms = self.hover_delay_ms.clamp(300, HOVER_MAX_MS);
        self.hide_delay_ms = self.hide_delay_ms.clamp(300, 5000);
        if !matches!(self.icon_size, 30 | 36 | 44) {
            // old builds stored 52 ("L"); map anything unknown to the nearest step
            self.icon_size = if self.icon_size >= 41 { 44 } else if self.icon_size >= 34 { 36 } else { 30 };
        }
        if !matches!(self.rail_scale.as_str(), "mini" | "compact" | "normal") {
            self.rail_scale = "compact".into();
        }
        if !matches!(self.side.as_str(), "left" | "right" | "top" | "bottom") {
            self.side = "right".into();
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Item {
    pub id: String,
    pub name: String,
    /// What we execute (original .lnk / .exe / file / folder).
    pub launch_path: String,
    /// Resolved exe used for window/process matching (lower-case).
    pub target: String,
    pub args: String,
    pub workdir: String,
    pub kind: String, // app | file | folder
    pub icon: String, // data:image/png;base64,...
    pub added_at: u64,
    pub last_launch: u64,
    pub launch_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Config {
    pub version: u32,
    pub settings: Settings,
    pub items: Vec<Item>,
}

fn candidates() -> Vec<PathBuf> {
    let mut v = vec![];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            v.push(dir.join("rail.config.json"));
        }
    }
    if let Ok(app) = std::env::var("APPDATA") {
        v.push(PathBuf::from(app).join("Rail").join("rail.config.json"));
    }
    v
}

pub fn path() -> PathBuf {
    let c = candidates();
    for p in &c {
        if p.exists() {
            return p.clone();
        }
    }
    // first writable location wins
    for p in &c {
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
            let probe = dir.join(".rail-write-test");
            if std::fs::write(&probe, b"1").is_ok() {
                let _ = std::fs::remove_file(probe);
                return p.clone();
            }
        }
    }
    c.into_iter().next().unwrap_or_else(|| PathBuf::from("rail.config.json"))
}

// ---------------------------------------------------------------- persistence

/// Why a save failed. Deliberately free of any config content: the message is
/// shown to the Owner and written to the diagnostic log, neither of which may
/// receive paths, names or anything else read from the config file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveError {
    Serialize(String),
    Write(String),
    Replace(String),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::Serialize(e) => write!(f, "serialize failed: {e}"),
            SaveError::Write(e) => write!(f, "temp write failed: {e}"),
            SaveError::Replace(e) => write!(f, "replace failed: {e}"),
        }
    }
}

impl std::error::Error for SaveError {}

/// `.rail.config.json.tmp` next to the destination — same directory, so the
/// rename can never cross a volume boundary.
fn tmp_path(dest: &Path) -> PathBuf {
    let name = dest.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "rail.config.json".to_string());
    dest.with_file_name(format!(".{name}.tmp"))
}

/// Best-effort directory flush. Opening a directory handle is not portable, so
/// every failure here is ignored: it only shortens the crash window, it is not
/// required for correctness of the replace itself.
fn sync_dir(dir: Option<&Path>) {
    let Some(dir) = dir else { return };
    if let Ok(f) = File::open(dir) {
        let _ = f.sync_all();
    }
}

/// Atomically replace `dest` with `cfg`.
///
/// The previous file is only ever *replaced*, never unlinked first: if any step
/// fails the caller still has the last good config on disk.
pub fn save_to(cfg: &Config, dest: &Path) -> Result<(), SaveError> {
    let text = serde_json::to_string_pretty(cfg).map_err(|e| SaveError::Serialize(e.to_string()))?;
    let tmp = tmp_path(dest);
    {
        let mut f = OpenOptions::new().write(true).create(true).truncate(true).open(&tmp).map_err(|e| SaveError::Write(e.to_string()))?;
        f.write_all(text.as_bytes()).map_err(|e| SaveError::Write(e.to_string()))?;
        f.sync_all().map_err(|e| SaveError::Write(e.to_string()))?;
    }
    // Windows: MoveFileExW with MOVEFILE_REPLACE_EXISTING — an existing
    // destination is replaced, it does not have to be absent.
    if let Err(e) = std::fs::rename(&tmp, dest) {
        let _ = std::fs::remove_file(&tmp);
        return Err(SaveError::Replace(e.to_string()));
    }
    sync_dir(dest.parent());
    Ok(())
}

/// Save to the resolved config location.
pub fn save(cfg: &Config) -> Result<(), SaveError> {
    save_to(cfg, &path())
}

// ---------------------------------------------------------------- loading

/// v1 shipped with a 5s hold and an oversized rail; migrate those defaults
/// instead of forcing users to re-tune the panel by hand.
fn migrate(cfg: &mut Config) {
    if cfg.version >= 2 {
        return;
    }
    if cfg.settings.hover_delay_ms >= 4000 {
        cfg.settings.hover_delay_ms = 900;
    }
    if cfg.settings.rail_scale.is_empty() || cfg.settings.rail_scale == "normal" {
        cfg.settings.rail_scale = "compact".into();
    }
    if cfg.settings.icon_size >= 52 {
        cfg.settings.icon_size = 44;
    }
}

/// Pure half of [`load`]: parse + migrate + sanitize. No filesystem access, so
/// it is unit-testable without touching the Owner's config or the registry.
pub fn parse(text: &str) -> Config {
    let mut cfg: Config = serde_json::from_str(text.trim_start_matches('\u{feff}')).unwrap_or_default();
    migrate(&mut cfg);
    cfg.settings.sanitize();
    cfg.version = 2;
    cfg
}

pub fn load() -> Config {
    let p = path();
    let text = std::fs::read_to_string(&p).unwrap_or_default();
    parse(&text)
}

pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("rail-cfg-test-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn sample() -> Config {
        let mut c = Config::default();
        c.settings.theme = "studio".into();
        c.settings.side = "left".into();
        c.settings.icon_size = 44;
        c.items = vec![
            Item { id: "a".into(), name: "A".into(), launch_path: "C:\\A\\a.exe".into(), target: "c:\\a\\a.exe".into(), kind: "app".into(), ..Default::default() },
            Item { id: "b".into(), name: "B".into(), launch_path: "C:\\B\\b.exe".into(), target: "c:\\b\\b.exe".into(), kind: "app".into(), ..Default::default() },
        ];
        c
    }

    #[test]
    fn initial_save_creates_valid_utf8_without_bom() {
        let d = tmpdir("initial");
        let dest = d.join("rail.config.json");
        save_to(&sample(), &dest).expect("first save");
        let raw = std::fs::read(&dest).unwrap();
        assert!(!raw.starts_with(&[0xEF, 0xBB, 0xBF]), "config must not start with a BOM");
        let text = String::from_utf8(raw).expect("valid UTF-8");
        assert!(text.contains("\"theme\": \"studio\""));
        // no temp file left behind
        assert!(!d.join(".rail.config.json.tmp").exists());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn repeated_replacement_of_existing_file_keeps_working() {
        let d = tmpdir("replace");
        let dest = d.join("rail.config.json");
        save_to(&sample(), &dest).expect("first save");
        for i in 0..5 {
            let mut c = sample();
            c.settings.hide_delay_ms = 1000 + i;
            save_to(&c, &dest).unwrap_or_else(|e| panic!("save #{i}: {e}"));
        }
        let back = parse(&std::fs::read_to_string(&dest).unwrap());
        assert_eq!(back.settings.hide_delay_ms, 1004);
        assert_eq!(back.items.len(), 2);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn failed_replace_preserves_the_previous_valid_config() {
        let d = tmpdir("fail");
        let dest = d.join("rail.config.json");
        save_to(&sample(), &dest).expect("first save");
        let good = std::fs::read_to_string(&dest).unwrap();
        // A directory in place of the destination makes the rename fail on
        // Windows (ERROR_ACCESS_DENIED / ERROR_ALREADY_EXISTS) — a real,
        // reproducible failure that needs no permission games.
        std::fs::create_dir(d.join("rail.config.json.dir")).unwrap();
        let blocked = d.join("rail.config.json.dir");
        let err = save_to(&sample(), &blocked).expect_err("rename onto a directory must fail");
        assert!(matches!(err, SaveError::Replace(_)), "got {err}");
        // the good config is untouched and no temp file is left next to it
        assert_eq!(std::fs::read_to_string(&dest).unwrap(), good);
        assert!(!d.join(".rail.config.json.tmp").exists());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn settings_and_order_round_trip() {
        let d = tmpdir("roundtrip");
        let dest = d.join("rail.config.json");
        let c = sample();
        save_to(&c, &dest).unwrap();
        let back = parse(&std::fs::read_to_string(&dest).unwrap());
        assert_eq!(back.settings.theme, "studio");
        assert_eq!(back.settings.side, "left");
        assert_eq!(back.settings.icon_size, 44);
        assert_eq!(back.items.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(), vec!["a", "b"]);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn sanitize_rejects_unknown_theme_and_clamps_delays() {
        let mut s = Settings { theme: "neon".into(), hover_delay_ms: 99_999, hide_delay_ms: 1, icon_size: 52, rail_scale: "huge".into(), side: "nowhere".into(), ..Default::default() };
        s.sanitize();
        assert_eq!(s.theme, "glass");
        assert_eq!(s.hover_delay_ms, HOVER_MAX_MS);
        assert_eq!(s.hide_delay_ms, 300);
        assert_eq!(s.icon_size, 44);
        assert_eq!(s.rail_scale, "compact");
        assert_eq!(s.side, "right");
    }

    #[test]
    fn every_approved_theme_survives_sanitize() {
        for t in ["classic-light", "classic-dark", "porcelain", "aurora", "glass", "studio"] {
            let mut s = Settings { theme: t.into(), ..Default::default() };
            s.sanitize();
            assert_eq!(s.theme, t, "explicitly saved theme must be preserved");
        }
    }

    #[test]
    fn v1_migration_maps_oversized_defaults() {
        let c = parse(r#"{"version":1,"settings":{"hoverDelayMs":5000,"railScale":"normal","iconSize":52},"items":[]}"#);
        assert_eq!(c.version, 2);
        assert_eq!(c.settings.hover_delay_ms, 900);
        assert_eq!(c.settings.rail_scale, "compact");
        assert_eq!(c.settings.icon_size, 44);
    }

    #[test]
    fn v2_config_is_not_remigrated() {
        let c = parse(r#"{"version":2,"settings":{"hoverDelayMs":2500,"railScale":"normal","iconSize":44},"items":[]}"#);
        assert_eq!(c.settings.hover_delay_ms, 2500);
        assert_eq!(c.settings.rail_scale, "normal");
    }

    #[test]
    fn bom_prefixed_json_still_loads() {
        let c = parse("\u{feff}{\"version\":2,\"settings\":{\"theme\":\"aurora\"},\"items\":[]}");
        assert_eq!(c.settings.theme, "aurora");
    }

    #[test]
    fn broken_json_falls_back_to_defaults() {
        let c = parse("{ not json");
        assert_eq!(c.settings.theme, "glass");
        assert!(c.items.is_empty());
    }
}
