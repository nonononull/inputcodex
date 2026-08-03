# Issue #143 Watcher 偏好固定文件 mutation 报告

## 当前状态

- state: LOCAL_VERIFY_GREEN / PR_PENDING
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/143
- owner_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214
- baseline_ref: main@891866a914468d3979964a2fe89066a0bbb2fe53
- branch_ref: codex/issue-143-gate-5-watcher-preference-mutation
- planning_scope: 8
- planning_scope_hash: sha256:7011b4faa5d331e668b2eb837dbbc42b716541f9f88431c9a773ae830c0dbe11
- candidate_scope: 24
- candidate_scope_hash: sha256:f96f1a979eba89bc4de9744b3267bfd72fd81a828c9491cbfd3b95723b088ab9
- product_count_delta: source implemented `+2` / unassessed `-2`；feature implemented `+1` / total `+1`

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

## 实施结果

- Domain：六类 outcome、三类 setup commit、四类 marker commit、最终观察和脱敏 receipt 已建立；专项 `4/4`。
- Application：请求强制 `expected + desired`，独立 mutation phase 保证提交前取消与提交后 `TooLate`，receipt 不走 `Stale`；专项 `6/6`。
- Platform：固定父目录/状态根/marker 分类、Windows reparse 拒绝、单层 root setup、`create_new`、条件删除、
  单航班、竞态与失败后重观察已建立；内部矩阵 `13/13`，全包 `85` 个单元测试及全部集成测试通过。
- Parity：只移动 `enable_watcher` / `disable_watcher`，副作用精确为 `[filesystem-read, filesystem-write]`；
  feature `13/22/11`、source `19/83/30/3`、contract `46`、fixture manifest `12`，完整 Watcher 继续 `unassessed`。
- 禁止面：无任意路径/内容、递归目录、依赖、进程、网络、UI、SQLite、FFI/VFS 或 `unsafe`。

## 待完成

1. 形成 clean Final Head，普通 push 并创建 non-Draft PR。
2. 取得两个独立 `0 Critical / 0 Important` Final Head 复审。
3. 核验同 Head CI `7/7`、Performance `4/4`、Artifact `0` 后精确 Squash。
4. 合并后复验 main，重开 #140 并回到 `await-owner-decision`。
