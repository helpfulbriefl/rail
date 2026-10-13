---
wave: W1-RECON · t_accept: 2026-10-13 21:31 · verdict: PARTIAL · spot: 3 · false_done: 0
defects: 5 · causes: C@C4;E@Q-008;E@Q-009;S@stand-coverage · O1: 0 · O2: 0 · O3: 0 · O4: 1
owner_touches: 3 · owner_fixes: 1 · gh_calls: 12 · ctx_pct: - · friction_an: #env
read_list: .bro/exchange/W1-RECON/proof/INDEX.md; .bro/exchange/W1-RECON/proof/native-baseline.md; src-tauri/src/tracker.rs
---
# ACCEPT W1-RECON — PARTIAL

## Checks
- REPORT read at f4d2b983513cc74ff87d11f228c12f9c18e1787e. Current main resolved to this commit during MAP/NEXT reads. Snapshot tag independently resolved by GitHub to ef1a3c11e6f4eb26ec9dff3329a18caa43d7e5f0. Published native sources/manifests and exchange are present. Initial publication is usable for the next baseline wave; this is not complete functional acceptance or a full 48-file/hash/secret-history audit.
- Spot 1 INDEX: proof_ok=11/proof_req=12, row 8 unavailable. Rows with checkmarks establish evidence availability, not correctness of every causal claim. The required native drop/reorder/card stand was not completed.
- Spot 2 native-baseline: '+' clicks targeted window center rather than the verified button. No valid button result. Two independent WMI samples show small OS variation; no paired Rail UI/IPC series establishes whether the video complaint is OS load, sampling, rendering, or an exe mismatch. Flat process working set over a short window does not rule out leaks generally. Mtime/geometry/missing log support a desync hypothesis but do not prove binary build provenance. Panel already visible before 'edge reveal' also does not prove a hidden->visible edge transition.
- Spot 3 tracker.rs: current source uses sysinfo, not WMI, as application metric producer; A=0.25 verified. WMI is an independent reference only. Comments identify carry-forward and smoothing, not proof that the running executable contains them.
- Scope O3=0: not independently established. INDEX references diagnostic scripts under read-only P/diag; clarify pre-existing vs newly written. Delivery reports switching gh active account and restoring it; reconcile with the no-account/credential-change TASK. Do not repeat this pattern. No claim here that a secret was leaked or that P was definitely modified.
- O1=0 incomplete native proof; O2=0 causal/closure conclusions exceed evidence; O4=1 unavailable interactions and blocked gates were explicitly disclosed. false_done=0: unsupported conclusions are not counted as proven-false facts. ctx_pct unavailable rather than invented. gh_calls includes acceptance/preflight reads and this batch write; the normal five-call target was exceeded to establish safe build inputs.

## Defects -> W1B-BASELINE §Z
Z-01 Native functional scenarios incomplete; '+' was not clicked at a verified control.
Z-02 B-004/old-exe conclusions overclaim; require source-built binary and time-aligned observations.
Z-03 Source P / account invariance insufficiently evidenced; reconcile without fabricating retroactive evidence.
Z-04 Build/type/CSS/visual gates unrun; no verified exe from published commit.
Z-05 Publication metadata and helper OK require explicit meaning; final REPORT commit is not automatically the tested product commit. The published .bro/LOG.md was also absent on a direct read; restore missing BRO state from this ACCEPT without overwriting NEXT. CI workflow Q-005 remains absent and license approval remains open; neither requires switching credentials or declaring a release.

<!-- LOG -->
## L-002 · 2026-10-13 · W1-RECON · ◐PARTIAL @f4d2b983513cc74ff87d11f228c12f9c18e1787e · w1-snapshot
#tags: native-baseline wrong-click system-meters binary-provenance
Publication available; full stand incomplete. No product bug accepted as closed.
New D-007..D-011: baseline before design; immutable P; isolated helper; locked local build; no auth switching. Replaces automatic W2-DESIGN next step.
<!-- /LOG -->
<!-- MAP -->
### §5 Decisions · append, retain D-001..D-006
D-007 Reuse existing verified Q; the original D-004 'Q must not exist' was first-clone-only. P stays read-only. Next W1B-BASELINE before W2-DESIGN, not a product-fix wave.
D-008 Real Windows native stand required. Wrong-target clicks, missing item prerequisites and two unpaired WMI samples cannot close B-001..B-005. Old-exe history remains unproven.
D-009 Permit locked project-local dependency fetch/build and existing gates under TASK limits. No versions/locks/product/native-config changes, release, installers or elevation.
D-010 No gh account switch, global auth/config change, new token, broadened token scope, force/reset, --all history export, or destructive mirror. Auth failure reported, not bypassed.
D-011 Execute unchanged BRO helper from an ignored project-local generic-system copy, so automatic metrics cannot write to original SYS or public docs/prompt-system. No original system edits/pushes.
### §7 Rakes · append
G-06 WMI reference != Rail sysinfo stream. Capture paired observations before assigning a cause.
G-07 Click actual '+' button, not native window center. Hidden->visible transition must start hidden.
G-08 Report/push/helper OK is metadata, not native functional PASS. Source commit, exe hash and final REPORT commit are separate identifiers.
### §10 Roadmap · replace
W0-BRIEF=done!:Бриф; W1-RECON=done:Разведка PARTIAL; W1B-BASELINE=run:Сборка и реальный стенд; W2-DESIGN=todo!:Макеты; W3b-CORE=todo:Логика; W3c-UI-RAIL=todo:Интерфейс; W4-FIX=todo:Регрессия; W5-POLISH=todo!:Релиз
Next: W1B-BASELINE. B-001..B-005 remain open. Full functional acceptance requires native evidence; publication status alone is insufficient.
<!-- /MAP -->

## Handoff
Source P is immutable. Q and SYS paths are in MAP. Current public source cannot yet be tied to tested exe. Bootstrap next wave using published NEXT, not old first-wave TASK. No owner action needed until coder needs a real interaction it cannot perform, a security/cost blocker, or mockup approval.
