# Issue #159 SHA 绝对终止恢复实施计划

> 状态：`LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`

## 目标

执行 `gate5-sha-terminal-anchor-recovery-v1`：从 fresh main 手工重建未合并 predecessor 的必要身份绑定，并让 snapshot `expected_base`、post-merge parent collector 与 post-merge gate 共同拒绝 SHA 尾随 LF/CRLF。

本任务只有一个治理 PR、产品交付为零，不恢复 admission matrix，不修改 policy、产品、Parity、Cargo、Workflow、Release、upstream、UI 或 AGOS。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/159
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5187810181
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/159#issuecomment-5187842988
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- failed_delivery_ref: https://github.com/nonononull/inputcodex/pull/158
- failed_review_ref: https://github.com/nonononull/inputcodex/pull/158#issuecomment-5187767959
- baseline_ref: `main@f3e7d6f873f59399e71b602e1a9fbdee71760d64`
- baseline_tree: `e9d40d6f28f32bde98fbb6c9400f741e6ea1db29`
- branch_ref: `codex/issue-159-gate-5-sha-terminal-anchor-recovery`
- scope_count: `8`
- scope_hash: `sha256:f56d3b5f45794641ddb1186b485bdc23c9c583068d15dbae61ce8f958225bddf`

## TDD 顺序

1. RED：手工重建 predecessor 测试并新增 snapshot、collector、gate 三项 SHA LF/CRLF 合同。
2. 纠正 RED 夹具：补生产依赖，并把 direct gate 的 `parent_count` 固定为 Int64 `1L`。
3. GREEN：三个 SHA 边界改用 `\A[0-9a-f]{40}\z`；其余绑定保持原始投影和精确比较。
4. 回归：Review ref、普通 PR、merge/post-merge、upstream-sync 与 candidate-exhausted 保持通过，完整治理合同达到 `93/93`。
5. Delivery：精确八路径提交、non-Draft PR、双路独立复审与 Hosted 门禁。

## 本地证据

- RED：九项精确失败，其中三项独立证明 SHA 尾随换行绕过。
- GREEN：`CI_CONTRACT_GREEN passed=93`。
- 两份 PowerShell AST 零错误；产品、Parity、Cargo、Workflow、Release 与 upstream 零修改。

## 停止门

任一第九路径、第二 writer、main/Release 漂移，或 Final Head 独立复审出现任何 Critical/Important，立即关闭当前交付并重新打开 `#140`。不得在当前 PR 继续修复，也不得启动 admission matrix successor。
