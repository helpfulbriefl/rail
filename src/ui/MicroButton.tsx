import { motion } from 'motion/react';
import type { ReactNode } from 'react';

export function MicroButton({ label, active, onClick, children }: { label: string; active?: boolean; onClick: () => void; children: ReactNode }) {
  return (
    <motion.button
      type="button"
      className="micro"
      data-active={active ? '' : undefined}
      aria-label={label}
      aria-pressed={active}
      title={label}
      onClick={onClick}
      whileHover={{ scale: 1.08 }}
      whileTap={{ scale: 0.9 }}
      transition={{ type: 'spring', stiffness: 600, damping: 30 }}
    >
      {children}
    </motion.button>
  );
}
