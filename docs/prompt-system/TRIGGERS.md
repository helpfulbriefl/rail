# Строки-триггеры
Копируешь строку и меняешь `<…>`.

**Один раз:** `<BRO>` — полный путь к системе на ПК. Узнай его в PowerShell:
```powershell
"$env:LOCALAPPDATA\global-promt-system"
```
Получится примерно `C:\Users\Ivan\AppData\Local\global-promt-system`. Замени `<BRO>` во всех строках ниже (Блокнот → Ctrl+H) и сохрани себе. Дальше аналитик сам даёт строку для кодера уже с путями.

## Аналитику (новый чат Notion)
Новый проект:
```text
BRO START NEW · read github helpfulbriefl/global-promt-system builds/START-NEW.md · project <имя> · path <C:\proj\имя> · sys <BRO>
```
Проект уже есть (в том числе на v2.x):
```text
BRO START JOIN · read github helpfulbriefl/global-promt-system builds/START-JOIN.md · repo <owner/repo> · path <C:\proj\имя> · sys <BRO>
```
Без git — приложи файл `<BRO>\builds\START-LOCAL.md` (папка AppData скрыта: в окне выбора файла вставь в поле «Имя файла» `%LOCALAPPDATA%\global-promt-system\builds` и нажми Enter):
```text
BRO START LOCAL · read attached START-LOCAL.md · project <имя> · path <C:\proj\имя> · sys <BRO>
```
| Что нужно | Что написать |
|---|---|
| проверить волну | `BRO CHECK` (в LOCAL ещё приложи `.bro\archive\<волна>.zip`) |
| передать дела новому чату | `BRO HANDOFF` |
| задать лимит контекста | добавь в строку старта `· ctx 200000` |
| аудит версии системы | `BRO AUDIT · read github helpfulbriefl/global-promt-system auditor/AUDITOR.md` |

## Кодеру (новый чат Zoo Code на каждую волну)
Zoo Code спросит разрешение прочитать файл вне проекта — разреши.
Первая волна — ниже строки вставь блок задания от аналитика:
```text
BRO GO FIRST · boot <BRO>\core\CODER-BOOT.md · project <C:\proj\имя> · repo <https://github.com/owner/repo>
```
Следующие волны:
```text
BRO GO · boot <BRO>\core\CODER-BOOT.md · project <C:\proj\имя>
```
Без git — ниже строки вставь блок от аналитика:
```text
BRO GO LOCAL · boot <BRO>\core\CODER-BOOT.md · project <C:\proj\имя>
```
Кодер закончит фразой «Готово → напиши аналитику: BRO CHECK».
