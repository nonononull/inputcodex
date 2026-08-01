# Issue #132 Runtime Workflow：Zed 远程项目只读观察

## Runtime Metadata

- `task_id`: `issue-132-gate-5-zed-remote-project-observation`
- `tracking_issue_ref`: https://github.com/nonononull/inputcodex/issues/132
- `session_plan_ref`: `docs/plans/sessions/2026-08-02-issue-132-gate-5-zed-remote-project-observation.md`
- `approved_decision_ref`: https://github.com/nonononull/inputcodex/issues/131#issuecomment-5152838531
- `implementation_scope_approval_ref`: https://github.com/nonononull/inputcodex/issues/132#issuecomment-5152853748
- `baseline_ref`: `da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4`
- `branch_ref`: `codex/issue-132-gate-5-zed-remote-project-observation`
- `candidate_scope_hash`: `sha256:7ee5d47dca72d0d2f1ec683cc45e4bbac0e3ce40af1e57417d39608c8c0c26bb`
- `runtime_state`: `domain-red-next`

## Current Gate

```text
Issue #130 performance-first defer completed
  -> Issue #131 ADOPT_B completed
  -> Issue #132 planning checkpoint completed
  -> NEXT: Domain RED
```

`ALLOWED_OPS`：二十八路径内计划、TDD、唯一依赖、Parity、稳定文档、验证、checkpoint、普通 push/PR、只读复审和精确门禁合并。

`FORBIDDEN_OPS`：范围外修改、legacy registry、原始身份披露、写入、网络、进程、UI、unsafe、额外依赖、force push、GitHub auto-merge 或 main 直写。

## Node Order

### Node 0：Startup Baseline

确认本机时间、worktree/branch、Head/origin/main、Issue #132、单 writer、v1.2.44 与 `release_audit=current`。

状态：`completed`。

### Node 1：Decision Evidence

Issue #131 采用方案 B；两名 reviewer 均为 `ADOPT_B`，父线程采用更严格交集。

状态：`completed`。

### Node 2：Planning Freeze

写入 7 路径控制面，验证 planning/candidate hash、策略、Release Audit、仓库政策和 diff，建立 planning checkpoint。

状态：`completed`。

### Node 3：Domain TDD

目标 API 缺失 RED -> 最小领域值 GREEN -> tests/Clippy/fmt -> domain checkpoint。

状态：`pending`。

### Node 4：Application TDD

Request/Cancellation/Port/UseCase RED -> GREEN -> tests/Clippy/fmt -> application checkpoint。

状态：`pending`。

### Node 5：Platform TDD

稳定 ID、NOFOLLOW JSON、严格 SQLite、资源/错误/取消矩阵 RED -> GREEN -> 依赖/测试/Clippy/fmt -> platform checkpoint。

状态：`pending`。

### Node 6：Parity TDD

新 feature/contract/fixture 和单 command source RED -> GREEN -> 精确计数/Release Audit -> parity checkpoint。

状态：`pending`。

### Node 7：Local Closeout

更新 README/报告，运行四 crate、治理、范围、依赖、隐私和 diff 门禁，建立 local-verified checkpoint。

状态：`pending`。

### Node 8：Remote Delivery

普通 push -> 非 Draft PR -> Final Head 独立只读复审 -> Review/CI/Performance/Artifact -> 精确 Head Squash -> main 双 Workflow -> Issue/workspace/reviewer 归档。

状态：`pending`。

## Error Watchlist

- FNV 四字段拼接存在分隔符歧义和 64 位碰撞，不得进入产品 ID。
- `SelectedHostHint` 不是运行态；空 payload 不足以证明真实当前项目。
- JSON 必须同句柄限读并复验；单独 `symlink_metadata` 后再普通打开仍有替换窗口。
- SQLite 保留 WAL reader 的 SHM 协调例外，但主库/WAL 与目录清单不得改变。
- Partial 只存在于领域 snapshot coverage，不扩展通用 `LoadCompletion`。

## Delivery Gate

- Final Head、Issue planning evidence、PR evidence、28 路径、policy hash 与 review attestation 一致。
- Review thread 0；CI 7/7；Performance 4/4；成功 Artifact 0；mergeable=CLEAN；release audit current。
- Squash 后单父、tree 等价、GitHub 签名 valid、origin/main 新鲜且 main 双 Workflow 全绿。

## AGOS Boundary

本任务使用项目原生控制面；当前会话无 `superpowers:*` 和 `local_knowledge_lookup` 工具，按项目合同记录并绕过。不得修改任何 AGOS 跨仓资产。
