#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
mod diag;
mod config;
mod core;
mod tracker;
mod win;

use crate::config::{now, Config, Item, Settings};
use crate::core::{Rect, Shared};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use tauri::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, State, WebviewWindow};
use tauri_plugin_autostart::ManagerExt;

/// Cross-axis size of the rail window (CSS px).
///
/// The old build hard-coded 168/109 px for *any* icon size, so an "S" rail still
/// reserved a 168 px strip: the capsule looked huge and the transparent margin
/// swallowed clicks. Now it is derived from the actual content:
///   tile + chrome (head row, separators, meters) + drop shadow margin.
fn rail_cross(s: &Settings, vertical: bool) -> f64 {
    let (mul, pad, chrome, meters) = match s.rail_scale.as_str() {
        "mini" => (0.56, 10.0, 26.0, 0.0),
        "compact" => (0.72, 14.0, 34.0, 26.0),
        _ => (1.0, 20.0, 44.0, 34.0),
    };
    let s_px = (s.icon_size as f64 * mul).round();
    // `tileBox` grows the slot when an app is running: `+grow` across the main
    // axis, `+round(grow*0.45)` across the cross axis.
    let grow = (s_px * 0.28).round();
    if vertical {
        // Column layout: head / dock / foot stack along the *main* axis, so the
        // cross axis only has to fit the widest single child plus padding. The
        // old code summed every block here too — that is the double-count that
        // produced a 114 px window for a 41 px capsule.
        let tile = s_px + (grow * 0.45).round();
        let micro = match s.rail_scale.as_str() {
            "mini" => 16.0,
            "compact" => 19.0,
            _ => 24.0,
        };
        // `.rail-foot` is a fixed-width column of meter rows
        let foot = if s.show_meters { chrome } else { 0.0 };
        tile.max(micro).max(foot).max(20.0) + pad
    } else {
        // Row layout: head, dock and meters *do* stack across the cross axis.
        let tile = s_px + grow + 14.0;
        let feet = if s.show_meters { meters } else { 0.0 };
        (tile + pad + chrome + feet).max(68.0)
    }
}

/// Extra strip kept inside the window beyond the capsule so the magnified /
/// dragged tile is not clipped by the window region.
///
/// Derived from the icon size instead of a flat 26 px: at `compact` (S = 26) a
/// 26 px pad on *each* side was wider than the capsule itself, which is most of
/// why the window measured 114 px for a rail that only renders 41 px wide.
fn drag_pad(s: &Settings) -> f64 {
    let mul = match s.rail_scale.as_str() {
        "mini" => 0.56,
        "compact" => 0.72,
        _ => 1.0,
    };
    let s_px = (s.icon_size as f64 * mul).round();
    // magnification grow + the 8% `whileDrag` lift + the capsule shadow
    (s_px * 0.35).round().clamp(8.0, 18.0)
}

/// How long one `dragActive(true)` keeps the window region open.
///
/// The frontend refreshes this lease on a 600 ms interval, which used to leave a
/// window of only 1.2 s: a single throttled tick (a background WebView2 tab is
/// clamped to ~1 Hz) let the lease expire *mid-drag*, `apply_region` re-installed
/// the region, and Explorer's `IDropTarget` hit-test — which runs against the
/// live region — stopped finding the rail. The cursor turned into a "no entry"
/// sign and the drop was refused.
const DRAG_LEASE_MS: u64 = 5000;

struct LaunchQueue(mpsc::Sender<(String, bool)>);

// ---------------------------------------------------------------- geometry

fn rail_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window("rail")
}

fn hwnd_of(w: &WebviewWindow) -> isize {
    w.hwnd().map(|h| h.0 as isize).unwrap_or(0)
}

/// Single place that talks to `SetWindowRgn`.
///
/// Three bugs lived here before:
///   1. it was called on every spring frame (sub-pixel deltas) which re-entered
///      the OLE drag loop and cancelled Explorer drops;
///   2. it clipped the window to the bare capsule, so a tile lifted by
///      `whileDrag` was cut off and the pointer left the window mid-gesture;
///   3. during a drag the region must be gone entirely.
fn apply_region(app: &AppHandle, core: &Shared, force: bool) {
    let Some(w) = rail_window(app) else { return };
    let (want, open_now, scale, hit, drag) = {
        let rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        (rt.hit, rt.rgn_open, rt.rail_scale, rt.rgn, rt.dragging())
    };
    let h = hwnd_of(&w);
    diag!(
        "APPLY   force={force} drag={drag} open={open_now} hit=({:.1},{:.1},{:.1},{:.1})",
        want.x,
        want.y,
        want.w,
        want.h
    );
    if drag {
        if open_now && !force {
            diag::region_skipped("already_open");
            return;
        }
        diag!("REGION  clear (drag in flight)");
        {
            let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
            rt.rgn_open = true;
            rt.rgn = Rect::default();
        }
        let _ = app.run_on_main_thread(move || win::clear_click_region(h));
        return;
    }
    if want.w <= 0.0 || want.h <= 0.0 {
        diag::region_skipped("empty_hit_rect");
        return;
    }
    // breathing room for magnified / dragged tiles and the capsule shadow
    let target = want.grow(drag_pad(&core.cfg.lock().unwrap_or_else(|e| e.into_inner()).settings));
    if !force && !open_now && target.near(&hit) {
        diag::region_skipped("deduped");
        return;
    }
    diag::region_applied(&target, scale);
    {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.rgn = target;
        rt.rgn_open = false;
    }
    let _ = app.run_on_main_thread(move || {
        win::set_click_region(h, target.x * scale, target.y * scale, target.w * scale, target.h * scale);
    });
}

