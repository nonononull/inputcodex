# Issue #35 Session Plan：Release 审计基线解耦

session_plan_status: ci-repair-verified-awaiting-github-hosted-ci
task_id: 2026-07-22-issue-35-release-catalog-decoupling
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/35
session_plan_ref: docs/plans/sessions/2026-07-22-issue-35-release-catalog-decoupling.md
implementation_plan_ref: docs/plans/2026-07-22-issue-35-release-catalog-decoupling.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-35-release-catalog-decoupling-runtime.md
report_ref: docs/reports/issue-35-release-catalog-decoupling.md
branch_ref: codex/issue-35-release-catalog-decoupling
baseline_ref: 939f3454b34e0faa42897be7489b344f2bec1d4c
approved_decision_ref: user-message:approve-issue-35-14-path-scope-2026-07-22
scope_hash: sha256:446444f8cef61de3923d8fe40823ee6b1719a424d9f9e013ee26e70d2f20686a
allowed_operations: project-doc-write, rust-validation-write, test-write, ci-script-write, workflow-write, source-lock-metadata-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 让“最新完整缓存”与“已经审计的功能目录”可以有受控间隔；不修改上游快照字节，不实施性能或产品功能。
executor_enforcement: 在隔离工作树实施；每个行为先 RED 后 GREEN；只在批准的十四路径写入；全量 Rust 与三平台 CI 只在 GitHub-hosted runners 执行。
agos_status: bypassed-needs-input-unregistered
owner_merge_authorization_ref: none

## 输入与事实

- 项目所有者的十四路径批准已以 Issue 评论 `5051128101` 回写，允许普通提交、推送和 PR 创建，不授权最终合并。
- 上游 `v1.2.42` / `657cd33e009ad02515d30db6492cd4e669b06318` 是 fresh 核验时的最新正式 Release；缓存中的 `v1.2.41` 是此任务唯一允许写入元数据的基础。
- `BigPizzaV3/CodexPlusPlus` 的 Tauri/React 管理面、注入脚本和远程推荐仍只作为审计输入，不进入 Rust 产品架构。
- AGOS 报告不能替代本任务的 Issue、计划、测试、PR、Review 或 CI 证据；发生 `needs-input` / `unregistered` 时立即绕过。

## 执行批次

1. 回写批准、移除 `status:needs-owner-decision`，然后 fresh 核验 Issue、`main`、上游 Release、分支与 PR 空状态。
2. 写入 Rust 和 PowerShell RED 用例并观察预期失败；不提前写生产验证逻辑。
3. 实现 `release_audit` 结构、Rust 可诊断状态、PR 门禁与 CI 汇总依赖。
4. 回跑 RED 对应 GREEN；记录任何可复用排错结论到 `err.md`。
5. 写入 ADR、构建入口、主计划、运行工作流和报告；验证范围哈希、策略、格式与差异后才允许提交与 PR。

## CI 修复检查点

- PR `#36` 首轮 CI 中，governance、macOS 与 Windows 已通过；release-audit 与 Linux Clippy 失败，`required` 因依赖失败而拒绝。
- release-audit Artifact 显示 base `source-lock` 未包含新字段，故 base 只能作为 fingerprint 而非新 schema 验证对象；已以 legacy-base RED/GREEN 合同锁定。
- Rust 夹具的八参数 helper 触发 Clippy；已重构为状态结构体并以定向 Clippy 验证，不添加 lint allow。
- 提交前 Fresh 核验确认 `main` 仍为 `939f345`、上游最新正式 Release 仍为 `v1.2.42@657cd33`；修复后的 Head 必须等待同一 PR 的新 GitHub-hosted CI 结果。

## 不可变边界

- 不创建额外 Closeout Issue；动态交付证据写 Issue `#35`、后续 PR 及本任务报告。
- 不更新 `upstream/CodexPlusPlus/`；未来 `v1.2.42` 缓存必须由独立 upstream-sync Issue/PR 处理。
- 合法 stale 不是可合并的性能或产品实现许可。性能基线、预算和 Gate 5 产品迁移仍需先恢复 `current`。
- 禁止 force push、删除 `main`、未解决 Review 对话合并、管理员绕过或自动合并。
