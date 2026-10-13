<#
 bro.ps1 - BRO v3.0 helper. Windows PowerShell 5.1+ and PowerShell 7. Keep this file ASCII only.
 Call: powershell -NoProfile -ExecutionPolicy Bypass -File SYS\tools\bro.ps1 <cmd> [args]
   init <P> <W> [NEW|JOIN|LOCAL] [prevW]  create missing .bro tree, write inbox/NEXT.md
   begin <P>        phase A: apply prev ACCEPT LOG, metrics row, summary, state (turn coder)
   finish <P>       phase D: auto keys in REPORT, mirrors, check, state (turn analyst)
   check <P> [W]    validate REPORT keys + proof/INDEX.md
   ptok <path>...   proxy tokens = cyrillic/3 + other/4 (files or folders)
   added <P> [base] diff vs base without .bro: files, +lines, -lines, out_of_scope vs TASK allow
   row <P> <prevW> [curW]   upsert WAVES.csv row (+ copy to SYS/metrics/projects)
   summary <P>|ALL  SUMMARY.md: 7 indicators per version
   state <P> [coder|analyst|owner]  state.js + STATUS.md
   build            builds/START-*.md from core + mode + templates, print weights
#>
param(
  [Parameter(Position = 0)][string]$Cmd = 'help',
  [Parameter(Position = 1, ValueFromRemainingArguments = $true)][string[]]$Rest
)
$ErrorActionPreference = 'Stop'
$SYS = Split-Path -Parent $PSScriptRoot
$Enc = New-Object System.Text.UTF8Encoding $false
$Inv = [System.Globalization.CultureInfo]::InvariantCulture
$SEP = ' ' + [string][char]0x00B7 + ' '
$OK = [string][char]0x2705
$NO = [string][char]0x274C
$EmptyTree = '4b825dc642cb6eb9a060e54bf8d69288fbee4904'
$Cols = @('wave','ver','build','layer','t_issued','t_start','t_report','t_accept','tasks','done','gates_first','repair','diff_files','diff_add','diff_del','out_of_scope','q','contradictions','proof_ok','proof_req','k','o','spot','false_done','defects','causes','owner_touches','owner_fixes','gh_calls','ctx_pct','an_in','an_out','cd_in','cd_out','prompt_weight','friction_cd','friction_an','verdict')
$ReportKeys = @('wave','ver','t_start','t_report','commit','tag','push','bundle','pack','exchange','tasks','done','gates_first','repair','proof_ok','proof_req','diff_files','diff_add','diff_del','out_of_scope','q','contradictions','K1','K2','K3','K4','k_why','friction_cd','cd_in','cd_out','prompt_weight','an_in_prev','an_out_prev')
$TextExt = @('.md','.txt','.js','.mjs','.cjs','.ts','.tsx','.jsx','.json','.yml','.yaml','.csv','.html','.htm','.css','.scss','.ps1','.psm1','.log','.py','.toml','.xml','.svg','.sh','.cmd','.bat','.ini','.cfg','.rs','.go','.java','.kt','.cs','.vue','.svelte','.sql','.tape')
$script:GitCode = 0
try { [Console]::OutputEncoding = [System.Text.Encoding]::UTF8 } catch { }

