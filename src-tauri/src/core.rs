//! Shared runtime state between commands, tray and the tracker thread.
use crate::config::{self, Config};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
impl Rect {
    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && py >= self.y && px <= self.x + self.w && py <= self.y + self.h
    }
    /// Same rect within half a pixel — spring animations emit sub-pixel deltas
    /// and re-applying the region for them caused flicker + broke OLE drops.
    pub fn near(&self, o: &Rect) -> bool {
        (self.x - o.x).abs() < 0.5 && (self.y - o.y).abs() < 0.5 && (self.w - o.w).abs() < 0.5 && (self.h - o.h).abs() < 0.5
    }
    pub fn grow(&self, d: f64) -> Rect {
        Rect { x: self.x - d, y: self.y - d, w: self.w + d * 2.0, h: self.h + d * 2.0 }
    }
}

#[derive(Default)]
pub struct Runtime {
    pub visible: bool,
    pub want_visible: bool,
    pub seq: u64,
    /// rail window origin (physical px) + scale at last show
    pub rail_origin: (f64, f64),
    pub rail_scale: f64,
    /// monitor work area (physical) used for the rail
    pub work: Rect,
    /// capsule rect in CSS px, relative to the rail window
    pub hit: Rect,
    /// last region actually pushed to Win32 (physical px) — dedupes SetWindowRgn
    pub rgn: Rect,
    /// true while the window region is cleared (whole window interactive)
    pub rgn_open: bool,
    pub hover: bool,
    pub drag_mode: bool,
    /// a drag (internal tile or external Explorer) is in flight until this time;
    /// while set, the window region stays open and auto-hide is suspended
    pub drag_until: Option<Instant>,
    pub edge_since: Option<Instant>,
    pub last_fg: isize,
    pub own: Vec<isize>,
    /// item id -> z-ordered hwnds
    pub item_windows: HashMap<String, Vec<isize>>,
    /// item id -> (deadline, windows when launched)
    pub launching: HashMap<String, (Instant, usize)>,
    pub flyout_seq: u64,
    pub fast_until: Option<Instant>,
    /// flyout popup is currently shown; drives the fast metrics cadence
    pub flyout_open: bool,
    /// flyout window rect (physical px) — the away-dismiss keepalive zone
    pub flyout_rect: Option<Rect>,
    /// when the cursor left both the rail and the flyout (away-dismiss timer)
    pub flyout_away: Option<Instant>,
}

impl Runtime {
    pub fn dragging(&self) -> bool {
        self.drag_until.map(|t| t > Instant::now()).unwrap_or(false)
    }
}

/// Canonical form of a Windows executable path.
///
/// Windows path comparison is case-insensitive and separator-insensitive, and
/// `QueryFullProcessImageNameW` / `sysinfo` may hand back a `\\?\`-prefixed
/// verbatim path for the same file. Two spellings of one executable must map to
/// one identity; two *different* directories holding `app.exe` must never map to
/// the same identity — that conflation is what made activate / minimize / close
/// and the metrics hit the wrong process.
pub fn norm_path(p: &str) -> String {
    let mut s = p.trim().replace('/', "\\");
    // strip the verbatim / extended-length prefix, it carries no identity;
    // a UNC share keeps its canonical leading `\\server\share` form
    if let Some(rest) = s.strip_prefix("\\\\?\\UNC\\") {
        s = format!("\\\\{rest}");
    } else if let Some(rest) = s.strip_prefix("\\\\?\\") {
        s = rest.to_string();
    }
    while s.len() > 1 && s.ends_with('\\') {
        s.pop();
    }
    s.to_lowercase()
}

/// File name of a path, without any directory component.
pub fn file_name(p: &str) -> &str {
    p.rsplit(['\\', '/']).next().unwrap_or(p)
}

/// The actual disk write. Injected so the ordering guarantees can be tested
/// without touching the Owner's config file.
type Writer = Box<dyn Fn(&Config) -> Result<(), String> + Send + Sync>;

/// Ordered, error-visible persistence pipeline.
///
/// The old design had two independent writers (a 300 ms debounce thread and an
/// unused `save_now`) that both wrote `rail.config.json.tmp` and then renamed
/// it. A shutdown flush could therefore interleave with a background write and
/// replace newer state with an older snapshot, and every failure was swallowed
/// by `let _ =`.
///
/// Now there is exactly one writer path:
///   * `save()` bumps the revision and marks the config dirty;
///   * the debounce thread and `flush()` both go through [`SavePipe::write_latest`],
///     which holds one mutex for the whole sequence, so the fixed temp file name
///     can never be shared by two writers;
///   * the revision is read and the snapshot cloned while the config mutex is
///     held, so a snapshot always matches the revision it is written under;
///   * a revision that is already on disk is never rewritten, and writes are
///     serialized in increasing revision order, so a stale snapshot can never
///     land on top of a newer one;
///   * a failure keeps the dirty flag set (bounded retry on the next tick, no
///     busy loop) and records a short, content-free message for the UI.
struct SavePipe {
    /// revision of the in-memory config; bumped by every `save()`
    rev: AtomicU64,
    /// last revision that reached the disk
    written: AtomicU64,
    dirty: AtomicBool,
    /// serializes writers (debounce thread vs. shutdown flush)
    lock: Mutex<()>,
    /// last failure, content-free
    last_error: Mutex<Option<String>>,
    write: Writer,
}

