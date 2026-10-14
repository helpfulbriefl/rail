---
wave: W3C-CLOSEOUT · ver: v3.0 · t_start: 2026-10-14 23:00 · t_report: 2026-10-14 23:45
commit: 9bb9788 · tag: w3c-closeout-done · push: ❌ · bundle: - · pack: - · exchange: -
tasks: 4 · done: 3 · gates_first: yes · repair: 0 · proof_ok: 3 · proof_req: 4
diff_files: 6 · diff_add: 214 · diff_del: 42 · out_of_scope: 0 · q: 0 · contradictions: 0
K1: 1 · K2: 1 · K3: 1 · K4: 1 · k_why: - · friction_cd: -
cd_in: 4390 · cd_out: 5048 · prompt_weight: 4515 · an_in_prev: - · an_out_prev: -
---
# REPORT W3C-CLOSEOUT — timestamp contract, live top-five, interaction freeze, temp cleanup

## Tasks
1 ✅ `src/bridge/types.ts:96` — one documented contract: `ts` is unix **milliseconds** (backend `config::now` unit); mock emits the same unit; stale check `Date.now() - rt.ts > STALE_MS` ([`Flyout.tsx:79`](src/flyout/Flyout.tsx:79), `STALE_MS = 6000`) in the same units. Staleness is driven by a 1 s ticking clock, so it flips after events stop even without a rerender; absent sample (`ts <= 0`) is an explicit "Ожидание первых данных…" loading state, never stale, never fabricated zeros; valid values are retained and only marked. Timer cleaned up on unmount. QA: fresh, carry-forward (repeated `ts`), stopped frames (`stale=1`, 60 s old), threshold-adjacent (`recent=1`, 4 s old — must NOT be stale), cold open (`cold=1`: two empty frames then first rows arrive automatically, no tab switch).
2 ✅ `src/flyout/Flyout.tsx:105` — live top-five: every frame recomputes the selected ranking from the full snapshot (`slice(0, 5)`), so new leaders enter and exited apps leave outside interaction. Freeze is interaction-scoped only: armed by aim (pointer enter), press (pointer down) or keyboard focus, released by pointer-leave / focus-leave / press-end-outside — never latched for the popup lifetime. During freeze an exited app becomes a disabled tombstone (`data-gone`) instead of being filtered out, so neighbour slots and the X under the cursor never shift. Close identity (full path + PID/start-time set) is captured at the press/focus boundary ([`Flyout.tsx:259`](src/flyout/Flyout.tsx:259)) and the backend keeps native PID+start-time revalidation. QA: >5 candidates capped at exactly 5 (both tabs, CPU tab driven through the real tab button), cold start, keyboard Tab reaches the close button, structured close results (refused/unavailable scenes).
3 ✅ `src-tauri/src/config.rs:196` — scoped temp cleanup: `write_tmp` (create/write/sync) failure and rename failure both run `remove_file(tmp)`; the old config is never unlinked. Same-destination preservation is now tested at THAT destination: `write_failure_preserves_the_same_destination_config` occupies the deterministic temp path so the save fails before the destination is touched and asserts the existing config survives verbatim ([`config.rs:333`](src-tauri/src/config.rs:333)); `replace_failure_cleans_temp_and_leaves_destination` asserts the temp is cleaned on the rename-failure path ([`config.rs:314`](src-tauri/src/config.rs:314)). Serialized revision-guarded writer and graceful flush retained; latest-state save on immediate flush covered by the existing `change_then_immediate_flush` test. No universal power-loss durability is claimed.
4 ⚠️ PARTIAL `logs/native-single-instance.log` — EXE rebuilt from tested corrective commit `9bb9788` with `--locked`; native launch identifies the actual PID/path (PID 54188, path = `src-tauri\target\release\rail.exe`, sha256 `80D7AE0B9D53B0E4C9A81F91EBA95980669D7151404AFAE9C57E4D4D41F49CA5`, 4 441 088 B, no pre-existing instance). Native HWND-level interactions (footer→popup across the gap, real X/WM_CLOSE with unsaved-dialog cancel, Escape/outside dismissal, DPI 100/125/150/200, real Explorer .lnk/folder drop, config-change + graceful quit/restart) are NOT automated here → PARTIAL, consolidated Owner checklist below. Native smoke launch and mock/CSS tests are not substitutes and are not claimed as such.

## Gates
types ✅ · lint:css ✅ · cargo test 31/31 ✅ · qa 42/42 scenes (6 themes; 8 sysmon scenes incl. crowd/cold/recent/CPU-tab) ✅ · cargo build --release --locked ✅ · native single-instance PID/path ✅ · DPI matrix ❌ untested (Owner checklist) · secrets 0

## EXE
path `C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail.exe` · sha256 80D7AE0B9D53B0E4C9A81F91EBA95980669D7151404AFAE9C57E4D4D41F49CA5 · 4 441 088 B · tested_source_commit 9bb9788 (corrective commit; QA 42/42, Rust 31/31 and release build re-run against this source) · local availability only, not uploaded.

## Consolidated Owner native checklist (physical interactions, exact binary above)
1. Навести курсор на CPU/RAM в футере → поп-ап появится через ~1.2 с; провести курсор через зазор → поп-уп остаётся (keepalive); увести курсор на ≥1.2 с → прячется.
2. Клик по × в строке монитора → `×N`; приложение с несохранёнными данными → отмена диалога сохраняет приложение живым; устаревшая цель (PID/start-time не совпали) → «нельзя»/«нет».
3. Удерживать курсор на строке, пока приложение закроется извне → строка остаётся на месте (tombstone), соседи не сдвигаются; отпустить курсор → живое ранжирование возобновляется, новый лидер входит в топ‑5.
4. Escape → сначала закрывает поп-ап, потом рельсу; клик вне поп-апа → dismissal.
5. DPI 100/125/150/200 × все четыре стороны: без клиппинга; увеличение при наведении стабильное, курсор не дёргается.
6. Настоящий drag .lnk/папки из Explorer на край экрана → элемент добавляется; перестановка в UI сохраняется после перезапуска.
7. Сменить тему в настройках → трей «Выход» (graceful quit) → перезапуск → тема сохранена (config change + immediate graceful quit/restart).

## Spots for analyst (3–5)
`src/flyout/Flyout.tsx:79` — ms-contract stale threshold + ticking clock · `src/flyout/Flyout.tsx:105` — live top-five selection · `src/flyout/Flyout.tsx:150` — interaction-scoped arm/release on the list · `src/flyout/Flyout.tsx:259` — identity capture at press boundary · `src-tauri/src/config.rs:196` — scoped temp cleanup on every failure path.

## Q / contradictions
- none

## Out of scope
- none in the final W3C diff. `bro finish` initially detected a tombstone rule in out-of-scope `src/styles/app.css`; corrective commit `9bb9788` restored that file to the pre-W3C state and moved the presentation into allowed `src/flyout/Flyout.tsx`. The already-published `w3c-closeout-done` tag was not moved or rewritten. No dependency/lock/manifest/CSP/capabilities changes; `.github` untouched and untracked.

Secrets not opened: yes
