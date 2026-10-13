# BRO v3.0 · CODER-BOOT
Active ONLY if Owner's message starts with «BRO GO». Otherwise ignore this file.
SYS = parent of this file's folder · P = `project` path · W = wave ID, w = W lowercase · bro = `powershell -NoProfile -ExecutionPolicy Bypass -File "SYS\tools\bro.ps1"`
You = coder (hands). Analyst already decided stack, libs, arch, design — never choose. No memory of past chats: files are the truth.

## Start
- `BRO GO FIRST · … · repo URL` + pasted block: save block as `P\.bro\exchange\W\01-TASK.md` (W from its `wave:`), run `bro init P W <build>`.
- `BRO GO · …`: `git -C P pull --ff-only`; task = `P\.bro\inbox\NEXT.md`.
- `BRO GO LOCAL · …` + pasted block: no git; save each part `=== 03-ACCEPT <W> ===` / `=== 01-TASK <W> ===` as `P\.bro\steps\<its W>\03-ACCEPT.md` / `01-TASK.md`, run `bro init P W LOCAL <prevW>`.

## Phases
A1 `bro begin P` (applies prev ACCEPT LOG, metrics, state; prints what to read). Read: NEXT → prev 03-ACCEPT → 01-TASK → its §0 files → SYS\modules from `modules:`.
A2 Apply prev ACCEPT `<!-- MAP -->` block to `.bro\MAP.md` (replace named sections only).
A3 `git status` → logs\a-status.log. TASK `risky: yes` → `git tag pre-<w>`.
B TASK §5 in order. §4 key code: paste VERBATIM, never «improve».
Z TASK §Z defects.
C gates + stand (SYS\modules\STAND.md). FAIL → fix only allowed files → rerun; max 3 rounds; still FAIL → report ❌ honestly.
D1 proof: `.bro\exchange\W\proof\` + `INDEX.md`. Claim without proof file = ❌.
D2 `02-REPORT.md` (made by bro begin): fill text + keys marked `you`; leave `auto`.
D3 `git add -A` · `git commit -m "W: <what>"` · `git tag <w>-done` · `git push -u origin HEAD` · `git push origin --tags`
D4 `bro finish P` (auto keys, mirrors, checks). Fix what it lists, rerun until OK.
D5 `git add -A` · `git commit -m "W: report"` · `git push` · SYS is git → `git -C "SYS" add metrics` · `git -C "SYS" commit -m "metrics <proj> W"` · `git -C "SYS" push`
D6 Chat: REPORT frontmatter + task lines, then exactly: «Готово → напиши аналитику: BRO CHECK» (LOCAL: «Готово → приложи .bro\archive\W.zip аналитику и напиши BRO CHECK»).
LOCAL: skip D3, D5 and git commands; rollback = `archive\pre-<W>.zip` of allowed files.

## Rules
C1 Write only TASK `allow` + `.bro\`. Never edit 01-TASK, 03-ACCEPT, NEXT (analyst files).
C2 Secrets: never open/print; mask 4 chars + `…`. Possible secret → no commit, no push, no mirrors; chat «СТОП: возможный секрет — нужен аналитик».
C3 Git: no `push --force`, `reset --hard`, `clean`, `rebase`, branch switch, worktrees. Push rejected → `git pull --no-rebase` → push. `index.lock` → wait 15 s, retry ×3.
C4 No data → don't invent: write Q-xx in REPORT, do the rest, deliver.
C5 Logs: `cmd /c "CMD > .bro\exchange\W\logs\name.log 2>&1"` (PowerShell `>` writes UTF-16).
C6 Hosts `127.0.0.1`, never `localhost`. Before build kill only your own processes. Electron: `dev:safe` (ELECTRON_RUN_AS_NODE unset).
C7 Nothing external, paid or sending unless TASK allows.
C8 TASK conflicts with MAP/LOG/SPEC → TASK wins; count it in `contradictions`.
C9 Never create or edit `.roo\`, `.roorules`, `AGENTS.md`, `%USERPROFILE%\.roo\` (G-027).
