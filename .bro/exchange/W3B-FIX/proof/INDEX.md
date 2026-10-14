# PROOF W3B-FIX
| # | requirement | proof file(s) | ok |
|---|---|---|---|
| 1 | T1 interactive sys popup (click-through only for non-sys), flyout clamped in size AND work area, footer hover/click opener anchored to footer, Escape/outside dismissal | `logs/cargo-test.log` · `flyout-sys-aurora.png` · `src/flyout/Flyout.tsx:67` · `src/rail/Rail.tsx:163` — native HWND click/DPI: PARTIAL (manual checklist in 02-REPORT) | ✅ |
| 2 | T2 ranking over the complete eligible snapshot, separate CPU/RAM answers, deterministic ties, full-path identity, CPU-heavy low-RAM app outside RAM top-5 | `logs/cargo-test.log` · `flyout-sys-aurora.png` · `src-tauri/src/tracker.rs:531` | ✅ |
| 3 | T3 sample ts/seq continuity, interim frames carry sys_top forward, stale marked honestly, both metrics per row + system charts, history appends only on fresh samples | `flyout-sys-stale-glass.png` · `flyout-sys-empty-porcelain.png` · `src/ui/useApp.ts:36` | ✅ |
| 4 | T4 close bound to snapshot id + PID/start-time + window ownership, revalidated, identity-based protection, frozen rows during interaction, structured dispatched/refused/unavailable | `flyout-sys-refused-studio.png` · `src-tauri/src/main.rs:569` · `logs/cargo-test.log` | ✅ |
| 5 | T5 clipping fixed in real geometry (negative fixture proves assertions bite), bounded dock with reachable footer/settings, crowded dock, README updated, W3 errata appended | `negative-broken-layout.png` · `rail-crowd-studio.png` · `settings-studio.png` · `logs/qa.log` · `README.md` | ✅ |
| 6 | T6 real gates with recorded outputs, rebuilt EXE from tested commit, native launch identifies actual PID/path, SHA-256/size recorded | `logs/qa.log` · `logs/cargo-test.log` · `logs/cargo-build-release.log` · `logs/native-single-instance.log` | ✅ |
| 7 | Addendum 1: durable config (atomic replace, serialized writer, flush on every exit route) + visible save-error state | `logs/cargo-test.log` · `rail-savefail-aurora.png` | ✅ |
