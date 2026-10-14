//! Background loop: cursor hit-testing, edge reveal, window/process scan, metrics.
use crate::core::{self, Shared};
use crate::win;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ItemRt {
    pub windows: usize,
    pub procs: usize,
    pub hung: bool,
    pub minimized: bool,
    pub launching: bool,
    pub cpu: f32,
    pub mem: u64,
    pub read: u64,
    pub write: u64,
    pub uptime: u64,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SysRt {
    pub cpu: f32,
    pub mem_used: u64,
    pub mem_total: u64,
}

/// One process of an aggregated application. `start_time` is what makes the
/// identity stable: a PID alone is recycled by Windows within seconds.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SysProc {
    pub pid: u32,
    pub start_time: u64,
}

/// One aggregated application in the system-wide monitor.
#[derive(Serialize, Clone, Default, Debug)]
pub struct SysApp {
    /// normalized full exe path — the frozen identity, never a bare basename
    pub id: String,
    pub name: String,
    /// % of total logical CPU (all cores = 100%)
    pub cpu: f32,
    /// working-set bytes (all processes under this exe summed)
    pub mem: u64,
    /// representative PID (lowest) for display only
    pub pid: u32,
    /// every process aggregated into this row, with its start time
    pub procs: Vec<SysProc>,
    /// validated top-level windows this row could ask to close
    pub windows: usize,
    /// false for Rail's own exe and for protected / shell processes
    pub closable: bool,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuntimePayload {
    pub items: HashMap<String, ItemRt>,
    pub sys: SysRt,
    pub sys_top: Vec<SysApp>,
    /// wall-clock ms of the sample behind `sys` / `sys_top`
    pub ts: u64,
    /// monotonically increasing frame counter
    pub seq: u64,
}

/// Which metric a ranking is ordered by.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Metric {
    Cpu,
    Ram,
}

/// Processes that must never receive a close request from the monitor.
///
/// This is an explicit identity list, not a path-substring heuristic: a
/// `contains("system32")` test would also refuse Notepad, Paint or Calc, which
/// are exactly the disposable apps the close path has to be tested against.
const CRITICAL_EXES: [&str; 11] = [
    "csrss.exe",
    "wininit.exe",
    "winlogon.exe",
    "smss.exe",
    "services.exe",
    "lsass.exe",
    "svchost.exe",
    "dwm.exe",
    "explorer.exe",
    "runtimebroker.exe",
    "sihost.exe",
];

/// True when this executable must not be closed by the mini monitor.
pub fn is_protected_exe(path: &str, own_exe: &str) -> bool {
    let p = core::norm_path(path);
    if p.is_empty() {
        return true;
    }
    if !own_exe.is_empty() && p == core::norm_path(own_exe) {
        return true; // never let the ranking close Rail itself
    }
    CRITICAL_EXES.contains(&core::file_name(&p))
}

/// Deterministic ranking: selected metric first, the other metric as the
/// tie-break, then the full path so equal values never reorder between frames.
pub fn rank_top(mut apps: Vec<SysApp>, metric: Metric, limit: usize) -> Vec<SysApp> {
    apps.sort_by(|a, b| match metric {
        Metric::Cpu => b.cpu.total_cmp(&a.cpu).then_with(|| b.mem.cmp(&a.mem)).then_with(|| a.id.cmp(&b.id)),
        Metric::Ram => b.mem.cmp(&a.mem).then_with(|| b.cpu.total_cmp(&a.cpu)).then_with(|| a.id.cmp(&b.id)),
    });
    apps.truncate(limit);
    apps
}

/// Bounded snapshot that can genuinely answer *both* rankings.
///
/// The old code sorted by RAM and published five rows, so the CPU tab could only
/// ever reorder those five — a CPU-heavy, low-memory application was invisible.
/// Publishing the union of both top-`per_metric` lists (capped) keeps the payload
/// small while making either tab a real system-wide top-N.
pub fn sys_snapshot(apps: Vec<SysApp>, per_metric: usize, cap: usize) -> Vec<SysApp> {
    let mut out: Vec<SysApp> = Vec::new();
    for m in [Metric::Cpu, Metric::Ram] {
        for a in rank_top(apps.clone(), m, per_metric) {
            if !out.iter().any(|x| x.id == a.id) {
                out.push(a);
            }
        }
    }
    out.truncate(cap);
    out
}

