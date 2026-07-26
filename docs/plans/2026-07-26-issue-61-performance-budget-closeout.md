# Issue #61：性能预算数值合并后稳定状态 Closeout 计划

## 目标

将 Issue `#59` / PR `#60` 已完成的固定复测与性能预算数值从“待 Review/CI 与 Squash Merge”改写为长期稳定终态，并把下一合法工作固定为独立预算 CI 观察 Issue。

本任务只修改治理和状态文档，不修改预算数值、性能实现、CI、Rust 或产品功能。

## 已验证来源事实

- Issue `#59` 已按 `COMPLETED` 关闭。
- PR `#60` Final Head 为 `61c088d74d61a329fbe67e14b8280dfa9701c6b2`。
- PR `#60` 已正常 Squash Merge 为 `e225144831a0928bfa3aaa0d169a054779005812`。
- Squash 提交只有一个父提交，tree 为 `56eb1e8d95dfce22726c1aef1bdde1c353af055e`，GitHub 签名为 `valid`。
- PR Final Head CI Run `30194465259` 七 Job 全绿，Performance Baseline Run `30194465231` 四 Job 全绿，Artifact 数均为 `0`。
- 合并后主干 CI Run `30194897171` 七 Job 全绿，Performance Baseline Run `30194897166` 四 Job 全绿，Artifact 数均为 `0`。
- Windows 严格目标队列为 `5`，macOS 同队列为 `12`，不存在 `run-05`。
- 预算 JSON 归一化 SHA-256 为 `sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5`，离线复算合同为 `10/10` GREEN。
- 来源分支与工作树均保留，没有 Force Push 或删除 `main`。

## Brainstorming 决策

### 方案 A：独立八路径稳定状态 Closeout

采用。通过独立 Issue/PR 更新长期控制面，并把本 Closeout 自身的动态 PR/CI/授权证据外置到 GitHub 评论。该方案符合项目治理链，并避免合并后再次产生同类 Closeout。

### 方案 B：直接修改 `main`

拒绝。它违反 Issue、分支、PR 和 Squash Merge 约束，也会绕过 Review/CI。

### 方案 C：只保留 GitHub 评论

拒绝。评论可以保存动态证据，但不能修复 Master Plan、README、`AGENTS.md` 和 `build.md` 的过期长期状态。

项目所有者已通过 `https://github.com/nonononull/inputcodex/issues/61#issuecomment-5082819125` 批准方案 A、精确八路径、范围哈希和实施/PR 边界。

## 精确范围

基线：`e225144831a0928bfa3aaa0d169a054779005812`

分支：`codex/issue-61-performance-budget-closeout`

以下路径按 Windows PowerShell `Sort-Object` 排序：

```text
AGENTS.md
build.md
docs/plans/2026-07-26-issue-61-performance-budget-closeout.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-61-performance-budget-closeout.md
docs/reports/issue-61-performance-budget-closeout.md
docs/workflows/2026-07-26-issue-61-performance-budget-closeout-runtime.md
README.md
```

`scope_hash: sha256:dafe55bfc38c38782558c1577215d227ac8c83b7110735c4ddd58b48d66264b5`

## 执行批次

1. **issue-and-scope**：建立 Issue `#61`、隔离分支、八路径、范围哈希、允许操作和禁止面。
2. **control-plane**：创建任务计划、Session Plan、Runtime Workflow 和稳定 Closeout 报告；AGOS `ReportOnly` 返回 `unregistered`、`needs-input` 与所有者直接写入阻断，按项目原生流程绕过。
3. **stable-state**：更新 `AGENTS.md`、README、`build.md` 与 Master Plan，清除 Issue `#59` 的待合并活动状态。
4. **verification**：运行 `build.md` 的 Issue `#61` 轻量验证、性能 Evidence、预算复算、CI 合同、仓库政策和差异检查。
5. **delivery**：普通提交、普通推送并创建非 Draft PR；Review/CI 和最终 Head 证据只回写 GitHub。

## 反递归合同

- 永久文档可以记录 Issue `#59` / PR `#60` 的已验证终态。
- 永久文档不得把 Issue `#61`、其分支或未来 PR 写成新的“待合并活动任务”。
- Issue `#61` 的 PR Head、CI、Review、授权、Squash 和分支状态只保留在 Issue/PR 评论。
- 本 Closeout PR 合并后不得再为同一状态创建二次 Closeout。

## 明确不做

- 不修改已批准预算数值、公式、量子、队列、样本或 JSON 哈希。
- 不运行新的 hosted 性能测量。
- 不修改 `benchmarks/`、Rust、Cargo、`apps/`、`crates/`、`parity/` 或 `upstream/`。
- 不修改 Workflow、Ruleset、Release 或 AGOS。
- 不实施性能优化，不迁移功能，不解锁 Gate 5。
- 不 Force Push，不删除 `main`、来源分支或工作树。

## 验收标准

- 实际差异精确等于八路径，scope hash 一致。
- 四个长期入口不再把 Issue `#59` / PR `#60` 描述为待合并任务。
- PR `#60` 的 Squash、单父、tree、签名、双套主干 CI、Artifact 和 Issue 关闭状态均可从长期文档复核。
- 下一合法工作只指向独立预算 CI 观察 Issue，且保持非 required、无 Ruleset 改动。
- 本地轻量验证通过，完整 Workspace 和三平台构建交给 GitHub-hosted CI。
- 非 Draft PR 完成 Review/CI 后，最终 Squash Merge 仍需项目所有者针对最终 Head 单独授权。
