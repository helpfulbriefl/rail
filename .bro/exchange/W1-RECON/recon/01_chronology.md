# 01 · chronology

Timeline reconstructed from Q `git log` + P file mtimes. `git log --reverse --date=short --format="%h %ad %s"`:

```
69ed182 2026-10-13 Initial commit
5d77079 2026-10-13 BRO W1-RECON: safe publication task and analyst audit; no product changes
ef1a3c1 2026-10-13 W1: snapshot   ← w1-snapshot (this wave)
```

P directory mtimes (no VCS in P — raw filesystem):

| path | mtime |
|---|---|
| P\Rail.exe | 2026-10-04 23:25:01 +03:00 |
| P\Rail-source\Rail-source (nested source tree) | varies, no VCS timestamp |
| P\Rail-source\global-promt-system.zip | 213 KB; internal .git HEAD `e35e7e28893bddcf613d2a85b86575b9a0b3ee92` ('metrics topath-saver W4-STORE') |
| P\Rail-source\src-tauri\target\debug\rail.exe | older debug build, not the delivered `Rail.exe` |
| P\Rail-source\src-tauri\target\release\rail.exe | release build, P root `Rail.exe` (4,218,880 B) is presumably a copy |

Docs in P:
- `Rail-source\docs\SPEC.md` — 13 KB, product spec
- `Rail-source\README.md` — dev entry page

`Rail.exe.bak-old-0504` (4,199,424 B) predates `Rail.exe` (4,218,880 B) — both are build artifacts, not source.

**Source↔exe relationship: UNKNOWN.** `P\Rail.exe` SHA-256 `22B21D2899BE9B3F8E56F628A2C261983C519E0DD44FE4F7EFB2A232854B7E82` cannot be mapped to a source commit without a signed/signed-build pipeline. No `.git` in P; no build provenance recorded. This is a Q-item, not a fix.

Report folder dates:
- `.bro/exchange/W1-RECON/00-ANALYST-NOTES.md` (Q, remote initial commit) — 2026-10-13
- `.bro/exchange/W1-RECON/01-TASK.md` (Q) — 2026-10-13
- No prior REPORT/ACCEPT for this wave
