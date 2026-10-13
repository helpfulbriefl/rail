export function bytes(n: number): { v: string; u: string } {
  if (n >= 1e9) return { v: (n / 1e9).toFixed(n >= 1e10 ? 1 : 2), u: 'GB' };
  if (n >= 1e6) return { v: (n / 1e6).toFixed(n >= 1e8 ? 0 : 1), u: 'MB' };
  if (n >= 1e3) return { v: (n / 1e3).toFixed(0), u: 'KB' };
  return { v: n.toFixed(0), u: 'B' };
}
export function rate(n: number): string {
  const b = bytes(n);
  return `${b.v} ${b.u}/s`;
}
export function ago(ts: number): { v: string; u: string } {
  if (!ts) return { v: '—', u: '' };
  const s = Math.max(0, (Date.now() - ts) / 1000);
  if (s < 60) return { v: String(Math.floor(s)), u: 'с' };
  if (s < 3600) return { v: String(Math.floor(s / 60)), u: 'мин' };
  if (s < 86400) return { v: String(Math.floor(s / 3600)), u: 'ч' };
  return { v: String(Math.floor(s / 86400)), u: 'д' };
}
export function uptime(sec: number): string {
  const h = Math.floor(sec / 3600);
  const m = Math.floor((sec % 3600) / 60);
  const s = Math.floor(sec % 60);
  const p = (x: number) => String(x).padStart(2, '0');
  return h > 0 ? `${h}:${p(m)}:${p(s)}` : `${p(m)}:${p(s)}`;
}
/** Keep both ends of a long path: C:\Program Files\…\app.exe */
export function midPath(p: string, max = 46): string {
  if (p.length <= max) return p;
  const tail = p.slice(p.lastIndexOf('\\'));
  const headLen = Math.max(8, max - tail.length - 1);
  return p.slice(0, headLen) + '…' + tail;
}
