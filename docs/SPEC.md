# Rail — спецификация v1.0

## 1. Бриф
**Проблема:** панель задач Windows не показывает состояние и ресурсы программ, а Диспетчер задач — отдельное тяжёлое окно.
**Решение:** трей‑приложение с выезжающей рельсой: быстрый запуск + живой статус каждой программы + метрики по удержанию курсора.
**Принципы:** меньше текста, больше цифр · микрокнопки · бейджи‑светофоры · визуал отделён от логики · анимация на каждом переходе состояния.

## 2. Стек (проверен по реестрам npm / crates.io и докам, октябрь 2026)

| Слой | Выбор | Версия | Почему | Отклонено |
|---|---|---|---|---|
| Оболочка | **Tauri 2** (Rust + WebView2) | 2.12 | exe ≈ 4 МБ, трей, мультиокна, прозрачность, click‑through | WinForms (нет GPU‑анимаций), Electron (100+ МБ), GPUI (нет CSS, молодая экосистема на Win) |
| WinAPI | `windows` crate | 0.62 | EnumWindows, IsHungAppWindow, ShellExecuteEx, IShellLink, IShellItemImageFactory | — |
| Метрики | `sysinfo` | 0.39 | CPU / RAM / диск по процессу одним системным вызовом | WMI (медленно) |
| UI | React | 19.3 | — | — |
| Анимации | **Motion** (ex Framer Motion) | 14 | layout‑морф, Reorder, spring‑физика, AnimatePresence | anime.js 4 (нет layout/reorder в React), react‑bits — взят как паттерн Dock |
| Иконки UI | lucide‑react | 1.51 | микро‑иконки 13–14 px | developer‑icons — не нужен: реальные иконки извлекаются из exe |
| Шрифты | Geist Variable + Geist Mono | 5.3 | цифры моно, tabular‑nums; классические темы — Segoe UI Variable | — |
| CSS | токены CSS‑переменных + **stylelint‑config‑standard** | 17 | одна таблица токенов на тему, линт в CI | Tailwind — лишний слой для 4 окон |
| Визуальный QA | Playwright + Chromium + mock‑бэкенд | — | скриншоты всех окон × тем + проверки overflow / overlap | — |
| Сборка | GitHub Actions `windows-latest` / cargo‑xwin | — | push → exe | — |

Из референсов: AtomicCRM — блочная сетка и минимализм карточек; taste‑skill — правила «меньше шума, один акцент»; Infographic / ThreeUI — слишком тяжёлые для трея, не используются.

## 3. Архитектура
```
Windows ──► tracker.rs (поток)                 main.rs (команды, трей, окна)
            · курсор 30 Гц: hit‑test, край      · очередь запусков (1 поток, ShellExecuteEx)
            · окна 0.7–2 с: EnumWindows         · show/hide с seq, позиционирование по work area
            · метрики 1 Гц (только если видно)   · нативные меню (трей, ПКМ)
                 │ event "runtime"                       ▲ invoke
                 ▼                                       │
          bridge/api.ts  ◄──── единственный шов UI ↔ ОС (в браузере → mock.ts)
                 │
     rail/ (Rail, Tile, magnify)   flyout/ (Info, Metrics)   settings/
```
Окна: `rail` (прозрачное, topmost, click‑through вне капсулы), `flyout` (только показ, всегда сквозное), `settings`.

## 4. Машина состояний иконки
`Pinned → (клик) Launching[блок + янтарная мигающая лампа, подпрыгивание] → Running×1[дышащая лампа] ⇄ Running×N[стопка + цифра] → (окна закрыты) Pinned[обратный морф]`.
Сбой запуска / отмена UAC → тряска. Нет нового окна за 10 с → обратно в Pinned.
Лампа: hung > вспышка (диск ≥ 1 МБ/с, память ≥ 25 МБ/с) > CPU ≥ 40 % > работает.

## 5. Процессы — сценарии
| Сценарий | Поведение |
|---|---|
| 2 клика по Chrome | 2 окна → счётчик «2» (считаются видимые окна верхнего уровня, не процессы) |
| Клик по запущенному | фокус; если уже в фокусе и 1 окно → свернуть; если окон много → следующее |
| Лаунчеры (Update.exe --processStart) | сопоставление по реальному exe |
| Совпадение пути не найдено | запасное сопоставление по имени exe |
| Зависание | IsHungAppWindow → красная лампа |
| Метрики | сумма по всем процессам с этим exe (Chrome: 24 процесса → одно число) |
| Нагрузка самого Rail | скрыт: только окна раз в 2 с; виден: метрики 1 Гц |

## 6. Токены анимаций
| Переход | Параметры |
|---|---|
| Появление панели | spring 420/34, blur 8→0, сдвиг 26 px от края, stagger 30 мс |
| Скрытие | 200 мс ease‑in → окно прячется после exit (seq‑защита от гонок) |
| Увеличение (dock) | гаусс σ = 0.95·S, ×1.4 иконка / ×1.24 блок, соседи раздвигаются |
| Морф иконка → блок | layout spring 520/36 |
| Счётчик окон | цифра уезжает вверх / въезжает снизу |
| Карточка | spring 520/34, scale 0.94→1 от стороны рельсы |
| Прогресс удержания | кольцо pathLength за hoverDelay |
| Тема Аврора | вращающаяся радужная кромка (@property угол), свечение блока цветом иконки |

