# Issue #111 Runtime Workflow：无人值守重构控制面

## Runtime Metadata

- task_id: issue-111-autonomous-refactor-control-plane
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_ref: origin/main@86a7dd837652f63198c7682b84d82180b8558e3a
- branch_ref: codex/issue-111-autonomous-refactor-control-plane
- planning_scope: 7
- planning_scope_hash: sha256:4c33c7597fcf2afbcc084a8e32560a3ea1801db4d984ccff5b7338c30ef10431
- candidate_scope: 12
- candidate_scope_hash: sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- github_native_auto_merge: forbidden
- merge_method: exact-head-squash

## Current Gate

    PR #110 post-merge verified
      -> Issue #111 owner standing authorization
      -> Paseo isolated worktree
      -> planning freeze
      -> policy RED/GREEN
      -> state resolver RED/GREEN
      -> local closeout and read-only review
      -> non-Draft PR
      -> Hosted CI and Artifact verification
      -> exact Final Head autonomous Squash
      -> post-merge main verification
      -> activate durable Paseo loop
      -> select next safe Gate 5 candidate

当前 ALLOWED_OPS：十二路径内治理文档、离线策略与验证器、只读状态解析、测试、普通 Git
checkpoint/push、PR、独立只读 Review、Hosted CI 核验、精确 Head Squash、合并后验证和 Paseo 控制面激活。

当前 FORBIDDEN_OPS：产品功能、Cargo、Workflow、Runner、Release、upstream、Ruleset、AGOS、
GitHub 原生 auto-merge、付费/self-hosted Runner、密钥/签名、force push、删除 main 和未通过门禁的合并。

## Node 0：Startup Baseline

1. 使用 Get-Date 记录本机时间。
2. 确认 origin/main 与 worktree HEAD 均为 86a7dd837652f63198c7682b84d82180b8558e3a。
3. 确认分支和 worktree 由 Paseo 管理，工作树 clean。
4. 运行 Test-CiScripts、Verify-RepositoryPolicy 和 git diff --check。

状态：completed。CI 合同 35/35、仓库政策零违规、Git 空白检查通过。

## Node 1：Decision Evidence

1. Issue #111 记录项目所有者“不需要逐项意见”的 standing authorization。
2. 记录授权边界、自动决策顺序、精确合并门和硬停止。
3. 核对 Ruleset、仓库 merge 设置、开放 Issue/PR 和 22 个 unassessed 功能。

状态：completed。

## Node 2：Planning Freeze

1. 只修改七条规划路径。
2. 固定 planning 和 candidate scope hash。
3. 记录机器策略、状态解析、TDD、Review、远端交付和 Paseo 激活节点。
4. 运行规划范围与文档轻量验证。
5. 建立 planning checkpoint 并普通 push。

状态：completed。规划范围 7 / sha256:4c33c759...，CI 合同 35/35、仓库政策、Release Audit 与
Git 空白检查通过；planning checkpoint 为 edcd066bfa9a4fe24c8bd63a5d079854b0402922。

## Node 3：Policy TDD

1. RED：在 Test-CiScripts.ps1 中加入有效策略与八类非法策略合同。
2. VERIFY RED：策略或验证器缺失，专项测试必须以预期错误失败。
3. GREEN：创建 autonomous-refactor-policy.json 和 Verify-AutonomousRefactorPolicy.ps1。
4. VERIFY GREEN：专项与完整 CI 合同通过。
5. CHECKPOINT：issue-111-policy-green。

## Node 4：State Resolver TDD

1. RED：覆盖 idle、active Issue、active PR、重复 writer、stale audit、base 漂移和未知输入。
2. VERIFY RED：状态解析器缺失或行为未实现。
3. GREEN：实现只读 Get-AutonomousRefactorState.ps1。
4. VERIFY GREEN：输出稳定 JSON，ReportOnly 无 Git/GitHub mutation。
5. CHECKPOINT：issue-111-state-green。

Node 3 状态：completed。缺失策略与验证器的 RED 为稳定退出码 10；九条新增策略合同和完整
CI 合同 44/44 通过，policy checkpoint 为 aefa02f0f9b3c884178214baa808ccfd9f95c5c0，
规范化策略 hash 为 sha256:2cb8467153892fcb1510c86cdcb186cd9dabc3d4f08055ec9c503b823d760275。

Node 4 状态：completed。缺失状态解析器的 RED 为稳定退出码 10；十条状态分类合同和完整
CI 合同 54/54 通过，state checkpoint 为 eaca92e9b70b804f5ed55b97296c0a2d28d3212e。
live 模式先复现 Int32/Int64 类型差异，再在采集边界归一化；脏/clean 工作树分别稳定返回
resume-worktree 与 resume-issue。

## Node 5：Local Closeout

