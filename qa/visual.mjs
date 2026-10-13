// Visual QA: renders every window × theme × state with the mock backend,
// runs layout assertions (overflow / overlap / out-of-bounds) and saves PNGs.
import { chromium } from 'playwright-core';
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { existsSync, mkdirSync } from 'node:fs';
import { extname, join } from 'node:path';
import { execSync } from 'node:child_process';

// fileURLToPath: on Windows `URL.pathname` yields "/c:/..." which mkdir rejects.
import { fileURLToPath } from 'node:url';
const DIST = fileURLToPath(new URL('../dist/', import.meta.url));
const OUT = fileURLToPath(new URL('./shots/', import.meta.url));
mkdirSync(OUT, { recursive: true });
const types = { '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.woff2': 'font/woff2', '.png': 'image/png', '.svg': 'image/svg+xml' };
const server = createServer(async (req, res) => {
  const p = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  const f = join(DIST, p === '/' ? 'index.html' : p);
  if (!existsSync(f)) return res.writeHead(404).end();
  res.writeHead(200, { 'content-type': types[extname(f)] ?? 'application/octet-stream' });
  res.end(await readFile(f));
}).listen(4173, '127.0.0.1');

// Resolve a browser without `which` (it does not exist on Windows).
const CANDIDATES = [
  process.env.CHROME,
  process.env.CHROMIUM,
  'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
  'C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe',
  'C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe',
  '/usr/bin/chromium',
  '/usr/bin/chromium-browser',
  '/usr/bin/google-chrome',
  '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
];
const exe = CANDIDATES.find((p) => p && existsSync(p));
if (!exe) {
  console.error('No browser found; set CHROME=<path>');
  process.exit(1);
}
const browser = await chromium.launch({ executablePath: exe, args: ['--force-device-scale-factor=2'] });
const themes = ['classic-light', 'classic-dark', 'porcelain', 'aurora'];
const WALL = 'html{background:linear-gradient(135deg,#1e3a8a 0%,#6d28d9 45%,#db2777 75%,#f59e0b 100%) !important}';
const scenes = [];
// window sizes follow the new cross-axis geometry (see rail_cross in main.rs):
// compact ≈ 120 px, mini ≈ 92 px, normal ≈ 150 px
for (const t of themes) {
  scenes.push({ name: `rail-right-${t}`, q: `w=rail&theme=${t}`, size: [130, 700], hover: 2 });
  scenes.push({ name: `flyout-metrics-${t}`, q: `w=flyout&kind=metrics&id=m1&theme=${t}`, size: [304, 236], wait: 3600 });
  scenes.push({ name: `flyout-info-${t}`, q: `w=flyout&kind=info&id=m3&theme=${t}`, size: [268, 158] });
  scenes.push({ name: `settings-${t}`, q: `w=settings&theme=${t}`, size: [560, 720] });
}
scenes.push({ name: 'rail-bottom-aurora', q: 'w=rail&theme=aurora&side=bottom', size: [860, 130], hover: 1 });
scenes.push({ name: 'rail-bottom-classic-dark', q: 'w=rail&theme=classic-dark&side=bottom', size: [860, 130] });
scenes.push({ name: 'rail-empty-porcelain', q: 'w=rail&theme=porcelain&empty=1', size: [130, 380] });
scenes.push({ name: 'rail-left-classic-light-L', q: 'w=rail&theme=classic-light&side=left&size=44', size: [150, 800] });
scenes.push({ name: 'rail-mini-aurora', q: 'w=rail&theme=aurora&scale=mini', size: [100, 620] });
scenes.push({ name: 'rail-no-meters-porcelain', q: 'w=rail&theme=porcelain&meters=0', size: [130, 620] });
// external-drop path: drive the mock's __railDrop hook the way OLE would
scenes.push({
  name: 'rail-drop-aurora',
  q: 'w=rail&theme=aurora',
  size: [130, 700],
  drop: { x: 60, y: 300, paths: ['C:\\Program Files\\Code\\Code.exe'] },
});

