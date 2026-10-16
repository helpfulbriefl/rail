# exit-fade.ps1 — reveal, then capture the EXIT fade at ~5 ms cadence so we
# reliably get a PARTIAL-OPACITY frame (the window stays IsWindowVisible=True
# through the ~200 ms frontend exit fade, then w.hide() flips it False).
$ErrorActionPreference = 'Stop'
$railHwnd = 0x221C8E
$diag = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail-diag.log"
$outDir = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native"

Add-Type @'
using System;
using System.Runtime.InteropServices;
public class XF {
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

# 1) start hidden
[void][XF]::SetCursorPos(1280, 400)
Start-Sleep -Milliseconds 3000

# 2) reveal and confirm fully shown
[void][XF]::SetCursorPos(2559, 500)
Start-Sleep -Milliseconds 800
$shown = [XF]::IsWindowVisible([IntPtr]$railHwnd)
Write-Output ("shown after reveal=" + $shown)
$full = Shot "xf-fully-shown.png"
Write-Output ("fully-shown: " + $full)

# 3) trigger exit; rapid-capture through the fade (vis stays True ~200 ms).
# Auto-hide fires ~1200 ms after cursor leaves (everHovered=True from the
# reveal dwell). Arm at 1250 ms so the loop catches the True-frames of the fade.
[void][XF]::SetCursorPos(1280, 400)
Start-Sleep -Milliseconds 1250
$frames = @()
$firstFalse = -1
for ($i=0; $i -lt 120; $i++) {
  $v = [XF]::IsWindowVisible([IntPtr]$railHwnd)
  if ($v) { $frames += (Shot ("xf-" + $i + ".png")) }
  else { if ($firstFalse -lt 0) { $firstFalse = $i } }
  Start-Sleep -Milliseconds 5
}
Write-Output ("exit-fade True-frames=" + $frames.Count + " first-False at " + $firstFalse)
$frames | ForEach-Object { Write-Output ("  " + $_) }

# 4) diag tail
if (Test-Path -LiteralPath $diag) {
  $c = Get-Content -LiteralPath $diag
  Write-Output ("diag lines=" + $c.Count)
  $c | Select-Object -Last 10 | ForEach-Object { Write-Output ("  " + $_) }
}
Write-Output "EXIT-FADE DONE"
