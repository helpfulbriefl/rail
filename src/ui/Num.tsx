import { animate, useMotionValue, useTransform, motion } from 'motion/react';
import { useEffect, useRef } from 'react';

/**
 * Number that glides to its new value (tabular mono digits).
 *
 * Why this is not a plain `animate()` any more:
 *
 *  - The spring emitted a new string on *every frame*, so a value drifting by
 *    0.4 repainted the text ~30×/s. With proportional fallback fonts (Geist Mono
 *    is not always installed) each repaint changed the measured width and the
 *    whole meter row jittered — the "twitching resource values".
 *  - Small deltas are now snapped instead of animated: below `EPS` the number is
 *    simply not worth a 500 ms tween.
 *  - The animation is skipped entirely when the rendered text would not change,
 *    which is the common case for an idle machine.
 */
const EPS = 0.35;

export function Num({ value, digits = 0, className }: { value: number; digits?: number; className?: string }) {
  const safe = Number.isFinite(value) ? value : 0;
  const mv = useMotionValue(safe);
  // Pad to a fixed width so 9 → 10 → 9 does not resize the element.
  const text = useTransform(mv, (v) => {
    const s = (Number.isFinite(v) ? v : 0).toFixed(digits);
    return s;
  });
  const prev = useRef(safe);
  useEffect(() => {
    const from = prev.current;
    prev.current = safe;
    // identical rendering → nothing to do, keep the DOM untouched
    if (from.toFixed(digits) === safe.toFixed(digits)) {
      mv.set(safe);
      return;
    }
    if (Math.abs(safe - from) < EPS) {
      mv.set(safe);
      return;
    }
    const c = animate(mv, safe, { duration: 0.45, ease: [0.22, 1, 0.36, 1] });
    return () => c.stop();
  }, [safe, digits, mv]);
  return <motion.span className={className}>{text}</motion.span>;
}
