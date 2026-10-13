<!-- bro-i18n
turn.coder=🤖 кодер
turn.analyst=🧠 аналитик
turn.owner=👤 ты
todo.coder=ничего — ждём «Готово» от кодера
todo.analyst=напиши аналитику: BRO CHECK
todo.owner=ответь аналитику на 🛑
stop.in=🛑 {title} через {n} шаг.
stop.now=🛑 {title} — сейчас
stop.none=остановок больше нет
st.done=✅
st.run=▶
st.wait=🛑
st.todo=○
st.pause=⏸
st.frozen=🧊
-->
# Статус · {{PROJECT}} · BRO {{VER}}
Обновлено {{UPDATED}} · режим {{MODE}} · волна {{WAVE}}

{{BAR}} **{{DONE}} из {{TOTAL}} · {{PCT}} %** · сейчас «{{CURRENT}}» · ход за {{TURN}}

**Тебе:** {{TODO}} · **Дальше:** {{NEXT_STOP}}

```mermaid
{{MERMAID}}
```

| Показатель | Значение |
|---|---|
| 🎯 Принято с первого раза | {{FIRST_PASS}} |
| 🙈 Ложные «готово» | {{FALSE_DONE}} |
| ⏱ Последняя волна: работа / ожидание, мин | {{MINUTES}} / {{WAIT}} |
| 🪙 ≈ptok на принятую задачу | {{PTOK_TASK}} |
| 🔌 Обращений к GitHub за волну (среднее) | {{GH}} |

<details><summary>Волны · {{WAVES_N}}</summary>

{{WAVES_TABLE}}
</details>

Страница на ПК: `.bro\progress.html` (двойной щелчок).
