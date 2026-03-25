# Data Model: 日报助手 MVP

## 1. TaskEntry

### 目的

表示某一天的一条任务记录，是快速记录、汇总编辑和日报生成的基础实体。

### 字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `id` | UUID / 字符串 | 是 | 唯一标识 |
| `date` | 日期 | 是 | 任务归属日期，按本地时区确定 |
| `created_at` | 日期时间 | 是 | 创建时间 |
| `updated_at` | 日期时间 | 是 | 最近更新时间 |
| `content` | 文本 | 是 | 任务标题或内容 |
| `status` | 枚举 | 是 | `done` / `in_progress` |
| `note` | 文本 | 否 | 备注信息 |

### 校验规则

- `content` 不能为空，去除首尾空白后仍需有内容。
- `status` 只能为 `done` 或 `in_progress`。
- `date` 必须与 `created_at` 的本地日期一致；当天补录仍按当前日期保存。

### 状态转换

- 初始状态：`done`
- 允许转换：
  - `done -> in_progress`
  - `in_progress -> done`

## 2. DailyReportDraft

### 目的

表示某一天用于日报展示、生成和 AI 润色的组装结果。

### 字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `date` | 日期 | 是 | 汇总日期 |
| `entries` | TaskEntry[] | 是 | 当日任务快照 |
| `done_entries` | TaskEntry[] | 是 | 已完成任务集合 |
| `in_progress_entries` | TaskEntry[] | 是 | 进行中任务集合 |
| `basic_report_text` | 文本 | 是 | 基础日报文本 |
| `polished_report_text` | 文本 | 否 | AI 润色结果 |
| `generation_source` | 枚举 | 是 | `basic` / `ai` |
| `last_generated_at` | 日期时间 | 否 | 最近生成时间 |

### 组装规则

- 每次打开汇总页或保存任务编辑后，均可重新从 `TaskEntry` 组装。
- `basic_report_text` 必须始终可生成，即使 `entries` 为空。
- `polished_report_text` 仅在 AI 润色成功后存在。

## 3. AppSettings

### 目的

表示应用本地配置，包括提醒、快捷键和 AI 配置。

### 字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `reminder_enabled` | 布尔 | 是 | 是否启用提醒 |
| `off_work_time` | 时间 | 是 | 下班时间 |
| `remind_before_minutes` | 整数 | 是 | 提前提醒分钟数，MVP 默认 5 |
| `global_hotkey` | 文本 | 是 | 全局快捷键 |
| `ai_enabled` | 布尔 | 是 | 是否启用 AI 润色 |
| `ai_base_url` | 文本 | 否 | AI 服务地址 |
| `ai_model` | 文本 | 否 | 模型名 |
| `ai_api_key_ref` | 文本 | 否 | API Key 引用或受保护存储定位 |
| `report_template` | 文本 | 是 | 模板标识，MVP 默认 `default` |
| `updated_at` | 日期时间 | 是 | 最近更新时间 |

### 校验规则

- `global_hotkey` 必须可被解析为宿主可注册的快捷键格式。
- `off_work_time` 必须是合法时间值。
- 当 `ai_enabled` 为 `true` 时，Base URL、模型名和 API Key 都必须通过配置校验后才能发起 AI 润色。

## 4. 关系说明

- 一个 `DailyReportDraft` 对应一个日期。
- 一个 `DailyReportDraft` 聚合当天的多个 `TaskEntry`。
- `AppSettings` 为全局单例配置，不按日期分片。

## 5. 迁移策略

- 初始版本建立 `task_entries` 与 `app_settings` 两张主表。
- 后续若新增状态类型、日报模板或 AI 服务预设，通过 schema migration 追加字段或新表。
- 所有迁移必须保证既有任务记录与设置可读，不允许破坏性覆盖。
