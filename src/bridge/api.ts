/**
 * Single seam between UI and OS. UI never touches Tauri directly.
 * In a plain browser (design previews / visual QA) a mock backend is used.
 */
import type { Config, DropEvent, FlyoutCmd, LaunchResult, RailCmd, Rect, RuntimePayload, Settings } from './types';
import { mock } from './mock';

type Unlisten = () => void;

export interface Api {
  label: 'rail' | 'flyout' | 'settings';
  snapshot(): Promise<{ config: Config; visible: boolean }>;
  addPaths(paths: string[], index?: number): Promise<void>;
  pickFiles(): Promise<void>;
  reorder(ids: string[]): Promise<void>;
  removeItem(id: string): Promise<void>;
  launch(id: string): Promise<void>;
  activate(id: string): Promise<void>;
  itemMenu(id: string): Promise<void>;
  setSettings(s: Settings): Promise<void>;
  setHitRect(r: Rect): Promise<void>;
  /** Tell the backend a drag is in flight (keeps the window region open). */
  dragActive(active: boolean): Promise<void>;
  railReady(): Promise<void>;
  requestHide(): Promise<void>;
  railHidden(seq: number): Promise<void>;
  /** Phase-0 telemetry sink; a no-op unless the backend runs with RAIL_DIAG=1. */
  diagLog(msg: string): void;
  showFlyout(req: { kind: 'info' | 'metrics' | 'sys'; id: string; width: number; height: number } & Rect): Promise<void>;
  sysClose(exe: string): Promise<void>;
  hideFlyout(): Promise<void>;
  flyoutHidden(seq: number): Promise<void>;
  openSettings(): Promise<void>;
  closeSettings(): Promise<void>;
  quit(): Promise<void>;
  onConfig(cb: (c: Config) => void): Unlisten;
  onRuntime(cb: (r: RuntimePayload) => void): Unlisten;
  onRail(cb: (c: RailCmd) => void): Unlisten;
  onHover(cb: (h: boolean) => void): Unlisten;
  onFlyout(cb: (c: FlyoutCmd) => void): Unlisten;
  onLaunch(cb: (r: LaunchResult) => void): Unlisten;
  onMenuClosed(cb: () => void): Unlisten;
  onDrop(cb: (e: DropEvent) => void): Unlisten;
  startDrag(): void;
}

export const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

async function tauriApi(): Promise<Api> {
  const { invoke } = await import('@tauri-apps/api/core');
  const { listen } = await import('@tauri-apps/api/event');
  const { getCurrentWebviewWindow } = await import('@tauri-apps/api/webviewWindow');
  const win = getCurrentWebviewWindow();
  const on = <T,>(name: string, cb: (p: T) => void): Unlisten => {
    let un: Unlisten | null = null;
    let dead = false;
    void listen<T>(name, (e) => cb(e.payload)).then((u) => (dead ? u() : (un = u)));
    return () => {
      dead = true;
      un?.();
    };
  };
  const diag = (msg: string) => {
    void invoke('diag_log', { msg }).catch(() => {});
  };
  return {
    label: win.label as Api['label'],
    // never awaited and never allowed to reject: diagnostics must not be able to
    // disturb a drag that is in flight
    diagLog: diag,
    snapshot: () => invoke('snapshot'),
    addPaths: (paths, index) => invoke('add_paths', { paths, index }),
    pickFiles: () => invoke('pick_files'),
    reorder: (ids) => invoke('reorder', { ids }),
    removeItem: (id) => invoke('remove_item', { id }),
    launch: (id) => invoke('launch', { id }),
    activate: (id) => invoke('activate', { id }),
    itemMenu: (id) => invoke('item_menu', { id }),
    setSettings: (settings) => invoke('set_settings', { settings }),
    setHitRect: (r) => invoke('set_hit_rect', { r }),
    dragActive: (active) => invoke('drag_active', { active }),
    railReady: () => invoke('rail_ready'),
    requestHide: () => invoke('request_hide'),
    railHidden: (seq) => invoke('rail_hidden', { seq }),
    showFlyout: (req) => invoke('show_flyout', { req }),
    sysClose: (exe) => invoke('sys_close', { exe }),
    hideFlyout: () => invoke('hide_flyout'),
    flyoutHidden: (seq) => invoke('flyout_hidden', { seq }),
    openSettings: () => invoke('open_settings'),
    closeSettings: () => invoke('close_settings'),
    quit: () => invoke('quit'),
    onConfig: (cb) => on('config', cb),
    onRuntime: (cb) => on('runtime', cb),
    onRail: (cb) => on('rail', cb),
    onHover: (cb) => on('rail-hover', cb),
    onFlyout: (cb) => on('flyout', cb),
    onLaunch: (cb) => on('launch', cb),
    onMenuClosed: (cb) => on('menu-closed', cb),
    onDrop: (cb) => {
      let un: Unlisten | null = null;
      let dead = false;
      const ratio = () => window.devicePixelRatio || 1;
      // `over` fires continuously while a file hovers the window, so the log is
      // rate-limited to one line per 250 ms — enough to prove the drag reached
      // the webview, without drowning the region/lease lines it must be read with.
      let lastOver = 0;
      // Tauri reports drag positions in *physical* pixels relative to the
      // window; the DOM works in CSS pixels. Dividing by devicePixelRatio was
      // right, but the value has to be read per event (the rail can move to a
      // monitor with a different scale factor mid-drag).
      void win
        .onDragDropEvent((e) => {
          const p = e.payload;
          if (p.type === 'leave') {
            diag('drop:leave');
            return cb({ type: 'leave' });
          }
          const r = ratio();
          const x = p.position.x / r;
          const y = p.position.y / r;
          if (p.type === 'over') {
            const now = performance.now();
            if (now - lastOver > 250) {
              lastOver = now;
              diag(`drop:over x=${x.toFixed(0)} y=${y.toFixed(0)} dpr=${r}`);
            }
            cb({ type: 'over', x, y });
          } else {
            diag(`drop:${p.type} n=${p.paths.length} x=${x.toFixed(0)} y=${y.toFixed(0)}`);
            cb({ type: p.type, paths: p.paths, x, y });
          }
        })
        .then((u) => (dead ? u() : (un = u)));
      return () => {
        dead = true;
        un?.();
      };
    },
    startDrag: () => void win.startDragging(),
  };
}

let cached: Promise<Api> | null = null;
export function getApi(): Promise<Api> {
  cached ??= isTauri ? tauriApi() : Promise.resolve(mock());
  return cached;
}
