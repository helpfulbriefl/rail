# RAKES · G-001… (read on demand; cite by ID)
| ID | rake | guard |
|---|---|---|
| G-001 | coder writes false summary: wrong folder (`.kilo\worktrees`) or invented result | main tree only; truth = commit; spot path:line |
| G-002 | `localhost`/`::1` on Windows → WSAEACCES | always `127.0.0.1` (C6) |
| G-003 | PowerShell redirect writes UTF-16 | `cmd /c "… > file 2>&1"` (C5) |
| G-004 | dev server holds files (EBUSY) | kill own processes before build |
| G-005 | files scattered everywhere | everything in `.bro/exchange/W/` |
| G-006 | secret in git/log | .gitignore + scan before push + masks (R12, C2) |
| G-007 | several requirement sources disagree | one SPEC; changes only via D |
| G-008 | step numbers collide with old ones | unique IDs, never reused |
| G-009 | outdated library | check support status before choosing (R6) |
| G-010 | screenshots of mocks sold as done | accept on real build (STAND) |
| G-011 | coder «fixes» during recon | «change nothing» + empty-diff self-check |
| G-012 | two VS Code windows on one repo | one coder; `index.lock` → wait 15 s, retry ×3 (C3) |
| G-013 | home-made acceptance machine grows | standard tools, one `gates` command |
| G-014 | product feature confused with process («2 coders») | ask: product or our workflow? |
| G-015 | handoff lags behind git | first git log, tags, LOG; trust after check |
| G-016 | `ELECTRON_RUN_AS_NODE=1` in Zoo terminal | `dev:safe`, checked in gates |
| G-017 | Rust↔JS seam breaks weak coder | TS everywhere |
| G-018 | fresh framework → model invents API | mass stack; novelty after a trial wave |
| G-019 | «light» means different things | ask; numbers per option (Electron ≈85–200 MB) |
| G-020 | micro-fixes eat analyst 5h window | waves of 3–6 tasks, ❌ batched (Z), self-repair ≤3 (R16) |
| G-021 | analyst edits git step by step | 1 commit per wave, only inbox/exchange (R15) |
| G-022 | limit ends, context lost | MAP + LOG + exchange files in git + PC mirror (§8) |
| G-023 | visuals agreed in words → fix series | mockups before code, one S2 per batch |
| G-024 | Owner repeats «don't change code yet…» | ritual built in (R3) |
| G-025 | full diffs burn analyst tokens | REPORT + ≤3 spots (R13) |
| G-026 | coder «improves» analyst code | paste verbatim (TASK §4) |
| G-027 | Zoo autoloads `%USERPROFILE%\.roo\rules`, `.roo/`, `.roorules`, `AGENTS.md` → system leaks into every project | never create/touch them; system only via trigger line (C9) |
| G-028 | analyst and coder push at the same time | analyst writes only on his turn, only inbox/exchange; coder `pull --no-rebase` on reject (C3); `bro begin` refuses a reported wave |
| G-029 | agent self-assessment is optimistic | nobody scores self (K↔O); spot 1 number per wave |
| G-030 | LLM arithmetic is wrong | all sums by `bro.ps1` |
| G-031 | Russian costs ~1.3× tokens of English | agent text EN, Owner text RU (M1–M2) |
| G-032 | acceptance from coder's words | proof pack + spots; no proof = FAIL (R17) |
| G-033 | robocopy exit 1–7 = success, not error | treat <8 as OK (bro.ps1 does) |
| G-034 | PowerShell does not expand `%VAR%` (makes a literal folder); Zoo read tool expands nothing | PowerShell: `$env:LOCALAPPDATA`; read tool: full path; bro.ps1 expands both in paths |
