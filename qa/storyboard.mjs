// Motion storyboard: frames of the key animations (design approval artefact).
import { chromium } from 'playwright-core';
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { existsSync, mkdirSync } from 'node:fs';
import { extname, join } from 'node:path';
import { execSync } from 'node:child_process';
import sharp from 'sharp';

const DIST = new URL('../dist/', import.meta.url).pathname;
const OUT = new URL('./shots/', import.meta.url).pathname;
mkdirSync(OUT, { recursive: true });
const types = { '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.woff2': 'font/woff2', '.png': 'image/png' };
const server = createServer(async (req, res) => {
  const p = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  const f = join(DIST, p === '/' ? 'index.html' : p);
  if (!existsSync(f)) return res.writeHead(404).end();
  res.writeHead(200, { 'content-type': types[extname(f)] ?? 'application/octet-stream' });
  res.end(await readFile(f));
}).listen(4174);
const browser = await chromium.launch({ executablePath: execSync('which chromium').toString().trim() });
const WALL = 'html{background:linear-gradient(135deg,#1e3a8a 0%,#6d28d9 45%,#db2777 75%,#f59e0b 100%) !important}';

async function strip(name, q, size, act, times, clip) {
  const page = await browser.newPage({ viewport: { width: size[0], height: size[1] }, deviceScaleFactor: 2 });
  await page.goto(`http://localhost:4174/?${q}`);
  await page.addStyleTag({ content: WALL });
  const frames = [];
  const t0 = await act(page);
  for (const t of times) {
    const wait = t0 + t - Date.now();
    if (wait > 0) await page.waitForTimeout(wait);
    frames.push(await page.screenshot(clip ? { clip } : {}));
  }
  await page.close();
  const metas = await Promise.all(frames.map((f) => sharp(f).metadata()));
  const W = metas[0].width, H = metas[0].height, gap = 16;
  await sharp({ create: { width: W, height: H * frames.length + gap * (frames.length - 1), channels: 4, background: '#ffffff00' } })
    .composite(frames.map((f, i) => ({ input: f, left: 0, top: i * (H + gap) })))
    .png()
    .toFile(`${OUT}${name}.png`);
  console.log('saved', name);
}

// 1) appear: rail slides in from the edge with blur→sharp and staggered icons
//    (compact rail is ~130 px tall on the bottom edge)
await strip('story-appear', 'w=rail&theme=aurora&side=bottom', [900, 130], async () => Date.now(), [20, 90, 160, 260, 600], { x: 120, y: 0, width: 660, height: 130 });
// 2) launch: idle icon → bounce + amber lamp → morph into running block
await strip('story-launch', 'w=rail&theme=classic-light&side=bottom', [900, 130], async (page) => {
  await page.waitForTimeout(900);
  const el = (await page.$$('.dock-item'))[3];
  const b = await el.boundingBox();
  await page.mouse.click(b.x + b.width / 2, b.y + b.height / 2);
  await page.mouse.move(5, 5);
  return Date.now();
}, [60, 450, 1500, 1700, 2300], { x: 280, y: 20, width: 300, height: 110 });
await browser.close();
server.close();
process.exit(0);
