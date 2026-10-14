# W3-MICRO — analyst code acceptance

Verdict: CHANGES REQUIRED / functional FAIL. Delivery exists, but W3 is not DONE as a functioning product. Keep original 02-DELIVERY immutable; append corrections in the next wave.

Independently loaded report and source from remote main 439702e43f29736ed26d18a503a2a0a3fe875ef6. EXE SHA a88149c696bd8456978694f461071343d47b7d03d2f713a2ae7c32b362796105 and gates are coder-reported; analyst did not execute Windows binary or independently hash it. Published source/report confirmed, native readiness not confirmed.

Blocking source findings:
1 src-tauri/src/main.rs show_flyout unconditionally calls win::set_click_through(hwnd_of(&f), true), including kind sys. Native popup cannot be accepted as interactive while intentionally configured click-through. Flyout.tsx also has no popup-specific Escape/outside/hover lifecycle.
2 tracker.rs sorts all groups by RAM then takes five before publishing. Flyout.tsx CPU tab only reorders those RAM-selected five. It is not system CPU top-five.
3 scan initializes sys_top empty and fills it only when metrics=true. Carry-forward restores sys/items but not sys_top; interim emitted frames erase the list. No sample timestamp/stale contract. Empty list is called stale in UI but actual stale samples cannot be detected.
4 closable is true for any exe-ending path, even with no window. sys_close accepts arbitrary executable string, matches freshly enumerated path but has no snapshot-bound PID/start-time identity, no shell exclusion, no structured result. Name-substring filters are not a protected-process policy. UI only freezes closingId, not row layout/targets; .finally clears pending immediately after request, not actual app outcome. Report claim frozen target identity unsupported.
5 Rail.tsx footer opens on click/Enter/Space only, no hover opener, and anchors to capRef rather than footer. SysMonitor shows only selected metric percent, no per-row memory units/both metrics, no system history graph. User's functional hover mini-monitor requirement is incomplete.

Repair/proof gap: no native evidence for clipping, stable hover, reachable actions, working OLE, crowding, DPI, close/unsaved dialogs. overflow:auto alone is not proof of bounded scrolling. 02-DELIVERY says labelled close/hide/exit in tray; inspected build_tray_menu has show/settings/quit but no explicit hide or app-close item (app-close is in item_menu). Correct location wording. Do not invent proofs or claim every existing issue is unchanged; actual native outcomes require tests.

Retain approved themes; no new design wave. Next W3B-FIX closes these specific defects and completes the original repair/native acceptance, through the accepted GCM route.
