import { AnimatePresence, motion, Reorder, useMotionValue, useMotionValueEvent, useSpring, type MotionValue } from 'motion/react';
import { Pin, Plus, SlidersHorizontal } from 'lucide-react';
import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from 'react';
import { isTauri } from '../bridge/api';
import { SCALE_MUL } from '../bridge/types';
import { MicroButton } from '../ui/MicroButton';
import { Num } from '../ui/Num';
import { useApp } from '../ui/useApp';
import { computeMagnify, MagContext, type MagReg } from './magnify';
import { Tile } from './Tile';

const SHOW = { type: 'spring', stiffness: 420, damping: 34, mass: 0.8 } as const;
/** How long a `leave` must persist before the drop indicator is dropped. */
const LEAVE_GRACE_MS = 220;

export function Rail() {
  const { api, config, rt } = useApp();
  const s = config.settings;
  const side = s.side;
  const vertical = side === 'left' || side === 'right';
  const scale = s.railScale ?? 'compact';
  const compact = scale !== 'normal';
  const S = Math.round(s.iconSize * (SCALE_MUL[scale] ?? 1));

  const [shown, setShown] = useState(false);
  const seq = useRef(0);
  const [hover, setHover] = useState(false);
  const everHovered = useRef(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [dragging, setDragging] = useState(false);
  const [drop, setDrop] = useState<number | null>(null);
  const [flyoutOpen, setFlyoutOpen] = useState(false);
  const dragEndAt = useRef(0);
  /** external (Explorer) drag is hovering the window */
  const extDrag = drop !== null;

  // ---------- backend commands
  const shownRef = useRef(false);
  useEffect(() => {
    const un = api.onRail((c) => {
      seq.current = c.seq;
      if (c.action === 'show') everHovered.current = false;
      // already hidden on our side → nothing to animate, release the window now
      if (c.action === 'hide' && !shownRef.current) void api.railHidden(c.seq);
      shownRef.current = c.action === 'show';
      setShown(c.action === 'show');
    });
    void api.railReady();
    return un;
  }, [api]);
  useEffect(() => api.onHover((h) => {
    setHover(h);
    if (h) {
      everHovered.current = true;
      setMenuOpen(false);
    }
  }), [api]);
  useEffect(() => api.onMenuClosed(() => setMenuOpen(false)), [api]);
  useEffect(() => api.onFlyout((c) => setFlyoutOpen(c.action === 'show')), [api]);
  useEffect(() => {
    if (!menuOpen) return;
    const t = setTimeout(() => setMenuOpen(false), 5000);
    return () => clearTimeout(t);
  }, [menuOpen]);

  // ---------- auto-hide: mouse away → hideDelay → slide out
  useEffect(() => {
    if (!shown || s.pinned || hover || menuOpen || dragging || extDrag || flyoutOpen) return;
    // First reveal with no interaction yet: keep the rail around a bit longer so
    // "flash on the edge" is readable. Capped so the panel never lingers >3s
    // past the configured delay.
    const extra = everHovered.current ? 0 : 1200;
    const t = setTimeout(() => void api.requestHide(), s.hideDelayMs + extra);
    return () => clearTimeout(t);
  }, [api, shown, s.pinned, hover, menuOpen, dragging, extDrag, flyoutOpen, s.hideDelayMs]);

  // ---------- drag lease: refresh while any drag is alive, release on end
  //
  // The backend keeps the window region open (and suppresses auto-hide) only
  // while the lease is fresh, so a crashed webview can never pin the rail.
  const dragAlive = dragging || extDrag;
  useEffect(() => {
    if (!dragAlive) return;
    void api.dragActive(true);
    const t = setInterval(() => void api.dragActive(true), 600);
    return () => {
      clearInterval(t);
      void api.dragActive(false);
    };
  }, [api, dragAlive]);

  useEffect(() => {
    const k = (e: KeyboardEvent) => e.key === 'Escape' && void api.requestHide();
    window.addEventListener('keydown', k);
    return () => window.removeEventListener('keydown', k);
  }, [api]);

  // ---------- order (local while dragging, persisted on drop)
  const ids = useMemo(() => config.items.map((i) => i.id), [config.items]);
  const [order, setOrder] = useState(ids);
  const orderRef = useRef(order);
  orderRef.current = order;
  // `reorder` is async: between drop and the config echo the backend still
  // reports the old order, and syncing to it snapped tiles back to where they
  // started. Only adopt the backend order when the sets actually differ
  // (add / remove), never for a pure permutation we just authored.
  useEffect(() => {
    if (dragging) return;
    const cur = orderRef.current;
    const sameSet = cur.length === ids.length && cur.every((id) => ids.includes(id));
    if (sameSet) return;
    setOrder(ids);
  }, [ids, dragging]);
  const byId = useMemo(() => new Map(config.items.map((i) => [i.id, i])), [config.items]);

  // ---------- magnification
  const pointer = useMotionValue(-1e4);
  const activeRaw = useMotionValue(0);
  const active = useSpring(activeRaw, { stiffness: 300, damping: 30 });
  const halfRaw = useMotionValue(0);
  const half = useSpring(halfRaw, { stiffness: 520, damping: 34, mass: 0.5 });
  const negHalf = useMotionValue(0);
  useMotionValueEvent(half, 'change', (v) => negHalf.set(-v));
  const regs = useRef(new Map<string, MagReg>());
  const recompute = useCallback(() => {
    halfRaw.set(computeMagnify(orderRef.current, regs.current, pointer.get(), active.get(), vertical, S * 0.95));
  }, [halfRaw, pointer, active, vertical, S]);
  useMotionValueEvent(pointer, 'change', recompute);
  useMotionValueEvent(active, 'change', recompute);
  const magCtx = useMemo(
    () => ({
      register(id: string, r: MagReg) {
        regs.current.set(id, r);
        return () => {
          if (regs.current.get(id) === r) regs.current.delete(id);
        };
      },
    }),
    [],
  );
  const listRef = useRef<HTMLDivElement | null>(null);
  const onMove = (e: React.PointerEvent) => {
    const r = listRef.current?.getBoundingClientRect();
    if (!r) return;
    pointer.set(vertical ? e.clientY - r.top : e.clientX - r.left);
    activeRaw.set(dragging || s.reduceMotion ? 0 : 1);
  };

  // ---------- hit area for click-through outside the capsule
  const capRef = useRef<HTMLDivElement | null>(null);
  useLayoutEffect(() => {
    if (!shown) return;
    let raf = 0;
    let last = { x: 0, y: 0, w: 0, h: 0 };
    const push = () => {
      const el = capRef.current;
      if (!el) return;
      const r = el.getBoundingClientRect();
      // The magnification spring makes the capsule grow along its axis; quantise
      // to 4 px so a running animation does not fire one IPC call per frame
      // (that storm of SetWindowRgn calls was what killed Explorer drops).
      const q = (v: number) => Math.round(v / 4) * 4;
      const along = S * 0.9 + half.get();
      const cross = S * 0.5;
      const next = vertical
        ? { x: q(r.left - cross), y: q(r.top - along), w: q(r.width + cross * 2), h: q(r.height + along * 2) }
        : { x: q(r.left - along), y: q(r.top - cross), w: q(r.width + along * 2), h: q(r.height + cross * 2) };
      if (next.x === last.x && next.y === last.y && next.w === last.w && next.h === last.h) return;
      last = next;
      void api.setHitRect(next);
    };
    const report = () => {
      cancelAnimationFrame(raf);
      raf = requestAnimationFrame(push);
    };
    report();
    const ro = new ResizeObserver(report);
    if (capRef.current) ro.observe(capRef.current);
    const un = half.on('change', report);
    return () => {
      ro.disconnect();
      un();
      cancelAnimationFrame(raf);
    };
  }, [api, shown, vertical, S, half]);

  // ---------- external drag & drop (Explorer / desktop → rail)
  const indexAt = useCallback(
    (x: number, y: number) => {
      const list = listRef.current;
      if (!list) return orderRef.current.length;
      const kids = Array.from(list.querySelectorAll<HTMLElement>('[data-slot-id]'));
      for (let i = 0; i < kids.length; i++) {
        const r = kids[i].getBoundingClientRect();
        const c = vertical ? r.top + r.height / 2 : r.left + r.width / 2;
        if ((vertical ? y : x) < c) return i;
      }
      return kids.length;
    },
    [vertical],
  );
  const dropIndex = useRef(0);
  useEffect(() => {
    let grace = 0;
    let lease = 0;
    // An external Explorer drag only reaches `over` if the window region is
    // already open — so the very first `over` both re-arms the lease and asks
    // the backend to clear the region. Without this the region stayed
    // installed, Windows treated the rail as absent, and the cursor showed the
    // "no entry" sign instead of accepting the drop.
    const arm = () => {
      void api.dragActive(true);
      window.clearInterval(lease);
      lease = window.setInterval(() => void api.dragActive(true), 600);
    };
    const disarm = () => {
      window.clearInterval(lease);
      lease = 0;
      void api.dragActive(false);
    };
    const un = api.onDrop((e) => {
      // Windows emits spurious `leave` events whenever the drag crosses the
      // WebView2 child-window boundary (which happens constantly over the
      // tiles). Reacting immediately cleared the indicator and the rail hid
      // itself out from under the drop — hence "drag and drop doesn't work".
      // A short grace period swallows the bogus pairs.
      if (e.type === 'leave') {
        window.clearTimeout(grace);
        grace = window.setTimeout(() => {
          setDrop(null);
          disarm();
        }, LEAVE_GRACE_MS);
        return;
      }
      window.clearTimeout(grace);
      arm();
      const i = indexAt(e.x, e.y);
      dropIndex.current = i;
      if (e.type === 'drop') {
        setDrop(null);
        if (e.paths.length) void api.addPaths(e.paths, i);
        disarm();
      } else setDrop(i);
    });
    return () => {
      window.clearTimeout(grace);
      window.clearInterval(lease);
      un();
    };
  }, [api, indexAt]);

  // ---------- render
  const hiddenPose = {
    opacity: 0,
    scale: 0.94,
    filter: 'blur(8px)',
    x: side === 'right' ? 26 : side === 'left' ? -26 : 0,
    y: side === 'bottom' ? 26 : side === 'top' ? -26 : 0,
  };
  const axisStyle = (mv: MotionValue<number>) => (vertical ? { y: mv } : { x: mv });
  const list = order.filter((id) => byId.has(id));

  return (
    <div className="stage" data-side={side} data-scale={scale} data-compact={compact ? '' : undefined}>
      <AnimatePresence onExitComplete={() => void api.railHidden(seq.current)}>
        {shown && (
          <motion.div
            key="rail"
            ref={capRef}
            className="rail"
            data-vertical={vertical ? '' : undefined}
            initial={hiddenPose}
            animate={{ opacity: 1, scale: 1, filter: 'blur(0px)', x: 0, y: 0, transition: { ...SHOW, staggerChildren: 0.03, delayChildren: 0.05 } }}
            exit={{ ...hiddenPose, transition: { duration: 0.2, ease: [0.4, 0, 1, 1] } }}
            onPointerEnter={isTauri ? undefined : () => setHover(true)}
            onPointerLeave={isTauri ? undefined : () => setHover(false)}
          >
            <motion.div className="rail-bg" data-drop={extDrag ? '' : undefined} style={vertical ? { top: negHalf, bottom: negHalf } : { left: negHalf, right: negHalf }} />
            <motion.div className="rail-head" style={axisStyle(negHalf)}>
              <MicroButton label={s.pinned ? 'Открепить' : 'Закрепить'} active={s.pinned} onClick={() => void api.setSettings({ ...s, pinned: !s.pinned })}>
                <motion.span animate={{ rotate: s.pinned ? 0 : 45 }} transition={SHOW} className="micro-ico">
                  <Pin size={13} strokeWidth={2.2} fill={s.pinned ? 'currentColor' : 'none'} />
                </motion.span>
              </MicroButton>
              <MicroButton label="Добавить" onClick={() => void api.pickFiles()}>
                <Plus size={13} strokeWidth={2.2} />
              </MicroButton>
              <MicroButton label="Настройки" onClick={() => void api.openSettings()}>
                <SlidersHorizontal size={13} strokeWidth={2.2} />
              </MicroButton>
            </motion.div>
            <motion.div className="rail-sep" style={axisStyle(negHalf)} />

            <MagContext.Provider value={magCtx}>
              <div ref={listRef} className="dock" onPointerMove={onMove} onPointerLeave={() => activeRaw.set(0)}>
                <Reorder.Group as="div" axis={vertical ? 'y' : 'x'} values={order} onReorder={setOrder} className="dock-list">
                  {list.map((id, i) => {
                    const item = byId.get(id)!;
                    return (
                      <FragmentWithDrop key={id} show={drop === i} S={S}>
                        <Reorder.Item
                          as="div"
                          value={id}
                          data-slot-id={id}
                          className="dock-item"
                          initial={{ opacity: 0, scale: 0.5 }}
                          animate={{ opacity: 1, scale: 1 }}
                          exit={{ opacity: 0, scale: 0.5 }}
                          transition={{ ...SHOW, delay: shown ? 0 : i * 0.03 }}
                          // Lifting by 1.12 used to push the tile outside the
                          // window region, where it was clipped away and the
                          // pointer left the window, aborting the gesture. The
                          // region now opens for the duration of the drag, and
                          // the lift is gentler so the tile stays inside the
                          // capsule's drag margin.
                          whileDrag={{ scale: 1.08, zIndex: 10, cursor: 'grabbing' }}
                          dragElastic={0.12}
                          dragMomentum={false}
                          // Starting the lease on the first pointer press (not
                          // on the first movement) means the region is already
                          // open by the time the tile actually moves.
                          onPointerDown={() => void api.dragActive(true)}
                          onDragStart={() => {
                            setDragging(true);
                            activeRaw.set(0);
                            void api.dragActive(true);
                          }}
                          onDragEnd={() => {
                            setDragging(false);
                            dragEndAt.current = performance.now();
                            void api.reorder(orderRef.current);
                            void api.dragActive(false);
                          }}
                        >
                          <Tile
                            item={item}
                            rt={rt.items[id]}
                            side={side}
                            size={S}
                            magnify={!s.reduceMotion}
                            dragging={dragging}
                            onMenu={() => {
                              setMenuOpen(true);
                              void api.itemMenu(id);
                            }}
                            justDragged={() => performance.now() - dragEndAt.current < 120}
                          />
                        </Reorder.Item>
                      </FragmentWithDrop>
                    );
                  })}
                  <AnimatePresence>{drop !== null && drop >= list.length && <DropSlot key="drop-end" S={S} />}</AnimatePresence>
                </Reorder.Group>
                {list.length === 0 && drop === null && (
                  <motion.div className="empty" initial={{ opacity: 0, scale: 0.8 }} animate={{ opacity: 1, scale: 1 }} transition={SHOW}>
                    <button type="button" className="empty-tile" style={{ width: S, height: S }} onClick={() => void api.pickFiles()} aria-label="Добавить">
                      <Plus size={16} strokeWidth={2.2} />
                    </button>
                    <span className="empty-hint">Перетащи ярлык или нажми +</span>
                  </motion.div>
                )}
              </div>
            </MagContext.Provider>

            {s.showMeters !== false && (
              <>
                <motion.div className="rail-sep" style={axisStyle(half)} />
                <motion.div
                  className="rail-foot"
                  style={axisStyle(half)}
                  data-interactive=""
                  role="button"
                  tabIndex={0}
                  aria-label="Монитор системных приложений"
                  onClick={() => {
                    const el = capRef.current;
                    if (!el) return;
                    const r = el.getBoundingClientRect();
                    void api.showFlyout({
                      kind: 'sys',
                      id: 'sys',
                      x: r.left,
                      y: r.top,
                      w: r.width,
                      h: r.height,
                      width: 300,
                      height: 280,
                    });
                  }}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      e.preventDefault();
                      const el = capRef.current;
                      if (!el) return;
                      const r = el.getBoundingClientRect();
                      void api.showFlyout({
                        kind: 'sys',
                        id: 'sys',
                        x: r.left,
                        y: r.top,
                        w: r.width,
                        h: r.height,
                        width: 300,
                        height: 280,
                      });
                    }
                  }}
                >
                  <Meter k="CPU" v={rt.sys.cpu} />
                  <Meter k="RAM" v={(rt.sys.memUsed / Math.max(rt.sys.memTotal, 1)) * 100} />
                </motion.div>
              </>
            )}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

function FragmentWithDrop({ show, S, children }: { show: boolean; S: number; children: React.ReactNode }) {
  return (
    <>
      <AnimatePresence>{show && <DropSlot key="drop" S={S} />}</AnimatePresence>
      {children}
    </>
  );
}

function DropSlot({ S }: { S: number }) {
  return (
    <motion.div
      className="drop-slot"
      layout
      initial={{ opacity: 0, width: 0, height: 0 }}
      animate={{ opacity: 1, width: S, height: S }}
      exit={{ opacity: 0, width: 0, height: 0 }}
      transition={{ type: 'spring', stiffness: 500, damping: 34 }}
    >
      <Plus size={14} strokeWidth={2.4} />
    </motion.div>
  );
}

function Meter({ k, v }: { k: string; v: number }) {
  const n = Number.isFinite(v) ? Math.min(Math.max(v, 0), 100) : 0;
  // Thresholds get 3 pp of hysteresis-free padding via rounding: the level is
  // derived from the *displayed* integer, so the colour can't flip while the
  // digits stay put.
  const shown = Math.round(n);
  const level = shown >= 85 ? 'hot' : shown >= 60 ? 'warm' : 'ok';
  return (
    <div className="meter" data-level={level}>
      <div className="meter-row">
        <span className="meter-k">{k}</span>
        <span className="meter-v">
          <Num value={n} />
          <small>%</small>
        </span>
      </div>
      <div className="meter-bar">
        <motion.i animate={{ scaleX: Math.max(shown, 2) / 100 }} transition={{ type: 'spring', stiffness: 160, damping: 28 }} />
      </div>
    </div>
  );
}
