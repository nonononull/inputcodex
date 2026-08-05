# Issue #157 snapshot 标量与 Review ref 终端边界实施计划

> 状态：`LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`

## 目标

执行 `gate5-snapshot-scalar-terminal-anchor-recovery-v1`：从 fresh main 重建 PR `#156` 已验证的当前 PR 与单父绑定，并修复其独立复审确认的终端锚点、`expected_base` 和 `parent_count` 原始类型缺口。

本任务只有一个治理 PR、产品交付为零，不恢复 admission matrix，不修改 policy、产品、Parity、Cargo、Workflow、Release、upstream、UI 或 AGOS。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/157
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5183731074
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/157#issuecomment-5183806138
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- failed_delivery_ref: https://github.com/nonononull/inputcodex/pull/156
- failed_review_ref: https://github.com/nonononull/inputcodex/pull/156#issuecomment-5183389014
- baseline_ref: `main@f3e7d6f873f59399e71b602e1a9fbdee71760d64`
- baseline_tree: `e9d40d6f28f32bde98fbb6c9400f741e6ea1db29`
- branch_ref: `codex/issue-157-gate-5-snapshot-scalar-terminal-anchor-recovery`
- scope_count: `8`
- scope_hash: `sha256:caeb53933c58fc8d5ddf1a90af6274bad8ba1f9292fbaf4bcb4536ea721a41dd`

## TDD 顺序

1. RED：直接抽取生产 helper/collector 并运行完整状态脚本，形成六个精确失败面。
2. GREEN：新增一个严格 Review ref helper；collector、merge 与 post-merge 共享它。
3. GREEN：从原始 property projection 验证 `expected_base`、`parent_count`、Review ref 与 `parent_oid`。
4. 回归：合法当前 PR、单父 Squash 与规范 SHA 保持通过，完整治理合同达到 `90/90`。
5. Delivery：精确八路径提交、non-Draft PR、双路独立复审与 Hosted 门禁。

## 本地证据

- RED：六项失败分别命中缺 helper、collector、merge ref、post-merge ref/父结构和 snapshot scalar。
- GREEN：`CI_CONTRACT_GREEN passed=90`。
- 两份 PowerShell AST 零错误；产品、Parity、Cargo、Workflow、Release 与 upstream 零修改。

## 停止门

任一第九路径、第二 writer、main/Release 漂移，或 Final Head 独立复审出现任何 Critical/Important，立即关闭当前交付并重新打开 `#140`。不得在当前 PR 继续修复，也不得启动 admission matrix successor。
