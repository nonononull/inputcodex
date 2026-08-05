# Issue #167 副作用准入矩阵 successor v5 实施计划

> 状态：`LOCAL_GREEN / FINAL_HEAD_PENDING`

## 目标

从 fresh `main@5a7465252b56f7e90673e72d3e02881ac9238141` 独立重建 83-source
side-effect admission matrix，并修复 predecessor 暴露的 Issue 身份边界与 consumed-idle 动作矛盾。
本任务只有一个治理 PR、产品交付为零，不复用或修补 `#165/#166`。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/167
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5194288107
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/167#issuecomment-5194309949
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- predecessor_finding_ref: https://github.com/nonononull/inputcodex/pull/166#issuecomment-5194104949
- baseline_ref: `main@5a7465252b56f7e90673e72d3e02881ac9238141`
- branch_ref: `codex/issue-167-gate-5-side-effect-admission-matrix-successor-v5`
- scope_count: `20`
- scope_hash: `sha256:2a86b376612663f38302cc9c95cd32a25dfd5f0c9d2b24d770cdcd6d95e70113`

## TDD 顺序

1. RED：83-source 矩阵缺失、三类 successor 终态缺失、Issue 身份数组/矛盾值旁路、consumed idle
   仍选择候选。
2. GREEN：建立严格矩阵解析与仓库交叉验证；专用终态只绑定精确 #167 身份；consumed 无任务路径
   统一进入 `blocked-candidate-exhausted / await-owner-decision`。
3. 回归：普通 refactor、upstream-sync、candidate-exhausted、merge 与 post-merge 路径不退化。
4. Closeout：执行 Rust、PowerShell、Policy、Release Audit、Repository Policy、scope 与 Git 空白门。

## 当前实现

- 83 个 `unassessed` source 已一对一写入版本化矩阵，按 feature 聚合为 write `16/70`、process
  `2/5`、network `4/8`；全部保持 `blocked`、owner missing、零实现授权。
- Rust 仓库验证已对 source、feature、主桶、typed owner、blocker 与 Release 绑定执行 fail closed。
- fixed-file tranche 已固定为 `v2 / consumed`；无任务路径不再返回候选选择动作。
- #167 身份读取保留 `url/author_login/number` 原始类型，并拒绝数组、错误标量和 number/URL 矛盾。
- 三类 successor 终态测试调用真实生产状态脚本；Parity 全目标 tests/Clippy、rustfmt、97 条治理合同、
  三份 PowerShell AST、Policy、Release Audit 与 Repository Policy 已全部本地通过。

## 停止门

任一第 21 路、第二 writer、main/Release 漂移、失败分支复用，或独立复审任一 Critical/Important，
立即关闭当前交付并重开 `#140`；不得在同一 PR 修复。
