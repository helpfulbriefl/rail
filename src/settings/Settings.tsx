import { motion } from 'motion/react';
import { ArrowDown, ArrowLeft, ArrowRight, ArrowUp, LogOut, X } from 'lucide-react';
import type { ReactNode } from 'react';
import { HOVER_MAX_MS, type RailScale, type Settings, type Side, type Theme } from '../bridge/types';
import { Lamp } from '../ui/Lamp';
import type { LampState } from '../ui/status';
import { useApp } from '../ui/useApp';

const SPRING = { type: 'spring', stiffness: 500, damping: 36 } as const;

const THEMES: { id: Theme; name: string }[] = [
  { id: 'classic-light', name: 'Классика' },
  { id: 'classic-dark', name: 'Классика' },
  { id: 'porcelain', name: 'Фарфор' },
  { id: 'aurora', name: 'Аврора' },
];

export function SettingsView() {
  const { api, config } = useApp();
  const s = config.settings;
  const set = (p: Partial<Settings>) => void api.setSettings({ ...s, ...p });
  const holdS = Math.min(s.hoverDelayMs, HOVER_MAX_MS) / 1000;

  return (
    <div className="set-stage">
      <motion.div className="set" initial={{ opacity: 0, scale: 0.97, y: 8 }} animate={{ opacity: 1, scale: 1, y: 0 }} transition={SPRING}>
        <header className="set-head" data-tauri-drag-region>
          <img src="/logo.png" alt="" className="set-logo" data-tauri-drag-region />
          <span className="set-title" data-tauri-drag-region>
            Rail
          </span>
          <span className="set-ver" data-tauri-drag-region>
            1.0
          </span>
          <span className="grow" data-tauri-drag-region />
          <button type="button" className="micro" aria-label="Закрыть" onClick={() => void api.closeSettings()}>
            <X size={14} />
          </button>
        </header>

        <Section k="Тема">
          <div className="themes">
            {THEMES.map((t) => (
              <button key={t.id} type="button" className="theme" aria-pressed={s.theme === t.id} onClick={() => set({ theme: t.id })}>
                {s.theme === t.id && <motion.span layoutId="theme-ring" className="theme-ring" transition={SPRING} />}
                <span className="theme-prev" data-theme={t.id}>
                  <span className="tp-rail">
                    <i />
                    <i data-on="" />
                    <i />
                  </span>
                </span>
                <span className="theme-name">
                  {t.name}
                  {t.id.startsWith('classic') && <small>{t.id.endsWith('light') ? 'светлая' : 'тёмная'}</small>}
                </span>
              </button>
            ))}
          </div>
        </Section>

        <Section k="Положение">
          <Segmented<Side>
            id="side"
            value={s.side}
            onChange={(side) => set({ side })}
            options={[
              { v: 'left', n: <ArrowLeft size={14} />, l: 'Слева' },
              { v: 'top', n: <ArrowUp size={14} />, l: 'Сверху' },
              { v: 'bottom', n: <ArrowDown size={14} />, l: 'Снизу' },
              { v: 'right', n: <ArrowRight size={14} />, l: 'Справа' },
            ]}
          />
        </Section>

        <Section k="Поведение">
          <div className="toggles">
            <Toggle label="Запуск с Windows" on={s.autostart} onChange={(autostart) => set({ autostart })} />
            <Toggle label="Показ у края экрана" on={s.edgeReveal} onChange={(edgeReveal) => set({ edgeReveal })} />
            <Toggle label="Закрепить панель" on={s.pinned} onChange={(pinned) => set({ pinned })} />
            <Toggle label="Меньше анимации" on={s.reduceMotion} onChange={(reduceMotion) => set({ reduceMotion })} />
            <Toggle label="Счётчики CPU / RAM" on={s.showMeters !== false} onChange={(showMeters) => set({ showMeters })} />
          </div>
          <div className="rows">
            {/* capped at 3s — longer holds felt like the flyout was broken */}
            <Slider label="Метрики через" unit="с" min={0.3} max={HOVER_MAX_MS / 1000} step={0.1} value={holdS} onChange={(v) => set({ hoverDelayMs: Math.round(v * 1000) })} />
            <Slider label="Скрытие через" unit="с" min={0.5} max={5} step={0.5} value={s.hideDelayMs / 1000} onChange={(v) => set({ hideDelayMs: Math.round(v * 1000) })} />
            <div className="row">
              <span className="row-l">Иконки</span>
              <Segmented<number>
                id="size"
                value={s.iconSize}
                onChange={(iconSize) => set({ iconSize })}
                options={[
                  { v: 30, n: 'S', l: 'Маленькие' },
                  { v: 36, n: 'M', l: 'Средние' },
                  { v: 44, n: 'L', l: 'Большие' },
                ]}
              />
            </div>
            <div className="row">
              <span className="row-l">Размер панели</span>
              <Segmented<RailScale>
                id="scale"
                value={s.railScale ?? 'compact'}
                onChange={(railScale) => set({ railScale })}
                options={[
                  { v: 'mini', n: 'Мини', l: 'Только иконки' },
                  { v: 'compact', n: 'Компакт', l: 'Компактный' },
                  { v: 'normal', n: 'Обычный', l: 'Обычный' },
                ]}
              />
            </div>
          </div>
        </Section>

        <Section k="Светофор">
          <div className="legend">
            {(
              [
                ['run', 'работает'],
                ['load', 'нагрузка'],
                ['hung', 'завис'],
                ['io', 'диск'],
                ['mem', 'память'],
              ] as [LampState, string][]
            ).map(([st, l]) => (
              <span key={st} className="legend-i">
                <Lamp state={st} size={7} />
                {l}
              </span>
            ))}
          </div>
        </Section>

        <footer className="set-foot">
          <span className="hint">ЛКМ — фокус / свернуть · Shift / колесо — новое окно · ПКМ — меню</span>
          <button type="button" className="btn-quit" onClick={() => void api.quit()}>
            <LogOut size={13} />
            Выход
          </button>
        </footer>
      </motion.div>
    </div>
  );
}

