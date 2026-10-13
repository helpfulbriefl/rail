# PROOF W1C-NATIVE
Raw run JSON/CSV/scripts under run/ are test-owned, working-tree only (ignored).
Sanitized paired samples + structured records below are the published proof.
Only real, individually-named files are backticked; raw/descriptive tokens are plain.
| # | requirement | proof file(s) | ok |
|---|---|---|---|
| 1 | §3.1 live owned fixture: PID/window, monitor/work-area/DPI/client→screen/region, visible non-empty DOM | `run/t1-facts.json` (pid 70288, Rail top-level 48×294 @(2512,369), monitor 2560×1080/work 2560×1032/DPI 96/dpr 1.0); `logs/provenance.log` (BOM root-cause, exe hash) | ✅ |
| 2 | §3.2 exact QA fix + true npm-run-qa exit/duration, mock visual/storyboard status, generated storyboard names/dims | `qa/storyboard.mjs`, `qa/visual.mjs` (§4 verbatim edits); `run/exits.txt` (typecheck 0/3s, lint:css 0/1.6s, qa 0/80s); raw `run/qa.log`, `run/typecheck.log`, `run/lint-css.log` in working tree | ✅ |
| 3 | §3.3 '+' physical click+cancel, 2-item reorder/restart/hold, OLE shell drag, open/reveal/cancel ×3 | `proof/native-t3.md` (sanitized); raw `run/t3-plus-trial-1.json`, `run/cdp-plus-reveal.mjs`, `run/t3-reorder-t1.json`, `run/t3-reorder-restart-47828.json`, `run/t3-hold-hover-t5.json`, `run/t3-ole-limitation.md` in working tree | partial (4/5) — OLE real-shell drag not executable by automation (exact limitation + manual checklist, not a hidden blocker) |
| 4 | §3.4 60 s paired rail-foot footer + independent system reference, same host clock, sanitized finite units | `proof/meter-t4.md` (sanitized paired samples, units, timing); raw `run/t4-foot-series.json` (30 finite samples CPU 27–72% / RAM 89–98%), `run/t4-ref-meters.json` (WMI ref RAM 94.7–98.9%, host clock) in working tree | ✅ |
| 5 | §3.5 published parser-correct INDEX, redacted gate/provenance/sample proof, honest REPORT/README, delivery verified | `proof/INDEX.md` (this file); `02-REPORT.md`; `../W1B-BASELINE/04-ERRATA.md`; `README.md`; `logs/provenance.log` | ✅ |

Wave verdict: PARTIAL — criteria 1,2,4,5 fully proved; criterion 3 is partial (4 of 5 legs),
its OLE leg being an exact documented automation limitation (not a product FAIL, not
"impossible in principle", with a compact operator checklist). No product code changed;
B-001..B-005 stay open.
