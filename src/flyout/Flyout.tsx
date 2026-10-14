import { AnimatePresence, motion } from 'motion/react';
import { FolderOpen, Layers, X } from 'lucide-react';
import { useEffect, useMemo, useRef, useState } from 'react';
import type { FlyoutCmd, SysApp } from '../bridge/types';
import { Lamp } from '../ui/Lamp';
import { Num } from '../ui/Num';
import { Sparkline } from '../ui/Sparkline';
import { ago, bytes, midPath, rate, uptime } from '../ui/format';
import { lampOf } from '../ui/status';
import { useApp } from '../ui/useApp';

export function Flyout() {
  const { api, config } = useApp();
  const [cmd, setCmd] = useState<FlyoutCmd | null>(null);
  const [open, setOpen] = useState(false);
  const seq = useRef(0);
  useEffect(
    () =>
      api.onFlyout((c) => {
        seq.current = c.seq;
        if (c.action === 'show') {
          setCmd(c);
          setOpen(true);
        } else setOpen(false);
      }),
    [api],
  );
  const isSys = cmd?.kind === 'sys';
  const item = isSys ? undefined : config.items.find((i) => i.id === cmd?.id);
  const side = cmd?.side ?? config.settings.side;
  const from = { right: { x: 10 }, left: { x: -10 }, bottom: { y: 10 }, top: { y: -10 } }[side];
  const origin = { right: '100% 50%', left: '0% 50%', bottom: '50% 100%', top: '50% 0%' }[side];

  return (
    <div className="fly-stage">
      <AnimatePresence onExitComplete={() => void api.flyoutHidden(seq.current)}>
        {open && (isSys || item) && cmd && (
          <motion.div
            key={cmd.seq}
            className="card"
            style={{ transformOrigin: origin }}
            initial={{ opacity: 0, scale: 0.94, filter: 'blur(4px)', ...from }}
            animate={{ opacity: 1, scale: 1, filter: 'blur(0px)', x: 0, y: 0 }}
            exit={{ opacity: 0, scale: 0.96, ...from, transition: { duration: 0.12 } }}
            transition={{ type: 'spring', stiffness: 520, damping: 34 }}
          >
            {isSys ? <SysMonitor /> : cmd.kind === 'metrics' ? <Metrics id={item!.id} /> : <Info id={item!.id} />}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

/**
 * System-wide mini monitor.
 *
 * The backend sends a bounded snapshot that already answers *both* rankings
 * (top-CPU ∪ top-RAM), so switching tabs never re-asks the OS and the CPU tab
 * is a real system-wide ranking, not a reorder of the RAM five.
 *
 * Sample contract (`types.ts`): `ts` is unix **milliseconds** of the newest
 * real sample; `seq` counts frames and carry-forward frames repeat the last
 * `ts`. A sample is stale once wall-clock time since `ts` exceeds
 * `STALE_MS` — measured from the sample stamp, so the marker flips even
 * while carry-forward frames keep arriving (sampling stopped) and even
 * without a rerender (a ticking clock drives the check). An absent sample
 * (`ts <= 0`) is an explicit loading state: never stale, never zeros.
 *
 * Ranking is **live** outside interaction: every frame recomputes the
 * selected top-five, so new leaders enter and exited apps leave on their own.
 * While someone aims at a row (hover), presses (pointer down) or focuses it
 * (keyboard), membership and order freeze; a row whose app exits inside that
 * interval becomes a disabled tombstone instead of disappearing, so neighbour
 * slots and the X under the cursor never shift. The close request captures
 * the row identity (full path + PID/start-time set) at the press boundary;
 * the backend revalidates it before any WM_CLOSE.
 */
const STALE_MS = 6000;

function SysMonitor() {
  const { api, rt, sysHist } = useApp();
  const apps = rt.sysTop;
  const sys = rt.sys;
  const [tab, setTab] = useState<'ram' | 'cpu'>('ram');
  const [closeState, setCloseState] = useState<Record<string, { pending: boolean; text: string; title: string }>>({});
  // frozen membership while the user interacts; null = live ranking
  const [frozenIds, setFrozenIds] = useState<string[] | null>(null);
  // identity captured at the press/focus boundary of the X button
  const capture = useRef<{ id: string; procs: SysApp['procs'] } | null>(null);
  // whether the pointer is inside the list: a press that ends inside keeps
  // the freeze (the user is still aiming), one that ends outside releases it
  const inside = useRef(false);
  // ticking clock: staleness must flip while events have stopped, without
  // waiting for the next frame to rerender the popup
  const [, setTick] = useState(0);
  useEffect(() => {
    const t = window.setInterval(() => setTick((n) => n + 1), 1000);
    return () => window.clearInterval(t);
  }, []);

  const byId = useMemo(() => new Map(apps.map((a) => [a.id, a])), [apps]);
  // live selection: recomputed from every frame — a new leader enters the
  // top-five as soon as its sample arrives, an exited app leaves
  const liveIds = useMemo(() => {
    const sorted = [...apps].sort((a, b) =>
      tab === 'cpu' ? b.cpu - a.cpu || a.id.localeCompare(b.id) : b.mem - a.mem || a.id.localeCompare(b.id),
    );
    return sorted.slice(0, 5).map((a) => a.id);
  }, [apps, tab]);
  const ids = frozenIds ?? liveIds;

  const noSample = rt.ts <= 0;
  const stale = !noSample && Date.now() - rt.ts > STALE_MS;

  // interaction-scoped freeze: `arm` snapshots the current top-five once,
  // `release` resumes live ranking. Released by pointer-leave, focus-leave
  // or a press that ends outside the list — never latched for the popup.
  const arm = () => setFrozenIds((cur) => cur ?? liveIds);
  const release = () => setFrozenIds(null);
  useEffect(() => {
    const up = () => {
      if (!inside.current) setFrozenIds(null);
    };
    window.addEventListener('pointerup', up);
    return () => window.removeEventListener('pointerup', up);
  }, []);

  const doClose = (app: SysApp) => {
    // dispatch the identity captured at the press boundary, not a live
    // lookup; the backend revalidates PID + start-time before WM_CLOSE
    const target = capture.current && capture.current.id === app.id ? capture.current : { id: app.id, procs: app.procs };
    capture.current = null;
    setCloseState((s) => ({ ...s, [app.id]: { pending: true, text: '…', title: 'запрос отправлен' } }));
    void api
      .sysClose({ id: target.id, procs: target.procs })
      .then((res) => {
        const text = res.status === 'dispatched' ? `×${res.dispatched}` : res.status === 'refused' ? 'нельзя' : 'нет';
        setCloseState((s) => ({ ...s, [app.id]: { pending: false, text, title: res.reason } }));
        window.setTimeout(() => {
          setCloseState((s) => {
            const next = { ...s };
            delete next[app.id];
            return next;
          });
        }, 2600);
      })
      .catch(() => setCloseState((s) => ({ ...s, [app.id]: { pending: false, text: 'ошибка', title: 'запрос не выполнен' } })));
  };

  return (
    <>
      <div className="sysmon-head">
        <span className="sysmon-title">Система</span>
        <div className="sysmon-tabs" role="tablist">
          <button
            type="button"
            role="tab"
            aria-selected={tab === 'cpu'}
            className={tab === 'cpu' ? 'on' : ''}
            onClick={() => setTab('cpu')}
          >
            CPU
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={tab === 'ram'}
            className={tab === 'ram' ? 'on' : ''}
            onClick={() => setTab('ram')}
          >
            RAM
          </button>
        </div>
      </div>
      <div
        className="sysmon-body"
        onPointerEnter={() => {
          inside.current = true;
          arm();
        }}
        onPointerLeave={() => {
          inside.current = false;
          release();
        }}
        onPointerDown={arm}
        onFocus={arm}
        onBlur={release}
      >
        {noSample ? (
          <div className="sysmon-empty">Ожидание первых данных…</div>
        ) : apps.length === 0 ? (
          <div className="sysmon-empty">Нет данных</div>
        ) : (
          ids.map((id) => {
            const app = byId.get(id);
            const st = closeState[id];
            if (!app) {
              // tombstone: the app exited while the user was interacting —
              // the slot stays put (disabled) instead of shifting neighbours
              return (
                <div key={id} className="sysmon-row" data-gone="" data-closing={st ? '' : undefined}>
                  <span className="sysmon-icon">—</span>
                  <div className="sysmon-info">
                    <span className="sysmon-name">приложение закрыто</span>
                    <span className="sysmon-sub">—</span>
                  </div>
                  <span className="sysmon-val">—</span>
                  <span className="sysmon-val">—</span>
                  {st ? (
                    <span className="sysmon-msg" title={st.title}>
                      {st.text}
                    </span>
                  ) : null}
                  <button
                    type="button"
                    className="sysmon-close"
                    disabled
                    title="приложение закрыто"
                    aria-label="Приложение закрыто"
                  >
                    <X size={12} />
                  </button>
                </div>
              );
            }
            const mem = bytes(app.mem);
            const letter = app.name[0]?.toUpperCase() ?? '?';
            return (
              <div key={id} className="sysmon-row" data-closing={st ? '' : undefined}>
                <span className="sysmon-icon">{letter}</span>
                <div className="sysmon-info">
                  <span className="sysmon-name" title={app.id}>
                    {app.name}
                  </span>
                  <span className="sysmon-sub">
                    {app.procs.length > 1 ? `${app.procs.length} proc` : app.procs.length === 1 ? '1 proc' : '—'}
                    {app.windows > 0 ? ` · ${app.windows} win` : ''}
                  </span>
                </div>
                <span className={`sysmon-val${tab === 'cpu' ? ' on' : ''}`} title="процессор, % всех ядер">
                  {app.cpu.toFixed(1)}%
                </span>
                <span className={`sysmon-val${tab === 'ram' ? ' on' : ''}`} title="память, рабочий набор">
                  {mem.v}
                  <small>{mem.u}</small>
                </span>
                {st ? (
                  <span className="sysmon-msg" title={st.title}>
                    {st.text}
                  </span>
                ) : null}
                <button
                  type="button"
                  className="sysmon-close"
                  disabled={!app.closable || st !== undefined}
                  title={app.closable ? 'Закрыть окна приложения' : 'защищённый или системный процесс'}
                  aria-label={`Закрыть ${app.name}`}
                  onPointerDown={() => {
                    capture.current = { id, procs: app.procs };
                  }}
                  onFocus={() => {
                    capture.current = { id, procs: app.procs };
                  }}
                  onClick={() => doClose(app)}
                >
                  <X size={12} />
                </button>
              </div>
            );
          })
        )}
        {stale ? <div className="sysmon-stale">данные устарели</div> : null}
      </div>
      <div className="sysmon-foot">
        <span className="sysmon-tot">
          CPU {sys.cpu.toFixed(0)}% · RAM {bytes(sys.memUsed).v}
          {bytes(sys.memUsed).u} / {bytes(sys.memTotal).v}
          {bytes(sys.memTotal).u}
        </span>
        <span className="sysmon-charts" aria-hidden>
          <Sparkline data={sysHist.cpu} max={100} w={64} h={16} />
          <Sparkline data={sysHist.memPct} max={100} w={64} h={16} color="var(--lamp-mem)" />
        </span>
      </div>
    </>
  );
}

function Head({ id, right }: { id: string; right?: React.ReactNode }) {
  const { config } = useApp();
  const item = config.items.find((i) => i.id === id)!;
  return (
    <div className="card-head">
      {item.icon ? <img className="card-icon" src={item.icon} alt="" /> : <span className="card-icon tile-fallback">{item.name[0]}</span>}
      <span className="card-title">{item.name}</span>
      {right}
    </div>
  );
}

function Info({ id }: { id: string }) {
  const { config } = useApp();
  const item = config.items.find((i) => i.id === id)!;
  const last = ago(item.lastLaunch);
  const added = ago(item.addedAt);
  return (
    <>
      <Head id={id} right={<span className="chip">{item.kind === 'app' ? 'APP' : item.kind === 'folder' ? 'DIR' : 'FILE'}</span>} />
      <div className="stats3">
        <Stat v={last.v} u={last.u} k="запуск" />
        <Stat v={String(item.launchCount)} u="×" k="запусков" />
        <Stat v={added.v} u={added.u} k="на панели" />
      </div>
      <div className="path" title={item.launchPath}>
        <FolderOpen size={12} />
        <span>{midPath(item.launchPath)}</span>
      </div>
    </>
  );
}

function Stat({ v, u, k }: { v: string; u: string; k: string }) {
  return (
    <div className="stat">
      <span className="stat-k">{k}</span>
      <span className="stat-v">
        {v}
        <small>{u}</small>
      </span>
    </div>
  );
}

function Metrics({ id }: { id: string }) {
  const { rt, hist } = useApp();
  const r = rt.items[id];
  const h = hist[id] ?? { cpu: [], mem: [], io: [] };
  if (!r) return <Head id={id} />;
  const mem = bytes(r.mem);
  return (
    <>
      <Head
        id={id}
        right={
          <span className="head-badges">
            <span className="badge">
              <Lamp state={lampOf(r, null)} size={6} />
              {r.hung ? 'не отвечает' : r.windows > 1 ? `×${r.windows}` : 'окно'}
            </span>
            <span className="badge">
              <Layers size={11} />
              {r.procs}
            </span>
          </span>
        }
      />
      <div className="grid2">
        <div className="tile-m">
          <span className="m-k">CPU</span>
          <span className="m-v">
            <Num value={r.cpu} digits={1} />
            <small>%</small>
          </span>
          <Sparkline data={h.cpu} max={10} color="var(--accent)" />
        </div>
        <div className="tile-m">
          <span className="m-k">RAM</span>
          <span className="m-v">
            <Num value={Number(mem.v)} digits={mem.u === 'GB' ? 2 : mem.u === 'MB' && r.mem < 1e8 ? 1 : 0} />
            <small>{mem.u}</small>
          </span>
          <Sparkline data={h.mem} color="var(--lamp-mem)" />
        </div>
        <div className="tile-m">
          <span className="m-k">ДИСК</span>
          <span className="m-io">
            <span>
              <i className="io-r" />R {rate(r.read)}
            </span>
            <span>
              <i className="io-w" />W {rate(r.write)}
            </span>
          </span>
          <Sparkline data={h.io} color="var(--lamp-io)" h={14} />
        </div>
        <div className="tile-m">
          <span className="m-k">В РАБОТЕ</span>
          <span className="m-v m-time">{uptime(r.uptime)}</span>
          <span className="m-sub">{r.minimized ? 'свёрнуто' : 'активно'}</span>
        </div>
      </div>
    </>
  );
}
