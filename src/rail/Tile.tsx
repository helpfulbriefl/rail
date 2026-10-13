import { AnimatePresence, motion, useAnimate, useMotionValue, useSpring } from 'motion/react';
import { useEffect, useRef, useState } from 'react';
import type { Item, ItemRt, Side } from '../bridge/types';
import { Lamp } from '../ui/Lamp';
import { IO_FLASH_BPS, MEM_FLASH_BPS, lampOf } from '../ui/status';
import { iconColor } from '../ui/iconColor';
import { useApp } from '../ui/useApp';
import { useMag } from './magnify';

const SPRING = { type: 'spring', stiffness: 520, damping: 36, mass: 0.7 } as const;

export interface TileProps {
  item: Item;
  rt?: ItemRt;
  side: Side;
  size: number;
  magnify: boolean;
  dragging: boolean;
  onMenu: () => void;
  justDragged: () => boolean;
}

export function tileBox(running: boolean, vertical: boolean, S: number) {
  if (!running) return { w: S, h: S };
  // Keep the growth proportional: the old flat +30 px made small icons look
  // like wide pills and pushed the rail far past its configured width.
  const grow = Math.round(S * 0.28);
  return vertical ? { w: S + Math.round(grow * 0.45), h: S + grow } : { w: S + grow, h: S + Math.round(grow * 0.1) };
}

