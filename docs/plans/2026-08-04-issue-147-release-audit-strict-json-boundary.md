# Issue #147 Release Audit 严格 JSON 边界实施计划

> 状态：`LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`

## 目标

执行 `gate5-governance-recovery-v1` 第一批治理交付：Release Audit 的 source-lock 输入只接受 JSON object，
changes 输入只接受 JSON array；所有受信标量保留原始 JSON 类型，禁止 PowerShell 将单元素数组降级为
字符串或整数。自治 live 状态必须以相同规则读取 `release_audit.status`。

本任务不修改产品、Parity、Cargo、Workflow、Runner、Ruleset、Release、上游快照或 AGOS，产品交付为零。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/147
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5174720172
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/147#issuecomment-5174752927
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_ref: `main@42c73f401e7a758cdc5eca374613625dad46340b`
- branch_ref: `codex/issue-147-release-audit-strict-json-boundary`
- scope_count: `9`
- scope_hash: `sha256:720cb6be8c376df908dfddbb718f3595f39d0d116da7d3352b64f5a754a6096c`

## TDD 顺序

1. RED：用真实生产脚本证明 source-lock 根数组、changes 根对象、Release Audit 单元素数组和 live status 数组被接受。
2. GREEN：严格解析根类型；保留属性原始类型；对 schema、状态、引用、SHA、路径与统计标量做精确类型检查。
3. 回归：保持 current/stale、上游同步、快照完整性、变更路径与自治状态既有行为。
4. Closeout：执行九路径范围、AST、CI 合同、Release Audit、自治策略、仓库政策与 Git 空白检查。

## 本地证据

- RED：历史合同保持通过；新增 live typed projection 与 Release Audit 根/标量变异两个合同失败。
- 夹具纠正：`switch` 管道吞掉空 changes 数组造成两个假失败，改为直接赋值后 RED 只剩真实生产绕过。
- GREEN：`CI_CONTRACT_GREEN passed=84`；真实 Release Audit 为 `current / errors=[]`；三份脚本 AST 零错误。

## 停止门

任一第十路径、第二 writer、产品计数变化、Release/catalog 漂移，或 Final Head 独立复审出现任何
Critical/Important，立即停止并重开 #140。第一批合并并完成 fresh-main 复验前禁止启动第二批。
