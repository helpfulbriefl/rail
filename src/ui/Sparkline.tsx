import { useId } from 'react';

export function Sparkline({ data, max, color = 'var(--accent)', w = 112, h = 26 }: { data: number[]; max?: number; color?: string; w?: number; h?: number }) {
  const gid = useId().replace(/:/g, '');
  const n = 40;
  const pts = data.slice(-n);
  const top = Math.max(max ?? 0, ...pts, 1e-6);
  const step = w / (n - 1);
  const off = (n - pts.length) * step;
  const xy = pts.map((v, i) => [off + i * step, h - 2 - (v / top) * (h - 4)] as const);
  const line = xy.map(([x, y], i) => `${i ? 'L' : 'M'}${x.toFixed(1)},${y.toFixed(1)}`).join('');
  const area = xy.length ? `${line}L${xy[xy.length - 1][0].toFixed(1)},${h}L${xy[0][0].toFixed(1)},${h}Z` : '';
  return (
    <svg className="spark" width={w} height={h} viewBox={`0 0 ${w} ${h}`} aria-hidden>
      <defs>
        <linearGradient id={gid} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0" stopColor={color} stopOpacity="0.28" />
          <stop offset="1" stopColor={color} stopOpacity="0" />
        </linearGradient>
      </defs>
      {area && <path d={area} fill={`url(#${gid})`} />}
      {/* Plain path, not motion: `pathLength` is applied through stroke-dasharray
          derived from the path's total length, so every new `d` re-derived the dash
          values and the line visibly re-drew itself on each metrics frame. */}
      {line && <path d={line} fill="none" stroke={color} strokeWidth={1.6} strokeLinejoin="round" strokeLinecap="round" />}
      {xy.length > 0 && <circle cx={xy[xy.length - 1][0]} cy={xy[xy.length - 1][1]} r={2.2} fill={color} />}
    </svg>
  );
}
