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
