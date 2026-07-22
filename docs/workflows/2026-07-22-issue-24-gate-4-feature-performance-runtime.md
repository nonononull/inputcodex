# Runtime Workflow：Issue #24 Gate 4 功能目录与性能基线规划

workflow_status: pr-open-review-ci-pending
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/24
branch_ref: codex/issue-24-gate-4-planning
approved_decision_ref: user-message:approve-gate-4-option-2-planning-2026-07-22
scope_hash: sha256:72e2f5d774080a55599297909600aba3c9f58470710b71db25d3690a61a1cbf0
baseline_ref: f470c062037042a1f7833a29cdcf216f6c0f5601
pr_ref: https://github.com/nonononull/inputcodex/pull/25

## Phase 0：启动基线

1. 确认当前权威 `main` 为 `f470c062037042a1f7833a29cdcf216f6c0f5601`，工作树干净。
2. 核对 PR `#23` 为 MERGED、Issue `#22` 为 `CLOSED / COMPLETED`、主干运行 `29922385227` 六 Job 成功且 Artifact 数为 `0`。
3. 核对最新正式 Release 仍为 `v1.2.41`，tag 提交仍为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
4. 记录上游 `main` `91376ee3518cb5fe5ec8eead179418f706c25870` 为预警输入，不修改 Issue `#20` 或缓存快照。
5. 核对 Ruleset `19395456` active、无 bypass、审批 `0`、必须解决 Review 对话、Squash-only；唯一人类维护者仍为 `nonononull`。

## Phase 1：Issue 与授权冻结

1. 创建 Issue `#24`，记录三种方案、采用方案 2、Fresh 基线、最大写集合、禁止范围和停止条件。
2. 创建 `codex/issue-24-gate-4-planning`，禁止直接写 `main`。
3. 记录项目所有者批准 `user-message:approve-gate-4-option-2-planning-2026-07-22`。
4. 以路径升序、LF 分隔和末尾 LF 计算范围哈希 `sha256:72e2f5d774080a55599297909600aba3c9f58470710b71db25d3690a61a1cbf0`。
5. 明确本授权不包含 PR 合并或 Gate 4 执行。

## Phase 2：规划合同落盘

1. 创建任务计划、Session Plan、Runtime Workflow 和初始报告。
2. 更新 AGENTS、README、build、Master Plan，清除 Gate 3 closeout 的陈旧活动状态。
3. 只有真实纠错时更新 `err.md`；本任务记录 Windows 沙箱启动器与 apply-patch 包装器问题的根因和验证证据。
4. 功能矩阵只冻结稳定 ID、五分域、行为字段、八种既有状态和决策引用。
5. 行为合同只冻结输入、输出、数据、副作用、错误、加载、超时、取消、隔离、可观测与双平台语义。
6. 夹具只冻结合成/不可逆脱敏、结构保真和敏感信息禁区。
7. 性能协议冻结比较对象、可比性、样本数量、统计方法、资源窗口和预算批准门；不得填写绝对预算。

## Phase 3：执行 Issue 拆分

1. 定义功能目录执行 Issue：允许 `parity/` 与必要 parity 验证代码，不迁移产品功能。
2. 定义性能基线执行 Issue：允许 `benchmarks/`、测量脚本和结果报告，不顺带优化产品。
3. 两个执行 Issue 都必须重新计算范围哈希、建立 Session Plan/Runtime Workflow 并取得项目所有者批准。
4. 争议项必须另建 `parity-exception`；半成品参考必须先固定提交、许可证和可复现构建方式。

## Phase 4：本地验证

1. 运行 `build.md` 的 Issue `#24` 验证，要求路径恰好为 9 条。
2. 确认 Cargo、apps、crates、scripts/ci、Workflow、upstream、parity、benchmarks、Ruleset 和 AGOS 零差异。
3. 运行治理合同，必须输出 `CI_CONTRACT_GREEN passed=30`。
4. 运行真实仓库政策，必须输出 `ok=true`、`violation_count=0`。
5. 运行 `git diff --check`，确认工作树无空白错误。
6. 自审计划：无未决占位标记、无平行状态机、无未经实测预算、无执行授权泄漏。

## Phase 5：提交、推送与 PR

1. 创建普通提交；推送前核对分支、Head、范围与工作树。
2. 普通推送分支，禁止 Force Push。
3. 创建非 Draft PR，正文包含 `Closes #24`、两阶段设计、9 条路径、本地验证和“未授权合并/执行”。
4. PR 创建后只通过后续普通提交回写 PR URL、Review 和 CI 引用，不重写已推送历史。

## Phase 6：Review/CI 停止点

1. Fresh 核对最终 Head、文件列表、自动合并、Ruleset、维护者数量和 Review 对话。
2. 文档-only 分类时，`classify`、`governance`、`required` 必须成功；Linux/Windows/macOS 可按合同跳过。
3. 每条 Review 对话必须记录根因、处理方式和验证证据；不存在 Review 对话时记录数量 `0`。
4. 回写 Issue/PR checkpoint 后停止，等待项目所有者新的 Squash Merge 授权。
5. 未获得授权不得执行 `gh pr merge`，不得提前创建 Gate 4 执行 Issue。

## 停止条件

- 最新正式 Release 不再是 `v1.2.41` 或缓存 tag/许可证证据变化。
- 需要修改 9 条批准路径外文件，或实际创建 `parity/`、`benchmarks/`、产品、CI、upstream、Ruleset、Release、AGOS。
- 需要运行上游/半成品、决定一致性例外或填写性能预算。
- Head、Ruleset、维护者数量、Review/CI 或授权发生物质漂移。
