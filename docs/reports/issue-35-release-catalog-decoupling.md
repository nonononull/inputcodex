# Issue #35：Release 审计基线解耦报告

report_status: ci-repair-verified-awaiting-github-hosted-ci
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/35
branch_ref: codex/issue-35-release-catalog-decoupling
baseline_ref: 939f3454b34e0faa42897be7489b344f2bec1d4c
approved_decision_ref: user-message:approve-issue-35-14-path-scope-2026-07-22
owner_scope_approval_comment_ref: https://github.com/nonononull/inputcodex/issues/35#issuecomment-5051128101
scope_hash: sha256:446444f8cef61de3923d8fe40823ee6b1719a424d9f9e013ee26e70d2f20686a
session_plan_ref: docs/plans/sessions/2026-07-22-issue-35-release-catalog-decoupling.md
implementation_plan_ref: docs/plans/2026-07-22-issue-35-release-catalog-decoupling.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-35-release-catalog-decoupling-runtime.md
adr_ref: docs/adr/0003-release-snapshot-catalog-audit-decoupling.md
agos_status: bypassed-needs-input-unregistered
pr_ref: https://github.com/nonononull/inputcodex/pull/36
ci_ref: https://github.com/nonononull/inputcodex/actions/runs/29957699187
review_ref: https://github.com/nonononull/inputcodex/pull/36#issuecomment-5051536714
merge_ref: none
owner_merge_authorization_ref: none

## 范围与根因

此任务只实现状态和门禁，未更新 `upstream/CodexPlusPlus/` 快照字节。根因是旧验证把 `snapshot` 同时作为“最新完整缓存”和“已完成目录审计”的唯一 Release：缓存上游 `v1.2.42` 时会误判为损坏，机械重写目录 Release 又会伪造一致性。

最终模型保持 `snapshot` 表示完整缓存，新增 `release_audit.catalog_release` 表示目录审计基线。当前 `source-lock.json` 仍是 `v1.2.41` / `3dafffcafb2566a1e8bce4b35671656d6adb3eda` 的 `current` 状态；新 `v1.2.42` 的未来缓存将以独立 PR 进入显式 stale，而不是在本 PR 提前改写上游快照。

## TDD 证据

1. **RED，CI 合同**：新增合同后脚本不存在，`pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1` 以 `CI_CONTRACT_RED_MISSING_IMPLEMENTATION` 和退出码 `10` 失败。
2. **RED，Rust 行为**：新增目录审计用例后，`cargo test -p inputcodex-parity --test catalog_repository --offline release_audit_显式解耦快照与功能目录审计基线` 以三处 `E0599`（缺少 `requires_reaudit`）和退出码 `101` 失败。
3. **GREEN，CI 合同**：实现门禁与 workflow 接线后，`Test-CiScripts.ps1` 输出 `CI_CONTRACT_GREEN passed=32`。
4. **GREEN，Rust 行为**：同一 Rust 定向命令通过，结果为 `1 passed; 0 failed`，用时 `0.39s`。
5. **夹具排错**：发现 `[string]` 会将 `$null` 转为空字符串，修正为 `[object]` 后保持 JSON `null`；根因、处理与复现证据已写入 `err.md`。
6. **空变更集回归**：复核发现 `[] | ConvertFrom-Json` 会折叠为 `$null`；新增 `current-empty-change-set` RED 用例后，门禁改为零路径处理，CI 合同重新通过。

## 行为结果

- `current`：快照与目录 Release 必须相同，且 stale 字段必须为 `null`。
- `stale-re-audit-required`：允许目录验证成功并让 `requires_reaudit()` 返回 `true`，但必须有非空根因和有效 `inputcodex` Issue URL。
- 非法 stale 组合仍返回 `ReleaseMismatch`；`source-index` 与 feature catalog 只匹配 `catalog_release`。
- 独立 `release-audit` Job 在 PR 上读取 base/head 变更；stale 阻断 `benchmarks/`、`apps/`、产品 crate、`Cargo.toml`、`Cargo.lock`，并阻断同 PR 的 audit 与受阻路径混合。
- `push` 与 `workflow_dispatch` 仅验证状态结构，合法 stale 不会被误报为仓库损坏；`required` Job 已显式依赖 `release-audit`。

## 第一轮 PR CI 根因与修复

- PR `#36` 的首轮 GitHub-hosted CI（run `29957699187`）中，governance、macOS 与 Windows 通过；`release-audit`、Linux Clippy 与其 `required` 汇总失败，因此没有进入 Review 或合并阶段。
- `release-audit` Artifact 证明 base `939f345` 的 `source-lock.json` 仍是 schema 迁移前版本，没有 `release_audit`。门禁错误地校验 base 与 Head 使用相同的新 schema，导致三条 `RELEASE_AUDIT_INVALID`。修复后只验证 Head 的状态，base 只用于 fingerprint；新增 `current-legacy-base` RED/GREEN 合同。
- Linux Clippy 报 `write_source_lock` 有 `8/7` 个参数。未使用 lint allow，改为 `SourceLockState` 测试状态结构体；本地 `cargo clippy -p inputcodex-parity --tests --locked --offline -- -D warnings` 已通过。
- 修复后的本地证据为：`Test-CiScripts.ps1` 通过、parity `catalog_repository` 为 `10/10`、定向 Clippy 通过；提交前 Fresh 核验确认 `main` 仍为 `939f345`、上游最新正式 Release 仍为 `v1.2.42@657cd33`。修复后的 Head 必须等待同一 PR 的新 GitHub-hosted CI 结果。

## 未完成交付

- PR `#36` 已创建；修复后的 Head 尚未取得新的 GitHub-hosted CI，首轮失败不能视为通过证据。
- 尚未取得修复后 GitHub-hosted 全量 CI、Review 对话、最终 Head 或 Squash Merge 证据。
- 项目所有者的范围批准不包含合并授权；在 PR CI 全绿、Review 对话为零且范围保持不变后，仍必须请求独立 Squash Merge 授权。
- 此报告不关闭 Issue `#35`，不创建额外 Closeout Issue，也不授权更新 Issue `#34` 的上游快照。
