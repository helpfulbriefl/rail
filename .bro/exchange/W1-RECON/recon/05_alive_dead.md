# 05 · alive / dead

Status legend: 🟢 alive (wired, exercised) · 🟡 partial (wired but not exercised this stand) · 🔴 dead / unreachable / stale

| module / path | status | proof |
|---|---|---|
| `src/main.tsx` (root render, mounts `Rail`) | 🟢 | imported by `index.html`; entry graph in 02_repomap |
| `src/rail/Rail.tsx` (capsule, dock, drop, hover, «+») | 🟢 | 10 imports; exercised by the running UI |
| `src/rail/Tile.tsx` (single tile, lamp, reorder handle) | 🟢 | 8 imports |
| `src/rail/magnify.ts` (dock spring magnify) | 🟢 | used by `Rail.tsx` |
| `src/flyout/Flyout.tsx` (info + metrics card) | 🟡 | 10 imports; not opened in this stand (no items) |
| `src/settings/Settings.tsx` | 🟡 | 7 imports; window exists hidden at launch |
| `src/bridge/api.ts` (IPC seam, drop, hover) | 🟢 | single UI↔OS seam |
| `src/bridge/mock.ts` (browser fallback) | 🟡 | used only outside Tauri |
| `src-tauri/src/main.rs` (commands, tray, windows, pick_files, add_paths) | 🟢 | `tauri::generate_handler!` |
| `src-tauri/src/tracker.rs` (cursor/edge/scan/metrics loop) | 🟢 | spawned at setup (`main.rs:875`) |
| `src-tauri/src/win.rs` (EnumWindows, SetWindowRgn, OLE, ShellExecute) | 🟢 | used by tracker + do_launch |
| `src-tauri/src/config.rs` (portable config + migration) | 🟢 | `config::load()` at startup |
| `src-tauri/src/core.rs` (Shared state, Rect) | 🟢 | `Core::new()` |
| `src-tauri/src/diag.rs` (`RAIL_DIAG=1` opt-in log) | 🔴 **stale exe** | present in Q source, but delivered `Rail.exe` produced **no** `rail-diag.log` under `RAIL_DIAG=1` → binary predates this module |
| `qa/visual.mjs` + `qa/storyboard.mjs` | 🟡 | invoked by `npm run qa`; need `node_modules` (absent in Q, install not authorized this wave) |
| `docs/SPEC.md §9.x` "closed" claims (drop §9.1, «+» §9.2, RAM §9.10) | 🔴 stale | Owner still reports B-001/B-003 broken; SPEC out of date vs code (α 0.35 vs 0.25) |
| `P\Rail.exe.bak-old-0504` | 🔴 dead | older build artifact, superseded by `Rail.exe`; not source |
| `src-tauri/target/**` (debug+release) | 🔴 build cache | excluded from Q by `.gitignore` |
| `global-promt-system.zip` | — | private-system snapshot, excluded from Q source tree (published only as redacted `docs/prompt-system`) |
