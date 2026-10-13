# MAP · Rail · updated 2026-10-13 by W1-RECON @ef1a3c1

## §1 Project
Rail — portable Windows tray "dock" (Tauri 2 + React, ~4 MB exe). Icons → live process blocks with a traffic-light, window counter and mini task-manager. **Status: active development, not release-ready** (see README + recon 09). Stack is fixed by D-001 (no Electron / no framework swap).

<details open><summary>§2 Anchors · PC ↔ git ↔ mirror</summary>

| what | PC | git | mirror |
|---|---|---|---|
| source P (authoritative) | `C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail` | — (raw tree, no VCS) | — |
| publication Q | `C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish` | `github.com/helpfulbriefl/rail` @ `main` `ef1a3c1` (tag `w1-snapshot`) | **proposed, not created:** `%LOCALAPPDATA%\BRO-MIRROR\rail` (empty dedicated path; D-006, needs Owner approval — do not mirror this wave) |
| system SYS | `C:\Users\BazaCode\AppData\Local\Temp\gpx\global-promt-system` (extracted from `P\Rail-source\global-promt-system.zip`, HEAD `e35e7e2…`) | `github.com/helpfulbriefl/global-promt-system` (private) | — |

</details>

<details open><summary>§3 Stack · versions from lock</summary>
React 19.3 · TS 5.9.3 · Motion 14 · Vite 8.3.2 · Tauri 2.12 · windows 0.62 · sysinfo 0.39 · Playwright 1.63. Pinned by `package-lock.json` + `src-tauri/Cargo.lock` (committed, hash-identical to P). See recon 04.
</details>

<details open><summary>§4 Invariants · don't touch · secrets (paths only) · ports · Legacy</summary>
- **No product source changes this wave** (D-002). No dependency upgrade (D-001).
- P source tree is **read-only** (D-004); never modify P.
- Secret-shaped / per-machine paths, all `.gitignore`d: `rail.config.json`, `*.log`, `.env*`, `*.pem`, `*.key`, `**/.auth/`, `qa/shots/`, bro proof media.
- Private SYS history stays private (D-003): only redacted `docs/prompt-system/**` is public.
- No OLE-relevant native constants to touch; `DRAG_LEASE_MS` (`main.rs`) and `RAIL_DIAG` (`diag.rs`) are existing knobs.
- Legacy: `P\Rail-source.zip` (frontend-only, B-006) and `P\Rail.exe.bak-old-0504` are superseded, delete-only candidates (recon 07).
</details>

<details open><summary>§5 Decisions · active D only</summary>
- **D-001** JOIN existing Tauri/Rust+React stack; no Electron, no dep upgrades, no new UI framework.
- **D-002** No product/QA/manifest/native-config/executable changes; no bug fixes in recon.
- **D-003** Public source; private prompt repo stays private; clean snapshot in `docs/prompt-system` (reviewed + redacted-scan'd).
- **D-004** Work in fresh clone Q; P untouched; if Q exists/reparse → STOP.
- **D-005** No raw videos / private metrics / local creds / diagnostic dumps / private git history. License: record Q-LICENSE, propose MIT, don't invent owner.
- **D-006** No mirror deletion; `mirror: -`; no /MIR, no retention-bundle deletion, no --all bundles. Propose empty dedicated local mirror path for later approval.
- **D-007** Reuse existing verified Q (D-004 'Q must not exist' was first-clone-only). P stays read-only. W1B-BASELINE before W2-DESIGN.
- **D-008** Real Windows native stand required. Wrong-target clicks, missing item prerequisites and two unpaired WMI samples cannot close B-001..B-005. Old-exe history remains unproven.
- **D-009** Permit locked project-local dependency fetch/build and existing gates under TASK limits. No versions/locks/product/native-config changes, release, installers or elevation.
- **D-010** No gh account switch, global auth/config change, new token, broadened token scope, force/reset, --all history export, or destructive mirror. Auth failure reported, not bypassed.
- **D-011** Execute unchanged BRO helper from an ignored project-local generic-system copy (`Q\.bro\tooling\bro-v3`), so automatic metrics cannot write to original SYS or public docs/prompt-system. No original system edits/pushes.
</details>

<details open><summary>§6 Owner prefs · P</summary>
- P-01 Owner works in RU; agent reports in EN (per analyst notes).
- P-02 Proof-based acceptance; small waves; mockup approval before UI waves.
- P-03 Existing technical stack is preserved until a separately-approved architecture decision.
- P-04 (draft, `.bro`-only, not published) profile/site wording: label Rail as **in-development** on `helpfulbriefl` README + site until a verified release exists; do not list as released. No profile/site files changed this wave.
</details>

<details open><summary>§7 Rakes · project G</summary>
- G-01 `git add -A` forbidden (TASK override) — stage only reviewed paths.
- G-02 No pushing all tags; only `w1-snapshot` + the REPORT commit.
- G-03 No installing stand packages / blanket Node+Electron install; blocked gates stay blocked.
- G-04 No destructive `bro.ps1` mirror/retention (`/MIR`, bundle-delete) — D-006.
- G-05 `bro finish OK` ≠ application PASS; the native defects are still open.
- G-06 WMI reference != Rail sysinfo stream. Capture paired observations before assigning a cause.
- G-07 Click actual '+' button, not native window center. Hidden->visible transition must start hidden.
- G-08 Report/push/helper OK is metadata, not native functional PASS. Source commit, exe hash and final REPORT commit are separate identifiers.
</details>

<details open><summary>§8 Mockups · M → screen → file</summary>
None this wave (recon/publication). Mockups enter at W2-DESIGN per the roadmap.
</details>

<details open><summary>§9 Top files · path · role · imported by N</summary>
(recon 02 repomap; N = verified import counts)
| path | role | imported by |
|---|---|---|
| `src/main.tsx` | root render (mounts `Rail`) | entry (11 imports) |
| `src/rail/Rail.tsx` | capsule/dock/drop/hover/«+» | 10 |
| `src/flyout/Flyout.tsx` | info + metrics card | 10 |
| `src/rail/Tile.tsx` | single tile + reorder | 8 |
| `src/settings/Settings.tsx` | settings window | 7 |
| `src/bridge/api.ts` | single UI↔OS IPC seam | n/a (seam) |
| `src-tauri/src/main.rs` | commands/tray/windows/pick_files | 14 use/mod |
| `src-tauri/src/win.rs` | Win32 (EnumWindows/SetWindowRgn/OLE) | 13 |
| `src-tauri/src/tracker.rs` | cursor/edge/scan/metrics loop | 7 |
</details>

## §10 Roadmap
`W0-BRIEF=done!:Бриф; W1-RECON=done:Разведка PARTIAL; W1B-BASELINE=run:Сборка и реальный стенд; W2-DESIGN=todo!:Макеты; W3b-CORE=todo:Логика; W3c-UI-RAIL=todo:Интерфейс; W4-FIX=todo:Регрессия; W5-POLISH=todo!:Релиз` · next: **W1B-BASELINE**. B-001..B-005 remain open. Full functional acceptance requires native evidence; publication status alone is insufficient.
