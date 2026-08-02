# Issue #138 候选前沿耗尽自治终态报告

## 当前状态

- state: LOCAL_GREEN_PENDING_DELIVERY
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
- RED 保留 76 条既有合同通过，并精确失败 5 项：candidate exhaustion 策略未验证、hard stop 未要求、
  正确 snapshot 无专用终态、required label 无专用 reason、task-kind helper 无新 marker 参数。
- GREEN 后完整 CI 脚本合同为 `80/80`，策略验证通过，规范化策略哈希为
  `sha256:c907410a535020e9276fd8de5f448fca38a91ebd84aa26e8768a51818f300d53`。
- 合法 terminal snapshot 返回 `blocked-candidate-exhausted / await-owner-decision`；label、main、clean、
  三方 Head、零 scope、Release Audit、活动或已交付 PR 均有 fail-closed 负例。
- 首次真实 live 预检暴露 label 数组被 PowerShell 函数展开；改用属性投影并加入空/单 label 回归后，
  #138 继续正确返回 `active-worktree-execution / resume-worktree`。
- 首轮 Final Head 独立复审为 `0 Critical / 2 Important / FAIL`：typed marker 前缀大小写被降级为普通
  `refactor`，`candidate-exhausted` hard stop 的大小写漂移又被宽松成员比较接受。
- 同族变异进一步证明 candidate 对象的单元素数组包装和额外字段会被静默接受，只有 typed marker 的
  Issue 会在严格解析前被 live 预筛忽略。纠正合同稳定得到 `77 PASS / 3 FAIL`，修复后恢复 `80/80`；
  candidate 对象固定为五字段、hard stop 固定字符串数组和精确大小写，所有 typed marker 均进入严格解析。

## 待完成

1. 十二路径本地门禁与独立 review。
2. PR、Hosted CI、精确 Head Squash 与主干验证。
3. 真实 live 候选耗尽终态证明。
