# Session Plan：Issue #24 Gate 4 功能目录与性能基线规划

schema_version: inputcodex.session-plan.v1
session_status: local-verified-pr-creation-pending
task_id: issue-24-gate-4-feature-performance-plan
task_summary: 采用两阶段拆分冻结 Gate 4 功能矩阵、行为合同、脱敏夹具、性能测量协议和后续执行 Issue 边界；本任务不创建实际目录、基准或产品实现。
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/24
branch_ref: codex/issue-24-gate-4-planning
session_plan_ref: docs/plans/sessions/2026-07-22-issue-24-gate-4-feature-performance-plan.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-24-gate-4-feature-performance-runtime.md
task_plan_ref: docs/plans/2026-07-22-issue-24-gate-4-feature-performance-plan.md
report_ref: docs/reports/issue-24-gate-4-feature-performance-plan.md
approved_decision_ref: user-message:approve-gate-4-option-2-planning-2026-07-22
scope_hash: sha256:72e2f5d774080a55599297909600aba3c9f58470710b71db25d3690a61a1cbf0
mutation_intent: planning-control-plane-only
executor_enforcement: 每个写入批次前后核对分支、9 条批准路径、受保护表面和 Git 状态；PR 创建后只允许回写 PR/Review/CI 动态引用，任何产品或执行需求立即停止。

## 一、批准状态

- 项目所有者已批准方案 2：“规划合同先行，规划合并后拆分功能目录执行 Issue 与性能基线执行 Issue”。
- 当前授权允许 Issue、分支、文档、验证、普通提交、推送和规划 PR 创建。
- 当前授权不允许规划 PR 合并、Gate 4 执行、`parity-exception` 决策或 Gate 5 工作。
- AGOS 在本任务中绕过：项目原生控制面输入完整，且本 Issue 禁止修改或优化外部 AGOS。

## 二、Fresh 事实

- 启动基线 `main`：`f470c062037042a1f7833a29cdcf216f6c0f5601`。
- Gate 3 closeout：PR `#23` 已合并，Issue `#22` 为 `CLOSED / COMPLETED`，主干运行 `29922385227` 六 Job 全绿。
- 最新正式 Release：`v1.2.41`；缓存 tag 提交 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 上游 `main`：`91376ee3518cb5fe5ec8eead179418f706c25870`，只作为预警；Issue `#20` 保持 OPEN。
- 仓库只有一名具备合并权限的人类维护者 `nonononull`；required approvals 继续为 `0`，但合并必须有所有者决策证据。

## 三、allowed_operations

```text
github.issue.create: Issue #24
git.branch.create: codex/issue-24-gate-4-planning
docs.write: 9 条批准路径
validation.local: Issue #24 build.md 验证、治理合同、仓库政策、git diff --check
git.commit: 普通提交，禁止 amend 覆盖已推送证据
git.push: 普通 push，禁止 force/force-with-lease
github.pr.create: 关联 Issue #24 的非 Draft 规划 PR
github.issue_or_pr.comment: 回写验证、Review、CI 与所有者决策证据
```

未列出的 mutation 默认禁止，特别是 merge、Ruleset、upstream、Cargo、产品、CI、`parity/`、`benchmarks/` 和 AGOS。

## 四、批准写集合

```text
AGENTS.md
README.md
build.md
err.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/2026-07-22-issue-24-gate-4-feature-performance-plan.md
docs/plans/sessions/2026-07-22-issue-24-gate-4-feature-performance-plan.md
docs/workflows/2026-07-22-issue-24-gate-4-feature-performance-runtime.md
docs/reports/issue-24-gate-4-feature-performance-plan.md
```

`err.md` 记录本任务真实遇到的 Windows 沙箱启动器拒绝与 apply-patch 包装器问题；禁止添加无真实复现的模板化错误。

## 五、设计决定

1. 功能矩阵沿用架构已批准的八种一致性状态，不创造平行状态机。
2. 功能标识按五个 Gate 5 分域生成稳定 ID，UI 重设计和文件移动不能改变 ID。
3. 行为合同必须覆盖数据、副作用、错误、加载、超时、取消、隔离、可观测和双平台语义。
4. 夹具只能使用合成或不可逆脱敏数据，同时保留结构与边界值。
5. 性能协议先固定可比对象、环境、样本和统计，再由独立执行 Issue 产生预算；规划阶段不写预算数字。
6. 功能目录和性能基线使用两个独立执行 Issue；基线与后续优化也不能混在同一 PR。

## 六、执行批次

### Batch 0：startup baseline

- 验证 `main`、PR `#23`、Issue `#22`、运行 `29922385227`、Release `v1.2.41`、上游 main、Ruleset 和维护者数量。
- 创建 Issue `#24` 与独立分支。

### Batch 1：control-plane authoring

- 创建任务计划、Session Plan、Runtime Workflow 和初始报告。
- 更新 `AGENTS.md`、README、build 和 Master Plan；只因真实工具链错误更新 `err.md`。
- 冻结 `approved_decision_ref`、`scope_hash`、`allowed_operations`、`mutation_intent` 与 `executor_enforcement`。

### Batch 2：local verification

- 验证 9/9 路径、受保护表面零差异和陈旧 Gate 3 状态清除。
- 运行 `scripts/ci/Test-CiScripts.ps1`，期望 `CI_CONTRACT_GREEN passed=30`。
- 运行 `scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .`，期望 `ok=true`、`violation_count=0`。
- 运行 `git diff --check`，期望无输出且退出码为 `0`。

### Batch 3：delivery

- 创建普通提交并推送；禁止 amend、Force Push 或删除 `main`。
- 创建关联 Issue `#24` 的非 Draft PR，正文包含 `Closes #24`、范围、验证和不含合并授权的边界。
- 回写 PR URL 后重新验证最终 Head。

### Batch 4：review checkpoint

- 文档-only PR 的 `classify`、`governance`、`required` 必须成功；三个重型平台 Job 可按合同跳过。
- Review 对话必须完成根因、处理与验证闭环。
- 自动合并保持关闭；没有项目所有者新的 Squash Merge 授权时停止。

## 七、验证命令

```powershell
Set-Location 'C:\Users\dashuai\Documents\inputcodex'
git branch --show-current
git -c core.quotePath=false diff --name-only f470c062037042a1f7833a29cdcf216f6c0f5601
& .\scripts\ci\Test-CiScripts.ps1
& .\scripts\ci\Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
git status --short --branch
```

完整路径和内容断言使用 `build.md` 的 `Issue #24 Gate 4 规划本地验证`。

## 八、完成定义

- Issue、分支、四份任务控制面和原生状态文件均已建立。
- 9 条路径无越界，产品、CI、upstream、Ruleset、`parity/`、`benchmarks/` 和 AGOS 零差异。
- 规划内容没有未决占位标记、未定义状态或无证据预算。
- 本地验证与 PR 适用 CI 通过，Review 对话全部闭环。
- PR 处于等待项目所有者合并决策状态，不自动合并。

## 九、停止条件

- Release/tag、许可证、Ruleset、维护者数量或批准范围发生物质变化。
- 需要运行参考产品、创建实际矩阵/合同/基准、修改产品或填写预算。
- 需要处理上游同步或争议功能，但没有独立 Issue。
- Head 漂移、适用 CI 失败或 Review 根因未解决。
