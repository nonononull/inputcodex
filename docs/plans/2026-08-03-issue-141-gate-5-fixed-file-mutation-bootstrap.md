# Issue #141 固定文件 mutation tranche 治理 bootstrap 实施计划

## 目标

执行 `gate5-fixed-file-mutation-tranche-v1` 的批次 1，只修复自治治理控制面，使批次 2 的唯一候选
固定为 `feature.foundation-platform.watcher-preference-mutation`。本 Issue 不实现产品功能，不改变
Parity 或产品计数，也不合并 PR。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/141
- owner_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214
- retry_resume_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159471091
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_ref: origin/main@42f12ead9209d6eceb7e355562fcd339f1daae81
- branch_ref: codex/issue-141-gate-5-fixed-file-mutation-bootstrap
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/141#issuecomment-5159504479

## 已选择方案

在自治 policy 中新增精确版本化 `fixed_file_mutation_tranche`：

- 绑定 decision、owner decision、retry resume 与 standing authorization。
- `repository_batches_max=2`，`product_deliveries_max=1`。
- candidate 数组严格只有 Watcher preference mutation 一项。
- 完整 tranche 的条件收益固定为 source `+2 implemented / -2 unassessed`；批次 1 实际变化为零。
- completed 或 hard-stop 后只允许重开同一个 #140，并恢复
  `blocked-candidate-exhausted / await-owner-decision`。

验证器拒绝缺失、额外字段、类型、大小写、顺序或数值漂移，也拒绝重新出现旧 `first_candidate`。
状态解析器在消费验证器输出时再次由生产 helper 做精确投影；空闲态只返回固定候选，活动 #141 时继续
恢复 worktree，不提前进入批次 2。

## Scope Freeze

候选范围为 11 路径，使用 `StringComparer.Ordinal` 排序、LF 分隔并保留末尾 LF，以无 BOM UTF-8
计算得到 `sha256:9b0c24a9844ff02143624843e0762d6f0ed5cc84c618d8ff1b3bea16c76bbce3`。

范围只包含 policy、三份 PowerShell、四份 task-local 文档、Master Plan、build 与 err。产品 Rust、Cargo、
Parity、Workflow/Runner/Ruleset、Release/upstream、UI 与 AGOS 全部禁止；任何路径增删都必须停止。

## TDD 证据

- RED：完整合同稳定得到 4 个失败，分别证明生产 policy 缺少 tranche、真实策略变异无目标、生产 helper
  不存在，以及空闲态仍返回泛化 `select-candidate`；其余既有合同保持通过。
- GREEN：生产策略 13 类真实变异与生产 helper 5 类真实变异全部 fail-closed；完整合同为 `82/82`。
- live：真实 #141 返回 `active-worktree-execution / resume-worktree`，Planning Freeze 有效，reason 为空。
- snapshot：洁净 main 空任务快照只返回 `select-fixed-file-mutation-candidate` 与唯一 Watcher 候选。

## 执行清单

- [x] 启动审计与 #132 指纹复验。
- [x] 在任何编辑前发布 Planning Freeze。
- [x] 完成真实变异 RED 与最小 GREEN。
- [x] 修正 Master Plan 陈旧 #128 active task。
- [x] 维护 task-local 控制面、build 与 err。
- [x] 执行十一路径最终本地门禁与自审。
- [ ] 形成普通提交并普通 push。
- [ ] 创建关联 #141 的 non-Draft PR，绑定 Final Head 与验证证据。
- [ ] 停在 PR Final Head；禁止 auto-merge 与任何形式的合并。

## 完成标准

十一条路径无漂移，策略与 live/snapshot 均 fail-closed，Release Audit current，Repository Policy 零违规，
`git diff --check` 通过，产品计数不变，#132 指纹不变，并交付 non-Draft PR Final Head 后停止。
