# Issue #141 固定文件 mutation tranche 治理 bootstrap 报告

## 当前状态

- state: ITERATION_3_LOCAL_VERIFIED_PENDING_FINAL_HEAD_REFRESH
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/141
- owner_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214
- retry_resume_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159471091
- baseline_ref: origin/main@42f12ead9209d6eceb7e355562fcd339f1daae81
- branch_ref: codex/issue-141-gate-5-fixed-file-mutation-bootstrap
- candidate_scope: 11
- candidate_scope_hash: sha256:9b0c24a9844ff02143624843e0762d6f0ed5cc84c618d8ff1b3bea16c76bbce3
- policy_sha256: sha256:e19914a62c09e212ab47dfb350d99af2b3f05a5de3467f82b29dc8dec57a843a
- product_count_delta: 0

## 根因与处理

自治 policy 仍保留已失效的 `first_candidate=feature.session-data.token-usage-history`，无法表达 #140 批准的
两批有限 Watcher mutation 路径，也没有机器绑定批次/产品上限、source delta 与重开 #140 的终态。

处理后 policy 只包含一个版本化 fixed-file mutation tranche；验证器和 live 生产 helper 均按精确 shape、
类型、值与数组顺序 fail-closed。空闲状态只投影 Watcher preference mutation，活动 #141 继续恢复当前
worktree，不会提前创建批次 2。

PR #142 iteration 2 复审发现 `Get-PropertyValue` 的 PowerShell 输出管道会把单元素 `System.Object[]` 展开为
标量，导致 9 个应为 JSON string 的 tranche 字段绕过仅有的 `-ceq`。iteration 2 直接从原始
`PSPropertyInfo.Value` 保留运行时类型，再逐项同时执行 `-is [string]` 与 `-ceq`；该根因复用 `err.md`
2026-07-31 的集合返回值展开结论，不重复增加排错条目。

iteration 3 复审证明同一输出管道还会把四个单元素数值数组展开为 Int64 标量。验证器与状态 helper 现均
直接读取 `repository_batches_max`、`product_deliveries_max` 和两项 `expected_source_delta` 的原始
`PSPropertyInfo.Value`，同时要求 `[long]` 与精确值；输出 projection 复用已验证变量。tranche 全字段审计
确认两层均不再通过 getter 读取 typed 字段，数组身份与严格字段集合/顺序门保持不变。

## TDD 与本地证据

- Planning Freeze 在编辑前落盘：11 路径 / `sha256:9b0c24...`。
- RED：4 个预期失败；生产 policy、真实变异目标、生产 helper 与固定候选动作均缺失。
- GREEN：`CI_CONTRACT_GREEN passed=82`；策略 13 类真实变异与 helper 5 类真实变异全部拒绝。
- iteration 2 RED：两个既有 contract test 失败并各自完整报告
  `schema_version/decision_id/owner_decision_ref/retry_resume_ref/standing_authorization_ref/terminal.owner_issue_ref/terminal.action/terminal.state/terminal.next_action`；其余 80 个通过。
- iteration 2 GREEN：两条生产路径的 9 个单元素数组反例全部拒绝，完整合同恢复
  `CI_CONTRACT_GREEN passed=82`，既有自治状态回归全部通过。
- iteration 3 RED：两个既有 tranche contract test 各自精确报告
  `repository_batches_max/product_deliveries_max/expected_source_delta.implemented/expected_source_delta.unassessed`，
  其余 80 个合同通过。
- iteration 3 GREEN：两条生产路径的 4 个单元素数组 Int64 反例全部拒绝，完整合同恢复
  `CI_CONTRACT_GREEN passed=82`；普通 refactor、upstream-sync、candidate-exhausted、PR/merge/post-merge
  回归不退化。
- 策略验证：`ok=true`、0 violations、policy hash `sha256:e19914...`。
- live：`active-worktree-execution / resume-worktree`，active issue #141，Planning Freeze 有效，reason 为空。
- `2026-08-03 13:57:02.097 +08:00` 完成 iteration 3 十一路径预提交门禁：Release Audit
  `current`、Repository Policy `0`、live/snapshot、scope/hash 与 Git 空白全部通过。
- PR #142 旧 Head `c18dbbaa...` 复审为 `0 Critical / 1 Important`，唯一 blocker 已在本轮修复；本地、
  origin 与 GitHub main 仍精确为基线，既有 PR 保持 OPEN、non-Draft，auto-merge 未启用。

## 产品与隔离边界

本批没有修改产品 Rust、Cargo、Parity、Workflow、Release、upstream 或 UI，产品计数保持
`135/45/45/12/11/3/0`。完整 tranche 的 `+2/-2` 只是批次 2 通过全部门禁后的条件收益，不属于本 Issue。

#132 启动与提交前两次复验均保持 head `eb93a2c21ac68070c436df5c413dbe7c109395e7`、ahead `6` /
behind `1`、8 dirty、`+691/-105`、binary diff git hash
`833667cd6a49dad8bced474c040566dd3621510a`；未执行任何写操作。

## 待完成

1. 形成 iteration 3 普通提交并普通 push，刷新既有 non-Draft PR #142 Final Head。
2. 在 PR 留存新 Head 验证证据后等待独立复审/Hosted CI；不得 Resolve、auto-merge 或 merge。
