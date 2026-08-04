# Issue #149 Gate 5 副作用准入矩阵 successor 实施计划

> **状态：** `LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`
>
> **执行纪律：** 单 writer；只从 Batch 1 合并后的 fresh main 重建，不 cherry-pick、merge 或恢复 #145/#146。

## 目标

执行 `gate5-governance-recovery-v1/batch-2`：消费已完成的 fixed-file mutation tranche，为当前
83 个 `unassessed` source 建立机器可验证的副作用准入矩阵，并补齐旧 PR #146 Final Head 剩余的
GitHub 身份绑定 finding。

本 Issue 不实现产品功能，不改变任何 Parity disposition，也不授权矩阵条目进入实现。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/149
- owner_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5174720172
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/149#issuecomment-5176946126
- baseline_ref: `main@c26e97ee534b74ebe1252346477640dc196f89b9`
- branch_ref: `codex/issue-149-gate-5-side-effect-admission-matrix-successor`
- prior_evidence_only: `PR #146 @ de2e7e0015ce1f173b66402fa9a38768ae0e9e64`
- candidate_scope: `20`
- candidate_scope_hash: `sha256:7269cdd0eb2726d967bca4f1f183a2c4f7082bf51312cb19ba31432bae0809c5`
- release: `v1.2.44@77091ccaee4423f35a1b2c51c4ecd703e6201092`
- repository_delivery: `2/2`
- product_deliveries: `0/0`

## 恢复策略

1. 保留 Issue #147 / PR #148 已合入的 Release Audit 严格根对象与原始标量修复。
2. 从 PR #146 Final Head 机械读取 policy、Parity schema、83-source matrix 和已验证测试成果；禁止 cherry-pick、merge 或复用旧提交历史。
3. 将 tracking Issue、baseline、decision、Planning Freeze、scope/hash 与 terminal 全部重绑到 #149。
4. 以真实负例修复 review 分页、push branch、Issue/PR/ref 和 post-merge commit/tree 四项剩余身份 finding。
5. 执行完整二十路径门禁；任一新 finding 或第 21 路立即停止。

## 领域语义

- **准入矩阵**只记录副作用边界、typed owner 完整度和阻断事实，不是实现授权。
- **Consumed tranche** 是不可复活的历史交付状态；候选字段只作历史证据，不得进入选择器。
- **能力桶**按 feature 聚合并使用固定优先级 `write > process > network`。
- 每个 source 直接携带 feature、能力桶、typed owner、阻断引用、`blocked` 与
  `implementation_authorized=false`。
- GitHub 交付证据必须在每个分页和终态重新证明完整标量身份，不从首页或强制字符串投影继承可信度。

## TDD 证据

- RED：在旧移植面上新增四个真实合同，得到 `94 pass / 4 fail`，精确命中 review 分页、push branch、对象身份和 post-merge commit/tree。
- GREEN：新增 typed identity helper 与四个 reason code 后，治理合同为 `CI_CONTRACT_GREEN passed=98`。
- Parity：当前 fresh baseline 上 admission `6/6`、目录 `33/33`、all-targets、Clippy 与 rustfmt 已通过。
- Repository：四份 PowerShell AST、策略、仓库政策、Release Audit、二十路径 scope/hash 与空白门已通过；policy hash 为 `sha256:8ae82c50cfb90257141189bd79097f88c9a505caef026f0002d92d291107bed5`。

## 完成终态

- source：`135 = 19 implemented / 83 unassessed / 30 exception-pending / 3 excluded`
- feature：`46 = 13 implemented / 22 unassessed / 11 exception-pending`
- contract：`46`；fixture manifest：`12`
- admission matrix：`83 = write 70 + process 5 + network 8`
- product deliveries：`0`
- selected candidate：`null`
- `gate_5_product_complete=false`；`gate_6_unlocked=false`

## 硬停止

出现第 21 路、产品实现、目录 disposition/计数变化、新依赖、Workflow/Runner/Ruleset、Release/upstream/UI/AGOS、
secret、SQLite、进程或网络执行、`unsafe`/FFI/VFS、任何 #132/#133 写操作，或独立复审任一
Critical/Important 时立即停止并重开 #140。
