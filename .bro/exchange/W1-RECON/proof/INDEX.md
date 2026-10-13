# PROOF W1-RECON

Maps each TASK claim to the artifact that proves it. Backticks wrap **only** proof files (must exist under the wave dir or repo root). All other identifiers are plain text. Local-only diagnostic scripts (P\diag\secret-scan.ps1, P\diag\hash-compare.ps1, P\diag\prompt-snapshot.ps1, P\diag\stand-baseline.ps1) and the P tree are referenced in prose; they are not published to Q.

| # | requirement | proof file(s) | ok |
|---|---|---|---|
| 1 | Public target = sanitized full source tree; source files hash-identical to P (48/48) | `recon/02_repomap.md` `recon/01_chronology.md` | ✅ |
| 2 | P product files + P git/creds/remotes unchanged (P has no VCS) | `recon/01_chronology.md` | ✅ |
| 3 | Secret review before publication: 0 unresolved hits | `recon/06_config_secrets.md` | ✅ |
| 4 | Prompt snapshot = reviewed generic files + SHA-256 manifest + source HEAD e35e7e2 | `docs/prompt-system/MANIFEST.sha256` `docs/prompt-system/SOURCE.md` | ✅ |
| 5 | Snapshot tag w1-snapshot on main; REPORT commit verified on remote main; no push-all-tags | `recon/01_chronology.md` `README.md` `logs/begin.md` | ✅ |
| 6 | Recon 01-09 + proof/INDEX + MAP + REPORT published; unavailable evidence marked unavailable, never PASS | `recon/03_truth_map.md` `recon/04_stack.md` `recon/05_alive_dead.md` `recon/06_config_secrets.md` `recon/07_leftovers.md` `recon/08_gates_baseline.md` `recon/09_project_questions.md` `.bro/MAP.md` `proof/INDEX.md` | ✅ |
| 7 | Native baseline: open panel + empty-panel CPU/RAM (independent WMI ref) + «+» x3 + timestamps + tested exe SHA-256 | `native-baseline.md` `stand-b1.log` `shot-revealed.png` `shot-nodialog-1.png` `shot-hover.png` | ✅ |
| 8 | External OLE drop / internal reorder / hover-card NOT automatable this stand (no live drag source; 0 config items) -> marked unavailable, not PASS (Q-008/Q-009) | `native-baseline.md` | — |
| 9 | Delivered Rail.exe is a stale binary (168px strip, no RAIL_DIAG log) -> defects must be re-validated on a Q-source build | `native-baseline.md` `recon/05_alive_dead.md` `recon/09_project_questions.md` | ✅ |
| 10 | Honest README: dev status + real build steps + known defects, not "fixed"/"release-ready" | `README.md` | ✅ |
| 11 | Gates = existing commands with exit code + duration; unrun = blocked, not failed | `recon/08_gates_baseline.md` | ✅ |
| 12 | Out-of-scope edits = 0 (all edits within the TASK allow-list: .bro/**, .gitignore, .gitattributes, README.md, docs/prompt-system/**) | `recon/07_leftovers.md` `logs/begin.md` | ✅ |

## Notes (exclusions, plain text - not proof rows)
- Raw desktop video: not published (D-005). Private global-promt-system git history / private metrics: not imported (D-003/D-005). Local diagnostic dumps stay in P\diag, referenced above, not committed to Q.
- CI .github/workflows/build.yml is on-disk but untracked in Q this wave (Q-005, token lacks workflow scope); re-publish needs a workflow-scoped token or classic PAT.