function Say([string]$m) { Write-Host $m }
function Warn([string]$m) { Write-Host ('WARN: ' + $m) }
function Fail([string]$m) { Write-Host ('ERROR: ' + $m); exit 1 }
function Norm([string]$p) { $s = [System.IO.Path]::DirectorySeparatorChar; return $p.Replace('/', $s).Replace('\', $s) }
function J([string]$a, [string]$b) { return (Norm ([System.IO.Path]::Combine($a, $b))) }
function Expand-Env([string]$p) {
  if (-not $p) { return $p }
  $p = [Environment]::ExpandEnvironmentVariables($p)
  return [regex]::Replace($p, '\$env:([A-Za-z_][A-Za-z0-9_]*)', { param($m) $envVal = [Environment]::GetEnvironmentVariable($m.Groups[1].Value); if ($envVal) { $envVal } else { $m.Value } })
}
function Full([string]$p) { return $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath((Expand-Env $p)) }
function Read-Text([string]$p) {
  if (-not $p -or -not (Test-Path -LiteralPath $p -PathType Leaf)) { return $null }
  return [System.IO.File]::ReadAllText((Full $p))
}
function Ensure-Dir([string]$p) { if (-not (Test-Path -LiteralPath $p)) { New-Item -ItemType Directory -Force -Path $p | Out-Null } }
function Write-Text([string]$p, [string]$t) {
  $f = Full $p; $d = Split-Path -Parent $f
  if ($d) { Ensure-Dir $d }
  [System.IO.File]::WriteAllText($f, $t, $Enc)
}
function Now { return (Get-Date).ToString('yyyy-MM-dd HH:mm', $Inv) }
function Num($v) {
  $d = 0.0
  if ($null -ne $v -and [double]::TryParse([string]$v, [System.Globalization.NumberStyles]::Float, $Inv, [ref]$d)) { return $d }
  return $null
}
function Get-Ptok([string]$t) {
  if (-not $t) { return 0 }
  $c = [regex]::Matches($t, '[\u0400-\u04FF]').Count
  return [int][math]::Round(($c / 3.0) + (($t.Length - $c) / 4.0), [System.MidpointRounding]::AwayFromZero)
}
function Get-FilePtok([string]$p) {
  if (-not $p -or -not (Test-Path -LiteralPath $p -PathType Leaf)) { return 0 }
  $ext = [System.IO.Path]::GetExtension($p).ToLower()
  if ($TextExt -notcontains $ext) { return 0 }
  return (Get-Ptok (Read-Text $p))
}
function Ptok-List([string]$P, [string[]]$paths) {
  $seen = @{}; $tot = 0
  foreach ($x in $paths) {
    if (-not $x) { continue }
    $q = ($x.Trim()) -replace ':\d+(-\d+)?$', ''
    if (-not $q -or $q -eq '-') { continue }
    $f = $q
    if (-not [System.IO.Path]::IsPathRooted($f)) { $f = J $P $q }
    $f = Norm $f
    if ($seen.ContainsKey($f)) { continue }
    $seen[$f] = 1
    $tot += (Get-FilePtok $f)
  }
  return $tot
}
function Split-Seg([string]$line) { return $line.Split([string[]]@($SEP), [System.StringSplitOptions]::None) }
function Parse-Segs([string]$line, $h) {
  foreach ($seg in (Split-Seg $line)) {
    $k = $seg.IndexOf(': ')
    if ($k -gt 0) { $h[$seg.Substring(0, $k).Trim()] = $seg.Substring($k + 2).Trim() }
  }
}
function Get-Passport([string]$p) {
  $h = [ordered]@{}
  $t = Read-Text $p
  if (-not $t) { return $h }
  $lines = $t -split "`r?`n"
  if ($lines.Count -lt 2 -or $lines[0].Trim() -ne '---') { return $h }
  for ($i = 1; $i -lt $lines.Count; $i++) {
    if ($lines[$i].Trim() -eq '---') { break }
    Parse-Segs $lines[$i] $h
  }
  return $h
}
function Set-Passport([string]$p, $vals) {
  $t = Read-Text $p
  $nl = "`n"; if ($t.Contains("`r`n")) { $nl = "`r`n" }
  $lines = $t -split "`r?`n"
  if ($lines[0].Trim() -ne '---') { Fail ('no --- passport in ' + $p) }
  for ($i = 1; $i -lt $lines.Count; $i++) {
    if ($lines[$i].Trim() -eq '---') { break }
    $segs = Split-Seg $lines[$i]
    for ($j = 0; $j -lt $segs.Count; $j++) {
      $k = $segs[$j].IndexOf(': ')
      if ($k -gt 0) {
        $key = $segs[$j].Substring(0, $k).Trim()
        if ($vals.Contains($key)) { $segs[$j] = $key + ': ' + [string]$vals[$key] }
      }
    }
    $lines[$i] = [string]::Join($SEP, $segs)
  }
  Write-Text $p ([string]::Join($nl, $lines))
}
function Get-Info([string]$P) {
  $h = [ordered]@{ bro = 'v3.0'; mode = 'JOIN'; project = (Split-Path -Leaf $P) }
  $t = Read-Text (J $P '.bro/VERSION')
  if ($t) { Parse-Segs (($t -split "`r?`n")[0]) $h }
  return $h
}
function Get-WaveDir([string]$P, [string]$W) {
  $st = J $P ('.bro/steps/' + $W)
  if (Test-Path -LiteralPath $st) { return $st }
  if ((Get-Info $P)['mode'] -eq 'LOCAL') { return $st }
  return (J $P ('.bro/exchange/' + $W))
}
function Get-Next([string]$P) { return (Get-Passport (J $P '.bro/inbox/NEXT.md')) }
function Invoke-Git([string]$Repo, [string[]]$GitArgs) {
  $old = $ErrorActionPreference; $ErrorActionPreference = 'Continue'
  $out = @()
  try { $out = & git -C $Repo -c core.quotepath=false @GitArgs 2>$null; $script:GitCode = $LASTEXITCODE }
  catch { $script:GitCode = 1 }
  finally { $ErrorActionPreference = $old }
  return $out
}
function Git1([string]$Repo, [string[]]$GitArgs) { $o = @(Invoke-Git $Repo $GitArgs); if ($o.Count -gt 0) { return ([string]$o[0]).Trim() }; return '' }
function Is-Git([string]$P) { return (Test-Path -LiteralPath (J $P '.git')) }
function Csv-Esc([string]$v) {
  if ($null -eq $v) { return '' }
  if ($v -match '[,"\r\n]') { return '"' + $v.Replace('"', '""') + '"' }
  return $v
}
function Read-Csv([string]$p) {
  $t = Read-Text $p
  if (-not $t) { return @() }
  $lines = @($t -split "`r?`n" | Where-Object { $_.Trim() -ne '' })
  if ($lines.Count -lt 2) { return @() }
  return @($lines | ConvertFrom-Csv)
}
function Write-Csv([string]$p, $rows) {
  $sb = New-Object System.Text.StringBuilder
  [void]$sb.Append([string]::Join(',', $Cols)).Append("`n")
  foreach ($r in $rows) {
    $vals = @()
    foreach ($c in $Cols) { $vals += (Csv-Esc ([string]$r.$c)) }
    [void]$sb.Append([string]::Join(',', $vals)).Append("`n")
  }
  Write-Text $p $sb.ToString()
}
function Upsert-Row([string]$p, $row) {
  $out = New-Object System.Collections.ArrayList
  $found = $false
  foreach ($r in @(Read-Csv $p)) {
    if ($r.wave -eq $row['wave']) { [void]$out.Add([pscustomobject]$row); $found = $true } else { [void]$out.Add($r) }
  }
  if (-not $found) { [void]$out.Add([pscustomobject]$row) }
  Write-Csv $p $out
}
function Sum-Keys($h, [string[]]$keys) {
  $s = 0; $any = $false
  foreach ($k in $keys) { $n = Num $h[$k]; if ($null -ne $n) { $s += $n; $any = $true } }
  if ($any) { return $s }
  return ''
}
function Get-Min([string]$a, [string]$b) {
  try {
    $x = [datetime]::ParseExact($a, 'yyyy-MM-dd HH:mm', $Inv)
    $y = [datetime]::ParseExact($b, 'yyyy-MM-dd HH:mm', $Inv)
    return [math]::Round(($y - $x).TotalMinutes)
  } catch { return $null }
}
function Avg($list) {
  $a = @($list | Where-Object { $null -ne $_ })
  if ($a.Count -eq 0) { return '-' }
  return [math]::Round((($a | Measure-Object -Sum).Sum / $a.Count), 1)
}

function Get-Base([string]$P, [string]$W) {
  $wd = Get-WaveDir $P $W
  $T = Get-Passport (J $wd '01-TASK.md')
  if ($T.Contains('base') -and $T['base'] -and $T['base'] -ne '-') { return $T['base'] }
  $B = Get-Passport (J $wd 'logs/begin.md')
  if ($B.Contains('base') -and $B['base'] -and $B['base'] -ne '-') { return $B['base'] }
  return $EmptyTree
}
function Get-Allow($T) {
  $a = @('.bro/*')
  if ($T.Contains('allow') -and $T['allow']) {
    foreach ($x in ([string]$T['allow']).Split(';')) {
      $y = $x.Trim().Replace('\', '/').Replace('**', '*')
      if ($y.EndsWith('/')) { $y = $y + '*' }
      if ($y -and $y -ne '-') { $a += $y }
    }
  }
  return $a
}
function Get-Diff([string]$P, [string]$Base, $Allow) {
  $r = [ordered]@{ files = 0; add = 0; del = 0; oos = 0; oosList = @(); addText = '' }
  $ns = @(Invoke-Git $P @('diff', '--numstat', $Base, 'HEAD', '--', '.', ':(exclude).bro'))
  foreach ($l in $ns) {
    $parts = ([string]$l).Split("`t")
    if ($parts.Count -lt 3) { continue }
    $r['files'] = $r['files'] + 1
    $a = Num $parts[0]; $d = Num $parts[1]
    if ($null -ne $a) { $r['add'] = $r['add'] + $a }
    if ($null -ne $d) { $r['del'] = $r['del'] + $d }
    $path = $parts[2]
    if ($path -match '=>') { $path = ($path -replace '\{[^}]*=> ([^}]*)\}', '$1') -replace '^.*=> ', '' }
    $in = $false
    foreach ($g in $Allow) { if ($path -like $g) { $in = $true; break } }
    if (-not $in) { $r['oos'] = $r['oos'] + 1; $r['oosList'] += $path }
  }
  $sb = New-Object System.Text.StringBuilder
  foreach ($l in @(Invoke-Git $P @('diff', '--unified=0', '--no-color', $Base, 'HEAD', '--', '.', ':(exclude).bro'))) {
    $s = [string]$l
    if ($s.StartsWith('+') -and -not $s.StartsWith('+++')) { [void]$sb.AppendLine($s.Substring(1)) }
  }
  $r['addText'] = $sb.ToString()
  return $r
}
function Get-Proof([string]$P, [string]$wd) {
  $r = [ordered]@{ exists = $false; req = 0; ok = 0; missing = @() }
  $t = Read-Text (J $wd 'proof/INDEX.md')
  if ($null -eq $t) { return $r }
  $r['exists'] = $true
  foreach ($l in ($t -split "`r?`n")) {
    $s = $l.Trim()
    if (-not $s.StartsWith('|')) { continue }
    $cells = @($s.Trim('|').Split('|') | ForEach-Object { $_.Trim() })
    if ($cells.Count -lt 3) { continue }
    if ($cells[0] -eq '#' -or $cells[0] -match '^:?-{2,}') { continue }
    $r['req'] = $r['req'] + 1
    if (-not $cells[$cells.Count - 1].Contains($OK)) { continue }
    $files = @([regex]::Matches($s, '`([^`]+)`') | ForEach-Object { $_.Groups[1].Value })
    $good = ($files.Count -gt 0)
    foreach ($f in $files) {
      $c = @((J $wd ('proof/' + $f)), (J $wd $f), (J $P $f))
      $hit = $false
      foreach ($x in $c) { if (Test-Path -LiteralPath ($x -replace ':\d+(-\d+)?$', '')) { $hit = $true; break } }
      if (-not $hit) { $good = $false; $r['missing'] += ($cells[0] + ': ' + $f) }
    }
    if ($files.Count -eq 0) { $r['missing'] += ($cells[0] + ': no `file` in row') }
    if ($good) { $r['ok'] = $r['ok'] + 1 }
  }
  return $r
}
function Apply-Log([string]$P, [string]$prev) {
  $ap = J (Get-WaveDir $P $prev) '03-ACCEPT.md'
  $t = Read-Text $ap
  if (-not $t) { Warn ('no ACCEPT for ' + $prev); return }
  $m = [regex]::Match($t, '(?s)<!-- LOG -->\s*(.*?)\s*<!-- /LOG -->')
  if (-not $m.Success) { Warn ('no <!-- LOG --> block in ' + $ap); return }
  $entry = $m.Groups[1].Value.Trim()
  $first = (($entry -split "`r?`n")[0]).Trim()
  $lp = J $P '.bro/LOG.md'
  $log = Read-Text $lp
  if ($null -eq $log) { $log = '' }
  if ($first -and $log.Contains($first)) { Say 'LOG: entry already there'; return }
  if (@($log -split "`r?`n").Count -gt 300) {
    $ld = J $P '.bro/log'; Ensure-Dir $ld
    $vol = @(Get-ChildItem -LiteralPath $ld -Filter 'LOG-*.md').Count + 1
    $vp = J $ld ('LOG-' + $vol.ToString('000') + '.md')
    Write-Text $vp $log
    $head = (Read-Text (J $SYS 'templates/LOG.md')).Replace('<project>', (Get-Info $P)['project']).TrimEnd()
    $log = $head + "`nPrevious volume: .bro/log/" + (Split-Path -Leaf $vp) + "`n"
    Say ('LOG: old volume -> ' + $vp)
  }
  Write-Text $lp ($log.TrimEnd() + "`n`n" + $entry + "`n")
  Say ('LOG: appended ' + $first)
  if ($t -match '(?s)<!-- MAP -->\s*\S.*?<!-- /MAP -->') { Say ('MAP: replace sections from <!-- MAP --> block of ' + $ap) }
}
function Cmd-Init([string]$P, [string]$W, [string]$Build, [string]$Prev) {
  if (-not $W) { Fail 'usage: init <P> <W> [NEW|JOIN|LOCAL] [prevW]' }
  if (-not $Build) { $Build = 'JOIN' }
  $Build = $Build.ToUpper()
  if (-not $Prev) { $Prev = '-' }
  $bro = J $P '.bro'
  foreach ($d in @('design', 'inbox', 'metrics', 'log')) { Ensure-Dir (J $bro $d) }
  $ver = (Read-Text (J $SYS 'VERSION')).Trim()
  $vf = J $bro 'VERSION'
  if (-not (Test-Path -LiteralPath $vf)) { Write-Text $vf ('bro: ' + $ver + $SEP + 'mode: ' + $Build + $SEP + 'project: ' + (Split-Path -Leaf $P) + "`n") }
  $info = Get-Info $P
  $rel = 'exchange'
  if ($info['mode'] -eq 'LOCAL') { $rel = 'steps'; Ensure-Dir (J $bro 'archive') }
  foreach ($f in @('MAP.md', 'LOG.md', 'SPEC.md')) {
    $dst = J $bro $f
    if (-not (Test-Path -LiteralPath $dst)) { Write-Text $dst ((Read-Text (J $SYS ('templates/' + $f))).Replace('<project path>', $P).Replace('<sys full path>', $SYS).Replace('<project>', $info['project'])) }
  }
  $wcsv = J $bro 'metrics/WAVES.csv'
  if (-not (Test-Path -LiteralPath $wcsv)) { Write-Text $wcsv ([string]::Join(',', $Cols) + "`n") }
  Write-Text (J $bro 'progress.html') (Read-Text (J $SYS 'templates/progress.html'))
  $wd = J $bro ($rel + '/' + $W)
  Ensure-Dir (J $wd 'proof'); Ensure-Dir (J $wd 'logs')
  $T = Get-Passport (J $wd '01-TASK.md')
  if ($T.Count -eq 0) { Warn ('no 01-TASK.md passport in ' + $wd + ' - save the pasted block there') }
  $ti = Now
  if ($T.Contains('t_issued') -and $T['t_issued']) { $ti = $T['t_issued'] }
  $acc = '-'
  if ($Prev -ne '-') { $acc = '.bro/' + $rel + '/' + $Prev + '/03-ACCEPT.md' }
  $nx = "---`nwave: " + $W + $SEP + 'prev: ' + $Prev + $SEP + 't_issued: ' + $ti + "`ntask: .bro/" + $rel + '/' + $W + '/01-TASK.md' + $SEP + 'accept: ' + $acc + "`n---`nBRO GO -> bro begin -> read accept -> read task.`n"
  Write-Text (J $bro 'inbox/NEXT.md') $nx
  Say ('init: ' + $bro + ' mode ' + $info['mode'] + ' wave ' + $W + ' prev ' + $Prev)
}
function Cmd-Row([string]$P, [string]$prev, [string]$cur) {
  if (-not $prev) { Fail 'usage: row <P> <prevW> [curW]' }
  $pd = Get-WaveDir $P $prev
  $T = Get-Passport (J $pd '01-TASK.md'); $R = Get-Passport (J $pd '02-REPORT.md'); $A = Get-Passport (J $pd '03-ACCEPT.md')
  if (-not $A.Contains('verdict')) { Warn ('row: no ACCEPT passport for ' + $prev); return }
  $row = [ordered]@{}
  foreach ($c in $Cols) { $row[$c] = '' }
  $row['wave'] = $prev
  foreach ($c in @('ver', 'build', 'layer', 't_issued')) { if ($T.Contains($c)) { $row[$c] = $T[$c] } }
  if (-not $row['ver'] -and $R.Contains('ver')) { $row['ver'] = $R['ver'] }
  foreach ($c in @('t_start','t_report','tasks','done','gates_first','repair','diff_files','diff_add','diff_del','out_of_scope','q','contradictions','proof_ok','proof_req','cd_in','cd_out','prompt_weight','friction_cd')) { if ($R.Contains($c)) { $row[$c] = $R[$c] } }
  foreach ($c in @('t_accept','spot','false_done','defects','causes','owner_touches','owner_fixes','gh_calls','ctx_pct','friction_an','verdict')) { if ($A.Contains($c)) { $row[$c] = $A[$c] } }
  $row['k'] = Sum-Keys $R @('K1', 'K2', 'K3', 'K4')
  $row['o'] = Sum-Keys $A @('O1', 'O2', 'O3', 'O4')
  $reads = @((J $pd '02-REPORT.md'))
  if ($A.Contains('read_list')) { $reads += @(([string]$A['read_list']).Split(';') | ForEach-Object { $_.Trim() }) }
  $row['an_in'] = Ptok-List $P $reads
  $outs = @((J $pd '03-ACCEPT.md'), (J $P '.bro/inbox/NEXT.md'))
  if ($cur) { $outs += (J (Get-WaveDir $P $cur) '01-TASK.md') }
  $row['an_out'] = Ptok-List $P $outs
  Upsert-Row (J $P '.bro/metrics/WAVES.csv') $row
  $pc = J $SYS ('metrics/projects/' + (Get-Info $P)['project'] + '.csv')
  Upsert-Row $pc $row
  Say ('row ' + $prev + ': ' + $row['verdict'] + ', an_in ' + $row['an_in'] + ', an_out ' + $row['an_out'] + ' -> WAVES.csv + ' + $pc)
}
function Cmd-Check([string]$P, [string]$W) {
  if (-not $W) { $W = (Get-Next $P)['wave'] }
  $wd = Get-WaveDir $P $W
  $R = Get-Passport (J $wd '02-REPORT.md')
  $errs = New-Object System.Collections.ArrayList
  if ($R.Count -eq 0) { [void]$errs.Add('02-REPORT.md missing or without --- passport') }
  else {
    foreach ($k in $ReportKeys) {
      if (-not $R.Contains($k)) { [void]$errs.Add('missing key: ' + $k); continue }
      $v = [string]$R[$k]
      if ($v -eq '' -or $v -eq 'auto' -or $v -eq 'you' -or $v.StartsWith('you(')) { [void]$errs.Add('not filled: ' + $k) }
    }
    foreach ($k in @('K1', 'K2', 'K3', 'K4')) { if ($R.Contains($k) -and @('0', '1') -notcontains [string]$R[$k]) { [void]$errs.Add($k + ' must be 0 or 1') } }
    if ($R.Contains('gates_first') -and @('yes', 'no') -notcontains [string]$R['gates_first']) { [void]$errs.Add('gates_first must be yes or no') }
    if ($R.Contains('repair')) { $n = Num $R['repair']; if ($null -eq $n -or $n -lt 0 -or $n -gt 3) { [void]$errs.Add('repair must be 0-3') } }
  }
  $pr = Get-Proof $P $wd
  if (-not $pr['exists']) { [void]$errs.Add('no proof/INDEX.md -> wave = FAIL (R17)') }
  elseif ($pr['req'] -eq 0) { [void]$errs.Add('proof/INDEX.md has no rows') }
  foreach ($m in $pr['missing']) { [void]$errs.Add('proof row marked ok but file missing: ' + $m) }
  if ($errs.Count -eq 0) { Say ('check ' + $W + ': OK') } else { foreach ($e in $errs) { Say ('ERROR: ' + $e) } }
  return $errs.Count
}
function Cmd-Begin([string]$P) {
  $N = Get-Next $P
  if (-not $N.Contains('wave')) { Fail 'no .bro/inbox/NEXT.md (first wave: run init)' }
  $W = $N['wave']; $prev = '-'
  if ($N.Contains('prev') -and $N['prev']) { $prev = $N['prev'] }
  $wd = Get-WaveDir $P $W
  $tp = J $wd '01-TASK.md'
  if (-not (Test-Path -LiteralPath $tp)) { Fail ('TASK missing: ' + $tp + ' (git pull? analyst wrote it?)') }
  $rp = J $wd '02-REPORT.md'
  $R = Get-Passport $rp
  if ($R.Contains('t_report') -and $R['t_report'] -and $R['t_report'] -ne 'auto') { Fail ('wave ' + $W + ' is already reported: analyst has not written the next TASK yet. Owner: write BRO CHECK to analyst first.') }
  Ensure-Dir (J $wd 'proof'); Ensure-Dir (J $wd 'logs')
  $bf = J $wd 'logs/begin.md'
  if (-not (Test-Path -LiteralPath $bf)) {
    $base = '-'
    if (Is-Git $P) { $h = Git1 $P @('rev-parse', 'HEAD'); if ($script:GitCode -eq 0 -and $h) { $base = $h } }
    Write-Text $bf ("---`nt_start: " + (Now) + $SEP + 'base: ' + $base + "`n---`n")
  }
  $ver = (Read-Text (J $SYS 'VERSION')).Trim()
  if (-not (Test-Path -LiteralPath $rp)) {
    $tpl = (Read-Text (J $SYS 'templates/02-REPORT.md')).Replace('wave: W' + $SEP, 'wave: ' + $W + $SEP).Replace('ver: v3.0', 'ver: ' + $ver).Replace('# REPORT W ', '# REPORT ' + $W + ' ')
    Write-Text $rp $tpl
  }
  $ip = J $wd 'proof/INDEX.md'
  if (-not (Test-Path -LiteralPath $ip)) { Write-Text $ip ('# PROOF ' + $W + "`n| # | requirement | proof file(s) | ok |`n|---|---|---|---|`n") }
  if ($prev -ne '-') { Apply-Log $P $prev; Cmd-Row $P $prev $W }
  Cmd-Summary $P
  Cmd-State $P 'coder'
  $T = Get-Passport $tp
  $acc = '-'; if ($N.Contains('accept')) { $acc = $N['accept'] }
  $mods = '-'; if ($T.Contains('modules')) { $mods = $T['modules'] }
  Say ('begin ' + $W + ' (prev ' + $prev + ')')
  Say ('read: ' + $acc + ' -> ' + $tp + ' -> TASK 0 files -> SYS/modules: ' + $mods)
  Say ('write: ' + $rp + ' + proof/INDEX.md')
  if ($T.Contains('risky') -and $T['risky'] -eq 'yes') {
    if ((Get-Info $P)['mode'] -eq 'LOCAL') { Say ('risky: zip allowed files to .bro/archive/pre-' + $W + '.zip') } else { Say ('risky: git tag pre-' + $W.ToLower()) }
  }
}
function Copy-Bro([string]$P, [string]$mir) {
  $src = J $P '.bro'; $dst = J $mir '.bro'
  $rc = Get-Command robocopy -ErrorAction SilentlyContinue
  if ($rc) {
    $old = $ErrorActionPreference; $ErrorActionPreference = 'Continue'
    & robocopy $src $dst /MIR /R:1 /W:1 /NFL /NDL /NJH /NJS /NP | Out-Null
    $code = $LASTEXITCODE
    $ErrorActionPreference = $old
    return ($code -lt 8)
  }
  try {
    if (Test-Path -LiteralPath $dst) { Remove-Item -LiteralPath $dst -Recurse -Force }
    Copy-Item -LiteralPath $src -Destination $dst -Recurse -Force
    return $true
  } catch { return $false }
}
function Zip-To([string[]]$paths, [string]$dst) {
  $have = @($paths | Where-Object { $_ -and (Test-Path -LiteralPath $_) })
  if ($have.Count -eq 0) { return $false }
  try { Compress-Archive -Force -Path $have -DestinationPath $dst; return $true } catch { Warn ('zip failed: ' + $_.Exception.Message); return $false }
}
function Cmd-Finish([string]$P) {
  $info = Get-Info $P
  $N = Get-Next $P
  if (-not $N.Contains('wave')) { Fail 'no .bro/inbox/NEXT.md' }
  $W = $N['wave']; $prev = '-'
  if ($N.Contains('prev') -and $N['prev']) { $prev = $N['prev'] }
  $wd = Get-WaveDir $P $W
  $tp = J $wd '01-TASK.md'; $rp = J $wd '02-REPORT.md'
  if (-not (Test-Path -LiteralPath $rp)) { Fail ('REPORT missing: ' + $rp) }
  $T = Get-Passport $tp; $B = Get-Passport (J $wd 'logs/begin.md')
  $isGit = Is-Git $P
  $v = [ordered]@{}
  $v['t_start'] = '-'; if ($B.Contains('t_start')) { $v['t_start'] = $B['t_start'] }
  $v['t_report'] = Now
  $v['tasks'] = '-'; if ($T.Contains('tasks')) { $v['tasks'] = $T['tasks'] }
  $rt = Read-Text $rp
  $done = 0
  $sec = [regex]::Match($rt, '(?ms)^## Tasks[ \t]*\r?$(.*?)(?=^## |\z)')
  if ($sec.Success) { foreach ($l in ($sec.Groups[1].Value -split "`r?`n")) { if ($l -match '^\s*\d+' -and $l.Contains($OK)) { $done++ } } }
  $v['done'] = $done
  $body = $rt
  $fm = [regex]::Match($rt, '(?s)^---.*?\n---')
  if ($fm.Success) { $body = $rt.Substring($fm.Length) }
  $qs = @{}
  foreach ($m in [regex]::Matches($body, 'Q-\d+')) { $qs[$m.Value] = 1 }
  $v['q'] = $qs.Count
  $pr = Get-Proof $P $wd
  $v['proof_req'] = $pr['req']; $v['proof_ok'] = $pr['ok']
  $added = ''
  if ($isGit) {
    $d = Get-Diff $P (Get-Base $P $W) (Get-Allow $T)
    $v['diff_files'] = $d['files']; $v['diff_add'] = $d['add']; $v['diff_del'] = $d['del']; $v['out_of_scope'] = $d['oos']
    foreach ($f in $d['oosList']) { Say ('out_of_scope: ' + $f) }
    $added = $d['addText']
    $v['commit'] = Git1 $P @('rev-parse', '--short', 'HEAD')
    $tag = $W.ToLower() + '-done'
    if ((Git1 $P @('tag', '-l', $tag)) -eq $tag) { $v['tag'] = $tag } else { $v['tag'] = $NO }
    $h = Git1 $P @('rev-parse', 'HEAD')
    $u = Git1 $P @('rev-parse', '@{u}')
    if ($script:GitCode -eq 0 -and $u -and $u -eq $h) { $v['push'] = $OK } else { $v['push'] = $NO }
  } else { foreach ($k in @('commit', 'tag', 'push', 'diff_files', 'diff_add', 'diff_del', 'out_of_scope')) { $v[$k] = '-' } }
  $boot = J $SYS 'core/CODER-BOOT.md'
  $inList = @($boot, (J $P '.bro/inbox/NEXT.md'), $tp, (J $SYS 'templates/02-REPORT.md'))
  if ($prev -ne '-') { $inList += (J (Get-WaveDir $P $prev) '03-ACCEPT.md') }
  if ($T.Contains('modules')) { foreach ($m in ([string]$T['modules']).Split(',')) { $mm = $m.Trim(); if ($mm -and $mm -ne '-') { $inList += (J $SYS ('modules/' + $mm + '.md')) } } }
  $v['cd_in'] = Ptok-List $P $inList
  $build = 'JOIN'; if ($T.Contains('build')) { $build = $T['build'] }
  $v['prompt_weight'] = (Get-FilePtok (J $SYS ('builds/START-' + $build + '.md'))) + (Get-FilePtok $boot)
  $v['an_in_prev'] = '-'; $v['an_out_prev'] = '-'
  foreach ($r in @(Read-Csv (J $P '.bro/metrics/WAVES.csv'))) { if ($r.wave -eq $prev) { $v['an_in_prev'] = $r.an_in; $v['an_out_prev'] = $r.an_out } }
  $v['cd_out'] = (Get-Ptok $added) + (Get-FilePtok $rp) + (Get-FilePtok (J $wd 'proof/INDEX.md'))
  $mir = ''; if ($T.Contains('mirror')) { $mir = [string]$T['mirror'] }
  $v['bundle'] = '-'; $v['exchange'] = '-'; $v['pack'] = '-'
  if ($mir -and $mir -ne '-') {
    $mir = Full $mir
    Ensure-Dir $mir
    if ($isGit) {
      Invoke-Git $P @('bundle', 'create', (J $mir ('repo-' + $W + '.bundle')), '--all') | Out-Null
      if ($script:GitCode -eq 0) { $v['bundle'] = $OK } else { $v['bundle'] = $NO }
      @(Get-ChildItem -LiteralPath $mir -Filter 'repo-*.bundle' | Sort-Object LastWriteTime -Descending | Select-Object -Skip 5) | ForEach-Object { Remove-Item -LiteralPath $_.FullName -Force }
    }
    $v['exchange'] = $OK; $v['pack'] = $OK
    Set-Passport $rp $v
    if (-not (Copy-Bro $P $mir)) { $v['exchange'] = $NO }
    $bro = J $P '.bro'
    if (-not (Zip-To @((J $bro 'MAP.md'), (J $bro 'LOG.md'), (J $bro 'SPEC.md'), (J $bro 'metrics'), $wd) (J $mir 'BRO-PACK.zip'))) { $v['pack'] = $NO }
  }
  Set-Passport $rp $v
  if ($mir -and $mir -ne '-') {
    $rel = $rp.Substring((J $P '.bro').Length).TrimStart('\', '/')
    try { Copy-Item -LiteralPath $rp -Destination (J (J $mir '.bro') $rel) -Force } catch { Warn 'mirror copy of REPORT failed' }
  }
  if ($info['mode'] -eq 'LOCAL') {
    $bro = J $P '.bro'
    $z = J $bro ('archive/' + $W + '.zip')
    Ensure-Dir (J $bro 'archive')
    if (Zip-To @($wd, (J $bro 'MAP.md'), (J $bro 'LOG.md'), (J $bro 'metrics')) $z) { Say ('archive: ' + $z) } else { Warn 'archive zip failed' }
  }
  $errs = Cmd-Check $P $W
  Cmd-State $P 'analyst'
  if ($errs -gt 0) { Say 'finish: fix the errors above, then run finish again'; exit 1 }
  $fmt = [regex]::Match((Read-Text $rp), '(?s)^---.*?\n---')
  Say $fmt.Value
  Say 'finish: OK'
}
function Cmd-Added([string]$P, [string]$Base) {
  if (-not (Is-Git $P)) { Say 'added: no git -> diff keys = -'; return }
  $W = (Get-Next $P)['wave']
  $T = Get-Passport (J (Get-WaveDir $P $W) '01-TASK.md')
  if (-not $Base) { $Base = Get-Base $P $W }
  $d = Get-Diff $P $Base (Get-Allow $T)
  Say ('base ' + $Base + ': diff_files: ' + $d['files'] + $SEP + 'diff_add: ' + $d['add'] + $SEP + 'diff_del: ' + $d['del'] + $SEP + 'out_of_scope: ' + $d['oos'] + $SEP + 'added_ptok: ' + (Get-Ptok $d['addText']))
  foreach ($f in $d['oosList']) { Say ('out_of_scope: ' + $f) }
}
function Cmd-Ptok([string[]]$paths) {
  $tot = 0
  foreach ($p in $paths) {
    $files = @($p)
    if (Test-Path -LiteralPath $p -PathType Container) { $files = @(Get-ChildItem -LiteralPath $p -Recurse -File | ForEach-Object { $_.FullName }) }
    foreach ($f in $files) { $n = Get-FilePtok $f; $tot += $n; Say (('{0,7}' -f $n) + '  ' + $f) }
  }
  Say (('{0,7}' -f $tot) + '  total')
}
function Cmd-Summary([string]$Target) {
  $files = @(); $out = ''; $title = ''
  if ($Target -eq 'ALL') {
    $files = @(Get-ChildItem -LiteralPath (J $SYS 'metrics/projects') -Filter '*.csv' | ForEach-Object { $_.FullName })
    $out = J $SYS 'metrics/SUMMARY.md'; $title = 'all projects (' + $files.Count + ')'
  } else {
    $files = @((J $Target '.bro/metrics/WAVES.csv'))
    $out = J $Target '.bro/metrics/SUMMARY.md'; $title = (Get-Info $Target)['project']
  }
  $rows = New-Object System.Collections.ArrayList
  foreach ($f in $files) { foreach ($r in @(Read-Csv $f)) { [void]$rows.Add($r) } }
  $vers = @($rows | ForEach-Object { $_.ver } | Where-Object { $_ } | Sort-Object -Unique)
  $ind = [ordered]@{}
  $labels = [ordered]@{
    'n' = '0 | waves (verdict needs >=10) | >=10'
    'i1' = '1 | accepted first time: PASS / waves | grows'
    'i2' = '2 | false done, sum | 0'
    'i3' = '3 | Owner touches per wave | <=2 + stops'
    'i4' = '4 | Owner fixes: sum / per wave | falls'
    'i5' = '5 | minutes per wave: work / wait | falls'
    'i6' = '6 | ptok per accepted task / GitHub calls per wave | falls / <=5'
    'i7' = '7 | prompt-caused defects share / avg K (0-4) | falls / grows'
    'w' = '- | prompt weight START+BOOT, last | stable'
    'o' = '- | avg O (0-4) | grows'
  }
  foreach ($k in $labels.Keys) { $ind[$k] = @() }
  $weak = @{}; $fric = @{}
  foreach ($ver in $vers) {
    $rs = @($rows | Where-Object { $_.ver -eq $ver })
    $n = $rs.Count
    $pass = @($rs | Where-Object { $_.verdict -eq 'PASS' }).Count
    $fd = 0; $fix = 0; $tch = @(); $work = @(); $wait = @(); $gh = @(); $ks = @(); $os = @(); $tok = 0; $acc = 0; $pdef = 0; $def = 0; $wlast = '-'
    foreach ($r in $rs) {
      $x = Num $r.false_done; if ($null -ne $x) { $fd += $x }
      $x = Num $r.owner_fixes; if ($null -ne $x) { $fix += $x }
      $tch += (Num $r.owner_touches); $gh += (Num $r.gh_calls); $ks += (Num $r.k); $os += (Num $r.o)
      $work += (Get-Min $r.t_start $r.t_report)
      $w1 = Get-Min $r.t_issued $r.t_start; $w2 = Get-Min $r.t_report $r.t_accept
      if ($null -ne $w1 -and $null -ne $w2) { $wait += ($w1 + $w2) }
      foreach ($c in @('an_in', 'an_out', 'cd_in', 'cd_out')) { $x = Num $r.$c; if ($null -ne $x) { $tok += $x } }
      if ($r.verdict -eq 'PASS' -or $r.verdict -eq 'PARTIAL') { $x = Num $r.done; if ($null -ne $x) { $acc += $x } }
      $x = Num $r.defects; if ($null -ne $x) { $def += $x }
      foreach ($c in ([string]$r.causes).Split(';')) {
        $cc = $c.Trim()
        if ($cc -match '^P@(.+)$') { $pdef++; $a = $Matches[1]; if ($weak.ContainsKey($a)) { $weak[$a]++ } else { $weak[$a] = 1 } }
      }
      foreach ($c in @([string]$r.friction_cd, [string]$r.friction_an)) {
        foreach ($tg in [regex]::Matches($c, '#[\w-]+')) { $t = $tg.Value; if ($fric.ContainsKey($t)) { $fric[$t]++ } else { $fric[$t] = 1 } }
      }
      if ($r.prompt_weight) { $wlast = $r.prompt_weight }
    }
    $pct = 0; if ($n -gt 0) { $pct = [math]::Round(100.0 * $pass / $n) }
    $ppt = '-'; if ($acc -gt 0) { $ppt = [math]::Round($tok / $acc) }
    $psh = '-'; if ($def -gt 0) { $psh = [math]::Round($pdef / $def, 2) } elseif ($n -gt 0) { $psh = 0 }
    $fpw = '-'; if ($n -gt 0) { $fpw = [math]::Round($fix / $n, 2) }
    $ind['n'] += [string]$n
    $ind['i1'] += ([string]$pass + '/' + $n + ' (' + $pct + '%)')
    $ind['i2'] += [string]$fd
    $ind['i3'] += [string](Avg $tch)
    $ind['i4'] += ([string]$fix + ' / ' + $fpw)
    $ind['i5'] += ([string](Avg $work) + ' / ' + (Avg $wait))
    $ind['i6'] += ([string]$ppt + ' / ' + (Avg $gh))
    $ind['i7'] += ([string]$psh + ' / ' + (Avg $ks))
    $ind['w'] += [string]$wlast
    $ind['o'] += [string](Avg $os)
  }
  $sb = New-Object System.Text.StringBuilder
  [void]$sb.Append('# SUMMARY ' + $title + $SEP + 'updated ' + (Now) + $SEP + 'by tools/bro.ps1 summary').Append("`n")
  [void]$sb.Append('| # | indicator | good if | ' + [string]::Join(' | ', $vers) + ' |').Append("`n")
  $dash = '|---|---|---|'; foreach ($x in $vers) { $dash += '---|' }
  [void]$sb.Append($dash).Append("`n")
  foreach ($k in $labels.Keys) { [void]$sb.Append('| ' + $labels[$k] + ' | ' + [string]::Join(' | ', $ind[$k]) + ' |').Append("`n") }
  [void]$sb.Append("`nTop weak rules (P causes): ")
  $top = @($weak.GetEnumerator() | Sort-Object -Property Value -Descending | Select-Object -First 3 | ForEach-Object { $_.Key + ' x' + $_.Value })
  if ($top.Count -gt 0) { [void]$sb.Append([string]::Join(', ', $top)) } else { [void]$sb.Append('-') }
  [void]$sb.Append("`nFriction tags: ")
  $ft = @($fric.GetEnumerator() | Sort-Object -Property Value -Descending | ForEach-Object { $_.Key + ' x' + $_.Value })
  if ($ft.Count -gt 0) { [void]$sb.Append([string]::Join(', ', $ft)) } else { [void]$sb.Append('-') }
  [void]$sb.Append("`nRule: change 1-3 things per version; call a version better only after >=10 waves.`n")
  Write-Text $out $sb.ToString()
  Say ('summary: ' + $out + ' (' + $rows.Count + ' waves, versions: ' + [string]::Join(', ', $vers) + ')')
}
function Get-I18n {
  $d = @{}
  $t = Read-Text (J $SYS 'templates/STATUS.md')
  $m = [regex]::Match([string]$t, '(?s)<!-- bro-i18n\s*(.*?)-->')
  if ($m.Success) { foreach ($l in ($m.Groups[1].Value -split "`r?`n")) { $i = $l.IndexOf('='); if ($i -gt 0) { $d[$l.Substring(0, $i).Trim()] = $l.Substring($i + 1).Trim() } } }
  return $d
}
function Parse-Road([string]$s) {
  $list = New-Object System.Collections.ArrayList
  if ($s) {
    foreach ($seg in $s.Split(';')) {
      $x = $seg.Trim()
      $m = [regex]::Match($x, '^(?<id>[^=]+)=(?<st>[a-z]+)(?<stop>!?)(:(?<title>.*))?$')
      if (-not $m.Success) { continue }
      $title = $m.Groups['title'].Value.Trim()
      if (-not $title) { $title = $m.Groups['id'].Value.Trim() }
      [void]$list.Add([ordered]@{ id = $m.Groups['id'].Value.Trim(); title = $title; status = $m.Groups['st'].Value; stop = ($m.Groups['stop'].Value -eq '!') })
    }
  }
  return ,$list
}
function Cmd-State([string]$P, [string]$Turn) {
  if (-not $Turn) { $Turn = 'analyst' }
  $i18n = Get-I18n
  $info = Get-Info $P
  $N = Get-Next $P
  $W = ''; if ($N.Contains('wave')) { $W = $N['wave'] }
  $T = [ordered]@{}; if ($W) { $T = Get-Passport (J (Get-WaveDir $P $W) '01-TASK.md') }
  $road = Parse-Road ([string]$T['road'])
  $rows = @(Read-Csv (J $P '.bro/metrics/WAVES.csv'))
  if ($road.Count -eq 0) {
    foreach ($r in $rows) { [void]$road.Add([ordered]@{ id = $r.wave; title = $r.wave; status = 'done'; stop = $false }) }
    if ($W) { [void]$road.Add([ordered]@{ id = $W; title = $W; status = 'run'; stop = $false }) }
  }
  $total = $road.Count; $done = 0; $ci = -1
  for ($i = 0; $i -lt $total; $i++) { if ($road[$i]['status'] -eq 'done') { $done++ }; if ($road[$i]['id'] -eq $W) { $ci = $i } }
  $pct = 0; $filled = 0
  if ($total -gt 0) { $pct = [int][math]::Round(100.0 * $done / $total); $filled = [int][math]::Round(8.0 * $done / $total, [System.MidpointRounding]::AwayFromZero) }
  $bar = ([string][char]0x25B0) * $filled + ([string][char]0x25B1) * (8 - $filled)
  $cur = $W; if ($ci -ge 0) { $cur = $road[$ci]['title'] }
  $stop = [string]$i18n['stop.none']
  for ($i = [math]::Max(0, $ci); $i -lt $total; $i++) {
    if ($road[$i]['stop'] -and $road[$i]['status'] -ne 'done') {
      $dist = $i - [math]::Max(0, $ci)
      if ($dist -le 0) { $stop = ([string]$i18n['stop.now']).Replace('{title}', $road[$i]['title']) }
      else { $stop = ([string]$i18n['stop.in']).Replace('{title}', $road[$i]['title']).Replace('{n}', [string]$dist) }
      break
    }
  }
  $pass = @($rows | Where-Object { $_.verdict -eq 'PASS' }).Count
  $fd = 0; $tok = 0; $acc = 0; $gh = @(); $ks = @(); $os = @()
  $waves = New-Object System.Collections.ArrayList
  foreach ($r in $rows) {
    $x = Num $r.false_done; if ($null -ne $x) { $fd += $x }
    foreach ($c in @('an_in', 'an_out', 'cd_in', 'cd_out')) { $x = Num $r.$c; if ($null -ne $x) { $tok += $x } }
    if ($r.verdict -eq 'PASS' -or $r.verdict -eq 'PARTIAL') { $x = Num $r.done; if ($null -ne $x) { $acc += $x } }
    $gh += (Num $r.gh_calls); $ks += (Num $r.k); $os += (Num $r.o)
    [void]$waves.Add([ordered]@{ wave = $r.wave; verdict = $r.verdict; minutes = (Get-Min $r.t_start $r.t_report); tasks = $r.tasks; done = $r.done; defects = $r.defects; gh = $r.gh_calls })
  }
  $last = $null; if ($rows.Count -gt 0) { $last = $rows[$rows.Count - 1] }
  $mins = '-'; $wait = '-'
  if ($last) {
    $mm = Get-Min $last.t_start $last.t_report; if ($null -ne $mm) { $mins = $mm }
    $w1 = Get-Min $last.t_issued $last.t_start; $w2 = Get-Min $last.t_report $last.t_accept
    if ($null -ne $w1 -and $null -ne $w2) { $wait = $w1 + $w2 }
  }
  $ppt = '-'; if ($acc -gt 0) { $ppt = [math]::Round($tok / $acc) }
  $fp = '-'; if ($rows.Count -gt 0) { $fp = [string]$pass + '/' + $rows.Count }
  $metrics = [ordered]@{ waves = $rows.Count; first_pass = $fp; false_done = $fd; minutes = $mins; wait = $wait; ptok_task = $ppt; gh = (Avg $gh); k = (Avg $ks); o = (Avg $os) }
  $state = [ordered]@{
    project = $info['project']; ver = $info['bro']; mode = $info['mode']; updated = (Now)
    wave = $W; current = $cur; turn = $Turn; turn_text = [string]$i18n['turn.' + $Turn]; todo = [string]$i18n['todo.' + $Turn]; next_stop = $stop
    done = $done; total = $total; pct = $pct; bar = $bar
    road = $road.ToArray(); metrics = $metrics; waves = $waves.ToArray()
  }
  $json = ConvertTo-Json -InputObject $state -Depth 6
  Write-Text (J $P '.bro/state.js') ('window.BRO_STATE = ' + $json + ";`n")
  $mer = New-Object System.Text.StringBuilder
  [void]$mer.Append('flowchart LR').Append("`n")
  for ($i = 0; $i -lt $total; $i++) {
    $it = $road[$i]
    $lab = [string]$i18n['st.' + $it['status']] + ' ' + ([string]$it['title']).Replace('"', "'")
    if ($it['stop']) { $lab = $lab + ' ' + [string]$i18n['st.wait'] }
    [void]$mer.Append('  n' + $i + '["' + $lab + '"]:::' + $it['status']).Append("`n")
    if ($i -gt 0) { [void]$mer.Append('  n' + ($i - 1) + ' --> n' + $i).Append("`n") }
  }
  [void]$mer.Append('  classDef done fill:#DCFCE7,stroke:#16A34A,color:#16181D').Append("`n")
  [void]$mer.Append('  classDef run fill:#E0E7FF,stroke:#4F46E5,color:#16181D').Append("`n")
  [void]$mer.Append('  classDef wait fill:#FEF3C7,stroke:#D97706,color:#16181D').Append("`n")
  [void]$mer.Append('  classDef todo fill:#FFFFFF,stroke:#E6E7EB,color:#5B6170').Append("`n")
  [void]$mer.Append('  classDef pause fill:#F3F4F6,stroke:#9CA3AF,color:#5B6170').Append("`n")
  [void]$mer.Append('  classDef frozen fill:#F3F4F6,stroke:#9CA3AF,color:#5B6170')
  $tb = New-Object System.Text.StringBuilder
  [void]$tb.Append('| W | verdict | done/tasks | defects | min | GitHub |').Append("`n").Append('|---|---|---|---|---|---|').Append("`n")
  foreach ($wv in $waves) { [void]$tb.Append('| ' + $wv['wave'] + ' | ' + $wv['verdict'] + ' | ' + $wv['done'] + '/' + $wv['tasks'] + ' | ' + $wv['defects'] + ' | ' + $wv['minutes'] + ' | ' + $wv['gh'] + ' |').Append("`n") }
  $tpl = [regex]::Replace([string](Read-Text (J $SYS 'templates/STATUS.md')), '(?s)<!-- bro-i18n.*?-->\s*', '')
  $map = [ordered]@{
    '{{PROJECT}}' = $info['project']; '{{VER}}' = $info['bro']; '{{MODE}}' = $info['mode']; '{{UPDATED}}' = (Now); '{{WAVE}}' = $W
    '{{BAR}}' = $bar; '{{DONE}}' = $done; '{{TOTAL}}' = $total; '{{PCT}}' = $pct; '{{CURRENT}}' = $cur
    '{{TURN}}' = $state['turn_text']; '{{TODO}}' = $state['todo']; '{{NEXT_STOP}}' = $stop; '{{MERMAID}}' = $mer.ToString()
    '{{FIRST_PASS}}' = $fp; '{{FALSE_DONE}}' = $fd; '{{MINUTES}}' = $mins; '{{WAIT}}' = $wait; '{{PTOK_TASK}}' = $ppt; '{{GH}}' = $metrics['gh']
    '{{WAVES_N}}' = $rows.Count; '{{WAVES_TABLE}}' = $tb.ToString().TrimEnd()
  }
  foreach ($k in $map.Keys) { $tpl = $tpl.Replace($k, [string]$map[$k]) }
  Write-Text (J $P '.bro/STATUS.md') $tpl
  Say ('state: ' + $W + ' ' + $done + '/' + $total + ' ' + $bar + ' turn ' + $Turn)
}
function Build-Head([string]$ver, [string]$m, [int]$w) {
  return '<!-- BRO ' + $ver + $SEP + 'START-' + $m + $SEP + 'weight ' + [string][char]0x2248 + ' ' + $w + ' ptok' + $SEP + 'generated by tools/bro.ps1 build: edit core/ modes/ templates/, not this file -->'
}
function Cmd-Build {
  $ver = (Read-Text (J $SYS 'VERSION')).Trim()
  $fence = '````'
  foreach ($m in @('NEW', 'JOIN', 'LOCAL')) {
    $sb = New-Object System.Text.StringBuilder
    [void]$sb.Append((Read-Text (J $SYS 'core/ANALYST-CORE.md')).TrimEnd()).Append("`n`n")
    [void]$sb.Append((Read-Text (J $SYS ('modes/' + $m + '.md'))).TrimEnd()).Append("`n`n")
    [void]$sb.Append('# Templates (you write 01-TASK, 03-ACCEPT, NEXT; coder fills 02-REPORT)').Append("`n")
    foreach ($t in @('01-TASK.md', '03-ACCEPT.md', 'NEXT.md')) {
      [void]$sb.Append('## ' + $t).Append("`n").Append($fence + 'markdown').Append("`n")
      [void]$sb.Append((Read-Text (J $SYS ('templates/' + $t))).TrimEnd()).Append("`n").Append($fence).Append("`n")
    }
    $body = $sb.ToString()
    $w = Get-Ptok ((Build-Head $ver $m 9999) + "`n" + $body)
    Write-Text (J $SYS ('builds/START-' + $m + '.md')) ((Build-Head $ver $m $w) + "`n" + $body)
  }
  $list = @()
  foreach ($m in @('NEW', 'JOIN', 'LOCAL')) { $list += (J $SYS ('builds/START-' + $m + '.md')) }
  $list += @((J $SYS 'core/CODER-BOOT.md'), (J $SYS 'core/RAKES.md'), (J $SYS 'auditor/AUDITOR.md'))
  $list += @(Get-ChildItem -LiteralPath (J $SYS 'modules') -Filter '*.md' | ForEach-Object { $_.FullName })
  Say ('build ' + $ver + ' (targets: START <= 4500, CODER-BOOT <= 600)')
  Cmd-Ptok $list
}

function Arg([int]$i) { if ($Rest -and $Rest.Count -gt $i) { return $Rest[$i] }; return '' }
function ProjArg {
  $p = Arg 0
  if (-not $p) { Fail 'project path required' }
  $f = (Full $p).TrimEnd('\', '/')
  if (-not (Test-Path -LiteralPath $f)) { Fail ('project not found: ' + $f) }
  return $f
}
switch ($Cmd.ToLower()) {
  'init'    { Cmd-Init (ProjArg) (Arg 1) (Arg 2) (Arg 3) }
  'begin'   { Cmd-Begin (ProjArg) }
  'finish'  { Cmd-Finish (ProjArg) }
  'check'   { $n = Cmd-Check (ProjArg) (Arg 1); if ($n -gt 0) { exit 1 } }
  'ptok'    { Cmd-Ptok $Rest }
  'added'   { Cmd-Added (ProjArg) (Arg 1) }
  'row'     { Cmd-Row (ProjArg) (Arg 1) (Arg 2) }
  'summary' { $t = Arg 0; if ($t -ne 'ALL') { $t = ProjArg }; Cmd-Summary $t }
  'state'   { Cmd-State (ProjArg) (Arg 1) }
  'build'   { Cmd-Build }
  default   { Get-Content -LiteralPath $PSCommandPath -TotalCount 15 | Select-Object -Skip 1 | ForEach-Object { Write-Host $_ } }
}
exit 0
