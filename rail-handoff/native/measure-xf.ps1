# measure-xf.ps1 — for each xf-*.png, compute mean + max alpha/opacity of the
# panel region to detect the partial-opacity (smooth-transition) frames.
# Panel region in the 160x440 capture strip: roughly x 95..150, y 130..320
# (the white glass capsule). We report the fraction of near-opaque bright pixels.
$ErrorActionPreference = 'Stop'
$outDir = "C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native"
Add-Type -AssemblyName System.Drawing

$files = Get-ChildItem -Path $outDir -Filter "xf-*.png" | Where-Object { $_.BaseName -match '^xf-\d+$' } | Sort-Object { [int]($_.BaseName -replace 'xf-','') }
$lines = @()
foreach ($f in $files) {
  $bmp = New-Object System.Drawing.Bitmap $f.FullName
  $w = $bmp.Width; $h = $bmp.Height
  $nOpaque = 0; $nBright = 0; $nSample = 0
  # Sample a panel sub-rectangle (avoid the whole strip which is mostly desktop).
  $px0 = 95; $py0 = 130; $px1 = 150; $py1 = 320
  if ($px1 -gt $w) { $px1 = $w }
  if ($py1 -gt $h) { $py1 = $h }
  for ($y=$py0; $y -lt $py1; $y++) {
    for ($x=$px0; $x -lt $px1; $x++) {
      $px = $bmp.GetPixel($x, $y)
      $nSample++
      $bright = ($px.R -gt 235 -and $px.G -gt 235 -and $px.B -gt 235)
      if ($bright) { $nBright++ }
    }
  }
  $frac = 0
  if ($nSample -gt 0) { $frac = [math]::Round($nBright / $nSample, 3) }
  $lines += ("{0,-10} brightFrac={1}" -f $f.BaseName, $frac)
  $bmp.Dispose()
}
$lines | ForEach-Object { Write-Output $_ }
# Also report a compact sparkline of brightFrac values
$fracs = $lines | ForEach-Object { ($_ -split 'brightFrac=')[1] }
Write-Output ("frames=" + $files.Count + " minBright=" + ($fracs | Measure-Object -Minimum).Minimum + " maxBright=" + ($fracs | Measure-Object -Maximum).Maximum)
Write-Output "MEASURE-XF DONE"