/// Per-item exponential smoothing state. Raw `sysinfo` samples swing by tens of
/// percent between ticks (Windows reports cached CPU deltas), which is what made
/// the on-rail numbers visibly twitch.
#[derive(Default, Clone, Copy)]
struct Smooth {
    cpu: f32,
    mem: f64,
    io: f64,
    seeded: bool,
}

/// Low-pass + dead-band. Below `dead` the published value is frozen, so a metric
/// that is merely noisy renders as a stable number instead of flickering digits.
fn smooth(prev: f32, next: f32, alpha: f32, dead: f32) -> f32 {
    let v = prev + (next - prev) * alpha;
    if (v - prev).abs() < dead {
        prev
    } else {
        v
    }
}

pub fn spawn(app: AppHandle, core: Shared) {
    std::thread::spawn(move || {
        win::com_init();
        let mut sys = System::new();
        let mut paths = win::PathCache::default();
        let mut last_scan = Instant::now() - Duration::from_secs(10);
        let mut last_metrics = Instant::now() - Duration::from_secs(10);
        let mut ema_cpu = 0.0f32;
        let mut ema_mem = 0.0f64;
        let mut seeded = false;
        let mut per: HashMap<String, Smooth> = HashMap::new();
        // Last frame that actually carried samples. `scan` leaves every metric
        // field at its `Default` (zero) whenever `metrics` is false, and emitting
        // that straight to the UI is what made RAM/SSD and the sparkline history
        // sawtooth down to zero between real samples.
        let mut last_sys: Option<SysRt> = None;
        let mut last_m: HashMap<String, (usize, f32, u64, u64, u64, u64)> = HashMap::new();
        // Same carry-forward for the system-wide list: an interim windows-only
        // frame used to erase the whole monitor.
        let mut last_top: Vec<SysApp> = Vec::new();
        let mut last_ts: u64 = 0;
        let mut seq: u64 = 0;
        let mut last_err: Option<String> = None;
        loop {
            let (visible, fast, dragging, flyout_open) = {
                let rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
                (rt.visible, rt.fast_until.map(|t| t > Instant::now()).unwrap_or(false), rt.dragging(), rt.flyout_open)
            };
            cursor_tick(&app, &core);

            let scan_every = if fast { 250 } else if visible { 700 } else { 2000 };
            if last_scan.elapsed() >= Duration::from_millis(scan_every) {
                last_scan = Instant::now();
                // 1.4 s between metric frames: one frame per visible digit change.
                // Faster sampling only added jitter, it never added information.
                // While dragging we publish nothing — a re-render mid-gesture
                // reflows the dock and makes the drop index jump.
                // The mini monitor keeps ~1 s cadence while it is actually open,
                // even if the rail itself auto-hid behind it.
                let want_metrics = (visible || flyout_open) && !dragging;
                let metric_every = if flyout_open { 1000 } else { 1400 };
                let with_metrics = want_metrics && last_metrics.elapsed() >= Duration::from_millis(metric_every);
                if with_metrics {
                    last_metrics = Instant::now();
                }
                let mut payload = scan(&core, &mut paths, &mut sys, with_metrics);
                if with_metrics {
                    const A: f32 = 0.25;
                    if !seeded {
                        seeded = true;
                        ema_cpu = payload.sys.cpu;
                        ema_mem = payload.sys.mem_used as f64;
                    }
                    // system CPU: ignore wobble under 0.6 pp
                    ema_cpu = smooth(ema_cpu, payload.sys.cpu, A, 0.6);
                    let m = payload.sys.mem_used as f64;
                    let nm = ema_mem + (m - ema_mem) * A as f64;
                    // system RAM: ignore wobble under 24 MB
                    if (nm - ema_mem).abs() >= 24.0 * 1024.0 * 1024.0 {
                        ema_mem = nm;
                    }
                    payload.sys.cpu = ema_cpu.clamp(0.0, 100.0);
                    payload.sys.mem_used = ema_mem as u64;

                    for (id, r) in payload.items.iter_mut() {
                        let e = per.entry(id.clone()).or_default();
                        if !e.seeded {
                            *e = Smooth { cpu: r.cpu, mem: r.mem as f64, io: (r.read + r.write) as f64, seeded: true };
                        }
                        e.cpu = smooth(e.cpu, r.cpu, A, 0.3);
                        let nm = e.mem + (r.mem as f64 - e.mem) * A as f64;
                        if (nm - e.mem).abs() >= 4.0 * 1024.0 * 1024.0 {
                            e.mem = nm;
                        }
                        e.io = e.io + ((r.read + r.write) as f64 - e.io) * 0.4;
                        r.cpu = e.cpu.clamp(0.0, 100.0);
                        r.mem = e.mem.max(0.0) as u64;
                    }
                    per.retain(|k, _| payload.items.contains_key(k));
                    // Remember the sample so the frames between real metric reads
                    // can repeat it instead of publishing zeros.
                    last_sys = Some(payload.sys.clone());
                    last_m = payload.items.iter().map(|(id, r)| (id.clone(), (r.procs, r.cpu, r.mem, r.read, r.write, r.uptime))).collect();
                    last_top = payload.sys_top.clone();
                    last_ts = payload.ts;
                } else {
                    // Carry-forward: window/process counts stay fresh from `enum_windows`,
                    // only the sampled fields are reused from the last real frame.
                    if let Some(prev) = &last_sys {
                        payload.sys = prev.clone();
                    }
                    for (id, r) in payload.items.iter_mut() {
                        if let Some(&(procs, cpu, mem, read, write, uptime)) = last_m.get(id) {
                            r.procs = procs;
                            r.cpu = cpu;
                            r.mem = mem;
                            r.read = read;
                            r.write = write;
                            r.uptime = uptime;
                        }
                    }
                    // …and the system list survives an interim frame unchanged,
                    // still stamped with the timestamp of the sample it came from.
                    payload.sys_top = last_top.clone();
                    payload.ts = last_ts;
                }
                seq += 1;
                payload.seq = seq;
                // Only the dock and the flyout render runtime data. Broadcasting woke
                // the settings webview on every tick for nothing.
                let _ = app.emit_to("rail", "runtime", payload.clone());
                let _ = app.emit_to("flyout", "runtime", payload);
                // Surface persistence failures once per change (the UI must be
                // able to say "config not saved" instead of silently losing it).
                let err = core.last_save_error();
                if err != last_err {
                    last_err = err.clone();
                    let _ = app.emit("save-error", err.unwrap_or_default());
                }
            }
            std::thread::sleep(Duration::from_millis(if visible { 33 } else { 90 }));
        }
    });
}

