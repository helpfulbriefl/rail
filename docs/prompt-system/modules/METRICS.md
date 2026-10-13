# MODULE METRICS · passports, rubrics, bro.ps1 (reference for coder, analyst, auditor)
Only agents count, only by this prompt; no external tools. Every sum/average = `bro.ps1` (G-030).
Passport = frontmatter between `---` at file top; segments ` · `; `key: value`; values never contain ` · `.
| file | written by | keys |
|---|---|---|
| 01-TASK | analyst | ver build wave layer t_issued tasks pass_criteria an_turn · allow modules mirror risky road [base] |
| 02-REPORT | coder `you` + `bro finish` `auto` | wave ver t_start t_report commit tag push bundle pack exchange tasks done gates_first repair proof_ok proof_req diff_files diff_add diff_del out_of_scope q contradictions K1–K4 k_why friction_cd cd_in cd_out prompt_weight an_in_prev an_out_prev |
| 03-ACCEPT | analyst | wave t_accept verdict spot false_done defects causes O1–O4 owner_touches owner_fixes gh_calls ctx_pct friction_an read_list |
Times `YYYY-MM-DD HH:mm` local. ptok = cyrillic/3 + other/4 (lower bound; same formula always — comparability beats accuracy).
Auto values: t_start = `bro begin` time · done = task lines with ✅ · q = distinct Q-NN · proof_ok/req from INDEX · diff_* and out_of_scope = `git diff --numstat base..HEAD` without `.bro`, vs TASK allow · cd_in = ptok(CODER-BOOT + NEXT + TASK + prev ACCEPT + named modules + REPORT template) · cd_out = ptok(added lines + REPORT + INDEX) · prompt_weight = ptok(START-<build> + CODER-BOOT) · an_*_prev = WAVES.csv row of prev wave.
Row of wave X is written by `bro begin` of the next wave: an_in = ptok(REPORT(X) + read_list files) · an_out = ptok(ACCEPT(X) + TASK(next) + NEXT) · k = ΣK · o = ΣO → `.bro/metrics/WAVES.csv` + `SYS/metrics/projects/<project>.csv`.
Rubrics 0/1. Coder scores TASK: K1 enough data (paths, inputs, key code) · K2 no conflict with MAP/SPEC · K3 3–6 tasks of one layer · K4 PASS checkable. Analyst scores REPORT: O1 proof complete per TASK list · O2 spots match · O3 only allowed files · O4 unclear → Q-, not invented. Nobody scores self.
causes `P@R12;C@C4;E@-;S@§2` — P prompt rule (→ AUDITOR) · C coder rule · E environment · S spec. false_done = ✅ proven false (worst). Honest ❌ never punished.
Friction tags: #unclear-task #no-access #env #invented #too-big #tool-fail.
SUMMARY.md per version (bro summary P | ALL): 1 PASS share · 2 false_done · 3 Owner touches/wave · 4 Owner fixes · 5 minutes work / wait · 6 ptok per accepted task + GitHub calls/wave · 7 P-share of defects + avg K · prompt weight · top-3 weak rules · friction tags. Version verdict only after ≥10 waves; change 1–3 things per version.
Commands: `init P W [NEW|JOIN|LOCAL] [prevW]` · `begin P` · `finish P` · `check P [W]` · `ptok <paths>` · `added P [base]` · `row P prevW [curW]` · `summary P|ALL` · `state P [coder|analyst|owner]` · `build`.