1. 更新稳定控制面与报告；err.md 只记录新可复用根因。
2. 运行 build.md Issue #111 完整门禁。
3. 执行独立只读 reviewer 和安全审查。
4. 父线程逐项复核发现，接受项必须有 RED/GREEN；不成立项必须有证据。
5. 建立 issue-111-local-verified checkpoint 并普通 push。

状态：corrective-in-progress。local-verified checkpoint 已普通 push 并创建 PR #112；仓库已加固为
auto-merge=false、squash=true、merge/rebase=false。首轮真实 PR live 恢复发现 `$head` 被 PR 投影覆盖，
已新增 RED 合同并以独立 `$worktreeHead` 修复；完整 CI 合同为 66/66，真实 live 退出 0。当前只待
校正提交、Final Head 独立复审、PR evidence 刷新和新 Head Hosted CI。

## Node 6：Remote Delivery

1. 创建关联 Issue #111 的非 Draft PR。
2. 在 Final Head 运行并核验 CI 七 Job与 Performance Baseline 四 Job。
3. 核验两个成功 Run Artifact 为 0、Review threads 为 0、release audit current。
4. 核验 origin/main 仍为任务基线；漂移时停止并重新规划。
5. 回写 standing authorization ref、策略 hash、scope hash 和精确 Final Head。

## Node 7：Autonomous Exact-Head Merge

1. 重新读取 PR Head；必须等于已验证 Final Head。
2. 使用精确 Head 约束执行 Squash Merge。
3. 验证 Squash 单父、tree 等价、GitHub 签名 valid 和 origin/main。
4. 等待并核验合并后 main CI 七 Job、Performance 四 Job和 Artifact 0。
5. 关闭 Issue #111 并按 Paseo 所有权归档 worktree。

## Node 8：Durable Loop Activation

1. 从本地主工作目录启动单一 Paseo loop 或恢复 schedule。
2. 每轮先执行策略与状态解析；有 active Issue/PR 时只能恢复。
3. writer 最多一个；只读 reviewer/verifier 可以并行。
4. 外部等待有最大时间，失败最多重试三次；GitHub 服务事故不得诱发代码改动。
5. 首个默认候选为 feature.session-data.token-usage-history。
6. 完成一次 ReportOnly 与一次 claim/resume 幂等证明。

## Resume Algorithm

每次中断恢复按以下顺序：

1. 列出 inputcodex Paseo workspace 与 agent，区分当前主会话、active writer 和 stale worker。
2. fetch origin/main，不 checkout 或 pull 项目所有者主工作目录。
3. 查询带自治标记的开放 PR；若恰好一个，恢复 PR 节点。
4. 否则查询带自治标记的 active Issue；若恰好一个，恢复其 worktree 或从远端分支重建。
5. 否则验证 release audit 与上游 Release，再选择下一候选。
6. 多个 active 对象、多个 writer、未知分支来源或 scope 漂移均返回 blocked，不猜测。

## Error Watchlist

- err.md 先查重；已知 gh 直连失败先使用本机 127.0.0.1:7897 代理重试一次。
- PowerShell 空输出不得直接调用 Trim；先包装为数组并显式 join。
- JavaScript 编排中的 PowerShell 反引号不得直接放入模板字面量。
- GitHub 原生 auto-merge 在无 required status checks 时可能提前合并，永久保持关闭。
- 项目所有者主工作目录可能落后 origin/main，不得自动 pull、checkout 或 reset。
- GitHub 评论、Issue、PR 创建必须有稳定 marker 并先做重复查询。
- 状态解析器 ReportOnly 不得调用 gh issue create/edit、gh pr create/merge、git commit/push 或 Paseo mutation。

## Checkpoint Rules

- 每个 execute batch、重验证、handoff/pause 前运行 Git snapshot governance ReportOnly。
- 每个 RED/GREEN 层完成后创建独立普通提交。
- 禁止 amend、rebase、force push 和 main 直接写入。
- 所有 Git 时间使用本机默认时间，不设置 GIT_AUTHOR_DATE 或 GIT_COMMITTER_DATE。

## AGOS Boundary

AGOS 仅作可选 ReportOnly 辅助。Issue #111 不修改 AGOS Registry、规则、脚本、Workflow 或 Vault；
AGOS 不可用、未登记或 needs-input 时记录并绕过。

## Rollout Draft

- workflow_family: gate-5-autonomous-refactor-control-plane
- reusable_path: policy -> state resolution -> bounded worker -> exact-head merge -> post-merge verify
- skill_usage: brainstorming、writing-plans、using-git-worktrees、test-driven-development、paseo-loop、verification-before-completion
- record_after_closeout: 仅在真实 claim/resume 与精确 Head 自动合并均验证后记录
