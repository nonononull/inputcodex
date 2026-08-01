# Issue #123：Local Storage 模型后缀清理一致性例外 Discovery 报告

## 结论

`feature.session-data.token-usage-history` 是错误分类。其唯一来源
`core-module:codex_local_storage` 并不读取 SQLite 或 rollout Token 历史，而是在 renderer
注入成功后通过 CDP 执行 JavaScript，读取并可能改写 Electron Local Storage。该来源必须改名为
`feature.session-data.local-storage-model-suffix-sanitization` 并保持 `exception-pending`。

## 可复核调用链

1. `launcher.rs` 在 `injection_ready` 后调用
   `sanitize_local_storage_model_suffixes_nonfatal(debug_port)`。
2. `sanitize_local_storage_model_suffixes` 列出 CDP targets、选择 page target、取得 WebSocket
   debugger URL，并执行固定 JavaScript。
3. JavaScript 读取 `__codexDailyTokenUsageV1`，清理 `turn.model` 尾部后缀，并在变化时调用
   `localStorage.setItem` 写回。
4. 成功路径写 `codex_local_storage.sanitize_model_suffixes` 诊断记录；失败路径写
   `codex_local_storage.sanitize_model_suffixes_failed`，nonfatal 包装器不向调用方传播错误。

因此旧登记的 `[filesystem-read, database-read]`、无注入只读语义和 synthetic Token history
fixture 均与缓存源码不符。

## 采用方案

- source side effects：`[network-read, filesystem-write, injection]`。
- disposition 与 feature：`exception-pending`。
- 合同：只返回稳定 `PARITY_EXCEPTION_PENDING`，`side_effects: [none]`，无 fixture。
- 计数：`135/44/44/12/11/3/0`。
- 产品行为：零变化；Windows/macOS 均不执行上游副作用链。

## 延期能力

真正的 Token 历史位于 `codex-plus-data/src/storage.rs`：

- `codex_thread_usage_history` 从受控 SQLite 定位 rollout。
- `read_rollout_usage_history` 逐行解析 `turn_context` 与 `token_count` 事件。
- `routes.rs` 通过 `/thread-usage-history` 暴露该能力。

它需要独立评估会话 ID、SQLite/rollout 允许来源、读取上限、超时取消、最小披露和错误隔离，
不能与本次 harmful-side-effect isolation 合并。

## TDD 证据

- Fresh 基线：`28/28`。
- RED：`e4b134953d802e6968511b241e669b1fb67d4ed9`，`26/29`；失败精确命中
  source side effects、exception count 和 fixture count。
- GREEN 核心：`29/29`，旧 feature/contract ID 与两份 fixture 文件均不存在。

## 冻结范围

- 路径数：`16`
- Ordinal hash：`sha256:aa27e2551cfa743248ef7a2ab53fad5f1a1954b369ae40bf3485ada2099f7bdc`
- Planning Freeze：`https://github.com/nonononull/inputcodex/issues/123#issuecomment-5150729887`
- 明确不含：产品 Rust/Cargo、Workflow、Ruleset、上游快照、README、UI、Release 与 AGOS。
