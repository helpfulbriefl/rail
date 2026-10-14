//! Portable config: `rail.config.json` next to the exe (falls back to %APPDATA%\Rail).
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub theme: String,        // classic-light | classic-dark | porcelain | aurora
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

pub fn load() -> Config {
    let p = path();
    let mut cfg: Config = std::fs::read_to_string(&p)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    // v1 shipped with a 5s hold and an oversized rail; migrate those defaults
    // instead of forcing users to re-tune the panel by hand.
    if cfg.version < 2 {
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
    cfg.settings.sanitize();
    cfg.version = 2;
    cfg
}

pub fn save(cfg: &Config) {
    let p = path();
    if let Ok(s) = serde_json::to_string_pretty(cfg) {
        let tmp = p.with_extension("json.tmp");
        if std::fs::write(&tmp, s).is_ok() {
            let _ = std::fs::rename(&tmp, &p);
        }
    }
}

pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
