# Issue #24：Gate 4 功能目录与性能基线规划报告

report_status: candidate-review-ci-green-final-seal-pending
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/24
branch_ref: codex/issue-24-gate-4-planning
pr_ref: https://github.com/nonononull/inputcodex/pull/25
review_ref: github-pr-25-review-comments-0
ci_ref: https://github.com/nonononull/inputcodex/actions/runs/29925622836
merge_ref: not-authorized
approved_decision_ref: user-message:approve-gate-4-option-2-planning-2026-07-22
scope_hash: sha256:72e2f5d774080a55599297909600aba3c9f58470710b71db25d3690a61a1cbf0

## 一、当前结论

- 项目所有者已批准方案 2：先冻结 Gate 4 规划合同，再分别创建功能目录与性能基线执行 Issue。
- Issue `#24` 已创建，分支为 `codex/issue-24-gate-4-planning`。
- 当前授权覆盖规划 PR 创建，不覆盖合并、Gate 4 执行或一致性例外决定。
- 本报告只记录规划交付证据，不宣称功能矩阵、合同夹具、性能基线或预算已经完成。

## 二、Fresh 基线

- Gate 3 closeout 权威 `main`：`f470c062037042a1f7833a29cdcf216f6c0f5601`。
- PR `#23` 已 Squash Merge，Issue `#22` 为 `CLOSED / COMPLETED`。
- 合并后主干运行 `29922385227` 六 Job 全绿，成功 Artifact 数为 `0`。
- 最新正式 Release 仍为 `v1.2.41`，缓存 tag 提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 上游 `main` `91376ee3518cb5fe5ec8eead179418f706c25870` 只作为 Issue `#20` 预警，不进入本任务运行面。

## 三、冻结的规划合同

- 功能 ID：`feature.<domain>.<slug>`；五个 domain 对齐 Gate 5 分域。
- 一致性状态：只复用 `unassessed`、`planned`、`implementing`、`implemented`、`verified`、`exception-pending`、`exception-approved`、`retired`。
- 行为合同覆盖输入、输出、数据格式、持久化、副作用、错误、加载、超时、取消、错误隔离、可观测和双平台语义。
- 夹具只允许合成或不可逆脱敏数据，禁止真实凭据、会话、设备标识、私人路径和签名材料。
- 性能协议要求相同场景/夹具/可比环境，保存原始样本；规划阶段不填写绝对预算。

## 四、执行拆分

1. **功能目录执行 Issue**：创建 `parity/features/`、`parity/contracts/`、`parity/fixtures/` 和必要验证器，不迁移产品功能。
2. **性能基线执行 Issue**：创建 `benchmarks/`、测量脚本、结果和预算提案，不顺带优化产品。

两个执行 Issue 均需新的 Session Plan、Runtime Workflow、范围哈希和项目所有者批准；争议项继续分流到独立 `parity-exception`。

## 五、路径与保护面

- 最大写集合为 9 条治理/文档路径，哈希 `sha256:72e2f5d774080a55599297909600aba3c9f58470710b71db25d3690a61a1cbf0`。
- Cargo、产品源码、测试、CI、upstream、parity、benchmarks、Ruleset、Release 和 AGOS 必须保持零差异。
- Issue `#16/#20` 不得手工修改。

## 六、排错证据

- Codex Windows 沙箱偶发在命令执行前返回 `CreateProcessAsUserW failed: 5`。
- 临时 `apply_patch.bat` 指向受限 WindowsApps 可执行文件；改用 npm Codex CLI 的同一 apply-patch 模式，并以 UTF-8 参数传递补丁。
- 未修改 Git 配置、远端、Ruleset、系统 ACL 或项目脚本；详细记录见 `err.md`。

## 七、待完成

- 提交候选 Review/CI checkpoint，等待新的最终 Head CI。
- 在 Review/CI 完成后停止，等待项目所有者新的 Squash Merge 授权。

## 八、本地验证结果

- 分支：`codex/issue-24-gate-4-planning`。
- 变更路径：`9/9`，越界、缺失和受保护路径均为 `0`。
- 规划自审：占位标记 `0`，五分域、八种一致性状态、合同字段和性能采样协议覆盖完整。
- 治理合同：`CI_CONTRACT_GREEN passed=30`。
- 真实仓库政策：`ok=true`、`violation_count=0`。
- `git diff --check`：通过。

## 九、交付 checkpoint

- 首个规划提交：`8ada763da8b8205866b2f4a4e1e30eeaacd7e409`。
- 已普通推送 `codex/issue-24-gate-4-planning`，未使用 Force Push。
- 非 Draft PR：`https://github.com/nonononull/inputcodex/pull/25`。
- PR 正文包含 `Closes #24`、9 条范围、验证证据和“未授权合并/执行”边界。

## 十、PR #25 候选 Review/CI checkpoint

- 候选 Head：`8b2c9688f3e2d533804c80dd0d7dd290b7e18c90`。
- 运行 `29925622836` 为 `success`；`classify`、`governance`、`required` 成功，Linux/Windows/macOS 因文档-only 分类按合同跳过。
- 成功 Artifact 数为 `0`。
- PR 为 OPEN、非 Draft、Merge State `CLEAN`、自动合并关闭。
- Review 对话/行评论数为 `0`。
- Ruleset `19395456` active、无 bypass、审批 `0`、必须解决 Review 对话、Squash-only；具备合并权限的人类维护者仍只有 `nonononull`。
- 当前授权仍不包含合并；最终封口提交后必须等待新的 Head CI，不能把本运行替代最终 Head 验证。