function Section({ k, children }: { k: string; children: ReactNode }) {
  return (
    <section className="sec">
      <h2 className="sec-k">{k}</h2>
      {children}
    </section>
  );
}

function Segmented<T extends string | number>({ id, value, onChange, options }: { id: string; value: T; onChange: (v: T) => void; options: { v: T; n: ReactNode; l: string }[] }) {
  return (
    <div className="seg" role="radiogroup">
      {options.map((o) => (
        <button key={String(o.v)} type="button" role="radio" aria-checked={value === o.v} aria-label={o.l} title={o.l} className="seg-i" onClick={() => onChange(o.v)}>
          {value === o.v && <motion.span layoutId={`seg-${id}`} className="seg-pill" transition={SPRING} />}
          <span className="seg-n">{o.n}</span>
        </button>
      ))}
    </div>
  );
}

function Toggle({ label, on, onChange }: { label: string; on: boolean; onChange: (v: boolean) => void }) {
  return (
    <button type="button" className="tog" role="switch" aria-checked={on} onClick={() => onChange(!on)}>
      <span className="row-l">{label}</span>
      <span className="sw" data-on={on ? '' : undefined}>
        <motion.span className="sw-k" layout transition={SPRING} />
      </span>
    </button>
  );
}

function Slider({ label, unit, min, max, step, value, onChange }: { label: string; unit: string; min: number; max: number; step: number; value: number; onChange: (v: number) => void }) {
  const pct = ((value - min) / (max - min)) * 100;
  return (
    <label className="row">
      <span className="row-l">{label}</span>
      <span className="sl">
        <input type="range" min={min} max={max} step={step} value={value} onChange={(e) => onChange(Number(e.target.value))} style={{ ['--pct' as string]: `${pct}%` }} />
        <span className="sl-v">
          {value.toFixed(value % 1 ? 1 : 0)}
          <small>{unit}</small>
        </span>
      </span>
    </label>
  );
}
