# 04-ERRATA — W1B-BASELINE (appended by W1C-NATIVE; W1B REPORT/INDEX kept immutable)

Scope note: this file **adds** errata to the W1B-BASELINE wave only; it does not
retroactively replace or mark-complete any W1B evidence row. Source of these
corrections: the W1C-NATIVE live test-panel stand (see `../W1C-NATIVE/logs/provenance.log`
and `../W1C-NATIVE/proof/INDEX.md`).

## E-001 — proof-format issue: INDEX zero-rows and raw-run exclusion (Z-02)
W1B's `proof/INDEX.md` shipped with an empty body (zero criterion rows) while the
raw gate logs / `run/meters-ref.csv` are working-tree only (excluded per §5.6
"raw logs", "run"). Consequence observed by the helper: **proof_ok counted 0 /
proof_req 6**. W1C corrects the format — the published INDEX now carries one
backticked row per §3 criterion with an honest ✅/❌/— status, and the
authoritative gate/build evidence is the **structured, force-added**
`logs/*.log` records, never the ignored raw dumps. W1B rows stay as-is.

## E-002 — pre-push metadata now delivered
The exe provenance metadata that W1B recorded only in the working tree
(`logs/exe-provenance.log`) is now **committed and referenced** from the W1C
report/INDEX (SHA-256 `43a13486…`, 4 404 224 B, `tested_source_commit f7f740d…`).
The "pre-push metadata was not delivered" gap is closed.

## E-003 — hidden-target '+' trials were on the wrong HWND (Z-01)
W1B's four INVALID '+' attempts targeted the auto-hidden capsule region
(`empty_hit_rect` / `dev.rail.dock-siw` 16×16 placeholder). W1C established that,
with a **pinned** test-owned config, the real visible window is the separate
`0x3D31420 [Rail]` top-level (48×294, owner pid) created/repositioned by
`show_rail`; the 16×16 placeholders are not the hit target. The single real
Windows '+' trial (W1C `run/t3-plus-trial-1.json`) hit that post-reveal window;
a `#32770` dialog owned by the fixture pid was detected and ESC-cancelled. B-003
remains **open** (invoke-path proven via CDP, one physical trial recorded) but
is no longer "all attempts on a hidden region".

## E-004 — region/monitor were unmeasured in W1B; W1C measured them
W1B recorded geometry only as a stable-screen assumption. W1C measured the
actual single monitor (2560×1080, work 2560×1032, DPI 96, dpr 1.0) and the
client→screen mapping (rail origin (2512,369), DPR 1.0, `+` button CSS
(7.5,85) 19×19 → physical (2529,464)). `GetWindowRgn`/`GetRgnBox` on the post-reveal
HWND returned a non-empty region for the visible rail (the 16×16 placeholders are
the "empty" case that must be distinguished, not treated as NULLREGION proof).

## E-005 — actual gate status (Z-03 / D-013)
W1B reported `storyboard.mjs` as a pre-existing Windows `URL.pathname` failure.
W1C applied the exact §4 QA-only exception (`fileURLToPath` + loopback `127.0.0.1`
bind + `which chromium`→`executablePath: exe`) to `qa/storyboard.mjs` and the two
`qa/visual.mjs` host/loopback edits. Result this wave: **`npm run qa` EXIT 0 (80 s)**,
`visual.mjs` 23/23 PASS and the storyboard leg now completes (generated PNGs
present). This is a **mock-visual** pass only — no native acceptance claim.
B-001..B-005 remain open; no product code was touched.

## E-006 — meters: paired live footer captured; CPU ref degraded in VM
W1B captured an independent reference only (Rail footer was IPC-only). W1C
captured the **actual rendered `.rail-foot` CPU/RAM series** via loopback CDP
(30 finite samples, 60 s) paired with an independent WMI reference on the same
host clock. RAM pairing is valid (rail 89–98% vs ref 94.7–98.9%). CPU: the
independent reference reports **0%** in this VM (Win32_Processor/PerfFormattedData
`LoadPercentage` artifact), so CPU pairing is degraded and documented — **not**
inferred as "not a leak". B-004 stays open.
