# stationary.ps1 — re-verify accepted hover-stability on the new EXE:
# panel must NOT flicker (rapid show/hide) when the cursor is stationary
# over the panel body, and must NOT spuriously reveal at center/boundaries.
$ErrorActionPreference = 'Stop'
$railHwnd = 0x221C8E
$outDir   = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native"
$diagLog  = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail-diag.log"

Add-Type @'
using System;
using System.Runtime.InteropServices;
public class ST {
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
}
'@

function DiagCount([string]$label, [string]$pattern) {
  if (Test-Path $diagLog) {
    $m = (Select-String -Path $diagLog -Pattern $pattern -SimpleMatch | Measure-Object).Count
    return ("{0}={1}" -f $label, $m)
  }
  return ("{0}=n/a" -f $label)
}

# hold a point, sample visibility at 100ms; return a trace string of True/False runs
function Hold([int]$x, [int]$y, [int]$ms) {
  [void][ST]::SetCursorPos($x, $y)
  $trace = @()
  $step = 100
  $n = [math]::Ceiling($ms / $step)
  for ($i = 0; $i -lt $n; $i++) {
    Start-Sleep -Milliseconds $step
    $v = [ST]::IsWindowVisible([IntPtr]$railHwnd)
    $trace += [string]$v
  }
  # compress to runs
  $runs = @()
  $cur = $null; $cnt = 0
  foreach ($c in $trace) {
    if ($c -eq $cur) { $cnt++ } else { if ($cur -ne $null) { $runs += ("{0}x{1}" -f $cur, $cnt) }; $cur = $c; $cnt = 1 }
  }
  if ($cur -ne $null) { $runs += ("{0}x{1}" -f $cur, $cnt) }
  $trueCt  = ($trace | Where-Object { $_ -eq 'True' } | Measure-Object).Count
  $falseCt = ($trace | Where-Object { $_ -eq 'False' } | Measure-Object).Count
  $trans = 0
  for ($i = 1; $i -lt $trace.Count; $i++) { if ($trace[$i] -ne $trace[$i-1]) { $trans++ } }
  return @{ trace = ($runs -join ' '); trueCt = $trueCt; falseCt = $falseCt; transitions = $trans }
}

$res = @()
$res += "=== stationary re-verification (rail hwnd 0x221C8E, pid 33984) ==="
$res += ("diag @start: " + (DiagCount "region-set" "REGION  set") + "  " + (DiagCount "region-skip" "REGION  skip"))

# 1) ensure hidden
[void][ST]::SetCursorPos(1280, 400)
Start-Sleep -Milliseconds 3500
$res += ("pre-phase hidden-check IsWindowVisible=" + [ST]::IsWindowVisible([IntPtr]$railHwnd))

# 2) CENTER stationary: no spurious reveal
$c = Hold 1280 400 3000
$res += ("CENTER hold3s: trace=" + $c.trace + "  trueCt=" + $c.trueCt + " falseCt=" + $c.falseCt + " transitions=" + $c.transitions)

# 3) BOUNDARY top / bottom of work area: no reveal
$bt = Hold 1280 2 2000
$res += ("BOUNDARY-top hold2s: trace=" + $bt.trace + "  trueCt=" + $bt.trueCt + " transitions=" + $bt.transitions)
Start-Sleep -Milliseconds 2000
$bb = Hold 1280 1038 2000
$res += ("BOUNDARY-bottom hold2s: trace=" + $bb.trace + "  trueCt=" + $bb.trueCt + " transitions=" + $bb.transitions)

# 4) FOOTER / panel-body hover: reveal via edge, move onto capsule, hold -> no flicker
[void][ST]::SetCursorPos(1280, 400); Start-Sleep -Milliseconds 1000
[void][ST]::SetCursorPos(2559, 500)   # edge dwell -> reveal
Start-Sleep -Milliseconds 800
$firstAfterReveal = [ST]::IsWindowVisible([IntPtr]$railHwnd)
[void][ST]::SetCursorPos(2525, 525)   # move onto capsule body (hover)
$f = Hold 2525 525 3000
$res += ("FOOTER/hover: after-edge-reveal=" + $firstAfterReveal + " hold3s trace=" + $f.trace + "  trueCt=" + $f.trueCt + " falseCt=" + $f.falseCt + " transitions=" + $f.transitions)

# 5) let it auto-hide cleanly
[void][ST]::SetCursorPos(1280, 400)
Start-Sleep -Milliseconds 3000
$res += ("post auto-hide IsWindowVisible=" + [ST]::IsWindowVisible([IntPtr]$railHwnd))

$res += ("diag @end:   " + (DiagCount "region-set" "REGION  set") + "  " + (DiagCount "region-skip" "REGION  skip"))
$res += "STATIONARY DONE"

[System.IO.File]::WriteAllLines((Join-Path $outDir "_stationary-out.txt"), $res)
$res | ForEach-Object { Write-Host $_ }
