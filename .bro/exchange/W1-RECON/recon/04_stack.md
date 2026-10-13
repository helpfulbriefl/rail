# 04 · stack

## Languages & runtimes (from manifests + installed toolchain)
| layer | value | source |
|---|---|---|
| Frontend UI | React 19.3 + TypeScript 5.9.3 | `package.json` |
| Animation | Motion 14 (`motion/react`) | `package.json` (ex-Framer Motion) |
| Bundle/build | Vite 8.3.2, `@vitejs/plugin-react` 6.1.1 | `package.json` |
| Icons | `lucide-react` 1.51 | `package.json` |
| Fonts | `@fontsource-variable/geist[-mono]` 5.3 | `package.json` |
| Visual QA | `playwright-core` 1.63 + `sharp` 0.35.5 | `package.json` dev |
| CSS lint | stylelint 17.16 + stylelint-config-standard 40 | `package.json` dev |
| Native shell | Tauri 2.12 (`@tauri-apps/cli` 2.12.1) | `package.json` + `Cargo.toml` |
| Native lang | Rust, edition 2021, `rail` crate | `Cargo.toml` |
| Win32 API | `windows` crate 0.62 (Win32_Foundation/UI_*/System_*/Graphics_*/Storage_*) | `Cargo.toml` |
| Process metrics | `sysinfo` 0.39 (feature `system`) | `Cargo.toml` |
| Native plugins | `tauri-plugin-{autostart,single-instance,dialog}` 2 | `Cargo.toml` |
| WebView2 | required at runtime (bundled with Windows 10/11) | `README.md` |

## Installed toolchain on this machine (recorded, not installed this wave)
- Node `v24.16.0` · `npm` · Rust `cargo 1.98.1` · `git 2.54.0`
- PowerShell `5.1.26100.9444` · Windows 11 Pro 26300 / 26H2
- `Get-Counter` perf counters unavailable on this PS 5.1 → WMI `Win32_PerfFormattedData_PerfOS_Processor` / `Win32_OperatingSystem` used instead.

## CI / build pipeline
- `.github/workflows/build.yml` (present in P, **excluded from the public Q snapshot** this wave — see `Q-005`): `windows-latest`, cargo build, push → exe artifact, tag `v*` → release.
- The public snapshot ships source + `qa/` (visual + storyboard), no workflow file.

## Lockfiles
- `package-lock.json` and `src-tauri/Cargo.lock` are committed in Q and hash-identical to P — they are the authoritative version pin. No dependency upgrades in this wave (D-001).
