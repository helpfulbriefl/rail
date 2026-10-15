param([string]$Destination = (Join-Path $env:TEMP 'rail-smooth-transition'))
$ErrorActionPreference = 'Stop'
$expected = 'ee478bc8e37d2475c4e85c678a3c085a72e47ff046b50410fdaf4bd857e34df1'
New-Item -ItemType Directory -Path $Destination -Force | Out-Null
$base64Url = 'https://raw.githubusercontent.com/helpfulbriefl/rail/refs/heads/analyst/rail-smooth-transition-package/.bro/packages/rail-smooth-transition/rail-smooth-transition.zip.b64'
$encoded = (Invoke-WebRequest -UseBasicParsing -Uri $base64Url).Content
if ($encoded -is [byte[]]) { $encoded = [Text.Encoding]::UTF8.GetString($encoded) }
$bytes = [Convert]::FromBase64String([string]$encoded)
$sha = [Security.Cryptography.SHA256]::Create()
try { $actual = ([BitConverter]::ToString($sha.ComputeHash($bytes))).Replace('-', '').ToLowerInvariant() }
finally { $sha.Dispose() }
if ($actual -ne $expected) { throw 'Archive SHA256 mismatch. Nothing extracted or applied.' }
$zipPath = Join-Path $Destination 'rail-smooth-transition.zip'
if (Test-Path -LiteralPath $zipPath) {
  if ((Get-FileHash -LiteralPath $zipPath -Algorithm SHA256).Hash.ToLowerInvariant() -ne $expected) { throw 'A different ZIP already exists. Choose a new Destination; no overwrite.' }
} else { [IO.File]::WriteAllBytes($zipPath, $bytes) }
$folder = Join-Path $Destination 'rail-smooth-transition'
if (!(Test-Path -LiteralPath $folder)) { Expand-Archive -LiteralPath $zipPath -DestinationPath $Destination }
Write-Host "Verified ZIP: $zipPath"
Write-Host "Read: $(Join-Path $folder 'CODER-TASK.md')"
Write-Host 'Package downloaded only. No project changes, patch apply, git operations or credentials.'
