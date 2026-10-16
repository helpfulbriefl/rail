# reorder.ps1 — real in-rail reorder via a synthetic pointer drag on the WebView,
# verified by the on-disk config order actually changing (Reorder -> setOrder ->
# reorder command -> save). Captures before/after PNGs.
$ErrorActionPreference = 'Stop'
$railHwnd = 0x221C8E
$cfg      = 'C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail.config.json'
$outDir   = 'C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native'

Add-Type @'
using System;
using System.Runtime.InteropServices;
public class RD {
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern void mouse_event(uint dwFlags, uint dx, uint dy, uint data, UIntPtr extraInfo);
}
'@
Add-Type -AssemblyName System.Drawing
$capL=2400; $capT=300; $capW=160; $capH=440

function Order() {
  $j = Get-Content $cfg -Raw | ConvertFrom-Json
  $names = @(); $ids = @()
  foreach ($it in $j.items) { $names += $it.name; $ids += $it.id }
  return @{ names = ($names -join ' > '); ids = ($ids -join ',') }
}
function Shot([string]$name) {
  $bmp = New-Object System.Drawing.Bitmap $capW, $capH
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $sz = New-Object System.Drawing.Size $capW, $capH
  $g.CopyFromScreen($capL, $capT, 0, 0, $sz, [System.Drawing.CopyPixelOperation]::SourceCopy)
  $g.Dispose()
  $p = Join-Path $outDir $name
  $bmp.Save($p, [System.Drawing.Imaging.ImageFormat]::Png)
  $bmp.Dispose()
}
function Reveal() {
  [void][RD]::SetCursorPos(1280,400); Start-Sleep -Milliseconds 1200
  [void][RD]::SetCursorPos(2559,500)
  $ok = $false
  for ($t=0; $t -lt 12; $t++) { Start-Sleep -Milliseconds 100; if ([RD]::IsWindowVisible([IntPtr]$railHwnd)) { $ok=$true; break } }
  return $ok
}
function Drag([int]$x1,[int]$y1,[int]$x2,[int]$y2) {
  [void][RD]::SetCursorPos($x1,$y1); Start-Sleep -Milliseconds 400
  [RD]::mouse_event(0x0002,0,0,0,[UIntPtr]::Zero)   # left down
  Start-Sleep -Milliseconds 400                        # hold the press (drag activation)
  $steps = 22
  for ($s=1; $s -le $steps; $s++) {
    $xx = $x1 + [int]([math]::Round(($x2-$x1) * $s / $steps))
    $yy = $y1 + [int]([math]::Round(($y2-$y1) * $s / $steps))
    [void][RD]::SetCursorPos($xx,$yy); Start-Sleep -Milliseconds 60
  }
  Start-Sleep -Milliseconds 300
  [RD]::mouse_event(0x0004,0,0,0,[UIntPtr]::Zero)   # left up
  [void][RD]::SetCursorPos(1280,400)                 # move off the panel
  Start-Sleep -Milliseconds 2500
}

$lines = @()
$before = Order
$lines += ('before-order(names): ' + $before.names)
$lines += ('before-order(ids):   ' + $before.ids)

Shot 'reorder-before.png'

$candidates = @(
  @{n='comet-up';  x1=2518; y1=500; x2=2518; y2=432},
  @{n='folder-down'; x1=2518; y1=452; x2=2518; y2=524},
  @{n='comet-up2'; x1=2516; y1=502; x2=2516; y2=430}
)
$done = $false
foreach ($c in $candidates) {
  if (-not (Reveal)) { $lines += ($c.n + ': reveal-fail'); continue }
  Start-Sleep -Milliseconds 350
  $o = Order
  Drag $c.x1 $c.y1 $c.x2 $c.y2
  $after = Order
  $changed = ($after.names -ne $o.names)
  $lines += ('{0}: visible=True drag({1},{2})->({3},{4})  order {5}  changed={6}' -f $c.n, $c.x1,$c.y1,$c.x2,$c.y2, $after.names, $changed)
  if ($changed) { Shot 'reorder-after.png'; $done = $true; break }
}

$final = Order
$lines += ('final-order(names): ' + $final.names)
$lines += ('reorder-executed: ' + $done)
$lines += 'REORDER DONE'
[System.IO.File]::WriteAllLines((Join-Path $outDir '_reorder-out.txt'), $lines)
$lines | ForEach-Object { Write-Host $_ }
