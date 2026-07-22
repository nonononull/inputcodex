# Issue #22：Gate 3 合并 closeout 报告

report_status: pr-open-review-ci-in-progress
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/22
source_issue_ref: https://github.com/nonononull/inputcodex/issues/19
source_pr_ref: https://github.com/nonononull/inputcodex/pull/21
closeout_branch_ref: codex/issue-22-gate-3-closeout
closeout_pr_ref: https://github.com/nonononull/inputcodex/pull/23
review_ref: github-pr-23-review-threads-pending
ci_ref: github-pr-23-checks-pending
merge_ref: pending-closeout-pr
approved_decision_ref: user-message:create-gate3-closeout-through-squash-merge-2026-07-22
owner_merge_authorization_ref: user-message:create-gate3-closeout-through-squash-merge-2026-07-22

## 一、PR #21 合并结论

- PR `#21` 于 `2026-07-22T12:25:59Z` 由 `nonononull` Squash Merge。
- 最终获批 Head：`9a4a4425f2fb0d8235554d3e83577111ae34efcc`。
- 合并提交：`0716ec0debcd3e059cc4ca88a072232841ca73b4`。
- 唯一父提交：`477d110a9b284e127af365f5278901bcfa69e093`。
- Merge tree 与最终 Head tree 均为 `4881ce609370f77181d9545474c029ab0c5d4972`。
- GitHub verification 为 `verified=true`、reason=`valid`。
- 远端 `codex/issue-19-gate-3-rust-workspace-ci` 已删除，本地 `main` 已快进到 merge commit。

## 二、Issue、CI 与 Review 结论

- Issue `#19` 于 `2026-07-22T12:26:00Z` 以 `CLOSED / COMPLETED` 关闭。
- 最终 PR Head 运行 `29918843397` 六 Job 全绿，成功 Artifact 数为 `0`。
- 合并后 `main` 运行 `29919596057` 六 Job 全绿，成功 Artifact 数为 `0`。
- PR `#21` Review 对话为 `0`；合并前自动合并关闭，Merge State 为 `CLEAN`。
- Ruleset `19395456` 保持 active、无 bypass、required approvals `0`、必须解决 Review 对话、Squash-only。
- 具备合并权限的人类维护者只有 `nonononull`；项目所有者授权证据已写入 PR 评论。

## 三、Gate 3 实现结论

- 七成员纯 Rust Workspace、Rust `1.97.1`、Iced 展示层隔离、加载/取消/错误/平台最小语义已进入 `main`。
- 治理合同为 `30/30`，仓库政策 `violation_count=0`。
- 治理、rustfmt、通用 Rust、Windows 条件编译、macOS 条件编译五类失败语义均完成普通提交 RED→GREEN。
- Linux、Windows、macOS 各取得 `3/3` 次无缓存成功样本；Job 执行时间中位数分别为 `133`、`212`、`96` 秒。
- 当前仍未迁移上游业务功能；Gate 4 功能目录与性能预算继续锁定。

## 四、控制面漂移根因

- PR `#21` 合并后，来源分支中的 Session/Runtime/报告仍保留等待授权和 `merge_ref: pending`，因为已合并 PR 不能再写回同一 commit。
- 根 `AGENTS.md` 仍写“尚未导入应用源码”，与已进入 `main` 的最小 Workspace 骨架冲突。
- 正确处理是独立 closeout Issue/PR；禁止直接写 `main`，也不能仅依赖 GitHub 评论。

## 五、排错收口

- `git fetch --prune origin` 经 HTTPS 出现 `Recv failure: Connection was reset` 和端口 443 暂时不可达；不修改远端配置，使用一次性 SSH refspec 成功同步并 prune。
- PowerShell 把 `gh pr view --jq .body` 的多行输出保留为数组，直接传给 `gh pr edit --body` 会把正文行误解析为 CLI 参数；将行数组按换行连接并通过 `--body-file -` 输入后成功。
- 两个错误均未改变 Git 历史、远端配置、产品代码或 GitHub 合并结果。

## 六、当前交付边界

- Issue `#22` 只允许 14 条治理/文档路径。
- closeout PR 自身的 PR/Review/CI/merge 动态引用在创建和验证后回写。
- 当前消息已授权最终 Squash Merge，但只有在范围、Head、CI、Review 和 Ruleset Fresh 一致时才允许执行。
- closeout 合并后 Gate 3 完成，Gate 4 仍需新的独立规划 Issue 与项目所有者批准。
