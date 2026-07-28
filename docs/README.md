# inputcodex 文档导航

本页是仓库文档的稳定入口。它按主题提供精选文档和目录导航，不复制全部计划、Session Plan、
Runtime Workflow 或历史报告清单。动态 Head、Review、CI、授权与合并证据保留在对应的
GitHub Issue、PR 和 Actions 中。

## 项目入口

- [项目首页](../README.md)：产品定位、开发状态、已迁移能力和架构概览。
- [项目术语](../CONTEXT.md)：统一领域语言和禁止混用的概念。
- [构建与验证](../build.md)：本地轻量命令、Hosted CI 和发布构建合同。
- [排错记录](../err.md)：可复用根因、处理方式和验证证据。
- [项目执行规则](../AGENTS.md)：产品、开发、Git、Review 和文档职责约束。
- [项目总计划](plans/PROJECT-MASTER-PLAN.md)：产品 Gate、稳定决策和下一合法产品阶段。

## 架构决策

- [ADR 0001：纯 Rust + Iced 桌面架构](adr/0001-pure-rust-iced-desktop.md)
- [ADR 0002：上游快照与自主发布线](adr/0002-upstream-snapshot-and-release-line.md)
- [ADR 0003：Release 快照与功能目录审计解耦](adr/0003-release-snapshot-catalog-audit-decoupling.md)
- [ADR 0004：性能预算政策](adr/0004-performance-budget-policy.md)

完整目录：[docs/adr/](adr/)

## Gate 5 已迁移能力

### 平台路径

- [设计](plans/2026-07-27-issue-75-gate-5-platform-paths.md)
- [实施报告](reports/issue-75-gate-5-platform-paths.md)

### 应用概览

- [设计](plans/2026-07-27-issue-78-gate-5-application-overview.md)
- [实施报告](reports/issue-78-gate-5-application-overview.md)

### 版本与启动意图

- [设计](plans/2026-07-28-issue-81-gate-5-version-startup.md)
- [实施报告](reports/issue-81-gate-5-version-startup.md)

任务级 Session Plan 和 Runtime Workflow 分别归档在
[docs/plans/sessions/](plans/sessions/) 与 [docs/workflows/](workflows/)。

## 上游审计与功能一致性

- [最新正式 Release 快照报告](reports/2026-07-26-upstream-v1.2.43-sync.md)
- [最新功能目录重新审计报告](reports/issue-65-v1.2.43-catalog-reaudit-discovery.md)
- [Parity 行为合同入口](../parity/README.md)
- [上游只读审计缓存](../upstream/CodexPlusPlus/)

功能目录、行为合同、脱敏夹具和一致性例外证据由 `parity/` 维护；审计快照不得参与产品运行。

## 性能基线与预算

- [性能基线报告](reports/issue-32-performance-baseline.md)
- [性能预算方法 Discovery](reports/issue-50-performance-budget-discovery.md)
- [固定队列复测与预算数值](reports/issue-59-epyc-7763-fixed-remeasurement.md)
- [非阻断预算观察](reports/issue-63-performance-budget-ci-observation.md)

性能数据只允许在同平台、同可比环境中解释；自动 observation 不是 required budget。

## 历史证据目录

- [设计与实施计划](plans/)
- [Session Plan](plans/sessions/)
- [Runtime Workflow](workflows/)
- [实施、Discovery 与 Closeout 报告](reports/)

历史文档用于审计和复盘，不应被复制回项目首页作为活动状态。

## 推荐阅读顺序

1. 新用户：项目首页 → 已迁移能力 → 构建与验证。
2. 贡献者：AGENTS → 项目术语 → 项目总计划 → 当前任务计划。
3. Reviewer：任务 Session Plan → Runtime Workflow → 实施报告 → GitHub Review/CI 证据。
