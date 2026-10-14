# W3C-CLOSEOUT continuation — ONE consolidated Owner native checklist

Use **only** the exact binary below. Do not test any older build and do not
judge these checks from the automated/browser run — they are the physical
Windows confirmations the coder cannot perform. Native verdict stays **PARTIAL**
until you tick these off.

Binary under test
- path: `c:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail.exe`
- SHA-256: `e6e0bf48846844e25be8f86c284ba0600033c94c851223dcbb3b46b936697a2c`
- size: `4441600` bytes
- built from source commit: `c3b9db1`
- locally built only — it is **not** attached to a GitHub release

Timing: use your **configured** delays, not a fixed number.
- open hold = Settings → *hover delay* (your value, hard-capped at 3000 ms)
- hide delay = Settings → *hide delay*
Read both values first and judge every "wait" against them.

Before you start: close any *previous* rail instance so you are not testing a
stale single-instance process. Do **not** kill any of your own working apps and
do **not** change any OS security setting to make a step pass.

## (a) Geometry — no clipping / jitter, crowded footer reachable
- [ ] Ordinary rail: icons fully inside the screen edge, no half-pixel jitter at rest.
- [ ] Crowded rail (many icons, normal scale): the last icon and the footer stay fully reachable; nothing is pushed off-screen or overlaps.
- [ ] Repeat at your available DPI: `100%` [ ] `125%` [ ] `150%` [ ] `200%` [ ] — tick each you actually try, mark the rest *untested*.
- [ ] Each side you use: `right` [ ] `left` [ ] `bottom` [ ] `top` [ ]; note any other monitor/scaling as *untested*.

## (b) Hover → popup across the gap + real dismiss
- [ ] From the footer, move across the gap: the popup opens after your configured hold, without flicker.
- [ ] Click the CPU value and the RAM value in the popup: each is really received (tab/selection responds).
- [ ] Press **Escape** and click **outside**: both dismiss.
- [ ] Hold the pointer **stationary** on a row: the ranking freezes (does not reshuffle under the cursor), then resumes live once you leave.

## (c) Close (X) only on a disposable app
- [ ] Open a throwaway app with **unsaved work** (e.g. Notepad with an edited, unsaved file).
- [ ] Click its **X** in the monitor: it must show its own *save / don't-save / cancel* dialog. **Cancel** keeps the app alive (no forced kill).
- [ ] A protected/system row's X is disabled and never sends a close.

## (d) True Explorer drop + reorder
- [ ] Drag a real `.lnk` shortcut from Explorer onto the rail: it appears as a new tile.
- [ ] Drag a real folder from Explorer onto the rail: it appears.
- [ ] Reorder tiles by dragging in the UI; the new order holds.

## (e) Graceful quit / restart preserves state
- [ ] Change a setting (theme and/or tile order), then quit the app **gracefully** (its own quit, not Task-Kill) and start it again.
- [ ] Theme and tile order are exactly as you left them.

## What this confirms
- Ticking (a)–(e) on the binary above upgrades rows 1, 2 and 4 of
  `proof/INDEX-CONTINUATION.md` from PARTIAL to native PASS.
- Any step you cannot perform (DPI, a side, a monitor) — leave it explicitly
  **untested**; do not mark it pass. Report the exact step that fails, if any.
