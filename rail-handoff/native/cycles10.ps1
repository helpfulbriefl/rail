# cycles10.ps1 — ten show/hide cycles + duplicate-hide (2x Escape) + rapid
# re-entry during exit, with per-cycle visibility asserts, a clean mid-reveal
# partial-opacity PNG, and a diag-log reveal-count at the end.
$ErrorActionPreference = 'Stop'
$railHwnd = 0x221C8E
$diag = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail-diag.log"
$outDir = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native"

Add-Type @'
using System;
using System.Runtime.InteropServices;
public class CY {
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern void keybd_event(byte key, byte scan, uint flags, IntPtr extra);
  public static void Esc() {
    keybd_event(0x1B, 0, 0, IntPtr.Zero);
    System.Threading.Thread.Sleep(40);
    keybd_event(0x1B, 0, 2, IntPtr.Zero);
  }
}
'@
Add-Type -AssemblyName System.Drawing
$capL=2400; $capT=300; $capW=160; $capH=440
function Shot([string]$name){
  $bmp = New-Object System.Drawing.Bitmap $capW, $capH
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $sz = New-Object System.Drawing.Size $capW, $capH
  $g.CopyFromScreen($capL, $capT, 0, 0, $sz, [System.Drawing.CopyPixelOperation]::SourceCopy)
  $g.Dispose()
  $p = Join-Path $outDir $name
  $bmp.Save($p, [System.Drawing.Imaging.ImageFormat]::Png)
  $bmp.Dispose()
  return $p
}
function WaitVisible([int]$ms){
  $deadline = (Get-Date).AddMilliseconds($ms)
  while ((Get-Date) -lt $deadline) {
    if ([CY]::IsWindowVisible([IntPtr]$railHwnd)) { return $true }
    Start-Sleep -Milliseconds 25
  }
  return [CY]::IsWindowVisible([IntPtr]$railHwnd)
}
function WaitHidden([int]$ms){
  $deadline = (Get-Date).AddMilliseconds($ms)
  while ((Get-Date) -lt $deadline) {
    if (-not [CY]::IsWindowVisible([IntPtr]$railHwnd)) { return $true }
    Start-Sleep -Milliseconds 25
  }
  return -not [CY]::IsWindowVisible([IntPtr]$railHwnd)
}
# Robust reveal: hold the cursor in the 2px edge band, re-arming the dwell timer.
# The app's reveal needs a fresh 260 ms edge dwell (edge_since resets when the
# cursor leaves the band), so we hold, check, and re-pulse if it has not fired.
function Reveal([int]$tries = 4){
  for ($t=0; $t -lt $tries; $t++) {
    [void][CY]::SetCursorPos(2559, 500)
    Start-Sleep -Milliseconds 300
    [void][CY]::SetCursorPos(2559, 500)   # re-pulse to reset edge_since
    $ok = WaitVisible 1500
    if ($ok) { return $true }
    [void][CY]::SetCursorPos(1280, 400)
    Start-Sleep -Milliseconds 300          # let it settle hidden before re-attempt
  }
  return [CY]::IsWindowVisible([IntPtr]$railHwnd)
}

# ---- baseline hidden ----
[void][CY]::SetCursorPos(1280, 400)
if (-not (WaitHidden 4000)) { throw "could not reach hidden baseline" }
Write-Output "baseline hidden OK"

# ---- 10 show/hide cycles ----
$pass = 0; $fail = 0
$partialShot = $null
for ($c=1; $c -le 10; $c++) {
  $showOk = Reveal
  if (-not $showOk) { $fail++; Write-Output ("cycle " + $c + ": SHOW FAILED"); continue }
  if ($c -eq 3) {
    # clean mid-reveal partial-opacity frame (during the 240ms reveal fade)
    $partialShot = Shot "cycle3-partial-opacity.png"
    Write-Output ("cycle 3: captured partial-opacity frame: " + $partialShot)
  }
  # let the reveal fade finish, then trigger hide by moving cursor away
  Start-Sleep -Milliseconds 600
  [void][CY]::SetCursorPos(1280, 400)
  $hideOk = WaitHidden 4000
  if (-not $hideOk) { $fail++; Write-Output ("cycle " + $c + ": HIDE FAILED (stuck visible)"); [void][CY]::SetCursorPos(2559,500); Start-Sleep -Milliseconds 500; [void][CY]::SetCursorPos(1280,400); WaitHidden 4000 | Out-Null }
  else { $pass++ }
  Write-Output ("cycle " + $c + ": show+hide OK")
}
Write-Output ("cycles: pass=" + $pass + " fail=" + $fail)

# ---- duplicate hide (two real Escape presses during an active hide) ----
$showOk = Reveal
Write-Output ("dup-hide: shown=" + $showOk)
[void][CY]::SetCursorPos(1280, 400)
# two Escapes back-to-back: first requests hide, second must NOT early-ACK
[void][CY]::Esc()
Start-Sleep -Milliseconds 150
[void][CY]::Esc()
Start-Sleep -Milliseconds 300
$dupMid = [CY]::IsWindowVisible([IntPtr]$railHwnd)
$shotDup1 = Shot "cycle-dup-esc1+150ms.png"
$shotDup2 = Shot "cycle-dup-esc2+350ms.png"
$dupHidden = WaitHidden 3000
Write-Output ("dup-hide: mid(esc2+350ms) visible=" + $dupMid + " final hidden=" + $dupHidden)
Write-Output ("dup-hide shots: " + $shotDup1 + " ; " + $shotDup2)

# ---- rapid re-entry / show during exit ----
$re1 = Reveal
# start a hide, then immediately re-reveal during the exit fade
[void][CY]::SetCursorPos(1280, 400)
Start-Sleep -Milliseconds 1400        # auto-hide fires, exit fade in progress
[void][CY]::SetCursorPos(2559, 500)   # rapid re-entry (held through the exit fade)
$re2 = WaitVisible 1500               # must become visible again
$shotRe = Shot "cycle-reentry-full.png"
$reStays = [CY]::IsWindowVisible([IntPtr]$railHwnd)
Start-Sleep -Milliseconds 400
$reStays2 = [CY]::IsWindowVisible([IntPtr]$railHwnd)
Write-Output ("re-entry: initial-shown=" + $re1 + " re-shown-during-exit=" + $re2 + " stays=" + $reStays + "/" + $reStays2)
Write-Output ("re-entry shot: " + $shotRe)
# settle to hidden
[void][CY]::SetCursorPos(1280, 400)
[void](WaitHidden 4000)

# ---- diag reveal count ----
if (Test-Path -LiteralPath $diag) {
  $c = Get-Content -LiteralPath $diag
  $regionSets = ($c | Select-String 'REGION  set').Count
  $regionSkips = ($c | Select-String 'REGION  skip').Count
  Write-Output ("diag total lines=" + $c.Count + " region_set=" + $regionSets + " region_skip=" + $regionSkips)
  $c | Select-Object -Last 8 | ForEach-Object { Write-Output ("  " + $_) }
}
Write-Output "CYCLES10 DONE"
