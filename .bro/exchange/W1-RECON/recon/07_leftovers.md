# 07 · leftovers (delete candidates)

Only *local* P/Q filesystem items are candidates. No deletion performed this wave (D-002, TASK §5 "recon only").

| path | what | used by | recommend |
|---|---|---|---|
| `P\Rail-source.zip` | supplied ZIP (frontend/docs only) | — | **delete** after Owner confirms no longer needed; Q has the full tree (B-006 explains the ZIP gap). Not copied into Q. |
| `P\Rail.exe.bak-old-0504` | older build (4,199,424 B) | — | **delete** (superseded by `Rail.exe`). Q-item, not a fix. |
| `P\rail.config.json` | per-machine runtime config | `Rail.exe` | **keep local** (excluded from Q by `.gitignore`). |
| `P\diag\**` | this session's recon scripts + logs + shots | coder | **keep local** (diagnostic data, TASK §4 "record diagnostic dumps" stay out of public Q). Not published. |
| `P\Rail-source\global-promt-system.zip` | private system snapshot | — | **keep local**; published only as redacted `Q/docs/prompt-system` (D-003). Not published whole. |
| `P\Rail-source\src-tauri\target\**` | Rust build cache (debug+release) | cargo | **keep local** (ignored by Q `.gitignore`). |
| `Q\push-main.log` / `Q\push-tag.log` | git push stdout artifacts from this session | — | **delete from Q** before final commit (log files are secret-shaped-class; already in `.gitignore` `*.log`). |
| `Q\.github\workflows\build.yml` | CI (untracked on disk) | GitHub Actions | **keep on disk**, but it is intentionally untracked this wave (Q-005). Re-add only with a `workflow`-scoped token later. |

## Out-of-scope edits: 0
No product file was created, modified, or deleted in P. Q additions are limited to `.bro/**`, `.gitignore`, `.gitattributes`, `README.md`, `docs/prompt-system/**` per TASK `allow`.
