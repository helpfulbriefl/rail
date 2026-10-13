# METERS T4 — W1C-NATIVE (TASK §3.4: 60 s paired live `.rail-foot` + independent reference)

Diagnostic launch: fixture exe only, child-only CDP loopback `127.0.0.1:9222`
(`WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` set on the launched process, not the user's daily
browser; port verified free; listener owned by the fixture child tree). Reads the **actual
rendered `.rail-foot` text** (`.meter` → `.meter-k` / `.meter-v`), not backend synthetic telemetry.

## 1. Live Rail footer series (producer = `tracker.rs` sysinfo via IPC → `.rail-foot`)
- Source: `run/t4-foot-series.json` (CDP `Runtime.evaluate` on the rail webview, 60 s).
- **30 samples**, poll ≈ 2 s, **all finite**, unit = percent (0–100, `Math.min(max(v,0),100)`
  display clamp in `Rail.tsx:405 Meter`).
- Ranges across the 60 s window: **CPU 27–72 %**, **RAM 89–98 %**.
- Representative points (t s : CPU %, RAM %, level):
  - t=0  : 62 warm / 97 hot
  - t=10 : 57 ok   / 98 hot
  - t=20 : 55 ok   / 94 hot
  - t=30 : 34 ok   / 91 hot
  - t=40 : 35 ok   / 90 hot
  - t=50 : 32 ok   / 90 hot

## 2. Independent system reference (same host clock, NOT the Rail producer)
- Source: `run/t4-ref-meters.json` (WMI/CIM: `Win32_Processor`/`Win32_OperatingSystem`
  `FreePhysicalMemory` / `TotalVisibleMemorySize`, host-wide, ≈60 s, 20 samples).
- **RAM paired & valid**: reference **94.7–98.9 %** of physical (32 344 MB total) vs rail
  89–98 % — consistent, both finite, same host clock.
- **CPU pairing DEGRADED (documented, not inferred)**: the independent CPU reference read
  **0 %** across the window in this VM (`Win32_Processor.LoadPercentage` /
  `Win32_PerfFormattedData_PerfOS_Processor` artifact under virtualization; `Get-Counter`
  `% Processor Time` raised `CounterApiError`). The Rail-side CPU figures are finite and
  plausible; the *reference* leg for CPU is invalid here, so CPU is **not** cross-validated.
- Fixture process working set held at **16.7 MiB** throughout (no growth trend observed).

## 3. Pairing verdict
- RAM: **paired, valid, finite** (rail footer ≈ independent reference).
- CPU: rail-series finite, independent reference invalid in VM (0 %) → **not paired, documented
  as a VM limitation** (re-run on a non-VM host to validate the CPU leg).
- **No "not a leak / fixed" conclusion** — B-004 remains open.

## 4. Provenance / units / timing (TASK §3.4 "document CPU sampling intervals, EMA lag, RAM units/denominator")
- Producer EMA: CPU α=0.25; RAM 24 MB dead-band (system-level, `tracker.rs`).
- Reference sampling: `Win32_OperatingSystem` snapshot per 3 s loop (non-blocking WMI, chosen
  after `Get-Counter` CPU counter errored in this VM); RAM denominator = `TotalVisibleMemorySize`
  (KB), numerator delta = `FreePhysicalMemory` (KB).
- Records use the same host clock (ISO-8601 UTC+3); raw `run/*.json` ignored, sanitized paired
  samples published here. No credentials/personal paths/raw dumps published.
