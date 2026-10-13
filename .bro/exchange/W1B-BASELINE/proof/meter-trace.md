# METER TRACE — W1B-BASELINE (TASK §3.4 / §5.4)

Run: `run/meter-trace.ps1`, rail-test.exe PID 62452, `RAIL_DIAG=1`, config `run/rail.config.json`
(pinned=false, edgeReveal=true, 2 items, showMeters=true — fixture seed, B-003 open).

## 1. 60 s independent CPU/RAM reference (CAPTURED)

File: `run/meters-ref.csv` — 60 rows, ~1 s interval, UTC timestamped.
- CPU: global `\Processor(_Total)\% Processor Time` via `PerformanceCounter` (raw ~1 s delta, **no EMA**). Observed range t=15..60: ~24–56 % (idle-ish desktop, no load driver in this wave).
- RAM: `Win32_OperatingSystem` `TotalVisibleMemorySize − FreePhysicalMemory` (WMI). Observed ~28 084–28 708 MB used of 32 343.9 MB total (denominator = TotalVisibleMemorySize, unit MB).
- Units/denominator + no-EMA note recorded in the CSV header/`note` column.

This reference is **independent** of the Rail in-process `sysinfo` producer.

## 2. Rail footer (sysinfo producer) series — UNAVAILABLE

In this build the runtime meters are emitted **only** over the Tauri IPC bus
(`app.emit_to("rail","runtime", ...)` / `emit_to("flyout","runtime", ...)`, `tracker.rs::spawn`),
and are **not** written to any log/file. The release a11y tree is absent (§native-baseline),
so the on-screen footer value cannot be read back externally. Therefore a **time-aligned Rail
footer series is not capturable** without an in-process/IPC tap that this wave is not permitted to add
(no code repairs). Per TASK §3.4: **no "not a leak" / "not fixed" conclusion is drawn**; B-004
(source/exe age + leak claim) **remains unresolved** pending a paired trace with a real producer tap.

Expected vs actual (separate):
- Expected (per §3.4): 60 s Rail-footer sysinfo series **and** an independent reference, time-aligned,
  with the two compared.
- Actual: independent reference captured (above); Rail-footer series unavailable (IPC-only, no file/
  a11y). The comparison is therefore **one-sided** and is recorded as such, not merged.

## 3. Diag log absent in this launch — recorded, not inferred from

- Launch env recorded: `RAIL_DIAG=1` set on the child via `ProcessStartInfo` (`CreateNoWindow=true`).
- Expected sink: `run/rail-diag.log` (config.rs::candidates → exe-adjacent, i.e. `run/`).
- Inspected source condition: `diag.rs` opens `SINK` lazily on the **first** `diag!` call
  (`region_applied`/`region_skipped`); when the rail is auto-hidden with a non-empty item list,
  the only region calls early are `REGION skip reason=empty_hit_rect`. No file was created in this run.
- Per TASK: a missing diag log must **not** be read as "stale build"; it is recorded as a
  launch/sink non-emission for this specific invocation only.

## 4. Edge reveal (hidden→shown) — unproven (operator/manual)

Requirement met to the extent possible: the rail **starts auto-hidden** (non-empty config → no
`show_rail` at startup, main.rs `items.is_empty()` gate). Real cursor was moved to the right screen
edge `(2559,540)` and held 1.2 s (band need = 260 ms). **No `REGION set` observed** (diag empty), so
the reveal is **not confirmed** — edge-reveal stays **unproven**, matching the W1 note.

Manual checklist (compact, operator-performed) to close edge-reveal + hidden-rail drop:
1. Confirm rail hidden (no capsule on the right edge; tray icon present).
2. Move the physical mouse to the right edge, central third, hold ~0.5 s → capsule slides out.
3. While a file is being dragged (left button held, band widens to 12 px / 60 ms), touch the edge
   → rail stays open for 2500 ms (`drag_until`) to accept the drop.
4. Drop a desktop `.lnk` onto an empty dock slot → new tile appears; drag an existing tile to
   reorder; restart the exe → order + both tiles persist (B-002).

## 5. Real OLE drags (B-001) — exact limitation + manual checklist

- What WAS automated: genuine cursor gestures via Win32 `SetCursorPos` (edge reveal above). The
  reference CPU/RAM loop is a real, independent measurement.
- Exact limitation: an external **OLE `IDropTarget` drop** (desktop/Explorer file drag onto the
  transparent WebView2 rail) cannot be driven by synthetic `mouse_event`/`SetCursorPos` — a real
  file-drag needs a genuine shell OLE drag-drop session, which is not reproducible headlessly. This is
  recorded as **unavailable (automated)**, not "cannot be automated in principle" and not a FAIL.
- Manual checklist (3 scenarios, 3 attempts each):
  1. Desktop → open rail: create `test-drop.lnk` on the desktop, drag it onto the visible rail's
     empty slot → tile added. Repeat 3×.
  2. Explorer → open rail: drag a `.lnk` from Explorer into the open rail dock → tile added. 3×.
  3. Shell drag → hidden rail at edge: hold a file, carry it to the screen edge (band widens),
     drop into the revealed rail → tile added. 3×.
  Record cropped, redacted screenshots of each.

## 6. Held-hover on item (B-005) + reorder persistence — manual

- Hold cursor on a tile for `hoverDelayMs` (900 ms) → info/metrics flyout opens; hold 10 s → flyout
  persists. Not externally observable (frontend `show_flyout`), so operator-performed.
- Reorder 2 items then restart → both tiles + new order persist in `run/rail.config.json` (B-002).
