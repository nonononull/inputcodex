# Issue #165 副作用准入矩阵 successor v4 实施计划

> 状态：`LOCAL_GREEN / COMMIT_PENDING`

## 目标

执行 `gate5-side-effect-admission-matrix-successor-v4`：从 fresh `main` 独立重建覆盖全部
`83` 个 `unassessed` source 的机器可验证副作用准入矩阵，并把 predecessor 复审指出的三类
终态路径固化为永久生产状态机测试。

本任务最多交付一个治理 PR、产品交付为零。矩阵只能描述阻断事实，不授权产品实现，也不修改
feature/source disposition、目录计数、Cargo、Workflow、Release、upstream、UI 或 AGOS。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/165
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5193014446
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/165#issuecomment-5193029156
- predecessor_finding_ref: https://github.com/nonononull/inputcodex/pull/164#issuecomment-5192773475
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_ref: `main@5a7465252b56f7e90673e72d3e02881ac9238141`
- baseline_tree: `348a55ce78d1c9da408238b6d9b63cb2e49e32ba`
- branch_ref: `codex/issue-165-gate-5-side-effect-admission-matrix-successor-v4`
- scope_count: `20`
- scope_hash: `sha256:26a9f85b8a24ccde06dfe407c84c2fe05b3b8c28d2ad0b80cee071f697954cb4`
- predecessor_branch_reuse: `forbidden`

## 矩阵合同

1. 矩阵与 `source-index` 中 `83` 个 `unassessed` source 一对一，重复、遗漏、未知或归属漂移均失败。
2. 桶优先级固定为 `write -> process -> network`；统计固定为 write `16 feature / 70 source`、
   process `2 / 5`、network `4 / 8`。
3. 每项记录 typed owner 的 `missing / partial` 状态、精确 owner kind 和稳定 blocker ref。
4. 每项必须为 `admission=blocked` 且 `implementation_authorized=false`，不得产生产品候选。
5. fixed-file tranche 必须为 `v2 / consumed`，live 的 `selected_candidate` 必须为 `null`。

## 强制状态机测试

三类测试必须通过 `Invoke-AutonomousStateCase` 或从生产脚本 AST 提取的真实 helper/尾段执行：

1. hard-stop：关闭 successor、重开 `#140`，返回 `blocked-candidate-exhausted / await-owner-decision`。
2. successful post-merge：精确 Squash 与全部主干门完成后返回同一 owner 终态。
3. pending post-merge：任一主干终态证据缺失时保持 pending，禁止提前成功。

## TDD 顺序

1. RED：上述三类状态测试在基线上得到 hard-stop `stop`、成功 post-merge `close-issue-and-archive`、
   pending post-merge `verify-main`；前两类精确失败，pending 保持通过。
2. GREEN：admission 类型、仓库接线与变异测试已覆盖 83-source 完整性、桶、owner、blocker、授权和未知字段。
3. GREEN：严格 policy/helper 投影、已消费 tranche 和 successor 专用终态路由已通过完整合同。
4. Closeout：Rust、Clippy、rustfmt、PowerShell、Policy、Release Audit、Repository Policy、live、
   精确 scope 与 Git 空白门均已通过；下一节点为普通提交、push 与唯一 non-Draft PR。

## 本地证据

- PowerShell AST：三份脚本零错误。
- CI 合同：`CI_CONTRACT_GREEN passed=99`，耗时 `240.1s`。
- Policy：`ok=true`，hash `sha256:6e75a35260c078c7e03b32bbaafd2381e1c4919cd1dc8d7167c052a9a6e858ec`。
- Rust：`cargo test -p inputcodex-parity --all-targets --offline` 全绿；Clippy `-D warnings` 全绿。
- 矩阵：`83 = 70 write + 5 process + 8 network`，`implementation_authorized=true` 为零。
- AGOS：ReportOnly 返回 `blocked / v1-state-unavailable / write-status=report-only`，已按项目规则绕过且未修改 AGOS。
- Release Audit：`current / requires_reaudit=false / errors=[]`；Repository Policy 为零违规。
- live：#165 正文补齐既有自治任务 marker 后，返回 `active-worktree-execution / resume-worktree`，
  `reason_codes=[]`、`selected_candidate=null`。
- scope：精确 20 路，hash `sha256:26a9f85b8a24ccde06dfe407c84c2fe05b3b8c28d2ad0b80cee071f697954cb4`；
  `main == origin/main == remote main == baseline`，`git diff --check` 通过。

## 停止门

任一第 21 路、第二 writer、main/Release 漂移、产品或目录状态变化、predecessor 分支复用，或独立复审
任一 Critical/Important，立即停止当前交付并重开 `#140`；不得在同一 PR 修补 finding。
