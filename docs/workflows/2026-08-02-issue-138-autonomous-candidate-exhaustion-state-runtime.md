# Issue #138 Runtime Workflow：候选前沿耗尽自治终态

## Runtime Metadata

- task_id: issue-138-autonomous-candidate-exhaustion-state
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/138
- baseline_ref: origin/main@da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4
- branch_ref: codex/issue-138-autonomous-candidate-exhaustion-state
- planning_scope: 7
- planning_scope_hash: sha256:2b264858d4a3bcde53885c7121e2ef65e3c9b04cf7f685bf1095a583de32604a
- candidate_scope: 12
- candidate_scope_hash: sha256:aac82afc513192fa9adbe1ed6b85f81c56fa3bd8cd3ea20e718d1e31c794f417
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/137#issuecomment-5157787716

## Current Gate

    Issue #137 owner decision
      -> Issue #138 isolated worktree
      -> planning freeze
      -> policy and marker RED/GREEN
      -> terminal state RED/GREEN
      -> local closeout and independent review
      -> non-Draft PR and Hosted CI
      -> exact-head Squash and main verification
      -> live blocked-candidate-exhausted proof

ALLOWED_OPS：十二路径内文档、策略、只读状态解析、PowerShell 测试、普通 checkpoint/push、PR、Review、
Hosted CI、精确 Head Squash、合并后验证与真实终态 Issue。

FORBIDDEN_OPS：产品、Cargo、Parity、Workflow、Ruleset、Runner、Release、upstream、UI、AGOS、依赖、
unsafe、FFI/VFS、网络、进程控制、签名、安装、付费资源、force push 和删除 main。

## Node 0：Startup Baseline

- 状态：completed
- HEAD、origin/main 与 remote main 均为 `da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4`。
- 分支和 Paseo worktree 正确，工作树 clean。
- Git snapshot governance 返回 ready。
- AGOS default entry 为 unregistered/needs-input，已按项目规则绕过。

## Node 1：Planning Freeze

1. 只修改七条规划路径。
2. 固定 planning/candidate scope hash、边界、TDD 与交付节点。
3. 运行文档、范围、仓库政策、Release Audit 和 Git 空白验证。
4. 建立普通 planning checkpoint，push 后在 Issue 写入 Planning Freeze。

状态：completed。CI 合同 `76/76`、仓库政策零违规、Release Audit current、7/7 规划路径和 Git 空白检查
通过；planning checkpoint 与 Planning Freeze 已建立。

## Node 2：Policy And Marker TDD

1. RED：新增 candidate exhaustion 策略和 marker 正负合同，确认生产缺失导致失败。
2. GREEN：最小修改策略与验证器，扩展 task-kind parser。
3. 证明未知、重复、伪装和位置漂移全部 fail-closed。
4. 建立 policy-marker-green checkpoint。

RED 状态：completed。完整合同保留原有 76 条通过，并稳定得到 5 个新失败面：策略漂移未拒绝、
候选耗尽 hard stop 缺失、正确终态未分类、required label 无专用 reason，以及 parser 缺少新 marker 参数。

GREEN 状态：completed。策略验证器严格保留 JSON 标量类型，task kind parser 只接受顶部唯一精确 marker；
未知、重复、大小写/版本、代码块和后置 marker 均 fail-closed。

## Node 3：Terminal State TDD

1. RED：正确候选耗尽 snapshot 当前仍落入 active issue planning。
2. 覆盖 label、main、Head、scope、PR、Release 与 owner 负例。
3. GREEN：只在全部条件成立时输出 blocked-candidate-exhausted / await-owner-decision。
4. 重跑现有状态矩阵，证明普通路径不回归。
5. 建立 terminal-state-green checkpoint。

RED 状态：completed。正确候选耗尽 snapshot 当前落入普通 blocked-hard-stop；label、仓库状态和 delivery
负例尚无候选耗尽专用 reason code。

GREEN 状态：completed。合法 snapshot 返回 `blocked-candidate-exhausted / await-owner-decision`；label、
branch、clean、Head、scope、Release、活动或已交付 PR 均有专用 hard stop，完整合同为 `80/80`。

## Node 4：Local Closeout

1. 更新稳定文档、报告与 err 根因。
2. 执行 build.md Issue #138 完整轻量门禁。
3. 独立 reviewer 只读复核 Final Head 候选，关闭全部 Critical/Important。
4. 建立 local-verified checkpoint 并普通 push。

状态：in-progress。稳定文档与 err 根因已更新，下一节点为十二路径完整门禁和独立只读复审。

## Node 5：Remote Delivery

1. 创建非 Draft PR 并绑定 Issue #138。
2. 回写 Final Head、策略 hash、scope hash、独立 review 与 owner decision ref。
3. 核验 CI 七 Job、Performance 四 Job、Artifact 0、Review thread 0 和 origin/main freshness。
4. 精确 Head Squash Merge，验证单父、tree、签名与主干双 Workflow。

## Node 6：Live Terminal Proof

1. 创建 owner 的 OPEN 零范围 Issue，正文顶部使用新 typed marker并添加 required label。
2. 从 clean main 运行 live ReportOnly。
3. 必须返回 blocked-candidate-exhausted / await-owner-decision，且 active Issue 精确匹配。
4. 重复运行结果幂等，不创建分支、PR、评论或文件。
5. 关闭 Issue #138，保留终态 Issue OPEN，归档本工作区。

## Resume Algorithm

每次恢复先运行状态解析：active Issue #138 恢复当前 worktree；active PR 恢复 Review/CI；合并后先完成
主干验证；新终态 Issue 存在时只允许 await-owner-decision。任何 scope、Head、marker、label 或对象数量
漂移都停止，不自动修复外部状态。
