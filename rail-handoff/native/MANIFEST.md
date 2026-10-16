# Rail native proof package — MANIFEST

Package: `rail-handoff/native/` (proof for the 4-file smooth-transition + hover-stability patch, additive, uncommitted on main).
Published: 2026-10-16, handoff branch (not main). No source modifications, no rebuild, no git commits on main.

## Provenance
- Repo: `helpfulbriefl/rail`, base HEAD `cba74d7` (clean base; 6 modified source files remain uncommitted on main and are NOT part of this package).
- Tested EXE: `src-tauri/target/release/rail.exe`, size 4442112 B, pid 33984, CreationDate 2026-10-15 23:00:14, env `RAIL_DIAG=1`.
- Settings: glass theme, side=right, rail_scale=compact, hoverDelayMs=900, hideDelayMs=1200 (defaults, clamped 300..5000), 2560x1080 @100% DPI.
- Capture method: `CopyFromScreen` strip x=2400..2560, y=300..740; panel sub-rect x95-150, y130-320 (55x190 = 10450 px); bright = R,G,B all >235.

## Contents (existing capture artifacts, 2026-10-15 23:30-23:58)
- PNG frames: xf-0..xf-91 (exit-fade series, 92 files), xf-fully-shown.png, rr-0..rr-11 (reveal ramp), reveal-fade-0..13, exit-fade-0.png, po-fully-shown.png, po-frame-0.png, cycle3-partial-opacity.png, cycle-dup-esc1+150ms.png, cycle-dup-esc2+350ms.png, cycle-reentry-full.png, reveal-shot-1/2.png, reorder-before.png.
- Scripts: exit-fade.ps1, reveal-ramp.ps1, measure-xf.ps1, stationary.ps1, cycles10.ps1, partial-opacity.ps1, reorder.ps1, reveal-shot.ps1, gates-frontend.ps1, enum-rail-windows3.ps1, find-cfg.ps1.
- Raw logs: _cfg.txt, _git.txt, _hashloc.txt, _gates-frontend.txt, _ramp-out.txt, _reorder-out.txt, _stationary-out.txt, _status-proc.txt, _where.txt, _diag-base.txt (snapshot, 108 lines), _diag-final.txt (snapshot, 145 lines).
- Report: PARTIAL-REPORT.md (verdict PARTIAL).

## Added by this packaging (2026-10-16) — NOT historical capture data
- `rail-diag.log` — copy of `src-tauri/target/release/rail-diag.log` (151 lines, regions #1..#44) added per owner instruction. NOTE: the 82-min label previously quoted is wrong; the actual gap in the log is between [2308635ms] and [3605364ms] = ~21.6 min, and it covers the xf/rr/stationary/reorder reveal runs (no HITRECT/APPLY/REGION lines for them; only later lines @3605364/@4076524 ms exist).
- `SHA256SUMS.txt` — SHA256 of every package file (generated 2026-10-16).
- `MANIFEST.md` — this file.
- `xf-sat-audit.ps1` — POST-CAPTURE audit tool (created 2026-10-16, GetPixel; LockBits overloads fail in PS 5.1). Its run output was console-only and is NOT saved in this package. It is not part of the original capture session and must not be read as historical measurement.

## MISSING files / absent data (explicit)
1. `new-exe-and-source-hashes.txt` — referenced by `_hashloc.txt` (which claims it exists), but the file is NOT present in this directory. Hash content (if any) survives only inside PARTIAL-REPORT.md.
2. `partial-opacity.ps1` stdout — never saved (no `_partial-out.txt`); only its PNGs (exit-fade-0.png, po-frame-0.png, po-fully-shown.png, reveal-fade-0..13) exist.
3. Per-gate exit codes — `_gates-frontend.txt` contains only "FRONTEND GATES DONE"; individual gate results were not preserved.
4. Hide-chain event logs — `requestHide` (frontend timer), exit-fade-complete (`onExitComplete`/`completeHide`), `railHidden` ack, and `w.hide()` are NOT logged anywhere (no `diag!` in those paths). No timestamps for them exist; they are not reconstructed.
5. Frame-capture cadence — nominal `Start-Sleep 5` in exit-fade.ps1 was not achieved; real NTFS cadence is ~15-30 ms/frame (xf) and ~47 ms/frame (rr). Event times must not be derived from the nominal sleep.
6. Clock alignment — process CreationDate 23:00:14 vs diag-ms anchors implies a possible NTP step; diag-ms must not be converted to wall clock without that caveat.

## FOOTER hover bounds
**STATUS: NOT VERIFIED.** No actually-measured footer hover-zone bounds are saved in this package. The `HITRECT` lines in `rail-diag.log` are the capsule hit-rects (x=4 y=53 w=55 h=206, CSS px at compact scale), not footer hover-zone geometry. The stationary FOOTER test held the cursor at (2525,525) and observed auto-hide firing during hold (`trace=Truex21 Falsex9`), which only proves hover did NOT engage at that point. No positive footer-boundary evidence exists, so FOOTER is marked NOT VERIFIED, not PASS.

## Verification
- Per-file SHA256: `SHA256SUMS.txt` (154 files).
- Package archive SHA256: recorded in the handoff branch commit message and returned with the publish link.
