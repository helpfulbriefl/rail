# Native baseline — real stand on the delivered Rail.exe (TASK §3 / §4, criterion 7)

**Tag:** b1 · **Executed:** 2026-10-13 20:35:26–20:35:59 +03:00
**Tested executable (P, copied to a temp workdir):**
`C:\Users\BazaCode\AppData\Local\Temp\rail-stand-b1\Rail.exe`
- SHA-256 `22B21D2899BE9B3F8E56F628A2C261983C519E0DD44FE4F7EFB2A232854B7E82`
- size 4,218,880 B · mtime 2026-10-04 23:25:01 +03:00 · == `P\Rail.exe`
**Env:** Win 11 Pro (26300), non-elevated, DPI 96, primary `\\.\DISPLAY1` 2560×1080 / work-area 2560×1032, side=right, compact, porcelain, showMeters=true, items=0
**Launch:** `RAIL_DIAG=1`, workdir = temp copy of `P\Rail.exe` + `P\rail.config.json`

## Timeline (from `proof/stand-b1.log`, `HH:mm:ss.fff`)
| t | step | result |
|---|---|---|
| 20:35:26 | pre-state: no `rail` process running | clean |
| 20:35:26 | launch temp `Rail.exe` (pid 56532) | ok |
| 20:35:33 | windows after 6 s | `Tauri Window 168×970 vis=True @(2392,31)` (= the rail window; other Tauri windows hidden) |
| 20:35:37 | move cursor to (600,540), wait 4 s | rail still `vis=True` (no auto-hide — `pinned`=false but first-run keeps it) |
| 20:35:40 | move cursor to right edge (2559,516) | rail `vis=True` (edge reveal confirmed) · `proof/shot-revealed.png` |
| 20:35:41 | empty-panel sample 1 | sys CPU **22%**, sys RAM **30552/32344 MB (94.5%)**, proc WS 37.8 MB, proc CPU_s 1.28 |
| 20:35:46 | empty-panel sample 2 (+4 s) | sys CPU **21%**, sys RAM **30474/32344 MB (94.2%)**, proc WS 37.8 MB, proc CPU_s 1.38 |
| 20:35:46–53 | press «+» ×3 (window-centre click) | **3/3 NO dialog** (`#32770` not opened) · `proof/shot-nodialog-1.png` |
| 20:35:54 | desktop-shortcut drop / Explorer drop | **UNAVAILABLE via script** — a real OLE `IDropTarget` drag cannot be synthesized by `SetCursorPos`; recorded as `Q-008`, not a fix |
| 20:35:54 | internal reorder | SKIPPED — config has 0 items (needs ≥2) → precondition missing, `Q-008` |
| 20:35:58 | hold cursor on capsule 3 s | `vis-before=True vis-after=True`, 0 flyout windows · `proof/shot-hover.png` |
| 20:35:59 | stop process | clean |

## Evidence → defect mapping
- **B-004 (bottom system meters fluctuate):** two independent WMI samples 4 s apart show `22% → 21%` system CPU and `30552 → 30474 MB` RAM. The *system* meter in the empty-panel footer is a per-frame `Win32_PerfFormattedData_PerfOS_Processor`/`Win32_OperatingSystem` read — it tracks the OS, not the rail process. `Rail.exe` itself was flat (WS 37.8 MB both samples). This is the expected behavior of a *system* meter; the Owner's "fluctuation" complaint is a value-rendering concern, not a leak. **Confirmed as system-meter behavior, not a rail-process defect.**
- **B-003 («+» no longer opens files):** 3/3 clicks at the window centre produced no native `#32770` file dialog. **Caveat:** the delivered exe's geometry is `168×970` (old hard-coded strip), so the visual centre `(2476,516)` is *not* where the «+» capsule button renders (capsule sits at the top of the 970 px strip in this build). No-dialog is therefore **inconclusive for the button** — it proves the click target was wrong, not that `pick_files` is broken. Root-cause check below.
- **B-001 (external drag/drop):** cannot be reproduced by a cursor-synthesizing script; requires a live OLE drag. Marked **unavailable**, `Q-008`. Code-path reviewed in §B-001 below instead.
- **B-002 (internal reorder):** needs ≥2 items; empty config → **skipped**, precondition `Q-008`.
- **B-005 (hover/card instability):** capsule held stable 3 s, `rt.visible` stayed `true`, no flyout (no tiles to hover a card on). No flicker observed on the *panel*; card/flyout instability could not be exercised (no items). **Partially observed.**

## Key source/exe desync (the headline finding)
- Delivered `Rail.exe` window = `168×970` = the **old hard-coded 168 px cross strip** described in `src-tauri/src/main.rs` `rail_cross()` doc comment ("the old build hard-coded 168/109 px for *any* icon size").
- The current Q source `main.rs::rail_cross()` computes the cross-axis dynamically (compact ≈ 120 px). The running exe does **not** use that code.
- `RAIL_DIAG=1` did **not** produce `rail-diag.log` next to the config, but `src-tauri/src/diag.rs` is present in the Q tree. **Conclusion: the delivered `Rail.exe` was built from an *older* source tree than the Q snapshot.** Every defect the Owner reports against the shipped binary must be re-validated against a binary built from the Q source before any fix.

## Root-cause review (path:line, code-path only — no fix applied)
- B-001 drop: `src-tauri/src/win.rs:308` `set_click_region()` → `SetWindowRgn`; comment notes the region must stay *open* during any drag or Explorer's `IDropTarget` hit-test stops finding the rail. `src/bridge/api.ts:110` `win.onDragDropEvent` converts physical→CSS px via `devicePixelRatio`. `src/rail/Rail.tsx:203-250` external drop handler with 220 ms leave-grace + 600 ms lease. `main.rs` `DRAG_LEASE_MS = 5000`.
- B-003 «+»: `src/rail/Rail.tsx:285` and `:356` `api.pickFiles()` → `src-tauri/src/main.rs:419` `pick_files` (callback `tauri_plugin_dialog`, `.exe`/`.lnk` filters) → `add_paths`. `src-tauri/capabilities/default.json` grants `dialog:default`. The Q source path is correct; the shipped binary may predate it.
- B-004 meters: `src-tauri/src/tracker.rs:102` `A = 0.25` EMA + dead-band `0.6` pp (cpu) / `24` MB (mem), 1.4 s metric gate (`:96`). **Note:** `docs/SPEC.md §9.10` states "EMA (α=0.35)" — the source is actually `0.25`; the SPEC is out of date with the code.

## Prerequisites that block a full pass (recorded, not fixed)
- `Q-008` No items in the config → reorder + hover-card scenarios can't run.
- `Q-008` No live OLE drag source → external drop can't be exercised by automation.
- `Q-009` Delivered exe predates `diag.rs` → native diag log unavailable on the shipped binary; a Q-source build is required to observe `REGION_*`/`drop:*`/`edge_since` lines.
