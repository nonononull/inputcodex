# Issue #161 嵌套 snapshot 对象边界实施计划

> 状态：`LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`

## 目标

执行 `gate5-nested-snapshot-object-boundary-recovery-v1`：merge 与 post-merge gate 只接受具体
`System.Management.Automation.PSCustomObject` 形式的 `evidence`、`planning_evidence`、
`review_attestation` 与 `post_merge`，拒绝 PowerShell 管道展开后的数组伪装。

本任务只有一个治理 PR、产品交付为零；不恢复 admission matrix，不修改 policy、产品、Parity、Cargo、
Workflow、Release、upstream、UI 或 AGOS。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/161
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5189227962
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/161#issuecomment-5189247475
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- failed_review_ref: https://github.com/nonononull/inputcodex/pull/160#issuecomment-5188910901
- baseline_ref: `main@f3e7d6f873f59399e71b602e1a9fbdee71760d64`
- branch_ref: `codex/issue-161-gate-5-nested-snapshot-object-boundary-recovery`
- scope_count: `8`
- scope_hash: `sha256:8f336603e25d05f96a45c9e57e0adb4724ebbfe19e6194097cc780d036d1ce40`
- execution_limit: `12h / 6 iterations`

## TDD 顺序

1. RED：7 个端到端单元素数组反例与 2 个直接生产 gate 的形状矩阵。
2. GREEN：只在两个 gate 使用原始 property projection 与具体对象类型检查。
3. 回归：保留 task kind 数组身份，普通 PR、post-merge、upstream-sync 与 candidate-exhausted 不退化。
4. Closeout：执行八路径、AST、Policy、Release Audit、Repository Policy、live 与 Git 空白门。

## 本地证据

- 可信 RED：既有 `85` 项通过，新增 `9` 项精确失败。
- 首轮 GREEN 发现并纠正一个 `if` 输出管道展开 task kind 的旧回归。
- 最终 GREEN：`CI_CONTRACT_GREEN passed=94`，两份 PowerShell AST 零错误。

## 停止门

任一第九路径、第二 writer、main/Release 漂移，或独立复审任一 Critical/Important，立即关闭当前交付并
重开 `#140`；不得在同一 PR 修复。
