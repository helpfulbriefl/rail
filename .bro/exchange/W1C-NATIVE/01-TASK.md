---
ver: v3.0 · build: JOIN · wave: W1C-NATIVE · layer: integration
t_issued: 2026-10-13 23:28 · tasks: 5 · pass_criteria: 5 · an_turn: 4
allow: .bro/**; README.md; .gitignore; qa/storyboard.mjs; qa/visual.mjs
modules: STAND, METRICS · mirror: - · risky: no
road: W0-BRIEF=done!:Бриф; W1-RECON=done:Разведка PARTIAL; W1B-BASELINE=done:Сборка PARTIAL; W1C-NATIVE=run:Живая панель и стенд; W2-DESIGN=todo!:Макеты; W3b-CORE=todo:Логика; W3c-UI-RAIL=todo:Интерфейс; W4-FIX=todo:Регрессия; W5-POLISH=todo!:Релиз
---
# TASK W1C-NATIVE — expose the live test panel; repair only the QA harness

## 0 Read
NEXT, previous ACCEPT, MAP, W1B native-baseline/INDEX/meter-trace/provenance; current main.rs/core.rs/config.rs/diag.rs/win.rs; Rail.tsx/Tile.tsx/useApp.ts/bridge/api.ts; qa/storyboard.mjs and qa/visual.mjs. Apply ACCEPT LOG/MAP using ignored local BRO copy from previous wave; original SYS/public prompt snapshot unchanged. Do not init/overwrite analyst files. No product/permission/native-config edits; no release.

## 1 Context
Q=C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish. Original P remains read-only. Current delivery 6f48c5066441c2cae7a219fad74d9178c345b173. Reuse source-built exe SHA-256 43a13486de178300e9c36a466471522ed2dba647a965004a663500e23db5a838 from f7f740d; hash before use. No new Rust rebuild needed while product inputs unchanged. Compare committed product inputs vs f7f740d (src/, src-tauri/, assets/, public/, index.html, manifests/locks/configs); report drift, don't assume 5 selected hashes cover all files.
Goal is valid observations, not product fixes. Native failures with correct prerequisites count as valid baseline; unavailable/incorrect targets do not. Do not spend another full wave on four hidden-window click attempts.

## 2 Decisions
D-012..D-015 per ACCEPT. Five integration tasks. Exact QA exceptions only; preserve animation scenes, times, mock data, assertions, existing product code/style. All expensive/unbounded operations avoided. No new packages/browser downloads/global installs. Generated run/tooling/artifacts and configs ignored and test-owned. No credential/account/global config/remote mutation. No data deletion or unrelated process termination. B-001..B-005 remain open. License/CI/site/profile stay outside this wave.

## 3 PASS/proof (one criterion per task)
1 Owned source-built test PID/window verified; actual monitor/work-area/DPI/client-to-screen/region and visible non-empty DOM shown, or precise diagnosed bootstrap failure with live errors and tested preconditions. Hidden 'no dialog' is not a valid observation.
2 Exact QA fixes committed; npm run qa has true exit/duration, separate mock visual/storyboard status and generated storyboard file names/dimensions. Failure is captured, no 'all EXIT 0' summary with a failed leg.
3 '+' valid physical click and dialog selection/cancel; 2-item reorder/restart/hold; actual desktop/Explorer OLE entry, cancellation and hidden->visible reveal, expected/actual for three attempts. A product failure can satisfy observation if its target/prerequisites and event chain are proved. Unsupported automation is not 'impossible in principle'. If native bootstrap blocks all, provide root evidence, no repetitive invalid attempts and no functional PASS.
4 60-second paired timestamped Rail-rendered footer + independent system reference captured on same test machine, or a concrete capture/producer failure proven by live DOM/error record. Publish sanitized finite values with units and timing, not only references to ignored CSV. No inferred 'not a leak'.
5 Published proof with parser-correct INDEX, redacted gate/provenance/sample evidence, honest REPORT/README and delivery verification. Boundaries explicitly evidenced; unknowns preserved. Tag only w1c-native-done once, no moving old/new public tags. Main push before next analyst writes.
A full wave PASS requires complete evidence for all five; an exact bootstrap blocker -> PARTIAL/FAIL, not an excuse to close native defects.

## 4 Key code / exact allowed changes
### qa/storyboard.mjs
Add `import { fileURLToPath } from 'node:url';`.
Replace EXACT:
`const DIST = new URL('../dist/', import.meta.url).pathname;`
with `const DIST = fileURLToPath(new URL('../dist/', import.meta.url));`.
Replace EXACT:
`const OUT = new URL('./shots/', import.meta.url).pathname;`
with `const OUT = fileURLToPath(new URL('./shots/', import.meta.url));`.
Remove execSync import (unused after next change). Copy CANDIDATES and `const exe`/missing-browser guard VERBATIM from existing qa/visual.mjs. Replace storyboard chromium.launch using `which chromium` with `const browser = await chromium.launch({ executablePath: exe });`. Do not add force-device-scale args not previously in storyboard (its contexts already set deviceScaleFactor).
Replace `}).listen(4174);` with `}).listen(4174, '127.0.0.1');`.
Replace page.goto URL host localhost:4174 with 127.0.0.1:4174.
Replace `.toFile(`${OUT}${name}.png`);` with `.toFile(join(OUT, `${name}.png`));`.
Do not change scene data, viewport/timing, mock hooks, clipping, WALL or geometry checks.
### qa/visual.mjs (only two changes)
Replace `}).listen(4173);` with `}).listen(4173, '127.0.0.1');`.
Replace page.goto URL host localhost:4173 with 127.0.0.1:4173.
No other edits, cleanup/refactor or assertions change.
### .bro/exchange/W1C-NATIVE/scripts/cdp-observe.mjs
Paste the following VERBATIM. It observes actual WebView2 via local CDP without input, CSS injection, overrides or mock events. Run only after the PID/listener/profile safety checks in task 1/4. Raw output stays in ignored run/. Inspect/redact before creating public proof. Node 24 built-in fetch/WebSocket; no new package. Connecting does not make a browser screenshot/DOM click a native OLE proof.
```js
import fs from 'node:fs';
import path from 'node:path';
const [portArg, outArg, secondsArg = '8'] = process.argv.slice(2);
const port = Number(portArg);
const seconds = Number(secondsArg);
if (!Number.isInteger(port) || port < 1024 || port > 65535 || !outArg || !Number.isInteger(seconds) || seconds < 1 || seconds > 65) throw new Error('Usage: node cdp-observe.mjs PORT RUN/output.json SECONDS(1..65)');
const deadline = setTimeout(() => { console.error('STOP: CDP capture deadline exceeded'); process.exit(2); }, (seconds + 30) * 1000);
deadline.unref();
const out = path.resolve(outArg);
const targets = await fetch(`http://127.0.0.1:${port}/json/list`, { signal: AbortSignal.timeout(10000) }).then(r => { if (!r.ok) throw new Error(`CDP HTTP ${r.status}`); return r.json(); });
const pages = targets.filter(t => t.type === 'page');
if (!pages.length) throw new Error('No live WebView page targets');
const events = [], samples = [], clients = [];
function open(t) {
  return new Promise((resolve, reject) => {
    const u = new URL(t.webSocketDebuggerUrl);
    if (!['127.0.0.1', 'localhost', '[::1]'].includes(u.hostname) || Number(u.port) !== port) return reject(new Error('CDP endpoint is not approved loopback port'));
    u.hostname = '127.0.0.1';
    const ws = new WebSocket(u), pending = new Map(); let seq = 0;
    const timer = setTimeout(() => { ws.close(); reject(new Error('CDP open timeout')); }, 10000);
    ws.addEventListener('open', () => { clearTimeout(timer); resolve({ target: t, ws, call(method, params = {}) { return new Promise((done, fail) => { const id = ++seq; const timeout = setTimeout(() => { pending.delete(id); fail(new Error(`${method} timeout`)); }, 10000); pending.set(id, { done, fail, timeout }); ws.send(JSON.stringify({ id, method, params })); }); } }); });
    ws.addEventListener('error', () => { clearTimeout(timer); reject(new Error('CDP websocket error')); });
    ws.addEventListener('message', e => {
      const m = JSON.parse(String(e.data));
      if (m.id && pending.has(m.id)) { const p = pending.get(m.id); pending.delete(m.id); clearTimeout(p.timeout); m.error ? p.fail(new Error(JSON.stringify(m.error))) : p.done(m.result); }
      else if (['Runtime.exceptionThrown', 'Runtime.consoleAPICalled', 'Log.entryAdded'].includes(m.method)) events.push({ at: new Date().toISOString(), target: t.id, method: m.method, detail: JSON.stringify(m.params).slice(0, 4000) });
    });
    ws.addEventListener('close', () => { for (const p of pending.values()) { clearTimeout(p.timeout); p.fail(new Error('CDP disconnected')); } pending.clear(); });
  });
}
const expression = `(() => {
 const rect = e => { const r=e.getBoundingClientRect(); return {x:r.x,y:r.y,width:r.width,height:r.height}; };
 return {url:location.href,label:document.documentElement.dataset.window??null,ready:document.readyState,
 viewport:{width:innerWidth,height:innerHeight,dpr:devicePixelRatio},rootChildren:document.getElementById('root')?.childElementCount??null,
 rail:document.querySelector('.rail')?rect(document.querySelector('.rail')):null,
 footer:document.querySelector('.rail-foot')?.innerText??null,
 buttons:[...document.querySelectorAll('button')].slice(0,40).map(e=>({label:e.getAttribute('aria-label')??e.title??'',text:e.innerText.slice(0,80),disabled:e.disabled,rect:rect(e),display:getComputedStyle(e).display,visibility:getComputedStyle(e).visibility})),
 bodyText:document.body.innerText.slice(0,3000)};
})()`;
try {
  for (const t of pages) { const c = await open(t); clients.push(c); await c.call('Runtime.enable'); await c.call('Log.enable'); }
  for (let i = 0; i < seconds; i++) {
    for (const c of clients) { const at = new Date().toISOString(); try { const r = await c.call('Runtime.evaluate', { expression, returnByValue: true }); samples.push({ at, target:c.target.id, value:r.result?.value??null, exception:r.exceptionDetails??null }); } catch(e) { samples.push({ at, target:c.target.id, error:e.message }); } }
    await new Promise(r=>setTimeout(r,1000));
  }
  fs.mkdirSync(path.dirname(out), { recursive:true });
  fs.writeFileSync(out, JSON.stringify({source:'LIVE WebView2 CDP; read-only DOM observation, not mock',started: samples[0]?.at??null,targets:pages.map(t=>({id:t.id,url:t.url,title:t.title})),events,samples},null,2));
  console.log(`Saved ${samples.length} observations to test-owned run file`);
} finally { for (const c of clients) c.ws.close(); }
process.exit(0);
```
Sources: Microsoft https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/debug-visual-studio-code documents process-local WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9222; no VSCode config/Developer Mode installation required for this task. Win32 https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn distinguishes ERROR/no region from NULLREGION. CDP is diagnostic, not guaranteed available under every runtime policy.

## 5 Tasks
1. Establish live fixture FIRST. Pull --ff-only; begin with project-local BRO helper; apply ACCEPT. Use new test-owned run dir and source-built copy with existing supported config isolation, autostart disabled; inspect startup effects before launching. Confirm PID remains alive, actual process image/hash, and every target HWND owner PID. Detect single-instance forwarding; don't query another running Rail as this fixture. Close only prior test-owned processes whose executable path/ownership is proved. If an unrelated owner instance prevents isolated test, report exactly; don't kill it. Record virtual-screen bounds and all relevant monitor work areas, not a default guessed 1920x1040. Show via actual tray Show action or supported existing show route; use test-owned pinned config to prevent subsequent auto-hide (pinned alone need not open the window). Require visible window and live rendered controls/nonempty hit-rect before trying '+'. Measure actual GetWindowRgn/GetRgnBox with correct HWND; 'skip empty' does not prove NULLREGION. DPI=0 -> invalid measurement, never a scale; reconcile physical GetClientRect/ClientToScreen with DOM CSS pixels and devicePixelRatio. If still no root/control render after ONE verified show attempt, capture actual frontend bootstrap/IPC errors via task 4 and read implicated code; do not fix product and do not repeat blind clicks.
2. QA portability fix. Apply §4 exactly, no broader source changes. Run node --check qa/storyboard.mjs and qa/visual.mjs, npm run typecheck, npm run lint:css, npm run qa (existing npm ci not repeated). Preserve exact per-command exit codes and duration; record any real failure. Inspect generated story-appear/story-launch PNG existence/dimensions. These are existing mock scenes only, no new design and no native acceptance claim. Product/locks must remain unchanged; no dependency upgrade. Max 10 minutes per command, no runaway retry loop.
3. Genuine controls/gestures. Once task 1 visible fixture is proved, locate '+' using live DOM rectangle or cropped live window, convert CSS->physical->screen with measured client origin/DPR, and send a real Windows click inside the visible button. CDP DOM click may diagnose invoke path but is NOT physical hit-test evidence. Identify dialog by PID/window/control tree; do not rely only on #32770. Select existing test-owned shortcuts and cancel one trial. Seeded config is separate fixture-only preparation, not evidence '+' works. Then reorder two test items, restart the same isolated fixture and compare order; hold on a real item for configured delay +10s and record card transitions. For OLE use actual shell sender mouse-down/move/up (desktop and Explorer), not SetCursorPos alone or __railDrop. Test open panel, hidden->edge reveal, and cancel. Record timestamps and valid expected/actual; three trials each critical interaction. Raw desktop screenshots/videos forbidden; crop to Rail/dialog. If source application's reported bug can be exercised from the Owner's file manager as well, record sender name/version distinctly, not a generic Explorer claim.
4. Live DOM/errors/meters, no product tap. In a separate diagnostic launch of ONLY the fixture exe use child-process-only WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS='--remote-debugging-port=9222 --remote-debugging-address=127.0.0.1'; port must be free. Create a test-owned WebView profile via the runtime's supported child-process setting only if needed, never redirect a personal profile. Verify listener belongs to fixture/WebView child tree and binds loopback only; if it binds public/all interfaces or port belongs to another process, terminate only owned fixture/debug processes and report, no firewall/registry/system change. Environment must not persist in global/user config; no DevTools of daily browser. Verify /json/list has owned WebView targets. Run §4 probe 8s for boot/visible controls, then 60s for meters in parallel with existing independent PerformanceCounter/WMI reference. Records use same host clock; align timestamps and document CPU sampling intervals, EMA lag, RAM units/denominator. It reads actual .rail-foot text, not backend synthetic telemetry. Empty footer/label/root is evidence to diagnose, not permission to inject fake data. If early errors happened before attaching, a single explicitly labelled page reload may aid diagnostics; never call that cold-start proof. Use native screenshot/actual region to distinguish displayed state from merely present DOM. Label diagnostic launch vs normal launch; repeat at least the native visibility/'+' check WITHOUT CDP flags before asserting normal-mode outcome. Shut down owned listener/process after capture. No source edit to add instrumentation. If CDP is unsupported, give exact runtime/endpoint evidence; do not claim 'IPC-only means inaccessible'. Sanitize raw run JSON/CSV into compact published proof and paired samples; don't publish credentials, personal paths or raw dumps.
5. Proof/corrections/delivery. Do not rewrite W1B REPORT/INDEX; append W1B 04-ERRATA.md noting proof-format issue, pre-push metadata now delivered, hidden-target trials, unmeasured region/monitor, and actual gate status. New INDEX follows template: `file` references in backticks, ✅ only when criterion fully proved, ❌/— with reason otherwise. Publish redacted gate summary (command/exit/duration), exe hash, window ownership/geometry/region, cropped application images, valid interaction timeline and paired meter values. Local ignored CSV/raw logs alone cannot satisfy proof. Do not auto-complete every task line or target proof_ok==proof_req via formatting. Update README checked QA status and native issues honestly. Use helper check/finish only from ignored tooling, mirror:-; original SYS untouched. Stage explicit allowed paths; no raw run/tooling/cache/bin/user config. Preserve fixed script scopes exactly. Push with already proved command-local GCM manager route and explicit https://helpfulbriefl@github.com/helpfulbriefl/rail.git, interactive false/GIT_TERMINAL_PROMPT=0 transient then restore. No gh switch, token extraction, force/reset/rebase/remote change/all-tags push. If delivery blocked, no new analyst remote write and report exact non-secret blocker. Final REPORT parent/tested-source identifiers distinct; verify final remote main and tag vs local, report full SHAs. End 'Готово -> напиши аналитику: BRO CHECK'.

## Z W1B defects
Z-01 Native visibility/PID/geometry prerequisite absent; all previous '+' attempts INVALID.
Z-02 INDEX syntax causes helper zero; incomplete native/meter rows stay incomplete. Published structured exe log accepted as provenance, not blanket gate proof.
Z-03 Storyboard pathname/which Windows defects + loopback binding; exact QA-only exceptions approved in D-013. Visual 23 scenes are mock-backed.
Z-04 Pair live footer and system reference; all B-001..B-005 remain open. No repeated unobserved stand and no product bug closure from unavailable tests.
