# Owner approval — MICRO light designs and mini system monitor

Status: DESIGN APPROVED / IMPLEMENTATION QUEUED. This is an analyst decision record, not a BRO GO instruction or a functional PASS. S3-AUTH Owner hold in .bro/inbox/NEXT.md remains unchanged. No product, config, credential or permission changes are authorized by this record alone.

## Owner decision
The Owner approved all five light MICRO mockups: Classic Windows, Porcelain, Aurora, Glass and Studio. Glass is the requested default; Classic Windows must remain available. Keep existing themes and saved user preferences; do not silently overwrite an existing explicit theme selection. Default Glass applies to new/default configuration. A dark redesign is not newly approved by this decision.

Approved reference: attached Rail-5-светлых-микро-макетов.html (session file 95017952-41e3-40a5-994c-bae4da79b979). Visible capsule width 40 CSS px and icons 21 CSS px are design references, NOT measured native window bounds. Preserve compact placement, inward-opening cards, state lamps and controlled motion.

## New Owner requirement
Hovering the bottom CPU/RAM block opens an interactive compact system resource monitor, with the top five resource-consuming applications and close controls. It is a hybrid mini Task Manager / performance monitor, not a static demo and not restricted to pinned Rail items.

Implementation defaults below are proposed technical choices, not additional Owner quotations:
- CPU and RAM ranking tabs, RAM initially selected; each row shows both CPU and memory. Up to five rows; never fabricate rows or unavailable readings.
- Collect real Windows metrics through the existing Rust/sysinfo seam. Aggregate processes by validated application identity/full executable path, not filename alone. Clearly label grouping limits and inaccessible/background entries. CPU is normalized to total machine capacity; memory is resident working set with units, not private bytes or exclusive memory. Summed working sets can include shared pages.
- Show system CPU/RAM totals and short rolling history. No extra graphics engine, dependency upgrade, GPUI migration or unapproved heavy effect library. No additional I/O subsystem in the first increment.
- Update around once per second while visible, using monotonic sample timestamps. Rank by the selected metric with deterministic ties. Preserve last valid samples between reads; unavailable/stale is not zero. Freeze row identity during interaction so a moving ranking cannot redirect a close action.
- Popup opens inward within the monitor work area, accounting for DPI and multi-monitor boundaries. It must accept clicks; transition from footer to popup must not close it. Support Escape/outside dismissal, keyboard access and reduced motion. Pause hover magnification/card competition during drag; retain functioning OLE/drop lease and native hit-test behavior.
- Row × requests normal closure of the group's validated top-level windows, preserving each app's unsaved-work dialogs. This is not forced process termination. Verify current PID/start time/window ownership before dispatch. Show pending, refused, unavailable or not-responding results honestly. Do not auto-elevate, close Rail itself or terminate protected/system/shell processes. Entries without safely closable windows have a disabled close control with explanation. Forced termination, if later requested, needs a separate explicit confirmation and scoped task; it is not part of this MVP.

## Recon grounding
Reviewed main 5439e3c4fdb59964b499e33562c7c52fea7330bd: tracker.rs samples all processes but publishes per-item metrics only for configured targets; SysRt contains system totals. Flyout.tsx requires a configured item and renders per-item info/metrics. bridge/api.ts has no system-top-five or dedicated group-close contract. Therefore this feature needs a real backend/UI/interactive-window contract, not merely five UI rows or a decorative ×. Existing metrics retention and native drag protections must be preserved and tested.

## Delivery sequence — no further design wave required
1. After S3 authorization is actually resolved, issue a bounded repair TASK: clipping/window bounds, stable icon slots and stationary-hover behavior, reachable close/hide/exit actions; apply approved light themes with default Glass and retain Classic Windows. Preserve working shell drag/drop. Update project README through the coder's explicit allow-list.
2. Separate bounded mini-monitor increment: backend application snapshot, interactive footer popup and safe close requests. Use the approved micro visual language; only a focused popup review if a material layout decision is needed.
3. Native regression and delivery: build identity (source commit/exe SHA/path/PID), real Explorer shortcut and folder drops, reorder + persistence/restart, stationary hover, crowded rail/scroll, DPI 100/125/150/200 percent and monitor edges. Exercise CPU/RAM ranking and normal close with disposable test apps (including unsaved-work dialog), never automation that kills the Owner's working apps. Timestamp-paired independent reference metrics and explicit unavailable cases; no PASS from IPC/mock substitutes. Publish provenance and release instructions so the Owner launches the tested executable.

A theme change or successful helper check is not proof that product bugs are fixed. Keep PARTIAL/FAIL for missing native evidence. Analyst must issue executable TASK/allow-list after S3, not infer authorization from this design approval.