pub fn show_rail(app: &AppHandle, core: &Shared) {
    let Some(w) = rail_window(app) else { return };
    let side = core.cfg.lock().unwrap_or_else(|e| e.into_inner()).settings.side.clone();
    let (cx, cy) = win::cursor();
    let work = tracker::work_for_point(app, cx as f64, cy as f64);
    let scale = app
        .monitor_from_point(cx as f64, cy as f64)
        .ok()
        .flatten()
        .map(|m| m.scale_factor())
        .unwrap_or(1.0);
    let vertical = side == "left" || side == "right";
    let cross = (rail_cross(&core.cfg.lock().unwrap_or_else(|e| e.into_inner()).settings, vertical) * scale).round();
    // Only reserve as much of the edge as the capsule can actually use. The old
    // build always took 94% of the screen, so the invisible window covered the
    // whole side and ate clicks far away from the visible rail.
    let (n, s_icon, s_scale, s_met) = {
        let c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        (c.items.len().max(1) as f64, c.settings.icon_size as f64, c.settings.rail_scale.clone(), c.settings.show_meters)
    };
    let mul = match s_scale.as_str() {
        "mini" => 0.56,
        "compact" => 0.72,
        _ => 1.0,
    };
    let step = (s_icon * mul).round() + 8.0;
    let head = 3.0 * (26.0 * mul + 4.0) + 60.0; // buttons + separators
    let feet = if s_met { 56.0 * mul } else { 0.0 };
    let need = (n * step + head + feet + drag_pad(&core.cfg.lock().unwrap_or_else(|e| e.into_inner()).settings) * 2.0 + 40.0) * scale;
    let avail = (if vertical { work.h } else { work.w }) * 0.96;
    let along = need.min(avail).max(220.0 * scale).round();
    let (x, y, ww, hh) = match side.as_str() {
        "left" => (work.x, work.y + (work.h - along) / 2.0, cross, along),
        "top" => (work.x + (work.w - along) / 2.0, work.y, along, cross),
        "bottom" => (work.x + (work.w - along) / 2.0, work.y + work.h - cross, along, cross),
        _ => (work.x + work.w - cross, work.y + (work.h - along) / 2.0, cross, along),
    };
    let _ = w.set_size(PhysicalSize::new(ww as u32, hh as u32));
    let _ = w.set_position(PhysicalPosition::new(x as i32, y as i32));
    let seq = {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.seq += 1;
        rt.want_visible = true;
        rt.visible = true;
        rt.rail_origin = (x, y);
        rt.rail_scale = scale;
        rt.work = work;
        rt.hover = false;
        rt.rgn = Rect::default(); // resize invalidates the old region
        rt.seq
    };
    let _ = w.show();
    let _ = w.set_always_on_top(true);
    apply_region(app, core, true);
    let _ = app.emit_to("rail", "rail", serde_json::json!({"action": "show", "seq": seq}));
}

fn request_hide_inner(app: &AppHandle, core: &Shared) {
    let seq = {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        // never yank the rail out from under an in-flight drag
        if !rt.visible || rt.dragging() {
            return;
        }
        rt.want_visible = false;
        rt.seq
    };
    let _ = app.emit_to("rail", "rail", serde_json::json!({"action": "hide", "seq": seq}));
    hide_flyout_inner(app, core);
}

fn toggle_rail(app: &AppHandle, core: &Shared) {
    let visible = { core.rt.lock().unwrap_or_else(|e| e.into_inner()).want_visible };
    if visible {
        request_hide_inner(app, core);
    } else {
        show_rail(app, core);
    }
}

pub(crate) fn hide_flyout_inner(app: &AppHandle, core: &Shared) {
    let seq = {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.flyout_open = false;
        rt.flyout_rect = None;
        rt.flyout_away = None;
        rt.flyout_seq
    };
    let _ = app.emit_to("flyout", "flyout", serde_json::json!({"action": "hide", "seq": seq}));
}

fn emit_config(app: &AppHandle, core: &Shared) {
    let c = core.cfg.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let _ = app.emit("config", c);
}

// ---------------------------------------------------------------- items

fn stem(p: &str) -> String {
    let f = p.rsplit(['\\', '/']).next().unwrap_or(p);
    let s = match f.rfind('.') {
        Some(i) if i > 0 => &f[..i],
        _ => f,
    };
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => s.to_string(),
    }
}

fn make_item(path: &str) -> Item {
    let lower = path.to_lowercase();
    let meta = std::fs::metadata(path).ok();
    let is_dir = meta.map(|m| m.is_dir()).unwrap_or(false);
    let mut it = Item {
        id: format!("i{:x}{:x}", now(), rand_u32()),
        name: stem(path),
        launch_path: path.to_string(),
        kind: "file".into(),
        added_at: now(),
        ..Default::default()
    };
    let mut icon_src = path.to_string();
    if is_dir {
        it.kind = "folder".into();
        it.name = path.trim_end_matches(['\\', '/']).rsplit(['\\', '/']).next().unwrap_or(path).to_string();
    } else if lower.ends_with(".lnk") {
        it.kind = "app".into();
        if let Some(l) = win::resolve_lnk(path) {
            if l.target.to_lowercase().ends_with(".exe") {
                it.target = l.target.to_lowercase();
                icon_src = l.target.clone();
            }
            // Squirrel-style launchers (Discord, Slack, Teams…): Update.exe --processStart App.exe
            let a = l.args.to_lowercase();
            if it.target.ends_with("\\update.exe") {
                if let Some(pos) = a.find("--processstart") {
                    let rest = a[pos + 14..].trim().trim_matches('"');
                    let name = rest.split(['"', ' ']).next().unwrap_or("");
                    if name.ends_with(".exe") {
                        let dir = it.target.rsplit_once('\\').map(|x| x.0).unwrap_or("");
                        it.target = format!("{dir}\\{name}");
                    }
                }
            }
            it.args = l.args;
            it.workdir = l.workdir;
        }
    } else if lower.ends_with(".exe") {
        it.kind = "app".into();
        it.target = lower.clone();
    }
    it.icon = win::icon_data_url(&icon_src, 96)
        .or_else(|| win::icon_data_url(path, 96))
        .unwrap_or_default();
    it
}

