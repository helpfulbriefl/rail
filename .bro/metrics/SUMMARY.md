# SUMMARY rail-publish · updated 2026-10-13 21:57 · by tools/bro.ps1 summary
| # | indicator | good if | v3.0 |
|---|---|---|---|
| 0 | waves (verdict needs >=10) | >=10 | 1 |
| 1 | accepted first time: PASS / waves | grows | 0/1 (0%) |
| 2 | false done, sum | 0 | 0 |
| 3 | Owner touches per wave | <=2 + stops | 3 |
| 4 | Owner fixes: sum / per wave | falls | 1 / 1 |
| 5 | minutes per wave: work / wait | falls | 8 / - |
| 6 | ptok per accepted task / GitHub calls per wave | falls / <=5 | 3252 / 12 |
| 7 | prompt-caused defects share / avg K (0-4) | falls / grows | 0 / 3 |
| - | prompt weight START+BOOT, last | stable | 4515 |
| - | avg O (0-4) | grows | 1 |

Top weak rules (P causes): -
Friction tags: #no-access x1, #env x1
Rule: change 1-3 things per version; call a version better only after >=10 waves.
