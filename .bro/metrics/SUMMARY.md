# SUMMARY rail-publish · updated 2026-10-13 23:48 · by tools/bro.ps1 summary
| # | indicator | good if | v3.0 |
|---|---|---|---|
| 0 | waves (verdict needs >=10) | >=10 | 2 |
| 1 | accepted first time: PASS / waves | grows | 0/2 (0%) |
| 2 | false done, sum | 0 | 0 |
| 3 | Owner touches per wave | <=2 + stops | 2.5 |
| 4 | Owner fixes: sum / per wave | falls | 1 / 0.5 |
| 5 | minutes per wave: work / wait | falls | 39 / 47 |
| 6 | ptok per accepted task / GitHub calls per wave | falls / <=5 | 3156 / 8.5 |
| 7 | prompt-caused defects share / avg K (0-4) | falls / grows | 0 / 3.5 |
| - | prompt weight START+BOOT, last | stable | 4515 |
| - | avg O (0-4) | grows | 1 |

Top weak rules (P causes): -
Friction tags: #env x2, #no-access x2
Rule: change 1-3 things per version; call a version better only after >=10 waves.
