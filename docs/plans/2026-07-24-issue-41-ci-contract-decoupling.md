# Issue #41：解耦上游快照同步与 CI 基线断言计划

## 任务元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/41`
- 发现基线：`317349a2cee1d2777472c8ccbd55204e570176c4`
- 实现分支：`codex/issue-41-ci-contract-decoupling`
- 本机计划时间：`2026-07-24 15:57:49 +08:00`，来自 Windows `Get-Date`
- 所有者发现授权：`user-message:批准独立-CI-合同-Issue-PR-2026-07-24`
- 精确写入范围状态：`已批准`
- 精确范围批准：`https://github.com/nonononull/inputcodex/issues/41#issuecomment-5067938336`
- 阻塞 PR：`#40`，Head 为 `86d48ad261669daaf14666556372a12f9b908726`；本任务不得改写其二十路径缓存同步范围。

## 根因与证据

1. `.github/scripts/tests/test_upstream_watch.py` 的 `test_repository_source_lock_loads_as_baseline` 将仓库 `source-lock` 与辅助函数中固定的 `v1.2.41` `Baseline` 全对象相等比较。该辅助函数仍服务于合成观测单测；当 PR `#40` 合法地把 `source-lock` 更新为 `v1.2.42` 时，仓库输入已正确，但测试把可变 Release 值错误地当作基线不变量。
2. `crates/inputcodex-parity/tests/catalog_repository.rs` 的 `仓库功能目录通过完整引用与安全验证` 无条件断言 `!summary.requires_reaudit()`。同文件已有专项测试明确证明 `current` 与 `stale-re-audit-required` 都是合法状态：前者必须为 `false`，后者必须为 `true`。
3. 本机在未修改的 PR `#40` 工作树复现：Python 上游监控测试退出码 `1`，唯一失败为 `test_repository_source_lock_loads_as_baseline`；Rust 精确测试退出码 `101`，失败为 `assertion failed: !summary.requires_reaudit()`。复现后 #40 工作树仍无变更。
4. Git 历史显示 `d7438a0` 在引入 stale 专项合同的同一轮中加入全局 `!summary.requires_reaudit()` 断言，形成两个相互矛盾的合同。#40 没有修改上述两个测试文件。

## 方案比较

### 方案 A：改为稳定字段与状态语义合同（采用）

- Python 仓库 `source-lock` 测试继续调用 `load_baseline`，并与文件中动态读取的 snapshot 字段构造出的 `Baseline` 做全对象对账；不再比较某个固定 Release 的完整对象。
- Rust 仓库总体验证继续要求 `validate_repository` 成功和所有计数不变量成立，删除唯一将合法 stale 状态视为失败的断言；保留现有 current/stale 专项测试作为状态语义证据。
- 优点：根因处解除“快照值”和“状态值”的错误耦合，不降低 `Baseline`、Release URL、SHA、目录审计或三平台验证强度。

### 方案 B：把 Python 固定值更新为 `v1.2.42`（拒绝）

- 只能让本次缓存同步暂时通过；下一次合法 Release 同步会再次失败。
- 不能解决 Rust 总体验证与 stale 专项合同的矛盾。

### 方案 C：把 CI/测试修复混入 PR #40（拒绝）

- 违反“上游缓存同步 PR 只能更新 `upstream/` 与同步报告”的项目规则。
- 破坏 #40 已批准的二十路径范围与审计可追溯性。

## 精确写入范围提案

按 Windows `Sort-Object` 的大小写不敏感字典序固定为以下七条 POSIX 路径：

```text
.github/scripts/tests/test_upstream_watch.py
crates/inputcodex-parity/tests/catalog_repository.rs
docs/plans/2026-07-24-issue-41-ci-contract-decoupling.md
docs/plans/sessions/2026-07-24-issue-41-ci-contract-decoupling.md
docs/reports/issue-41-ci-contract-decoupling.md
docs/workflows/2026-07-24-issue-41-ci-contract-decoupling-runtime.md
err.md
```

路径以 LF 连接并保留一个末尾 LF 的 SHA-256：

```text
scope_hash: sha256:ada2baa0a524b2c8f0831d946236197b056513981c30b4530d903114b709c1b8
```

