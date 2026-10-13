# 04-ERRATA — W1-RECON (appended by W1B-BASELINE; original REPORT/INDEX kept immutable)

Scope note: this file **adds** errata for the W1-RECON wave; it does not retroactively
replace or mark-complete any existing evidence row. Source of these corrections:
W1B-BASELINE controlled native fixture + 60 s meter trace (see
`../W1B-BASELINE/proof/native-baseline.md` and `../W1B-BASELINE/proof/meter-trace.md`).

## E-001 — B-004 remains unresolved (no leak/fixed claim)
The W1 recon attributed the footer meters to a "system counter (EMA α=0.25)". Correct
nuance: `tracker.rs` publishes the footer via the Tauri IPC bus
(`emit_to("rail","runtime")`) and applies a **system-level** EMA (CPU α=0.25, RAM
24 MB dead-band). W1B captured a **60 s independent WMI/PerformanceCounter reference**
(`run/meters-ref.csv`), but the Rail-footer series is **IPC-only** and not externally
readable in the release build (no a11y tree, no telemetry file). A paired comparison
therefore could not be completed this wave → **B-004 stays open**; no "not a leak /
fixed" conclusion is drawn (TASK §3.4). The "system counter, not a Rail leak" reading
is retained as the best-evidence hypothesis only.

## E-002 — source/exe age remains a hypothesis (Q-009 / Q-SRC)
W1B produced a **traceable source-built exe** (SHA-256 `43a13486…`, 4 404 224 B) from
`tested_source_commit = f7f740d…` (pulled head) and smoke-tested a **copy**
(`run/rail-test.exe`, equality verified). This does **not** prove the *delivered*
`Rail.exe` age or its P↔mapping — that stays a hypothesis (three distinct binaries in P,
none with build provenance). No retroactive closure of Q-009/Q-SRC.

## E-003 — "+" previously reported on an INVALID target (B-003)
W1's "<+> ×3 (no dialog on stale binary)" was run against the delivered binary and,
per the native stand, was an **invalid target** (stale exe + a11y/region unavailable).
W1B re-ran 4 automated approaches (UIA-DOM, UIA-rect, Win32-rect, Win32-DPI) against
the **source-built** exe: the release WebView2 exposes **no accessibility tree** and the
auto-hidden capsule region is `empty_hit_rect`, so synthetic clicks are no-ops
(click-through). B-003 is recorded as **unavailable (automated)**, not a verified
button FAIL, with a compact operator checklist (native-baseline.md §2).

## E-004 — edge reveal not previously proven
The hidden→edge-reveal path was **not confirmed** in W1 (stable screen geometry does not
imply the card was tested). W1B moved a **real cursor** to the right edge and held
1.2 s; no `REGION set` was observed (diag log absent in that launch), so edge-reveal
remains **unproven** — operator checklist in meter-trace.md §4. The `drag_until` 2500 ms
keep-open band for OLE drops is the documented mechanism.

## E-005 — scope / account claim pending evidence (Q-005)
The public-snapshot CI scope limitation (`.github/workflows/build.yml` untracked /
local-only) and the reported gh account switching have **no verifiable evidence** in
this wave. Recorded as pending; the untracked `.github/` is **not** in this wave's allow
list and is left untracked (documented in the W1B report, not committed). No credential/
account change was made or inferred.

## E-006 — P/diag writes & prior invariance remain unknown (Z-03)
Prior scope of any P-side diag writes / auth identity is **not** established by W1B;
recorded as unknown. W1B operated only on test-owned files under `run/` and the copied
exe; P (read-only tree, not a git repo) was not modified.
