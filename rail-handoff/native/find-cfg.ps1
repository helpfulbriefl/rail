$c1 = 'C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\src-tauri\target\release\rail.config.json'
$c2 = Join-Path $env:APPDATA 'Rail\rail.config.json'
$outDir = 'C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native'
$r = @()
$r += ('{0}  exists={1}' -f $c1, (Test-Path $c1))
$r += ('{0}  exists={1}' -f $c2, (Test-Path $c2))
if (Test-Path $c1) {
  $r += '=== next-to-exe content ==='
  $r += (Get-Content $c1 -Raw)
  $r += ('CFG_PATH=' + $c1)
} elseif (Test-Path $c2) {
  $r += '=== appdata content ==='
  $r += (Get-Content $c2 -Raw)
  $r += ('CFG_PATH=' + $c2)
} else {
  $r += 'NOT FOUND'
}
[System.IO.File]::WriteAllLines((Join-Path $outDir '_cfg.txt'), $r)
