# Issue #52：性能预算 Discovery 合并后稳定状态 Closeout 报告

## 结论

Issue `#50` / PR `#51` 已完成性能预算 Discovery 并进入 `main`。项目现在拥有可审计的预算方法、可比队列、样本数量、统计语义和阶段升级合同，但没有任何已批准的 warning/blocking 数值；Gate 5 继续锁定。

本 Closeout 只把已经发生的来源事实固化为稳定状态，不实施新的性能工作。

## 来源合并证据

- 来源 Issue：`https://github.com/nonononull/inputcodex/issues/50`，状态为 `CLOSED / COMPLETED`。
- 来源 PR：`https://github.com/nonononull/inputcodex/pull/51`。
- Final Head：`e0154c61d8b05835db10437c79f029909516eac1`。
- Squash 提交：`fea8824c652665df710a7e6ef941854060eb6e1f`。
- tree：`9fb518cda8b35a9388fb9fce0a1ff6ba976d80cb`。
- 提交结构：单父。
- GitHub 签名：`valid`。
- GitHub 服务端合并时间：`2026-07-25T21:25:03Z`。

## CI 与 Review 证据

- PR Final Head CI Run `30174131581`：七个预期 Job 全部终态，`classify`、`governance`、`release-audit`、`required` 成功，Linux/Windows/macOS 重型 Job 依据纯文档分类合同跳过；Artifact 数为 `0`。
- 合并后主干 CI Run `30175592979`：`classify`、`governance`、`release-audit`、Linux、Windows、macOS、`required` 七 Job 全部成功；Artifact 数为 `0`。
- 合并前 Review 对话总数与未解决数均为 `0`。
- 项目所有者授权已绑定 Final Head，合并使用正常 Squash Merge，没有 Force Push、删除 `main` 或删除来源分支。

## 稳定产品语义

- Windows 与 macOS 分别维护预算，不跨平台排名或共用阈值。
- 每个平台、每个可比队列至少需要五次独立 GitHub-hosted Run。
- Issue `#32` 的单次种子证据不能直接生成预算。
- 当前阶段是 `baseline-only`；预算数值必须由独立 Issue/PR 批准。
- 预算 CI 必须先以 `approved-observation` 进入 `main` 并完成双平台观察，之后才可能解锁 Gate 5 首个功能 Issue。

## 下一合法工作

下一项工作只能是独立性能复测与数值批准 Issue：

1. 按平台和可比队列收集至少五次独立 Run。
2. 保存 run-level median/P95、跨 Run 中位数、min/max/MAD 和全部 IQR 标记样本。
3. 由项目所有者批准 warning limit、blocking limit、单位、队列指纹、来源 Run 和安全裕量。

预算 CI 实施、性能优化和 Gate 5 产品迁移必须继续使用不同 Issue/PR。

## 反递归边界

本报告可以记录 Issue `#52` 是产生此稳定状态的 Closeout，但不记录其未来 PR Head、CI、Review、授权或 Squash 状态。这些动态证据只保留在 GitHub Issue/PR 评论中，因此本 Closeout 合并后不需要再创建同类 Closeout。

## 外部治理探测

本任务已对 AGOS 运行 `ReportOnly`。返回状态为 `unregistered`、`needs-input` 与所有者直接写入阻断；项目 Git 基础、入口文档和本地知识查询均为 ready。根据项目规则，AGOS 只能作为外部辅助，未登记状态不得阻塞 Issue、PR、Review、验证或合并，因此已记录原因并继续使用 `inputcodex` 原生控制面；本任务没有修改 AGOS。

## 未发生的变更

- 没有填写或批准预算数值。
- 没有运行新的 hosted 性能测量。
- 没有修改 `benchmarks/`、Rust、Cargo、产品功能、Workflow、Ruleset、Release、`upstream/` 或 AGOS。
- 没有实施性能优化，没有迁移业务功能，没有解锁 Gate 5。
- 没有 Force Push、删除 `main`、来源分支或工作树。
