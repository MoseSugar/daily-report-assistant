---

description: "Task list for daily-report-assistant MVP implementation"

---

# Tasks: 日报助手 MVP

**Input**: Design documents from `/specs/001-desktop-daily-report/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: 关键流程必须包含最小必要验证。本任务列表同时包含可自动化测试任务和手动验收脚本任务。

**Organization**: 任务按用户故事与 MVP 阶段组织，优先保证“不依赖 AI 的本地闭环”，再补桌面提醒、AI 接入与体验收尾。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 可并行执行（不同文件、无未完成前置依赖）
- **[Story]**: 用户故事标签，仅在用户故事阶段使用
- 所有任务都包含明确文件路径，便于 Codex 分批落地

## Phase 1: Setup (第一阶段：项目骨架与最小数据闭环)

**Purpose**: 建立 Tauri 桌面应用骨架、目录结构和基础开发工具链

- [ ] T001 初始化 Tauri + React + TypeScript 项目骨架于 `src/`、`src-tauri/`、`src-tauri/Cargo.toml`、`package.json`、`vite.config.ts`
- [ ] T002 创建实现计划约定的目录结构于 `src/app/`、`src/components/`、`src/windows/`、`src/stores/`、`src/services/`、`src/models/`、`src/styles/`、`src-tauri/src/commands/`、`src-tauri/src/desktop/`、`src-tauri/src/domain/`、`src-tauri/src/storage/`、`src-tauri/src/ai/`、`src-tauri/src/scheduling/`
- [ ] T003 [P] 配置前端开发与代码质量工具于 `tsconfig.json`、`eslint.config.*`、`.prettierrc*`、`src/main.tsx`
- [ ] T004 [P] 配置 Rust 开发基础与 Tauri 启动入口于 `src-tauri/src/main.rs`、`src-tauri/src/lib.rs`
- [ ] T005 [P] 建立基础手动验收脚本模板于 `tests/manual/smoke-checklist.md`

**Checkpoint**: 代码仓库具备可启动的桌面应用骨架，后续基础能力可以分层接入

---

## Phase 2: Foundational (第二阶段前置：共享基础设施)

**Purpose**: 建立所有用户故事共用的数据模型、存储、命令接口和错误边界

**⚠️ CRITICAL**: 本阶段完成前，不开始用户故事实现

- [ ] T006 定义共享领域模型与枚举于 `src/models/task.ts`、`src/models/report.ts`、`src/models/settings.ts`、`src-tauri/src/domain/task_entry.rs`、`src-tauri/src/domain/daily_report_draft.rs`、`src-tauri/src/domain/app_settings.rs`
- [ ] T007 实现 SQLite 初始化、schema 与迁移于 `src-tauri/src/storage/database.rs`、`src-tauri/src/storage/migrations.rs`、`src-tauri/migrations/0001_initial.sql`
- [ ] T008 [P] 实现任务仓储与设置仓储接口于 `src-tauri/src/storage/task_repository.rs`、`src-tauri/src/storage/settings_repository.rs`
- [ ] T009 [P] 实现宿主层统一错误类型与脱敏日志边界于 `src-tauri/src/domain/error.rs`、`src-tauri/src/desktop/logging.rs`
- [ ] T010 实现 Tauri 命令注册与前后端调用骨架于 `src-tauri/src/commands/mod.rs`、`src-tauri/src/commands/capture.rs`、`src-tauri/src/commands/summary.rs`、`src-tauri/src/commands/settings.rs`、`src-tauri/src/commands/system.rs`、`src/services/tauriClient.ts`
- [ ] T011 [P] 建立共享状态存储与窗口路由骨架于 `src/stores/appStore.ts`、`src/app/App.tsx`、`src/windows/CaptureWindow.tsx`、`src/windows/DailySummaryWindow.tsx`、`src/windows/SettingsWindow.tsx`
- [ ] T012 编写基础仓储与模型自动化测试于 `tests/unit/taskRepository.test.ts`、`tests/unit/settingsRepository.test.ts`、`src-tauri/src/storage/tests.rs`

**Checkpoint**: 基础数据与命令通道可用，用户故事可开始独立推进

---

## Phase 3: User Story 1 - 快速记录当下任务 (Priority: P1) 🎯 MVP

**Goal**: 支持本地任务录入、保存、追加记录和基础错误反馈，形成最小数据闭环

**Independent Test**: 启动应用后可从可见入口打开快速记录窗口，连续保存多条任务并在应用重启后仍能查到

### Verification for User Story 1

- [ ] T013 [P] [US1] 编写任务新增与校验规则测试于 `tests/unit/taskService.test.ts`、`src-tauri/src/domain/task_entry.rs`
- [ ] T014 [P] [US1] 编写快速记录手动验收步骤于 `tests/manual/us1-capture-flow.md`

### Implementation for User Story 1

- [ ] T015 [US1] 实现任务新增业务服务于 `src-tauri/src/domain/task_service.rs`
- [ ] T016 [US1] 实现 `capture.createTask` 命令与输入校验于 `src-tauri/src/commands/capture.rs`
- [ ] T017 [P] [US1] 实现快速记录窗口 UI 与表单状态于 `src/windows/CaptureWindow.tsx`、`src/components/CaptureForm.tsx`、`src/stores/captureStore.ts`
- [ ] T018 [US1] 接通快速记录保存动作与成功/失败反馈于 `src/services/captureService.ts`、`src/windows/CaptureWindow.tsx`
- [ ] T019 [US1] 实现启动后默认可见入口打开快速记录窗口于 `src/app/App.tsx`、`src/components/AppShell.tsx`
- [ ] T020 [US1] 实现任务数据重载与应用重启后读取校验于 `src-tauri/src/storage/task_repository.rs`、`tests/integration/capture-persistence.test.ts`

**Checkpoint**: 用户故事 1 完成后，应用已经能本地记录并持久化当天任务

---

## Phase 4: User Story 2 - 当日汇总、编辑与提醒弹窗 (Priority: P2)

**Goal**: 支持查看当日任务、编辑状态与备注、生成基础日报，并完成提醒与汇总窗口唤起

**Independent Test**: 准备多条任务后，用户可以打开当日汇总窗口修改状态与备注，生成基础日报；调整提醒时间后可触发自动弹窗

### Verification for User Story 2

- [ ] T021 [P] [US2] 编写当日任务聚合与基础日报拼装测试于 `tests/unit/dailyReportService.test.ts`、`src-tauri/src/domain/daily_report_service.rs`
- [ ] T022 [P] [US2] 编写提醒计算与重算测试于 `tests/unit/reminderService.test.ts`、`src-tauri/src/scheduling/reminder_service.rs`
- [ ] T023 [P] [US2] 编写汇总编辑与提醒弹窗手动验收步骤于 `tests/manual/us2-summary-and-reminder.md`

### Implementation for User Story 2

- [ ] T024 [US2] 实现当日任务查询与 `summary.getTodayDraft` 命令于 `src-tauri/src/domain/daily_report_service.rs`、`src-tauri/src/commands/summary.rs`
- [ ] T025 [P] [US2] 实现当日汇总窗口列表与空状态 UI 于 `src/windows/DailySummaryWindow.tsx`、`src/components/TaskList.tsx`、`src/components/EmptyState.tsx`
- [ ] T026 [US2] 实现任务状态切换与备注编辑持久化于 `src-tauri/src/domain/task_service.rs`、`src-tauri/src/commands/summary.rs`、`src/components/TaskRowEditor.tsx`
- [ ] T027 [US2] 实现基础日报文本拼装规则与 `summary.generateBasicReport` 命令于 `src-tauri/src/domain/daily_report_service.rs`、`src/services/reportFormatter.ts`
- [ ] T028 [US2] 在汇总窗口接通“生成基础日报”结果展示于 `src/windows/DailySummaryWindow.tsx`、`src/components/ReportPreview.tsx`
- [ ] T029 [P] [US2] 实现提醒设置模型读写与重调度逻辑于 `src-tauri/src/domain/settings_service.rs`、`src-tauri/src/storage/settings_repository.rs`
- [ ] T030 [US2] 实现本地提醒调度器与触发规则于 `src-tauri/src/scheduling/reminder_service.rs`、`src-tauri/src/scheduling/mod.rs`
- [ ] T031 [US2] 实现汇总窗口显示/聚焦/置顶命令于 `src-tauri/src/desktop/window_manager.rs`、`src-tauri/src/commands/system.rs`
- [ ] T032 [US2] 接通提醒触发后的汇总窗口自动唤起于 `src-tauri/src/main.rs`、`src-tauri/src/scheduling/reminder_service.rs`、`src/windows/DailySummaryWindow.tsx`

**Checkpoint**: 用户故事 2 完成后，应用具备“记录 -> 汇总 -> 编辑 -> 基础日报”的完整非 AI 闭环

---

## Phase 5: User Story 3 - AI 润色与可发送日报输出 (Priority: P3)

**Goal**: 支持配置兼容 OpenAI 风格 API 的 AI 服务，对基础日报进行润色，并在失败时可靠降级

**Independent Test**: 配置有效 AI 参数时可生成润色日报；配置缺失或请求失败时仍能返回基础日报且不阻塞复制

### Verification for User Story 3

- [ ] T033 [P] [US3] 编写 AI 请求构造、响应解析与失败降级测试于 `tests/unit/aiClient.test.ts`、`src-tauri/src/ai/openai_client.rs`
- [ ] T034 [P] [US3] 编写 AI 成功/失败手动验收步骤于 `tests/manual/us3-ai-polish.md`

### Implementation for User Story 3

- [ ] T035 [US3] 实现 AI 配置模型校验与设置存储扩展于 `src-tauri/src/domain/app_settings.rs`、`src-tauri/src/domain/settings_service.rs`、`src/models/settings.ts`
- [ ] T036 [P] [US3] 实现 OpenAI 风格 AI 请求客户端于 `src-tauri/src/ai/openai_client.rs`、`src-tauri/src/ai/mod.rs`
- [ ] T037 [P] [US3] 实现 AI Prompt 构造与响应文本提取于 `src-tauri/src/ai/prompt_builder.rs`、`src-tauri/src/ai/response_parser.rs`
- [ ] T038 [US3] 实现 `summary.generateAiPolishedReport` 命令与失败降级返回于 `src-tauri/src/commands/summary.rs`、`src-tauri/src/domain/daily_report_service.rs`
- [ ] T039 [US3] 在汇总窗口接通“AI 润色”触发、加载态与结果展示于 `src/windows/DailySummaryWindow.tsx`、`src/components/ReportActions.tsx`

**Checkpoint**: 用户故事 3 完成后，应用具备 AI 可选润色能力，且失败时能保持本地闭环

---

## Phase 6: Polish & Cross-Cutting Concerns (第六阶段：复制、配置与体验收尾)

**Purpose**: 完成复制、设置页、托盘与关键错误场景处理，形成可交付 MVP

- [ ] T040 实现设置页 UI 与保存动作于 `src/windows/SettingsWindow.tsx`、`src/components/SettingsForm.tsx`、`src/services/settingsService.ts`
- [ ] T041 实现全局快捷键注册、重注册与失败处理于 `src-tauri/src/desktop/hotkey_manager.rs`、`src-tauri/src/commands/settings.rs`、`src-tauri/src/main.rs`
- [ ] T042 实现快速记录窗口的全局快捷键唤起与自动聚焦于 `src-tauri/src/desktop/window_manager.rs`、`src-tauri/src/desktop/hotkey_manager.rs`、`src/windows/CaptureWindow.tsx`
- [ ] T043 [P] 实现剪贴板复制命令与失败提示于 `src-tauri/src/desktop/clipboard.rs`、`src-tauri/src/commands/system.rs`、`src/components/ReportActions.tsx`
- [ ] T044 [P] 实现最小托盘入口与手动打开窗口动作于 `src-tauri/src/desktop/tray.rs`、`src-tauri/src/main.rs`
- [ ] T045 实现关键错误提示文案与敏感日志脱敏检查于 `src/components/ErrorBanner.tsx`、`src-tauri/src/desktop/logging.rs`、`src-tauri/src/domain/error.rs`
- [ ] T046 [P] 补充空数据场景、复制失败、快捷键失败等集成测试于 `tests/integration/error-flows.test.ts`、`tests/integration/summary-empty-state.test.ts`
- [ ] T047 依据 `specs/001-desktop-daily-report/quickstart.md` 执行完整手动验收并记录结果于 `tests/manual/mvp-validation-report.md`
- [ ] T048 打包前核对 Windows 11 行为与发布说明于 `src-tauri/tauri.conf.json`、`tests/manual/prepackage-checklist.md`

**Checkpoint**: MVP 可在 Windows 11 上完整运行，具备配置、容错、复制与打包前验证能力

---

## Phase 7: 后续增强项（非 MVP，不纳入第一版必须实现）

**Purpose**: 列出第二阶段建议任务，供后续批次单独排期

- [ ] T049 实现多模板日报输出能力于 `src/models/settings.ts`、`src/services/reportFormatter.ts`、`src/components/TemplateSelector.tsx`
- [ ] T050 实现更多任务状态类型与状态筛选于 `src/models/task.ts`、`src/components/TaskRowEditor.tsx`、`src-tauri/src/domain/task_entry.rs`
- [ ] T051 实现历史日期查看与补录入口于 `src/windows/DailySummaryWindow.tsx`、`src-tauri/src/domain/daily_report_service.rs`
- [ ] T052 实现多 AI 服务配置预设与切换于 `src/models/settings.ts`、`src/windows/SettingsWindow.tsx`、`src-tauri/src/ai/`
- [ ] T053 实现更细粒度提醒策略与提醒开关于 `src/models/settings.ts`、`src-tauri/src/scheduling/reminder_service.rs`

---

## 当前不建议实现

- 账号系统、云同步、多设备同步
- 多人协作、审批流、企业系统集成
- 周报、月报、统计分析面板
- 项目/标签/优先级体系
- 围绕 AI 的复杂提示工程工作台

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 无依赖，可立即开始
- **Foundational (Phase 2)**: 依赖 Setup 完成，阻塞所有用户故事
- **User Story 1 (Phase 3)**: 依赖 Foundational 完成
- **User Story 2 (Phase 4)**: 依赖 User Story 1 的数据闭环完成
- **User Story 3 (Phase 5)**: 依赖 User Story 2 的基础日报生成完成
- **Polish (Phase 6)**: 依赖前三个用户故事完成
- **Enhancements (Phase 7)**: 不属于当前 MVP，可延后

### User Story Dependencies

- **US1**: 无业务前置故事，是 MVP 起点
- **US2**: 依赖 US1 已能产生和读取当日任务
- **US3**: 依赖 US2 已能生成基础日报文本

### Within Each User Story

- 验证任务先定义，再进入主要实现
- 数据模型/仓储先于业务服务
- 业务服务先于命令与 UI 接线
- UI 接线完成后补集成验证

### Parallel Opportunities

- T003、T004、T005 可并行
- T008、T009、T011 可并行
- US1 中 T013、T014 可并行；T017 可在 T015/T016 前后并行准备
- US2 中 T021、T022、T023 可并行；T025 与 T029 可并行
- US3 中 T033、T034 可并行；T036 与 T037 可并行
- Polish 阶段中 T043、T044、T046 可并行

---

## Parallel Example: User Story 2

```bash
Task: "T021 [US2] 编写当日任务聚合与基础日报拼装测试于 tests/unit/dailyReportService.test.ts、src-tauri/src/domain/daily_report_service.rs"
Task: "T022 [US2] 编写提醒计算与重算测试于 tests/unit/reminderService.test.ts、src-tauri/src/scheduling/reminder_service.rs"
Task: "T023 [US2] 编写汇总编辑与提醒弹窗手动验收步骤于 tests/manual/us2-summary-and-reminder.md"

