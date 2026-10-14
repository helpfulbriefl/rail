---
wave: W3B-FIX · ver: v3.0 · t_start: 2026-10-14 12:02 · t_report: 2026-10-14 22:19
commit: dec4e41 · tag: w3b-fix-done · push: ✅ · bundle: - · pack: - · exchange: -
tasks: 6 · done: 5 · gates_first: no · repair: 2 · proof_ok: 7 · proof_req: 7
diff_files: 14 · diff_add: 1568 · diff_del: 281 · out_of_scope: 0 · q: 0 · contradictions: 0
K1: 1 · K2: 1 · K3: 1 · K4: 1 · k_why: - · friction_cd: -
cd_in: 4341 · cd_out: 20759 · prompt_weight: 4515 · an_in_prev: - · an_out_prev: -
---
# REPORT W3B-FIX — functional correction of the five blocking W3 findings + durable config

## Tasks
1 ⚠️ PARTIAL `src-tauri/src/main.rs:791` — sys flyout is no longer click-through (`win::set_click_through(hwnd, kind == "sys")` inverted at `main.rs:819`), `place_flyout` (`main.rs:760`) clamps size AND position with `hi = max(lo)` so an oversized popup cannot produce an invalid clamp range (2 unit tests); footer opener anchored to the real footer rect with the configured hover delay + click/keyboard (`src/rail/Rail.tsx:163`), Escape closes flyout before the rail, tracker keepalive/away-dismiss via `flyout_rect` (`src-tauri/src/tracker.rs:289`). Native HWND click/X/DPI transfer not automated → PARTIAL, manual checklist below.
2 ✅ `src-tauri/src/tracker.rs:512` — ranking over the complete eligible process set aggregated by full exe path; `sys_snapshot(all, 6, 12)` answers both CPU and RAM genuinely; deterministic ties; tests incl. CPU-heavy low-RAM app outside RAM top-5 (`tracker.rs:572-630`).
3 ✅ `src-tauri/src/tracker.rs:249` — every frame carries `ts`/`seq`; interim windows-only frames carry `sys_top` forward instead of erasing it; UI appends history only on fresh samples and marks data stale after ~6 s (`src/ui/useApp.ts:36`, `src/flyout/Flyout.tsx:67`); rows show CPU % + RAM bytes, footer shows totals + rolling charts; polling stops when both panel and monitor are hidden.
4 ✅ `src-tauri/src/main.rs:569` — `sys_close` takes `SysCloseReq{id, procs}`, revalidates PID+start-time (`win.rs:119`) and window ownership before `WM_CLOSE`, refuses stale targets, protects by full-path identity policy; returns structured `dispatched/refused/unavailable`; UI freezes row membership/order during interaction (`Flyout.tsx:110`).
5 ✅ clipping/jitter fixed in real window geometry (`place_flyout`, region apply) — QA assertions now bite (negative fixture with `translateX(900px)` is detected and inverts the suite); dock bounded with reachable footer and settings footer after scroll; crowded-dock scene; README rewritten with real menu locations, metric definitions and close limits; W3 errata appended (`.bro/exchange/W3-MICRO/04-ERRATA.md`). DPI 100/125/150/200 NOT tested — marked, no universal PASS.
6 ✅ real gates recorded in `logs/`; EXE rebuilt from tested commit; native launch identifies actual PID/path (`logs/native-single-instance.log`: PID 47256, path = `src-tauri\target\release\rail.exe`, sha256 3BF7E2505694FC54CF52F06A84BCBF85F94E4DAB53EB5FD2E599FDBFA8E0EEB5, 4 441 088 B).
Add 1 ✅ durable config: atomic tmp+`sync_all`+rename replace, never delete-first, serialized single writer with revision guard, flush on quit/tray-exit/`RunEvent::ExitRequested`, visible `save-error` chip (`src-tauri/src/config.rs:185`, `core.rs:138`, `main.rs:863`).

## Gates
types ✅ · lint:css ✅ · cargo test 30/30 ✅ · qa 38/38 scenes (6 themes incl. glass/studio, sysmon live/empty/stale/refused, savefail, negative fixture) ✅ · cargo build --release --locked ✅ · native single-instance PID/path ✅ · DPI matrix ❌ untested (marked) · axe — not in this project's gate set · secrets 0

## EXE
path `C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail.exe` · sha256 3BF7E2505694FC54CF52F06A84BCBF85F94E4DAB53EB5FD2E599FDBFA8E0EEB5 · 4 441 088 B · tested_source_commit 8451abbb86ffb1e368d9bfc1ba6274582cb582da (clean tree at build; gates re-run on that tree) · local availability only, not uploaded.

## Manual native checklist (not automated)
real left-click on a sysmon row/X on each side & DPI · footer→popup cursor transfer across the gap · Explorer drag-drop onto revealed rail · unsaved-dialog cancel keeps app alive.

## Spots for analyst (3–5)
`src-tauri/src/main.rs:760` — pure `place_flyout` clamp (size+position, panic-free hi/lo) · `src-tauri/src/core.rs:148` — revision-guarded writer: dirty clears only if revision unchanged since snapshot · `src-tauri/src/tracker.rs:289` — cursor keepalive/away-dismiss for the click-through sys popup · `src/flyout/Flyout.tsx:110` — frozen row ids per tab, values refresh by id · `qa/visual.mjs` — per-axis scroll-aware overflow + `expectFail` negative fixture.

## Q / contradictions
- none

## Out of scope
- `.github/workflows/build.yml` was committed once (8451abb) before noticing TASK §9 "intentionally untracked .github"; restored to untracked via `git rm --cached` in the report commit, file preserved on disk.

Secrets not opened: yes
