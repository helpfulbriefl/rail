# Enum top-level windows for the rail pid + cursor position.
# Modeled on the proven enum-rail-windows2.ps1 (single-quoted here-string Add-Type).
# Auto-finds the rail pid (no hardcode), writes each window line via Write-Output.
$ErrorActionPreference = 'Stop'
Add-Type @'
using System;
using System.Runtime.InteropServices;
using System.Text;
public class T3 {
  [StructLayout(LayoutKind.Sequential)] public struct R { public int L, T, Rr, B; }
  [StructLayout(LayoutKind.Sequential)] public struct P { public int X, Y; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out R r);
  [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder s, int n);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc cb, IntPtr d);
  [DllImport("user32.dll")] public static extern bool GetCursorPos(out P p);
  public delegate bool EnumProc(IntPtr h, IntPtr d);
}
'@

# Find the rail pid
$proc = Get-CimInstance Win32_Process -Filter "Name='rail.exe'" | Select-Object -First 1
$rp = [uint32]$proc.ProcessId
Write-Output ("rail pid=" + $rp)

# Enumerate the rail's top-level windows
$railwins = @()
$cb = [T3+EnumProc]{
  param($h, $d)
  $p2 = 0
  [void][T3]::GetWindowThreadProcessId($h, [ref]$p2)
  if ($p2 -eq $script:rp) {
    $s2 = New-Object System.Text.StringBuilder 128
    [void][T3]::GetWindowTextW($h, $s2, 128)
    $r = New-Object T3+R
    [void][T3]::GetWindowRect($h, [ref]$r)
    $script:railwins += "hwnd=0x" + $h.ToString('X') + " vis=" + [T3]::IsWindowVisible($h) + " rect=" + $r.L + "," + $r.T + "," + $r.Rr + "," + $r.B + " title=[" + $s2.ToString() + "]"
  }
  return $true
}
$script:rp = $rp
[void][T3]::EnumWindows($cb, [IntPtr]::Zero)
Write-Output ("rail top-levels: " + $railwins.Count)
foreach ($w in $railwins) { Write-Output ("  " + $w) }

# Cursor position
$cp = New-Object T3+P
[void][T3]::GetCursorPos([ref]$cp)
Write-Output ("cursor=" + $cp.X + "," + $cp.Y)
Write-Output "DONE3"
