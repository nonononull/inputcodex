# Issue #176 source-lock 与 Release 身份完整性实施计划

> 状态：`LOCAL_VERIFICATION_COMPLETE_PR_PENDING`

## 目标

执行 `gate5-v1.2.45-closure-program-v2` 批次 `1/6`：严格验证既有
`upstream/source-lock.json` 的 JSON 结构、本地 Git 快照闭包，以及固定上游 Release、tag、commit、tree、
archive 和许可证身份。本任务只恢复治理门禁，不同步 `v1.2.45`，产品交付为零。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/176
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5226190361
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/176#issuecomment-5226216846
- baseline_ref: `main@5a7465252b56f7e90673e72d3e02881ac9238141`
- baseline_tree: `348a55ce78d1c9da408238b6d9b63cb2e49e32ba`
- branch_ref: `codex/issue-176-source-lock-release-integrity`
- scope_count: `11`
- scope_hash: `sha256:973cd1dd29f70ab6cf3f8bd2c82f61c1673adf09eec7f15db3c76dbd93e97001`

## 实施顺序

1. 严格解析：拒绝重复 key、未知/缺失字段、根与嵌套类型漂移、数组伪装及大小写漂移。
2. 本地闭包：从 Git tree/blob 复算路径、mode、SHA、字节数、manifest、tree 统计和许可证证据。
3. 远端闭包：仅在 GitHub-hosted `release-audit` Job 查询固定上游 API，并有界下载固定 codeload archive。
4. Rust 投影：对 source-lock 全结构启用 `deny_unknown_fields`，以隔离负例锁定未知、缺失、重复和类型漂移。
5. 交付：完成本地门禁、exact-head Hosted CI、双独立复审和 Squash Merge；随后才允许从 fresh main 启动批次 2。

## 停止门

出现第十二路径、第二 writer、产品或目录数据变化、非 `release-audit` Job 漂移，或 Final Head 独立复审出现
Critical/Important，立即停止并禁止在当前 PR 内修补 finding。
