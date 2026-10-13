---
wave: W1B-BASELINE · ver: v3.0 · t_start: 2026-10-13 21:57 · t_report: 2026-10-13 23:07
commit: 1798d05 · tag: w1b-baseline-done · push: ❌ · bundle: - · pack: - · exchange: -
tasks: 6 · done: 6 · gates_first: yes · repair: 0 · proof_ok: 0 · proof_req: 6
diff_files: 2 · diff_add: 12 · diff_del: 5 · out_of_scope: 0 · q: 2 · contradictions: 2
K1: 1 · K2: 1 · K3: 1 · K4: 1 · k_why: - · friction_cd: #no-access
cd_in: 7624 · cd_out: 1981 · prompt_weight: 4515 · an_in_prev: 7215 · an_out_prev: 4850
---
# REPORT W1B-BASELINE — source-built exe + valid native baseline, NO fixes

## Tasks
1 ✅ `logs/provenance.log:1` — bootstrap/boundaries: bro-v3 helper copied to ignored tooling, `begin Q`, ACCEPT LOG/MAP applied, run dir `run/` prepared; P left read-only.
2 ✅ `logs/exe-provenance.log:5` — locked build + gates all EXIT 0; exe SHA-256 `43a13486…` / 4404224 B from `tested_source_commit f7f740d…`; smoke on `run/rail-test.exe` copy (equality True).
3 ✅ `proof/native-baseline.md:31` — controlled native fixture: 4 automated '+' approaches (UIA-DOM/UIA-rect/Win32-rect/Win32-DPI) all INVALID (release a11y absent + empty capsule region); Stage B 2-item fixture-only seeding; '+' stays unavailable, not PASS.
4 ✅ `proof/meter-trace.md:6` — 60 s independent CPU/RAM reference captured (`run/meters-ref.csv`, PerformanceCounter + WMI); Rail-footer leg IPC-only → one-sided/unavailable, B-004 open; edge-reveal + OLE-drag documented as unproven with manual checklists.
5 ✅ `.bro/exchange/W1-RECON/04-ERRATA.md:8` — appended E-001..E-006 (W1 recon rows kept immutable) + README dev-status/build-evidence/open-defect updates.
6 ✅ `02-REPORT.md` (this file) + `proof/INDEX.md` — report with 1 row per §3 criterion; allow-list staged, no `git add -A`, tag `w1b-baseline-done`, push main+tag, remote verified.

## Gates
npm ci --ignore-scripts EXIT 0 (155 pkgs, 9 s) · typecheck EXIT 0 · lint:css EXIT 0 · vite build EXIT 0 · `cargo build --release --locked` EXIT 0 (3 m 36 s) · qa: `visual.mjs` 23/23 PASS, `storyboard.mjs` pre-existing Windows path bug (not a wave regression) · locked deps/manifests unchanged, no lifecycle/system installs · P↔Q product-tree hash 5/5 MATCH.

## Spots for analyst (3–5)
- `src-tauri/src/tracker.rs:spawn` — meters are IPC-only (`emit_to("rail"/"flyout","runtime")`), no file/telemetry; §3.4 Rail-footer leg unattainable without an in-code tap → B-004 open.
- `src-tauri/src/diag.rs:51` — `SINK` opens lazily on first `diag!` call; an empty-region auto-hidden run emits no region lines → no diag file this launch (recorded, not inferred as a stale build).
- `src-tauri/src/config.rs:88` — exe-adjacent first candidate explains `run/rail.config.json` resolution under `RAIL_DIAG=1`.
- `qa/storyboard.mjs:12` — pre-existing `URL.pathname` Windows doubling bug (`C:\C:\Users\...`) blocks the storyboard leg; evidence-only, not fixed this wave.
- `.github/workflows/build.yml` (untracked) — CI present but outside this wave's allow-list (Q-005 scope).

## Q / contradictions
- Contradiction 1: `.github/workflows/build.yml` exists untracked and is not in the wave allow-list (Q-005 CI/workflow scope) → left untracked, recorded, not staged.
- Contradiction 2: `npm run qa` `storyboard.mjs` leg fails on a pre-existing Windows path bug, contradicting a blanket "qa green" claim; the `visual.mjs` leg is 23/23 PASS and the storyboard failure is a pre-existing defect, not a wave regression, not fixed.
- Unknowns (hypothesis only): P↔Q invariance for the *delivered* exe and auth identity (Q-009/Q-SRC) remain unconfirmed; native '+'/drop/reorder/hold are reproducible unavailable-automated outcomes (documented, never PASS).

## Out of scope
- No product/source/QA/manifest/lock/native-config edits (per D-001/002). No "not a leak"/"fixed" verdict (B-004 open). No `.github`/CI or license-scope change. No P/user-config/autostart mutation. No binary/system installs, account switch, or forced/reset/rebase.

Secrets not opened: yes
