/** Dominant colour of an icon (for the Aurora glow). Cached per src. */
const cache = new Map<string, string>();
export function iconColor(src: string): Promise<string> {
  const hit = cache.get(src);
  if (hit) return Promise.resolve(hit);
  return new Promise((res) => {
    const img = new Image();
    img.onload = () => {
      try {
        const c = document.createElement('canvas');
        c.width = c.height = 16;
        const g = c.getContext('2d', { willReadFrequently: true });
        if (!g) return res('#6366f1');
        g.drawImage(img, 0, 0, 16, 16);
        const d = g.getImageData(0, 0, 16, 16).data;
        let r = 0, gg = 0, b = 0, n = 0;
        for (let i = 0; i < d.length; i += 4) {
          const a = d[i + 3];
          const mx = Math.max(d[i], d[i + 1], d[i + 2]);
          const mn = Math.min(d[i], d[i + 1], d[i + 2]);
          if (a < 200 || mx - mn < 30) continue; // skip transparent & greys
          r += d[i]; gg += d[i + 1]; b += d[i + 2]; n++;
        }
        const out = n ? `rgb(${(r / n) | 0} ${(gg / n) | 0} ${(b / n) | 0})` : '#6366f1';
        cache.set(src, out);
        res(out);
      } catch {
        res('#6366f1');
      }
    };
    img.onerror = () => res('#6366f1');
    img.src = src;
  });
}
