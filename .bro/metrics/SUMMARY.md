# SUMMARY rail-publish · updated 2026-10-13 21:01 · by tools/bro.ps1 summary
| # | indicator | good if |  |
|---|---|---|
| 0 | waves (verdict needs >=10) | >=10 |  |
| 1 | accepted first time: PASS / waves | grows |  |
| 2 | false done, sum | 0 |  |
| 3 | Owner touches per wave | <=2 + stops |  |
| 4 | Owner fixes: sum / per wave | falls |  |
| 5 | minutes per wave: work / wait | falls |  |
| 6 | ptok per accepted task / GitHub calls per wave | falls / <=5 |  |
| 7 | prompt-caused defects share / avg K (0-4) | falls / grows |  |
| - | prompt weight START+BOOT, last | stable |  |
| - | avg O (0-4) | grows |  |

Top weak rules (P causes): -
Friction tags: -
Rule: change 1-3 things per version; call a version better only after >=10 waves.
