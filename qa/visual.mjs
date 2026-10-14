// Visual QA: renders every window × theme × state with the mock backend,
// runs layout assertions (overflow / overlap / out-of-bounds / footer
// reachability) and saves PNGs.
//
// Scroll-awareness: a child extending *below* a scrollable container is
// content, not a defect — vertical clipping is only asserted for
// non-scrollable boxes, while horizontal overflow is always a defect.
//
// The suite carries a **negative fixture**: a deliberately broken scene that
// MUST be reported. If QA passes it, the assertions are blind and the run
// fails — a green board that cannot see a real defect is worse than no board.
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
// all six approved themes — glass and studio used to be invisible to QA
const themes = ['classic-light', 'classic-dark', 'porcelain', 'aurora', 'glass', 'studio'];
const WALL = 'html{background:linear-gradient(135deg,#1e3a8a 0%,#6d28d9 45%,#db2777 75%,#f59e0b 100%) !important}';
const scenes = [];
// window sizes follow the new cross-axis geometry (see rail_cross in main.rs):
// compact ≈ 120 px, mini ≈ 92 px, normal ≈ 150 px
for (const t of themes) {
  scenes.push({ name: `rail-right-${t}`, q: `w=rail&theme=${t}`, size: [130, 700], hover: 2 });
  scenes.push({ name: `flyout-metrics-${t}`, q: `w=flyout&kind=metrics&id=m1&theme=${t}`, size: [304, 236], wait: 3600 });
  scenes.push({ name: `flyout-info-${t}`, q: `w=flyout&kind=info&id=m3&theme=${t}`, size: [268, 158] });
  // settings is scrolled to the bottom before the assertions: the footer must
  // be reachable there, not clipped by a fixed-height layout
  scenes.push({ name: `settings-${t}`, q: `w=settings&theme=${t}`, size: [560, 720], scroll: '.set', footer: '.set-foot' });
}
scenes.push({ name: 'rail-bottom-aurora', q: 'w=rail&theme=aurora&side=bottom', size: [860, 130], hover: 1 });
scenes.push({ name: 'rail-bottom-classic-dark', q: 'w=rail&theme=classic-dark&side=bottom', size: [860, 130] });
scenes.push({ name: 'rail-empty-porcelain', q: 'w=rail&theme=porcelain&empty=1', size: [130, 380] });
scenes.push({ name: 'rail-left-classic-light-L', q: 'w=rail&theme=classic-light&side=left&size=44', size: [150, 800] });
scenes.push({ name: 'rail-mini-aurora', q: 'w=rail&theme=aurora&scale=mini', size: [100, 620] });
scenes.push({ name: 'rail-no-meters-porcelain', q: 'w=rail&theme=porcelain&meters=0', size: [130, 620] });
// crowded dock: full-size icons at normal scale — the geometry that used to clip
scenes.push({ name: 'rail-crowd-studio', q: 'w=rail&theme=studio&scale=normal&size=44', size: [180, 940], hover: 3 });
// external-drop path: drive the mock's __railDrop hook the way OLE would
scenes.push({
  name: 'rail-drop-aurora',
  q: 'w=rail&theme=aurora',
  size: [130, 700],
  drop: { x: 60, y: 300, paths: ['C:\\Program Files\\Code\\Code.exe'] },
});
// system monitor: both rankings live, empty, stale, and a refused close
scenes.push({ name: 'flyout-sys-aurora', q: 'w=flyout&kind=sys&id=sys&theme=aurora', size: [330, 380], wait: 2600 });
scenes.push({ name: 'flyout-sys-empty-porcelain', q: 'w=flyout&kind=sys&id=sys&theme=porcelain&sysmon=empty', size: [330, 380] });
scenes.push({ name: 'flyout-sys-stale-glass', q: 'w=flyout&kind=sys&id=sys&theme=glass&stale=1', size: [330, 380] });
scenes.push({ name: 'flyout-sys-refused-studio', q: 'w=flyout&kind=sys&id=sys&theme=studio&close=refused', size: [330, 380], clickClose: true });
// more than five eligible apps: the UI must cap itself at five live rows
scenes.push({ name: 'flyout-sys-crowd-studio', q: 'w=flyout&kind=sys&id=sys&theme=studio', size: [330, 380], wait: 2600, maxRows: 5, focusClose: true });
// the CPU ranking tab, driven through the real tab button
scenes.push({ name: 'flyout-sys-cpu-crowd-studio', q: 'w=flyout&kind=sys&id=sys&theme=studio', size: [330, 380], wait: 2600, clickTab: 'CPU', maxRows: 5 });
// cold open before the first sample: first rows must arrive without a tab switch
scenes.push({ name: 'flyout-sys-cold-porcelain', q: 'w=flyout&kind=sys&id=sys&theme=porcelain&cold=1', size: [330, 380], wait: 3200 });
// sample younger than the staleness threshold: must NOT be marked stale
scenes.push({ name: 'flyout-sys-recent-glass', q: 'w=flyout&kind=sys&id=sys&theme=glass&recent=1', size: [330, 380], wait: 2600, noSelector: '.sysmon-stale' });
// interaction-freeze event sequences — behavioural regression, not pixels.
// Each drives a real pointer/focus/keyboard sequence against a scripted mock
// fixture and asserts the freeze edge case a screenshot cannot observe.
scenes.push({ name: 'seq-hover-cold-porcelain', q: 'w=flyout&kind=sys&id=sys&theme=porcelain&cold=1', size: [330, 380], wait: 150, seq: 'hover-cold' });
scenes.push({ name: 'seq-leave-focus-glass', q: 'w=flyout&kind=sys&id=sys&theme=glass&vanish=1', size: [330, 380], wait: 150, seq: 'leave-focus' });
scenes.push({ name: 'seq-tab-focus-studio', q: 'w=flyout&kind=sys&id=sys&theme=studio&vanish=1', size: [330, 380], wait: 150, seq: 'tab-focus' });
scenes.push({ name: 'seq-all-vanish-aurora', q: 'w=flyout&kind=sys&id=sys&theme=aurora&vanishall=1', size: [330, 380], wait: 150, seq: 'all-vanish' });
scenes.push({ name: 'seq-loading-totals-glass', q: 'w=flyout&kind=sys&id=sys&theme=glass&nosample=1', size: [330, 380], wait: 150, seq: 'loading-totals' });
scenes.push({ name: 'seq-new-leader-porcelain', q: 'w=flyout&kind=sys&id=sys&theme=porcelain&shift=1', size: [330, 380], wait: 150, seq: 'new-leader' });
// persistence failure must surface in the UI
scenes.push({ name: 'rail-savefail-aurora', q: 'w=rail&theme=aurora&savefail=1', size: [130, 700], needSelector: '.save-warn' });
// negative fixture: a deliberately broken layout that QA MUST catch
scenes.push({
  name: 'negative-broken-layout',
  q: 'w=rail&theme=aurora',
  size: [130, 700],
  inject: '.rail-bg{transform:translateX(900px) !important}',
  expectFail: true,
});

