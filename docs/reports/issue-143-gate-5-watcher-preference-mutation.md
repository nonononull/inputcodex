# Issue #143 Watcher 偏好固定文件 mutation 报告

## 当前状态

- state: PLANNING_FREEZE_LOCAL_READY
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/143
- owner_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214
- baseline_ref: main@891866a914468d3979964a2fe89066a0bbb2fe53
- branch_ref: codex/issue-143-gate-5-watcher-preference-mutation
- planning_scope: 8
- planning_scope_hash: sha256:7011b4faa5d331e668b2eb837dbbc42b716541f9f88431c9a773ae830c0dbe11
- candidate_scope: 24
- candidate_scope_hash: sha256:f96f1a979eba89bc4de9744b3267bfd72fd81a828c9491cbfd3b95723b088ab9
- product_count_delta: 0（实施尚未开始）

## Discovery 结论

- 上游 `enable_watcher` 只删除固定 marker，`disable_watcher` 只创建父目录并写固定 marker；两个 command 不控制进程。
- 既有 `SystemPlatformPaths` 只提供绝对 `PathBuf`，不是绑定目录身份的句柄；本轮只因 #140 明确采用
  `cooperative-same-user-v1` 才允许 safe std 路径提交。
- 既有 Watcher observation 已固定 `EnabledByDefault / ExplicitlyDisabled` 与脱敏边界，可作为请求和最终观察的领域语言。
- 既有 `LoadCoordinator` 在取消后把完成结果降为 `Stale`，不能用于已发生副作用的 mutation。
- source-index 把完整 Watcher 的 `process-control` 错误传播给两个 command；本轮只纠正这两条。

## 规划门

- fresh worktree、branch、HEAD 与 merge-base 已核验。
- Git snapshot governance：ready。
- AGOS default entry：`needs-input/session-plan-bootstrap`，按项目规则绕过，无跨仓写入。
- `.codegraph`：不存在，未初始化。
- `superpowers:*`：当前会话未暴露，使用 `karpathy-guidelines`、`domain-modeling` 与项目原生 TDD。
- 两个 Paseo 只读规划 reviewer 在模型可用性探测阶段超时，未创建 agent、未产生写入；Final Head 复审预算保留。

## 待完成

1. 验证八路径 Planning Freeze，形成 planning checkpoint 并发布 #143 评论。
2. 依次完成 Domain、Application、Platform、Parity RED/GREEN。
3. 更新稳定文档与 err，执行二十四路径本地门禁。
4. 完成 PR、独立复审、Hosted CI、精确 Squash、main 复验并重开 #140。