export function Tile({ item, rt, side, size: S, magnify, dragging, onMenu, justDragged }: TileProps) {
  const { api, config } = useApp();
  const vertical = side === 'left' || side === 'right';
  const inward = side === 'right' || side === 'bottom' ? -1 : 1; // direction away from screen edge
  const running = (rt?.windows ?? 0) > 0;
  const count = rt?.windows ?? 0;
  const launching = !!rt?.launching && !running;

  // ---- magnification registration
  const mag = useMag();
  const rawScale = useMotionValue(1);
  const rawShift = useMotionValue(0);
  const scale = useSpring(rawScale, { stiffness: 520, damping: 34, mass: 0.5 });
  const shift = useSpring(rawShift, { stiffness: 520, damping: 34, mass: 0.5 });
  const slotRef = useRef<HTMLDivElement | null>(null);
  useEffect(() => mag.register(item.id, { scale: rawScale, shift: rawShift, el: slotRef.current?.parentElement ?? null, max: magnify ? (running || launching ? 1.24 : 1.4) : 1 }), [item.id, running, launching, magnify, mag, rawScale, rawShift]);

  // ---- lamp flash on disk / memory bursts
  const [flash, setFlash] = useState<'io' | 'mem' | null>(null);
  const prevMem = useRef(rt?.mem ?? 0);
  useEffect(() => {
    if (!rt || !running) return;
    const io = rt.read + rt.write;
    const memDelta = prevMem.current > 0 ? rt.mem - prevMem.current : 0;
    prevMem.current = rt.mem;
    const f = io >= IO_FLASH_BPS ? 'io' : memDelta >= MEM_FLASH_BPS ? 'mem' : null;
    if (!f) return;
    setFlash(f);
    const t = setTimeout(() => setFlash(null), 280);
    return () => clearTimeout(t);
  }, [rt, running]);
  const lamp = lampOf(rt, flash);

  // ---- aurora glow colour
  const [glow, setGlow] = useState<string>('transparent');
  useEffect(() => {
    if (item.icon) void iconColor(item.icon).then(setGlow);
  }, [item.icon]);

  // ---- hold-to-reveal flyout
  const [holding, setHolding] = useState(false);
  const flyoutOpen = useRef(false);
  const holdTimer = useRef<number | undefined>(undefined);
  const magRef = useRef<HTMLDivElement | null>(null);
  const stopHold = () => {
    window.clearTimeout(holdTimer.current);
    setHolding(false);
    if (flyoutOpen.current) {
      flyoutOpen.current = false;
      void api.hideFlyout();
    }
  };
  const startHold = () => {
    if (dragging) return;
    stopHold();
    setHolding(true);
    // Hard cap at 3s: a stale config (or a hand-edited file) must never make the
    // flyout feel broken.
    const delay = Math.min(Math.max(config.settings.hoverDelayMs || 900, 250), 3000);
    holdTimer.current = window.setTimeout(() => {
      setHolding(false);
      const r = magRef.current?.getBoundingClientRect();
      if (!r) return;
      flyoutOpen.current = true;
      const kind = running ? 'metrics' : 'info';
      // Design size in CSS px — the backend multiplies by the monitor scale.
      // 304×236 / 268×158 was the pre-micro card; the tightened tokens in
      // app.css now fit in 220×176 / 200×120 (≈ ×0.72), so anything larger
      // leaves dead margin around the content.
      void api.showFlyout({ kind, id: item.id, x: r.left, y: r.top, w: r.width, h: r.height, width: kind === 'metrics' ? 220 : 200, height: kind === 'metrics' ? 176 : 120 });
    }, delay);
  };
  const holdMs = Math.min(Math.max(config.settings.hoverDelayMs || 900, 250), 3000);
  useEffect(() => () => window.clearTimeout(holdTimer.current), []);
  useEffect(() => {
    if (dragging) stopHold();
  }, [dragging]); // eslint-disable-line react-hooks/exhaustive-deps

  // ---- launch feedback (bounce while launching, shake on failure)
  const [kick, animateKick] = useAnimate();
  useEffect(
    () =>
      api.onLaunch((r) => {
        if (r.id !== item.id || !kick.current) return;
        const axis = vertical ? 'x' : 'y';
        if (!r.ok) void animateKick(kick.current, { [axis === 'x' ? 'y' : 'x']: [0, -5, 5, -4, 4, 0] }, { duration: 0.42 });
      }),
    [api, item.id, vertical, animateKick, kick],
  );

  const click = (e: React.MouseEvent) => {
    if (justDragged()) return;
    stopHold();
    if (e.shiftKey) return void api.launch(item.id);
    if (running) return void api.activate(item.id);
    void api.launch(item.id);
    if (kick.current) {
      const axis = vertical ? 'x' : 'y';
      void animateKick(kick.current, { [axis]: [0, inward * 10, 0, inward * 4, 0] }, { duration: 0.55, ease: 'easeOut' });
    }
  };

  const block = running || launching;
  const box = tileBox(block, vertical, S);
  const ghosts = Math.min(Math.max(count - 1, 0), 2);
  const origin = { right: '100% 50%', left: '0% 50%', bottom: '50% 100%', top: '50% 0%' }[side];
  const radius = block ? 'var(--r-block)' : 'var(--r-tile)';
  const iconSize = block ? Math.round(S * 0.6) : Math.round(S * 0.78);

  return (
    <div ref={slotRef} className="slot" style={{ width: box.w, height: box.h }}>
      <motion.div
        ref={magRef}
        className="mag"
        style={{ scale, [vertical ? 'y' : 'x']: shift, transformOrigin: origin }}
      >
        <motion.div
          ref={kick}
          className="kick"
          animate={launching ? { [vertical ? 'x' : 'y']: [0, inward * 7, 0] } : { [vertical ? 'x' : 'y']: 0 }}
          transition={launching ? { duration: 0.8, repeat: Infinity, ease: 'easeInOut' } : { duration: 0.2 }}
        >
          {/* stacked copies — "more than one window" */}
          <AnimatePresence>
            {Array.from({ length: ghosts }, (_, k) => (
              <motion.span
                key={k}
                className="ghost"
                style={{ borderRadius: radius }}
                initial={{ opacity: 0, x: 0, y: 0, scale: 1 }}
                animate={{
                  opacity: 0.75 - k * 0.25,
                  scale: 1 - (k + 1) * 0.05,
                  [vertical ? 'x' : 'y']: inward * (k + 1) * 4,
                }}
                exit={{ opacity: 0, x: 0, y: 0 }}
                transition={SPRING}
              />
            ))}
          </AnimatePresence>

          <motion.button
            type="button"
            className="tile"
            data-running={block ? '' : undefined}
            data-minimized={rt?.minimized ? '' : undefined}
            data-vertical={vertical ? '' : undefined}
            aria-label={item.name}
            layout
            transition={SPRING}
            style={{ width: box.w, height: box.h, borderRadius: radius, ['--glow' as string]: glow }}
            whileTap={{ scale: 0.92 }}
            onPointerEnter={startHold}
            onPointerLeave={stopHold}
            onPointerDown={() => window.clearTimeout(holdTimer.current)}
            onClick={click}
            onAuxClick={(e) => {
              if (e.button === 1) void api.launch(item.id);
            }}
            onContextMenu={(e) => {
              e.preventDefault();
              stopHold();
              onMenu();
            }}
          >
            {item.icon ? (
              <motion.img layout="position" transition={SPRING} className="tile-icon" src={item.icon} alt="" draggable={false} animate={{ width: iconSize, height: iconSize }} />
            ) : (
              <motion.span layout="position" className="tile-icon tile-fallback" animate={{ width: iconSize, height: iconSize }}>
                {item.name.slice(0, 1)}
              </motion.span>
            )}
            <AnimatePresence>
              {(running || launching) && (
                <motion.span
                  key="meta"
                  className="tile-meta"
                  layout="position"
                  initial={{ opacity: 0, scale: 0.4 }}
                  animate={{ opacity: 1, scale: 1 }}
                  exit={{ opacity: 0, scale: 0.4 }}
                  transition={SPRING}
                >
                  <Lamp state={lamp} breathe={count === 1} />
                  <AnimatePresence initial={false}>
                    {count >= 2 && (
                      <motion.span key="count" className="count" initial={{ opacity: 0, width: 0 }} animate={{ opacity: 1, width: 'auto' }} exit={{ opacity: 0, width: 0 }} transition={SPRING}>
                        <AnimatePresence mode="popLayout" initial={false}>
                          <motion.span key={count} className="count-n" initial={{ y: 9, opacity: 0 }} animate={{ y: 0, opacity: 1 }} exit={{ y: -9, opacity: 0 }} transition={SPRING}>
                            {count > 99 ? '99' : count}
                          </motion.span>
                        </AnimatePresence>
                      </motion.span>
                    )}
                  </AnimatePresence>
                </motion.span>
              )}
            </AnimatePresence>
          </motion.button>

          {/* hold progress ring */}
          <AnimatePresence>
            {holding && (
              <motion.svg key="ring" className="hold-ring" width={box.w + 8} height={box.h + 8} initial={{ opacity: 0 }} animate={{ opacity: [0, 0, 1] }} exit={{ opacity: 0 }} transition={{ duration: (holdMs / 1000) * 0.35, times: [0, 0.6, 1] }}>
                <motion.rect x={1.5} y={1.5} width={box.w + 5} height={box.h + 5} rx={block ? 14 : 16} fill="none" stroke="var(--accent)" strokeWidth={2} strokeLinecap="round" initial={{ pathLength: 0 }} animate={{ pathLength: 1 }} transition={{ duration: holdMs / 1000, ease: 'linear' }} />
              </motion.svg>
            )}
          </AnimatePresence>
        </motion.div>
      </motion.div>
    </div>
  );
}
