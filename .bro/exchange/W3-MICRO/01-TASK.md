---
ver: v3.0 · build: JOIN · wave: W3-MICRO · prev: S3-RECONCILE · layer: product
tasks: 6 · pass_criteria: 6
allow: src/**; src-tauri/src/config.rs; src-tauri/src/core.rs; src-tauri/src/diag.rs; src-tauri/src/main.rs; src-tauri/src/tracker.rs; src-tauri/src/win.rs; qa/**; README.md; .bro/**
modules: METRICS · mirror: - · risky: yes
---
# W3-MICRO — repair Rail, approved light themes, functional mini system monitor

## 0 Authority and boundaries
BOOT: Q/docs/prompt-system/core/CODER-BOOT.md. Q=C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish. P=C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail remains read-only. Read MAP, current NEXT, S3 07-DELIVERY-ACCEPT and W2-DESIGN/05-OWNER-APPROVAL. Product source baseline: published fbf7513e86e0c8e68f318c8ff95bfafadec2a493; subsequent analyst commits are expected documentation changes.

S3 publication gate is cleared. Synchronize analyst docs by safe authenticated fetch, ancestry check and --ff-only, preserving local/untracked files. Unexpected divergence/collision -> STOP, no discard/stash/reset/rebase/force. Use unchanged BRO helper from ignored Q tooling, never destructive mirrors or original SYS mutation.

Keep Tauri/Rust/React/TypeScript/Motion/CSS stack and existing locked dependencies. No migration, new graphics engine, dependency/lockfile/manifest/workflow changes, automatic elevation or startup-registration change. Intentionally untracked .github remains untouched. .bro allow excludes immutable earlier REPORT/SECURITY/ACCEPT, old tags, prompt-system sources and tooling modification. Apply accepted decisions to MAP/LOG only through normal coder protocol. Source edits must serve this TASK, not unrelated cleanup. If essential permissions/manifest changes fall outside allow, report the exact needed amendment, no bypass.

This task authorizes implementation, normal scoped commits/publication and local locked builds, not killing the Owner's working applications. Execute in two attributable increments: A repair/themes; B mini monitor. Do not wait for another broad mockup approval.

## 1 Task — identify and reproduce the correct baseline
Record source SHA, executable path/size/SHA-256, PID and running window identity. The old 22b21d28... EXE is not the baseline; the prior tested 43a13486... EXE also does not contain these future repairs. Account for single-instance forwarding; do not stop an Owner process without permission. Reproduce clipping, stationary-hover icon jitter, missing/unreachable close actions and footer sampling behavior. Retain correct existing behavior and capture native drop baseline. Focused source inspection is required; a new reconnaissance-only wave is not.

## 2 Task — increment A: geometry, stable motion and actions
Fix native window/render/hit-test geometry together, monitor work area, DPI physical/logical conversion and available multi-monitor edges. Reserve actual room for magnification and light effects without widening the visible rail unnecessarily. Crowded lists scroll internally while essential controls and footer stay reachable. Cover supported side placement rather than silently breaking horizontal/left layouts.

Fixed icon slots: stationary cursor settles; deliberate dock magnification remains smooth. Runtime digits/status/badges do not reflow neighbors. Keep popup movement coherent. Disable competing hover effects during dragging; preserve working OLE/drop lease, hit regions, reorder and persistence. Do not claim a whole subsystem fixed by turning off all animation.

Explicit actions: normal application-window closure, remove shortcut, hide Rail and exit Rail are separate, labelled and accessible. Include a recoverable way to reveal a hidden rail. Preserve unsaved-work dialogs; no forced kill. Do not add an ambiguous X whose meaning changes silently. Record actual defect causes and before/after proof; unrelated potential bugs are reported, not silently fixed.

## 3 Task — increment A: approved light themes
Implement Classic Windows, Porcelain, Aurora, Glass, Studio using approved MICRO material/light/state-lamp language. Glass is default for new/default configurations; preserve an explicitly saved theme and keep existing themes including dark variants. Do not invent newly approved dark redesigns. Classic Windows remains selectable. Compact capsule reference is 40 CSS px with 21 CSS px icons, not a native-window measurement or mandate to shrink existing user-selected sizes. Retain supported size settings and save/restart behavior. Reduced motion and high-legibility fallback must work; HTML glass imitation is not proof of real Windows desktop blur. No external component copying without verified redistribution rights.

## 4 Task — increment B: real interactive system mini monitor
Footer CPU/RAM hover opens an inward-facing compact popup; keyboard/click activation also possible. Proposed first layout around 300 CSS px wide, shrink/clamp to work area; no new gallery wave. Cursor can cross footer->popup without dismissal. Popup accepts actions, remains alive while used, supports Escape/outside dismissal and correct bounds/DPI. Coordinate autohide and existing app flyouts; don't reuse click-through behavior blindly.

CPU/RAM tabs, RAM initially selected, up to five system-wide applications including unpinned apps. Each row: icon/name, CPU %, resident memory with units, normal-close X. Include system totals and short rolling CPU/RAM histories. Backend must supply real data; current configured-item metrics are not a system-wide list. No new I/O subsystem or forced-termination function in this increment.

Use existing Rust/sysinfo seam. Define aggregation by validated application identity/full executable path, not basename alone. Account for multiprocess applications where identity is demonstrable; document background/inaccessible grouping limits. CPU normalized to total logical CPU capacity. Memory resident working set, not private/exclusive bytes; summed shared pages may double-count. Missing/unavailable/stale data is labelled, never fabricated zero. Timestamp samples, retain valid readings between refreshes, avoid repeated duplicate history points. Refresh about once per second while popup visible within existing sampler constraints; stop unnecessary hidden polling. Rank on the selected recent valid metric, deterministic ties. Freeze target identity through interaction; a rank refresh must not redirect a pending X click.

Close X: revalidate PID/start time/window ownership and request normal closure of validated top-level windows. Preserve each application's unsaved-work prompt. Show pending/refused/not-responding/unavailable honestly. No automatic elevation, forced termination, Rail self-close through ranking or protected/system/shell actions. No safely closable window -> disabled X with reason. Never pass UI-supplied arbitrary PID/path directly into privileged kill behavior. Test only disposable applications, not Owner's working apps.

## 5 Task — gates and native functional proof
Run existing locked commands after inspecting script side effects: npm run typecheck; npm run lint:css; npm run build; cargo build --manifest-path src-tauri/Cargo.toml --release --locked; npm run qa. Existing locks/install policy applies. Build/UI mock passes are not native functional proof.

Six acceptance rows, one per criterion with real evidence:
1 correct build identity and scoped locked gates;
2 clipping/bounds: crowded rail, all supported sides, work area, DPI 100/125/150/200 and available monitor cases, reachable footer/actions;
3 stationary hover settles, transition into cards stable, reduced motion, native Explorer shortcut/folder drop, UI reorder and restart persistence;
4 Glass default for fresh config, existing explicit selection preserved, five light themes and existing variants selectable/persisted;
5 functional unpinned system top-five, CPU/RAM sorting, timestamp-paired independent Windows metrics (definitions/tolerance stated before verdict), unavailable/stale cases, interactive footer->popup and stable close targeting;
6 normal close on disposable app, unsaved-work confirmation retained, inaccessible/background safety; tested binary handoff and publication provenance.

Use same-interval raw reference samples for comparison; do not compare raw instantaneous CPU to a smoothed UI value without explaining/filtering the reference. Native screenshots/video may remain private attachments; publish only sanitized scoped evidence. Physical OLE/close missing -> PARTIAL/unavailable with exact manual checklist, never PASS from IPC or seeded mocks. Persisted config fixtures must be valid UTF-8 without accidental BOM; do not silently overwrite Owner config. Do not fabricate benchmark claims or progress counts.

## 6 Task — README, commits and executable delivery
Update README with approved themes/default, safe mini-monitor behavior, normal-close limitations and exact build/launch instructions. Preserve author/site credit @yumi_acess — Tester & Developer, https://helpfulbriefl.github.io/. Record any remaining SPEC/code contradictions without modifying immutable history.

Commit A repair/themes and B mini monitor separately; explicit allow-list staging, cached scope/secret review, no git add -A. Publish source/docs via approved command-local GCM: -c credential.helper= -c credential.helper=manager -c credential.https://github.com.username=helpfulbriefl, host-only https://github.com/helpfulbriefl/rail.git, transient GIT_TERMINAL_PROMPT=0/GCM_INTERACTIVE=Never restored in finally. No gh switch/token, credential fill/keyring read, credential URL, global helper change, force/reset/rebase or alternate auth route. Failure -> STOP with non-secret status. Verify remote final SHA independently; do not move old tags.

REPORT/proof INDEX with exact tested_source_commit, build command, EXE SHA-256/size/path and actual readiness verdict. Deliver the tested EXE via an allowed attachment/artifact channel or exact local launch path if upload is unavailable; distinguish source publication from binary delivery. Never commit the 7 GB folder, target/node_modules/caches or raw sensitive logs. README must prevent launching an outdated EXE. Confirm actual new binary identity on native smoke, not just rebuild timestamp. Do not claim no bugs remain; list unresolved/native-unavailable cases. Finish BRO protocol without destructive retention/mirror effects.

Owner delivery RU: what works, what remains PARTIAL, exact EXE launch path/hash, short manual checks, commit/remote status.
End: Готово → напиши аналитику: BRO CHECK
