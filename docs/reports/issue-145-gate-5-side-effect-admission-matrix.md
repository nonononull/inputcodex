# Issue #145 Gate 5 副作用准入矩阵报告

## 当前状态

- state: LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/145
- owner_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5168939607
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/145#issuecomment-5169299292
- baseline_ref: main@42c73f401e7a758cdc5eca374613625dad46340b
- branch_ref: codex/issue-145-gate-5-side-effect-admission-matrix
- scope: 20
- scope_hash: sha256:b7955bb33dac2a5f58990dfbe2aff22cc6145a2b60e601e6de255bc0a8f4360f
- product_count_delta: 0

## Discovery 结论

- 当前 live catalog 为 83 个 unassessed source，对应 22 个 unassessed feature。
- Feature 级副作用聚合得到 `write 16/70`、`process 2/5`、`network 4/8`，合计精确 22/83。
- 当前控制面在无活动 Issue 时仍无条件选择已完成的 Watcher mutation；这是本批必须通过真实 RED 消除的控制面缺陷。
- 单独 admission 测试不足以形成仓库门，matrix 必须接入 `validate_repository`。
- Matrix 只允许 `blocked` 和 `implementation_authorized=false`，不表达候选排名或实现承诺。

## 本地交付

- fixed-file tranche 已绑定 `consumed`、完成引用与 main 提交；候选字段仅保留历史证据。
- policy 与状态 helper 对 matrix 授权的字段、类型、大小写、集合、上限和 `implementation_authorized=false` 双重 fail-closed。
- policy、PR evidence 与状态快照保留 JSON 原始类型；根数组和标量单元素数组不能再经 PowerShell 集合展开伪装成合法 object/string/bool/Int64。
- 无活动 owner Issue/PR 时返回 `blocked-hard-stop / stop / NO_AUTHORIZED_CANDIDATE`，`selected_candidate=null`。
- admission schema 使用严格未知字段拒绝；83 行直接记录 source/feature/bucket/typed owner/blocker/admission/authorization，并由 22-feature oracle 精确校验 owner state/kinds 与 blocker refs。
- `model-catalog` 的四个 source 同时绑定 filesystem mutation、network transport 与 credential profile owner；#145 完成或 hard-stop 后进入关闭任务并重开 #140 的复合 terminal action。
- `validate_repository` 已接入矩阵；变异 `implementation_authorized=true` 会由完整仓库验证报告 `AdmissionUnauthorized`。
- 产品、Cargo、Workflow/Runner/Ruleset、Release/upstream、Parity disposition 与计数均未改变。

## 本地验证

- Control Plane RED：`75 pass / 7 fail`。
- 复审纠正 RED：owner/blocker 合法值漂移、model-catalog owner 缺项、#145 terminal 缺失、policy/snapshot 单元素数组穿透与 PR marker 缺失。
- Control Plane GREEN：`CI_CONTRACT_GREEN passed=88`。
- Parity RED：缺失 admission API 与验证码导致 9 个编译错误。
- Parity GREEN：`admission_matrix 6/6`、`catalog_repository 33/33`、Parity all-targets、Clippy 与 rustfmt 通过。
- 仓库政策 `0 violations`；Release Audit `current / requires_reaudit=false / errors=[]`。
- scope 精确 `20`，hash 为 `sha256:b7955bb33dac2a5f58990dfbe2aff22cc6145a2b60e601e6de255bc0a8f4360f`；`git diff --check` 通过。
- policy hash 为 `sha256:39553c8e56ee8938eda39d7f760de2009d79a9b1f1109c1137d302f32cc14ae3`。

## 待完成

1. 修正 Head 的独立 Final Head 复审。
2. 修正 Head 的 CI 7/7、Performance 4/4、Artifact 0、Review thread 0。
3. 精确 Squash、主干复验、关闭 #145 并重开 #140。
