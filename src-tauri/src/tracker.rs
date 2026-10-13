//! Background loop: cursor hit-testing, edge reveal, window/process scan, metrics.
use crate::core::Shared;
use crate::win;
use serde::Serialize;
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

#[derive(Serialize, Clone, Default)]
pub struct RuntimePayload {
    pub items: HashMap<String, ItemRt>,
    pub sys: SysRt,
}

fn file_name(p: &str) -> &str {
    p.rsplit(['\\', '/']).next().unwrap_or(p)
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
        loop {
            let (visible, fast, dragging) = {
                let rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
                (rt.visible, rt.fast_until.map(|t| t > Instant::now()).unwrap_or(false), rt.dragging())
            };
            cursor_tick(&app, &core);

            let scan_every = if fast { 250 } else if visible { 700 } else { 2000 };
            if last_scan.elapsed() >= Duration::from_millis(scan_every) {
                last_scan = Instant::now();
                // 1.4 s between metric frames: one frame per visible digit change.
                // Faster sampling only added jitter, it never added information.
                // While dragging we publish nothing — a re-render mid-gesture
                // reflows the dock and makes the drop index jump.
                let with_metrics = visible && !dragging && last_metrics.elapsed() >= Duration::from_millis(1400);
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
                } else if let Some(prev) = &last_sys {
                    // Carry-forward: window/process counts stay fresh from `enum_windows`,
                    // only the sampled fields are reused from the last real frame.
                    payload.sys = prev.clone();
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
                }
                // Only the dock and the flyout render runtime data. Broadcasting woke
                // the settings webview on every tick for nothing.
                let _ = app.emit_to("rail", "runtime", payload.clone());
                let _ = app.emit_to("flyout", "runtime", payload);
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
    let (side, edge_reveal, reveal_ms) = {
        let c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        (c.settings.side.clone(), c.settings.edge_reveal, 260u64)
    };

    // Snapshot all rt fields we need in one lock acquisition to avoid deadlock.
    // work_for_point() round-trips to the main thread through monitor_from_point(), so it
    // must never be called while core.rt is held.
    let visible = {
        let rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.visible
    };
    let work = if !visible && edge_reveal { Some(work_for_point(app, cx, cy)) } else { None };

    let mut show = false;
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
    }
    if let Some(h) = hover_changed {
        let _ = app.emit("rail-hover", h);
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
        c.items.iter().filter(|i| !i.target.is_empty()).map(|i| (i.id.clone(), i.target.clone())).collect()
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
            by_path.entry(p).or_default().push(w);
        }
    }
    paths.retain(&alive);

    let mut out = RuntimePayload::default();
    let mut item_windows = HashMap::new();
    for (id, target) in &items {
        let list: Vec<win::AppWindow> = match by_path.get(target) {
            Some(v) => v.clone(),
            None => {
                let fname = file_name(target);
                by_path.iter().filter(|(p, _)| file_name(p) == fname).flat_map(|(_, v)| v.clone()).collect()
            }
        };
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
        let names: HashMap<&str, &str> = items.iter().map(|(id, t)| (file_name(t), id.as_str())).collect();
        for p in sys.processes().values() {
            let Some(exe) = p.exe() else { continue };
            let exe = exe.to_string_lossy().to_lowercase();
            let id = match targets.get(exe.as_str()).or_else(|| names.get(file_name(&exe))) {
                Some(id) => *id,
                None => continue,
            };
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
