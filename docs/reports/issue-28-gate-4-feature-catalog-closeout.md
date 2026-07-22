# Issue #28：Gate 4 功能目录执行合并证据 Closeout 报告

report_status: completed-pr-29-squash-merged
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/28
source_issue_ref: https://github.com/nonononull/inputcodex/issues/26
source_pr_ref: https://github.com/nonononull/inputcodex/pull/27
closeout_branch_ref: codex/issue-28-gate-4-feature-catalog-closeout
baseline_ref: a9b20f00ae069aedd42c8124d2789b230187258c
approved_decision_ref: user-message:approve-gate-4-closeout-issue-2026-07-22
scope_amendment_decision_ref: user-message:approve-issue-28-scope-amendment-agents-md-2026-07-22
scope_hash: sha256:a6559a9538dc16908d76c6a0360c1b7c7cb3323158764d244ca891f49d587e1a
allowed_operations: project-doc-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 仅将已发生的 Gate 4 功能目录合并事实收口到项目原生控制面。
executor_enforcement: 范围外路径、事实漂移、验证失败、未解决 Review 或缺少最终 owner 授权均阻止推进。
agos_status: bypassed-needs-input-unregistered
control_plane_checkpoint_ref: commit:608c6dd0f6b106d5f6bd77649be0c914d957ebdd;issuecomment:5049829624
source_pr_head_ref: 1d1bf32cdc4edc45e2d28f1047604222ebdb51e4
source_pr_ci_ref: run:29942593564;status:success;jobs:6;artifacts:0
source_main_ci_ref: run:29943399832;status:success;jobs:6;artifacts:0
source_review_ref: total:0;unresolved:0
source_merge_ref: commit:a9b20f00ae069aedd42c8124d2789b230187258c;parent-count:1;tree:205c24e05e0451a3aa39af4f43f0d9853cc7a6a2;github-signature:valid
source_owner_merge_authorization_ref: issuecomment:5049442605
source_branch_cleanup_ref: issuecomment:5049570338
closeout_pr_ref: https://github.com/nonononull/inputcodex/pull/29
closeout_pr_head_ref: 7ee316c6bf4d9ca44f3475283ae1aee9c83f8577
closeout_pr_ci_ref: run:29948253910;status:success;jobs:6;artifacts:0
closeout_review_ref: total:0;unresolved:0
closeout_merge_ref: commit:c07da0cad33e09b5c54e528a8a6728a048c88c0b;parent-count:1;tree:02ab8a3d8497ebb7b990e4078122b9bf916ef454;github-signature:valid
closeout_main_ci_ref: run:29948874307;status:success;jobs:6;artifacts:0
closeout_owner_merge_authorization_ref: issuecomment:5050199411
closeout_issue_state_ref: closed:2026-07-22T18:56:39Z
closeout_branch_status_ref: retained-awaiting-separate-owner-deletion-authorization

## 一、来源执行事实

- Issue `#26`：`[Architecture] Gate 4 功能目录、行为合同与脱敏夹具执行`，于 `2026-07-22T17:41:13Z` 关闭。
- PR `#27`：`[Gate 4] 建立功能目录行为合同与脱敏夹具`，最终 Head 为 `1d1bf32cdc4edc45e2d28f1047604222ebdb51e4`，于 `2026-07-22T17:41:11Z` Squash Merge 至 `main`。
- Squash 提交为 `a9b20f00ae069aedd42c8124d2789b230187258c`，只有一个父提交；来源 Head tree 与 Squash tree 均为 `205c24e05e0451a3aa39af4f43f0d9853cc7a6a2`，GitHub 签名状态为 `valid`。
- 来源 PR CI `29942593564` 的六个 Job 均成功且 Artifact 数为 `0`；合并后 main CI `29943399832` 同样六 Job 成功且 Artifact 数为 `0`。
- 来源 PR 的 Review 对话总数与未解决数均为 `0`；项目所有者的 Squash Merge 授权为 Issue 评论 `5049442605`。
- 远端 `codex/issue-26-gate-4-feature-catalog` 已删除并得到 GitHub API `404` 复核；本机已切换至 `main`、prune 远端跟踪引用并删除本地同名分支，删除证据为 Issue 评论 `5049570338`。

## 二、控制面漂移根因

