export type Theme = 'classic-light' | 'classic-dark' | 'porcelain' | 'aurora' | 'glass' | 'studio';
export type Side = 'right' | 'left' | 'bottom' | 'top';
export type RailScale = 'mini' | 'compact' | 'normal';

export interface Settings {
  theme: Theme;
  side: Side;
  autostart: boolean;
  edgeReveal: boolean;
  /** hold before the flyout appears; hard-capped at HOVER_MAX_MS */
  hoverDelayMs: number;
  hideDelayMs: number;
  iconSize: number;
  pinned: boolean;
  reduceMotion: boolean;
  railScale: RailScale;
  showMeters: boolean;
}

/** Longest allowed flyout hold (ms). Mirrors `config::HOVER_MAX_MS` in Rust. */
export const HOVER_MAX_MS = 3000;

/** Cross-axis multiplier applied to `iconSize` for each rail scale. */
export const SCALE_MUL: Record<RailScale, number> = { mini: 0.56, compact: 0.72, normal: 1 };

export interface Item {
  id: string;
  name: string;
  launchPath: string;
  target: string;
  args: string;
  workdir: string;
  kind: 'app' | 'file' | 'folder';
  icon: string;
  addedAt: number;
  lastLaunch: number;
  launchCount: number;
}

export interface Config {
  version: number;
  settings: Settings;
  items: Item[];
}

export interface ItemRt {
  windows: number;
  procs: number;
  hung: boolean;
  minimized: boolean;
  launching: boolean;
  cpu: number;
  mem: number;
  read: number;
  write: number;
  uptime: number;
}

export interface SysRt {
  cpu: number;
  memUsed: number;
  memTotal: number;
}

/** One process behind an aggregated row. `startTime` makes the identity
 *  survive PID recycling: the close request is revalidated against it. */
export interface SysProc {
  pid: number;
  /** whole seconds since the UNIX epoch (process creation time) */
  startTime: number;
}

/** One aggregated application in the system-wide monitor. */
export interface SysApp {
  /** full exe path, lower-case — the frozen identity */
  id: string;
  name: string;
  /** % of total logical CPU (all cores = 100%) */
  cpu: number;
  /** working-set bytes (all processes under this exe summed) */
  mem: number;
  /** representative PID (lowest) for display only */
  pid: number;
  /** every process behind this row — the close target set */
  procs: SysProc[];
  /** validated top-level windows this row can ask to close */
  windows: number;
  /** false when the exe belongs to a system / critical process */
  closable: boolean;
}

export interface RuntimePayload {
  items: Record<string, ItemRt>;
  sys: SysRt;
  sysTop: SysApp[];
  /** unix milliseconds of the newest sample this frame reflects (0 = never) */
  ts: number;
  /** monotonic frame counter; duplicates (same ts) must not extend history */
  seq: number;
}

/** Close request bound to the frozen snapshot identity, not a bare exe path. */
export interface SysCloseReq {
  id: string;
  procs: SysProc[];
}

export interface SysCloseResult {
  status: 'dispatched' | 'refused' | 'unavailable';
  /** WM_CLOSE messages posted (not: processes confirmed exited) */
  dispatched: number;
  windows: number;
  reason: string;
}

export interface RailCmd {
  action: 'show' | 'hide';
  seq: number;
}

export interface FlyoutCmd {
  action: 'show' | 'hide';
  seq: number;
  kind?: 'info' | 'metrics' | 'sys';
  id?: string;
  side?: Side;
}

export interface LaunchResult {
  id: string;
  ok: boolean;
  error?: string;
}

export type DropEvent =
  | { type: 'enter'; paths: string[]; x: number; y: number }
  | { type: 'over'; x: number; y: number }
  | { type: 'drop'; paths: string[]; x: number; y: number }
  | { type: 'leave' };

export interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}
