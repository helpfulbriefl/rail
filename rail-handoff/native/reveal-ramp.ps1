# reveal-ramp.ps1 — capture the reveal-fade at a fixed cadence and report the
# brightness ramp, proving the capsule fades in smoothly (smooth-transition).
$ErrorActionPreference = 'Stop'
$railHwnd = 0x221C8E
$outDir = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native"

Add-Type @'
using System;
using System.Runtime.InteropServices;
public class RR {
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
}
'@
Add-Type -AssemblyName System.Drawing
$capL=2400; $capT=300; $capW=160; $capH=440

# start hidden
[void][RR]::SetCursorPos(1280, 400)
Start-Sleep -Milliseconds 3000

$frames = @()
$firstTrue = -1
# fire the reveal, poll until the window is visible (dwell+tick latency up to ~1500 ms),
# then capture 12 frames (~300 ms = the capsule fade-in) offset from first-True
[void][RR]::SetCursorPos(2559, 500)
for ($i=0; $i -lt 60; $i++) {
  Start-Sleep -Milliseconds 25
  $v = [RR]::IsWindowVisible([IntPtr]$railHwnd)
  if ($v) {
    if ($firstTrue -lt 0) { $firstTrue = $i }
    $off = $i - $firstTrue
    if ($off -lt 12) {
      $bmp = New-Object System.Drawing.Bitmap $capW, $capH
      $g = [System.Drawing.Graphics]::FromImage($bmp)
      $sz = New-Object System.Drawing.Size $capW, $capH
      $g.CopyFromScreen($capL, $capT, 0, 0, $sz, [System.Drawing.CopyPixelOperation]::SourceCopy)
      $g.Dispose()
      $p = Join-Path $outDir ("rr-" + $off + ".png")
      $bmp.Save($p, [System.Drawing.Imaging.ImageFormat]::Png)
      $bmp.Dispose()
      $frames += $p
    }
    if ($off -ge 12) { break }
  }
}
$lines = @()
$lines += ("first-True at sample " + $firstTrue + " ( " + ($firstTrue*25) + " ms )")

# measure the panel-sub-rect brightness of each captured frame
$vals = @()
foreach ($p in $frames) {
  $b = New-Object System.Drawing.Bitmap $p
  $nBright=0; $n=0
  for ($y=130; $y -lt 320; $y++) { for ($x=95; $x -lt 150; $x++) { $px=$b.GetPixel($x,$y); if ($px.R -gt 235 -and $px.G -gt 235 -and $px.B -gt 235){$nBright++}; $n++ } }
  $b.Dispose()
  $vals += [math]::Round($nBright/$n,3)
}
$lines += ("brightness ramp across reveal-fade frames: " + ($vals -join ', '))
$lines += ("frame count: " + $frames.Count)
$lines += "RAMP DONE"
[System.IO.File]::WriteAllLines((Join-Path $outDir "_ramp-out.txt"), $lines)
$lines | ForEach-Object { Write-Host $_ }
