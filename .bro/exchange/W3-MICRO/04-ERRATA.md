# W3-MICRO — errata (appended during W3B-FIX; original 02-DELIVERY.md stays immutable)

Analyst verdict on W3-MICRO: CHANGES REQUIRED / functional FAIL (`03-ACCEPT.md`).
The following statements in `02-DELIVERY.md` are corrected here, not rewritten there:

1. "W3 … DONE" — DONE was unsupported. Five blocking source findings were confirmed
   by the analyst; the true status at `439702e43f29736ed26d18a503a2a0a3fe875ef6` is PARTIAL.
2. "frozen target identity" for the sysmon close — unsupported at that commit: the UI
   froze only `closingId` while row layout/targets stayed live, and the backend had no
   snapshot-bound PID/start-time identity. Fixed in W3B-FIX commit `0099e59`
   (`SysCloseReq{id, procs}` with PID+start-time revalidation, frozen row membership).
3. "labelled close/hide/exit in tray" — wrong location: `build_tray_menu` at `439702e`
   contained show/settings/quit only; app-close lived in the item menu. The tray now has
   explicit "Скрыть панель" and "Выход" (W3B-FIX commit `0099e59`).
4. "top-5 CPU/RAM" — the CPU tab only reordered the RAM-truncated five, so it was not a
   system CPU ranking. Fixed in W3B-FIX: bounded dual-metric snapshot ranked over the
   complete eligible process set (`sys_snapshot(all, 6, 12)`).

Native acceptance (clipping, stable hover, reachable actions, DPI, close/unsaved dialogs)
remained unproven at W3-MICRO. W3B-FIX evidence lives in
`.bro/exchange/W3B-FIX/proof/INDEX.md`; native HWND-level interaction and the DPI matrix
remain explicitly PARTIAL there.
