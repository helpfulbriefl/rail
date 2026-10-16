# PARTIAL REPORT — rail smooth-transition native proof (session, 2026-10-15T21:10Z)

## Verdict: **PARTIAL**
All local gates pass (clippy kept at baseline, not fixed). Native edge-reveal, 10 show/hide
cycles, duplicate-hide, rapid re-entry, stationary stability and the reveal/exit fade were
verified on the exact new EXE. The physical Windows DnD (real Explorer file-drop + in-rail
framer drag) was NOT re-performed headlessly; cross-DPI coverage is labelled untested, not
fabricated. No coloured/white-flash/trail artifacts observed in captured frames.

No git commits, no reset/rebase/stash/clean, no fm-bridge, no speculative Win32/WebView changes.

## Provenance
- Repo: `c:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish`, branch `main`, HEAD `cba74d7`
- Package base HEAD `cba74d798dea763ab6ea289d0d3e25e314fa3205`
- New EXE: `src-tauri\target\release\rail.exe`
  - SHA256 `92F501467169E36021B2CCCDC6008396E034D56CC8BD145305B8E0984A699246`
  - size `4442112`
  - launched once with `RAIL_DIAG=1` → pid `33984` (still running; ExecutablePath + size confirmed this session)

## Gate commands + exit codes (fresh, this session)
| Gate | Command (cwd) | Exit |
|---|---|---|
| typecheck | `npm run typecheck` (`tsc -p .`) | **0** |
| lint:css | `npm run lint:css` (`stylelint "src/**/*.css"`) | **0** |
| build | `npm run build` (`tsc -p . && vite build`, 2330 modules) | **0** |
| storyboard | `node qa/storyboard.mjs` | **0** |
| visual (63) | `node qa/visual.mjs` → `ALL PASS (63 scenes, source cba74d7)` | **0** |
| cargo test | `cargo test --locked` (src-tauri) → 33 passed; 0 failed | **0** |
| clippy | `cargo clippy --locked -- -D warnings` (src-tauri) → 3 baseline findings, unchanged | **101** (FAIL, baseline-equiv, NOT fixed) |
| cargo build release | `cargo build --release --locked` (prior session; artifact SHA above) | **0** |
| tauri build | `npx tauri build --no-bundle` (prior session; produced running EXE) | **0** |

### Clippy baseline (KEEP visible, do not fix) — 3 findings, all `src-tauri/src/main.rs`
1. `explicit_counter_loop` @ main.rs:419 (`for it in new` + `at`)
2. `explicit_counter_loop` @ main.rs:458 (`for it in new` + `at`)
3. `too_many_arguments` (10/7) @ main.rs:754 (`pub fn place_flyout`)
Identical to pre-patch baseline (see `STOP-REPORT-clippy.md`). Findings count/set unchanged → no STOP.

## 4-file smooth-transition diff state (additive, vs cba74d7)
Modified smooth-transition files:
- `src/rail/Rail.tsx`   raw `22CC24DB…95D63CF2` size 21463 canon `FF0D3696…E2377DF2`
- `src/rail/Tile.tsx`   raw `D0C7AEC…93BE`      size 10326 canon `5454E867…DA83FA6`
- `src/styles/app.css`  raw `EFD51500…57121BCE` size 26020 canon `A265D03E…1430CC`
- `qa/visual.mjs`       raw `57C4E216…33B1C922` size 30536 canon `060DF149…4ACDEB`

Preserved hover-stability files (NOT in the 4-file smooth scope; unmodified this session):
- `src-tauri/src/main.rs`, `src/rail/magnify.ts`

Working tree: 6 modified (4 smooth + 2 hover-stability), untracked `rail-handoff/`, `.github/`,
`docs/rail-hover-stability.zip`. **HEAD still `cba74d7` — no new commits.**

## Native proof results (new EXE, pid 33984, glass/compact, side=right, 2560x1080)
- **Edge reveal**: fires on the new EXE — `IsWindowVisible(rail)=True` on right-edge dwell;
  panel renders glass capsule + ✦/＋/list + folder/dev.rail tiles + live CPU/RAM meters.