fn cursor_tick(app: &AppHandle, core: &Shared) {
    let (cx, cy) = win::cursor();
    let (cx, cy) = (cx as f64, cy as f64);
    let fg = win::foreground();
    let lbtn = win::lbutton_down();
    let (side, edge_reveal, hide_delay) = {
        let c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        (c.settings.side.clone(), c.settings.edge_reveal, c.settings.hide_delay_ms)
    };
    let reveal_ms = 260u64;

    // Snapshot all rt fields we need in one lock acquisition to avoid deadlock.
    // work_for_point() round-trips to the main thread through monitor_from_point(), so it
    // must never be called while core.rt is held.
    let visible = {
        let rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.visible
    };
    let work = if !visible && edge_reveal { Some(work_for_point(app, cx, cy)) } else { None };

    let mut show = false;
    let mut hide_flyout = false;
    let mut hover_changed: Option<bool> = None;
    {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        if fg != 0 && !rt.own.contains(&fg) {
            rt.last_fg = fg;
        }
        if rt.drag_mode && !lbtn {
            rt.drag_mode = false;
        }
        if rt.visible {
            let s = rt.rail_scale;
            let r = rt.hit;
            let lx = (cx - rt.rail_origin.0) / s;
            let ly = (cy - rt.rail_origin.1) / s;
            // Hysteresis: entering needs the real capsule, leaving needs to clear
            // it by 24 px. Without the margin a pointer resting on the capsule
            // edge toggled hover every tick, which restarted the auto-hide timer
            // and cancelled drags.
            let inside = if rt.hover {
                r.w > 0.0 && r.grow(24.0).contains(lx, ly)
            } else {
                r.w > 0.0 && r.contains(lx, ly)
            };
            // dragging files over the rail counts as hovering, no matter where
            // the cursor is — otherwise the panel hides mid-drop
            let inside = inside || rt.dragging();
            if inside != rt.hover {
                rt.hover = inside;
                hover_changed = Some(inside);
            }
            rt.edge_since = None;
        } else if edge_reveal {
            // work area was resolved outside the lock above; if it is missing (the rail
            // flipped to hidden between our read and the lock) just retry next tick
            let Some(w) = work else {
                rt.edge_since = None;
                // the flyout away-timer must still run when the rail is hidden
                let mut hide_now = false;
                if rt.flyout_open {
                    let over_flyout = rt.flyout_rect.map(|fr| fr.contains(cx, cy)).unwrap_or(false);
                    if !over_flyout {
                        let since = *rt.flyout_away.get_or_insert_with(Instant::now);
                        if since.elapsed() >= Duration::from_millis(hide_delay.into()) {
                            rt.flyout_open = false;
                            rt.flyout_rect = None;
                            rt.flyout_away = None;
                            hide_now = true;
                        }
                    } else {
                        rt.flyout_away = None;
                    }
                }
                drop(rt); // release before calling anything that re-locks
                if hide_now {
                    crate::hide_flyout_inner(app, core);
                }
                return;
            };
            // A wider band while the left button is held: dragging a file from
            // Explorer is a coarse gesture and a 2 px strip was practically
            // impossible to hit, which is why "drag a shortcut to the edge"
            // never opened the rail.
            let band = if lbtn { 12.0 } else { 2.0 };
            let at_edge = match side.as_str() {
                "left" => cx <= w.x + band && cy >= w.y && cy <= w.y + w.h,
                "top" => cy <= w.y + band && cx >= w.x && cx <= w.x + w.w,
                "bottom" => cy >= w.y + w.h - band && cy <= w.y + w.h + 1.0 && cx >= w.x && cx <= w.x + w.w,
                _ => cx >= w.x + w.w - band && cy >= w.y && cy <= w.y + w.h,
            };
            // Only the central part of the edge, so screen corners stay free —
            // but dragging gets the full edge (users aim at the panel, not at
            // its vertical centre).
            let span = if lbtn { 0.48 } else { 0.35 };
            let central = match side.as_str() {
                "top" | "bottom" => (cx - (w.x + w.w / 2.0)).abs() <= w.w * span,
                _ => (cy - (w.y + w.h / 2.0)).abs() <= w.h * span,
            };
            if at_edge && central {
                let since = *rt.edge_since.get_or_insert_with(Instant::now);
                let need = if lbtn { 60 } else { reveal_ms };
                if since.elapsed() >= Duration::from_millis(need) {
                    rt.edge_since = None;
                    rt.drag_mode = lbtn;
                    if lbtn {
                        // hold the rail open long enough to finish the drop
                        rt.drag_until = Some(Instant::now() + Duration::from_millis(2500));
                    }
                    show = true;
                }
            } else {
                rt.edge_since = None;
            }
        }
        // Flyout keepalive / away-dismiss: the popup is click-through, so the
        // cursor being over its rect is the only signal that someone is still
        // reading it. Crossing the gap (rail capsule → popup) keeps the timer
        // reset through either zone; leaving both for `hide_delay_ms` dismisses.
        if rt.flyout_open {
            let over_flyout = rt.flyout_rect.map(|fr| fr.contains(cx, cy)).unwrap_or(false);
            if over_flyout || rt.hover {
                rt.flyout_away = None;
            } else {
                let since = *rt.flyout_away.get_or_insert_with(Instant::now);
                if since.elapsed() >= Duration::from_millis(hide_delay.into()) {
                    rt.flyout_open = false;
                    rt.flyout_rect = None;
                    rt.flyout_away = None;
                    hide_flyout = true;
                }
            }
        } else {
            rt.flyout_away = None;
        }
    }
    if let Some(h) = hover_changed {
        let _ = app.emit("rail-hover", h);
    }
    if hide_flyout {
        crate::hide_flyout_inner(app, core);
    }
    if show {
        crate::show_rail(app, core);
    }
}