impl SavePipe {
    fn new(write: Writer) -> Self {
        Self { rev: AtomicU64::new(0), written: AtomicU64::new(0), dirty: AtomicBool::new(false), lock: Mutex::new(()), last_error: Mutex::new(None), write }
    }

    fn lock(&self) -> MutexGuard<'_, ()> {
        self.lock.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Write the newest state to disk. Returns the error message on failure.
    fn write_latest(&self, cfg: &Mutex<Config>) -> Result<(), String> {
        // One writer at a time: the temp file name is fixed, so two concurrent
        // writers would corrupt each other's snapshot.
        let _writer = self.lock();
        // Read the revision and clone the snapshot under the same lock the
        // mutations take, so the two can never disagree.
        let (rev, snapshot) = {
            let guard = cfg.lock().unwrap_or_else(|e| e.into_inner());
            (self.rev.load(Ordering::Acquire), guard.clone())
        };
        if rev <= self.written.load(Ordering::Acquire) {
            return Ok(()); // nothing newer than what is already on disk
        }
        match (self.write)(&snapshot) {
            Ok(()) => {
                // fetch_max, never store: a concurrent flush may have advanced it
                self.written.fetch_max(rev, Ordering::AcqRel);
                // The snapshot that just landed covers every change up to `rev`;
                // only clear dirty when nothing newer arrived while we were writing.
                if self.rev.load(Ordering::Acquire) == rev {
                    self.dirty.store(false, Ordering::Release);
                }
                *self.last_error.lock().unwrap_or_else(|e| e.into_inner()) = None;
                Ok(())
            }
            Err(e) => {
                // Keep the dirty flag: the next debounce tick retries. The loop
                // is time-based, so a permanently failing disk costs one attempt
                // per 300 ms, never a spin.
                self.dirty.store(true, Ordering::Release);
                *self.last_error.lock().unwrap_or_else(|e| e.into_inner()) = Some(e.clone());
                Err(e)
            }
        }
    }
}

pub struct Core {
    pub cfg: Mutex<Config>,
    pub rt: Mutex<Runtime>,
    save: SavePipe,
}

pub type Shared = Arc<Core>;

