import { motion } from 'motion/react';
import type { LampState } from './status';

const color: Record<LampState, string> = {
  off: 'var(--lamp-idle)',
  launch: 'var(--lamp-load)',
  run: 'var(--lamp-run)',
  load: 'var(--lamp-load)',
  hung: 'var(--lamp-hung)',
  io: 'var(--lamp-io)',
  mem: 'var(--lamp-mem)',
};

/** Single status lamp. `breathe` = the "light bulb" animation for a single window. */
export function Lamp({ state, breathe = false, size = 7 }: { state: LampState; breathe?: boolean; size?: number }) {
  const c = color[state];
  const blink = state === 'launch' || state === 'hung';
  return (
    <span className="lamp" style={{ width: size, height: size }} data-state={state}>
      <motion.span
        className="lamp-core"
        animate={{
          backgroundColor: c,
          boxShadow: `0 0 0 0px ${c}, 0 0 ${size + 2}px ${c}`,
          opacity: blink ? [1, 0.35, 1] : 1,
          scale: breathe && !blink ? [1, 1.18, 1] : 1,
        }}
        transition={{
          backgroundColor: { duration: 0.18 },
          boxShadow: { duration: 0.18 },
          opacity: blink ? { duration: state === 'hung' ? 0.7 : 1.1, repeat: Infinity } : { duration: 0.2 },
          scale: breathe ? { duration: 2.4, repeat: Infinity, ease: 'easeInOut' } : { duration: 0.2 },
        }}
      />
      {breathe && state === 'run' && (
        <motion.span
          className="lamp-halo"
          style={{ borderColor: c }}
          animate={{ scale: [1, 2.3], opacity: [0.55, 0] }}
          transition={{ duration: 2.4, repeat: Infinity, ease: 'easeOut' }}
        />
      )}
    </span>
  );
}
