# reveal-shot.ps1 — hold cursor on the right edge, then screenshot the rail panel
# region (CopyFromScreen of the window rect) to SEE whether the panel actually renders.
# Also reads the rail-diag.log tail. Decisive evidence for the reveal.
$ErrorActionPreference = 'Stop'
$railHwnd = 0x221C8E  # "Rail" window (from W2 enumeration)
$diag = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail-diag.log"
$outDir = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-handoff\native"

Add-Type @'
using System;
using System.Runtime.InteropServices;
public class RS {
  [StructLayout(LayoutKind.Sequential)] public struct P { public int X, Y; }
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern bool GetCursorPos(out P p);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
}
'@
Add-Type -AssemblyName System.Drawing

# 1) hold the cursor on the right edge (2px band -> x=2559), central y
[void][RS]::SetCursorPos(2559, 500)
$cp = New-Object RS+P
[void][RS]::GetCursorPos([ref]$cp)
Write-Output ("cursor now at " + $cp.X + "," + $cp.Y + " (target 2559,500)")

# 2) wait for reveal (260 ms dwell) + a margin
Start-Sleep -Milliseconds 600
$vis = [RS]::IsWindowVisible([IntPtr]$railHwnd)
Write-Output ("IsWindowVisible(rail)=" + $vis)

# 3) screenshot the right-edge panel strip so we can SEE the rail
#    Panel rect from show_rail (side=right): x = work.w - cross .. 2560, y centered.
#    Capture a generous strip: x 2400..2560, y 300..740.
$capL = 2400; $capT = 300; $capW = 160; $capH = 440
$bmp = New-Object System.Drawing.Bitmap $capW, $capH
$g = [System.Drawing.Graphics]::FromImage($bmp)
$sz = New-Object System.Drawing.Size $capW, $capH
$g.CopyFromScreen($capL, $capT, 0, 0, $sz, [System.Drawing.CopyPixelOperation]::SourceCopy)
$g.Dispose()
$shot = Join-Path $outDir "reveal-shot-1.png"
$bmp.Save($shot, [System.Drawing.Imaging.ImageFormat]::Png)
$bmp.Dispose()
Write-Output ("saved " + $shot)

# 4) wait longer (full reveal fade ~240ms + settle) and shoot again
Start-Sleep -Milliseconds 800
$bmp2 = New-Object System.Drawing.Bitmap $capW, $capH
$g2 = [System.Drawing.Graphics]::FromImage($bmp2)
$g2.CopyFromScreen($capL, $capT, 0, 0, $sz, [System.Drawing.CopyPixelOperation]::SourceCopy)
$g2.Dispose()
$shot2 = Join-Path $outDir "reveal-shot-2.png"
$bmp2.Save($shot2, [System.Drawing.Imaging.ImageFormat]::Png)
$bmp2.Dispose()
Write-Output ("saved " + $shot2)

# 5) move cursor away and read diag log tail
[void][RS]::SetCursorPos(1280, 400)
Start-Sleep -Milliseconds 300
if (Test-Path -LiteralPath $diag) {
  $c = Get-Content -LiteralPath $diag
  Write-Output ("diag lines=" + $c.Count)
  $c | Select-Object -Last 25 | ForEach-Object { Write-Output ("  " + $_) }
} else {
  Write-Output "diag log not found"
}
Write-Output "REVEAL-SHOT DONE"
