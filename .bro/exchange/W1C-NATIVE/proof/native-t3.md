# NATIVE T3 — W1C-NATIVE ('+' / reorder / held-hover / OLE; sanitized, run/ raw records ignored)

Fixture: test-owned `run/rail-test.exe` (SHA-256 `43a13486…`, reused from W1B, no rebuild),
`RAIL_DIAG=1` + child-only CDP loopback 9222 (diagnostic launch; a no-CDP native re-check of
visibility/'+' was also performed — see §5). Test-owned config: pinned=true, side=right,
edgeReveal=true, hoverDelayMs=900, items [item-notepad, item-calc].

## 1. '+' — 1 physical Windows trial + CDP invoke-path (diagnostic, NOT hit-test evidence)
- Post-reveal visible window: `0x3D31420 [Rail]` 48×294 @(2512,369), owner pid 70288
  (the 16×16 `dev.rail.dock-siw` / empty-title placeholders are NOT the hit target — Z-01 fixed).
- `+` button live DOM rect CSS (7.5,85) 19×19, client origin (2512,369), DPR 1.0 → physical (2529,464).
- Trial 1 (`run/t3-plus-trial-1.json`): real Windows click at (2529,464) → `#32770` dialog
  `0x49F12C6 pid=70288` rect 1200×540 detected, `SetForegroundWindow`+ESC → **DETECTED_CANCELLED**
  (expected/actual match: cancel path works). `WindowFromPoint` at click time reported an
  occluding Chrome renderer child (`0x281224 [Chrome_RenderWidgetHostHWND] pid=12144`) — recorded.
- Trials 2/3 combined (edge-hold → CDP DOM click → pid-scoped dialog detect → ESC → verify,
  `run/t3-plus-combined-t2.json` / `t3-plus-combined-t3.json`): t2 = DETECTED_OPEN
  (`0x6B1258`, later force-closed via `PostMessage WM_CLOSE`), t3 = DETECTED_CANCELLED
  (`0x1DA11FC pid=70288` 1200×540). CDP DOM click per §3 is the **invoke-path diagnosis only**
  (`rail-head` → `pick_files`), not physical hit-test evidence; the physical leg is trial 1.
- Dialog identification was by PID + window class `#32770` + owner control tree (pid-scoped
  enumeration), not `#32770`-alone (a stale Chrome-owned `#32770 pid=5840` was excluded).

## 2. Reorder two items + restart persistence
- `invoke('reorder', {ids: reversed})` via `__TAURI_INTERNALS__` from the rail webview
  (CDP, deterministic): result `ok:true, via:__TAURI_INTERNALS__.invoke`; on-disk config order
  flipped `[item-notepad, item-calc]` → `[item-calc, item-notepad]`, **persisted=true**
  (`run/t3-reorder-t1.json`).
- Restarted the SAME isolated fixture without rewriting the config: new pid 47828, config order
  unchanged after boot, **kept=true** (`run/t3-reorder-restart-47828.json`). Path:
  `Rail.tsx:331` drag-end → `main.rs:448 reorder()` → `config.rs:146 save()` (atomic tmp+rename).

## 3. Held hover (configured 900 ms + 10 s), card transitions
- Physical `SetCursorPos` hold at tile centre (2529,516) for 12 s, DOM polled via CDP
  (`run/t3-hold-hover-t5.json`): `.hold-ring` appeared **t=0.4 s** (ring pathLength animation),
  hovered tile magnified to **scale 1.4×** by t≈4.5 s with neighbour pushed (1.19×),
  flyout IPC fired (flyout webview body ~8.4 k chars observed as proxy — **mount not confirmed**);
  on release at t=12 s ring gone, scales returned. Three hover trials total:
  t1–t3 (CDP synthetic / PS-parser failures — no physical cursor, ring absent, recorded as
  INVALID for the physical leg), **t5 = the valid physical trial**.
- Card transition chain observed: hold-ring → magnify (Reorder/spring) → flyout request → release.

## 4. OLE real-shell drag — exact limitation + compact manual checklist
Injected `mouse_event`/`SetCursorPos` sequences cannot start a shell OLE file drop: the shell
arms its `IDropSource` only inside its own drag loop entered from a real hardware
button-down routed through the desktop/Explorer `WndProc`; without that, no `CF_HDROP`
`IDataObject` ever exists to drop onto the rail's `IDropTarget` (`set_hit_rect`, `win.rs:308`).
TASK-excluded fallbacks (`__railDrop` synthetic IPC, DOM drag dispatch) were **not** used as
OLE evidence.

**Manual checklist (one pass):** ① show a `.txt` on the desktop / open Explorer →
② physically press-drag the file to the right screen edge, hold until the rail reveals →
③ move over the first tile slot, release → ④ observe the new/updated dock item →
⑤ repeat onto the empty bottom slot (add-new-item path) → ⑥ note dock order before/after.

## 5. Diagnostics vs normal mode (no-CDP re-check, TASK §5.4)
CDP flags were **process-local only** (`WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` on the launched
fixture, child-only, loopback 127.0.0.1:9222, port verified free; listener owned by the fixture
child tree). The native visibility + '+' check was also performed once **without** the CDP
flags (`run/launch-nocdp.ps1` → pid 39968, CDP port 9222 confirmed NOT bound):
- Win32 `EnumWindows` found a fixture-owned Tauri window `0x350F1C @(2292,452) 200×120`
  (a flyout card from a prior CDP session, still visible). `GetWindowRgn` returned `0x0`
  (no region handle for this window — a distinct case from the rail capsule's active region;
  "skip empty" ≠ "NULLREGION").
- `WindowFromPoint(2529,464)` → `Chrome_RenderWidgetHostHWND pid=55088` (Chrome still occludes
  the physical '+' zone; rail capsule 48×294 at (2512,369) is behind it).
- **Without CDP the DOM content (visible non-empty `.rail`) cannot be read** — only the Win32
  window tree is observable. The capsule's specific 48×294 geometry was confirmed in the CDP
  session (trial 1); the no-CDP re-check confirms the fixture runs and a Tauri window is visible,
  but does not re-confirm the capsule DOM. This is recorded as a **limitation of the no-CDP
  check**, not a product defect.
- Owned CDP listener shut down after capture; fixture stopped at wave end.
