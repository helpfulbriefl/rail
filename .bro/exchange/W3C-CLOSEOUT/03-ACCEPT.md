# W3C-CLOSEOUT — analyst acceptance

Verdict: PARTIAL overall; minimal changes required to UI interaction edge cases. No new broad wave. Independently loaded published report/INDEX and Flyout source from main c23b31cc0274c6068c7aec2a360223adbf590f26. Reported gates QA42/Rust31/release build and EXE 80D7AE0B9D53B0E4C9A81F91EBA95980669D7151404AFAE9C57E4D4D41F49CA5 (4441088 bytes, tested_source_commit 9bb9788) are local coder evidence, not analyst reruns/hash checks. Report distinguishes source main, tested commit and historical tag; preserve all references.

Accepted source changes: timestamp ms contract with ticking stale timer, visible cap five, live selection outside interaction, captured close identity and disabled tombstone row path. Native proof correctly remains PARTIAL in row4. Reported scope restoration is recorded, not independently diffed by this spot check.

Remaining source cases in Flyout.tsx SysMonitor:
- arm accepts empty liveIds. onPointerEnter/onFocus before the first sample freezes [] indefinitely while interaction continues. Cold-open QA without hover does not cover this. Do not freeze empty loading state; render arriving rows without tab switch or mouse exit, then stabilize valid rows for active interaction.
- onPointerLeave/onBlur call release unconditionally. Leave can occur while keyboard focus remains or pointer press is active. Blur bubbles on focus moving between descendants, so internal focus change also unfreezes. Model independent pointer/focus/press ownership; only release when all interaction guards end. Keep capture/backend revalidation.
- apps.length===0 empty branch bypasses frozen tombstones when ALL apps disappear. Render frozen slots first while freeze exists, even with empty latest candidates; disable close and retain slot positions.
- noSample body says waiting but totals below still format initial zero CPU/memory. Render unavailable placeholders until a valid system sample, not fabricated zero readings.

Row2 must remain conditional/PARTIAL until these sequence regression cases pass, not fully proved by static screenshots. Existing no-native-physics row4 stays PARTIAL. Corrected addendum report/index must be new files, original report/index immutable. Do not manufacture a release PASS or rewrite historical tag to point to later source.

Execute 04-MINIMAL-CONTINUATION.md within existing allow; then deliver corrected test binary for one consolidated Owner physical checkoff. No redesign, broad audit, CSP changes or baseline-only follow-up wave.
