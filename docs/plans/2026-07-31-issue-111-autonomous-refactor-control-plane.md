# Issue #111 无人值守重构控制面实施计划

## 目标

在不削弱 inputcodex 既有 Issue、范围哈希、Review、Hosted CI、Artifact、Squash 与主干验证证据的
前提下，把 Gate 5 常规重构从逐项人工意见改成 bounded standing authorization 驱动的无人值守流程。

本任务只建立治理和恢复控制面，不迁移第十二个产品功能。

## 已选择方案

采用 GitHub + Paseo 混合自治。

- GitHub Issue、PR、Actions 和分支保存长期事实。
- Paseo 保持一个写入者；研究、审计和验证 agent 只能只读。
- 每个任务从 origin/main 建立 Paseo 管理的隔离 worktree，不更新项目所有者主工作目录。
- GitHub 原生 auto-merge 保持关闭；自治执行器在精确 Final Head 上完成结构化断言后执行 Squash Merge。
- 中断恢复只依赖 GitHub、Git、策略文件和 worktree 状态，不依赖聊天上下文。

未选择的方案：

1. 单一长会话连续执行：实现最少，但网络或会话中断后容易丢失状态。
2. GitHub Actions 内运行 AI：需要额外密钥、写权限和成本控制，扩大攻击面，也违背当前 hosted CI 只负责编译验证的边界。

## 决策优先级

1. 性能优先。
2. 与上游最新正式 Release 的有效行为一致。
3. 优先最小、只读、无副作用、可独立验收切片。
4. 写入、网络、执行、安装和更新能力必须先有 typed owner、超时、取消、错误隔离、最小披露与可验证回滚。
5. 语义不安全或证据不足时 fail-closed，建立 parity-exception 证据并隔离该功能，再继续下一安全候选。
6. UI 仍交给 Gemini；不得因 Paseo 未注册 Gemini provider 而由 Codex 静默替代。

## 文件职责

- .github/autonomous-refactor-policy.json：机器可读的 standing authorization、单写者、选项排序、合并门、重试和硬停止合同。
- scripts/ci/Verify-AutonomousRefactorPolicy.ps1：离线严格验证策略 schema 与关键不变量。
- scripts/automation/Get-AutonomousRefactorState.ps1：只读采集 Git/GitHub 状态并输出下一合法动作；ReportOnly 永不修改 GitHub 或文件。
- scripts/ci/Test-CiScripts.ps1：策略验证器与状态解析器的 RED/GREEN 合同。
- AGENTS.md：人类可读的自治授权、合并门和当前 Gate 边界。
- build.md：可重复本地门禁和范围验证。
- task-local Plan、Session Plan、Runtime Workflow、Report：Issue #111 的执行与恢复证据。

## Task 1：规划冻结

- [x] 建立 Issue #111 并记录项目所有者 standing authorization。
- [x] 从 origin/main@86a7dd837652f63198c7682b84d82180b8558e3a 创建 Paseo 隔离 worktree。
- [x] 记录七路径规划范围与十二路径候选实现范围。
- [x] 运行规划范围、CI 合同、仓库政策与 Git 空白验证。
- [x] 建立普通 planning checkpoint 并普通 push。

规划范围：

    AGENTS.md
    build.md
    docs/plans/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    docs/plans/PROJECT-MASTER-PLAN.md
    docs/plans/sessions/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    docs/reports/issue-111-autonomous-refactor-control-plane.md
    docs/workflows/2026-07-31-issue-111-autonomous-refactor-control-plane-runtime.md

- count: 7
- hash: sha256:4c33c7597fcf2afbcc084a8e32560a3ea1801db4d984ccff5b7338c30ef10431

## Task 2：自治策略 RED 到 GREEN

- [x] 先在 Test-CiScripts.ps1 增加缺失策略文件、错误授权模式、启用 GitHub auto-merge、非 Squash、写入者大于一、门禁缺失、UI owner 非 Gemini 和硬停止缺失的失败测试。
- [x] 运行专项测试并确认因策略与验证器缺失而 RED。
- [x] 创建最小 JSON 策略和 Verify-AutonomousRefactorPolicy.ps1。
- [x] 重跑专项与完整 CI 合同，确认 GREEN。
- [x] 建立 policy-green checkpoint。

## Task 3：状态解析 RED 到 GREEN

