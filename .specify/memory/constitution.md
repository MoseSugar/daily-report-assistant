<!--
Sync Impact Report
- Version change: template/unset -> 1.0.0
- Modified principles:
  - Principle slot 1 -> I. 产品定位与核心闭环优先
  - Principle slot 2 -> II. 低打扰与键盘优先交互
  - Principle slot 3 -> III. 本地优先与数据可靠性
  - Principle slot 4 -> IV. 安全边界与可禁用网络
  - Principle slot 5 -> V. MVP、模块化与可验证交付
- Added sections:
  - 产品边界与范围控制
  - 本项目当前明确不追求的目标
- Removed sections:
  - None
- Templates requiring updates:
  - ✅ updated: .specify/templates/plan-template.md
  - ✅ updated: .specify/templates/spec-template.md
  - ✅ updated: .specify/templates/tasks-template.md
  - ✅ reviewed: .specify/templates/agent-file-template.md
  - ✅ reviewed: .specify/templates/constitution-template.md
- Follow-up TODOs:
  - None
-->
# 日报助手 Constitution

## Core Principles

### I. 产品定位与核心闭环优先
日报助手 MUST 被定义为 Windows 11 桌面应用，而非普通网页应用的桌面包装。
所有产品决策 MUST 服务于核心闭环：快速记录 -> 当日汇总 -> 编辑状态与备注
-> AI 润色 -> 生成并复制可发送日报。任何新增能力若削弱该闭环的完成度、响应速度
或理解成本，必须被降级或推迟到 MVP 之后。

### II. 低打扰与键盘优先交互
系统 MUST 支持通过全局快捷键呼出快速记录窗口；快速记录流程 MUST 以键盘优先，
并以最少步骤完成一次记录。交互设计 MUST 以“尽量不打断用户当前工作”为首要标准；
临近下班的汇总窗口 MUST 清晰展示当日任务，并支持快速批量确认、修改状态和补充备注。

### III. 本地优先与数据可靠性
所有任务记录 MUST 默认保存在本地，并优先保证离线可用。系统 MUST 以“不丢数据、
不误覆盖、不因异常损坏当日记录”为本地数据处理底线；用户每日任务数据 MUST 按日期
清晰组织，以支持读取、汇总、回溯与后续扩展。即使 AI 服务不可用，用户仍 MUST 能
查看、编辑并生成基础日报内容。

### IV. 安全边界与可禁用网络
API Key、Base URL、模型名及同类配置 MUST 被视为敏感信息，不得泄漏到日志、错误弹窗
或调试输出。系统 MUST 默认不向任何未经用户明确配置的远程服务上传任务内容；所有网络
调用 MUST 可被禁用，且禁用后不得影响本地基础功能。AI 润色属于辅助能力，绝不能成为
访问、编辑或生成基础日报的前置条件。

### V. MVP、模块化与可验证交付
实现顺序 MUST 服从 MVP 原则，先完成核心闭环，再考虑增强功能；每次迭代 MUST 聚焦当前
目标，避免与目标无关的重构。架构 MUST 明确区分桌面能力层、数据存储层、业务逻辑层、
AI 接入层和界面层。所有关键功能 MUST 具备明确验收标准；对关键用户流程 MUST 提供
最小必要的验证方式，包括自动化测试、手动验收脚本或可复现步骤之一。

## 产品边界与范围控制

MVP 阶段 MUST 聚焦日报助手核心目标，不得以“平台化”或“未来扩展”为理由引入账号系统、
云同步、多人协作、复杂权限、项目管理面板、周报月报分析等超出核心目标的能力。
AI 润色 MUST 被限制为辅助层，不得反向驱动产品复杂化，不得要求用户为了使用 AI 功能
而承担额外流程负担。任何范围扩展都必须证明其不会削弱快速记录、下班前汇总和日报生成
这三条主线体验。

## 本项目当前明确不追求的目标

本项目当前明确不追求成为任务管理系统、项目协作平台或组织级报表中心；不追求账号体系、
多端实时同步、团队协同编辑、复杂审批流、细粒度权限系统，也不追求围绕 AI 构建高度可
配置的工作台。当前阶段同样不追求周报、月报、绩效分析、跨项目统计面板等扩展分析能力。
判断优先级时，必须优先做对核心功能，而不是做多功能。

## Governance

本 Constitution 高于项目中的其他开发约定；规格、计划、任务拆解与实现评审 MUST 显式
检查是否符合上述原则。修订本 Constitution 时，必须同步说明变更原因、影响范围及对模板
或流程的同步更新；版本号采用语义化规则：新增原则或实质性扩展记为 MINOR，原则删除、
重定义或治理不兼容变更记为 MAJOR，纯澄清性修改记为 PATCH。每个关键功能在进入实现前
MUST 具备明确验收标准，在合并前 MUST 具备最小必要验证记录；若存在违反宪章的复杂度、
联网依赖或范围扩张，必须在计划阶段记录理由并获得明确接受。

**Version**: 1.0.0 | **Ratified**: 2026-03-24 | **Last Amended**: 2026-03-24
