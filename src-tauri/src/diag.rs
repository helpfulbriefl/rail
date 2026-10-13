//! Phase-0 diagnostics. Off by default; enabled with `RAIL_DIAG=1`.
//!
//! This module exists to answer exactly one question that code reading could not
//! settle: *why does Explorer's drag-and-drop show a "no entry" cursor over the
//! rail?* The candidates are
//!   * H-2 — the window region (SetWindowRgn) excludes the point, so Windows
//!     treats the rail as absent and the desktop underneath refuses the drop;
//!   * H-3 — the region is not cleared in time because the frontend never sends
//!     `dragActive(true)` for an *external* drag;
//!   * H-4 — WebView2's own IDropTarget never receives the drag.
//!
//! Each candidate leaves a different signature in this log, so one manual pass
//! is enough to pick a branch instead of guessing.
//!
//! Nothing here changes behaviour: every call is a side-channel write.

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

static ENABLED: OnceLock<bool> = OnceLock::new();
static START: OnceLock<Instant> = OnceLock::new();
static SINK: Mutex<Option<std::fs::File>> = Mutex::new(None);

/// Extra counters the manual scenario reads back at the end of the run.
static REGION_CALLS: Mutex<u64> = Mutex::new(0);
static REGION_SKIPS: Mutex<u64> = Mutex::new(0);

fn enabled() -> bool {
    *ENABLED.get_or_init(|| match std::env::var("RAIL_DIAG") {
        Ok(v) => v != "0" && !v.is_empty(),
        Err(_) => false,
    })
}

/// True when the caller should bother formatting a line.
pub fn on() -> bool {
    enabled()
}

fn ms() -> u128 {
    START.get_or_init(Instant::now).elapsed().as_millis()
}

/// Append one line to `rail-diag.log` next to the config file.
///
/// The file is opened once and reopened if it disappeared (the user may delete it
/// between runs). Failures are swallowed on purpose: diagnostics must never be
/// able to break the app it is measuring.
pub fn log(args: std::fmt::Arguments) {
    if !enabled() {
        return;
    }
    let mut guard = match SINK.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    if guard.is_none() {
        let p = crate::config::path().with_file_name("rail-diag.log");
        *guard = OpenOptions::new().create(true).append(true).open(p).ok();
    }
    if let Some(f) = guard.as_mut() {
        let _ = writeln!(f, "[{:>8}ms] {}", ms(), args);
        let _ = f.flush();
    }
}

#[macro_export]
macro_rules! diag {
    ($($arg:tt)*) => {
        if $crate::diag::on() {
            $crate::diag::log(format_args!($($arg)*));
        }
    };
}

/// Region bookkeeping — the counters that discriminate H-2 from H-3.
pub fn region_applied(rect: &crate::core::Rect, scale: f64) {
    let n = {
        let mut c = REGION_CALLS.lock().unwrap_or_else(|e| e.into_inner());
        *c += 1;
        *c
    };
    diag!(
        "REGION  set #{n} x={:.1} y={:.1} w={:.1} h={:.1} scale={scale:.2}",
        rect.x,
        rect.y,
        rect.w,
        rect.h
    );
}

pub fn region_skipped(reason: &str) {
    let n = {
        let mut c = REGION_SKIPS.lock().unwrap_or_else(|e| e.into_inner());
        *c += 1;
        *c
    };
    diag!("REGION  skip #{n} reason={reason}");
}

pub fn summary() {
    let set = *REGION_CALLS.lock().unwrap_or_else(|e| e.into_inner());
    let skip = *REGION_SKIPS.lock().unwrap_or_else(|e| e.into_inner());
    diag!("SUMMARY region_set={set} region_skip={skip}");
}
