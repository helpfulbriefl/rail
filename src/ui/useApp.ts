import { createContext, useContext, useEffect, useRef, useState } from 'react';
import { getApi, type Api } from '../bridge/api';
import type { Config, RuntimePayload } from '../bridge/types';

export interface AppState {
  api: Api;
  config: Config;
  rt: RuntimePayload;
  /** rolling history per item: cpu %, mem bytes, disk B/s */
  hist: Record<string, { cpu: number[]; mem: number[]; io: number[] }>;
}

const empty: RuntimePayload = { items: {}, sys: { cpu: 0, memUsed: 0, memTotal: 1 } };

export function useBoot(): AppState | null {
  const [api, setApi] = useState<Api | null>(null);
  const [config, setConfig] = useState<Config | null>(null);
  const [rt, setRt] = useState<RuntimePayload>(empty);
  const hist = useRef<AppState['hist']>({});
  useEffect(() => {
    let off: (() => void)[] = [];
    void getApi().then(async (a) => {
      setApi(a);
      off = [
        a.onConfig(setConfig),
        a.onRuntime((p) => {
          for (const [id, r] of Object.entries(p.items)) {
            if (!r.windows && !r.procs) continue;
            // A frame with a window but no sampled process carries no real
            // numbers — pushing its zeros made the sparklines sawtooth.
            if (!r.procs) continue;
            const h = (hist.current[id] ??= { cpu: [], mem: [], io: [] });
            h.cpu.push(r.cpu);
            h.mem.push(r.mem);
            h.io.push(r.read + r.write);
            for (const k of ['cpu', 'mem', 'io'] as const) if (h[k].length > 60) h[k].shift();
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
  return { api, config, rt, hist: hist.current };
}

export const AppCtx = createContext<AppState | null>(null);
export function useApp(): AppState {
  const v = useContext(AppCtx);
  if (!v) throw new Error('AppCtx missing');
  return v;
}
