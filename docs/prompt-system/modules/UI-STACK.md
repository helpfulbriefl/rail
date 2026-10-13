# MODULE UI-STACK · Bro-26 default for projects with UI (analyst decides, coder executes)
Versions only verified at W1 from lock files and sites, never from memory (R6). Alternatives per R14.

## 1 Stack
| layer | pick | why | link |
|---|---|---|---|
| language | TypeScript everywhere | weak coder, one language (G-017) | — |
| base | React + Vite + Tailwind (sample: Brainwave) | mass stack, coder knows it | https://github.com/adrianhajdin/brainwave |
| desktop (if needed) | Electron (TS), start via `dev:safe` | one language; G-016 | — |
| effects | React Bits, copy-paste or CLI (shadcn/jsrepo), no monolith package | live backgrounds, text, cards | https://github.com/DavidHDev/react-bits |
| primitives | shadcn/ui on Radix: Accordion, Collapsible, Dialog | a11y + keyboard out of the box | https://ui.shadcn.com/docs/components/accordion |
| 3D / hero | ThreeUI ready Three.js components (has MCP) | wow sections without own WebGL | https://threeui.com/browse |
| motion | Kinetics (spring, copy CSS/React) for micro-UI · anime.js v4 (`import { animate } from 'animejs'`) for timelines | live UI without hand easing | https://kinetics.colorion.co/ · https://animejs.com/ |
| infographics | AntV Infographic (declarative, MIT) | schemes, metrics, reports | https://github.com/antvis/Infographic |
| icons | developer-icons (tech SVG) + one UI icon set chosen by analyst | no home-made SVG | https://github.com/xandemon/developer-icons |
| app icon | appicon-forge | favicon/icon in 1 step | https://github.com/zhangyu1818/appicon-forge |
| taste | taste-skill rules against template slop → copy to `.bro/design/taste.md`, TASK §0 lists it (never `.roo/`, G-027) | main lever against visual fixes | https://github.com/Leonxlnx/taste-skill · https://www.tasteskill.dev/ |
| look reference | gpui-component / Zed: dense panels, even grids, «tool, not website» | look only, no Rust | https://github.com/longbridge/gpui-component |

## 2 Style «Bro-26 Light» (light theme = main, P-01; dark only on request, same tokens)
| token | start value (analyst refines in W2) |
|---|---|
| background | warm off-white `#F7F7F5`; cards `#FFFFFF` |
| text | `#16181D` / secondary `#5B6170` (contrast ≥4.5:1) |
| borders | 1px `#E6E7EB`, no heavy outlines |
| shadows | soft layered low density: `0 1px 2px` + `0 8px 24px` at 4–6 % black |
| accent | one bright, e.g. indigo `#4F46E5` · status 🟢 `#16A34A` 🟡 `#D97706` 🔴 `#DC2626` ⚪ `#9CA3AF` |
| radius | cards 12–16 px, buttons/fields 8 px |
| font | Inter / Geist (variable), metric digits tabular-nums |
Rules:
- Light bento: cards of different size on off-white, one card = one job, air instead of lines.
- List >5 rows → accordion group: title = summary (count + traffic light), click to open, spring height ≤250 ms, open state remembered, «expand all / collapse all» on top, keyboard Enter/Space, `aria-expanded`. Plain list only if ≤5 rows or opening hurts (nav, menu).
- Big metric numbers, minimal text, no junk labels. Micro buttons; badges 🟢 ok · 🟡 attention · 🔴 error · ⚪ loading/no data.
- Glass only light (`white/70` + blur), only floating panels and tooltips.
- Motion: spring, response ≤200 ms, stagger on enter, respect `prefers-reduced-motion`.
- One wow element per screen (light 3D hero / soft live background). No dark-only effects (neon, glow).
- Icons, fonts, effects only from §1 or found by recon (R14).

## 3 Mockups (W2-DESIGN and every new screen)
- Analyst makes 1 mockup per screen for all screens of the wave, no questions: source HTML + Tailwind on Bro-26 tokens, inside TASK §4.
- Coder saves `.bro/design/Mxx.html`, renders `Mxx.png` with Playwright at 1440×900, light theme, accordions both open and closed; commits.
- S2: Owner answers one line `M01 ✅ · M02 ✅ · M03 правка: …`; fixes = one repeat design wave, not one by one.
- Approved = anchor: LOG `D-xx: M03 approved`; UI is checked against it (STAND diff %).

## 4 gates — one command, PASS/FAIL table
lint + types + tests + build · style lint, colors only via tokens · Playwright geometry 1280/1440/1920: clipping, overflow, overlap · ARIA snapshots https://playwright.dev/docs/aria-snapshots · screenshot compare https://playwright.dev/docs/api/class-snapshotassertions · axe https://playwright.dev/docs/accessibility-testing · secret scan · `ELECTRON_RUN_AS_NODE` check (Electron). Accept only on real build, not mock data (G-010).
