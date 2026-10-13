//! Shared runtime state between commands, tray and the tracker thread.
use crate::config::{self, Config};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
}

impl Runtime {
    pub fn dragging(&self) -> bool {
        self.drag_until.map(|t| t > Instant::now()).unwrap_or(false)
    }
}

pub struct Core {
    pub cfg: Mutex<Config>,
    pub rt: Mutex<Runtime>,
    save_pending: std::sync::atomic::AtomicBool,
}

pub type Shared = Arc<Core>;

impl Core {
    pub fn new() -> Shared {
        let core = Arc::new(Core {
            cfg: Mutex::new(config::load()),
            rt: Mutex::new(Runtime { rail_scale: 1.0, ..Default::default() }),
            save_pending: std::sync::atomic::AtomicBool::new(false),
        });
        // Debounced save thread: waits 300ms after last request before writing to disk
        let weak = Arc::downgrade(&core);
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(300));
                let Some(c) = weak.upgrade() else { break };
                if c.save_pending.swap(false, std::sync::atomic::Ordering::AcqRel) {
                    let cfg = c.cfg.lock().unwrap_or_else(|e| e.into_inner()).clone();
                    config::save(&cfg);
                }
            }
        });
        core
    }
    /// Schedule a debounced save (300ms). Multiple calls within the window coalesce.
    pub fn save(&self) {
        self.save_pending.store(true, std::sync::atomic::Ordering::Release);
    }
    /// Immediate save — used only at shutdown.
    #[allow(dead_code)]
    pub fn save_now(&self) {
        let c = self.cfg.lock().unwrap_or_else(|e| e.into_inner()).clone();
        config::save(&c);
    }
}