fn rand_u32() -> u32 {
    use std::hash::{BuildHasher, Hasher};
    let mut h = std::collections::hash_map::RandomState::new().build_hasher();
    h.write_u64(now());
    h.finish() as u32
}

fn do_launch(app: &AppHandle, core: &Shared, id: &str) {
    let item = {
        let c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        c.items.iter().find(|i| i.id == id).cloned()
    };
    let Some(item) = item else { return };
    {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        let before = rt.item_windows.get(id).map(|v| v.len()).unwrap_or(0);
        if item.kind == "app" {
            rt.launching.insert(id.to_string(), (Instant::now() + Duration::from_secs(10), before));
        }
        rt.fast_until = Some(Instant::now() + Duration::from_secs(5));
    }
    let res = win::shell_open(&item.launch_path, if item.launch_path.to_lowercase().ends_with(".lnk") { "" } else { &item.args }, &item.workdir);
    let ok = res.is_ok();
    if !ok {
        core.rt.lock().unwrap_or_else(|e| e.into_inner()).launching.remove(id);
    } else {
        let mut c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(i) = c.items.iter_mut().find(|i| i.id == id) {
            i.last_launch = now();
            i.launch_count += 1;
        }
    }
    core.save();
    emit_config(app, core);
    let _ = app.emit("launch", serde_json::json!({"id": id, "ok": ok, "error": res.err()}));
}

// ---------------------------------------------------------------- commands

#[derive(Serialize)]
struct Snapshot {
    config: Config,
    visible: bool,
}

/// Frontend diagnostics sink. The webview has no console in a release build, so
/// `onDragDropEvent` traffic is forwarded here and lands in the same file as the
/// Rust-side region/lease lines — the two halves have to be read together to tell
/// "the drag never arrived" (H-4) from "it arrived but the region was closed"
/// (H-2/H-3).
#[tauri::command]
fn diag_log(msg: String) {
    if diag::on() {
        diag::log(format_args!("UI      {msg}"));
    }
}

#[tauri::command]
fn snapshot(core: State<Shared>) -> Snapshot {
    Snapshot { config: core.cfg.lock().unwrap_or_else(|e| e.into_inner()).clone(), visible: core.rt.lock().unwrap_or_else(|e| e.into_inner()).visible }
}

/// Paths that would really be inserted.
///
/// De-duplicated within the drop (Explorer hands the same file twice when it is
/// both selected and dragged) *and* against what is already on the rail, using
/// the same full-path normalization the tracker matches processes with.
///
/// The old code computed `added` from "did the drop contain any new path", then
/// skipped the already-present ones inside the insert loop — so dropping a file
/// that was already pinned still saved the config, re-emitted `config` and
/// re-fitted the window for nothing. Duplicate-only input now yields an empty
/// plan and is a complete no-op.
pub fn plan_inserts(items: &[Item], paths: &[String]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    for p in paths {
        let k = crate::core::norm_path(p);
        if k.is_empty() || seen.contains(&k) {
            continue;
        }
        seen.push(k.clone());
        if items.iter().any(|x| crate::core::norm_path(&x.launch_path) == k) {
            continue;
        }
        out.push(p.clone());
    }
    out
}

#[tauri::command]
fn add_paths(app: AppHandle, core: State<Shared>, paths: Vec<String>, index: Option<usize>) {
    diag!("DROPPED paths={} index={index:?} first={:?}", paths.len(), paths.first());
    // A drop always ends the drag, whatever the frontend thinks.
    {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.drag_until = None;
    }
    let existing: Vec<String> = paths.iter().filter(|p| std::path::Path::new(p.as_str()).exists()).cloned().collect();
    let plan = {
        let c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        plan_inserts(&c.items, &existing)
    };
    // Only a real insertion is allowed to touch the config, the window or the
    // region — a duplicate drop must be indistinguishable from a no-op.
    if plan.is_empty() {
        diag!("DROPPED nothing new ({} duplicate/already-present)", paths.len());
        return;
    }
    let new: Vec<Item> = plan.iter().map(|p| make_item(p)).collect();
    {
        let mut c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        let mut at = index.unwrap_or(c.items.len()).min(c.items.len());
        for it in new {
            c.items.insert(at, it);
            at += 1;
        }
    }
    core.save();
    emit_config(&app, &core);
    // the list just grew along the main axis — re-fit the window
    if core.rt.lock().unwrap_or_else(|e| e.into_inner()).visible {
        show_rail(&app, &core);
    }
    apply_region(&app, core.inner(), true);
}

