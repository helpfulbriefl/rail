# partial-opacity.ps1 (v2) — capture PARTIAL-OPACITY frames of the reveal-fade
# and the exit-fade. Backend sets want_visible + emits; the FRONTEND animates
# opacity and only calls w.hide() after the exit fade. So while vis=True during
# the exit fade the window is mid-fade -> that screenshot is the partial-opacity
# frame. Same for the reveal fade (vis flips True early in the 240ms reveal).
$ErrorActionPreference = 'Stop'
$railHwnd = 0x221C8E
$diag = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail-diag.log"
$outDir = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native"

Add-Type @'
using System;
using System.Runtime.InteropServices;
public class PO {
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
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

# Start hidden: move cursor to center, wait for auto-hide to complete.
[void][PO]::SetCursorPos(1280, 400)
Start-Sleep -Milliseconds 3000

# ---------- REVEAL FADE ----------
# Reveal fires ~260 ms after the edge dwell; sample up to ~1 s so we catch the
# 240 ms reveal-fade window (frames while vis flips True and stays through fade).
[void][PO]::SetCursorPos(2559, 500)   # 2px edge band, central y
Start-Sleep -Milliseconds 700          # let the 260 ms reveal dwell fire
$revShots = @()
for ($i=0; $i -lt 14; $i++) {
  $v = [PO]::IsWindowVisible([IntPtr]$railHwnd)
  if ($v) { $revShots += (Shot ("reveal-fade-" + $i + ".png")) }
  Start-Sleep -Milliseconds 40
  if (-not $v) { break }
}
$revStart = -1
if ($revShots.Count -gt 0) { $revStart = 0 }
Write-Output ("reveal first-True at sample " + $revStart + ", frames: " + $revShots.Count)
$revShots | ForEach-Object { Write-Output ("  " + $_) }
Start-Sleep -Milliseconds 500          # let reveal finish
$full = Shot "po-fully-shown.png"
Write-Output ("fully-shown: " + $full)

# ---------- EXIT FADE ----------
# Auto-hide fires ~1200 ms after cursor leaves; the exit fade is ~200 ms during
# which vis stays True (opacity animating). Sample up to ~4 s at 10 ms and grab
# the True-frames in the fade window.
[void][PO]::SetCursorPos(1280, 400)   # trigger auto-hide
$exitShots = @()
$exitStart = -1
for ($i=0; $i -lt 400; $i++) {
  Start-Sleep -Milliseconds 10
  $v = [PO]::IsWindowVisible([IntPtr]$railHwnd)
  if ($v -and $exitStart -lt 0) { $exitStart = $i }
  if ($v -and $exitStart -ge 0 -and $i -lt ($exitStart + 25)) {
    $exitShots += (Shot ("exit-fade-" + $i + ".png"))
  }
  if ($exitStart -ge 0 -and -not $v -and $exitShots.Count -ge 2) { break }
}
Write-Output ("exit first-True at sample " + $exitStart + ", frames: " + $exitShots.Count)
$exitShots | ForEach-Object { Write-Output ("  " + $_) }

# diag tail
if (Test-Path -LiteralPath $diag) {
  $c = Get-Content -LiteralPath $diag
  Write-Output ("diag lines=" + $c.Count)
  $c | Select-Object -Last 12 | ForEach-Object { Write-Output ("  " + $_) }
}
Write-Output "PARTIAL-OPACITY V2 DONE"
