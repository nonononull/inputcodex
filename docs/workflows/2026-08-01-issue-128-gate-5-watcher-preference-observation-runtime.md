# Issue #128 Runtime Workflow：Watcher 偏好状态只读观察

## Runtime Metadata

- `task_id`: `issue-128-gate-5-watcher-preference-observation`
- `tracking_issue_ref`: https://github.com/nonononull/inputcodex/issues/128
- `session_plan_ref`: `docs/plans/sessions/2026-08-01-issue-128-gate-5-watcher-preference-observation.md`
- `approved_decision_ref`: https://github.com/nonononull/inputcodex/issues/127#issuecomment-5151021881
- `implementation_scope_approval_ref`: https://github.com/nonononull/inputcodex/issues/128#issuecomment-5151032720
- `baseline_ref`: `43ace17de1505f251812e4ead3035ef3274a8455`
- `branch_ref`: `codex/issue-128-gate-5-watcher-preference-observation`
- `candidate_scope_hash`: `sha256:0be3dd45ed7e91d1cb7f633da369bb2b9052cefc512852d80362775da7e81699`
- `runtime_state`: `parity-red-ready`

## Current Gate

```text
Issue #127 decision completed
  -> Issue #128 scope and planning evidence frozen
  -> Planning control plane completed
  -> Domain RED/GREEN completed
  -> Application RED/GREEN completed
  -> Platform RED/GREEN completed
  -> NEXT: Parity RED
  -> Local verification
  -> Review / CI / exact-head Squash
```

当前 `ALLOWED_OPS`：二十四路径内 TDD、文档、验证、checkpoint、普通 push、PR、只读复审与门禁合并。

当前 `FORBIDDEN_OPS`：范围外修改、内容读取、任意路径、路径回显、写入、进程控制、网络、子进程、线程、UI、依赖、force push、GitHub auto-merge 或 `main` 写入。

## Node Order

### Node 0：Startup Baseline

1. 本机 `Get-Date`。
2. 确认 worktree/branch/Head 与 `origin/main@43ace17...`。
3. 确认 Issue #128 planning evidence、单 writer、Release Audit current。
4. 确认最新正式 Release 仍为 `v1.2.44`。

状态：`completed`。

### Node 1：Decision Evidence

Issue #127 采用方案 B：只拆分偏好标记观察，完整 Watcher 管理继续未评估。

状态：`completed`。

### Node 2：Planning Freeze

1. 写入 7 路径控制面。
2. 验证 planning hash `sha256:359bfb23...` 与 candidate hash `sha256:0be3dd45...`。
3. 运行自治策略、Release Audit、仓库政策和 diff 检查。
4. 建立 `issue-128-planning-freeze` checkpoint。

状态：`completed`。

### Node 3：Domain TDD

RED -> GREEN -> Domain tests/Clippy/fmt -> `issue-128-domain-green`。

状态：`completed`。

### Node 4：Application TDD

RED -> GREEN -> Application tests/Clippy/fmt -> `issue-128-application-green`。

状态：`completed`。

### Node 5：Platform TDD

RED -> GREEN -> Platform tests/Clippy/fmt -> `issue-128-platform-green`。

状态：`completed`。

### Node 6：Parity TDD

RED -> GREEN -> Parity tests/Release Audit/Clippy/fmt -> `issue-128-parity-green`。

状态：`pending`。

### Node 7：Local Closeout

更新 README/报告/err，运行四 crate、治理、范围、隐私和 diff 门禁，建立 `issue-128-local-verified`。

状态：`pending`。

### Node 8：Remote Delivery

普通 push -> 非 Draft PR -> 独立 Final Head 只读复审 -> Review/CI/Performance/Artifact -> 精确 Head Squash -> main 双 Workflow -> Issue/workspace/reviewer 归档。

状态：`pending`。

## Scope Enforcement

实际路径必须精确等于 Issue #128 的 24 路径，使用 `StringComparer.Ordinal`、UTF-8 无 BOM、LF 与末尾 LF；hash 必须为 `sha256:0be3dd45ed7e91d1cb7f633da369bb2b9052cefc512852d80362775da7e81699`。

## Error Watchlist

- 上游 `Path::exists` 会把部分 I/O 错误折叠为 false；生产实现必须使用可返回错误的元数据 API。
- `symlink_metadata` 必须先分类 symlink，再判断普通文件；不得跟随标记链接。
- 标记缺失是 Ready/EnabledByDefault，不是 Empty，也不是 watcher 已运行。
- Windows PowerShell 的路径 hash 使用 Ordinal 集合，不能使用 culture-sensitive `Sort-Object -Unique`。

## Delivery Gate

- Final Head、Issue planning evidence、PR evidence、scope、policy hash 和 review attestation 全部一致。
- Review thread 0；CI 7/7；Performance 4/4；成功 Artifact 0；mergeable=CLEAN。
- Squash 后单父、tree 等价、GitHub 签名 valid、`origin/main` 新鲜且 main 两套 Workflow 全绿。

## AGOS Boundary

本项目使用 project-native 流程；AGOS 不可用或未登记不得阻塞，也不得在 Issue #128 修改任何跨仓控制面。