/// Open a native file picker and add selected files/shortcuts to the rail.
/// Uses the callback API so the dialog runs on an OS-appropriate thread
/// (blocking_pick_files in an async/tokio context can fail on COM STA).
#[tauri::command]
fn pick_files(app: AppHandle, core: State<Shared>) {
    use tauri_plugin_dialog::DialogExt;
    let core = core.inner().clone();
    app.dialog()
        .file()
        .add_filter("Приложения и ярлыки", &["exe", "lnk"])
        .add_filter("Все файлы", &["*"])
        .pick_files(move |picked| {
            let Some(files) = picked else { return };
            let paths: Vec<String> = files.iter().filter_map(|f| f.as_path().map(|p| p.to_string_lossy().to_string())).collect();
            let plan = {
                let c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
                plan_inserts(&c.items, &paths)
            };
            if plan.is_empty() {
                return;
            }
            let new: Vec<Item> = plan.iter().map(|p| make_item(p)).collect();
            {
                let mut c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
                let mut at = c.items.len();
                for it in new {
                    c.items.insert(at, it);
                    at += 1;
                }
            }
            core.save();
            let _ = app.emit("config", core.cfg.lock().unwrap_or_else(|e| e.into_inner()).clone());
        });
}

#[tauri::command]
fn reorder(app: AppHandle, core: State<Shared>, ids: Vec<String>) {
    {
        let mut c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        let mut items = std::mem::take(&mut c.items);
        let mut out = Vec::with_capacity(items.len());
        for id in &ids {
            if let Some(p) = items.iter().position(|i| &i.id == id) {
                out.push(items.remove(p));
            }
        }
        out.extend(items);
        c.items = out;
    }
    core.save();
    emit_config(&app, &core);
}

#[tauri::command]
fn remove_item(app: AppHandle, core: State<Shared>, id: String) {
    core.cfg.lock().unwrap_or_else(|e| e.into_inner()).items.retain(|i| i.id != id);
    core.save();
    emit_config(&app, &core);
}

#[tauri::command]
fn launch(q: State<LaunchQueue>, id: String) {
    let _ = q.0.send((id, true));
}

/// Taskbar-like click: focus → minimize → cycle windows; launch if nothing is open.
#[tauri::command]
fn activate(q: State<LaunchQueue>, core: State<Shared>, id: String) {
    let (wins, last_fg) = {
        let rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        (rt.item_windows.get(&id).cloned().unwrap_or_default(), rt.last_fg)
    };
    if wins.is_empty() {
        let _ = q.0.send((id, true));
        return;
    }
    match wins.iter().position(|&h| h == last_fg) {
        Some(_) if wins.len() == 1 => win::minimize(wins[0]),
        Some(p) => win::activate(wins[(p + 1) % wins.len()]),
        None => win::activate(wins[0]),
    }
}

#[tauri::command]
fn close_windows(core: State<Shared>, id: String) {
    let wins = core.rt.lock().unwrap_or_else(|e| e.into_inner()).item_windows.get(&id).cloned().unwrap_or_default();
    for h in wins {
        win::close(h);
    }
}

/// What the UI asks to close: the frozen row identity, not a bare exe string.
///
/// `id` is the normalized full exe path and `procs` is the exact process set the
/// row was built from. Both are re-validated against the live process table
/// before anything is sent, so a stale row can never redirect the request at a
/// newly created, unrelated instance that inherited a recycled PID.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SysCloseReq {
    id: String,
    procs: Vec<tracker::SysProc>,
}

/// Honest outcome of a close request.
///
/// `dispatched` counts WM_CLOSE messages that were *posted* — it deliberately
/// does not claim the application exited: an unsaved-work dialog, a hung app or
/// an access-denied target all leave the process alive, and the UI must be able
/// to say so.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SysCloseResult {
    /// "dispatched" | "refused" | "unavailable"
    status: &'static str,
    /// windows we posted WM_CLOSE to
    dispatched: usize,
    /// validated top-level windows that existed for this identity
    windows: usize,
    /// short, content-free explanation for the Owner
    reason: String,
}

/// Safe system-app close.
///
/// Sends WM_CLOSE — never TerminateProcess — to the validated top-level windows
/// of one frozen application identity, so every target keeps its own
/// unsaved-work prompt. Refuses Rail's own exe and protected / shell processes
/// by explicit identity, and refuses anything it cannot revalidate.
#[tauri::command]
fn sys_close(core: State<Shared>, req: SysCloseReq) -> SysCloseResult {
    let id = crate::core::norm_path(&req.id);
    let own_exe = std::env::current_exe().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
    let own: Vec<isize> = core.rt.lock().unwrap_or_else(|e| e.into_inner()).own.clone();

    if id.is_empty() {
        return SysCloseResult { status: "unavailable", dispatched: 0, windows: 0, reason: "нет идентификатора приложения".into() };
    }
    if tracker::is_protected_exe(&id, &own_exe) {
        return SysCloseResult { status: "refused", dispatched: 0, windows: 0, reason: "системный или защищённый процесс".into() };
    }
    if req.procs.is_empty() {
        return SysCloseResult { status: "unavailable", dispatched: 0, windows: 0, reason: "нет данных о процессе".into() };
    }

    // Revalidate every PID the UI handed us: same executable *and* same start
    // time. A recycled PID or a restarted app fails here and is dropped.
    let mut valid: Vec<u32> = Vec::new();
    for p in &req.procs {
        let live = win::process_path(p.pid).map(|x| crate::core::norm_path(&x));
        let started = win::process_start_time(p.pid);
        if live.as_deref() == Some(id.as_str()) && started == Some(p.start_time) {
            valid.push(p.pid);
        }
    }
    if valid.is_empty() {
        return SysCloseResult { status: "unavailable", dispatched: 0, windows: 0, reason: "приложение закрыто или перезапущено".into() };
    }

    let wins: Vec<win::AppWindow> = win::enum_windows().into_iter().filter(|w| valid.contains(&w.pid) && !own.contains(&w.hwnd)).collect();
    if wins.is_empty() {
        return SysCloseResult { status: "unavailable", dispatched: 0, windows: 0, reason: "нет окна, которое можно закрыть".into() };
    }
    let mut dispatched = 0;
    for w in &wins {
        win::close(w.hwnd);
        dispatched += 1;
    }
    diag!("SYSCLOSE id={id} pids={} dispatched={dispatched}", valid.len());
    SysCloseResult { status: "dispatched", dispatched, windows: wins.len(), reason: "отправлен запрос на закрытие".into() }
}

