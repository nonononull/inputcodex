# Issue #61：性能预算数值合并后稳定状态 Closeout 报告

## 结论

Issue `#59` / PR `#60` 已完成四次固定串行复测、预算数值生成与离线复算，并进入 `main`。项目现在拥有 Windows/macOS 分平台的 `approved-observation` 预算证据；预算 CI、required check、Ruleset、性能优化和 Gate 5 仍未获得授权。

本 Closeout 只把已经发生的来源事实固化为稳定状态，不实施新的性能工作。

## 来源合并证据

- 来源 Issue：`https://github.com/nonononull/inputcodex/issues/59`，状态为 `CLOSED / COMPLETED`。
- 来源 PR：`https://github.com/nonononull/inputcodex/pull/60`。
- Final Head：`61c088d74d61a329fbe67e14b8280dfa9701c6b2`。
- Squash 提交：`e225144831a0928bfa3aaa0d169a054779005812`。
- 父提交：`d9d1ed77b9796ac6a99e250d1547217a39426aa9`，父提交数量为 `1`。
- tree：`56eb1e8d95dfce22726c1aef1bdde1c353af055e`。
- GitHub 签名：`valid`。
- GitHub 服务端合并时间：`2026-07-26T08:35:25Z`；Issue 服务端关闭时间：`2026-07-26T08:35:26Z`。

## CI 与 Review 证据

- PR Final Head CI Run `30194465259`：七 Job 全部成功，Artifact 数为 `0`。
- PR Final Head Performance Baseline Run `30194465231`：四 Job 全部成功，Artifact 数为 `0`。
- 合并后主干 CI Run `30194897171`：七 Job 全部成功，Artifact 数为 `0`。
- 合并后主干 Performance Baseline Run `30194897166`：四 Job 全部成功，Artifact 数为 `0`。
- 合并前 review、inline comment、review thread 与未解决 thread 均为 `0`。
- 项目所有者授权绑定 Final Head，合并使用正常 Squash Merge，没有 Force Push、删除 `main` 或删除来源分支。

## 稳定预算证据

- Windows 严格目标环境指纹为 `sha256:f3954543f3cec519568345d9f40341ddeb8991a7d93b3a274cc324b047fb00cb`，历史 `3` 份加 Issue `#59` 新增 `2` 份，共 `5` 份可比 Run。
- macOS 同队列共有 `12` 份可比 Run。
- 四个固定新槽位全部结束，不存在 `run-05`。
- `benchmarks/budgets/issue-59-approved-observation.json` 归一化 SHA-256 为 `sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5`。
- warning/blocking 公式、MAD、安全裕量与量子舍入均由离线验证器独立复算，合同为 `BUDGET_APPROVAL_GREEN passed=10`。
- `budget_ci_enabled=false`、`gate_5_unlocked=false`，这些值不能由 Closeout 改写。

## Closeout 本地验证

- 实际差异精确为八路径，`scope_hash=sha256:dafe55bfc38c38782558c1577215d227ac8c83b7110735c4ddd58b48d66264b5`。
- Performance Evidence：`ok=true`、`violation_count=0`。
- 预算离线复算：`BUDGET_APPROVAL_GREEN passed=10`。
- CI 合同：`CI_CONTRACT_GREEN passed=34`。
- Repository Policy：`ok=true`、`violation_count=0`。
- `git diff --check` 通过；未运行完整 Workspace、桌面 Release 或真实性能采集。

## 下一合法工作

下一项工作只能是独立预算 CI 观察 Issue：

1. 只消费已经进入 `main` 的 `approved-observation` JSON，不修改数值、公式、量子、队列或来源 Run。
2. 先以非 required 观察模式接入 Windows/macOS，不修改 Ruleset 或仓库级 required checks。
3. 两个平台均成功执行并保留可诊断证据，成功 Run Artifact 继续为 `0`。
4. `release_audit=current` 后，再由项目所有者决定是否建立首个 Gate 5 功能迁移 Issue。

预算 CI 实施、性能优化和 Gate 5 产品迁移必须继续使用不同 Issue/PR。

## 反递归边界

本报告可以记录 Issue `#61` 是产生此稳定状态的 Closeout，但不记录其未来 PR Head、CI、Review、授权或 Squash 状态。这些动态证据只保留在 GitHub Issue/PR 评论中，因此本 Closeout 合并后不需要再创建同类 Closeout。

## 外部治理探测

本任务已对 AGOS 运行 `ReportOnly`。返回状态为 `unregistered`、`needs-input` 与所有者直接写入阻断；项目 Git、入口文档和本地知识查询均为 ready。根据项目规则，AGOS 只能作为外部辅助，未登记状态不得阻塞 Issue、PR、Review、验证或合并，因此已记录原因并继续使用 `inputcodex` 原生控制面；本任务没有修改 AGOS。

## 未发生的变更

- 没有修改预算数值、公式、量子、队列、样本或 JSON 哈希。
- 没有运行新的 hosted 性能测量。
- 没有修改 `benchmarks/`、Rust、Cargo、产品功能、Workflow、Ruleset、Release、`upstream/` 或 AGOS。
- 没有实施性能优化，没有迁移业务功能，没有解锁 Gate 5。
- 没有 Force Push、删除 `main`、来源分支或工作树。
