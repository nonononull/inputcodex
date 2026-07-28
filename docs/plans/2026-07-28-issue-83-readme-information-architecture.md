# Issue #83 README 信息架构与文档入口设计

## 控制元数据

```yaml
task_id: issue-83-readme-information-architecture
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/83
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/83
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/83#issuecomment-5101769289
branch: codex/issue-83-readme-information-architecture
baseline_main: da65f7d8402e4de27e2795ee8905be18ad565653
planning_scope_count: 3
planning_scope_hash: sha256:83c915a75626bbfb31d9520a519dba3a5a210adc8b47a535f46fc21412c3a95f
candidate_scope_count: 10
candidate_scope_hash: sha256:d8a404c19b108587a5e17b4ded454444d5e948c92410b759504a7eb7c63bed44
delivery_contract: agos.issue-pr-merge.v1
allowed_operations: plan,documentation-edit,local-light-verification,git-checkpoint,commit,push,non-draft-pr,review-ci
mutation_intent: separate-stable-readme-from-dynamic-governance-history
executor_enforcement: exact-path-scope-and-no-product-surface-hard-stop
final_merge_authorization: pending-separate-gate
```

## 问题事实

- `README.md` 当前为 `193` 行、`23,425` 字节、`158` 个列表项；
- README 包含 `121` 个 Issue 引用、`41` 个 PR 引用、`34` 个完整提交 SHA
  和 `18` 个 CI Run；
- “当前阶段/边界”与“项目文档”合计占 README 的 `84.5%`；
- `main` 的 `38` 个提交中有 `29` 个修改 README，动态状态持续推动首页膨胀；
- PR #82 已合并，但 README、Master Plan 与 build.md 仍保留合并前活动状态；
- `docs/` 已有 `146` 份文档，却没有稳定的分类入口；
- 产品当前仍处 Gate 5 分域迁移阶段，桌面展示层只有最小空容器，不能把项目描述成
  已具备完整终端用户体验。

## 已批准方案

项目所有者批准方案 B“稳定首页 + 文档门户”，并固定以下硬约束：

1. README 是公开项目首页，最终用户是第一读者，开发者是第二读者；
2. 治理历史、逐 Issue 时间线、提交与 CI 证据全部下沉；
3. README 必须诚实说明项目仍处开发阶段，不把领域能力描述成完整 UI 产品；
4. 本任务不新增自动生成器、文档 CI 或重型治理门禁；
5. 本任务不夹带第四个 Gate 5 功能、产品代码、UI、上游同步或性能工作。

## 文档职责

| 文档或证据 | 唯一职责 |
| --- | --- |
| `README.md` | 稳定产品定位、当前可用能力、架构入口、构建入口、贡献与许可证 |
| `docs/README.md` | 按主题提供精选入口和目录级导航，不复制全部文件清单 |
| `docs/plans/PROJECT-MASTER-PLAN.md` | 产品 Gate、稳定决策、下一合法产品阶段；不追踪每个维护任务的动态 PR 状态 |
| `build.md` | 可重复构建、测试和验证命令；不承担合并流水账 |
| `err.md` | 可复用根因、处理方式和验证证据 |
| task-local Plan / Session / Workflow / Report | 单个 Issue 的范围、执行、验证和收口证据 |
| GitHub Issue / PR / Actions | 动态 Head、Review、CI、授权与合并后事实 |

## README 目标结构

1. 项目定位与开发阶段警告；
2. 核心原则：Rust、Iced、无广告、性能优先、Windows/macOS 一致；
3. 当前已迁移能力：平台路径、应用概览只读事实、版本与启动意图；
4. 七成员 Workspace 架构表；
5. 构建与验证入口；
6. 上游功能真源、一致性例外与明确非目标；
7. 精简文档入口；
8. 贡献流程与许可证。

README 禁止出现：

- 活动分支、待推送、待建 PR 或待合并步骤；
- 逐 Issue/PR 历史时间线；
- 完整提交 SHA、Git tree、Actions Run 编号和 Artifact 流水账；
- 逐文件枚举 Session Plan、Runtime Workflow 和历史报告；
- 将空展示层描述成已经完成的管理界面；
- 把上游审计快照描述成产品运行依赖。

## 文档门户结构

`docs/README.md` 只维护以下分类入口：

- 项目控制面；
- 架构决策；
- Gate 5 已迁移能力；
- 上游审计与功能一致性；
- 性能基线与预算；
- 历史计划、运行工作流和报告目录。

每类允许精选少量长期入口，其余内容链接到目录，不复制 `146` 份文档清单。

## 候选完整实施范围

```text
AGENTS.md
README.md
build.md
docs/README.md
docs/plans/2026-07-28-issue-83-readme-information-architecture.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-83-readme-information-architecture.md
docs/reports/issue-83-readme-information-architecture.md
docs/workflows/2026-07-28-issue-83-readme-information-architecture-runtime.md
err.md
```

Ordinal 排序、UTF-8 无 BOM、末尾单个 LF 的路径清单哈希固定为：

`sha256:d8a404c19b108587a5e17b4ded454444d5e948c92410b759504a7eb7c63bed44`。

## 禁止范围

- `apps/`、`crates/`、`parity/`、`upstream/`、`benchmarks/`；
- `Cargo.toml`、`Cargo.lock`、Rust 源码或依赖；
- `.github/workflows/`、Ruleset、CI required checks；
- UI、视觉、交互、网络、文件写入、线程或更新能力；
- 删除历史计划、报告或工作流；
- 修改或优化 AGOS；
- 本 Issue 的递归 Closeout。

## 验收标准

1. README 控制在约 `70–100` 行，并且不含禁止的动态证据；
2. README 准确描述三个已迁移 Gate 5 能力和最小展示层现状；
3. `docs/README.md` 提供稳定分类导航且所有相对链接存在；
4. AGENTS 固定文档职责和 README 防回退规则；
5. Master Plan 与 build.md 不再宣称 Issue #81 等待推送或 PR；
6. 新增的 worktree ignore 查询根因进入 `err.md`，既有 PowerShell Markdown 根因只引用不重复；
7. CI 合同 `35/35`、Repository Policy `0` 违规、Markdown 链接检查和
   `git diff --check` 通过；
8. 变更保持纯文档分类，不触发本地 Rust 重型构建；
9. 非 Draft PR 停在项目所有者独立 Squash Merge 授权门。

## 实施顺序

1. Planning：创建三份规划控制面并验证 planning scope；
2. README：按目标结构重写首页；
3. Docs Portal：新增分类入口；
4. Boundaries：更新 AGENTS、Master Plan 和 build.md 的职责与稳定状态；
5. Evidence：记录新根因并创建实施报告；
6. Verification：执行范围、链接、政策与纯文档分类验证；
7. Delivery：提交、普通推送、非 Draft PR、Review/CI 和独立合并授权。

## 当前进度

- [x] Issue #83 已创建并保存项目所有者批准方案；
- [x] 隔离 worktree 与分支基于最新 `main` 创建；
- [x] 基线 CI 合同 `35/35`、Repository Policy `0` 违规；
- [x] planning/candidate scope 与哈希已计算；
- [x] planning scope Git checkpoint `c313cba06484eb00ccc373e63419602d13192f7c`；
- [x] 项目所有者批准十路径 candidate scope；
- [x] README、文档门户、职责边界、稳定状态和排错记录已实施；
- [x] 最终本地轻量验证通过；
- [ ] 提交、普通推送、非 Draft PR 与 Review/CI；
- [ ] 项目所有者独立 Squash Merge 授权。
