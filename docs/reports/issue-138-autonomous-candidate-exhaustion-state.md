# Issue #138 候选前沿耗尽自治终态报告

## 当前状态

- state: PLANNING_VALIDATED_CHECKPOINT_PENDING
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/138
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/137#issuecomment-5157787716
- baseline_ref: origin/main@da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4
- branch_ref: codex/issue-138-autonomous-candidate-exhaustion-state
- planning_scope: 7
- planning_scope_hash: sha256:2b264858d4a3bcde53885c7121e2ef65e3c9b04cf7f685bf1095a583de32604a
- candidate_scope: 12
- candidate_scope_hash: sha256:aac82afc513192fa9adbe1ed6b85f81c56fa3bd8cd3ea20e718d1e31c794f417

## 已确认根因

- 策略 hard stop 集合没有候选耗尽。
- 状态解析器无活动任务时无条件选择候选。
- 普通 OPEN owner Issue 只能映射为 active-issue-planning / resume-issue。
- #137 只能作为临时 GitHub 闩锁，不能形成机器终态。

## 决策边界

只修治理状态机。候选前沿饱和、Gate 5 产品完成和 Gate 6 解锁是三个不同事实；本任务只让第一个事实
可被机器表达，后两个继续为 false。

## 当前证据

- main、origin/main、remote main 与 worktree baseline 精确一致。
- 工作树启动时 clean，开放 PR 为 0，release_audit=current。
- AGOS 为 unregistered/needs-input，项目原生入口 ready，已按规则绕过。
- 规划与候选范围均由 Ordinal/LF/UTF-8 无 BOM 算法复算。
- 规划门为 CI 合同 `76/76`、仓库政策零违规、Release Audit current、7/7 路径与 Git 空白检查通过。

## 待完成

1. planning checkpoint 与 GitHub Planning Freeze。
2. policy/marker RED-GREEN。
3. terminal state RED-GREEN。
4. 本地门禁与独立 review。
5. PR、Hosted CI、精确 Head Squash 与主干验证。
6. 真实 live 候选耗尽终态证明。