来源 PR 的实现计划、Session Plan、Runtime Workflow 和报告在提交时必须真实记录“PR/CI/合并尚未发生”的状态。PR `#27` Squash Merge 后，不能直接改写已合并提交或向 `main` 直接推送，因此 `README.md`、Master Plan 和来源报告仍留有“待创建 / 待 CI / 待合并”文本。

独立 Issue `#28` 与 Closeout PR 是修复该文档事实漂移的合规路径；它不重做 Gate 4 功能目录执行，也不构成性能基线、预算或优化授权。

## 三、AGOS 外部状态

已对 `inputcodex-issue-28-gate-4-feature-catalog-closeout` 执行 AGOS `-ReportOnly`。结果为 `needs-input`、任务 `unregistered`、doctor `blocked`，且其输出禁止在该框架下的项目文档写入。按项目规则记录为绕过，不改动 AGOS 的任何文件；本任务继续遵守项目原生 Issue、计划、工作流、报告、PR 与 CI 控制面。

## 四、历史 checkpoint 与范围记录

控制面 checkpoint `608c6dd0f6b106d5f6bd77649be0c914d957ebdd` 已普通 push，并通过 Issue `#28` 评论 `5049829624` 回写。该 checkpoint 只新增以下四份控制面文件：

```text
docs/plans/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md
docs/plans/sessions/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md
docs/reports/issue-28-gate-4-feature-catalog-closeout.md
docs/workflows/2026-07-22-issue-28-gate-4-feature-catalog-closeout-runtime.md
```

本批次已按 Session Plan 完成 `README.md`、Master Plan 与 Issue `#26` 来源报告的来源事实回写；不得再扩展到产品、性能、上游、CI 或 AGOS 表面。

项目所有者随后批准将 `AGENTS.md` 纳入本 Issue 的状态回写范围，批准引用为 `user-message:approve-issue-28-scope-amendment-agents-md-2026-07-22`。当时的八路径允许集合为：

```text
AGENTS.md
README.md
docs/plans/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md
docs/reports/issue-26-gate-4-feature-catalog.md
docs/reports/issue-28-gate-4-feature-catalog-closeout.md
docs/workflows/2026-07-22-issue-28-gate-4-feature-catalog-closeout-runtime.md
```

该集合按路径升序、LF 分隔并保留末尾 LF 的 `scope_hash` 为 `sha256:a6559a9538dc16908d76c6a0360c1b7c7cb3323158764d244ca891f49d587e1a`。当时的 `AGENTS.md` 只更正 Gate 状态为 Issue `#28` / PR `#29` Closeout，并将性能基线锁定到 PR `#29` 合并之后。

PR `#29` 的旧 Head `479e083cd666356f05751a2610d4f27164c61cfd` 与其成功 CI `29946947621` 仅保留为范围扩展前的历史证据。该扩展必须形成新的普通提交与新 Head，并等待新的 GitHub-hosted CI；旧 Head CI 不得用于合并判断。

## 五、Closeout 完成证据

1. 项目所有者已在 Issue 评论 `5050199411` 明确授权针对最终 Head `7ee316c6bf4d9ca44f3475283ae1aee9c83f8577` 执行 Squash Merge。
2. PR `#29` 于 `2026-07-22T18:56:38Z` Squash Merge；提交 `c07da0cad33e09b5c54e528a8a6728a048c88c0b` 只有一个父提交、tree 为 `02ab8a3d8497ebb7b990e4078122b9bf916ef454`、GitHub 签名有效，`main` 精确指向该提交。
3. PR `#29` 新 Head CI `29948253910` 成功，Artifact 数为 `0`；Review 对话总数和未解决数均为 `0`。
4. Issue `#28` 于 `2026-07-22T18:56:39Z` 自动关闭；合并后 main CI `29948874307` 的 `classify`、`governance`、`linux-quality`、`macos`、`windows` 与 `required` 六个 Job 全部成功，Artifact 数为 `0`。
5. 源分支 `codex/issue-28-gate-4-feature-catalog-closeout` 保留且指向最终 Head；项目所有者未授权删除，因此没有执行任何分支清理。

## 六、持久状态与后续边界

Gate 4 功能目录执行与独立 Closeout 已完成。若需改变源分支状态或将新的动态事实写入其他持久控制面，必须创建独立 Issue/PR；不得改写已合并的 PR `#29`。下一项可启动工作是独立性能基线 Issue，其基线、预算与优化仍须相互分离。
