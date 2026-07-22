# Session Plan：Issue #28 Gate 4 功能目录执行合并证据收口

session_status: approved-executing-control-plane
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/28
source_issue_ref: https://github.com/nonononull/inputcodex/issues/26
source_pr_ref: https://github.com/nonononull/inputcodex/pull/27
branch_ref: codex/issue-28-gate-4-feature-catalog-closeout
baseline_ref: a9b20f00ae069aedd42c8124d2789b230187258c
approved_decision_ref: user-message:approve-gate-4-closeout-issue-2026-07-22
scope_hash: sha256:91cd1bd908b61e32c573706f26a4bb5d09c6cf5371382ebc0d14d87ae7a4fc29
allowed_operations: project-doc-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 关闭已合并 Gate 4 功能目录执行的控制面漂移，不增加任何产品、性能、上游或 CI 能力。
executor_enforcement: 每次写入前后执行 Git 快照、路径白名单、GitHub Fresh 证据和轻量验证；任何漂移立即停止。

## 一、已确认的输入

- 项目所有者于 `2026-07-22` 批准建立 Gate 4 Closeout Issue；该批准覆盖“文档与合并证据回写”，不覆盖性能基线、性能优化、产品迁移或最终 PR 合并。
- Issue `#28` 已建立，标签为 `gate:4` 与 `type:architecture`。
- 当前隔离工作树位于 `C:\Users\dashuai\Documents\inputcodex-worktrees\issue-28-gate-4-feature-catalog-closeout`；主工作树保留在干净的 `main`。
- 来源 Issue `#26` 已关闭，来源 PR `#27` 的 Head `1d1bf32cdc4edc45e2d28f1047604222ebdb51e4` 已以 Squash 提交 `a9b20f00ae069aedd42c8124d2789b230187258c` 进入 `main`。
- 来源 PR 六项 CI 与合并后 `main` CI 均已成功；这些事实仍须在最终变更前 Fresh 复核。

## 二、范围锁定

允许路径为：

```text
README.md
docs/plans/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md
docs/reports/issue-26-gate-4-feature-catalog.md
docs/reports/issue-28-gate-4-feature-catalog-closeout.md
docs/workflows/2026-07-22-issue-28-gate-4-feature-catalog-closeout-runtime.md
```

路径升序、LF 分隔、末尾 LF 的 SHA-256 为 `91cd1bd908b61e32c573706f26a4bb5d09c6cf5371382ebc0d14d87ae7a4fc29`。

以下表面永久禁止写入：Cargo、Rust、`parity/`、`benchmarks/`、`.github/`、`scripts/ci/`、`upstream/`、Ruleset、Release、Issue `#16/#20` 和 AGOS 跨仓文件。

## 三、AGOS 与本项目控制面

- 已执行 `invoke-agos-default-entry.ps1 -ReportOnly`，任务 ID 为 `inputcodex-issue-28-gate-4-feature-catalog-closeout`。
- 结果为 `AGOS_DEFAULT_ENTRY_STATUS=needs-input`、`DEFAULT_ENTRY_TASK_REGISTRATION_STATUS=unregistered`，并明确禁止其框架下的项目文档写入。
- 按 `inputcodex` 项目规则，该外部状态只记录为 `agos_status: bypassed-needs-input-unregistered`；不修改 AGOS 的 Registry、规则、Workflow、Vault 或脚本，不将其阻塞传播到本 Issue。
- 任务继续使用本项目原生控制面：`AGENTS.md`、`README.md`、`build.md`、`err.md`、Master Plan、Session Plan、Runtime Workflow、报告、Issue、PR 与 GitHub CI。

## 四、执行检查点

### Checkpoint A：控制面骨架

1. 新建实施计划、Session Plan、Runtime Workflow 和初始 Closeout 报告。
2. 验证只新增四条路径，范围哈希未漂移，工作树没有范围外变更。
3. 普通提交并推送；在 Issue `#28` 回写提交、文档引用、范围哈希、AGOS 绕过原因和后续写入范围。

### Checkpoint B：事实回写

1. Fresh 查询 Issue `#26`、PR `#27`、PR Head、Squash 提交、Review 对话、PR CI、合并后 main CI、远端分支和本地分支清理状态。
2. 更新 README、Master Plan、来源报告与 Closeout 报告，禁止将来源 PR 的动态字段伪造成来源提交的一部分。
3. 验证三份既有状态页不再把 Gate 4 功能目录标记为“待创建 PR / 待 CI / 待合并”。

### Checkpoint C：PR 就绪

1. 验证允许路径、仓库政策、文本控制字节与 `git diff --check`。
2. 普通提交、普通推送、创建非 Draft PR，并在 Issue `#28` 回写最终候选 Head 与 PR 引用。
3. 等待最终 Head 的 GitHub-hosted CI；逐条处理 Review 对话。

### Checkpoint D：合并前门槛

1. Fresh 核对 PR 的 Head、路径、Ruleset、维护者数量、Review 对话、自动合并状态和 CI。
2. 在 PR 或关联 Issue 留存项目所有者对该 Closeout PR 的明确 Squash Merge 决策。
3. 未获得该决策时保持 PR 开放，不执行合并、force push 或任何历史改写。

## 五、验证合同

- 文档和治理层：允许路径集合、范围哈希、状态文本、Issue/PR/CI 证据、仓库政策、文本控制字节和 Git 空白错误。
- Rust 与产品层：本 Issue 没有产品代码变更，不在本机执行全量 Rust 编译；现有 GitHub Actions 负责 PR 与合并后主干的完整 CI。
- 失败处理：先查 `err.md`；重复 PowerShell 反引号控制字节问题使用单引号/补丁文本并重跑控制字节扫描；AGOS `needs-input` 直接按本节绕过。

## 六、暂停与升级条件

- 任何 Fresh 事实与冻结基线不一致、Review 对话未闭环、CI 失败、Ruleset/维护者数量变化或范围外路径出现时暂停。
- 性能基线、预算、产品迁移、优化、一致性例外和上游快照变化必须由新的独立 Issue、Session Plan、Runtime Workflow、范围哈希与项目所有者批准承载。