pub fn work_for_point(app: &AppHandle, x: f64, y: f64) -> crate::core::Rect {
    let m = app.monitor_from_point(x, y).ok().flatten().or_else(|| app.primary_monitor().ok().flatten());
    match m {
        Some(m) => {
            let wa = m.work_area();
            crate::core::Rect { x: wa.position.x as f64, y: wa.position.y as f64, w: wa.size.width as f64, h: wa.size.height as f64 }
        }
        None => crate::core::Rect { x: 0.0, y: 0.0, w: 1920.0, h: 1040.0 },
    }
}

fn scan(core: &Shared, paths: &mut win::PathCache, sys: &mut System, metrics: bool) -> RuntimePayload {
    let items: Vec<(String, String)> = {
        let c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        c.items.iter().filter(|i| !i.target.is_empty()).map(|i| (i.id.clone(), core::norm_path(&i.target))).collect()
    };
    let own: Vec<isize> = core.rt.lock().unwrap_or_else(|e| e.into_inner()).own.clone();
    let wins = win::enum_windows();
    let mut alive = HashSet::new();
    let mut by_path: HashMap<String, Vec<win::AppWindow>> = HashMap::new();
    for w in wins {
        if own.contains(&w.hwnd) {
            continue;
        }
        alive.insert(w.pid);
        if let Some(p) = paths.get(w.pid) {
            by_path.entry(core::norm_path(&p)).or_default().push(w);
        }
    }
    paths.retain(&alive);

    let mut out = RuntimePayload::default();
    let mut item_windows = HashMap::new();
    for (id, target) in &items {
        // Full-path identity only. A basename fallback would let two different
        // installations of `app.exe` share one row, so activate / minimize /
        // close and the metrics would hit whichever process was enumerated
        // first. An item whose executable is not running simply reports zero
        // windows — that is "unavailable", not a guess.
        let list: Vec<win::AppWindow> = by_path.get(target).cloned().unwrap_or_default();
        let rt = ItemRt {
            windows: list.len(),
            hung: list.iter().any(|w| w.hung),
            minimized: !list.is_empty() && list.iter().all(|w| w.minimized),
            ..Default::default()
        };
        item_windows.insert(id.clone(), list.iter().map(|w| w.hwnd).collect::<Vec<_>>());
        out.items.insert(id.clone(), rt);
    }

    if metrics {
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing().with_cpu().with_memory().with_disk_usage().with_exe(UpdateKind::OnlyIfNotSet),
        );
        let ncpu = sys.cpus().len().max(1) as f32;
        out.sys = SysRt { cpu: sys.global_cpu_usage(), mem_used: sys.used_memory(), mem_total: sys.total_memory() };
        let targets: HashMap<&str, &str> = items.iter().map(|(id, t)| (t.as_str(), id.as_str())).collect();
        for p in sys.processes().values() {
            let Some(exe) = p.exe() else { continue };
            let exe = core::norm_path(&exe.to_string_lossy());
            let Some(id) = targets.get(exe.as_str()).copied() else { continue };
            if let Some(r) = out.items.get_mut(id) {
                r.procs += 1;
                r.cpu += p.cpu_usage() / ncpu;
                r.mem += p.memory();
                let d = p.disk_usage();
                r.read += d.read_bytes;
                r.write += d.written_bytes;
                r.uptime = r.uptime.max(p.run_time());
            }
        }

        // ---- system-wide aggregation by full exe path ----
        let own_exe = std::env::current_exe().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
        let mut by_exe: HashMap<String, (String, u32, f32, u64, Vec<SysProc>)> = HashMap::new();
        for p in sys.processes().values() {
            let Some(exe) = p.exe() else { continue };
            let exe = core::norm_path(&exe.to_string_lossy());
            if exe.is_empty() || exe == core::norm_path(&own_exe) {
                continue;
            }
            let name = p.name().to_string_lossy().to_string();
            let pid = p.pid().as_u32();
            let entry = by_exe.entry(exe.clone()).or_insert_with(|| (name, pid, 0.0, 0, Vec::new()));
            entry.2 += p.cpu_usage() / ncpu;
            entry.3 += p.memory();
            entry.4.push(SysProc { pid, start_time: p.start_time() });
            if pid < entry.1 {
                entry.1 = pid;
            }
        }
        let mut all: Vec<SysApp> = by_exe
            .into_iter()
            .map(|(exe, (name, pid, cpu, mem, mut procs))| {
                procs.sort_by_key(|p| p.pid);
                // A row is closable when the exe is not protected *and* it
                // actually owns a validated top-level window. `ends_with(".exe")`
                // used to mark window-less background processes as closable.
                let windows = by_path.get(&exe).map(|v| v.len()).unwrap_or(0);
                let closable = windows > 0 && !is_protected_exe(&exe, &own_exe);
                SysApp { id: exe, name, cpu, mem, pid, procs, windows, closable }
            })
            .collect();
        // Bounded snapshot that still answers both rankings (see sys_snapshot).
        out.sys_top = sys_snapshot(std::mem::take(&mut all), 6, 12);
        out.ts = crate::config::now();
    }

    let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
    let now = Instant::now();
    rt.launching.retain(|id, (deadline, before)| {
        let cur = out.items.get(id).map(|r| r.windows).unwrap_or(0);
        now < *deadline && cur <= *before
    });
    for id in rt.launching.keys() {
        if let Some(r) = out.items.get_mut(id) {
            r.launching = true;
        }
    }
    rt.item_windows = item_windows;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app(id: &str, cpu: f32, mem: u64) -> SysApp {
        SysApp { id: id.to_string(), name: id.to_string(), cpu, mem, pid: 1, procs: vec![], windows: 1, closable: true }
    }

    #[test]
    fn ram_ranking_is_system_wide_not_a_reorder_of_five() {
        let apps = vec![app("a", 1.0, 100), app("b", 2.0, 900), app("c", 3.0, 500), app("d", 4.0, 50), app("e", 5.0, 800), app("f", 6.0, 10)];
        let top = rank_top(apps, Metric::Ram, 5);
        assert_eq!(top.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(), vec!["b", "e", "c", "a", "d"]);
    }

    #[test]
    fn cpu_ranking_includes_a_cpu_heavy_low_ram_app() {
        // the regression the old "sort by RAM, take 5" caused: `d` burns CPU but
        // is 6th by memory, so it never reached the CPU tab at all.
        let apps = vec![app("a", 1.0, 900), app("b", 2.0, 800), app("c", 3.0, 700), app("d", 90.0, 5), app("e", 5.0, 600), app("f", 6.0, 500)];
        let top = rank_top(apps.clone(), Metric::Cpu, 5);
        assert_eq!(top[0].id, "d");
        assert!(top.iter().any(|a| a.id == "d"));
        // and the published snapshot must still carry it for the CPU tab
        let snap = sys_snapshot(apps, 6, 12);
        assert!(snap.iter().any(|a| a.id == "d"), "snapshot must contain the CPU-heavy app");
    }

    #[test]
    fn ties_are_deterministic() {
        let apps = vec![app("z", 5.0, 100), app("a", 5.0, 100), app("m", 5.0, 100)];
        let first = rank_top(apps.clone(), Metric::Cpu, 3);
        let second = rank_top(apps, Metric::Cpu, 3);
        assert_eq!(first.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(), vec!["a", "m", "z"]);
        assert_eq!(first.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(), second.iter().map(|a| a.id.as_str()).collect::<Vec<_>>());
    }

    #[test]
    fn snapshot_is_bounded_and_deduplicated() {
        let apps: Vec<SysApp> = (0..40).map(|i| app(&format!("a{i:02}"), i as f32, (40 - i) as u64 * 10)).collect();
        let snap = sys_snapshot(apps, 6, 12);
        assert!(snap.len() <= 12, "payload must stay bounded, got {}", snap.len());
        let ids: HashSet<&str> = snap.iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids.len(), snap.len(), "no duplicate rows");
    }

    #[test]
    fn protected_policy_is_identity_based_not_a_substring() {
        let own = "C:\\App\\rail.exe";
        assert!(is_protected_exe("C:\\Windows\\System32\\csrss.exe", own));
        assert!(is_protected_exe("c:\\windows\\system32\\svchost.exe", own));
        assert!(is_protected_exe("C:\\Windows\\explorer.exe", own));
        assert!(is_protected_exe("C:\\App\\rail.exe", own), "Rail must never close itself");
        // disposable Windows accessories stay closable — they are what the close
        // path is tested against, and a `contains("system32")` rule would refuse them
        assert!(!is_protected_exe("C:\\Windows\\System32\\notepad.exe", own));
        assert!(!is_protected_exe("C:\\Windows\\System32\\mspaint.exe", own));
        assert!(!is_protected_exe("C:\\Users\\me\\App\\app.exe", own));
        assert!(is_protected_exe("", own), "an unresolved identity is never closable");
    }

    #[test]
    fn same_basename_different_directory_stays_two_rows() {
        let a = app("c:\\one\\app.exe", 1.0, 10);
        let b = app("c:\\two\\app.exe", 1.0, 10);
        let snap = sys_snapshot(vec![a, b], 6, 12);
        assert_eq!(snap.len(), 2, "two directories must not collapse into one identity");
    }
}