const sha = (() => {
  try {
    return execSync('git rev-parse --short HEAD', { cwd: fileURLToPath(new URL('..', import.meta.url)) }).toString().trim();
  } catch {
    return 'unknown';
  }
})();
console.log(`source ${sha} · ${scenes.length} scenes`);

// Event-sequence regression harness: drives real Playwright pointer/focus/
// keyboard sequences against the scripted mock fixtures and asserts the
// interaction-freeze behaviour. These are behavioural assertions, not pixels —
// they catch the freeze edge cases a screenshot cannot see.
async function runSeq(page, name, issues) {
  const bodyCenter = async () => {
    const b = await page.$('.sysmon-body');
    if (!b) return null;
    const r = await b.boundingBox();
    return r ? { x: r.x + r.width / 2, y: r.y + r.height / 2 } : null;
  };
  if (name === 'hover-cold') {
    // pointer already inside before the first sample: rows must still arrive
    await page.waitForSelector('.sysmon-body');
    const c = await bodyCenter();
    if (c) await page.mouse.move(c.x, c.y, { steps: 4 });
    await page.waitForTimeout(2800);
    const rows = await page.$$eval('.sysmon-row', (e) => e.length);
    if (rows < 1) issues.push('hover-cold: no rows appeared while the pointer was already inside (empty frame latched the freeze)');
  } else if (name === 'leave-focus') {
    // pointer enters (arm) then leaves while keyboard focus keeps its guard
    await page.waitForSelector('.sysmon-row .sysmon-close:not([disabled])');
    const c = await bodyCenter();
    if (c) await page.mouse.move(c.x, c.y, { steps: 4 });
    await page.focus('.sysmon-row .sysmon-close:not([disabled])');
    await page.mouse.move(2, 2, { steps: 3 });
    await page.waitForTimeout(3400);
    const gone = await page.$$eval('.sysmon-row[data-gone]', (e) => e.length);
    if (gone < 1) issues.push('leave-focus: freeze dropped when the pointer left while focus remained (disappeared app vanished instead of a tombstone)');
  } else if (name === 'tab-focus') {
    // internal Tab transitions between rows must not be read as a focus-leave.
    // Tab lands on a row that survives the disappearance (the vanished app is
    // a different slot), so the focused node is not unmounted by the rank
    // update and the freeze can only persist if internal blurs are ignored.
    await page.waitForSelector('.sysmon-row .sysmon-close:not([disabled])');
    await page.focus('.sysmon-row .sysmon-close:not([disabled])');
    await page.keyboard.press('Tab');
    await page.keyboard.press('Tab');
    await page.waitForTimeout(3400);
    const gone = await page.$$eval('.sysmon-row[data-gone]', (e) => e.length);
    if (gone < 1) issues.push('tab-focus: an internal Tab between rows released the freeze (disappeared app vanished instead of a tombstone)');
  } else if (name === 'all-vanish') {
    // every candidate disappears during a held freeze: slots must stay put
    await page.waitForSelector('.sysmon-row');
    const c = await bodyCenter();
    if (c) await page.mouse.move(c.x, c.y, { steps: 4 });
    await page.waitForTimeout(3400);
    const gone = await page.$$eval('.sysmon-row[data-gone]', (e) => e.length);
    const empty = await page.$('.sysmon-empty');
    if (gone < 5) issues.push(`all-vanish: expected five frozen tombstone slots, got ${gone}`);
    if (empty) issues.push('all-vanish: an empty panel replaced the frozen slots during an active freeze');
  } else if (name === 'loading-totals') {
    // never-sampled frame: totals are placeholders, not fabricated zeros
    await page.waitForSelector('.sysmon-foot');
    const tot = await page.$eval('.sysmon-tot', (e) => e.textContent || '');
    if (tot.includes('0%')) issues.push('loading-totals: footer shows a fabricated zero CPU before the first sample');
    if (!tot.includes('—')) issues.push('loading-totals: footer is missing the loading placeholder before the first sample');
    const empty = await page.$('.sysmon-empty');
    if (!empty) issues.push('loading-totals: expected the loading placeholder body when no sample exists');
  } else if (name === 'new-leader') {
    // outside any interaction the live ranking must admit a new top app
    await page.waitForTimeout(3400);
    const has = await page.$$eval('.sysmon-row .sysmon-name', (els) => els.some((e) => /miner/i.test(e.textContent || '')));
    if (!has) issues.push('new-leader: a new top app outside interaction did not enter the live ranking');
  }
}

