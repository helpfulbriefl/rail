---
ver: v3.0 · build: JOIN · wave: S3-AUTH · layer: integration
t_issued: 2026-10-14 14:15 · tasks: 3 · pass_criteria: 3 · an_turn: 5
allow: .bro/**
modules: METRICS · mirror: - · risky: yes
road: W0-BRIEF=done!:Бриф; W1-RECON=done:Разведка PARTIAL; W1B-BASELINE=done:Сборка PARTIAL; W1C-NATIVE=done:Стенд PARTIAL; S3-AUTH=wait!:Доступ без секретов; W2-DESIGN=todo!:Макеты; W3b-CORE=todo:Логика; W3c-UI-RAIL=todo:Интерфейс; W4-FIX=todo:Регрессия; W5-POLISH=todo!:Релиз
---
# S3-AUTH — Owner gate; no coder execution before approval

## 0 Read
W1C 03-ACCEPT.md, 04-SECURITY.md, MAP, NEXT. Q=C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish. Preserve P, all existing binaries/configs, original SYS, global helper configuration, original tags and credentials. No product/QA/dependency/permission-source edits.

## 1 Owner decision (not yet granted)
Proposed default: Owner grants their own other account Apex4Coder collaborator access for push ONLY to helpfulbriefl/rail, via https://github.com/helpfulbriefl/rail/settings/access, then accepts that selected invitation as Apex4Coder. Do not grant all-repo/organization access. This changes access to this repo; it does not disconnect/change the Notion GitHub connection. Analyst/coder must not perform permission changes or accept an invitation before the Owner approves this route. If declined, stop publication and use sanitized attached evidence; no token workaround.

## 2 Strict rules
No gh auth switch/login/logout/token, git credential fill, keyring read, token environment assignment, URL password/user-token, print/mask-preview of a secret, persistent auth/helper/remote changes, GIT_TRACE/curl verbose, broad local history scan, force/reset/rebase, private-system push, or automatic revocation. This TASK authorizes normal secure Git/GH credential-helper use only AFTER Owner approval; not secret extraction. Any failure -> STOP with non-secret status. No fallback route. Approval must be recorded, not inferred from 'everything is done'.

## 3 PASS/proof
1 Owner route approved and invitation accepted; current actor proved as Apex4Coder and .permissions.push on helpfulbriefl/rail true by normal authenticated metadata request, no credential value.
2 W1C delivery deviation recorded honestly in an append-only errata; original reports unchanged; no misleading 'no secret opened/account changed' acceptance. Public zero-pattern scan scope quoted accurately, not a full safety claim.
3 One normal host-only authenticated push of allowed REPORT/proof files succeeds; final remote main verified vs local; no global account/config/system change. If route fails, deliver blocker without bypass. No product readiness PASS.

## 4 Tasks — only after Owner confirms approval/invite acceptance
1. Verify actor and repo permission without switching: gh api user --jq .login; gh api repos/helpfulbriefl/rail --jq .permissions.push. Expected Apex4Coder / true. Don't print GH token/status credential output. If different/missing, stop; don't alter auth. Pull Q --ff-only only when local unpublished state does not conflict; never discard files. Execute unchanged BRO helper only from ignored local tooling and mirror:-; apply prior ACCEPT.
2. Create append-only W1C 05-AUTH-ERRATA.md with actual steps admitted in delivery and corrected scope statement (no secret values), S3 Owner approval record and metadata result. Do NOT scan unrelated keyrings/history, revoke or modify credentials. If Owner privately discovers token exposure, pause and report that revocation/re-authorization needs Owner action; no model handling of actual token.
3. Prepare S3 REPORT/proof (three rows, real backticked files/accurate marks). Stage explicit .bro paths only, excluding tooling/run/raw secret-prone logs. With Owner-approved existing actor/helper, push host-only https://github.com/helpfulbriefl/rail.git HEAD:refs/heads/main, GIT_TERMINAL_PROMPT=0 transient and restored; no token URL/gh account change. Publish only new s3-auth-done tag if used, never move old tags. Verify remote main/final HEAD and record externally in delivery. Auth failure -> no fallback. End 'Готово -> напиши аналитику: BRO CHECK'.

## Next
Once S3 is resolved, analyst issues W2-DESIGN (mockups, then Owner S2 approval). Product repairs and release remain later. Do not manufacture a fifth native-baseline wave or claim physical drag/card success from IPC/proxy evidence.