impl Core {
    pub fn new() -> Shared {
        let core = Arc::new(Core {
            cfg: Mutex::new(config::load()),
            rt: Mutex::new(Runtime { rail_scale: 1.0, ..Default::default() }),
            save: SavePipe::new(Box::new(|c| config::save(c).map_err(|e| e.to_string()))),
        });
        // Debounced save thread: waits 300 ms after the last request before
        // writing to disk. Coalescing happens through the revision counter, so
        // N requests inside the window cost exactly one write of the newest state.
        let weak = Arc::downgrade(&core);
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(300));
                let Some(c) = weak.upgrade() else { break };
                if c.save.dirty.swap(false, Ordering::AcqRel) {
                    if let Err(e) = c.save.write_latest(&c.cfg) {
                        diag!("SAVE    deferred failure: {e}");
                    }
                }
            }
        });
        core
    }

    /// Schedule a debounced save (300 ms). Multiple calls within the window coalesce.
    pub fn save(&self) {
        self.save.rev.fetch_add(1, Ordering::AcqRel);
        self.save.dirty.store(true, Ordering::Release);
    }

    /// Immediate, serialized write of the newest state. Used by the shutdown
    /// flush and by anything that must not lose a change on the next tick.
    pub fn save_now(&self) -> Result<(), String> {
        self.save.write_latest(&self.cfg)
    }

    /// Shutdown flush: write whatever is still dirty, then report.
    /// Never removes or rewrites a config that is already current.
    pub fn flush(&self) -> Result<(), String> {
        self.save.write_latest(&self.cfg)
    }

    /// True when a change has not reached the disk yet.
    pub fn dirty(&self) -> bool {
        self.save.dirty.load(Ordering::Acquire)
    }

    /// Last persistence failure, content-free (no paths, no item names).
    pub fn last_save_error(&self) -> Option<String> {
        self.save.last_error.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    fn pipe(write: Writer) -> SavePipe {
        SavePipe::new(write)
    }

    #[test]
    fn norm_path_folds_case_separators_and_verbatim_prefix() {
        assert_eq!(norm_path("C:\\Program Files\\App\\app.exe"), norm_path("c:/program files/app/APP.EXE"));
        assert_eq!(norm_path("\\\\?\\C:\\Tools\\app.exe"), norm_path("c:\\tools\\app.exe"));
        assert_eq!(norm_path("\\\\?\\UNC\\srv\\share\\app.exe"), norm_path("\\\\srv\\share\\app.exe"));
        assert_eq!(norm_path("C:\\Tools\\app.exe\\"), norm_path("c:\\tools\\app.exe"));
    }

    #[test]
    fn norm_path_keeps_distinct_directories_distinct() {
        // the whole point: same basename, different folder => different identity
        assert_ne!(norm_path("C:\\One\\app.exe"), norm_path("C:\\Two\\app.exe"));
        assert_ne!(norm_path("C:\\One\\app.exe"), norm_path("C:\\One\\other.exe"));
    }

    #[test]
    fn file_name_splits_on_both_separators() {
        assert_eq!(file_name("C:\\a\\b\\app.exe"), "app.exe");
        assert_eq!(file_name("C:/a/b/app.exe"), "app.exe");
        assert_eq!(file_name("app.exe"), "app.exe");
    }

    #[test]
    fn revision_guard_skips_a_stale_snapshot() {
        let p = pipe(Box::new(|_| Ok(())));
        let cfg = Mutex::new(Config::default());
        // nothing was ever requested
        assert!(p.write_latest(&cfg).is_ok());
        assert_eq!(p.written.load(Ordering::Acquire), 0);
        // request rev 1, write it
        p.rev.fetch_add(1, Ordering::AcqRel);
        p.dirty.store(true, Ordering::Release);
        assert!(p.write_latest(&cfg).is_ok());
        assert_eq!(p.written.load(Ordering::Acquire), 1);
        // a second call with no new request must not rewrite
        assert!(p.write_latest(&cfg).is_ok());
        assert_eq!(p.written.load(Ordering::Acquire), 1);
    }

    #[test]
    fn failed_write_stays_dirty_and_records_a_content_free_error() {
        let p = pipe(Box::new(|_| Err("replace failed: access denied".into())));
        let cfg = Mutex::new(Config::default());
        p.rev.fetch_add(1, Ordering::AcqRel);
        p.dirty.store(true, Ordering::Release);
        let err = p.write_latest(&cfg).expect_err("writer fails");
        assert!(err.contains("access denied"));
        assert!(p.dirty.load(Ordering::Acquire), "a failed save must stay dirty for the bounded retry");
        assert_eq!(p.last_error.lock().unwrap().as_deref(), Some("replace failed: access denied"));
        assert_eq!(p.written.load(Ordering::Acquire), 0, "a failed write must not be marked as written");
    }

    #[test]
    fn concurrent_scheduled_save_and_flush_never_write_a_stale_snapshot() {
        // Every mutation stamps the config with its own revision; the writer
        // records what it saw. Under the old design a shutdown flush could
        // interleave with the debounce thread and land an older snapshot last.
        let (tx, rx) = mpsc::channel::<u64>();
        let p = Arc::new(pipe(Box::new(move |c| {
            // a slow write widens the race window on purpose
            std::thread::sleep(std::time::Duration::from_millis(2));
            tx.send(c.settings.hide_delay_ms as u64).map_err(|_| "closed".to_string())
        })));
        let cfg = Arc::new(Mutex::new(Config::default()));
        let base = cfg.lock().unwrap().settings.hide_delay_ms as u64;
        let mut hs = vec![];
        for _ in 0..4 {
            let (p, cfg) = (Arc::clone(&p), Arc::clone(&cfg));
            hs.push(std::thread::spawn(move || {
                for _ in 0..40 {
                    {
                        let mut c = cfg.lock().unwrap();
                        c.settings.hide_delay_ms += 1;
                    }
                    p.rev.fetch_add(1, Ordering::AcqRel);
                    p.dirty.store(true, Ordering::Release);
                    let _ = p.write_latest(&cfg);
                }
            }));
        }
        for h in hs {
            h.join().unwrap();
        }
        // final flush must persist the newest state
        p.write_latest(&cfg).unwrap();
        let seen: Vec<u64> = rx.try_iter().collect();
        assert!(!seen.is_empty(), "the writer must have been called");
        let newest = base + 160;
        assert_eq!(*seen.last().unwrap(), newest, "the last write must carry the newest revision");
        assert!(seen.windows(2).all(|w| w[0] <= w[1]), "writes must be serialized in increasing revision order, got {seen:?}");
        assert_eq!(cfg.lock().unwrap().settings.hide_delay_ms as u64, newest);
    }

    #[test]
    fn change_then_immediate_flush_persists_it() {
        let (tx, rx) = mpsc::channel::<u64>();
        let p = Arc::new(pipe(Box::new(move |c| tx.send(c.settings.hide_delay_ms as u64).map_err(|_| "closed".to_string()))));
        let cfg = Arc::new(Mutex::new(Config::default()));
        {
            let mut c = cfg.lock().unwrap();
            c.settings.theme = "studio".into();
            c.settings.hide_delay_ms = 2400;
        }
        p.rev.fetch_add(1, Ordering::AcqRel);
        p.dirty.store(true, Ordering::Release);
        // no debounce tick in between — this is the "quit immediately" path
        p.write_latest(&cfg).unwrap();
        assert_eq!(rx.try_recv().unwrap(), 2400);
        assert!(!p.dirty.load(Ordering::Acquire));
    }
}
