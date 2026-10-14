# W3C-CLOSEOUT continuation — corrected evidence index (additive)

This index is **new** and additive. The original `proof/INDEX.md` and
`02-REPORT.md` are left immutable as historical evidence of the `c23b31c`
delivery. It records only the minimal same-wave continuation that closed the
four interaction edge cases named in `03-ACCEPT.md`, on
`tested_source_commit c3b9db1`.

Verdict legend: **PASS** = proven by a committed artefact; **PARTIAL** = source
implemented and mock/browser-proven, native physical proof still pending.

| # | Requirement (W3C) | This continuation | Evidence | Verdict |
|---|-------------------|-------------------|----------|---------|
| 1 | ms timestamp + initial loading | ms contract and ticking stale timer preserved (unchanged); totals now render **placeholders until the first valid sample** instead of zeros; rows arriving while the pointer is already inside render without a tab switch | `src/flyout/Flyout.tsx` (footer `noSample` branch, `arm` no-latch); `seq-loading-totals-glass` PASS; `seq-hover-cold-porcelain` PASS; `logs/qa-visual2.log` | **PASS** (browser/mock) · real-OS sampling cadence **PARTIAL** |
| 2 | five rows + interaction-scoped freeze | the four accepted edge cases fixed: (a) empty frame never latches the freeze; (b) independent pointer/focus/press ownership — pointer-leave while focus remains, and an internal Tab, no longer release; (c) frozen slots kept when **all** apps disappear; (d) close identity captured at press/focus boundary (unchanged) | `src/flyout/Flyout.tsx` (`pointerIn`/`focusIn`/`press` guards, `contains(relatedTarget)`); `seq-leave-focus-glass`, `seq-tab-focus-studio`, `seq-all-vanish-aurora`, `seq-new-leader-porcelain` PASS | **PASS** (browser/mock) · native HWND/WM_CLOSE freeze across a real close **PARTIAL** |
| 3 | persistence cleanup | unchanged this wave; revision-guarded writer + scoped temp cleanup retained | `cargo test` 31/31 incl. `replace_failure_cleans_temp_and_leaves_destination`, `write_failure_preserves_the_same_destination_config` (`logs/cargo-test2.log`) | **PASS** |
| 4 | rebuilt executable + native checkoff | clean release rebuilt from `c3b9db1`; provenance recorded; **one** consolidated Owner physical checklist issued | `logs/exe-provenance2.log`; `06-OWNER-CHECKLIST.md` | **PARTIAL** (Owner physical checks not yet performed) |

Native status: no native HWND / WM_CLOSE / DPI / real Explorer-drop evidence was
produced by this coder. Browser and mock sequence tests are **not** substitutes
for those physical checks; rows 1–2 stay PARTIAL at the native level and row 4
stays PARTIAL until the Owner checkoff on the exact binary in
`logs/exe-provenance2.log`.
