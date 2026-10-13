# CHANGELOG

## v3.0 · 2026-10-10
Первая версия из git. Заменяет ANALYST-START v2.1 (docs/legacy/), что изменилось — docs/MIGRATION-v2.1.md.
- Ядро + 3 режима (NEW, JOIN, LOCAL) + CODER-BOOT + AUDITOR; сборки `builds/START-*.md` собирает `tools/bro.ps1 build`.
- Обмен через git: аналитик пишет только в `.bro/inbox` и `.bro/exchange`, не больше 5 обращений к GitHub за волну.
- Нет автозагрузки: система включается только строкой-триггером.
- Приёмка: стенд «как человек» + `proof/INDEX.md`, без доказательств FAIL.
- Метрики: паспорта волн, рубрики K/O, WAVES.csv, SUMMARY.md (7 показателей); все суммы считает `bro.ps1`.
- Прогресс: карточка в ответе аналитика, `.bro/STATUS.md`, `.bro/progress.html`.
- Микропромтинг M1–M6: агентам английский, Владельцу русский.
- Система на ПК — `%LOCALAPPDATA%\global-promt-system`, зеркала — `%LOCALAPPDATA%\BRO-MIRROR\<проект>\` (диск D: не нужен); в строках-триггерах полный путь `<BRO>`.

Тег ставит кодер в клоне на ПК (PowerShell, одна строка): `$B="$env:LOCALAPPDATA\global-promt-system"; git -C $B pull; git -C $B tag v3.0; git -C $B push origin v3.0`
