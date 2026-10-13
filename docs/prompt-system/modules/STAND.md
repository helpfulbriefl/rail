# MODULE STAND · human-like stand + proof (coder)
No proof → ❌ (R17). Proof only from REAL build + real/test data, never mocks (G-010). Folder PR = `.bro/exchange/W/proof/` (LOCAL: steps).
`PR/INDEX.md` (bro begin creates header): `| # | requirement | proof file(s) | ok |` · files in backticks, relative to PR or project root · ok ✅/❌. `bro finish` checks every ✅ file exists.
| project | «human» | proof files |
|---|---|---|
| web | Playwright runs TASK §3 scenario on real build: open → see → click/type → wait → get | `screens/<w>-NN-step.png` at 1280/1440/1920 · `trace.zip` · `aria/<w>-<screen>.yml` · `diff/<Mxx>.txt` · `axe-<w>.json` |
| Electron | Playwright `_electron.launch({ args: ['.'], env })`, `env` without ELECTRON_RUN_AS_NODE; same scenario | same |
| CLI / script | VHS types commands like a human; no VHS → `cmd /c` logs | `term.png`/`.gif` · `input` / `expected` / `actual` + `diff.txt` · `run.log` |
| any | newbie test: `git clone P %TEMP%\bro-fresh-W` (LOCAL: copy without node_modules), follow README literally | `newbie.log`: step → command → exit code |
Setup once (web/Electron), TASK allow has `tests/stand/**`: `npm i -D @playwright/test @axe-core/playwright pixelmatch pngjs` · `npx playwright install chromium` · optional `npx playwright init-agents --loop=vscode` (planner/generator/healer help write and heal tests). Config: `use: { baseURL: 'http://127.0.0.1:<port>', trace: 'on' }`; copy `test-results/**/trace.zip` → `PR/trace.zip`.

`tests/stand/<W>.spec.ts` skeleton (steps = TASK §3 words):
```ts
import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import fs from 'node:fs';
const PR = process.env.BRO_PROOF ?? '.bro/exchange/W/proof';
for (const w of [1280, 1440, 1920]) test(`stand ${w}`, async ({ page }) => {
  for (const d of ['screens', 'aria']) fs.mkdirSync(`${PR}/${d}`, { recursive: true });
  await page.setViewportSize({ width: w, height: 900 });
  await page.goto('/');                                                    // open
  await expect(page.getByRole('heading', { name: '<title>' })).toBeVisible(); // see
  await page.screenshot({ path: `${PR}/screens/${w}-01-open.png`, fullPage: true });
  // click / type / wait / get … one screenshot per step
  fs.writeFileSync(`${PR}/aria/${w}-main.yml`, await page.locator('body').ariaSnapshot());
  expect(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth)).toBe(false);
  const axe = await new AxeBuilder({ page }).analyze();
  fs.writeFileSync(`${PR}/axe-${w}.json`, JSON.stringify(axe.violations, null, 1));
  expect(axe.violations).toEqual([]);
});
```
`tests/stand/bro-diff.mjs` — % difference vs mockup (render both at 1440×900): `node tests/stand/bro-diff.mjs shot.png .bro/design/M03.png PR/diff/M03.txt`
```js
import fs from 'node:fs'; import { PNG } from 'pngjs'; import pixelmatch from 'pixelmatch';
const [a, b, out] = process.argv.slice(2);
const A = PNG.sync.read(fs.readFileSync(a)), B = PNG.sync.read(fs.readFileSync(b));
const w = Math.min(A.width, B.width), h = Math.min(A.height, B.height);
const crop = (s) => { const o = new PNG({ width: w, height: h }); PNG.bitblt(s, o, 0, 0, w, h, 0, 0); return o; };
const d = new PNG({ width: w, height: h });
const n = pixelmatch(crop(A).data, crop(B).data, d.data, w, h, { threshold: 0.1 });
fs.mkdirSync(out.replace(/[\\/][^\\/]+$/, ''), { recursive: true });
fs.writeFileSync(out.replace(/\.txt$/, '.png'), PNG.sync.write(d));
fs.writeFileSync(out, `diff ${(100 * n / (w * h)).toFixed(2)} % · ${w}x${h} · ${a} vs ${b}\n`);
```
VHS tape (`tests/stand/<W>.tape`, run `vhs <file>`; needs ttyd + ffmpeg):
```
Output .bro/exchange/W/proof/term.gif
Set Shell "powershell"
Type "<command from README>"
Enter
Sleep 3s
Screenshot .bro/exchange/W/proof/term.png
```
Analyst reads text, not pictures: keep aria `.yml` and `diff/*.txt` small and named in INDEX.
