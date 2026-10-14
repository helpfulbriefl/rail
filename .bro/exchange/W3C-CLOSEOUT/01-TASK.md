---
ver: v3.0 · build: JOIN · wave: W3C-CLOSEOUT · prev: W3B-FIX · layer: product
tasks: 4 · pass_criteria: 4
allow: src/flyout/Flyout.tsx; src/bridge/**; src/ui/useApp.ts; src-tauri/src/config.rs; qa/**; README.md; .bro/**
modules: METRICS · mirror: - · risky: yes
---
# W3C-CLOSEOUT — minimal UI/persistence corrections and native evidence

Read BOOT, MAP/NEXT, W3B/03-ACCEPT and original W3B requirements. Product baseline cf92b2b680285b9ca31f46d183e17f26954ac08f; preserve accepted implementation, themes/default Glass, working drag/drop, Owner config and P. This is executable product follow-up, not recon or redesign. Existing ignored BRO tooling unchanged; mirror:-. Safe fetch/ancestry/--ff-only analyst docs; conflicts or unexpected local changes -> preserve/report, no reset/stash/clean/rebase/force. Original reports/index/tags immutable. .github not authorized. If real native tests reveal another source file must change, request exact allow amendment rather than edit outside list.

## Task / proof row 1 — timestamp and initial loading
Use backend ts in its actual milliseconds unit, one documented frontend contract and threshold in same units. Prefer monotonic elapsed time since a fresh seq/ts arrival for visible stale timer; handle absent/invalid sample explicitly, retain valid values rather than fabricate zeros. Detect stale after events stop, not only during rerender. Timer cleanup on hide/unmount. Test fresh, carry-forward, threshold boundary, stopped frames and resume. Cold popup opened before first sample must render first arriving rows automatically, without tab switch/reopen. Empty/loading/nonempty transitions all covered in existing QA fixtures, no dependency addition.

## Task / proof row 2 — five rows and interaction-scoped freeze
Show up to five eligible apps, true selected CPU/RAM ranking. Backend top-six union candidates need not be rewritten just to impose UI five. Outside pointer/focus/press interaction, update live ranking for new leaders; freeze layout/target membership while someone aims at or presses close. Keep stable slots/disabled disappeared-row placeholders during that interval, don't filter them away and shift neighbor X. Resume/update when interaction ends; keyboard equivalent. Capture intended PID/start-time snapshot at press/action boundary and keep current native revalidation. Freeze must not latch an empty snapshot forever or keep old ranking for entire popup. Test >5 apps, new leader, disappearing row while pressed, cold start, both tabs, keyboard focus and normal structured close results. No forced kill.

## Task / proof row 3 — persistence cleanup and honest scope evidence
Ensure temporary file cleaned on create/write/sync/replace failure using existing std mechanisms/scoped guard; preserve original config and do not delete arbitrary or Owner data. Test failure cleanup and same-destination last-valid preservation in isolated fixtures. Current test targeting a different directory destination is not sufficient evidence for that claim. Retain serialized/revision-guarded writer and graceful flush; test latest-state save on controlled graceful exit. No promise of universal power-loss durability.

Append accurate W3B errata: workflow was committed/published at 8451abb then removed; out-of-scope event is not zero. Do not re-add workflow, force-rewrite commits/tags or alter original REPORT. Correct stale/6-row/empty/freeze defects and evidence claims, not counts invented by helper. Update README only affected semantics/launch/version instructions. Existing native-unavailable acceptance rows remain PARTIAL until actual proof.

## Task / proof row 4 — rebuilt executable and native checkoff
Run actual locked typecheck/CSS lint/build/Rust tests/QA with logs. Use installed local build tools, no installs/upgrades/locks/manifests/CSP/capabilities changes. Rebuild and identify exact tested_source_commit, EXE path/PID/size/SHA-256; ensure no old single-instance process is being tested. Never kill Owner's working apps.

On native Windows check: hover footer->popup across gap, click CPU/RAM and X really received, Escape/outside dismissal; five valid rows/continuous samples/new ranking; disposable app WM_CLOSE preserves unsaved-dialog cancel and stale target refuses; ordinary and crowded rail not clipped with stationary hover settled; real Explorer .lnk/folder drop and UI reorder persist after restart; config change then immediate graceful quit/restart. Check available DPI100/125/150/200/sides/monitor cases and state coverage honestly. Use existing stand for doable interactions. If a specific physical scenario needs Owner interaction, deliver ONE short consolidated checkoff list with exact tested binary, expected results and unavailable reason; do not manufacture another automation-only baseline wave or wait for an entirely new design approval. Native smoke launch/negative CSS/mock tests are not substitutes. No native evidence -> PARTIAL, not FAIL hidden behind all-green INDEX.

Deliver four-row proof INDEX with appropriate PASS/PARTIAL per requirement, report actual gates, applied source changes, scope deviation, untested scenarios and exact local binary availability (upload not implied). Existing source build may be offered for Owner testing, not labelled finished. Same approved command-local GCM args and host-only URL; transient env restored in finally. Explicit staging; no git add -A, token/account switch/keyring extraction/credential fill/global changes/force/tag moves. Publish new allowed commits, verify remote/local SHA. Stop on push conflict/auth failure without bypass.

Owner summary RU: implemented vs verified, remaining PARTIAL, exact new EXE path/hash, consolidated native checks. End: Готово → напиши аналитику: BRO CHECK