- **10 show/hide cycles**: 10/10 pass (`cycles10.ps1`).
- **Duplicate hide** (two real Escape presses via `keybd_event` 0x1B): panel visible at
  esc2+350ms, hid only after the full exit fade — no premature disappearance / stale completion.
- **Rapid re-entry during exit**: re-reveal keeps the panel visible (stays True/True).
- **Reveal-fade brightness ramp** (`rr-0..rr-11`): first-visible @ ~200ms; brightFrac
  `0.934 → 0.83 → plateau 0.824`; monotonic, **no 1.0 white spike, no coloured edge, no trail**.
- **Exit-fade** (`xf-0..xf-91`): brightFrac steady ~0.875 (glass) across all visible frames,
  snapping to 1.0 only in the last ~5 frames immediately before `w.hide()`; no white-flash/trail.
- **Stationary** (`stationary.ps1`):
  - CENTER hold 3s: `False×30` (no spurious reveal, 0 transitions)
  - BOUNDARY top 2s: `False×20`; BOUNDARY bottom 2s: `False×20` (no reveal)
  - FOOTER/panel-body hover 3s: `True×21` then ONE clean auto-hide `False×9` (1 transition, no flicker)
  - diag `region-set` increased by exactly +1 for the single reveal (no reveal-loop spam).
- **Explorer drop / in-rail reorder**:
  - Drop-planning logic verified via cargo tests (`duplicate_only_drop_changes_nothing`,
    `mixed_drop_only_plans_the_new_paths`, `same_basename_*`) → part of 33/33 PASS.
  - Real physical OLE DnD (Explorer file → rail) and framer Reorder drag are **not headless-automatable**;
    synthetic pointerdown reached the rail's `drag_active` LEASE (diag `LEASE drag_active(true/false)`),
    but framer Reorder did not commit under synthetic input. Left to manual native verification (gap).

## Diagnostic log segment (`src-tauri/target/release/rail-diag.log`, RAIL_DIAG=1)
- total lines: **145**; `REGION set` count: **42**; `REGION skip` count: **1**
- Representative entries:
  ```
  [   0ms] HITRECT x=4 y=53 w=55 h=206
  [   0ms] APPLY   force=false drag=false open=true hit=(4.0,53.0,55.0,206.0)
  [   1ms] REGION  set #1 x=-5.0 y=44.0 w=73.0 h=224.0 scale=1.00
  ...
  [2304302ms] LEASE   drag_active(true) changed=false
  [2304573ms] LEASE   drag_active(false) changed=true
  [2304573ms] APPLY   force=true drag=false open=true hit=(4.0,53.0,55.0,206.0)
  [2304573ms] REGION  set #41 x=-5.0 y=44.0 w=73.0 h=224.0 scale=1.00
  [2308635ms] REGION  set #42 x=-5.0 y=44.0 w=73.0 h=224.0 scale=1.00
  ```
- Region stays `x=-5 y=44 w=73 h=224 scale=1.00` (hit `4,53,55,206`); the single skip was
  `already_open` during a drag.

## Captured PNGs (`rail-publish/rail-handoff/native/`)
- Fully-shown / reveal: `reveal-shot-1.png`, `reveal-shot-2.png`, `po-fully-shown.png`,
  `reveal-fade-0..13.png`, `xf-fully-shown.png`
- Partial-opacity (mid-fade): `rr-0.png` (first-visible, brightFrac 0.934), `po-frame-0.png`,
  `cycle3-partial-opacity.png`
- Plateau (fully shown): `rr-11.png`
- Exit-fade series: `xf-0..91.png`, `exit-fade-0.png`
- Duplicate-hide / re-entry: `cycle-dup-esc1+150ms.png`, `cycle-dup-esc2+350ms.png`,
  `cycle-reentry-full.png`
- Reorder attempt: `reorder-before.png`

## NOT fabricated / labelled
- Browser/mock PASS (63 scenes) does not prove Windows compositor quality.
- Cross-DPI coverage: untested (owner at 100%/compact/glass only).
- Physical DnD + in-rail reorder: manual native verification outstanding.
