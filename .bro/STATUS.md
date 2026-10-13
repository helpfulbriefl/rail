# Статус · rail-publish · BRO v3.0
Обновлено 2026-10-13 23:07 · режим JOIN · волна W1B-BASELINE

▰▰▱▱▱▱▱▱ **2 из 8 · 25 %** · сейчас «Сборка и реальный стенд» · ход за 🧠 аналитик

**Тебе:** напиши аналитику: BRO CHECK · **Дальше:** 🛑 Макеты через 1 шаг.

```mermaid
flowchart LR
  n0["✅ Бриф 🛑"]:::done
  n1["✅ Разведка PARTIAL"]:::done
  n0 --> n1
  n2["▶ Сборка и реальный стенд"]:::run
  n1 --> n2
  n3["○ Макеты 🛑"]:::todo
  n2 --> n3
  n4["○ Логика"]:::todo
  n3 --> n4
  n5["○ Интерфейс"]:::todo
  n4 --> n5
  n6["○ Регрессия"]:::todo
  n5 --> n6
  n7["○ Релиз 🛑"]:::todo
  n6 --> n7
  classDef done fill:#DCFCE7,stroke:#16A34A,color:#16181D
  classDef run fill:#E0E7FF,stroke:#4F46E5,color:#16181D
  classDef wait fill:#FEF3C7,stroke:#D97706,color:#16181D
  classDef todo fill:#FFFFFF,stroke:#E6E7EB,color:#5B6170
  classDef pause fill:#F3F4F6,stroke:#9CA3AF,color:#5B6170
  classDef frozen fill:#F3F4F6,stroke:#9CA3AF,color:#5B6170
```

| Показатель | Значение |
|---|---|
| 🎯 Принято с первого раза | 0/1 |
| 🙈 Ложные «готово» | 0 |
| ⏱ Последняя волна: работа / ожидание, мин | 8 / - |
| 🪙 ≈ptok на принятую задачу | 3252 |
| 🔌 Обращений к GitHub за волну (среднее) | 12 |

<details><summary>Волны · 1</summary>

| W | verdict | done/tasks | defects | min | GitHub |
|---|---|---|---|---|---|
| W1-RECON | PARTIAL | 6/6 | 5 | 8 | 12 |
</details>

Страница на ПК: `.bro\progress.html` (двойной щелчок).
