# GitHub: доступ для аналитика и для ПК

## 1. Токен для аналитика (Notion) — раз в 90 дней
1. Открой ссылку, в ней уже всё заполнено:
   https://github.com/settings/personal-access-tokens/new?name=BRO-analyst&description=BRO+analyst:+reads+REPORT,+writes+only+.bro/inbox+and+.bro/exchange&target_name=helpfulbriefl&expires_in=90&contents=write
2. Repository access → **Only select repositories** → отметь `global-promt-system` и репозитории проектов.
3. **Generate token** → скопируй (начинается с `github_pat_`).
4. В Notion вставь токен в подключение GitHub, в защищённое поле. В чат токен не вставляй.

Права: Contents — чтение и запись, Metadata — чтение (GitHub добавит сам). Больше ничего не нужно.

## 2. Новый проект → добавь его репозиторий в токен
github.com → Settings → Developer settings → Fine-grained tokens → BRO-analyst → Edit → Repository access → добавь репозиторий → Save. Сам токен не меняется.
Если надоест: можно выбрать «All repositories». Возни меньше, но тогда аналитик технически сможет писать в любой репозиторий. Правила ему это запрещают (R15), но защиты на стороне GitHub уже не будет.

## 3. ПК (кодер): вход через браузер, без токенов
- В Git for Windows уже есть Git Credential Manager. При первом `git push` откроется браузер → входишь в GitHub → готово.
- GitHub CLI (нужен для `gh repo create` в режиме NEW): `winget install GitHub.cli` → `gh auth login --web` → вводишь код из терминала в браузере.

Проверка: `git ls-remote https://github.com/helpfulbriefl/global-promt-system` проходит без ошибки.

## 4. Аналитик не может записать (ошибка 403 или 404)
| Ошибка | Причина | Что сделать |
|---|---|---|
| 404 | репозитория нет в токене или опечатка в имени | п. 2 |
| 403 | у токена только чтение | новый токен по ссылке из п. 1 |
| срок истёк | прошло 90 дней | новый токен по той же ссылке, заменить в Notion |

Пока не починено, аналитик отдаёт задание блоком в чат, как в первой волне.

## 5. Система на ПК
```powershell
git clone https://github.com/helpfulbriefl/global-promt-system "$env:LOCALAPPDATA\global-promt-system"
```
Полный путь (это `<BRO>` в TRIGGERS.md) покажет команда `"$env:LOCALAPPDATA\global-promt-system"`.
Обновить: `git -C "$env:LOCALAPPDATA\global-promt-system" pull`
