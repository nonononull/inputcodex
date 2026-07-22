---
status: accepted
---

# 解耦完整上游快照与功能目录审计基线

## 背景

`upstream/source-lock.json` 的 `snapshot` 表示已经完整缓存到本仓库、可供审计的上游源码。功能目录与 `source-index` 则代表已完成逐项审计的一致性基线。两者原先被强制相等：上游缓存一旦推进到新 Release，即使功能目录尚未复审，仓库级验证也会失败；若机械改写功能目录版本，又会伪造尚未发生的一致性结论。

## 决策

- `snapshot` 始终表示最新完整上游缓存，不再暗含功能目录已完成复审。
- 新增 `release_audit`：`catalog_release` 是 feature catalog 与 `source-index` 必须匹配的审计基线。
- `release_audit.status` 只允许两种状态：
  - `current`：`snapshot` 与 `catalog_release` 的 tag、commit 均一致，且没有 stale 原因或重新审计 Issue。
  - `stale-re-audit-required`：两套 Release 必须不同，且必须包含非空根因和有效的 `https://github.com/nonononull/inputcodex/issues/<编号>` 重新审计 Issue。
- 合法 stale 允许纯上游缓存同步和 parity 复审继续运行，并通过仓库验证；`RepositorySummary::requires_reaudit()` 必须显式返回 `true`。
- stale 时 PR 门禁阻断 `benchmarks/`、`apps/`、除 `crates/inputcodex-parity/` 以外的 `crates/`、`Cargo.toml` 与 `Cargo.lock`。`upstream/`、同步报告、文档、parity 复审资料和门禁本身不受该路径门禁阻断。
- 同一 PR 只要同时改变实际 `release_audit`、`upstream/source-lock.json` 与受阻产品路径，即使 Head 已恢复为 `current` 也必须失败。

## 影响

- 上游同步 PR 仍只能修改 `upstream/` 与同步报告；该 ADR 不授权功能迁移或对上游运行时实现的复用。
- 性能基线、预算和 Gate 5 产品迁移在 stale 状态下不能合并，必须先通过独立重新审计恢复 `current`。
- 非 PR 的 `push` 与手动 CI 只验证 `release_audit` 结构，避免把已知 stale 误报为仓库损坏；PR 才读取 base/head 差异执行路径门禁。
- 新的状态不是功能一致性的替代证明；它只让“已缓存但待复审”成为可诊断、不可误合并的中间状态。
