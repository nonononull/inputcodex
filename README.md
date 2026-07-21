# inputcodex

`inputcodex` 是面向 Codex 本地增强与管理场景的新项目，当前处于重构筹备阶段。

## 项目目标

- 软件名称统一为 `inputcodex`。
- 产品与代码中不引入广告、推广位或广告 SDK。
- 优先解决卡顿、功能加载异常和模块职责混乱问题。
- 在证据充分后重新确定架构，不默认照搬现有实现。

## 参考项目

- 上游项目：`BigPizzaV3/CodexPlusPlus`
- 半成品参考：`zsr131550/CodexPlusPlus`

两份参考项目均采用 GNU AGPLv3。当前仓库同样采用 GNU AGPLv3，但尚未导入任何参考项目源码。

## 当前阶段

截至 2026 年 7 月 21 日，已完成：

- 建立本地 Git 仓库与 GitHub 公开仓库。
- 固定项目名称、无广告原则和性能优先目标。
- 建立项目级构建、排错、计划与执行约束文档。
- 建立筹备 Issue，承载后续架构讨论与决策证据。

当前明确不做：

- 不导入上游或半成品源码。
- 不提前选择最终技术栈或架构。
- 不在缺少测量证据时直接开展大规模重写。

## 下一步

下一轮先审计两个参考仓库，再讨论并选择以下路线之一：

1. 从零重写，只复用经审计确认的行为与协议。
2. 以半成品为基础做渐进式重构。
3. 建立新壳与稳定核心，按模块择优迁移。

架构方案、性能基线、模块边界和首个可验收版本将在讨论通过后写入项目计划。

## 项目文档

- 构建说明：`build.md`
- 排错记录：`err.md`
- 项目总计划：`docs/plans/PROJECT-MASTER-PLAN.md`
- 本次筹备计划：`docs/plans/2026-07-21-bootstrap.md`
- 本次会话计划：`docs/plans/sessions/2026-07-21-inputcodex-bootstrap.md`
- 本次运行工作流：`docs/workflows/2026-07-21-inputcodex-bootstrap-runtime.md`

## 许可证

本项目采用 GNU Affero General Public License v3.0，详见 `LICENSE`。
