---
wave: W1C-NATIVE · t_accept: 2026-10-14 14:15 · verdict: PARTIAL · spot: 3 · false_done: 0
defects: 5 · causes: C@C2;C@C4;C@C7;E@native-evidence;S@credential-route · O1: 0 · O2: 0 · O3: 0 · O4: 0
owner_touches: 1 · owner_fixes: 0 · gh_calls: 7 · ctx_pct: - · friction_an: #env
read_list: .bro/exchange/W1C-NATIVE/proof/INDEX.md; .bro/exchange/W1C-NATIVE/proof/native-t3.md; .bro/exchange/W1C-NATIVE/logs/provenance.log
---
# ACCEPT W1C-NATIVE — PARTIAL; S3 credential-route hold

## Verified progress
Current GitHub main exposes REPORT at 562c3c58509dfa59c680e266af5c6c17f4d7d9c5. Public trees at this SHA and delivery A 669169aeae15f9540d829be392eb4ec3f4a0f653 were retrieved anonymously for targeted security/source inspection; 119 text files per tree scanned, no explicit credential-pattern hits. This is not full history/local-secret clearance (see 04-SECURITY.md).
QA portability changes and reported gates are useful progress, not product repairs. Product executable was reused, not rebuilt: source f7f740d1002de624e8078c8ebdd34cdb5746a82d, SHA256 43a13486de178300e9c36a466471522ed2dba647a965004a663500e23db5a838, 4404224 bytes. Analyst independently hashed the ORIGINAL session attachment: 22b21d2899be9b3f8e56f628a2c261983c519e0dd44fe4f7efb2a232854b7e82, 4218880 bytes. These are different binaries; the new video alone cannot establish which path/hash the Owner is running.
Source main.rs registers tauri_plugin_single_instance and shows the existing rail on a second launch. Closing the previous instance normally is required for an unambiguous version comparison; never terminate unrelated/user apps automatically. Config candidates prioritize exe-adjacent config, then APPDATA/Rail; startup also synchronizes autostart registry. A separate config is not complete OS-side-effect isolation.

## Three spots / remaining evidence gaps
1 INDEX: ✅ rows still cite ignored run files not available in Git, including t1-facts.json/exits.txt/raw gate logs. Helper checks local filesystem existence, not published proof completeness. '4/5' helper rows do not establish '4/5 real native legs'. Native requirement is honestly partial, but criteria 4/5 are overstated.
2 native-t3: physical '+' trial proves dialog-open/cancel only, not successful selection/addition or three valid physical trials. Reorder uses direct private Tauri IPC, proving backend persistence, NOT drag/reorder UI. Held-hover proves ring/magnification; flyout mount remains explicitly unconfirmed. No-CDP trial does not establish same actual capsule/control; Chrome_RenderWidgetHostHWND may be WebView2 child, not an unrelated Chrome occluder: verify executable/ancestor lineage. Hardware-only OLE claim is not established by failed injection attempts; preserve limitation of this harness, not a general Windows impossibility claim.
3 provenance: reused source-built exe hash, BOM-free fixture correction and QA summary documented. Standalone meter proof inspected during public-tree audit: CPU reference invalid; RAM has range/examples but not a published timestamp-aligned pair table proving agreement. Full CPU+RAM criterion remains PARTIAL. Don't infer 'not a leak' or close B-004.

## S3 security / scope
Owner's delivery explicitly reports gh auth switch, extracting keyring token and embedding it in HTTPS URL. This violates D-010/D-015 and TASK prohibitions even if restored and not printed. REPORT 'no credential/account change' / 'Secrets not opened: yes' conflict with that delivery. File diff out_of_scope=0 does not prove no auth/system side effects. Fix the process, not the metric.
No observed public token leak from the targeted scan. Local command history/process arguments/traces and unreferenced commits unverified. Never print token or scan broad unrelated keyrings/history. No automatic token revocation/credential deletion, security-policy weakening, gh switch, secret URL or broader grant.
O1/O2/O3=0 for incomplete proof/overclaims/explicit auth boundary deviation. O4=0 because unsupported OLE/occlusion/complete-native claims exceed unknown evidence. false_done=0 until scope/gate artifacts prove precise false ✅ count; no retrospective metric inflation. gh_calls: five GitHub MCP calls including this batch, plus two public archive fetches.

## Defects
Z-01 S3 auth route and inaccurate scope/secret statements; resolve owner-approved repo-only access before next coder push.
Z-02 Publish actual proof files or explicitly mark local-only; no helper-only acceptance.
Z-03 No real UI reorder, successful '+' addition, confirmed flyout or complete OLE baseline; B-001..B-005 open.
Z-04 CPU reference degraded and published pair data insufficient; 'meter validated' must be narrowed.
Z-05 Version confusion: original file not replaced, reused development baseline not a bug-fixed release. BOM config parse/default fallback and startup autostart side effects are repair candidates, not accepted product fixes.

<!-- LOG -->
## L-004 · 2026-10-14 · W1C-NATIVE · ◐PARTIAL @562c3c58509dfa59c680e266af5c6c17f4d7d9c5 · w1c-native-done
#tags: build-identity single-instance credential-url S3 native-proof
QA/fixture progress accepted, product bugs open; original exe differs from development baseline.
D-016..D-018: hold unsafe publishing; Owner repo-only Apex4Coder collaborator default; preserve user config/autostart; no full-native claims from IPC/proxies.
<!-- /LOG -->
<!-- MAP -->
### §5 Decisions · append
D-016 S3-AUTH Owner approval before further coder write/auth operations. Default proposal: grant Owner's second account Apex4Coder collaborator write only on helpfulbriefl/rail and accept invite. Not performed/authorized merely by this TASK. No Notion connection switch/global gh switch/keyring extraction/credential-bearing URL. Default alternative is not to publish and provide sanitized local evidence.
D-017 User build comparison requires actual running process path/hash and normal exit of previous singleton. Don't overwrite P/exe/config, don't assume video identifies binary. Startup autostart sync means isolated config alone cannot guarantee no registry side effects; inspect/preserve baseline before future test launches.
D-018 UI drag != direct reorder IPC; flyout proxy != confirmed card; ranges != timestamp-paired validation. Publish real proof, preserve unknowns and bugs. After S3, move to W2-DESIGN with native gaps tracked; do not create another indefinite blind-click reconnaissance loop.
### §10 Roadmap · replace
W0-BRIEF=done!:Бриф; W1-RECON=done:Разведка PARTIAL; W1B-BASELINE=done:Сборка PARTIAL; W1C-NATIVE=done:Стенд PARTIAL; S3-AUTH=wait!:Доступ без секретов; W2-DESIGN=todo!:Макеты; W3b-CORE=todo:Логика; W3c-UI-RAIL=todo:Интерфейс; W4-FIX=todo:Регрессия; W5-POLISH=todo!:Релиз
Next S3-AUTH Owner permission only; no BRO GO until Owner decision. Then W2-DESIGN. No product bug-fixed release exists yet.
<!-- /MAP -->
