---
ver: v3.0 · build: JOIN · wave: W3C-CLOSEOUT (continuation) · prev: W3C-CLOSEOUT
tested_source_commit: c3b9db11b1e4bc5091cbead8aa79a40c10531a26
source_main_at_handoff: c3b9db1
historical_tag: w3c-closeout-done @ 9bb9788 (unchanged) · continuation tag: w3c-closeout-cont-done
exe_path: src-tauri/target/release/rail.exe
exe_sha256: e6e0bf48846844e25be8f86c284ba0600033c94c851223dcbb3b46b936697a2c
exe_size_bytes: 4441600
gates: typecheck PASS · stylelint PASS · vite build PASS · qa/visual 48/48 PASS · storyboard PASS · cargo test 31/31 PASS · cargo release PASS
verdict: PARTIAL (source edge cases fixed + browser/mock sequence PASS; native physical checks pending Owner)
contradictions: 0
---
# W3C-CLOSEOUT continuation — report

Minimal same-wave continuation only, inside the existing W3C allow-list
(`src/flyout/Flyout.tsx`, `src/bridge/**`, `qa/**`, `.bro/**`). No redesign, no
new wave, no CSP/manifest/dependency/security-config change, no baseline-only
follow-up. Original `02-REPORT.md`, `proof/INDEX.md`, errata, `.github` and the
`w3c-closeout-done` tag were left untouched; all corrected evidence is additive
(`proof/INDEX-CONTINUATION.md`, `06-OWNER-CHECKLIST.md`, this report).

## Source changes (all in `src/flyout/Flyout.tsx` + mock/QA seams)

1. **Empty frame never latches the freeze.** `arm` snapshots the ranking only
   when there are real rows; rows arriving while the pointer/focus is already
   inside render without a tab switch or mouse exit, then stabilise for the
   ongoing interaction. Closes accept case "arm accepts empty liveIds".
2. **Independent pointer/focus/press ownership.** `pointerIn`, `focusIn` and
   `press` each hold the freeze; it is released only when **all** guards end.
   `onBlur`/`onFocus` use `contains(relatedTarget)`, so a pointer-leave while
   the keyboard still holds focus, and an internal Tab between rows, no longer
   unfreeze. Close capture and backend revalidation kept. Closes accept case
   "onPointerLeave/onBlur release unconditionally".
3. **Tombstones survive total disappearance.** The empty branch now keys on the
   rendered slot count, so while a guard is held the frozen slots stay put even
   when every current candidate vanishes; close stays disabled, positions kept.
   Closes accept case "apps.length===0 bypasses frozen tombstones".
4. **Loading placeholders, not zeros.** The footer totals render `—` until the
   first valid system sample instead of formatting an initial 0% / 0 B. Closes
   accept case "noSample totals still format zero".

No layout, theme or aesthetic change.

## QA — real event sequences, not screenshots
`qa/visual.mjs` gained six behavioural scenes driving genuine
pointer/focus/keyboard sequences against scripted mock fixtures (each fixture is
off unless its query flag is set, so every pre-existing scene is unchanged):
`seq-hover-cold`, `seq-leave-focus`, `seq-tab-focus`, `seq-all-vanish`,
`seq-loading-totals`, `seq-new-leader` — all PASS (48/48, `logs/qa-visual2.log`).
The `tab-focus` scene tabs onto a row that *survives* the disappearance so the
focused node is not unmounted — it isolates the "internal Tab must not release"
rule rather than a focus-loss-on-removal artefact.

## Honest scope
- Native HWND / WM_CLOSE / DPI / real Explorer-drop evidence was **not** produced
  by the coder. Browser/mock sequence tests are not substitutes. Rows 1–2 of the
  index are PASS at the browser/mock level and **PARTIAL** at the native level;
  row 4 is **PARTIAL** until the Owner checkoff.
- Persistence cleanup (row 3) was not touched this wave; its cargo tests still
  pass.
- The rebuilt binary is available locally only; nothing was uploaded to a GitHub
  release.

## Rebuilt executable
- `tested_source_commit` `c3b9db1` (clean tree; the release was built from it)
- path `src-tauri/target/release/rail.exe`
- SHA-256 `e6e0bf48846844e25be8f86c284ba0600033c94c851223dcbb3b46b936697a2c`
- size `4441600` bytes
- provenance `logs/exe-provenance2.log`

## Owner summary (RU)
Сделано и проверено в браузере/моке: четыре граничных случая взаимодействия из
`03-ACCEPT` исправлены (пустой кадр не фиксируется; pointer/focus/press
независимы — уход курсора при активном фокусе и внутренний Tab фиксацию не
снимают; tombstone-места сохраняются при исчезновении всех приложений; до первого
замера в итогах прочерки, не нули). Добавлены проверки реальных последовательностей
событий (6 сцен), QA 48/48, cargo 31/31, release собран.

Осталось PARTIAL: физические native-проверки (HWND/WM_CLOSE, DPI, реальный
drag-drop из Explorer) не выполнены — они переданы Owner в
`06-OWNER-CHECKLIST.md` для **одного** чек-листа на конкретный бинарник выше.
Native-вердикт остаётся PARTIAL до фактического подтверждения.
