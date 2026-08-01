# Issue #123：Local Storage 模型后缀清理一致性例外 Runtime Workflow

```yaml
task_id: issue-123-local-storage-sanitization-exception
task_kind: refactor
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/123
standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/123#issuecomment-5150729887
session_plan_ref: docs/plans/sessions/2026-08-01-issue-123-local-storage-sanitization-exception.md
implementation_plan_ref: docs/plans/2026-08-01-issue-123-local-storage-sanitization-exception.md
baseline_ref: 29f5317d66b9f16cf64797420bf2fd7e2aec45f7
branch: codex/issue-123-local-storage-sanitization-exception
scope_count: 16
scope_hash: sha256:aa27e2551cfa743248ef7a2ab53fad5f1a1954b369ae40bf3485ada2099f7bdc
local_time_source: Windows Get-Date
merge_policy: exact-head-squash-under-standing-authorization
```

## 节点

1. `startup-baseline`：核对 origin/main、工作树、Issue、目录 28/28、CI 76/76、Policy 与 Release Audit。
2. `discovery`：从缓存源码复核 CDP、WebSocket、JavaScript、Local Storage 写回、诊断日志和 launcher 调用链。
3. `planning-freeze`：回写 owner marker、16 路径、Ordinal hash、重命名与延期决定。
4. `tdd-red`：只改目录测试并确认 `26/29` 预期 RED，提交单文件检查点。
5. `bounded-green`：只更新冻结的目录、合同、fixture 删除与文档。
6. `local-verify`：执行目录、CI 合同、Release Audit、Repository Policy、rustfmt、scope 与 whitespace 门。
7. `delivery`：普通提交、普通 push、非 Draft PR；不得 force push。
8. `final-head-review`：独立只读复审、Review thread=0、CI 7/7、Performance 4/4、Artifact=0。
9. `exact-squash`：绑定 Final Head、policy hash、scope hash 与 #111 authorization 执行 Squash。
10. `post-merge`：验证单父、tree 等价、签名、main freshness、双套 main Actions 与 Artifact 0，再关闭 Issue 和归档。

## 精确路径

```text
AGENTS.md
CONTEXT.md
build.md
crates/inputcodex-parity/tests/catalog_repository.rs
docs/plans/2026-08-01-issue-123-local-storage-sanitization-exception.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-08-01-issue-123-local-storage-sanitization-exception.md
docs/reports/issue-123-local-storage-sanitization-exception.md
docs/workflows/2026-08-01-issue-123-local-storage-sanitization-exception-runtime.md
err.md
parity/README.md
parity/contracts/session-data.yml
parity/features/session-data.yml
parity/features/source-index.yml
parity/fixtures/feature.session-data.token-usage-history/baseline.yml
parity/fixtures/feature.session-data.token-usage-history/manifest.yml
```

## 验证命令

```powershell
cargo test -p inputcodex-parity --test catalog_repository --offline
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
cargo fmt --all -- --check
git diff --check
```

范围按 `StringComparer.Ordinal` 排序，以 LF 拼接并保留末尾 LF；必须精确为 `16` 与
`sha256:aa27e2551cfa743248ef7a2ab53fad5f1a1954b369ae40bf3485ada2099f7bdc`。

## 当前证据

```yaml
evidence:
  baseline_catalog: 28/28
  baseline_ci_contract: 76/76
  baseline_repository_policy: ok=true, violation_count=0
  baseline_release_audit: current, requires_reaudit=false
  planning_marker: valid, task_kind=refactor, scope=16
  red_checkpoint: e4b134953d802e6968511b241e669b1fb67d4ed9
  red_catalog: 26/29; three intended failures
  green_catalog: 29/29 at Windows local time 2026-08-01 17:17:44.717 +08:00
  green_ci_contract: 76/76
  green_release_audit: current, requires_reaudit=false, blocked_paths=0
  green_repository_policy: ok=true, violation_count=0
  green_scope: 16 paths, exact frozen hash
  pr_ref: pending
  final_head: pending
```

## 停止门

- 任一实际路径越过冻结集合或 scope hash 漂移。
- GREEN 需要执行或迁移 CDP、JavaScript、Local Storage/诊断日志写入、rollout Token 历史，
  或修改产品、Cargo、Workflow、Ruleset、upstream、README、UI、Release、AGOS。
- 独立 Review、Review threads、CI/Performance、Artifact、release audit、origin/main freshness
  或精确 Head 证据未通过。
