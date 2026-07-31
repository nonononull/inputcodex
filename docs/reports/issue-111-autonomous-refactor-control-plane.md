# Issue #111 无人值守重构控制面报告

## 当前状态

- state: PLANNING_FREEZE_IN_PROGRESS
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/111
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_ref: origin/main@86a7dd837652f63198c7682b84d82180b8558e3a
- branch_ref: codex/issue-111-autonomous-refactor-control-plane
- workspace_ref: paseo-managed-worktree
- planning_scope: 7
- planning_scope_hash: sha256:4c33c7597fcf2afbcc084a8e32560a3ea1801db4d984ccff5b7338c30ef10431
- candidate_scope: 12
- candidate_scope_hash: sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713
- pull_request_ref: pending
- remote_delivery_started: false

## 已确认事实

- Issue #109 / PR #110 已完成，Squash 提交为 86a7dd837652f63198c7682b84d82180b8558e3a。
- 合并后 CI Run 30559880826 七 Job、Performance Baseline Run 30559880720 四 Job全绿，Artifact 均为 0。
- 当前无开放 PR；Issue #54 为历史 STOPPED 状态，Issue #16/#20 为上游监控状态，不属于 active 产品任务。
- 五域目录还有 22 个 unassessed 总功能。
- main Ruleset 19395456 已要求 squash-only 与 Review thread resolution，required approvals 为 0。
- 仓库 allow_auto_merge=false，但仓库级 merge/rebase 开关仍为 true；Issue #111 只在文件策略合并后加固外部设置。
- 本地主工作目录落后 origin/main 三个提交；自治流程不得更新该目录。
- inputcodex 当前没有子 agent；唯一 running agent 是承接主会话。
- 本机 Gemini CLI 存在，Paseo provider registry 没有 Gemini provider。

## 方案结论

选择 GitHub + Paseo 混合自治，不使用单一长会话，也不把 AI 与写权限放入 GitHub Actions。

GitHub 保存 Issue、PR、Head、CI 与合并事实；Paseo 负责单写者、隔离 worktree、只读复审和中断
恢复。GitHub 原生 auto-merge 保持关闭，自治执行器只在精确 Final Head 全门禁通过后执行 Squash。

## Standing Authorization

项目所有者已授权常规重构不再逐项征求意见。授权仅覆盖：

- 既有 inputcodex 重构目标。
- 最新正式 Release 的有效功能一致性。
- 现有性能、稳定、安全、双平台、无广告和分层不变量。
- 按 Issue、分支、验证、PR、Review/CI、Squash、主干验证交付。

授权不覆盖付费资源、密钥/签名、正式 Release、许可证例外、force push、删除 main 或 Ruleset 绕过。

## 基线验证

- CI 合同：CI_CONTRACT_GREEN passed=35。
- 仓库政策：ok=true，violation_count=0。
- Git 空白检查：通过。
- worktree HEAD 与 origin/main：均为 86a7dd837652f63198c7682b84d82180b8558e3a。
- 初始工作树：clean。

## 规划范围

七路径与 sha256:4c33c7597fcf2afbcc084a8e32560a3ea1801db4d984ccff5b7338c30ef10431。

候选实现范围为十二路径与
sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713。

本任务禁止产品代码、Cargo、Workflow、Runner、Release、upstream、Ruleset 和 AGOS 改动。

## 下一节点

1. 完成七路径规划验证并建立 planning checkpoint。
2. 在 Test-CiScripts.ps1 写自治策略 RED。
3. 创建最小 JSON 策略和离线验证器使其 GREEN。
4. 为只读状态解析器完成第二轮 RED/GREEN。
5. 本地收口、独立复审、非 Draft PR 与 Hosted CI。
6. 依据 standing authorization 自动完成精确 Final Head Squash、主干验证和 Paseo 循环激活。