任何新增、删除、重命名路径，或修改 `.github/workflows/`、`upstream/`、`source-lock.json`、Cargo、产品 crate、Ruleset、Release、AGOS，都会使本提案失效，必须重新获得项目所有者批准。

## 计划补丁与 RED → GREEN

### Python 合同

保留 `baseline()` 为合成观测测试的固定输入；仓库 `source-lock` 测试继续全对象对账，但期望值改为从文件 snapshot 动态构造：

```python
source_lock_path = ROOT / "upstream" / "source-lock.json"
source_lock = json.loads(source_lock_path.read_text(encoding="utf-8"))
snapshot = source_lock["snapshot"]
loaded = watch.load_baseline(source_lock_path)
self.assertEqual(
    loaded,
    watch.Baseline(
        upstream_repository=snapshot["repository"],
        release_tag=snapshot["release_tag"],
        release_published_at=snapshot["release_published_at"],
        release_url=snapshot["release_url"],
        release_commit=snapshot["commit"],
    ),
)
```

`Baseline.__post_init__` 仍实际执行仓库、tag、UTC RFC3339、Release URL 和 SHA 的失败关闭验证；动态全对象对账还保留了 loader 字段映射强度，本测试不引入网络请求。

### Rust 合同

保留仓库级成功、源条目、功能、合同、夹具和覆盖缺口计数断言；只删除 `assert!(!summary.requires_reaudit())`。同文件 `release_audit_显式解耦快照与功能目录审计基线` 继续覆盖：

1. `current` 时 `requires_reaudit()` 为 `false`；
2. `stale-re-audit-required` 且快照/目录 Release 不同、根因和 Issue 引用完整时为 `true`；
3. 不合法 stale 组合仍以 `ReleaseMismatch` 失败关闭。

### 验证序列

1. **RED 已完成：** 在 #40 未修改工作树运行 Python 28 测试，唯一失败为固定 `v1.2.41` 对象比较；运行 Rust 精确测试，唯一失败为 `!summary.requires_reaudit()`。
2. **GREEN：** 在本分支运行完整上游监控单测、`upstream_watch.py --validate-only`、`catalog_repository` 定向测试、Release Audit Gate、仓库政策、`git diff --check` 和七路径哈希校验。
3. **合并模拟：** 在临时 detached 工作树中以未提交方式把 PR #40 Head 合入本任务最终 Head；只运行同一 Python/Rust 定向合同和 Release Audit Gate，然后 `git merge --abort` 并移除临时工作树，不创建提交、远端分支或缓存变更。
4. **远端：** 创建只含七路径的非 Draft PR；标准 GitHub-hosted CI 必须全绿。CI 合同 PR Squash Merge 后，观察 #40 以新的 `main` 基线自动重跑。

## 明确禁止

- 不将本任务的任何修改放入 PR #40；不修改 #40 Head 或其二十路径哈希。
- 不把固定版本替换成另一个固定版本，不跳过或降低现有检查。
- 不修改上游快照、目录审计数据、功能目录、产品代码、Cargo、Workflow、Ruleset、Release 或 AGOS。
- 不 force push，不删除 `main`，不使用 Merge Commit 或 Rebase Merge，不带未解决 Review 对话合并。

## 外部治理辅助状态

- `functions.list_mcp_resources` 与 `functions.list_mcp_resource_templates` 均返回空；当前会话未暴露可调用的 `local_knowledge_lookup`，不能伪造查询结果。
- AGOS `invoke-agos-default-entry.ps1 -ReportOnly` 返回 `needs-input` 与 `unregistered`；项目 Git 和入口文档检查均为 ready。按照项目规则立即绕过其写入/注册要求，不修改 AGOS。
- 后续以本计划、Session Plan、Runtime Workflow、Issue #41、Git、Review 与 CI 为项目原生证据面。

## 批准门

- 项目所有者已通过未变化范围下的“继续”继承批准上述七路径和 `scope_hash`，允许实现、轻量验证、临时叠加模拟、普通提交、正常推送、非 Draft PR 与 Review/CI。
- 最终 Squash Merge 不在当前授权内；必须在最终 PR Head、所有 Checks 和 Review 对话均通过后单独请求项目所有者授权。
