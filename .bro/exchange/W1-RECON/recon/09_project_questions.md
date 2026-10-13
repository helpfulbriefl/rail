# 09 · project questions (TASK §5)

Answers are diagnostic only; no fixes applied. Each answer is anchored to `path:line` or to the native stand (`proof/native-baseline.md`).

## Q1 — Which external drag/drop variants fail (B-001)?
The code path is `SetWindowRgn`-clipped click-through: `win.rs:308` `set_click_region()` calls `SetWindowRgn` with `redraw:false`. The comment documents that the region must stay *open* during **any** drag (internal tile and external OLE alike), otherwise Explorer's `IDropTarget` hit-test — which runs against the live region — stops finding the rail and the cursor shows "no entry". The frontend seam is `api.ts:110` `win.onDragDropEvent` with a physical→CSS `devicePixelRatio` conversion (`api.ts:105-128`). The drop handler with 220 ms leave-grace + 600 ms lease refresh is `Rail.tsx:203-250`. The native drag lease is `DRAG_LEASE_MS = 5000` (`main.rs`, module-level const). **Which exact variant fails (desktop .lnk vs Explorer vs .exe) could not be established by automation** (no live OLE drag source) → `Q-008`. The *mechanism* is known (region during OLE), the *reproducing variant* is not.

## Q2 — What exactly does the '+' symptom look like (B-003)?
The button is `Rail.tsx:285` (tile variant) and `:356` (empty-tile variant), both `onClick={() => void api.pickFiles()}`. `pickFiles` → `main.rs:419` `pick_files`, a **callback** `tauri_plugin_dialog` call with `.exe`/`.lnk` filters → `add_paths` (`main.rs:371`). `capabilities/default.json` grants `dialog:default`. On the *delivered* exe the stand got **no dialog on 3 centre-window clicks**, but that is inconclusive for the button: the running window is `168×970` (old strip) so the visual centre is not the «+» location. The Q source path looks correct; the failure is most plausibly a **stale-binary** artifact (the running exe predates the callback fix, SPEC §9.2). To confirm: build from Q source and re-test «+`.

## Q3 — System meter vs per-item meter (B-004)?
Two distinct producers:
- **System meter** (empty-panel footer): `tracker.rs` scans `Win32_PerfFormattedData_PerfOS_Processor` / `Win32_OperatingSystem` → `SysRt` → `rt.sys.cpu` / `memUsed`/`memTotal*100`. Smoothed by EMA `A = 0.25` (`tracker.rs:102`) with a 0.6 pp / 24 MB dead-band and a 1.4 s gate. The stand's two samples (`22%→21%`, `30552→30474 MB`) track the *OS*, so visible fluctuation here is expected and is **not** a leak.
- **Per-item meter**: per-`Item` `Smooth` struct (`tracker.rs:47-52`) from `sysinfo` process samples. These appear in the flyout, not the footer.
The confusion is that the footer's *system* value fluctuates with the whole machine while Owner expected a stable per-item figure. **No code fix in scope.**

## Q4 — Window geometry vs value changes?
`main.rs:25` `rail_cross()` and `:186` `show_rail()` compute the window rect from `settings.rail_scale` + `icon_size`. The delivered exe still uses the **old hard-coded 168 px** (window measured `168×970`), whereas the Q source derives it dynamically (compact ≈ 120 px). So the *geometry* the Owner sees is from the old binary; the Q source would produce a narrower strip. Geometry and meter values are independent subsystems; the stand saw flat process WS (37.8 MB) while the *system* meter moved.

## Q5 — Which fixes were already attempted?
`docs/SPEC.md §9` records the history: §9.1 OLE drop (3 failed attempts → `SetWindowRgn`), §9.2 «+» (blocking→callback), §9.4 tracker deadlock, §9.5 cascading mutex-poison panic, §9.6 SetWindowLongPtrW removal, §9.10 RAM jump (EMA). The Owner reports §9.1 and §9.2 still broken — and the delivered binary predates them (Q-SRC), so the "closed" labels are **stale** against the shipping artifact (B-007).

## Q6 — Actual source↔exe relationship?
**UNKNOWN / STALE.** `P\Rail.exe` (SHA-256 `22B21D28…`, 2026-10-04) behaves like the *pre-refactor* tree: 168 px strip, no `RAIL_DIAG=1` log output, no `diag.rs` module. It cannot be mapped to a Q commit (no VCS in P, no build provenance). This is recorded as `Q-SRC` / `Q-009`, **not** resolved.

## Q7 — Native QA coverage?
`qa/visual.mjs` + `qa/storyboard.mjs` (Playwright + mock backend) are the only automated checks; they exercise the **webview** windows × 4 themes, **not** the native Win32 `SetWindowRgn`/OLE path. No native unit/integration test for `win.rs`/`tracker.rs` exists. `npm run qa` chains typecheck + stylelint + build + both. All BLOCKED this wave (no `node_modules`).

## Q8 — License question (D-005)?
**Not set.** Q has no `LICENSE` file. TASK says "propose MIT, do not invent an owner/approval." Recorded as `Q-LICENSE`: propose MIT, awaiting Owner sign-off. No copyright owner asserted in this report.

## Q9 — Is the tracked history safe to publish later?
Q's published history is a **fresh** publication clone: 69ed182 (initial) → 5d77079 (analyst audit) → ef1a3c1 (w1-snapshot). No private pre-publication commits, no secrets, no `global-promt-system` git history, no credentials. The private SYS repo (`helpfulbriefl/global-promt-system`, HEAD `e35e7e2…`) was only *spot-copied* as redacted generic files into `docs/prompt-system`. So the **tracked history in Q is safe to keep public**; the *private* SYS history is not imported. Future waves must not import private SYS git history.

## Open Q-items carried to REPORT
`Q-005` (CI file excluded — workflow scope), `Q-008` (drop/reorder/hover-card can't be automated), `Q-009` (stale exe, no diag log), `Q-SRC`, `Q-LICENSE`, `Q-ENV` (WebView2 EKR not found at the three standard registry paths — Tauri likely bundles it; confirm via `Win32_VideoController` + runtime before concluding).
