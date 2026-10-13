---
wave: W1-RECON · ver: v3.0 · t_start: 2026-10-13 21:01 · t_report: 2026-10-13 21:09
commit: 2e5e014 · tag: w1-snapshot · push: ✅ · bundle: - · pack: - · exchange: -
tasks: 6 · done: 6 · gates_first: yes · repair: 0 · proof_ok: 11 · proof_req: 12
diff_files: 0 · diff_add: 0 · diff_del: 0 · out_of_scope: 0 · q: 3 · contradictions: 2
K1: 1 · K2: 0 · K3: 1 · K4: 1 · k_why: K2=0: SPEC §9.1/§9.2/§9.10 claim "closed" but Owner still reports drop/+/RAM broken, and α drifts (SPEC 0.35 vs code 0.25) → doc/code contradiction. · friction_cd: #no-access
cd_in: 5582 · cd_out: 1863 · prompt_weight: 4515 · an_in_prev: - · an_out_prev: -
---
# REPORT W1-RECON — publish a safe baseline and diagnose, without fixes

## Tasks
1 ✅ `recon/01_chronology.md` — preflight read-only: resolved SYS, read rules, recorded env (Win11 Pro 26300, PS 5.1.26100.9444, Node v24.16.0, cargo 1.98.1, git 2.54.0, non-admin, DPI 96, 2560×1080); exe SHA-256 `22B21D28…` (2026-10-04 23:25:01 +03). P↔exe mapping UNKNOWN (Q-SRC/Q-009). No P change.
2 ✅ `recon/02_repomap.md` — safe export: 48 product files hash-identical P→Q; fresh clone Q @ `ef1a3c1`, tag `w1-snapshot`; pushed `main` + tag only (`Q\push-main.log`, `Q\push-tag.log`); P `.git`/`.bro` untouched. REPORT commit `2e5e014` published to remote `main` and verified separately. Q-005: CI `build.yml` excluded (token lacks `workflow` scope).
3 ✅ `docs/prompt-system/` — prompt snapshot: 34 generic files + `MANIFEST.sha256` + `SOURCE.md` (HEAD `e35e7e2…`); `.git/`, `docs/legacy/`, `metrics/*` excluded; secret scan 0.
4 ✅ `proof/native-baseline.md` — native stand on delivered `Rail.exe`: open panel (edge reveal), empty-panel CPU/RAM (22%→21% sys, 30552→30474 MB), «+» ×3 (no dialog on stale binary), drop+reorder+hover-card → Q-008/Q-009 unavailable, not PASS.
5 ✅ `recon/03–09` + `.bro/MAP.md` + `README.md` — truth map, stack, alive/dead, config/secrets, leftovers, gates baseline (build/lint/qa BLOCKED, no deps), project Q&A. README now states dev status + real build steps + open defects.
6 ✅ `proof/INDEX.md` + this REPORT — BRO state in Q; analyst `01-TASK.md` / `.bro/inbox/NEXT.md` preserved untouched; no mirror/retention run (D-006); out-of-scope edits = 0.

## Gates
native-stand ✅ (exit 0, ~33 s) · secrets 0 ✅ · hash P→Q 48/48 ✅ · typecheck BLOCKED (no node_modules) · build BLOCKED · qa:visual BLOCKED · qa:storyboard BLOCKED · cargo build NOT run · CI not a local gate
- Blocked = dependency install not authorized this wave (TASK §4). Not failures.

## Spots for analyst (3–5)
- `src-tauri/src/win.rs:308` — `set_click_region()` `SetWindowRgn` region must stay open during any OLE drag; the B-001 mechanism, but the *reproducing drop variant* is unestablished (Q-008).
- `src-tauri/src/main.rs:419` — `pick_files` callback dialog path is correct in source; the *delivered exe* doesn't run it (stale binary, Q-009).
- `src-tauri/src/tracker.rs:102` — system-meter EMA `A=0.25` (SPEC §9.10 says 0.35) — B-004 is a *system* counter following the OS, not a Rail-process leak.
- `rail.exe` (P root, 4,218,880 B) vs `P\Rail-source\src-tauri\target\release\rail.exe` (4,391,424 B) vs `debug\rail.exe` (17,223,168 B) — three different binaries, none with build provenance; delivered one is the oldest (Q-SRC).
- `recon/06_config_secrets.md` — Q-005: `.github/workflows/build.yml` on-disk-but-untracked in Q; re-publish needs a `workflow`-scoped token or classic PAT.

## Q / contradictions
- **Q-005** CI `build.yml` excluded from public snapshot (OAuth token lacks `workflow` scope). Recorded, not fixed.
- **Q-008** external OLE drop + internal reorder + hover-card cannot be automated (no live drag source; config has 0 items). Marked unavailable, not PASS.
- **Q-009 / Q-SRC** delivered `Rail.exe` predates `diag.rs` and the `rail_cross()` refactor (168 px strip, no `RAIL_DIAG` log). Defect repros must be redone against a Q-source build.
- **Q-LICENSE** MIT proposed per D-005; no `LICENSE` file, no owner asserted.
- **Q-ENV** WebView2 EKR not found at the 3 standard registry paths (Tauri likely bundles it); confirm via `Win32_VideoController` + runtime before concluding.
- **Contradiction 1:** `docs/SPEC.md §9.10` "EMA α=0.35" vs `tracker.rs:102` `0.25`.
- **Contradiction 2:** `docs/SPEC.md §9.1/§9.2/§9.10` "closed" vs Owner still reporting B-001/B-003 broken (stale-claim, B-007).

## Out of scope
- 0. No product source, QA, manifest, native-config, or executable change in P or Q. No dependency install. No mirror/retention. `push-main.log`/`push-tag.log` are git push stdout, gitignored (`*.log`), not committed.

Secrets not opened: yes
