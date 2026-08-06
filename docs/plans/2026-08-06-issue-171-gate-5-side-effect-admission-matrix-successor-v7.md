# Issue #171 Gate 5 副作用准入矩阵 successor v7 实施计划

> 状态：`LOCAL_GREEN / DELIVERY_PENDING`

## 目标

从 fresh `main@5a7465252b56f7e90673e72d3e02881ac9238141` 独立建立当前 83 个
`unassessed` source 的副作用准入矩阵，并把严格 JSON policy、successor Issue/PR 身份与已消费
tranche 终态固化为生产验证和永久负例。本任务只交付治理能力，产品交付为零。

## 授权与范围

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/171
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5201101496
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/171#issuecomment-5201183608
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_tree: `348a55ce78d1c9da408238b6d9b63cb2e49e32ba`
- branch_ref: `codex/issue-171-gate-5-side-effect-admission-matrix-successor-v7`
- scope_count: `19`
- scope_hash: `sha256:653c4c927c77de1673a1cc1a21db68a262bf08325cd59a941962a66c88d2ea7d`
- limits: `1 repository PR / 0 product deliveries / 1 writer`

范围排序必须使用 `System.StringComparer.Ordinal`，载荷使用无 BOM UTF-8、LF 分隔并保留末尾 LF。
历史 #169/#170 及更早失败分支只允许引用 review finding，禁止读取实现后复制、checkout、merge、
rebase、cherry-pick 或修补。

## TDD 顺序

1. RED：先加入 root/Release/entry unknown、schema、duplicate、missing、unique disorder、matrix-only
   extra、tag-only、commit-only，以及 Issue/PR 身份和四类终态的生产路径负例。
2. GREEN：新增严格 YAML admission parser 与双向闭包验证；升级 policy projection；对原始
   `PSPropertyInfo.Value` 做精确类型判断；已消费 tranche 空闲时直接等待 owner。
3. 回归：目录保持 `135 source / 46 feature`、`19/83/30/3 source disposition` 与
   `13/22/11 feature status`；产品、Cargo、Workflow、Release/upstream 零改动。
4. 交付：完成本地定向门禁、唯一 non-Draft PR、双独立 Final Head 复审和 exact-head Hosted 门禁。

## 完成判据

矩阵必须与 83 个当前 `unassessed` source 一一对应，统计固定为 write `16 feature / 70 source`、
process-control `2/5`、network `4/8`；所有条目均为 `blocked` 与
`implementation_authorized=false`，`model-catalog` 必须包含 `credential-profile` owner kind。
删除或弱化任一生产拒绝逻辑时，至少一个隔离永久负例必须失败。

## 当前实现证据

- 严格 YAML parser 已拒绝 root、Release 与 entry 未知字段，仓库验证已接入 schema、Release、顺序、
  duplicate、missing、matrix-only extra 及 feature/bucket/owner/blocker/authorization 双向闭包。
- 矩阵与 83 个当前 `unassessed` source 一一对应，统计为 write `16/70`、process-control `2/5`、
  network `4/8`；全部条目为 `blocked / false`。
- fixed-file tranche 已进入 `v2 / consumed`；#171 Issue/PR 身份、通用 owner 原始字符串类型、consumed idle
  与 hard-stop/success/pending 三类 successor 终态已有真实生产路径永久测试。
- `cargo test -p inputcodex-parity`、`cargo fmt --all --check` 与 `CI_CONTRACT_GREEN passed=100` 已通过；
  策略 hash 为 `sha256:cc14052a74407914e683072eed92bf048934451f324d9ee800b69cf7e4d72681`。
- Final scope 已以 19 路 Ordinal+LF hash 通过；PR、双独立复审、Hosted CI、Squash 与 fresh-main
  证据仍未形成，不得提前宣称交付完成。

## 停止门

任一未冻结路径、第二 writer、主干/Release 漂移、产品或 Parity disposition/count 漂移，或独立复审
任一 Critical/Important，立即 hard stop；不得在同一 PR 内修补 finding。