- [x] 为无活动任务、单一 active Issue、单一 active PR、重复 active writer、release audit stale、main freshness 漂移和不可恢复输入编写失败/成功夹具。
- [x] 先确认状态解析器缺失的 RED。
- [x] 实现只读 Get-AutonomousRefactorState.ps1；默认只读，ReportOnly 明确禁止 mutation。
- [x] 输出稳定 JSON：state、next_action、reason_codes、active_issue、active_pr、expected_base、observed_head。
- [x] 重跑专项与完整 CI 合同，确认 GREEN。
- [x] 建立 state-green checkpoint。

## Task 4：本地收口

- [x] 完成 AGENTS、Master Plan、build 和报告的稳定内容。
- [x] err.md 只在形成新可复用根因时修改。
- [x] 运行 build.md 的 Issue #111 完整轻量门禁。
- [x] 执行独立只读 review 与安全审查，逐项处置发现。
- [x] 核对实际路径不越过候选范围。
- [ ] 建立 local-verified checkpoint 并普通 push。

本地收口证据：首轮独立只读 Reviewer 返回 `0 Critical / 5 Important / FAIL`。五项均由主线程
独立复现或论证后处置：严格 JSON 类型与有序决策、全量分页与外部 schema、分支来源与脏树优先、
非 owner marker 安全忽略语义，以及可执行的 exact-head / post-merge 状态门。最终十二路径轻量门禁
返回 `CI_CONTRACT_GREEN passed=65` 与 `ISSUE_111_LOCAL_GREEN scope=12`。第二轮独立只读复审绑定
冻结聚合 `sha256:2a33a1d7b43fc5b00afdbc5a5e367d0f0d1f5ec0f40cccccc384e679cac481db`，结论为
`PASS - 0 Critical / 0 Important`；主线程在复审后复算同一哈希并确认无 agent 写入。

## Task 5：远端交付与自治合并

- [ ] 创建关联 Issue #111 的非 Draft PR。
- [ ] 核验 Final Head、scope hash、独立 review、Review threads、CI 七 Job、Performance 四 Job、Artifact 0、release audit 和 origin/main freshness。
- [ ] 在 PR 回写 standing authorization ref、精确 Final Head 与策略 hash。
- [ ] 使用精确 Head 约束执行 Squash Merge；禁止 GitHub原生 auto-merge。
- [ ] 验证 Squash 单父、tree 等价、GitHub 签名、Issue 状态与合并后主干双套 Actions。
- [ ] 将仓库 merge/rebase 开关加固为关闭，保留 squash；动态设置证据只写 GitHub。
- [ ] 归档 Paseo worktree。

## Task 6：启动自治循环

- [ ] 从项目所有者本地主工作目录注册单一 Paseo 自治循环或恢复 schedule。
- [ ] 每次迭代先执行状态解析，已有活动 Issue/PR 时只恢复，不创建重复任务。
- [ ] 设置有限重试和最大运行时间；外部 CI 等待使用有上限轮询。
- [ ] 首个候选默认 feature.session-data.token-usage-history。
- [ ] 若 Discovery 证明该候选不是当前最小安全切片，按策略确定性选择下一项。
- [ ] 用一次 ReportOnly 和一次真实 claim/resume 验证幂等恢复，再关闭 Issue #111。

## 候选实现范围

    .github/autonomous-refactor-policy.json
    AGENTS.md
    build.md
    docs/plans/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    docs/plans/PROJECT-MASTER-PLAN.md
    docs/plans/sessions/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    docs/reports/issue-111-autonomous-refactor-control-plane.md
    docs/workflows/2026-07-31-issue-111-autonomous-refactor-control-plane-runtime.md
    err.md
    scripts/automation/Get-AutonomousRefactorState.ps1
    scripts/ci/Test-CiScripts.ps1
    scripts/ci/Verify-AutonomousRefactorPolicy.ps1

- count: 12
- candidate_scope_hash: sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713

## 完成标准

- 策略与状态解析均可离线测试且 fail-closed。
- 同一状态重复执行不会创建重复 Issue、PR、评论或 worktree。
- 普通重构不再等待逐项人工意见，但每次合并仍绑定精确 Final Head 和 owner authorization ref。
- GitHub 原生 auto-merge 保持关闭。
- 付费资源、密钥/签名、正式 Release、许可证例外、force push、删除 main 和 Ruleset 绕过仍为硬停止。
- 合并后主干 CI 与 Performance 全绿且 Artifact 为 0。
