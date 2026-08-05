# Issue #169 副作用准入矩阵 successor v6 实施计划

> 状态：`LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`

## 目标

执行 `gate5-side-effect-admission-matrix-successor-v6`：从 fresh main 独立重建当前 83 个
`unassessed` source 的副作用准入矩阵，并收紧 successor 身份、通用 owner 信任和 consumed tranche
终态。矩阵只表达阻断事实，不批准任何产品副作用。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/169
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5196105071
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/169#issuecomment-5196105940
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_ref: `main@5a7465252b56f7e90673e72d3e02881ac9238141`
- baseline_tree: `348a55ce78d1c9da408238b6d9b63cb2e49e32ba`
- branch_ref: `codex/issue-169-gate-5-side-effect-admission-matrix-successor-v6`
- scope_count: `20`
- scope_hash: `sha256:6d07b55562096afa10d3fa804dbb7d1c462fb1e21f09e499e7f655a527866baa`
- delivery_limit: `1 governance PR / 0 product deliveries`

## TDD 顺序

1. RED：保留原有治理合同，新增九组身份、终态、schema 和矩阵闭包反例。
2. GREEN：以原始 PowerShell property 类型、严格策略投影和 Rust typed schema 最小修复。
3. 闭包：精确覆盖 83 个来源、22 个 feature 与 `16/70、2/5、4/8` 三桶统计。
4. 交付：完成 20 路本地门禁、non-Draft PR、exact-head Hosted CI 和双路独立只读复审。

## 实施边界

- 允许：Parity 静态矩阵/schema/验证测试、自治策略与状态机、任务文档。
- 必须：全部条目 `blocked` 且 `implementation_authorized=false`；fixed-file tranche 为 `v2 / consumed`。
- 禁止：产品、Parity disposition/count、Cargo、Workflow/Runner/Ruleset、Release/upstream、UI、AGOS 和
  任何副作用实现。
- 禁止：checkout、merge、rebase、cherry-pick、复制或修补 #167/#168 及更早失败分支。

## 本地证据

- 可信 RED：原有合同通过，新增九组冻结合同按预期失败。
- GREEN：`CI_CONTRACT_GREEN passed=99`。
- `inputcodex-parity` 全目标测试及 Clippy `-D warnings` 通过。
- 自治策略 `ok=true / violation_count=0`，policy hash 为
  `sha256:d43d31ddb42d23ed4449566dba2af5048da67efc7a0accb26b8c5f91e715ee45`。

## 停止门

任一第 21 路、第二 writer、基线或范围漂移、产品/Parity disposition/count 或禁止面改动，立即 hard
stop。Final Head 任一路独立复审出现 Critical/Important 时，不得在当前 PR 修补，必须关闭交付并重开
`#140`。
