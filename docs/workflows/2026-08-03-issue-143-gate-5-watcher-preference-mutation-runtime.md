# Issue #143 Runtime Workflow：Watcher 偏好固定文件 mutation

## Runtime Metadata

- task_id: issue-143-gate-5-watcher-preference-mutation
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/143
- session_plan_ref: docs/plans/sessions/2026-08-03-issue-143-gate-5-watcher-preference-mutation.md
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214
- baseline_ref: main@891866a914468d3979964a2fe89066a0bbb2fe53
- branch_ref: codex/issue-143-gate-5-watcher-preference-mutation
- candidate_scope: 24
- candidate_scope_hash: sha256:f96f1a979eba89bc4de9744b3267bfd72fd81a828c9491cbfd3b95723b088ab9
- runtime_state: local-verify-green-pr-pending

## Node Order

```text
startup baseline
  -> eight-path Planning Freeze
  -> Domain receipt RED/GREEN
  -> Application cancellation/commit state RED/GREEN
  -> Platform fixed-file mutation RED/GREEN
  -> Parity two-source RED/GREEN
  -> local closeout and exact scope verification
  -> non-Draft PR and independent Final Head review
  -> hosted CI / Performance / Artifact gates
  -> exact Head Squash and main verification
  -> reopen #140 / await-owner-decision / STOP
```

ALLOWED_OPS：二十四路径内文档、Rust TDD、Parity TDD、定向本地验证、named Git checkpoint、普通
commit/push、non-Draft PR、只读复审和精确门禁合并。

FORBIDDEN_OPS：任意路径/内容、递归建目录、完整 Watcher、进程、网络、secret、环境/剪贴板、UI、
SQLite、Cargo 新依赖、Workflow/Runner/Ruleset、Release/upstream、AGOS、`unsafe`、FFI/VFS、auto-merge、
force push，以及任何 #132/#133 写操作。

## Runtime Nodes

### Node 0：Startup Baseline

状态：completed。fresh worktree、branch、HEAD、main 与 merge-base 均为 `891866a9...`；工作树 clean，
Release Audit 基线为 `current`。Git snapshot governance 为 ready。

### Node 1：Planning Freeze

状态：completed。八路径与二十四路径 hash 已复算；Planning checkpoint 已形成并在 #143 发布 Freeze。

### Node 2：Domain TDD

状态：completed。外部集成测试先取得类型缺失 RED；随后完成无路径字段的 receipt 领域模型，专项 `4/4`、
全目标与 Clippy 通过。

### Node 3：Application TDD

状态：completed。已固定 expected/desired、提交前 Cancelled、提交后 TooLate、完成后 Finished 与 receipt 不丢失；
独立 mutation control、Port 和 UseCase 专项 `6/6`、全目标与 Clippy 通过，未复用 `LoadCoordinator`。

### Node 4：Platform TDD

状态：completed。内存文件系统矩阵覆盖父目录、根、marker、create_dir、create_new、remove、竞态、取消和后置观察；
内部矩阵 `13/13`、Platform 全包 `85` 个单元测试与全部集成测试、Clippy 通过，只使用 safe std 与平台 cfg。

### Node 5：Parity TDD

状态：completed。先取得 feature 缺失与旧归属 RED；随后新增一个 implemented feature/contract，只移动两个
command 并移除错误 `process-control`。完整目录测试 `32/32`，`core-module:watcher` 与 install/uninstall 留在原 umbrella。

### Node 6：Local And Remote Closeout

状态：in-progress。本地 `build.md` Issue #143 已返回 `ISSUE143_LOCAL_VERIFY_OK`；下一步形成 Final Head、普通 push
与 non-Draft PR。取得两个独立 `0/0` 复审和同 Head hosted 门后才允许 owner exact-Head Squash；合并后重开 #140 并停止。

## Error Watchlist

- `Path::exists` 会吞 I/O 错误，禁止使用。
- `create_dir_all` 会扩大父链副作用，禁止使用。
- `fs::write` 会跟随并覆盖叶对象，禁止使用；Disable 只允许 `create_new`。
- `symlink_metadata -> remove_file` 不是强对象绑定；只在 `cooperative-same-user-v1` 下成立，不得改写为 hostile ABA 安全。
- 状态根创建成功后 marker 失败时保留空根，receipt 必须是 `RootCreated + MarkerFailed`，禁止回滚。
- 提交后取消不得走读取型 `Stale` 路径。
- Windows reparse 检查使用 safe `MetadataExt::file_attributes`；macOS 使用不跟随的 metadata 与 symlink 分类。

## Resume Algorithm

恢复时先核对本文件、Git status、HEAD/main、#143 和 Release Audit。八路径 planning dirty tree 只继续 Freeze；
产品 dirty tree 只允许恢复当前 TDD 节点。任何范围外路径、第二 writer、base 或授权漂移均停止，不自行扩大范围。
