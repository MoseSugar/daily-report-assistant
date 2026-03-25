# Phase 0 Research: 日报助手 MVP

## 决策 1：桌面宿主采用 Tauri 2

- **Decision**: 采用 Tauri 2 作为 Windows 11 桌面宿主。
- **Rationale**: 需要全局快捷键、窗口控制、托盘、剪贴板与本地存储等桌面能力，同时希望保持轻量和低打扰；Tauri 更适合 MVP 的资源占用目标。
- **Alternatives considered**:
  - Electron：能力成熟，但运行时更重，不利于常驻后台和低打扰体验。
  - WPF/.NET：Windows 原生性更强，但前后端协作与后续 UI 迭代弹性较低。

## 决策 2：界面层采用 React + TypeScript

- **Decision**: 采用 React + TypeScript 构建快速记录、汇总编辑和设置窗口。
- **Rationale**: 3 个核心窗口都偏表单与状态驱动，React 能快速组织界面状态和编辑交互；TypeScript 有利于 DTO、枚举和值对象约束。
- **Alternatives considered**:
  - 纯原生 Rust UI：开发速度与生态都不如 Web UI 成熟。
  - Vue/Svelte：也可行，但相较 React 在团队协作、组件生态和状态管理习惯上没有明显优势。

## 决策 3：本地存储采用 SQLite

- **Decision**: 使用 SQLite 保存任务与设置。
- **Rationale**: 任务需要按日期查询、批量更新、事务保护和后续迁移；SQLite 比纯 JSON 文件更适合保证数据完整性。
- **Alternatives considered**:
  - JSON 文件：实现简单，但并发写入、迁移和查询能力弱，更容易出现误覆盖。
  - 嵌入式 KV：不利于按日期聚合和后续扩展复杂查询。

## 决策 4：提醒采用应用内调度器

- **Decision**: 由应用常驻时的本地调度器负责提醒，不接入外部系统调度。
- **Rationale**: MVP 目标是打通闭环，应用内调度更直接，便于在设置变更、重启和跨日场景下统一控制。
- **Alternatives considered**:
  - Windows 任务计划器：可靠性较强，但增加外部依赖和安装后配置复杂度。

## 决策 5：AI 接入采用独立适配器层

- **Decision**: 使用独立的 OpenAI 风格 API 适配器层，而不是在界面层直接发请求。
- **Rationale**: 需要统一处理 Base URL、API Key、模型名、请求体、超时、错误映射与后续多服务扩展。
- **Alternatives considered**:
  - UI 直接请求：实现快，但难以复用、测试和控制敏感配置边界。

## 决策 6：基础日报必须先本地生成，再可选 AI 润色

- **Decision**: 先生成本地基础日报文本，再将其作为 AI 润色输入。
- **Rationale**: 这样即使 AI 配置缺失或网络失败，用户仍能完成日报输出；也有利于限制 AI 对原始事实的放大。
- **Alternatives considered**:
  - 直接将任务列表交给 AI 生成最终日报：依赖网络，失败时无法维持闭环。

## 决策 7：MVP 纳入最小托盘能力

- **Decision**: MVP 包含最小托盘入口。
- **Rationale**: 快捷键和提醒都依赖应用常驻，托盘能提供可见运行状态和手动打开入口。
- **Alternatives considered**:
  - 无托盘，仅后台运行：对用户不透明，容易误判应用已退出。
