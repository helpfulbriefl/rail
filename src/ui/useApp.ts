import { createContext, useContext, useEffect, useRef, useState } from 'react';
import { getApi, type Api } from '../bridge/api';
import type { Config, RuntimePayload } from '../bridge/types';

export interface AppState {
  api: Api;
  config: Config;
  rt: RuntimePayload;
  /** rolling history per item: cpu %, mem bytes, disk B/s */
  hist: Record<string, { cpu: number[]; mem: number[]; io: number[] }>;
  /** rolling system totals for the mini monitor charts */
  sysHist: { cpu: number[]; memPct: number[] };
  /** persistence failure surfaced by the backend, null while everything saved */
  saveError: string | null;
}

const empty: RuntimePayload = { items: {}, sys: { cpu: 0, memUsed: 0, memTotal: 1 }, sysTop: [], ts: 0, seq: 0 };
const SYS_HIST_MAX = 60;

export function useBoot(): AppState | null {
  const [api, setApi] = useState<Api | null>(null);
  const [config, setConfig] = useState<Config | null>(null);
  const [rt, setRt] = useState<RuntimePayload>(empty);
  const [saveError, setSaveError] = useState<string | null>(null);
  const hist = useRef<AppState['hist']>({});
  const sysHist = useRef<AppState['sysHist']>({ cpu: [], memPct: [] });
  /** last sample timestamp accepted into history */
  const lastTs = useRef(0);
  useEffect(() => {
    let off: (() => void)[] = [];
    void getApi().then(async (a) => {
      setApi(a);
      off = [
        a.onConfig(setConfig),
        a.onSaveError(setSaveError),
        a.onRuntime((p) => {
          // A frame is only real data when its sample stamp advanced; the
          // backend repeats the last known state (carry-forward) between
          // metric ticks, and pushing those repeats made the sparklines
          // plateau and the charts lie about "continuous readings".
          const fresh = p.ts > 0 && p.ts !== lastTs.current;
          lastTs.current = Math.max(lastTs.current, p.ts);
          if (fresh) {
            for (const [id, r] of Object.entries(p.items)) {
              if (!r.windows && !r.procs) continue;
              // A frame with a window but no sampled process carries no real
              // numbers — pushing its zeros made the sparklines sawtooth.
              if (!r.procs) continue;
              const h = (hist.current[id] ??= { cpu: [], mem: [], io: [] });
              h.cpu.push(r.cpu);
              h.mem.push(r.mem);
              h.io.push(r.read + r.write);
              for (const k of ['cpu', 'mem', 'io'] as const) if (h[k].length > SYS_HIST_MAX) h[k].shift();
            }
            sysHist.current.cpu.push(p.sys.cpu);
            sysHist.current.memPct.push((p.sys.memUsed / Math.max(p.sys.memTotal, 1)) * 100);
            for (const k of ['cpu', 'memPct'] as const) {
              if (sysHist.current[k].length > SYS_HIST_MAX) sysHist.current[k].shift();
            }
          }
          setRt(p);
        }),
      ];
      const s = await a.snapshot();
      setConfig(s.config);
    });
    return () => off.forEach((f) => f());
  }, []);
  useEffect(() => {
    if (config) document.documentElement.dataset.theme = config.settings.theme;
  }, [config?.settings.theme]); // eslint-disable-line react-hooks/exhaustive-deps
  // Cleanup hist for removed items
  useEffect(() => {
    if (!config) return;
    const ids = new Set(config.items.map((i) => i.id));
    for (const k of Object.keys(hist.current)) {
      if (!ids.has(k)) delete hist.current[k];
    }
  }, [config?.items]); // eslint-disable-line react-hooks/exhaustive-deps
  if (!api || !config) return null;
  return { api, config, rt, hist: hist.current, sysHist: sysHist.current, saveError };
}

export const AppCtx = createContext<AppState | null>(null);
export function useApp(): AppState {
  const v = useContext(AppCtx);
  if (!v) throw new Error('AppCtx missing');
  return v;
}
