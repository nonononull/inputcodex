# Issue #138 候选前沿耗尽自治终态实施计划

## 目标

为 Issue #111 的无人值守控制面补齐候选耗尽终态，使当前基线下没有安全候选时稳定返回
`blocked-candidate-exhausted / await-owner-decision`，而不是继续选择候选或把所有者闩锁恢复为普通规划。

本任务只修治理控制面，不迁移产品能力，不改变 Gate 5 或 Gate 6 的产品状态。

## 已选择方案

采用精确 typed marker 加结构化 label 和零交付门：

- 策略固定 `candidate-exhausted` task kind、唯一 marker、required label、state 和 next action。
- live collector 读取 GitHub Issue label，并把 task kind 与 label 投影到只读 snapshot。
- 状态解析器只在 owner 单一 OPEN Issue、main、三方 Head 一致、洁净零 scope、无 PR 且
  `release_audit=current` 时返回候选耗尽终态。
- marker、label、分支、Head、scope、PR 或 Release 任一漂移均进入稳定 hard stop。

不采用以下路线：

1. 仅靠 `status:needs-owner-decision` label：外部或误标 Issue 可能抢占控制面，缺少版本化合同。
2. 只在空闲分支返回 blocked：无法绑定候选耗尽证据，也无法从 GitHub 恢复。
3. 继续用普通 active Issue：状态仍是 `resume-issue`，不能表达终止语义。

## Task 1：规划冻结

- [x] 绑定 Issue #137 所有者决策与 `origin/main@da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4`。
- [x] 建立 Paseo 隔离 worktree 与分支 `codex/issue-138-autonomous-candidate-exhaustion-state`。
- [x] 冻结七路径规划范围与十二路径候选范围。
- [x] 完成规划轻量验证：CI 合同 `76/76`、仓库政策零违规、Release Audit current、7/7 路径和 Git 空白检查通过。
- [x] 建立普通 planning checkpoint、push 和 Planning Freeze 评论。

规划范围：7 路径，`sha256:2b264858d4a3bcde53885c7121e2ef65e3c9b04cf7f685bf1095a583de32604a`。

候选范围：12 路径，`sha256:aac82afc513192fa9adbe1ed6b85f81c56fa3bd8cd3ea20e718d1e31c794f417`。

## Task 2：策略与 marker RED/GREEN

- [x] RED：有效策略缺少 candidate exhaustion 合同时失败。
- [x] RED：未知、重复、大小写/版本漂移、代码块和后置 typed marker 均失败。
- [x] GREEN：策略、验证器和 task-kind parser 只接受唯一精确合同。
- [x] 建立 policy-marker-green checkpoint。

## Task 3：状态终态 RED/GREEN

- [x] RED：候选耗尽 snapshot 当前因未知 task kind 落入普通 `blocked-hard-stop`，无法返回专用终态。
- [x] 覆盖正确终态、required label 缺失、非 main、脏树、Head 漂移、非零 scope、活动或已交付 PR。
- [x] GREEN：正确状态返回 `blocked-candidate-exhausted / await-owner-decision`，所有负例 fail-closed。
- [x] 保持普通 refactor、upstream-sync、外部重试、PR 与 post-merge 语义不变。
- [x] 建立 state-green checkpoint。

## Task 4：本地收口与交付

- [x] 更新稳定规则、构建入口、Master Plan、报告和新根因记录。
- [ ] 执行 Issue #138 十二路径本地轻量门禁。
- [ ] 完成独立只读复审并关闭全部 Critical/Important。
- [ ] 创建非 Draft PR，核验 Hosted CI、Performance、Artifact、Review thread 和精确 Head。
- [ ] 按 standing authorization 执行 Squash Merge 与主干验证。
- [ ] 创建真实零范围候选耗尽 Issue，验证 live 终态后关闭 Issue #138 并归档工作区。

## 完成标准

- 机器策略显式包含候选耗尽硬停止和版本化 typed contract。
- 终态只在 owner、label、main、Head、零 scope、无 PR 与 current Release Audit 全部成立时出现。
- 重复运行不创建任务或进入规划、执行、PR、合并路径。
- Gate 5 产品完成与 Gate 6 解锁继续为 false。
- 十二路径、独立 review、双 Workflow、Artifact 0、Squash 与合并后 live 证明全部完成。
