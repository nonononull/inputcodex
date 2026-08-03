# Issue #145 Gate 5 副作用准入矩阵实施计划

> **状态：** `LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`
>
> **执行纪律：** 单 writer；先以真实负例证明旧 Watcher 候选会被重选及矩阵缺失，再按 Control Plane -> Parity Admission -> Repository Validation -> Closeout 顺序修复。

## 目标

执行 `gate5-side-effect-admission-matrix-v1` 单批治理/Discovery：消费已完成的 fixed-file mutation tranche，阻止 Watcher 被重复选择，并为当前 83 个 `unassessed` source 建立机器可验证的副作用准入矩阵。

本 Issue 不实现产品功能，不改变任何 Parity disposition，也不授权矩阵条目进入实现。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/145
- owner_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5168939607
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/145#issuecomment-5169299292
- baseline_ref: `main@42c73f401e7a758cdc5eca374613625dad46340b`
- branch_ref: `codex/issue-145-gate-5-side-effect-admission-matrix`
- candidate_scope: `20`
- candidate_scope_hash: `sha256:b7955bb33dac2a5f58990dfbe2aff22cc6145a2b60e601e6de255bc0a8f4360f`
- release: `v1.2.44@77091ccaee4423f35a1b2c51c4ecd703e6201092`
- product_count_delta: `0`

## 领域语义

- **准入矩阵**只记录副作用边界、typed owner 完整度和阻断事实，不是实现授权。
- **Consumed tranche** 是不可复活的历史交付状态；候选字段只作历史证据，不得进入选择器。
- **能力桶**按 feature 聚合并使用固定优先级 `write > process > network`；write 桶仍可包含进程或网络副作用。
- 每个 source 必须直接携带 feature、能力桶、typed owner、阻断引用、`blocked` 与 `implementation_authorized=false`。

## TDD 顺序

1. Control Plane RED：证明 clean main 无活动 Issue 时仍会选择已完成 Watcher；证明 consumed/matrix policy 缺失或类型漂移未被拒绝。
2. Control Plane GREEN：固定 consumed lifecycle、新 matrix policy 和无授权候选 fail-closed 状态；保留普通 refactor、upstream-sync、PR/merge/post-merge 行为。
3. Parity RED：证明矩阵 parser、83-source 集合、feature 聚合桶与完整仓库接线缺失。
4. Parity GREEN：新增严格 schema、83 行矩阵、动态集合对照和 `validate_repository` 接线。
5. Closeout：修正 Master Plan，更新术语、Parity 说明、build/err/report，并执行二十路径门禁。

## 本地实施证据

- Control Plane RED：完整治理套件保留历史通过项并稳定暴露 7 个失败，覆盖旧 tranche 未消费、matrix policy/helper 缺失和无任务时重复选择 Watcher。
- Control Plane GREEN：fixed-file tranche 升级为 `v2 / consumed`，新增严格 matrix policy，空闲终态固定为 `blocked-hard-stop / stop / NO_AUTHORIZED_CANDIDATE` 且 `selected_candidate=null`。
- Parity RED：新 admission 类型、解析器、验证码和仓库接线均不存在，定向测试以 9 个编译错误精确失败。
- Parity GREEN：独立 admission 测试 `5/5`、完整目录测试 `33/33`、Parity all-targets 与 Clippy 通过；矩阵动态复算为 22 feature / 83 source。

## 完成终态

- source：`135 = 19 implemented / 83 unassessed / 30 exception-pending / 3 excluded`
- feature：`46 = 13 implemented / 22 unassessed / 11 exception-pending`
- contract：`46`；fixture manifest：`12`
- admission matrix：`83 = write 70 + process 5 + network 8`
- product deliveries：`0`
- selected candidate：`null`
- `gate_5_product_complete=false`；`gate_6_unlocked=false`

## 硬停止

出现第 21 路径、产品实现、目录 disposition/计数变化、新依赖、Workflow/Runner/Ruleset、Release/upstream/UI/AGOS、secret、SQLite、进程或网络执行、`unsafe`/FFI/VFS，或任何 #132/#133 写操作时立即停止并重开 #140。
