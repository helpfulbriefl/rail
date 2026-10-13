---
ver: v3.0 · build: JOIN · wave: W1-RECON · layer: recon
t_issued: 2026-10-13 · tasks: 6 · pass_criteria: 8 · an_turn: 2
allow: .bro/**; .gitignore; .gitattributes; README.md; docs/prompt-system/**
modules: W1-RECON, STAND · mirror: - · risky: yes
road: W0-BRIEF=done!:Бриф; W1-RECON=run:Разведка; W2-DESIGN=todo!:Макеты; W3b-CORE=todo:Логика; W3c-UI-RAIL=todo:Интерфейс; W4-FIX=todo:Регрессия; W5-POLISH=todo!:Релиз
---
# TASK W1-RECON — publish a safe baseline and diagnose, without fixes

## 0 Read
Read SYS/core/CODER-BOOT.md, core/RAKES.md, modules/W1-RECON.md, modules/STAND.md, templates/02-REPORT.md. Read this whole TASK and .bro/exchange/W1-RECON/00-ANALYST-NOTES.md from the target repo. Read existing project rules without modifying them. This TASK overrides conflicting generic defaults, including automatic git add -A, pushing all tags, snapshotting all history, installing stand packages, and mirror cleanup.

## 1 Context
Owner approved repository creation and public/open-source delivery. Source P=C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail. Target https://github.com/helpfulbriefl/rail, main; already initialized, with analyst exchange files. Source size reported ~7 GB. Supplied Rail-source.zip lacks src-tauri and qa. The local source, not that ZIP, is authoritative for this wave. SYS is the actual verified full local path to BRO v3.0; resolve it, do not invent it.
Existing private helpfulbriefl/global-promt-system is readable by the analyst; source snapshot HEAD e35e7e28893bddcf613d2a85b86575b9a0b3ee92. Do not replace any GitHub connection, global Git credentials, or existing remotes.

## 2 Decisions — no product changes
D-001 JOIN existing Rail. Keep its Tauri/Rust + React stack; no Electron migration, dependency upgrades, or new UI framework.
D-002 No changes to product source, existing QA scripts, dependency manifests/locks, native configuration, or current executable. No bug fixes in recon.
D-003 Public Rail source; private prompt repo remains private. Publish a clean prompt snapshot in Rail/docs/prompt-system, not private repo history. All public content needs review and redacted scan.
D-004 Work in a fresh publication clone Q=C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish. P remains untouched. If Q exists or is a reparse point, STOP; do not delete or overwrite it. The Owner explicitly authorizes creation of Q. Never modify other directories.
D-005 No public raw desktop videos, private project metrics, local credentials/config, diagnostic dumps or private Git history. License: record Q-LICENSE; propose MIT, but do not invent an approval or copyright owner.
D-006 This wave has no mirror deletion authorization. mirror:-. Do not run /MIR, delete retention bundles, or make --all bundles. Report a proposed empty dedicated local mirror path for approval later.

## 3 PASS + real stand
1 Public target contains a sanitized full source tree, including native code/QA if locally present; exclusions documented, source files hash-identical to P.
2 Source P product files unchanged; P Git config/credentials/remotes unchanged.
3 Secret review before publication: no unresolved hits; no historical private data imported.
4 Public prompt snapshot contains reviewed generic system files and SHA-256 manifest; .git and unrelated metrics excluded; source HEAD recorded.
5 Snapshot commit/tag w1-snapshot published to target; final REPORT commit separately verified on remote main. Do not push all local tags.
6 Recon 01–09, proof/INDEX.md, MAP, REPORT published; all unavailable evidence marked unavailable, never PASS.
7 Real Windows baseline documents external drop, reorder, '+', bottom system meters and held-hover behavior; proof has timestamps and tested executable SHA-256.
8 README states development status, actual available build steps and known defects, not 'fixed' or 'release-ready'.
Stand: on current Rail executable open panel -> inspect empty-panel CPU/RAM -> press '+' -> try one desktop shortcut drop and one Explorer drop -> try internal reorder if items exist -> hold cursor on a tile if available. Log expected/actual for each; missing prerequisites are Q-ids, not fixes. Repeat critical failing interactions 3 times to establish a baseline, not acceptance of a repair. Do not kill unrelated user programs or alter privileges. Record current elevation instead.
Proof: sanitized source inventory and SHA-256 comparison, redacted secret-review summary, prompt manifest, source-unchanged evidence, native baseline timeline, environment, existing-gate logs, remote commit/tag verification; INDEX maps each claim to a file.
Gates: existing commands only, each exit code and duration. Run only after inspecting commands for side effects, under Q; no dependency upgrades, no lifecycle scripts before review, no >10-minute unbounded command. If safe dependencies are unavailable, report blocked. No blanket install of STAND web/Electron packages. bro finish OK is not application PASS.

## 4 Key code
None. No product changes. This is a reconnaissance/publication task.

## 5 Tasks
1. Preflight, read-only. Resolve SYS and read rules. Inspect P Git root/status and existing .bro; detect existing BRO state and report instead of overwriting. Record Windows version, PowerShell, Node, Rust/Cargo, WebView2, monitors/DPI, Rail elevation, current executable SHA-256 and timestamp. Check whether current exe maps to a source commit; unknown stays unknown. Never print credential-bearing remote URLs. No changes to P.
2. Safe export. Inventory directory sizes, nested repositories/reparse points, binaries >50 MB, tracked and untracked sensitive paths. Review candidate files and existing history locally using redacted secret scans; findings stop commit/push/mirror under S3. Produce an explicit include/exclude manifest. Clone the already existing public Rail repo to new Q, preserving its initialized main and analyst TASK/NEXT. Copy only reviewed source/build/resource/test files from P, excluding .git, .bro, dependencies, targets, dist, caches, diagnostic data, local rail.config.json and secrets. Do not follow reparse points or copy nested repositories. Adapt Q .gitignore/.gitattributes; do not normalize/rewrite product bytes. Compare hashes P->Q. Stage only reviewed paths, not blanket git add -A. Commit sanitized source as W1: snapshot; tag that commit w1-snapshot; push main and only this tag, with no force/reset/rebase. Auth failure: stop and report; do not switch accounts or overwrite remotes.
3. Prompt snapshot. Copy generic reviewed files from SYS: VERSION, README.md, CHANGELOG.md, TRIGGERS.md, core/, modes/, modules/, templates/, builds/, auditor/, tools/, and non-legacy generic docs/. Exclude .git, docs/legacy, metrics project datasets, credentials and unrelated attachments. Treat README links to the private mirror as source references, not publicly accessible guarantees. Preserve source contents; document exclusions and paths in docs/prompt-system/SOURCE.md with per-file SHA-256 and source HEAD. Confirm generic files match the reviewed source snapshot, allowing CRLF/LF differences only when declared. No private source history or visibility changes.
4. Native baseline. Perform §3 scenarios on the existing Windows executable with an independent CPU/RAM reference (Task Manager/Resource Monitor) over the same time window. Distinguish system meters from per-process metrics and hold progress. Collect diagnostic events only if the existing build supports them; inspect logs before sharing. Examine native tracker, pick_files command/dialog plugin permissions, drag registration/window regions, coordinate units, elevation, auto-hide and IPC failures by path:line. Hypotheses are not confirmed causes. Record missing src-tauri or qa if absent locally. No repair.
5. Recon/docs. Run W1-RECON phase B on exported Q, adapting evidence to native Tauri. Produce recon/01–09 and MAP anchored to P, Q, target and SYS. In 09 answer: failing drag variants, '+' symptom, system vs per-item meter, window geometry vs value changes, attempted fixes, actual source/exe relationship, native QA coverage, license question, and whether tracked history is safe to publish later. Update Q README as an honest development entry page with existing verified commands, known issues and prompt snapshot link. Draft profile/site Rail wording in .bro only; do not publish profile/site changes in this wave.
6. Deliver. Initialize BRO in Q only after preserving analyst TASK/NEXT and confirming no conflicting state. Record mirror:- and this TASK's overrides. Publish redacted REPORT and proof INDEX with recon/docs via reviewed staging. No raw video, unreviewed traces, all-tags push, metrics push to private SYS, or mirror cleanup. If bro init rewrites NEXT, restore the analyst-owned content, do not redefine the wave. Use bro begin/finish only after verifying their operations are allowed; otherwise report the precise blocker. A final REPORT commit is allowed, but verify its remote SHA AFTER publication and identify code snapshot SHA separately. Report failures honestly and list out-of-scope edits (must be 0). End: 'Готово → напиши аналитику: BRO CHECK'.

## Z Existing defects — diagnose only
B-001 external drag/drop reportedly fails after repeated fixes.
B-002 internal reorder: verify separately, status unknown.
B-003 '+' reportedly no longer opens/adds files.
B-004 bottom system CPU/RAM fluctuate on supplied video; compare raw values and rendering with an independent reference.
B-005 held-hover/card instability reported by Owner; isolate from B-004.
B-006 provided ZIP lacks native/QA sources; determine if local tree is complete.
B-007 docs claim fixes that Owner still reports as broken; preserve history and annotate current status.
