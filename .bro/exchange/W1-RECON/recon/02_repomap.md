# 02 · repomap

Tree of Q root (= product tree, 48 files, no build artifacts):

```
.
├── .github/workflows/build.yml        ← CI gate (excluded from public push — Q-005)
├── .gitignore                         ← Q-adapted
├── .gitattributes                     ← Q-adapted
├── .stylelintrc.json
├── index.html                         ← Vite entry
├── package.json                       ← React + Tauri v2
├── package-lock.json                  ← hash-identical to P
├── README.md                          ← dev status page
├── tsconfig.json
├── vite.config.ts
├── assets/logo.svg
├── public/logo.png
├── docs/SPEC.md                       ← full product spec
├── docs/prompt-system/                ← BRO v3.0 snapshot (this wave)
├── qa/
│   ├── storyboard.mjs
│   └── visual.mjs
├── src/
│   ├── main.tsx
│   ├── bridge/ (api.ts, mock.ts, types.ts)
│   ├── rail/   (Rail.tsx, Tile.tsx, magnify.ts)
│   ├── flyout/ (Flyout.tsx)
│   ├── settings/ (Settings.tsx)
│   ├── styles/ (app.css, tokens.css)
│   └── ui/     (Lamp, MicroButton, Num, Sparkline, format, iconColor, status, useApp)
└── src-tauri/
    ├── Cargo.toml · Cargo.lock · build.rs
    ├── tauri.conf.json
    ├── capabilities/default.json
    ├── icons/
    └── src/ (main.rs, core.rs, config.rs, tracker.rs, diag.rs, win.rs)
```

**Entry points:**
- `src/main.tsx` → Vite frontend entry
- `src-tauri/src/main.rs` → Tauri native entry
- `qa/visual.mjs` → visual-diff gate
- `qa/storyboard.mjs` → storyboard gate

**Run commands (from `package.json` + `tauri.conf.json`):**
- `npm run dev` → Vite dev server (frontend only)
- `npm run tauri` → `tauri` CLI, launches native app
- `npm run build` → Vite build
- `cargo build` in `src-tauri/` → native build
- `npm run qa:visual` → `node qa/visual.mjs`
- `npm run qa:storyboard` → `node qa/storyboard.mjs`

**Import frequency (verified by `Select-String '^import |^export .* from'`):**

Frontend `src/`:
| file | import-lines | role |
|---|---|---|
| `src/main.tsx` | 11 | app entry; mounts UI, injects bridge |
| `src/rail/Rail.tsx` | 10 | dock container (drop, reorder, '+', footer meters) |
| `src/flyout/Flyout.tsx` | 10 | hover flyout panel |
| `src/rail/Tile.tsx` | 8 | per-tile widget |
| `src/settings/Settings.tsx` | 7 | settings modal |
| `src/ui/useApp.ts` | 3 | shared state hook |

Native `src-tauri/src/` (by `use`/`mod` lines):
| file | use/mod-lines | role |
|---|---|---|
| `main.rs` | 14 | Tauri setup, commands, tray |
| `win.rs` | 13 | Win32 window/region/auto-hide |
| `tracker.rs` | 7 | desktop shortcut scanner |
| `core.rs` | 4 | event-loop plumbing |
| `diag.rs` | 4 | diagnostic event logger |
| `config.rs` | 2 | rail.config.json read/write |
