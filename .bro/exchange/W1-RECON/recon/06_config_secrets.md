# 06 · config & secrets

## Config surface
| key | where | notes |
|---|---|---|
| `rail.config.json` | `P\rail.config.json` (portable, next to exe) | **excluded from Q** (per-machine runtime data, D-005). Parsed in `config.rs:88` `candidates()`: exe-dir first, then `%APPDATA%\Rail`. |
| `rail.config.json` keys | `Settings` struct (`config.rs:7-20`) | `theme, side, autostart, edgeReveal, hoverDelayMs, hideDelayMs, iconSize, pinned, reduceMotion, railScale, showMeters`. No credentials. |
| `rail-diag.log` | next to config | written only when `RAIL_DIAG=1` (`diag.rs`). Not present on delivered exe (see 05). |
| `.github/workflows/build.yml` | P source | CI. No secret *values* in-file; uses GitHub-provided env (`GITHUB_TOKEN`). Excluded from public Q snapshot (Q-005). |
| registry (autostart) | `tauri-plugin-autostart` | `HKEY_CURRENT_USER\...\Run` when `autostart=true`. Not a file secret. |

## Secret-shape review
- W1-RECON PATTERN scan over Q text files: **0 hits** (44 text files). See `proof/secret-scan` notes in 08.
- `git grep --untracked` for `ghp_ / github_pat_ / AKIA / sk- / BEGIN PRIVATE KEY`: clean.
- No `.env*`, `*.pem`, `*.key` in P source. `**/.auth/` not present.
- The only "secret-ish" thing is the **private** `global-promt-system` repo; its published copy is `docs/prompt-system/` (redacted generic files + SHA-256 manifest + `SOURCE.md`), **not** the private git history (D-003). No credential-bearing remote URL is printed anywhere in this report.

## Coverage by .gitignore
`.gitignore` (Q, wave-created) covers: `node_modules/`, `dist/`, `src-tauri/target/`, `src-tauri/gen/`, `rail.config.json`, `playwright-report/`, `test-results/`, `qa/shots/`, `*.log`, `.env*`, `*.pem`, `*.key`, `**/.auth/`, bro proof media, OS junk. All secret-shaped paths are ignored.
