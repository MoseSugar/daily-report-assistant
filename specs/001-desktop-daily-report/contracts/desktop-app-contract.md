# Desktop App Contract: 日报助手 MVP

## 1. 范围

该契约定义界面层与宿主/业务层之间的核心命令与返回结果，用于后续任务拆解和接口实现对齐。

## 2. 核心命令

### `capture.createTask`

- **输入**
  - `content`: string
- **行为**
  - 创建一条当日 `TaskEntry`
  - 默认状态为 `done`
- **成功输出**
  - `task`: TaskEntry
  - `message`: "保存成功"
- **失败输出**
  - `error_code`: `VALIDATION_ERROR` | `PERSISTENCE_ERROR`
  - `message`: 面向用户的错误说明

### `summary.getTodayDraft`

- **输入**
  - 无
- **行为**
  - 查询当天任务并组装 `DailyReportDraft`
- **成功输出**
  - `draft`: DailyReportDraft

### `summary.updateTask`

- **输入**
  - `task_id`: string
  - `status`: `done` | `in_progress`
  - `note`: string
- **行为**
  - 更新目标任务的状态与备注
- **成功输出**
  - `task`: TaskEntry
- **失败输出**
  - `error_code`: `NOT_FOUND` | `VALIDATION_ERROR` | `PERSISTENCE_ERROR`

### `summary.generateBasicReport`

- **输入**
  - `date`: string
- **行为**
  - 根据当天任务生成基础日报文本
- **成功输出**
  - `draft`: DailyReportDraft
  - `generation_source`: `basic`

### `summary.generateAiPolishedReport`

- **输入**
  - `date`: string
- **行为**
  - 在最新基础日报之上发起 AI 润色
- **成功输出**
  - `draft`: DailyReportDraft
  - `generation_source`: `ai`
- **失败输出**
  - `error_code`: `AI_CONFIG_MISSING` | `AI_REQUEST_FAILED` | `AI_RESPONSE_INVALID`
  - `fallback_draft`: DailyReportDraft
  - `message`: "AI 润色失败，已保留基础日报"

### `settings.get`

- **输入**
  - 无
- **成功输出**
  - `settings`: AppSettings

### `settings.save`

- **输入**
  - `settings`: AppSettings
- **行为**
  - 保存设置并重新调度提醒、重新注册快捷键
- **成功输出**
  - `settings`: AppSettings
- **失败输出**
  - `error_code`: `INVALID_HOTKEY` | `HOTKEY_REGISTER_FAILED` | `PERSISTENCE_ERROR`

### `system.copyText`

- **输入**
  - `text`: string
- **成功输出**
  - `copied`: true
- **失败输出**
  - `error_code`: `CLIPBOARD_UNAVAILABLE`

## 3. 窗口事件

- `window.capture.request_show`
- `window.summary.request_show`
- `window.settings.request_show`
- `window.summary.reminder_triggered`

## 4. 界面保证

- 快速记录窗口打开后必须聚焦输入框。
- 汇总窗口重新生成日报后必须刷新结果文本区。
- AI 失败后必须继续展示基础日报，不允许清空结果区。
