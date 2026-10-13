# NATIVE BASELINE — W1B-BASELINE (B-003 '+', B-001 drop, B-002 reorder, B-005 hold)

Status: **PARTIAL / unavailable** (automated) — honest, not PASS.
tested_source_commit = f7f740d1002de624e8078c8ebdd34cdb5746a82d (pulled head)
exe under test = run/rail-test.exe (copy of src-tauri/target/release/rail.exe, SHA-256 43a13486…, 4404224 B, copy equality verified in exe-provenance.log)
launch env = `RAIL_DIAG=1`, WorkingDirectory = run/ , config resolved to run/rail.config.json (exe-adjacent first candidate per config.rs::candidates).

## 1. Reproducible blocker for the '+' button (B-003)

In the **release** build the WebView2 DOM is not exposed to Windows accessibility:

- UIA (v1/v2) sees only **2 top-level** windows of the process:
  `dev.rail.dock-sic` (class) and `Tao Thread Event Target`. No button, no edit,
  no tree — so `FindFirst(button[aria-label=Добавить])` returns nothing.
- UIA `BoundingRectangle` of the transparent Tauri window resolves to **(0,0)**,
  so an automated click at (0,0)-relative coords lands on the wrong screen point.
- Win32 `GetWindowRect` returns the true physical rect (e.g. `L=2512 T=386 W=48 H=260`
  on this 1920×1040 work-area display), but the **content region is empty** in the
  hidden state: diag log shows only

  ```
  [0ms] APPLY   force=true drag=false open=false hit=(0,0,0,0)
  [0ms] REGION  skip #1 reason=empty_hit_rect
  ```

  i.e. `set_hit_rect` was never called (React rail did not paint a hit-rect because the
  panel is auto-hidden). No `HITRECT`/`REGION set` line → the capsule window region is
  a 0×0 rect, so any synthetic click at the computed coordinate is a **no-op** on the
  desktop underneath (click-through). This is reproducible across v3 and v4 runs.

### Four automated attempts (all INVALID, none PASS)
| # | method | outcome | why |
|---|--------|---------|-----|
| v1 | UIA DOM `button[aria-label]` | no button found | a11y tree absent in release |
| v2 | UIA `BoundingRectangle` click | landed at screen (42,68) — wrong | transparent window rect → (0,0) |
| v3 | Win32 `EnumWindows`+`GetWindowRect` click | no dialog; hung on desktop-wide UIA `FindAll` | empty region → click-through; UIA hang |
| v4 | Win32 rect + DPI + bounded `#32770` detect | no dialog; empty hit-rect confirmed in diag | region empty in hidden state; `DeviceDpi`=0 (per-app DPI unaware in this launcher) |

Conclusion: **the '+' target cannot be verified by synthetic input in this build**
without first making the rail *visible* (hover/edge-reveal/pinned) so it publishes a
non-empty hit-rect. Per TASK §5.3/§5.4 a wrong/unverifiable click is INVALID, not a
button FAIL; the native '+' outcome stays **unavailable (automated)**.

## 2. Manual checklist for the '+' control (compact, operator-performed)

Precondition: rail must be **visible** first. One of:
- hover the screen edge (side=right) → edge-reveal (260 ms) opens the rail, or
- set `pinned=true` in run/rail.config.json before launch, or
- right-click the tray icon → Show Rail.

Steps (3 attempts incl. one cancel):
1. Visible rail on right edge. Click the 2nd head button ("+") → native file
   dialog (#32770, owned by the rail PID) opens.
2. Type/paste the full path to `run/test-notepad.lnk`, press Enter.
   Expected: run/rail.config.json gains 1 item `{name:"test-notepad", target:"notepad.exe"}`.
3. Repeat click "+" → open dialog → press Escape. Expected: item count unchanged.
4. Click "+" again → pick `run/test-calc.lnk` → Enter.
   Expected: item count 2, second `{name:"test-calc", target:"calc.exe"}`.
5. Restart the exe; both tiles persist (reorder/persistence, B-002).
Record a cropped screenshot of the dialog + the two config states (redact nothing here;
all test-owned).

## 3. Stage B fixture-only seeding (allowed by TASK §5.3 — "+" failure kept open)

Because the automated "+" is unavailable, a **clean test-owned** config with two items
is seeded directly from the `core.rs` schema (not guessed). This is labelled
**fixture-only seeding**; it is NOT evidence that "+" works. `run/rail-config-B.json`
is the 2-item seed; it is copied to `run/rail.config.json` before the reorder/hold run.

Items (schema Item, camelCase):
- id "item-notepad", name "test-notepad", launchPath run/test-notepad.lnk,
  target "notepad.exe", kind "app", icon "", addedAt/lastLaunch=now, launchCount=0
- id "item-calc", name "test-calc", launchPath run/test-calc.lnk,
  target "calc.exe", kind "app", icon "", addedAt/lastLaunch=now, launchCount=0

## 4. Edge reveal / hold / reorder (B-005 / B-001 / B-002)
- hidden→edge reveal: see meter-trace run — "confirm hidden BEFORE claiming edge
  reveal" requirement recorded in meter-trace.md (rail starts auto-hidden; hover the
  right edge 260 ms reveals it).
- reorder 2 items + restart persistence, hold cursor on item for hoverDelayMs then
  10 s, and OLE drag (desktop→rail, Explorer→rail, shell-drag→hidden rail): these are
  operator/manual (OLE `IDropTarget` on WebView2 is not drivable by `SetCursorPos`;
  see diag.rs H-2/H-3/H-4). Each is a **compact manual checklist item** in
  meter-trace.md, not an automated PASS.