const failures = [];
for (const sc of scenes) {
  const page = await browser.newPage({ viewport: { width: sc.size[0], height: sc.size[1] }, deviceScaleFactor: 2 });
  const seqIssues = [];
  await page.goto(`http://127.0.0.1:4173/?${sc.q}`);
  await page.addStyleTag({ content: WALL });
  if (sc.inject) await page.addStyleTag({ content: sc.inject });
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
  if (sc.clickClose) {
    const btn = await page.$('.sysmon-row .sysmon-close:not([disabled])');
    if (btn) {
      await btn.click();
      await page.waitForTimeout(350);
    }
  }
  if (sc.clickTab) {
    // drive the real tab button: proves the second ranking renders
    const btns = await page.$$('.sysmon-tabs button');
    for (const b of btns) {
      const txt = (await b.textContent() ?? '').trim();
      if (txt === sc.clickTab) {
        await b.click();
        break;
      }
    }
    await page.waitForTimeout(400);
  }
  if (sc.focusClose) {
    // keyboard reachability: Tab must land on a close button
    for (let i = 0; i < 12; i++) {
      const cls = await page.evaluate(() => String(document.activeElement?.className ?? ''));
      if (cls.includes('sysmon-close')) break;
      await page.keyboard.press('Tab');
    }
    const ok = await page.evaluate(() => String(document.activeElement?.className ?? '').includes('sysmon-close'));
    if (!ok) issues.push('keyboard: close button not reachable by Tab');
  }
  if (sc.scroll) {
    // scroll the container to its bottom; the footer assertions run right after
    await page.evaluate((sel) => {
      const el = document.querySelector(sel);
      if (el) el.scrollTop = el.scrollHeight;
    }, sc.scroll);
    await page.waitForTimeout(150);
  }
  if (sc.seq) await runSeq(page, sc.seq, seqIssues);
  // ---- layout assertions
  const issues = await page.evaluate((cfg) => {
    const out = [];
    const W = innerWidth, H = innerHeight;
    // 1) nothing visible outside the window
    for (const el of document.querySelectorAll('.rail-bg, .card, .set')) {
      const r = el.getBoundingClientRect();
      if (r.left < -0.5 || r.top < -0.5 || r.right > W + 0.5 || r.bottom > H + 0.5) out.push(`out-of-window: .${el.className} ${r.left | 0},${r.top | 0},${r.right | 0},${r.bottom | 0}`);
    }
    // 2) text overflow (clipped labels) — horizontal only: vertical overflow
    // inside a scrollable box is content, and ellipsis covers the rest
    for (const el of document.querySelectorAll('.card *, .set *, .rail *')) {
      const cs = getComputedStyle(el);
      if (el.children.length === 0 && el.textContent.trim() && cs.overflow !== 'visible' && cs.textOverflow !== 'ellipsis' && el.scrollWidth > el.clientWidth + 1) out.push(`text-overflow: ${el.className} "${el.textContent.trim().slice(0, 20)}"`);
    }
    // 3) children outside their container — scroll-aware per axis:
    //    horizontal overflow is always a defect; vertical overflow is only a
    //    defect when the container cannot scroll (otherwise it is just content
    //    below the fold, which the scroll test above covers properly)
    for (const box of document.querySelectorAll('.card, .set, .tile-m, .stat, .row, .sysmon-row')) {
      const cs = getComputedStyle(box);
      const canScrollY = cs.overflowY === 'auto' || cs.overflowY === 'scroll';
      const b = box.getBoundingClientRect();
      for (const c of box.querySelectorAll(':scope > *')) {
        const r = c.getBoundingClientRect();
        if (!r.width) continue;
        const name = `.${box.className.split(' ')[0]} > .${c.className.baseVal ?? c.className}`;
        if (r.left < b.left - 1 || r.right > b.right + 1) out.push(`child-overflow-x: ${name}`);
        if (!canScrollY && (r.top < b.top - 1 || r.bottom > b.bottom + 1)) out.push(`child-overflow-y: ${name}`);
      }
    }
    // 4) dock slots must not overlap at rest
    const slots = [...document.querySelectorAll('.dock-item')].map((e) => e.getBoundingClientRect());
    for (let i = 1; i < slots.length; i++) {
      const a = slots[i - 1], b = slots[i];
      const ov = Math.min(a.right, b.right) - Math.max(a.left, b.left) > 1 && Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top) > 1;
      if (ov) out.push(`slot-overlap ${i - 1}/${i}`);
    }
    // 5) footer reachability: after scrolling the container to its bottom the
    //    footer must sit fully inside it (the old fixed-height layout pushed
    //    the footer out of the window with no way to reach it)
    if (cfg.footer) {
      const set = document.querySelector(cfg.scroll ?? '.set');
      const foot = document.querySelector(cfg.footer);
      if (!set || !foot) out.push(`footer-missing: ${cfg.footer}`);
      else {
        const sr = set.getBoundingClientRect();
        const fr = foot.getBoundingClientRect();
        if (fr.bottom > sr.bottom + 1 || fr.top < sr.top - 1) out.push(`footer-unreachable: foot ${fr.top | 0}..${fr.bottom | 0} vs box ${sr.top | 0}..${sr.bottom | 0}`);
      }
    }
    return out;
  }, { footer: sc.footer ?? null, scroll: sc.scroll ?? null });
  issues.push(...seqIssues);
  if (sc.drop) {
    // a live drop indicator must be showing
    const has = await page.$('.drop-slot');
    if (!has) issues.push('no drop-slot rendered');
  }
  if (sc.needSelector && !(await page.$(sc.needSelector))) issues.push(`missing ${sc.needSelector}`);
  if (sc.noSelector && (await page.$(sc.noSelector))) issues.push(`unexpected ${sc.noSelector}`);
  if (sc.maxRows !== undefined) {
    const n = await page.$$eval('.sysmon-row', (els) => els.length);
    if (n > sc.maxRows) issues.push(`row-count: ${n} > ${sc.maxRows}`);
  }
  await page.screenshot({ path: `${OUT}${sc.name}.png`, omitBackground: false });
  const detected = issues.length > 0;
  const pass = sc.expectFail ? detected : !detected;
  if (sc.expectFail && !detected) failures.push(`${sc.name}: negative fixture NOT detected — the assertions are blind to a real defect`);
  if (!sc.expectFail && detected) failures.push(`${sc.name}: ${issues.join(' | ')}`);
  console.log(`${pass ? 'PASS' : 'FAIL'}  ${sc.name}${detected ? '  ' + issues.join(' | ') : ''}${sc.expectFail ? '  (negative fixture)' : ''}`);
  await page.close();
}
await browser.close();
server.close();
console.log(failures.length ? `\n${failures.length} scene(s) failed` : `\nALL PASS (${scenes.length} scenes, source ${sha})`);
process.exit(failures.length ? 1 : 0);
