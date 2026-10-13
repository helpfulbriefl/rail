# 08 · gates baseline

Existing commands only, run under Q after inspecting side effects. No dependency install this wave (TASK §4: "If safe dependencies are unavailable, report blocked").

**Gate prerequisites in Q:** `node_modules/` **absent**, `src-tauri/target/` **absent** (both excluded from the public snapshot and install not authorized this wave). Therefore the JS/CSS/build gates are **BLOCKED**, not failed. The native exe used for the stand was the *existing* `P\Rail.exe` (no rebuild).

| gate | command (package.json / existing) | exit | pass/fail | time | note |
|---|---|---|---|---|---|
| typecheck | `npm run typecheck` (`tsc -p .`) | blocked | n/a | — | `node_modules` missing; install not authorized |
| css lint | `npm run lint:css` (stylelint) | blocked | n/a | — | `node_modules` missing |
| frontend build | `npm run build` (`tsc && vite build`) | blocked | n/a | — | `node_modules` missing |
| visual QA | `node qa/visual.mjs` | blocked | n/a | — | needs `node_modules` + Playwright Chromium |
| storyboard QA | `node qa/storyboard.mjs` | blocked | n/a | — | same |
| native build | `cargo build` / `npm run tauri` build | blocked | n/a | — | would compile Q source; not run (no authorized native build this wave; no `cargo` dep fetch side-effect review done) |
| **native stand** | `diag\stand-baseline.ps1 -Tag b1` | 0 | **baseline recorded** | ~33 s | see `proof/native-baseline.md`; exit 0, all steps logged |
| **secret scan** | `diag\secret-scan.ps1` over Q text | 0 | **0 hits** | ~s | W1-RECON PATTERN; see `proof/` |
| prompt manifest | `diag\prompt-snapshot.ps1` | 0 | ok | ~s | 34 files + SHA-256 MANIFEST + SOURCE.md |
| hash P→Q | `diag\hash-compare.ps1` | 0 | ok | ~s | 48 product files hash-identical |
| CI | `.github/workflows/build.yml` | not run | — | — | excluded from public snapshot (Q-005); not a local gate |

**Conclusion:** the only gates with a *result* this wave are the native stand, the secret scan, and the P→Q hash comparison — all pass. All build/lint/QA gates are **BLOCKED** (dependency install not authorized), not failed. No `>10 min` unbounded command was run. No lifecycle script was executed before review.
