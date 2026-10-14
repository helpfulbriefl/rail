# W3-MICRO — Coder Delivery

## Status: DONE

### Increment A — Repair + Approved Themes
- **Glass** (default) and **Studio** themes added to tokens.css, Settings.tsx, tray menu
- Theme sanitize in config.rs accepts all 6 values, falls back to `glass`
- Themes grid in Settings: 4→3 columns
- Dock: `overflow: auto` with thin scrollbar for crowded lists
- Meter footer: `data-interactive`, `role="button"`, `tabIndex`, hover styles
- Close/hide/exit actions separate and labelled in tray menu

### Increment B — Working Mini-Dispatcher
- **Rust**: `SysApp` struct in tracker.rs, `sys_top` in RuntimePayload, top-5 aggregation by exe path (lowercase, multi-process summed), system/critical processes excluded
- **Rust**: `sys_close` command in main.rs — WM_CLOSE to validated windows, skips Rail's own HWDs and system32/drvfail/svchost
- **TS**: `SysApp` interface, `sysTop` in RuntimePayload, `'sys'` in FlyoutCmd.kind, `sysClose()` in Api
- **UI**: `SysMonitor` component in Flyout.tsx — CPU/RAM tabs (RAM default), 5-row display, letter icon, close X button (disabled when `!closable`), frozen target identity, "Нет данных" / "Закрытие…" states
- **Rail**: meter footer click/keyboard → `showFlyout({ kind: 'sys', ... })` 300×280 px

### Build Gates
| Gate | Result |
|---|---|
| `npm run typecheck` | PASS |
| `npm run lint:css` | PASS |
| `npm run build` (tsc + vite) | PASS |
| `cargo build --release --locked` | PASS |
| `npx tauri build --no-bundle` | PASS |
| Visual QA (`qa/visual.mjs`) | PARTIAL (requires running browser + app) |

### Commits
- `990c1d0` W3-A: glass/studio themes + default theme glass + theme sanitize + settings grid 3-col
- `7d0833f` W3-B: system top-5 mini-dispatcher (CPU/RAM tabs, safe WM_CLOSE) + meter footer interaction + sysmon UI

### EXE Delivery
- **Path**: `C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail.exe`
- **SHA-256**: `a88149c696bd8456978694f461071343d47b7d03d2f713a2ae7c32b362796105`
- **Size**: 4,420,096 bytes (~4.2 MB)
- **tested_source_commit**: `7d0833f1dd7e5381f1ecfb1bc12f9b8bb9e211c1`
- **Build command**: `npx tauri build --no-bundle` (from rail-publish root)

### Push
- Remote: `https://github.com/helpfulbriefl/rail.git`
- Branch: `main`
- Range: `0a39879..7d0833f`
- GCM: `GIT_TERMINAL_PROMPT=0`, `GCM_INTERACTIVE=Never`, credential.manager, username `helpfulbriefl`

### Credit
**Tester & Developer**: [@yumi_acess](https://helpfulbriefl.github.io/)
