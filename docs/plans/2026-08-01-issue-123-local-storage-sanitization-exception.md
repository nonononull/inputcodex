# Issue #123：Local Storage 模型后缀清理一致性例外实施计划

## 任务元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/123`
- standing authorization：`https://github.com/nonononull/inputcodex/issues/111`
- Planning Freeze：`https://github.com/nonononull/inputcodex/issues/123#issuecomment-5150729887`
- 基线：`29f5317d66b9f16cf64797420bf2fd7e2aec45f7`
- 分支：`codex/issue-123-local-storage-sanitization-exception`
- 候选范围：`16` 路径
- `scope_hash`：`sha256:aa27e2551cfa743248ef7a2ab53fad5f1a1954b369ae40bf3485ada2099f7bdc`

## 目标

修正 `core-module:codex_local_storage` 在功能目录中的错误只读建模，不迁移任何产品能力：

1. 将 `feature.session-data.token-usage-history` 重命名为
   `feature.session-data.local-storage-model-suffix-sanitization`。
2. 将状态改为 `exception-pending`，来源副作用固定为
   `network-read / filesystem-write / injection`。
3. 合同只允许返回 `PARITY_EXCEPTION_PENDING`，禁止执行 CDP、JavaScript、Local Storage
   写入或上游 nonfatal 吞错路径。
4. 删除与真实入口无关的 Token 用量 synthetic fixture。
5. 将真正的 rollout Token 历史留给独立只读 Discovery。

## Discovery 事实

- `codex_local_storage.rs` 通过 CDP 获取 page target 和 WebSocket debugger URL，再执行
  JavaScript 读取 `__codexDailyTokenUsageV1`。
- 数据变化时调用 `localStorage.setItem` 写回；成功与失败路径均尝试写诊断日志。
- `launcher.rs` 只在 renderer 注入成功后调用 nonfatal 入口，错误会被吞掉。
- 真正的逐轮 Token 历史位于 `codex-plus-data/src/storage.rs` 的
  `codex_thread_usage_history` / `read_rollout_usage_history`，并经 `/thread-usage-history`
  路由暴露；它涉及 SQLite、rollout、会话标识和资源边界，不属于本入口。

## 不变量

- 来源、feature、contract、fixture、例外、排除与缺口计数固定为
  `135/44/44/12/11/3/0`。
- 产品 Rust、Cargo、Workflow、Ruleset、上游快照、根 README、UI、Release 和 AGOS
  均不得变化。
- Windows 与 macOS 均保持同一 `exception-pending` 语义。
- 不运行上游 Tauri/React、CDP、注入脚本或 Local Storage 写入。

## TDD 顺序

1. **RED**：先锁定真实副作用、重命名、例外计数与 fixture 删除。
2. **GREEN**：只修改冻结范围内的目录、合同、测试和文档。
3. **REFACTOR**：仅清理本任务引入的重复描述。
4. **VERIFY**：执行 `build.md` 的 Issue #123 本地轻量门，再进入普通 push、非 Draft PR、
   独立复审和 Hosted CI。

RED 已在提交 `e4b134953d802e6968511b241e669b1fb67d4ed9` 复现为
`26 passed / 3 failed`；三个失败分别锁定错误来源副作用、例外计数和 fixture 计数。

GREEN 本地门已于 Windows 本机时间 `2026-08-01 17:17:44.717 +08:00` 通过：目录
`29/29`、CI 合同 `76/76`、Release Audit `current`、Repository Policy `ok=true / 0`、
rustfmt、whitespace 与 16 路径/hash 均通过；Final Head 与 Hosted CI 证据按 Runtime Workflow
继续收集。

## 精确范围

完整 16 路径与 Ordinal hash 复算命令位于 `build.md` 和 Runtime Workflow。删除的两份
fixture 文件仍属于冻结路径；Git 不保留其空目录。

## 验证

```powershell
cargo test -p inputcodex-parity --test catalog_repository --offline
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
cargo fmt --all -- --check
git diff --check
```

## 停止门

- 实际范围越过 16 路径或 Ordinal hash 漂移。
- 需要执行或迁移 CDP、JavaScript、注入、Local Storage/诊断日志写入或真正的 rollout
  Token 历史。
- 需要修改产品 Rust/Cargo、Workflow、Ruleset、上游快照、README、UI、Release 或 AGOS。
- Final Head Review、Review thread、CI/Performance、Artifact 0、release audit、
  origin/main freshness 或精确 Head 证据不成立。
