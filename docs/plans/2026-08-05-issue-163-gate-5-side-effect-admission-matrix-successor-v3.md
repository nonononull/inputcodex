# Issue #163 副作用准入矩阵 successor v3 实施计划

> 状态：`LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`

## 目标

执行 `gate5-side-effect-admission-matrix-successor-v3`：从 fresh `main` 重建覆盖全部 `83` 个
`unassessed` source 的机器可验证副作用准入矩阵，并把已消费的固定文件 mutation tranche 标记为终态。

本任务最多交付一个治理 PR、产品交付为零。矩阵只能描述阻断事实，不授权任何产品实现，不修改现有
feature/source disposition、目录计数、Cargo、Workflow、Release、upstream、UI 或 AGOS。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/163
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5191294948
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/163#issuecomment-5191304794
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_ref: `main@5a7465252b56f7e90673e72d3e02881ac9238141`
- baseline_tree: `348a55ce78d1c9da408238b6d9b63cb2e49e32ba`
- branch_ref: `codex/issue-163-gate-5-side-effect-admission-matrix-successor-v3`
- scope_count: `20`
- scope_hash: `sha256:02b2827a2bddd2f22fe41ea5b16ea2b2d3ba4dd672dda88e9f91b18b342907cc`
- historical_failed_prs_evidence_only: `#146 / #150 / #154`

## 矩阵合同

1. 矩阵与 `source-index` 中 `83` 个 `unassessed` source 一对一，重复、遗漏、未知或归属漂移均失败。
2. 桶优先级固定为 `write -> process -> network`；结果固定为 write `16 feature / 70 source`、
   process `2 / 5`、network `4 / 8`。
3. 每项必须记录 typed owner 的 `missing / partial` 状态、精确 owner kind 和稳定 blocker ref。
4. 每项 `admission=blocked` 且 `implementation_authorized=false`；本 PR 不产生候选。
5. fixed-file tranche 升级为 `v2 / consumed`，live 状态固定 `selected_candidate=null`。
6. GitHub/Paseo 不可用时保留有限外部重试；只有外部快照可信时才能断言
   `NO_AUTHORIZED_CANDIDATE`。

## TDD 顺序

1. RED：缺少 admission 类型、导出、验证码和仓库接线时，定向 Rust 测试产生编译失败。
2. GREEN：新增严格 YAML 类型、83-source 覆盖、桶、owner、blocker 与零授权验证。
3. 治理回归：策略与 live helper 拒绝对象/标量数组伪装，已消费 tranche 不再选择产品候选。
4. Closeout：执行 Rust、Clippy、rustfmt、PowerShell、Policy、Release Audit、Repository Policy、live、
   精确 scope 与 Git 空白门。

## 本地证据

- Parity 全目标、Clippy 与 rustfmt：通过；`admission_matrix 6/6`、`catalog_repository 33/33`。
- PowerShell：三份 AST 零错误，`CI_CONTRACT_GREEN passed=96`。
- Policy 与 Repository Policy：零违规；Release Audit：`current / errors=[]`。
- live：`active-worktree-execution / resume-worktree`，活动 Issue 为 `#163`，产品候选为空。
- 实际路径：精确 20 路，scope hash 与 Planning Freeze 一致。

## 停止门

任一第 21 路、第二 writer、main/Release 漂移、产品或目录状态变化，或独立复审任一
Critical/Important，立即停止当前交付并重开 `#140`；不得在同一 PR 修补 review finding。