Task: "T025 [US2] 实现当日汇总窗口列表与空状态 UI 于 src/windows/DailySummaryWindow.tsx、src/components/TaskList.tsx、src/components/EmptyState.tsx"
Task: "T029 [US2] 实现提醒设置模型读写与重调度逻辑于 src-tauri/src/domain/settings_service.rs、src-tauri/src/storage/settings_repository.rs"
```

---

## Implementation Strategy

### MVP First

1. 完成 Phase 1 和 Phase 2，先建立稳定骨架与存储基础
2. 完成 Phase 3，验证本地任务录入闭环
3. 完成 Phase 4，验证不依赖 AI 的完整日报闭环
4. 完成 Phase 5，增加可选 AI 能力与失败降级
5. 完成 Phase 6，处理配置、复制、托盘和错误边界

### Incremental Delivery

1. 第一批交付：T001-T012，得到可启动骨架和基础仓储
2. 第二批交付：T013-T020，得到“快速记录 -> 本地保存”
3. 第三批交付：T021-T032，得到“汇总 -> 编辑 -> 基础日报 -> 自动提醒”
4. 第四批交付：T033-T039，得到 AI 润色与降级
5. 第五批交付：T040-T048，完成配置、复制、托盘、容错和打包前验证

### Codex 执行策略

1. 每一批只处理一个明确闭环，避免同时改桌面能力、AI 接入和大面积 UI。
2. 先做数据层与命令层，再接 UI；这样出错时更容易定位，也更适合单批验证。
3. 每批实现后先跑对应自动化测试，再按 `tests/manual/*.md` 做最小人工验收，再提交。
4. 高风险能力拆开执行：
   - 快捷键注册与窗口唤起分两个任务
   - 提醒调度与窗口自动弹出分两个任务
   - AI 客户端、Prompt/解析、降级接线分三个任务
5. 每批建议一个提交：
   - `feat: bootstrap tauri daily report app`
   - `feat: add local task capture flow`
   - `feat: add daily summary editing and reminder flow`
   - `feat: add ai polishing with fallback`
   - `feat: finish settings clipboard tray and mvp validation`

## Notes

- T001-T048 属于 MVP 第一版必须做
- T049-T053 可以延后到第二阶段
- “当前不建议实现”列表不进入当前开发排期
- 所有任务都应局部修改，避免顺手重构无关模块