#[tauri::command]
fn reveal(core: State<Shared>, id: String) {
    let p = core.cfg.lock().unwrap_or_else(|e| e.into_inner()).items.iter().find(|i| i.id == id).map(|i| i.launch_path.clone());
    if let Some(p) = p {
        win::reveal_in_explorer(&p);
    }
}

#[tauri::command]
fn item_menu(app: AppHandle, core: State<Shared>, id: String) -> Result<(), String> {
    let running = core.rt.lock().unwrap_or_else(|e| e.into_inner()).item_windows.get(&id).map(|v| !v.is_empty()).unwrap_or(false);
    let kind = core.cfg.lock().unwrap_or_else(|e| e.into_inner()).items.iter().find(|i| i.id == id).map(|i| i.kind.clone()).unwrap_or_default();
    let e = |s: &str| s.to_string();
    let m = Menu::new(&app).map_err(|x| x.to_string())?;
    let new = MenuItem::with_id(&app, format!("ctx:new:{id}"), if kind == "app" { e("Новое окно") } else { e("Открыть") }, true, None::<&str>).map_err(|x| x.to_string())?;
    let close = MenuItem::with_id(&app, format!("ctx:close:{id}"), e("Закрыть все окна"), running, None::<&str>).map_err(|x| x.to_string())?;
    let rev = MenuItem::with_id(&app, format!("ctx:reveal:{id}"), e("Расположение файла"), true, None::<&str>).map_err(|x| x.to_string())?;
    let sep = PredefinedMenuItem::separator(&app).map_err(|x| x.to_string())?;
    let rm = MenuItem::with_id(&app, format!("ctx:remove:{id}"), e("Убрать с панели"), true, None::<&str>).map_err(|x| x.to_string())?;
    m.append_items(&[&new, &close, &rev, &sep, &rm]).map_err(|x| x.to_string())?;
    if let Some(w) = rail_window(&app) {
        w.popup_menu(&m).map_err(|x| x.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_settings(app: AppHandle, core: State<Shared>, mut settings: Settings) {
    settings.sanitize();
    let (geom_changed, autostart) = {
        let mut c = core.cfg.lock().unwrap_or_else(|e| e.into_inner());
        // icon size / scale / meters all change the window's cross-axis size, so
        // the rail has to be re-laid out, not just re-rendered.
        let geom_changed = c.settings.side != settings.side
            || c.settings.rail_scale != settings.rail_scale
            || c.settings.icon_size != settings.icon_size
            || c.settings.show_meters != settings.show_meters;
        let a = settings.autostart;
        c.settings = settings;
        (geom_changed, a)
    };
    let al = app.autolaunch();
    let _ = if autostart { al.enable() } else { al.disable() };
    core.save();
    emit_config(&app, &core);
    if geom_changed && core.rt.lock().unwrap_or_else(|e| e.into_inner()).visible {
        show_rail(&app, &core);
    }
    rebuild_tray_menu(&app, &core);
}

#[derive(Deserialize)]
struct HitRect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[tauri::command]
fn set_hit_rect(app: AppHandle, core: State<Shared>, r: HitRect) {
    diag!("HITRECT x={} y={} w={} h={}", r.x, r.y, r.w, r.h);
    {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.hit = Rect { x: r.x, y: r.y, w: r.w, h: r.h };
    }
    // Selective click-through: the window region is the capsule rect (plus a
    // drag margin), so clicks and Explorer drops outside it fall through while
    // the capsule stays fully interactive (no WS_EX_TRANSPARENT — that would
    // kill OLE drag-and-drop). apply_region dedupes and marshals to the UI
    // thread, which SetWindowRgn requires.
    apply_region(&app, core.inner(), false);
}

/// The UI tells us a drag started / is alive / ended.
///
/// `active` keeps a 1.2 s lease that the frontend refreshes while dragging, so a
/// crashed or unmounted webview can never leave the rail pinned open.
#[tauri::command]
fn drag_active(app: AppHandle, core: State<Shared>, active: bool) {
    let changed = {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        let was = rt.dragging();
        rt.drag_until = if active { Some(Instant::now() + Duration::from_millis(DRAG_LEASE_MS)) } else { None };
        was != rt.dragging()
    };
    diag!("LEASE   drag_active({active}) changed={changed}");
    if changed {
        apply_region(&app, core.inner(), true);
    }
}

#[tauri::command]
fn rail_ready(app: AppHandle, core: State<Shared>) {
    let (want, seq) = {
        let rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        (rt.want_visible, rt.seq)
    };
    let action = if want { "show" } else { "hide" };
    let _ = app.emit_to("rail", "rail", serde_json::json!({"action": action, "seq": seq}));
}

#[tauri::command]
fn request_hide(app: AppHandle, core: State<Shared>) {
    request_hide_inner(&app, &core);
}

#[tauri::command]
fn rail_hidden(app: AppHandle, core: State<Shared>, seq: u64) {
    let ok = {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        if !rt.want_visible && rt.seq == seq && !rt.dragging() {
            rt.visible = false;
            rt.hover = false;
            rt.drag_until = None;
            rt.rgn = Rect::default();
            true
        } else {
            false
        }
    };
    if ok {
        if let Some(w) = rail_window(&app) {
            let _ = w.hide();
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FlyoutReq {
    kind: String,
    id: String,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    width: f64,
    height: f64,
}

/// Places the flyout popup, clamping **size and position** so the whole card
/// always fits inside the work area.
///
/// The old code only clamped position and could panic in release builds when the
/// requested size exceeded the work area (`clamp` asserts `min <= max`); on a
/// small monitor the popup was also simply cut off. When the range collapses
/// (popup larger than the available span) `hi` is pulled up to `lo`, so the
/// popup is at least fully on-screen and never larger than the work area.
pub fn place_flyout(
    side: &str,
    work: Rect,
    ax: f64,
    ay: f64,
    aw: f64,
    ah: f64,
    fw: f64,
    fh: f64,
    gap: f64,
    margin: f64,
) -> (f64, f64, f64, f64) {
    let max_w = (work.w - 2.0 * margin).max(64.0);
    let max_h = (work.h - 2.0 * margin).max(64.0);
    let (fw, fh) = (fw.min(max_w), fh.min(max_h));
    let (mut x, mut y) = match side {
        "left" => (ax + aw + gap, ay + ah / 2.0 - fh / 2.0),
        "top" => (ax + aw / 2.0 - fw / 2.0, ay + ah + gap),
        "bottom" => (ax + aw / 2.0 - fw / 2.0, ay - gap - fh),
        _ => (ax - gap - fw, ay + ah / 2.0 - fh / 2.0),
    };
    let lo_x = work.x + margin;
    let hi_x = (work.x + work.w - margin - fw).max(lo_x);
    let lo_y = work.y + margin;
    let hi_y = (work.y + work.h - margin - fh).max(lo_y);
    x = x.clamp(lo_x, hi_x);
    y = y.clamp(lo_y, hi_y);
    (x, y, fw, fh)
}

#[tauri::command]
fn show_flyout(app: AppHandle, core: State<Shared>, req: FlyoutReq) {
    let Some(f) = app.get_webview_window("flyout") else { return };
    let side = core.cfg.lock().unwrap_or_else(|e| e.into_inner()).settings.side.clone();
    let (seq, ox, oy, s, work) = {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.flyout_seq += 1;
        rt.flyout_open = true;
        (rt.flyout_seq, rt.rail_origin.0, rt.rail_origin.1, rt.rail_scale, rt.work)
    };
    let (ax, ay, aw, ah) = (ox + req.x * s, oy + req.y * s, req.w * s, req.h * s);
    let (x, y, fw, fh) = place_flyout(&side, work, ax, ay, aw, ah, req.width * s, req.height * s, 14.0 * s, 4.0 * s);
    // the popup's own rect becomes part of the "pointer still engaged" zone, so
    // crossing the gap and resting on the monitor does not dismiss it
    {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        rt.flyout_rect = Some(Rect { x, y, w: fw, h: fh });
        rt.flyout_away = None;
    }
    let _ = f.set_size(PhysicalSize::new(fw as u32, fh as u32));
    let _ = f.set_position(PhysicalPosition::new(x as i32, y as i32));
    let kind = req.kind.clone();
    let _ = app.emit_to("flyout", "flyout", serde_json::json!({"action": "show", "seq": seq, "kind": req.kind, "id": req.id, "side": side}));
    let _ = f.show();
    let _ = f.set_always_on_top(true);
    // The item popup carries the real buttons ("Открыть", "Показать в
    // проводнике", the X) and must receive clicks; only the system monitor
    // stays click-through. The old build made *every* flyout transparent, so
    // nothing inside it was clickable.
    win::set_click_through(hwnd_of(&f), kind == "sys");
}

#[tauri::command]
fn hide_flyout(app: AppHandle, core: State<Shared>) {
    hide_flyout_inner(&app, &core);
}

#[tauri::command]
fn flyout_hidden(app: AppHandle, core: State<Shared>, seq: u64) {
    let same = {
        let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
        let same = rt.flyout_seq == seq;
        if same {
            rt.flyout_open = false;
        }
        same
    };
    if same {
        if let Some(f) = app.get_webview_window("flyout") {
            let _ = f.hide();
        }
    }
}

#[tauri::command]
fn open_settings(app: AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.center();
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

#[tauri::command]
fn close_settings(app: AppHandle) {
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.hide();
    }
}

/// Single shutdown path for every exit route (command, tray, RunEvent).
/// Flushes the newest config state before the process goes away.
fn shutdown(app: &AppHandle, core: &Shared) {
    // last line of the diagnostic run: how often the region was actually
    // (re)installed vs. skipped, which is the H-2/H-3 discriminator
    diag::summary();
    if let Err(e) = core.flush() {
        diag!("SAVE    shutdown flush failed: {e}");
    }
    app.exit(0);
}

#[tauri::command]
fn quit(app: AppHandle, core: State<Shared>) {
    shutdown(&app, &core);
}

// ---------------------------------------------------------------- tray

fn build_tray_menu(app: &AppHandle, core: &Shared) -> tauri::Result<Menu<tauri::Wry>> {
    let s = core.cfg.lock().unwrap_or_else(|e| e.into_inner()).settings.clone();
    let show = MenuItem::with_id(app, "tray:show", "Показать панель", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "tray:hide", "Скрыть панель", true, None::<&str>)?;
    let pin = CheckMenuItem::with_id(app, "tray:pin", "Закрепить панель", true, s.pinned, None::<&str>)?;
    let themes = [
        ("classic-light", "Классика · светлая"),
        ("classic-dark", "Классика · тёмная"),
        ("porcelain", "Фарфор"),
        ("aurora", "Аврора"),
        ("glass", "Стекло"),
        ("studio", "Студия"),
    ];
    let theme_items: Vec<CheckMenuItem<tauri::Wry>> = themes
        .iter()
        .map(|(id, name)| CheckMenuItem::with_id(app, format!("tray:theme:{id}"), *name, true, s.theme == *id, None::<&str>))
        .collect::<Result<_, _>>()?;
    let refs: Vec<&dyn tauri::menu::IsMenuItem<tauri::Wry>> = theme_items.iter().map(|i| i as _).collect();
    let theme = Submenu::with_items(app, "Тема", true, &refs)?;
    let sides = [("right", "Справа"), ("left", "Слева"), ("bottom", "Снизу"), ("top", "Сверху")];
    let side_items: Vec<CheckMenuItem<tauri::Wry>> = sides
        .iter()
        .map(|(id, name)| CheckMenuItem::with_id(app, format!("tray:side:{id}"), *name, true, s.side == *id, None::<&str>))
        .collect::<Result<_, _>>()?;
    let srefs: Vec<&dyn tauri::menu::IsMenuItem<tauri::Wry>> = side_items.iter().map(|i| i as _).collect();
    let side = Submenu::with_items(app, "Положение", true, &srefs)?;
    let auto = CheckMenuItem::with_id(app, "tray:autostart", "Запуск с Windows", true, s.autostart, None::<&str>)?;
    let settings = MenuItem::with_id(app, "tray:settings", "Настройки…", true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "tray:quit", "Выход", true, None::<&str>)?;
    Menu::with_items(app, &[&show, &hide, &pin, &sep, &theme, &side, &auto, &settings, &sep2, &quit])
}

fn rebuild_tray_menu(app: &AppHandle, core: &Shared) {
    if let (Some(t), Ok(m)) = (app.tray_by_id("main"), build_tray_menu(app, core)) {
        let _ = t.set_menu(Some(m));
    }
}

fn patch_settings(app: &AppHandle, core: &Shared, f: impl FnOnce(&mut Settings)) {
    let mut s = core.cfg.lock().unwrap_or_else(|e| e.into_inner()).settings.clone();
    f(&mut s);
    set_settings(app.clone(), app.state::<Shared>(), s);
    let _ = core;
}

fn on_menu(app: &AppHandle, ev: MenuEvent) {
    let core: Shared = app.state::<Shared>().inner().clone();
    let id = ev.id().0.as_str();
    if let Some(rest) = id.strip_prefix("ctx:") {
        let (act, item) = rest.split_once(':').unwrap_or((rest, ""));
        let item = item.to_string();
        match act {
            "new" => {
                let _ = app.state::<LaunchQueue>().0.send((item, true));
            }
            "close" => close_windows(app.state::<Shared>(), item),
            "reveal" => reveal(app.state::<Shared>(), item),
            "remove" => remove_item(app.clone(), app.state::<Shared>(), item),
            _ => {}
        }
        let _ = app.emit_to("rail", "menu-closed", ());
        return;
    }
    match id {
        "tray:show" => show_rail(app, &core),
        "tray:hide" => request_hide_inner(app, &core),
        "tray:pin" => patch_settings(app, &core, |s| s.pinned = !s.pinned),
        "tray:autostart" => patch_settings(app, &core, |s| s.autostart = !s.autostart),
        "tray:settings" => open_settings(app.clone()),
        "tray:quit" => shutdown(app, &core),
        x if x.starts_with("tray:theme:") => {
            let t = x["tray:theme:".len()..].to_string();
            patch_settings(app, &core, |s| s.theme = t)
        }
        x if x.starts_with("tray:side:") => {
            let t = x["tray:side:".len()..].to_string();
            patch_settings(app, &core, |s| s.side = t)
        }
        _ => {}
    }
}

// ---------------------------------------------------------------- main

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let p = config::path().with_file_name("rail.log");
        let _ = std::fs::write(p, format!("{info}"));
    }));
    let core = core::Core::new();
    let (tx, rx) = mpsc::channel::<(String, bool)>();
    let core_exit = core.clone();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let core = app.state::<Shared>().inner().clone();
            show_rail(app, &core);
        }))
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_dialog::init())
        .manage(core.clone())
        .manage(LaunchQueue(tx))
        .invoke_handler(tauri::generate_handler![
            snapshot, add_paths, pick_files, reorder, remove_item, launch, activate, close_windows, sys_close, reveal, item_menu,
            diag_log,
            set_settings, set_hit_rect, drag_active, rail_ready, request_hide, rail_hidden, show_flyout, hide_flyout,
            flyout_hidden, open_settings, close_settings, quit
        ])
        .on_menu_event(on_menu)
        .setup(move |app| {
            let handle = app.handle().clone();
            // own windows are excluded from tracking / last-foreground
            {
                let mut rt = core.rt.lock().unwrap_or_else(|e| e.into_inner());
                for l in ["rail", "flyout", "settings"] {
                    if let Some(w) = app.get_webview_window(l) {
                        rt.own.push(hwnd_of(&w));
                    }
                }
            }
            if let Some(w) = app.get_webview_window("settings") {
                let h = handle.clone();
                w.on_window_event(move |e| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = e {
                        api.prevent_close();
                        if let Some(w) = h.get_webview_window("settings") {
                            let _ = w.hide();
                        }
                    }
                });
            }

            // sequential launch queue (one ShellExecute at a time, off the UI thread)
            {
                let h = handle.clone();
                let c = core.clone();
                std::thread::spawn(move || {
                    win::com_init();
                    while let Ok((id, _)) = rx.recv() {
                        do_launch(&h, &c, &id);
                    }
                });
            }

            let menu = build_tray_menu(&handle, &core)?;
            let c2 = core.clone();
            TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().cloned().expect("icon"))
                .tooltip("Rail")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(move |tray, ev| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = ev {
                        toggle_rail(tray.app_handle(), &c2);
                    }
                })
                .build(app)?;

            // keep autostart registry in sync with the portable config
            let want = core.cfg.lock().unwrap_or_else(|e| e.into_inner()).settings.autostart;
            let al = handle.autolaunch();
            if al.is_enabled().unwrap_or(false) != want {
                let _ = if want { al.enable() } else { al.disable() };
            }

            tracker::spawn(handle.clone(), core.clone());
            if core.cfg.lock().unwrap_or_else(|e| e.into_inner()).items.is_empty() {
                show_rail(&handle, &core); // first run: show the empty rail with the drop hint
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running Rail");

    // Safety net for every exit route (taskbar kill, RunEvent, exit(0) from a
    // command is handled by `shutdown` itself): the newest state must reach the
    // disk even if the quit command never ran.
    app.run(move |_app, ev| {
        if let tauri::RunEvent::ExitRequested { .. } = ev {
            let _ = core_exit.flush();
        }
    });
}

