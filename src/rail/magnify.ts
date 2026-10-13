import { createContext, useContext } from 'react';
import type { MotionValue } from 'motion/react';

export interface MagReg {
  scale: MotionValue<number>;
  shift: MotionValue<number>;
  el: HTMLElement | null;
  max: number;
}

export interface MagCtx {
  register(id: string, r: MagReg): () => void;
}

export const MagContext = createContext<MagCtx | null>(null);
export const useMag = () => useContext(MagContext)!;

/** Gaussian dock magnification with neighbour push (macOS-like), computed for the whole row at once. */
export function computeMagnify(
  order: string[],
  regs: Map<string, MagReg>,
  pointer: number,
  active: number,
  vertical: boolean,
  sigma: number,
): number {
  const g: number[] = [];
  const sc: number[] = [];
  const ids = order.filter((id) => regs.has(id));
  for (const id of ids) {
    const r = regs.get(id)!;
    const el = r.el;
    if (!el) {
      g.push(0);
      sc.push(1);
      continue;
    }
    const size = vertical ? el.offsetHeight : el.offsetWidth;
    const c = (vertical ? el.offsetTop : el.offsetLeft) + size / 2;
    const d = pointer - c;
    const k = Math.exp(-(d * d) / (2 * sigma * sigma));
    const s = 1 + (r.max - 1) * k * active;
    sc.push(s);
    g.push((s - 1) * size);
  }
  const total = g.reduce((a, b) => a + b, 0);
  let before = 0;
  ids.forEach((id, i) => {
    const r = regs.get(id)!;
    const after = total - before - g[i];
    r.scale.set(sc[i]);
    r.shift.set((before - after) / 2);
    before += g[i];
  });
  return total / 2;
}
