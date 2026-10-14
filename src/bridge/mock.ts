/** Browser-only backend used for design previews and automated visual QA. */
import type { Api } from './api';
import type { Config, Item, ItemRt, RuntimePayload, Settings, Side, SysApp, SysCloseResult, Theme } from './types';

const q = new URLSearchParams(typeof location !== 'undefined' ? location.search : '');

function icon(c1: string, c2: string, glyph: string): string {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 96 96"><defs><linearGradient id="g" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="${c1}"/><stop offset="1" stop-color="${c2}"/></linearGradient></defs><rect x="4" y="4" width="88" height="88" rx="22" fill="url(#g)"/>${glyph}</svg>`;
  return 'data:image/svg+xml;utf8,' + encodeURIComponent(svg);
}
const W = '#fff';
const ICONS = [
  ['Code', icon('#3BA7F5', '#1268C4', `<path d="M58 22 34 48l24 26" stroke="${W}" stroke-width="9" fill="none" stroke-linecap="round" stroke-linejoin="round"/><path d="M62 30v36" stroke="${W}" stroke-width="9" stroke-linecap="round"/>`)],
  ['Chrome', icon('#FBBF24', '#EF4444', `<circle cx="48" cy="48" r="26" fill="${W}"/><circle cx="48" cy="48" r="13" fill="#3B82F6"/>`)],
  ['Telegram', icon('#38BDF8', '#0284C7', `<path d="M24 47l44-18-8 40-14-10-8 8 1-13 19-17-24 15z" fill="${W}"/>`)],
  ['Figma', icon('#A78BFA', '#7C3AED', `<circle cx="56" cy="48" r="10" fill="${W}"/><rect x="30" y="22" width="20" height="20" rx="10" fill="${W}" opacity=".85"/><rect x="30" y="44" width="20" height="20" rx="10" fill="${W}" opacity=".7"/><rect x="30" y="56" width="20" height="18" rx="9" fill="${W}" opacity=".55"/>`)],
  ['Terminal', icon('#334155', '#0F172A', `<path d="M28 34l14 14-14 14" stroke="#34D399" stroke-width="7" fill="none" stroke-linecap="round" stroke-linejoin="round"/><path d="M48 64h20" stroke="${W}" stroke-width="7" stroke-linecap="round"/>`)],
  ['Steam', icon('#1E3A5F', '#0B1B2E', `<circle cx="58" cy="40" r="12" stroke="${W}" stroke-width="6" fill="none"/><circle cx="36" cy="62" r="8" fill="${W}"/><path d="M42 58l12-12" stroke="${W}" stroke-width="6"/>`)],
  ['Notion', icon('#F5F5F4', '#D6D3D1', `<rect x="26" y="22" width="44" height="52" rx="6" fill="${W}" stroke="#1C1917" stroke-width="4"/><path d="M38 62V36l20 26V36" stroke="#1C1917" stroke-width="5" fill="none" stroke-linejoin="round"/>`)],
  ['Spotify', icon('#4ADE80', '#15803D', `<path d="M28 40q20-8 40 2M31 51q16-6 33 2M34 61q13-4 25 2" stroke="${W}" stroke-width="5" fill="none" stroke-linecap="round"/>`)],
] as const;

const DAY = 86_400_000;
const now = Date.now();

function makeItems(): Item[] {
  return ICONS.map(([name, ic], i) => ({
    id: 'm' + i,
    name,
    launchPath: `C:\\Program Files\\${name}\\${name.toLowerCase()}.exe`,
    target: `c:\\program files\\${name.toLowerCase()}\\${name.toLowerCase()}.exe`,
    args: '',
    workdir: '',
    kind: 'app',
    icon: ic,
    addedAt: now - (i + 2) * 6 * DAY,
    lastLaunch: i === 6 ? 0 : now - (i * 37 + 4) * 60_000,
    launchCount: 40 - i * 4,
  }));
}

export function mock(): Api {
  const label = (q.get('w') ?? 'rail') as Api['label'];
  const settings: Settings = {
    theme: (q.get('theme') as Theme) ?? 'aurora',
    side: (q.get('side') as Side) ?? 'right',
    autostart: true,
    edgeReveal: true,
    hoverDelayMs: Math.min(Number(q.get('delay') ?? 900), 3000),
    hideDelayMs: 1200,
    iconSize: Number(q.get('size') ?? 36),
    pinned: q.get('pinned') === '1',
    reduceMotion: false,
    railScale: (q.get('scale') ?? 'compact') as Settings['railScale'],
    showMeters: q.get('meters') !== '0',
  };
  const items = q.get('empty') === '1' ? [] : makeItems();
  let cfg: Config = { version: 1, settings, items };
  // preset runtime: Chrome ×3, Telegram ×1, Terminal hung, Code ×1
  const base: Record<string, Partial<ItemRt>> = {
    m0: { windows: 1, procs: 9, cpu: 4.2, mem: 812e6, uptime: 5260 },
    m1: { windows: 3, procs: 24, cpu: 11.8, mem: 2.31e9, uptime: 13_400 },
    m2: { windows: 1, procs: 1, cpu: 0.3, mem: 186e6, uptime: 40_100 },
    m4: { windows: 1, procs: 2, cpu: 0, mem: 42e6, uptime: 620, hung: true },
  };
  const subs: Record<string, Set<(p: never) => void>> = {};
  const on = <T,>(n: string, cb: (p: T) => void) => {
    (subs[n] ??= new Set()).add(cb as never);
    return () => subs[n].delete(cb as never);
  };
  const emit = (n: string, p: unknown) => subs[n]?.forEach((cb) => (cb as (x: unknown) => void)(p));
  let t = 0;
  const rt = (): RuntimePayload => {
    t++;
    const out: Record<string, ItemRt> = {};
    for (const it of cfg.items) {
      const b = base[it.id];
      const z: ItemRt = { windows: 0, procs: 0, hung: false, minimized: false, launching: false, cpu: 0, mem: 0, read: 0, write: 0, uptime: 0 };
      if (b) {
        const wob = Math.sin(t / 2 + it.id.charCodeAt(1)) * 0.5 + 0.5;
        out[it.id] = { ...z, ...b, cpu: (b.cpu ?? 0) * (0.6 + wob), read: it.id === 'm1' && t % 4 === 0 ? 3.2e6 : 12e3 * wob, write: 4e3 * wob, uptime: (b.uptime ?? 0) + t } as ItemRt;
      } else out[it.id] = z;
    }
    const proc = (pid: number, ago: number) => ({ pid, startTime: Math.floor(Date.now() / 1000) - ago });
    const sysTopAll: SysApp[] = [
      { id: 'c:\\program files\\chrome\\chrome.exe', name: 'chrome.exe', cpu: 8.2, mem: 2.3e9, pid: 4200, procs: [proc(4200, 13400), proc(4201, 13390), proc(4260, 8000)], windows: 3, closable: true },
      { id: 'c:\\program files\\code\\code.exe', name: 'code.exe', cpu: 3.1, mem: 812e6, pid: 5100, procs: [proc(5100, 5260)], windows: 1, closable: true },
      // deliberately CPU-heavy, RAM-light: proves the CPU tab is a real
      // system-wide ranking, not a reorder of the RAM five
      { id: 'c:\\tools\\compress\\compress.exe', name: 'compress.exe', cpu: 41.5, mem: 61e6, pid: 9100, procs: [proc(9100, 640)], windows: 0, closable: true },
      { id: 'c:\\program files\\steam\\steam.exe', name: 'steam.exe', cpu: 0.4, mem: 186e6, pid: 6200, procs: [proc(6200, 40100)], windows: 1, closable: true },
      { id: 'c:\\windows\\system32\\svchost.exe', name: 'svchost.exe', cpu: 2.8, mem: 42e6, pid: 800, procs: [proc(800, 90000)], windows: 0, closable: false },
      { id: 'c:\\program files\\telegram\\telegram.exe', name: 'telegram.exe', cpu: 0.1, mem: 95e6, pid: 7300, procs: [proc(7300, 1200)], windows: 1, closable: true },
      // two more eligible apps: more than five rows of candidates, so the UI
      // must cap itself at exactly five and re-rank them live
      { id: 'c:\\program files\\discord\\discord.exe', name: 'discord.exe', cpu: 0.9, mem: 300e6, pid: 8400, procs: [proc(8400, 3100)], windows: 1, closable: true },
      { id: 'C:\\Windows\\explorer.exe', name: 'explorer.exe', cpu: 1.9, mem: 120e6, pid: 2100, procs: [proc(2100, 200000)], windows: 2, closable: true },
    ];
    // `cold=1` emits two empty frames first: a popup opened before the first
    // sample must render the first arriving rows without a tab switch
    const cold = q.get('cold') === '1';
    // Event-sequence fixtures for the interaction regression QA (qa/visual.mjs).
    // Each is OFF unless its query flag is set, so every pre-existing scene is
    // unchanged; they let a real Playwright event sequence drive a genuine
    // disappearance / new-leader / all-vanish / never-sampled transition.
    const nosample = q.get('nosample') === '1';
    const vanish = q.get('vanish') === '1';
    const vanishall = q.get('vanishall') === '1';
    const shift = q.get('shift') === '1';
    let sysTop: SysApp[] = q.get('sysmon') === 'empty' || (cold && t <= 2) ? [] : sysTopAll;
    if (vanish && t >= 4) sysTop = sysTopAll.filter((a) => a.name !== 'code.exe');
    if (vanishall && t >= 4) sysTop = [];
    if (shift && t >= 4) {
      const leader: SysApp = { id: 'c:\\tools\\miner\\miner.exe', name: 'miner.exe', cpu: 97.5, mem: 500e6, pid: 9900, procs: [{ pid: 9900, startTime: Math.floor(Date.now() / 1000) - 10 }], windows: 0, closable: true };
      sysTop = [leader, ...sysTopAll.filter((a) => a.name !== 'explorer.exe')];
    }
    // `ts` is unix MILLISECONDS — the same unit the backend emits
    // (`config::now`). `stale=1` freezes the stamp 60 s in the past:
    // carry-forward frames keep flowing but must be marked stale. `recent=1`
    // keeps the sample just under the 6 s threshold: must NOT be stale.
    // `nosample=1` reports a never-sampled frame (ts 0): loading placeholders,
    // never fabricated zero totals.
    const ts = nosample ? 0 : q.get('stale') === '1' ? Date.now() - 60_000 : q.get('recent') === '1' ? Date.now() - 4_000 : Date.now();
    const sys = nosample ? { cpu: 0, memUsed: 0, memTotal: 0 } : { cpu: 18 + Math.sin(t / 3) * 6, memUsed: 11.2e9, memTotal: 31.9e9 };
    return { items: out, sys, sysTop, ts, seq: t };
  };
  if (typeof window !== 'undefined') {
    setInterval(() => emit('runtime', rt()), 1000);
    setTimeout(() => emit('runtime', rt()), 30);
    if (q.get('savefail') === '1') setTimeout(() => emit('save-error', 'диск недоступен'), 120);
    if (label === 'rail' && q.get('hidden') !== '1') setTimeout(() => emit('rail', { action: 'show', seq: 1 }), 60);
    if (label === 'flyout') setTimeout(() => emit('flyout', { action: 'show', seq: 1, kind: q.get('kind') ?? 'metrics', id: q.get('id') ?? 'm1', side: settings.side }), 60);
  }
  const setCfg = (c: Config) => {
    cfg = c;
    emit('config', cfg);
  };
  const api: Api = {
    label,
    // Browser harness has no backend to talk to; keep the signature so the
    // shared `Api` contract compiles, but drop the message on the floor.
    diagLog: () => {},
    snapshot: async () => ({ config: cfg, visible: true }),
    addPaths: async (paths, index) => {
      const add = paths.map((p, i) => ({ ...makeItems()[i % 8], id: 'n' + Date.now() + i, name: p.split(/[\\/]/).pop() ?? p }));
      const list = [...cfg.items];
      list.splice(index ?? list.length, 0, ...add);
      setCfg({ ...cfg, items: list });
    },
    pickFiles: async () => {},
    reorder: async (ids) => setCfg({ ...cfg, items: ids.map((id) => cfg.items.find((i) => i.id === id)!).filter(Boolean) }),
    removeItem: async (id) => setCfg({ ...cfg, items: cfg.items.filter((i) => i.id !== id) }),
    launch: async (id) => {
      const b = (base[id] ??= { windows: 0, procs: 0, mem: 120e6, cpu: 2, uptime: 0 });
      const before = b.windows ?? 0;
      b.windows = 0;
      base[id] = { ...b, launching: true, windows: before };
      emit('runtime', rt());
      setTimeout(() => {
        base[id] = { ...base[id], launching: false, windows: before + 1, procs: (b.procs ?? 0) + 1 };
        emit('runtime', rt());
      }, 1400);
      emit('launch', { id, ok: true });
    },
    activate: async (id) => {
      if (!(base[id]?.windows ?? 0)) return api.launch(id);
    },
    itemMenu: async () => {},
    setSettings: async (s) => setCfg({ ...cfg, settings: s }),
    setHitRect: async () => {},
    dragActive: async () => {},
    railReady: async () => {},
    requestHide: async () => emit('rail', { action: 'hide', seq: 1 }),
    railHidden: async () => {
      setTimeout(() => emit('rail', { action: 'show', seq: 2 }), 1200);
    },
    showFlyout: async (req) => emit('flyout', { action: 'show', seq: Date.now() % 100000, kind: req.kind, id: req.id, side: settings.side }),
    sysClose: async (req): Promise<SysCloseResult> => {
      // deterministic QA: `close=refused|unavailable` forces the outcome,
      // protected rows are always refused, everything else dispatches
      const forced = q.get('close');
      if (req.id.includes('svchost')) return { status: 'refused', dispatched: 0, windows: 0, reason: 'системный или защищённый процесс' };
      if (forced === 'refused') return { status: 'refused', dispatched: 0, windows: 0, reason: 'системный или защищённый процесс' };
      if (forced === 'unavailable') return { status: 'unavailable', dispatched: 0, windows: 0, reason: 'приложение закрыто или перезапущено' };
      return { status: 'dispatched', dispatched: 2, windows: 2, reason: 'отправлен запрос на закрытие' };
    },
    hideFlyout: async () => emit('flyout', { action: 'hide', seq: Date.now() % 100000 }),
    flyoutHidden: async () => {},
    openSettings: async () => {},
    closeSettings: async () => {},
    quit: async () => {},
    onConfig: (cb) => on('config', cb),
    onRuntime: (cb) => on('runtime', cb),
    onRail: (cb) => on('rail', cb),
    onHover: (cb) => on('rail-hover', cb),
    onFlyout: (cb) => on('flyout', cb),
    onLaunch: (cb) => on('launch', cb),
    onMenuClosed: (cb) => on('menu-closed', cb),
    onDrop: (cb) => on('drop', cb),
    onSaveError: (cb) => on('save-error', cb),
    startDrag: () => {},
  };
  // Browser-side harness for the external-drop path: the real OLE events only
  // exist inside Tauri, so visual QA drives them through window.__railDrop.
  if (typeof window !== 'undefined') {
    (window as unknown as { __railDrop: (e: unknown) => void }).__railDrop = (e) => emit('drop', e);
  }
  return api;
}
