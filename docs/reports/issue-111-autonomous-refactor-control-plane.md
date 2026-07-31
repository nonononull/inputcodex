# Issue #111 无人值守重构控制面报告

## 当前状态

- state: LOCAL_CLOSEOUT_IN_PROGRESS
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
- planning_checkpoint_ref: edcd066bfa9a4fe24c8bd63a5d079854b0402922
- policy_checkpoint_ref: aefa02f0f9b3c884178214baa808ccfd9f95c5c0
- state_checkpoint_ref: eaca92e9b70b804f5ed55b97296c0a2d28d3212e
- policy_sha256: sha256:2cb8467153892fcb1510c86cdcb186cd9dabc3d4f08055ec9c503b823d760275

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

## TDD 与恢复证据

- 策略 RED：CI_CONTRACT_RED_MISSING_IMPLEMENTATION，缺少策略 JSON 与验证器。
- 策略 GREEN：新增九条合同，完整 CI 合同 44/44。
- 状态 RED：CI_CONTRACT_RED_MISSING_IMPLEMENTATION，缺少状态解析器。
- 状态 GREEN：新增十条合同，完整 CI 合同 54/54。
- live 采集首次稳定复现 active_writer_count 的 Int32/Int64 类型差异；修复只在采集边界归一化。
- 脏工作树：active-worktree-execution / resume-worktree。
- clean 工作树：active-issue-planning / resume-issue。
- live 输出保留 Issue #111、origin/main、worktree Head、分支与规范化策略 hash，reason_codes 为空。
- 安全强化 RED/GREEN：严格 JSON 类型、有序决策、全量分页/schema、脏树优先、分支来源、owner
  marker、exact-head merge-ready、Planning/Review 双来源绑定和 post-merge 主干验证均形成失败后通过证据。
- 当前完整 CI 合同为 65/65；live 从 owner Planning Freeze 评论解析出批准范围
  12 / sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713。

## 首轮 Review 处置

- 结论：0 Critical / 5 Important / FAIL；Reviewer 只读，未修改文件。
- 合并门静态不可执行：已增加 active-pr-review-ci、merge-ready-exact-head 和
  post-merge-verification 的机器判定；CI/Performance Job、Artifact、Review thread、scope、授权、
  单父、tree、签名与 origin/remote main 均绑定精确 Head。
- CLI 截断与 schema：GitHub REST/GraphQL 使用 --paginate + --slurp，Paseo/GitHub 空输出、null、
  非数组或错误项结构均 fail-closed 为有限外部重试。
- 分支与脏树：main 脏树、Issue 分支不匹配和孤儿任务分支硬停止；PR 存在时未提交修复优先
  resume-worktree，不跳到 Review/CI。
- owner marker：live 收集全部 marker 后只信任 owner Issue 与 owner 仓库 PR；非 owner marker
  安全忽略并计数，外部快照不能借 marker 接管状态。
- 决策顺序：策略改为严格有序序列校验，并同步拒绝字符串伪装布尔/整数。

## 第二轮冻结复审

- 最终十二路径轻量门禁返回 `CI_CONTRACT_GREEN passed=65` 与 `ISSUE_111_LOCAL_GREEN scope=12`；
  策略、仓库政策、Release Audit、三脚本 AST、只读执行面、secret scan 和 Git 空白检查均通过。
- 冻结聚合为 `sha256:2a33a1d7b43fc5b00afdbc5a5e367d0f0d1f5ec0f40cccccc384e679cac481db`。
- 两次 Codex reviewer 因外部 `503` / `429` 未形成结论；Claude plan-mode 备用 Reviewer 在同一冻结
  聚合上返回 `PASS - 0 Critical / 0 Important`，未修改文件。主线程随后复算同一哈希并确认一致。
- 非阻塞缺口保留为 PR 端到端验证项：live GitHub/Paseo 采集没有离线 API 夹具，Paseo schema、限流、
  部分输出和并发漂移主要依赖 fail-closed catch；仓库 merge/rebase 开关仍须在精确合并门前关闭。

## 规划范围

七路径与 sha256:4c33c7597fcf2afbcc084a8e32560a3ea1801db4d984ccff5b7338c30ef10431。

候选实现范围为十二路径与
sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713。

本任务禁止产品代码、Cargo、Workflow、Runner、Release、upstream、Ruleset 和 AGOS 改动。

## 下一节点

1. 建立 local-verified checkpoint 并普通 push。
2. 创建非 Draft PR并完成 Hosted CI、Review thread 与 Artifact 证据。
3. 依据 standing authorization 自动完成精确 Final Head Squash、主干验证和 Paseo 循环激活。
