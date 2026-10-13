# MODULE W1-RECON · first wave: snapshot + recon (coder)
Allow: `.gitignore`, `.gitattributes`, `.bro/`. Code: NO changes (G-011). TASK has `base: w1-snapshot`. Output folder R = `.bro/exchange/W1-RECON/recon/`.

## A Snapshot
1 `git status`; «not a git repository» → `git init -b main`. Logs (C5): `git status`, `git log --oneline -30`, `git count-objects -vH`, `git remote -v`, `git branch --show-current`.
2 `.gitignore` — add what is missing:
```
node_modules/
dist/
build/
out/
**/target/
.venv/
venv/
__pycache__/
.env*
*.har
*.pem
*.key
**/.auth/
test-results/
playwright-report/
.bro/**/proof/**/*.zip
.bro/**/proof/**/*.webm
.bro/**/proof/**/*.mp4
<extra secret paths from TASK>
```
Already tracked → `git rm -r --cached <path>`; list in REPORT.
3 `.gitattributes`: `*.cmd *.bat *.ps1 text eol=crlf` · `*.png *.jpg *.ico *.zip *.bundle binary`. Then `git config core.longpaths true`.
4 Secret scan. PATTERN (one line):
`(api[_-]?key|API[_-]?KEY|secret|SECRET|token|TOKEN|passw(or)?d|PASSW(OR)?D|bearer|Bearer).{0,4}[:=].{0,4}[A-Za-z0-9_%+/=-]{32,}|sk-[A-Za-z0-9_-]{20,}|ghp_[A-Za-z0-9]{36}|github_pat_[A-Za-z0-9_]{20,}|AKIA[0-9A-Z]{16}|xox[baprs]-[A-Za-z0-9-]{10,}|AIza[0-9A-Za-z_-]{35}|BEGIN [A-Z ]*PRIVATE KEY`
- history: `git log --all -E -G "PATTERN" --name-only --format="%h %ad"`
- to be committed: `git grep --untracked -l -I -E "PATTERN"`
- gitleaks present: `gitleaks git --redact -v` (old versions: `gitleaks detect --redact -v`)
Hit → REPORT: where, test or real, first 4 chars + length, never the value. Then C2: no commit, no push, no mirrors (S3); still do phase B; chat «СТОП: возможный секрет — нужен аналитик».
5 Size: files >50 MB → REPORT; >100 MB → `.gitignore`; size-pack >1 GiB → no push.
`git ls-files -co --exclude-standard | ForEach-Object { Get-Item -LiteralPath $_ -ErrorAction SilentlyContinue } | Where-Object { $_.Length -gt 50MB } | Select-Object FullName, Length`
6 `git add -A -- . ":(exclude).bro"` → `git commit -m "W1: snapshot"` → `git tag w1-snapshot`. «embedded git repository» → `git rm --cached <path>`, add to `.gitignore`, repeat.
7 No `origin` → TASK repo URL: `git remote add origin <URL>.git` (NEW: `gh repo create <name> --private --source . --remote origin`). Other origin → keep it, note in REPORT, skip push. `git push -u origin HEAD` · `git push origin --tags`. GitHub sign-in window → Owner signs in once in the browser (SYS/docs/GITHUB-SETUP.md §3).

## B Recon (read-only; every claim = `path:line`; no proof → «not found»)
| file in R | content |
|---|---|
| 01_chronology.md | `git log --reverse --date=short --format="%h %ad %s"`; report/step folders with dates; docs + modified date |
| 02_repomap.md | tree 2–3 levels + purpose; entry points; run commands; import rank: how often each file is imported (rg / Select-String import\|require\|use) — top 30 |
| 03_truth_map.md | all requirement sources: path · date · governs · conflicts with |
| 04_stack.md | versions from manifests + lock files; `node -v`, `python --version`, `cargo -V`, `git --version`, `gitleaks version` |
| 05_alive_dead.md | module → 🟢/🟡/🔴 → proof |
| 06_config_secrets.md | paths + key names only (`.env.example`, code); covered by `.gitignore`? |
| 07_leftovers.md | delete candidates: path · what · used by · yes/no/ask |
| 08_gates_baseline.md | existing checks: command · exit code · pass/fail · time; >10 min = «timeout»; none = «not set up» |
| 09_project_questions.md | questions from TASK, or omit |

## D Deliver
`.bro/MAP.md` first version: §1–4, §9 from recon facts; §5–8, §10 from TASK. Proof rows: snapshot tag, scan log, recon 01–09, MAP. Self-check into REPORT: `git diff --stat w1-snapshot HEAD -- . ":(exclude).bro"` must be EMPTY.
NEW variant (W1-INIT, empty folder): skip A4–A5 and B except 04_stack (tool versions); create SPEC + scaffold exactly as TASK says.
