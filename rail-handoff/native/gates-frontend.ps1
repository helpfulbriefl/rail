# gates-frontend.ps1 — re-run the frontend/JS gates and record each exit code.
$ErrorActionPreference = 'Continue'
$root   = 'C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish'
$outDir = 'C:\Users\BazaCode\Desktop\cody-proj\win-utils\rail-publish\rail-handoff\native'
Push-Location $root

$rec = @()
function RunGate([string]$label, [scriptblock]$sb) {
  $rec += ('=== ' + $label + ' ===')
  & $sb 2>&1 | ForEach-Object { $rec += ($_ | Out-String).TrimEnd() }
  $ec = $LASTEXITCODE
  if ($null -eq $ec) { $ec = 0 }
  $rec += ('EXITCODE=' + $ec)
}

RunGate 'npm run typecheck'   { npm run typecheck }
RunGate 'npm run lint:css'    { npm run lint:css }
RunGate 'npm run build'       { npm run build }
RunGate 'node qa/storyboard.mjs' { node 'qa/storyboard.mjs' }
RunGate 'node qa/visual.mjs (63 scenes)' { node 'qa/visual.mjs' }

$rec += 'FRONTEND GATES DONE'
[System.IO.File]::WriteAllLines((Join-Path $outDir '_gates-frontend.txt'), $rec)
$rec | ForEach-Object { Write-Host $_ }
Pop-Location
