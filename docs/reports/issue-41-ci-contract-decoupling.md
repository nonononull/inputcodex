# Issue #41：解耦上游快照同步与 CI 基线断言执行报告

## 元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/41`
- 基线：`317349a2cee1d2777472c8ccbd55204e570176c4`
- 分支：`codex/issue-41-ci-contract-decoupling`
- 阻塞对象：PR `#40`，Head `86d48ad261669daaf14666556372a12f9b908726`
- 范围批准：`https://github.com/nonononull/inputcodex/issues/41#issuecomment-5067938336`
- `scope_hash`：`sha256:ada2baa0a524b2c8f0831d946236197b056513981c30b4530d903114b709c1b8`
- 本地验证时间：`2026-07-24 16:55:47 +08:00`，来自 Windows `Get-Date`
- 最终本地门禁时间：`2026-07-25 13:03:50 +08:00`，来自 Windows `Get-Date`
- 实现提交：`af866b6fabb41de3a9ea42b44859ef73d7a1549b`，项目所有者本机提交时间为 `2026-07-25 13:04:15 +08:00`。
- PR：`https://github.com/nonononull/inputcodex/pull/42`，非 Draft；`af866b6` 的首轮 GitHub-hosted CI 已全绿。
- 控制面更正时间：`2026-07-25 13:20:33 +08:00`，来自 Windows `Get-Date`。
- 当前阶段：首轮治理审查发现文档阶段滞后，本更正提交正在修复；更正后的最终 Head 必须重新完成双 reviewer、Review 对话检查和 GitHub-hosted CI，随后单独请求 Squash Merge 授权。

## 根因

### Python 固定 Release 对象

`.github/scripts/tests/test_upstream_watch.py` 的仓库测试读取 `upstream/source-lock.json` 后，与辅助函数 `baseline()` 中固定的 `v1.2.41` 完整对象比较。`baseline()` 是其他合成监控观测测试的稳定夹具，却被错误复用于验证仓库当前锁定 Release。

PR `#40` 合法更新至 `v1.2.42` 后，`load_baseline` 已成功读取并完成 schema、仓库、tag、UTC 时间、Release URL 和 SHA 校验，但最终被测试自己的陈旧期望值判为失败。

### Rust 总体验证拒绝合法 stale

`crates/inputcodex-parity/tests/catalog_repository.rs` 同时存在：

1. 专项状态测试，规定 `current` 为 `requires_reaudit=false`，合法 `stale-re-audit-required` 为 `true`，非法 stale 组合以 `ReleaseMismatch` 失败；
2. 仓库总体验证，无条件断言 `!summary.requires_reaudit()`。

两个合同互相矛盾。纯缓存同步后，目录审计尚未重跑的合法状态必须是 stale；总体验证不应重复决定具体状态，只应验证仓库结构、引用、安全和计数不变量。

## RED 证据

在未修改的 PR `#40` 工作树执行：

```text
test_repository_source_lock_loads_as_baseline ... FAIL
AssertionError: ... v1.2.42 ... != ... v1.2.41 ...
Ran 28 tests
FAILED (failures=1)
PYTHON_RED_EXIT=1

test 仓库功能目录通过完整引用与安全验证 ... FAILED
assertion failed: !summary.requires_reaudit()
test result: FAILED. 0 passed; 1 failed; 9 filtered out
RUST_RED_EXIT=101
```

复现后 PR `#40` 工作树无 tracked、staged 或 untracked 变更。

## 最小修复

- Python 增加标准库 `json`，读取 `source-lock.snapshot` 并动态构造期望 `watch.Baseline`；仍与 `load_baseline` 结果做全对象相等比较。
- 动态期望对象继续触发 `Baseline.__post_init__`，因此固定上游仓库、tag、UTC RFC3339、Release URL 与 SHA 的失败关闭校验没有降低。
- Rust 仓库总体验证只删除一条与专项状态合同冲突的 `assert!(!summary.requires_reaudit())`；源条目、功能、合同、夹具和覆盖缺口计数仍全部验证。
- 不修改 `upstream/`、`source-lock.json`、目录数据、Workflow、Cargo、产品 crate、Ruleset、Release、PR `#40` 或 AGOS。

## 主干基线 GREEN

```text
python -m unittest discover -s .github/scripts/tests -p 'test_upstream_watch.py' -v
Ran 28 tests
OK

python .github/scripts/upstream_watch.py --validate-only
release_tag=v1.2.41 status=valid

cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
test result: ok. 10 passed; 0 failed

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
ok=true status=current requires_reaudit=false

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
ok=true violation_count=0
```

## PR #40 临时叠加回放

从 PR `#40` Head 建立 detached 临时工作树，只应用本任务的两个测试文件差异；没有创建提交、分支或远端引用。结果：

```text
python -m unittest discover -s .github/scripts/tests -p 'test_upstream_watch.py' -v
Ran 28 tests
OK

python .github/scripts/upstream_watch.py --validate-only
release_tag=v1.2.42 release_commit=657cd33e009ad02515d30db6492cd4e669b06318 status=valid

cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
test result: ok. 10 passed; 0 failed

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
ok=true status=stale-re-audit-required requires_reaudit=true

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
ok=true violation_count=0
```

临时工作树 `issue-41-pr40-overlay-temp` 已移除，PR `#40` 工作树和提交内容均未改变。

## 最终本地门禁

```text
verify-session-plan.ps1
SESSION_PLAN_VERIFY_OK

python -m unittest discover -s .github/scripts/tests -p 'test_upstream_watch.py' -v
Ran 28 tests
OK

python -m py_compile .github/scripts/upstream_watch.py .github/scripts/tests/test_upstream_watch.py
exit=0

cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
test result: ok. 10 passed; 0 failed

cargo fmt --all -- --check
exit=0

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
CI_CONTRACT_GREEN passed=32

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
ok=true status=current requires_reaudit=false

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
ok=true violation_count=0

git diff --check
exit=0
```

## 精确范围

```text
.github/scripts/tests/test_upstream_watch.py
crates/inputcodex-parity/tests/catalog_repository.rs
docs/plans/2026-07-24-issue-41-ci-contract-decoupling.md
docs/plans/sessions/2026-07-24-issue-41-ci-contract-decoupling.md
docs/reports/issue-41-ci-contract-decoupling.md
docs/workflows/2026-07-24-issue-41-ci-contract-decoupling-runtime.md
err.md
```

## 未完成门禁

- 本控制面更正提交的精确七路径、`scope_hash`、Session Plan、仓库政策与 `git diff --cached --check`；
- 更正后的最终 Head 双 reviewer 审查；
- 更正后的最终 PR Head GitHub-hosted Upstream Watch、Linux、Windows、macOS、required CI；
- 所有 Review 对话解决、最终 Squash Merge 单独授权和合并后 closeout。

## 首轮审查与处理

- 技术 reviewer：无 Critical、Important 或 Minor 发现；动态 `source-lock.snapshot` 映射、失败关闭与 stale 专项状态语义均保持。
- 治理 reviewer：发现 Important——报告和 Session Plan 把已完成的暂存、提交、推送与 PR 仍列为待完成，可能误导后续 Review/Closeout。
- 根因：实施、提交和 PR 创建后没有在同一交付阶段回写控制面状态。
- 处理：本更正提交只更新获批范围内的计划、Runtime Workflow 与报告，保留首次 CI 仅对应旧 Head 的事实，并强制新 Head 重新审查和重新跑 CI。
- 验证：更正提交后重新核验七路径 `scope_hash`、Session Plan、政策与空白门禁；最终 Head 的双 reviewer 和 CI 结果将在本节追加。
