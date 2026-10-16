# xf-sat-audit.ps1 — per-frame ARTIFACT audit for the exit/reveal fade captures.
# READ-ONLY: opens each PNG, samples the same panel sub-rect as measure-xf.ps1
# (x 95..149, y 130..319 of the 160x440 capture strip) and reports:
#   brightFrac = share of pixels with R,G,B all > 235   (same metric as measure-xf.ps1)
#   satFrac    = share of pixels with (max(R,G,B)-min(R,G,B)) > 32  -> colored, not gray/white
#   darkFrac   = share of pixels with min(R,G,B) < 60
#   meanRGB    = channel means over the sub-rect
#   mtime      = NTFS LastWriteTime (actual capture time, wall clock local)
# Console output only; nothing is written to disk.
$ErrorActionPreference = 'Stop'
$outDir = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native"
Add-Type -AssemblyName System.Drawing

$names = @()
$names += (0..91 | ForEach-Object { "xf-$_" })
$names += (0..11 | ForEach-Object { "rr-$_" })
$names += @('xf-fully-shown', 'exit-fade-0', 'po-frame-0', 'po-fully-shown', 'cycle3-partial-opacity')
$names += (0..13 | ForEach-Object { "reveal-fade-$_" })

foreach ($nm in $names) {
  $path = Join-Path $outDir ($nm + '.png')
  if (-not (Test-Path -LiteralPath $path)) { Write-Output ($nm + '  MISSING'); continue }
  $mtime = (Get-Item -LiteralPath $path).LastWriteTime.ToString('HH:mm:ss.fff')
  $bmp = New-Object System.Drawing.Bitmap $path
  $w = $bmp.Width; $h = $bmp.Height
  $px0 = 95; $py0 = 130
  $px1 = [math]::Min(150, $w); $py1 = [math]::Min(320, $h)
  $n = 0; $br = 0; $sa = 0; $dk = 0; $mR = 0; $mG = 0; $mB = 0
  for ($y = $py0; $y -lt $py1; $y++) {
    for ($x = $px0; $x -lt $px1; $x++) {
      $p = $bmp.GetPixel($x, $y)
      $n++
      $mR += $p.R; $mG += $p.G; $mB += $p.B
      if ($p.R -gt 235 -and $p.G -gt 235 -and $p.B -gt 235) { $br++ }
      $mx = [math]::Max($p.R, [math]::Max($p.G, $p.B))
      $mn = [math]::Min($p.R, [math]::Min($p.G, $p.B))
      if (($mx - $mn) -gt 32) { $sa++ }
      if ($mn -lt 60) { $dk++ }
    }
  }
  $bmp.Dispose()
  Write-Output ($nm + '  mtime=' + $mtime + '  brightFrac=' + [math]::Round($br / $n, 3) +
    '  satFrac=' + [math]::Round($sa / $n, 3) + '  darkFrac=' + [math]::Round($dk / $n, 3) +
    '  meanRGB=' + [int]($mR / $n) + '/' + [int]($mG / $n) + '/' + [int]($mB / $n))
}
Write-Output "XF-SAT-AUDIT DONE"