## 7. Критерии PASS
| Проверка | Как |
|---|---|
| TypeScript strict | `tsc` = 0 ошибок |
| CSS | stylelint standard = 0 ошибок |
| Вёрстка | `qa/visual.mjs`: 20 сцен (4 темы × rail / flyout×2 / settings + низ / лево / пусто / L) без выхода за окно, обрезки текста и наложений |
| Анимации | `qa/storyboard.mjs`: кадры появления и морфа запуска |
| Сборка | exe собирается кросс‑компиляцией и в CI |

## 8. Известные ограничения
- Перетаскивание файла прямо на значок в трее Windows не поддерживает → тащите к краю экрана (панель выезжает сама).
- Метрики процессов, запущенных от администратора, могут быть неполными, если сам Rail запущен без прав.
- Store/UWP‑приложения запускаются, но статус окон определяется только для классических exe.
- Без подписи кода SmartScreen при первом запуске покажет «Подробнее → Выполнить в любом случае».

## 9. Аудит v1.1 — обнаруженные проблемы и исправления

### 9.1 Drag‑and‑drop не работает (закрыто)
**Причина:** `WS_EX_TRANSPARENT` (click‑through) блокирует OLE drag‑drop — Windows кэширует ex‑styles окна‑получателя в момент старта перетаскивания. Три предыдущие попытки (AtomicBool в WM_NCHITTEST, toggle ex‑style, subclass `WM_NCHITTEST`) провалились: WebView2‑child живёт в отдельном процессе (`msedgewebview2.exe`), родительский subclass его hit‑test не видит.
**Решение (финал):** [`win::set_click_region()`](../../Rail-source/src-tauri/src/win.rs) — `SetWindowRgn` физически вырезает из окна всё кроме капсулы. Вне региона клики и OLE‑дропы падают насквозь, внутри всё интерактивно, `WS_EX_TRANSPARENT` не используется вовсе. Вызов — только через `AppHandle::run_on_main_thread` (владелец окна), повторно применяется в [`show_rail()`](../../Rail-source/src-tauri/src/main.rs) после `set_size`/`show`. Верифицировано: `GetWindowRgn` → `SIMPLEREGION` (diag/verify-region.ps1 -Reveal).

### 9.2 Кнопка «+» — file picker (закрыто)
**Причина:** `blocking_pick_files()` в async‑контексте Tauri command вешает вызов (COM STA), диалог не открывается.
**Решение:** callback‑API [`pick_files(move |picked| …)`](../../Rail-source/src-tauri/src/main.rs) → `add_paths()`.

### 9.3 Размер рельсы
**Требование:** два режима — `normal` (текущий) и `compact` (−35% cross, уменьшенные padding/gap/icon).
**Решение:** новая настройка `railScale: 'normal' | 'compact'` в `Settings`. В compact‑режиме `RAIL_CROSS = 109`, `iconSize` уменьшается на 35%, CSS‑переменная `--rail-compact` управляет padding/gap.

### 9.4 Потенциальный deadlock в tracker
**Причина:** `cursor_tick()` берёт `core.rt` несколько раз, между ними вызывает `work_for_point()` через main thread; main thread может ждать `core.rt`.
**Решение:** собирать все данные из `core.rt` за одно взятие lock, `work_for_point()` вызывать вне lock.

### 9.5 Каскадный panic при poison mutex
**Причина:** `lock().unwrap()` во всех 20+ местах.
**Решение:** вспомогательные методы `cfg()` и `rt()` на `Core`, которые при poison возвращают данные через `unwrap_or_else(|e| e.into_inner())`.

### 9.6 Избыточный SetWindowLongPtrW каждые 33 мс (закрыто)
**Решение:** весь механизм `ignore_set`/`rt.ignoring` удалён — вместо него `SetWindowRgn` (см. 9.1), toggle ex‑style больше не нужен.

### 9.7 Auto‑hide не учитывает flyout
**Причина:** в условии auto‑hide нет проверки на открытый flyout.
**Решение:** добавить `flyoutOpen` state и учитывать его в auto‑hide.

### 9.8 Гонка при сохранении конфига
**Решение:** дебаунс `save()` — 300 мс.

### 9.9 Утечка hist
**Решение:** удалять записи из `hist` для items, которых нет в текущем `config.items`.

### 9.10 Прыгающая RAM‑статистика (закрыто)
**Причина:** sysinfo отдаёт «сырые» сэмплы каждый 950 мс — тротлинг‑визуал.
**Решение:** EMA‑сглаживание cpu/mem (α=0.35) в `tracker::spawn`.

### 9.11 Интернет‑исследование — подтверждение корневых причин
- [tauri#6164 — Add forward option to `setIgnoreCursorEvents`](https://github.com/tauri-apps/tauri/issues/6164) — selective click‑through через `set_ignore_cursor_events` невозможен в принципе (open). Наш `SetWindowRgn` — обходной путь на уровне Win32.
- [tauri#10564 — `set_ignore_cursor_events` для `Webview` в v2](https://github.com/tauri-apps/tauri/issues/10564) — API ограничен окном целиком (open).
- [tauri#15947 — transparent always‑on‑top: composition race при toggle cursor events](https://github.com/tauri-apps/tauri/issues/15947) — подтверждает: трогать ex‑styles прозрачного окна на лету опасно, статичный регион надёжнее (open).
- [plugins‑workspace#2153 — Dialog API: file selection bug](https://github.com/tauri-apps/plugins-workspace/issues/2153), [#2398 — `blocking_pick_files()` panic](https://github.com/tauri-apps/plugins-workspace/issues/2398) — blocking‑API диалогов ненадёжен, callback‑путь корректен.
