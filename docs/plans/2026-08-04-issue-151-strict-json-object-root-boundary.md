# Issue #151 StrictJsonObject 根类型边界实施计划

> 状态：`LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`

## 目标

执行 `gate5-strict-json-object-recovery-v1` 第一批治理交付：自治 live 状态的
`ConvertFrom-StrictJsonObjectOutput` 只接受 JSON object 根，明确拒绝 `[]`、`[{}]` 与
`[{},{}]`。本任务不修改调用方语义，也不恢复、合并或继续修改已关闭的 PR `#150`。

本任务产品交付为零，不修改产品、Parity、Cargo、Workflow、Runner、Ruleset、Release、上游快照、
UI 或 AGOS。第一批合并并完成 fresh-main 复验前，禁止启动 admission matrix successor。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/151
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5180271732
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/151#issuecomment-5180285101
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- failed_delivery_ref: https://github.com/nonononull/inputcodex/pull/150
- failed_review_ref: https://github.com/nonononull/inputcodex/pull/150#issuecomment-5178304101
- baseline_ref: `main@c26e97ee534b74ebe1252346477640dc196f89b9`
- branch_ref: `codex/issue-151-strict-json-object-root-boundary`
- scope_count: `9`
- scope_hash: `sha256:3ea6c5882d1e5c96708a4e9cdc837ba4c67209a49ebc45d9ad19b991efe7301c`

## TDD 顺序

1. RED：从生产脚本 AST 提取真实 helper；既有 `84` 项通过，唯一新增合同证明数组根被接受。
2. GREEN：只把 `[pscustomobject]` 替换为具体 `System.Management.Automation.PSCustomObject`。
3. 回归：合法 `{}` 保持接受，三种数组根全部拒绝；完整 CI 合同达到 `85/85`。
4. Closeout：执行九路径范围、AST、Release Audit、自治策略、仓库政策、live 状态与 Git 空白检查。

## 本地证据

- RED：既有 `84` 项通过，唯一新增生产 helper 合同失败。
- GREEN：`CI_CONTRACT_GREEN passed=85`。
- 两份 PowerShell AST 零错误；Release Audit 为 `current / requires_reaudit=false / errors=[]`。
- 自治策略与仓库政策零违规；live 精确恢复 Issue `#151`，selected candidate 为 `null`。
- 实际范围精确九路径，scope hash 与 Planning Freeze 一致，`git diff --check` 通过。

## 停止门

任一第十路径、第二 writer、产品或 Release/catalog 漂移，或 Final Head 独立复审出现任何
Critical/Important，立即停止。只有本批 Squash Merge 且 fresh main 全门通过，才允许从 fresh main
新建第二个且最后一个治理 PR；第二批仍为零产品交付。
