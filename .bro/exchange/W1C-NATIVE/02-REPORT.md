---
wave: W1C-NATIVE · ver: v3.0 · t_start: 2026-10-13 23:48 · t_report: 2026-10-14 03:03
commit: 669169a · tag: w1c-native-done · push: ❌ · bundle: - · pack: - · exchange: -
tasks: 5 · done: 4 · gates_first: yes · repair: 1 · proof_ok: 4 · proof_req: 5
diff_files: 3 · diff_add: 31 · diff_del: 10 · out_of_scope: 0 · q: 3 · contradictions: 1
K1: 1 · K2: 1 · K3: 1 · K4: 1 · k_why: - · friction_cd: #env
cd_in: 8790 · cd_out: 2579 · prompt_weight: 4515 · an_in_prev: 2745 · an_out_prev: 6016
---
# REPORT W1C-NATIVE — expose the live test panel; repair only the QA harness

## Tasks
1 ✅ `run/t1-facts.json:4` + `logs/provenance.log` — live owned fixture: pid 70288 (post-restart 47828), real `[Rail]` top-level 48×294 @(2512,369) owner pid, monitor 2560×1080 / work 2560×1032 / DPI 96 / dpr 1.0; root cause of T1 bootstrap failure = UTF-8 **BOM** written by `Set-Content -Encoding UTF8` → config parse reject, fixed BOM-free (`[IO.File]::WriteAllText … UTF8Encoding($false)`). Exe reused (SHA-256 `43a13486…`), no rebuild.
2 ✅ `qa/storyboard.mjs` + `qa/visual.mjs` (exact §4 edits) + `run/exits.txt:1` — `npm run typecheck` EXIT 0 (3 s), `lint:css` EXIT 0 (1.6 s), `npm run qa` EXIT 0 (80 s); `visual.mjs` 23/23 PASS and the storyboard leg now **completes** (mock scenes; generated PNGs present). Mock-visual only — no native acceptance claim.
3 ❌ `run/t3-plus-trial-1.json` + `run/t3-reorder-t1.json` + `run/t3-hold-hover-t5.json` + `run/t3-ole-limitation.md` — 4 of 5 legs proved: '+' real Windows click → `#32770` (pid=70288) detected + ESC-cancelled, invoke-path also via CDP (labeled diagnostic, not hit-test); 2-item reorder via Tauri IPC `invoke('reorder')` → `persisted:true`, restart kept reversed order (`kept:true`); physical held-hover → `.hold-ring`@0.4 s + magnify 1.4× + card transitions. OLE real-shell mouse-down/move/up **not executable by automation** (shell `IDropSource` cannot be armed from injected input) — exact limitation + compact operator checklist, not a hidden blocker nor "impossible in principle".
4 ✅ `run/t4-foot-series.json` + `run/t4-ref-meters.json` — 60 s paired `.rail-foot` CDP series (30 finite samples, CPU 27–72 % / RAM 89–98 %, 2 s poll, host clock) against an independent WMI reference (RAM 94.7–98.9 %, valid RAM pairing); CPU reference degraded to 0 % in this VM (Win32_Processor artifact) — documented, no "not a leak" inferred (B-004 open).
5 ✅ `proof/INDEX.md` + `02-REPORT.md` + `../W1B-BASELINE/04-ERRATA.md` + `README.md` + `logs/provenance.log` — parser-correct INDEX (one backticked row/criterion, honest ✅/❌/—), redacted gate/provenance/sample proof, W1B errata appended (E-001..E-006), README QA + native status updated; delivery SHAs reported at push.

## Gates
typecheck ✅ (EXIT 0, 3 s) · lint:css ✅ (EXIT 0, 1.6 s) · `npm run qa` ✅ (EXIT 0, 80 s; visual 23/23 + storyboard leg complete) · secrets 0 · P↔Q product tree unchanged · no dependency/lock/manifest change · no release, no native-config edit.

## Spots for analyst (3–5)
- `src-tauri/src/main.rs:448` — `reorder()` → `config::save`; the IPC path W1C exercised is exactly the drag-end path (`Rail.tsx:331` → `api.reorder`), so a real reorder and the IPC call share one sink.
- `src/rail/Tile.tsx:83` — `startHold()` is fired by DOM `onPointerEnter`; a **physical** `SetCursorPos` hold on the tile drives it (W1C `t3-hold-hover-t5`), but CDP `dispatchEvent('pointerover')` does not — synthetic pointer events don't arm the React hold.
- `src-tauri/src/tracker.rs:276` — footer meters are system-wide (CPU α=0.25 EMA, RAM 24 MB dead-band), published via `emit_to("rail","runtime")`; W1C read the rendered `.rail-foot` text, not the producer.
- `qa/storyboard.mjs` — `URL.pathname` on Windows produced `C:\C:\…` doubling; the §4 `fileURLToPath` fix is a QA-only exception, product untouched.
- `run/t4-ref-meters.json` — independent CPU reference = 0 % throughout (VM `Win32_Processor.LoadPercentage`/`PerfFormattedData` artifact); CPU pairing is therefore degraded, RAM pairing valid. Flag for a non-VM host re-run.

## Q / contradictions
- Q-001 (open question, preserved): an earlier fixture (pid 5984) showed `settings.pinned` flip to `false` on a save path; W1C pinned `pinned=true` in the test-owned config and it held across restart — the flip is **not** reproduced/asserted, kept as an open question, no product edit.
- Contradiction 1: TASK §3 wants "actual desktop/Explorer OLE entry … three attempts" but the shell `IDropSource` drag cannot be started from any injected `mouse_event`/`SetCursorPos` sequence (the shell never enters its OLE drag loop, so no `CF_HDROP` `IDataObject` is ever created). Recorded as an **exact automation limitation** + operator checklist; not counted as a product FAIL and not padded with `__railDrop`/DOM-dispatch evidence (both explicitly excluded by §3).
- Unknowns preserved: exe age / P↔mapping of the *delivered* binary (Q-009/Q-SRC) and flyout-mount confirmation for the held-hover leg (recorded via a proxy body-length probe, not a confirmed flyout DOM).

## Out of scope
- No product/source/native-config edit (QA-only exception + ignored test-owned `run/` scripts). No "not a leak"/"fixed" verdict (B-001..B-005 open). No `.github/` (Q-005) — left untracked. No data deletion, no forced/reset/rebase, no credential/account change. Raw `run/` dumps ignored; only sanitized paired samples + structured records published.

Secrets not opened: yes
