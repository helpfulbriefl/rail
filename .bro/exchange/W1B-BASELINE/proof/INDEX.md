# PROOF W1B-BASELINE
# Raw gate command logs (logs/npm-ci.log, logs/typecheck.log, logs/lint-css.log,
# logs/build.log, logs/cargo-build.log, logs/qa.log) and run/meters-ref.csv are
# working-tree only, excluded from git per TASK §5.6 ("raw logs", "run").
# Structured provenance records (logs/provenance.log, logs/exe-provenance.log)
# are force-added and are the authoritative gate/build evidence in the repo.
| # | requirement | proof file(s) | ok |
|---|---|---|---|
| 1 | §3.1 Gates: exit codes, locked deps unchanged, storyboard pre-existing bug | logs/provenance.log; logs/exe-provenance.log (structured); raw gate logs in working tree | yes |
| 2 | §3.2 Build: source-built exe from tested_source_commit, exe SHA-256/size, copy equality | logs/exe-provenance.log | yes |
| 3 | §3.3 Native baseline: '+' target/selection, shell drag, reorder, held-hover | proof/native-baseline.md | unavailable (4 INVALID, documented + manual checklists) |
| 4 | §3.4 Meters: 60 s Rail-footer series + independent CPU/RAM reference | proof/meter-trace.md (run/meters-ref.csv working-tree only) | one-sided (independent ref captured; Rail-footer IPC-only, B-004 open) |
| 5 | §3.5 Boundaries: P invariance, Q product/lock diff zero, no auth switch, secret review | logs/provenance.log | yes |
| 6 | §3.6 Deliver: REPORT + INDEX (1 row/criterion), truthful README, snapshot vs tested-source vs final-report SHAs separated | 02-REPORT.md; proof/INDEX.md; README.md | yes |
