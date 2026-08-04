# Issue #155 Review ref 与 post-merge 父提交绑定实施计划

> 状态：`LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`

## 目标

修复自治交付控制面的两个身份绑定缺口：独立 Review 评论必须来自当前 PR，Squash Merge 的唯一父提交
必须等于冻结的 `expected_base`。本任务不恢复 PR `#154` 或 admission matrix，产品交付为零。

## 授权与范围

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/155
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5182811601
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/155#issuecomment-5182853973
- finding_ref: https://github.com/nonononull/inputcodex/pull/154#issuecomment-5182631577
- baseline_ref: `main@f3e7d6f873f59399e71b602e1a9fbdee71760d64`
- branch_ref: `codex/issue-155-gate-5-review-parent-binding-recovery`
- scope_count: `8`
- scope_hash: `sha256:16fed02331530651a28e824c1ff1382478511945b4efc6892f4da7b23914247e`

## TDD 顺序

1. RED：直接执行生产 helper/gate，错误 PR/Issue Review ref、错误/缺失/数组 parent SHA 得到 `4` 个失败。
2. GREEN：增加当前 PR comment ref 判定；collector 严格解析 parent SHA；merge/post-merge gate 绑定两种身份。
3. 类型回归：所有新增字段从原始 property projection 读取，拒绝 PowerShell 单元素数组展开伪装。
4. 交付：八路径本地全门、独立 Final Head 复审、Hosted CI/Performance、精确 Squash 与 fresh main。

## 本地证据

- RED：既有 `81` 项通过，新增 `4` 项失败。
- 中间检查：Review ref 修复通过，数组展开仍留下 `2` 项失败。
- GREEN：`CI_CONTRACT_GREEN passed=87`。
- PowerShell AST：生产与测试脚本均为零错误。

## 停止门

任一第九路径、第二 writer、policy/产品/Parity/Cargo/Workflow/Release/upstream/UI/AGOS 漂移，或独立
复审任一 Critical/Important，立即停止且不在同一 PR 继续修复。