// ---------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(scale: &str) -> Settings {
        Settings {
            rail_scale: scale.into(),
            icon_size: 48,
            ..Default::default()
        }
    }

    fn item(path: &str) -> Item {
        Item {
            id: format!("i{}", path),
            name: path.to_string(),
            launch_path: path.to_string(),
            kind: "app".into(),
            ..Default::default()
        }
    }

    #[test]
    fn rail_cross_is_derived_from_scale_not_hard_coded() {
        let s = settings("mini");
        let v = rail_cross(&s, true);
        let h = rail_cross(&s, false);
        assert!(v > 0.0 && h > 0.0);
        assert!(v.is_finite() && h.is_finite());
        // "mini" must not reserve the full default strip
        let m = settings("compact");
        let d = settings("default");
        assert!(rail_cross(&s, true) < rail_cross(&m, true));
        assert!(rail_cross(&m, true) < rail_cross(&d, true));
    }

    #[test]
    fn drag_pad_grows_with_scale() {
        assert!(drag_pad(&settings("mini")) < drag_pad(&settings("compact")));
        assert!(drag_pad(&settings("compact")) < drag_pad(&settings("default")));
    }

    #[test]
    fn flyout_fits_inside_the_work_area_on_every_side() {
        let work = Rect { x: 0.0, y: 0.0, w: 1920.0, h: 1080.0 };
        for side in ["left", "right", "top", "bottom"] {
            // anchor near the outer screen edge, popup wants to hang off-screen
            let (x, y, w, h) = place_flyout(side, work, 1900.0, 1070.0, 40.0, 40.0, 420.0, 300.0, 14.0, 4.0);
            assert!(x >= work.x, "{side}: x {x}");
            assert!(y >= work.y, "{side}: y {y}");
            assert!(x + w <= work.x + work.w, "{side}: right {x}+{w}");
            assert!(y + h <= work.y + work.h, "{side}: bottom {y}+{h}");
        }
    }

    #[test]
    fn oversized_flyout_is_clamped_in_size_and_never_panics() {
        // work area smaller than the requested popup: old code hit
        // `clamp(lo, hi)` with hi < lo (panic in debug / garbage in release)
        let work = Rect { x: 0.0, y: 0.0, w: 300.0, h: 200.0 };
        let (x, y, w, h) = place_flyout("right", work, 280.0, 180.0, 20.0, 20.0, 800.0, 600.0, 14.0, 4.0);
        assert!(w <= work.w);
        assert!(h <= work.h);
        assert!(x >= work.x && x + w <= work.x + work.w + 0.001);
        assert!(y >= work.y && y + h <= work.y + work.h + 0.001);
    }

    #[test]
    fn duplicate_only_drop_changes_nothing() {
        let items = vec![item(r"C:\Apps\a.exe")];
        let plan = plan_inserts(&items, &[r"c:\apps\A.EXE".into()]);
        assert!(plan.is_empty());
    }

    #[test]
    fn mixed_drop_only_plans_the_new_paths() {
        let items = vec![item(r"C:\Apps\a.exe")];
        let plan = plan_inserts(&items, &[r"C:\Apps\a.exe".into(), r"C:\Apps\b.exe".into(), r"C:\Apps\b.exe".into()]);
        assert_eq!(plan, vec![r"C:\Apps\b.exe".to_string()]);
    }

    #[test]
    fn same_basename_different_directory_is_not_a_duplicate() {
        let items = vec![item(r"C:\Apps\a\tool.exe")];
        let plan = plan_inserts(&items, &[r"D:\Other\tool.exe".into()]);
        assert_eq!(plan.len(), 1);
    }
}