const failures = [];
for (const sc of scenes) {
  const page = await browser.newPage({ viewport: { width: sc.size[0], height: sc.size[1] }, deviceScaleFactor: 2 });
  await page.goto(`http://127.0.0.1:4173/?${sc.q}`);
  await page.addStyleTag({ content: WALL });
  await page.waitForTimeout(sc.wait ?? 1400);
  if (sc.hover !== undefined) {
    const items = await page.$$('.dock-item');
    const el = items[sc.hover];
    if (el) {
      const b = await el.boundingBox();
      await page.mouse.move(b.x + b.width / 2, b.y + b.height / 2, { steps: 6 });
      await page.waitForTimeout(700);
    }
  }
  if (sc.drop) {
    await page.evaluate((d) => window.__railDrop({ type: 'enter', ...d }), sc.drop);
    await page.evaluate((d) => window.__railDrop({ type: 'over', ...d }), sc.drop);
    await page.waitForTimeout(250);
  }
  // ---- layout assertions
  const issues = await page.evaluate(() => {
    const out = [];
    const W = innerWidth, H = innerHeight;
    // 1) nothing visible outside the window
    for (const el of document.querySelectorAll('.rail-bg, .card, .set')) {
      const r = el.getBoundingClientRect();
      if (r.left < -0.5 || r.top < -0.5 || r.right > W + 0.5 || r.bottom > H + 0.5) out.push(`out-of-window: .${el.className} ${r.left|0},${r.top|0},${r.right|0},${r.bottom|0}`);
    }
    // 2) text overflow (clipped labels)
    for (const el of document.querySelectorAll('.card *, .set *, .rail *')) {
      const cs = getComputedStyle(el);
      if (el.children.length === 0 && el.textContent.trim() && cs.overflow !== 'visible' && cs.textOverflow !== 'ellipsis' && el.scrollWidth > el.clientWidth + 1) out.push(`text-overflow: ${el.className} "${el.textContent.trim().slice(0, 20)}"`);
    }
    // 3) children outside their container (cards/rows)
    for (const box of document.querySelectorAll('.card, .set, .tile-m, .stat, .row')) {
      const b = box.getBoundingClientRect();
      for (const c of box.querySelectorAll(':scope > *')) {
        const r = c.getBoundingClientRect();
        if (r.width && (r.left < b.left - 1 || r.right > b.right + 1 || r.top < b.top - 1 || r.bottom > b.bottom + 1)) out.push(`child-overflow: .${box.className.split(' ')[0]} > .${c.className.baseVal ?? c.className}`);
      }
    }
    // 4) dock slots must not overlap at rest
    const slots = [...document.querySelectorAll('.dock-item')].map((e) => e.getBoundingClientRect());
    for (let i = 1; i < slots.length; i++) {
      const a = slots[i - 1], b = slots[i];
      const ov = Math.min(a.right, b.right) - Math.max(a.left, b.left) > 1 && Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top) > 1;
      if (ov) out.push(`slot-overlap ${i - 1}/${i}`);
    }
    return out;
  });
  if (sc.drop) {
    // a live drop indicator must be showing
    const has = await page.$('.drop-slot');
    if (!has) failures.push(`${sc.name}: no drop-slot rendered`);
  }
  await page.screenshot({ path: `${OUT}${sc.name}.png`, omitBackground: false });
  if (issues.length) failures.push(`${sc.name}: ${issues.join(' | ')}`);
  console.log(`${issues.length ? 'FAIL' : 'PASS'}  ${sc.name}${issues.length ? '  ' + issues.join(' | ') : ''}`);
  await page.close();
}
await browser.close();
server.close();
console.log(failures.length ? `\n${failures.length} scene(s) failed` : '\nALL PASS');
process.exit(failures.length ? 1 : 0);
