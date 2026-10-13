import type { ItemRt } from '../bridge/types';

export type LampState = 'off' | 'launch' | 'run' | 'load' | 'hung' | 'io' | 'mem';

/** Traffic-light rule. Order matters: worst state wins. */
export function lampOf(rt: ItemRt | undefined, flash: 'io' | 'mem' | null): LampState {
  if (!rt) return 'off';
  if (rt.launching && rt.windows === 0) return 'launch';
  if (rt.windows === 0) return 'off';
  if (rt.hung) return 'hung';
  if (flash) return flash;
  if (rt.cpu >= 40) return 'load';
  return 'run';
}

export const IO_FLASH_BPS = 1_000_000; // ≥1 MB/s disk → blue blink
export const MEM_FLASH_BPS = 25_000_000; // ≥25 MB/s RAM growth → violet blink
