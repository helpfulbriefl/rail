# 03 · truth map

Every requirement source, with date, what it governs, and conflicts.

| source (path) | date | governs | conflicts with |
|---|---|---|---|
| `P\Rail-source\docs\SPEC.md` | 2026-10 (v1.0, §1–§9) | product spec: stack, architecture, state machine, metrics smoothing §9.10, drop §9.1, «+» §9.2 | SPEC §9.10 says EMA α=0.35; `tracker.rs:102` is actually `0.25`. SPEC §9.2 marks «+» *closed*; Owner reports it broken (B-003). SPEC §9.1 marks drop *closed*; Owner reports it broken (B-001). |
| `P\Rail-source\README.md` | 2026-10 | dev entry page, user behavior table, build steps | README "hold 5 s → card" and "mouse away → hide after 2 s"; config ships `hoverDelayMs=900`/`hideDelayMs=1200`; `config.rs:24` caps hover at **3 s** (`HOVER_MAX_MS`) — the 5 s README figure is stale. |
| `P\rail.config.json` (root) | 2026-10-06 13:06 | runtime defaults for the delivered exe | `pinned:false`, `items:[]` — no fixtures, so reorder/hover-card can't run (Q-008). |
| `.bro/exchange/W1-RECON/01-TASK.md` | 2026-10-13 (an_turn 2) | wave scope, 8 pass criteria, D-001..006, Z B-001..007, §3 stand, §4 gates | Overrides generic `git add -A` / push-all-tags / mirror-cleanup / stand-package-install defaults. |
| `.bro/exchange/W1-RECON/00-ANALYST-NOTES.md` | 2026-10-13 | spot-verified repo facts, prompt-snapshot HEAD `e35e7e2…`, limitations | Analyst's 404 on `Apex4Coder/global-promt-system` does **not** prove absence; the authoritative private repo is `helpfulbriefl/global-promt-system`. |
| `P\Rail-source\package.json` / `Cargo.toml` | 2026-10 | dependency + stack truth | `package-lock.json` / `Cargo.lock` pin exact versions (see 04_stack). |

**Governance rule for this wave:** P source tree is authoritative over the supplied ZIP. SPEC/README "closed"/"fixed" claims are *annotated*, not trusted (B-007). The delivered `Rail.exe` is a stale artifact and is evidence, not the source of truth.
