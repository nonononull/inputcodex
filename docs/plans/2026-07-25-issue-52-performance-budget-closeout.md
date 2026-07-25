# Issue #52：性能预算 Discovery 合并后稳定状态 Closeout 计划

## 目标

将 Issue `#50` / PR `#51` 已完成的性能预算 Discovery 从“待 Review/CI 与 Squash Merge”改写为长期稳定终态，并把下一合法工作固定为独立性能复测与预算数值批准 Issue。

本任务只修改治理和状态文档，不填写预算数值，不修改性能实现、CI、Rust 或产品功能。

## 已验证来源事实

- Issue `#50` 已按 `COMPLETED` 关闭。
- PR `#51` Final Head 为 `e0154c61d8b05835db10437c79f029909516eac1`。
- PR `#51` 已正常 Squash Merge 为 `fea8824c652665df710a7e6ef941854060eb6e1f`。
- Squash 提交只有一个父提交，tree 为 `9fb518cda8b35a9388fb9fce0a1ff6ba976d80cb`，GitHub 签名为 `valid`。
- PR Final Head CI Run `30174131581` 的七个预期 Job 均为成功或合同性跳过，`required` 成功且 Artifact 为 `0`。
- 合并后主干 CI Run `30175592979` 七 Job 全部成功且 Artifact 为 `0`。
- `release_audit=current`，来源分支与工作树均保留，没有 Force Push 或删除 `main`。

## Brainstorming 决策

### 方案 A：独立八路径稳定状态 Closeout

采用。通过独立 Issue/PR 更新长期控制面，并把本 Closeout 自身的动态 PR/CI/授权证据外置到 GitHub 评论。优点是符合项目治理链，并避免合并后再次产生同类 Closeout。

### 方案 B：直接修改 `main`

拒绝。它违反 Issue、分支、PR 和 Squash Merge 约束，也会绕过 Review/CI。

### 方案 C：只保留 GitHub 评论

拒绝。评论可以保存动态证据，但不能修复 Master Plan、README、`AGENTS.md` 和 `build.md` 的过期长期状态。

项目所有者已通过 `user-message:建立独立-closeout-issue-pr-更新-master-plan-2026-07-25` 批准方案 A。

## 精确范围

基线：`fea8824c652665df710a7e6ef941854060eb6e1f`

分支：`codex/issue-52-performance-budget-closeout`

```text
AGENTS.md
README.md
build.md
docs/plans/2026-07-25-issue-52-performance-budget-closeout.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-25-issue-52-performance-budget-closeout.md
docs/reports/issue-52-performance-budget-closeout.md
docs/workflows/2026-07-25-issue-52-performance-budget-closeout-runtime.md
```

`scope_hash: sha256:af1cfffe1e72b847b212874ab6348bb6f375c54a43564cc702abb24145efb513`

## 执行批次

1. **issue-and-scope**：创建 Issue `#52`，建立隔离分支，冻结八路径、哈希、允许操作和禁止面。
2. **control-plane**：创建任务计划、Session Plan、Runtime Workflow 和稳定 Closeout 报告；AGOS `ReportOnly` 返回 `unregistered`、`needs-input` 与所有者直接写入阻断，已记录并按项目原生流程绕过。
3. **stable-state**：更新 `AGENTS.md`、README、`build.md` 与 Master Plan，清除 Issue `#50` 的待合并活动状态。
4. **verification**：运行 `build.md` 的 Issue `#52` 轻量验证、性能 Evidence、CI 合同、仓库政策和差异检查。
5. **delivery**：普通提交、普通推送并创建非 Draft PR；Review/CI 和最终 Head 证据只回写 GitHub。

## 反递归合同

- 永久文档可以记录 Issue `#50` / PR `#51` 的已验证终态。
- 永久文档不得把 Issue `#52`、其分支或未来 PR 写成新的“待合并活动任务”。
- Issue `#52` 的 PR Head、CI、Review、授权、Squash 和分支状态只保留在 Issue/PR 评论。
- 本 Closeout PR 合并后不得再为同一状态创建二次 Closeout。

## 明确不做

- 不填写或批准 warning/blocking 预算数值。
- 不运行新的 hosted 性能测量。
- 不修改 `benchmarks/`、Rust、Cargo、`apps/`、`crates/`、`parity/` 或 `upstream/`。
- 不修改 Workflow、Ruleset、Release 或 AGOS。
- 不实施性能优化，不迁移功能，不解锁 Gate 5。
- 不 Force Push，不删除 `main`、来源分支或工作树。

## 验收标准

- 实际差异精确等于八路径，scope hash 一致。
- 四个长期入口不再把 Issue `#50` / PR `#51` 描述为待合并任务。
- PR `#51` 的 Squash、单父、tree、签名、主干 CI、Artifact 和 Issue 关闭状态均可从长期文档复核。
- 下一合法工作只指向独立性能复测与数值批准 Issue。
- 本地轻量验证通过，完整 Workspace 和三平台构建交给 GitHub-hosted CI。
- 非 Draft PR 完成 Review/CI 后，最终 Squash Merge 仍需项目所有者针对最终 Head 单独授权。
