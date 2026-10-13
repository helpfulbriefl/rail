---
wave: W1B-BASELINE · t_accept: 2026-10-13 23:28 · verdict: PARTIAL · spot: 3 · false_done: 0
defects: 4 · causes: C@C4;E@native-precondition;S@proof-format · O1: 0 · O2: 0 · O3: 0 · O4: 1
owner_touches: 2 · owner_fixes: 0 · gh_calls: 5 · ctx_pct: - · friction_an: #env
read_list: .bro/exchange/W1B-BASELINE/proof/INDEX.md; .bro/exchange/W1B-BASELINE/proof/native-baseline.md; .bro/exchange/W1B-BASELINE/logs/exe-provenance.log
---
# ACCEPT W1B-BASELINE — PARTIAL

## Checks
Delivery accepted: current GitHub main exposes REPORT at 6f48c5066441c2cae7a219fad74d9178c345b173. Later owner/coder delivery states main and w1b-baseline-done both equal that SHA after command-local GCM push; the original REPORT push:❌ is historical pre-delivery metadata, not a request to rewrite it.
Spot 1 INDEX: artifacts exist, but 'yes' and unquoted proof paths do not meet Get-Proof parser (requires ✅ and backticked existing files). Thus helper 0/6 is not a reliable count of all available artifacts; native/meter rows really remain incomplete. No all-green reformatting to hide this.
Spot 2 native baseline: four INVALID attempts while hidden, not an actual '+' functional trial. The text itself names visible/pinned/tray-show prerequisites but did not establish them. UIA observations of a hidden/incorrect target do not establish universal absence of release accessibility. 'REGION skip empty_hit_rect' is not a GetWindowRgn NULLREGION measurement. A rectangle x=2512 labelled on a 1920-wide work-area needs monitor/virtual-screen reconciliation; DPI=0 is invalid evidence, not a valid DPI scale. No native feature accepted as working or broken on these trials.
Spot 3 exe provenance: source commit f7f740d1002de624e8078c8ebdd34cdb5746a82d, locked Cargo command, exe 4404224 bytes, SHA-256 43a13486de178300e9c36a466471522ed2dba647a965004a663500e23db5a838 equals copied test exe. Accepted as documented build provenance for the next test, not independent rebuilt verification or proof of native functionality.
Gate claim 'all EXIT 0' conflicts with the same REPORT's failed storyboard leg. Its cause is supported by separate next-wave preflight reading of qa/storyboard.mjs: URL.pathname, `which chromium`, localhost. qa/visual.mjs is mock-backed, including __railDrop injection; 23/23 is not native OLE proof. This preflight is separate from the three W1B spots.
O1=0 incomplete paired/native proof; O2=0 source/region/closure inferences too strong; O3=0 full boundaries/secret/hash gate evidence not independently established by selected spots; O4=1 missing outcomes were disclosed. false_done=0 until actual gate exit evidence resolves conflicting summary; no unsupported metric inflation. ctx_pct unavailable, not guessed.

## Defects -> W1C-NATIVE
Z-01 Fix test visibility/PID/geometry prerequisites; obtain valid native target interactions, not more hidden clicks.
Z-02 Repair proof syntax and publish compact redacted gate/paired-meter data; raw working-tree-only files cannot be analyst evidence.
Z-03 QA storyboard Windows portability broken; mock/native and PASS/blocked distinctions must stay explicit.
Z-04 Observe live WebView DOM/errors via process-local loopback CDP if available; IPC-only does not prove UI observations require an in-product tap. All B-001..B-005 remain open.

<!-- LOG -->
## L-003 · 2026-10-13 · W1B-BASELINE · ◐PARTIAL @6f48c5066441c2cae7a219fad74d9178c345b173 · w1b-baseline-done
#tags: hidden-panel proof-parser source-built-exe loopback-cdp
Delivery and documented build usable; native trials invalid/incomplete; meter trace unpaired.
D-012..D-015: verify visible owned window; QA-only portability exception; local CDP observation; command-local owner credential route. Next W1C-NATIVE.
<!-- /LOG -->
<!-- MAP -->
### §5 Decisions · append; preserve earlier decisions
D-012 W1C-NATIVE first establishes visible live test-owned rail, correct HWND/PID, monitor/DPI/client coordinates and actual region type. Never classify hidden-window clicks as '+' tests or 'skip' as NULLREGION.
D-013 Replaces D-002 ONLY for QA: allow exact Windows portability + loopback fixes in qa/storyboard.mjs and only localhost/bind changes in qa/visual.mjs. No product/native/frontend behavior, design, manifests, locks or dependencies change. This is test-harness integration, not a UI redesign.
D-014 Permit process-local WebView2 debug arguments with verified loopback-only endpoint on a free port for isolated test app. Observe its actual DOM via CDP, not mock. No Device Portal/Developer Mode/registry/firewall/browser-login changes, public listener, original-user profile or product tap.
D-015 Use existing owner credentials only per command (GCM manager + explicit helpfulbriefl URL + interactive false). No global gh/account/helper change. No new analyst remote commits while coder delivery is pending.
### §7 Rakes · append
G-09 INDEX proof fields require backticks and ✅; do not convert native-unavailable to PASS.
G-10 Show/verify live panel before controls. Validate window ownership; single-instance forwarding can invalidate fixture isolation. GetWindowRgn ERROR is not NULLREGION.
G-11 Existing browser mock QA != native WebView2/Windows OLE. Read actual DOM/errors before claiming instrumentation requires product changes.
### §10 Roadmap · replace
W0-BRIEF=done!:Бриф; W1-RECON=done:Разведка PARTIAL; W1B-BASELINE=done:Сборка PARTIAL; W1C-NATIVE=run:Живая панель и стенд; W2-DESIGN=todo!:Макеты; W3b-CORE=todo:Логика; W3c-UI-RAIL=todo:Интерфейс; W4-FIX=todo:Регрессия; W5-POLISH=todo!:Релиз
Next W1C-NATIVE. Native observed failures may complete reconnaissance, but no product defect is closed until an approved repair and real verification.
<!-- /MAP -->
