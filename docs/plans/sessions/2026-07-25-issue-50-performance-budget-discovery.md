# Session Plan：Issue #50 性能预算 Discovery

schema_version: inputcodex.session-plan.v1
task_id: issue-50-performance-budget-discovery
task_summary: 冻结性能预算方法、失败语义和 Gate 5 解锁前置条件，并同步 PR #49 合并后的长期状态；不填写预算、不实施优化或 CI。
task_class: Standard
decision_status: approved-scope-discovery-implementation
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/50
baseline_ref: fd9db9ca1c150b7db34dda8acc09b6f0cc357a17
baseline_tree_ref: 3fc4a5a7697850f048edcedf6a9ec5e4f76c847c
branch_ref: codex/issue-50-performance-budget-discovery
session_plan_ref: docs/plans/sessions/2026-07-25-issue-50-performance-budget-discovery.md
task_plan_ref: docs/plans/2026-07-25-issue-50-performance-budget-discovery.md
runtime_workflow_ref: docs/workflows/2026-07-25-issue-50-performance-budget-discovery-runtime.md
report_ref: docs/reports/issue-50-performance-budget-discovery.md
adr_ref: docs/adr/0004-performance-budget-policy.md
approved_decision_ref: user-message:按A方案开始-2026-07-25
owner_scope_approval_ref: https://github.com/nonononull/inputcodex/issues/50#issuecomment-5080410894
scope_hash: sha256:af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f
planning_write_authority: exact-nine-path-set-approved
implementation_status: discovery-documentation-complete-local-green-pr-pending
mutation_intent: planning-and-documentation-only; no-budget-values; no-ci-or-product-mutation
executor_enforcement: exact-nine-path-set, owner-scope-approval-before-adr-or-long-term-state, no-subagents, local-light-validation, normal-push-only, squash-merge-only, no-force-push, review-root-cause-closure-required
time_source: Windows Get-Date for Git operations; GitHub timestamps remain service-side evidence
agos_status: bypassed-report-only-unregistered-needs-input-no-cross-repo-mutation

## 一、批准状态

- 项目所有者已批准方案 A，并授权创建 Issue、隔离分支、工作树和规划控制面。
- 九路径与 `scope_hash` 已由项目所有者通过 `owner_scope_approval_ref` 单独批准。
- 当前允许创建 ADR、Discovery 报告并同步四个长期入口，随后执行轻量验证、普通提交、普通推送、非 Draft PR 和 Review/CI。
- 当前授权不包含创建预算数值、修改 CI、实施优化、解锁 Gate 5 或最终 Squash Merge。

## 二、Fresh 事实

- 远端 `main`、本工作树起点和 PR `#49` Squash 提交均为 `fd9db9ca1c150b7db34dda8acc09b6f0cc357a17`。
- Squash tree 为 `3fc4a5a7697850f048edcedf6a9ec5e4f76c847c`，唯一 parent 为 `f81f457f615bed3d0f177aae52516824651abd12`。
- Issue `#32` 已按 `COMPLETED` 关闭，但其实际交付没有预算数值或预算批准。
- 合并后主 CI Run `30171903289` 七 Job 全绿、Artifact 为 `0`；Performance Run `30171903279` 在 Evidence 模式四 Job 全绿、Artifact 为 `0`。
- 最新正式 Release 仍为 `v1.2.42`，tag commit 为 `657cd33e009ad02515d30db6492cd4e669b06318`。
- Ruleset `19395456` active，只允许 Squash，要求解决 Review 对话；人类维护者为 `1`，required approvals 为 `0`。

## 三、local_knowledge_lookup

- 项目入口：`AGENTS.md`、`README.md`、`build.md`、`err.md`、Master Plan。
- 性能证据：Issue `#32` 任务计划、Session Plan、Runtime Workflow、报告、配置、两平台原始 JSON 和组合 manifest。
- 现有指标：桌面与隔离基准 Release 构建元数据、首次 view、空闲 Working Set、空闲 CPU，以及三个 Rust 场景。
- 已知风险：GitHub Runner 镜像漂移、外部 Actions 事故、CRLF 工作树转换、微场景精度截断、IQR 标记样本和单次构建噪声。
- 既有决策：ADR `0001` 至 `0003` 没有性能预算合同；Issue `#24` 只冻结“先固定协议，再由独立执行 Issue 产生预算”。
- 外部 AGOS 只作为可选只读参考；没有发现可直接替代项目原生控制面的性能预算 workflow，禁止修改外部规则或 Vault。
- 默认入口 ReportOnly 的真实输出为 `DEFAULT_ENTRY_ROUTE_STATUS=needs-input`、`DEFAULT_ENTRY_TASK_REGISTRATION_STATUS=unregistered`、`DEFAULT_ENTRY_OWNER_DIRECT_WRITE_ADMISSION_STATUS=blocked`；项目入口与 Git 基础均为 `ready`。按 `AGENTS.md` 记录后绕过，不创建 AGOS task-backlog 记录，不修改任何跨仓控制面。

## 四、候选批准写集合

```text
AGENTS.md
README.md
build.md
docs/adr/0004-performance-budget-policy.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/2026-07-25-issue-50-performance-budget-discovery.md
docs/plans/sessions/2026-07-25-issue-50-performance-budget-discovery.md
docs/reports/issue-50-performance-budget-discovery.md
docs/workflows/2026-07-25-issue-50-performance-budget-discovery-runtime.md
```

哈希算法固定为 Windows `Sort-Object` 默认大小写不敏感排序，UTF-8 无 BOM，LF 连接并保留末尾 LF，再计算 SHA-256；结果为：

`sha256:af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f`

## 五、预算 Discovery 必须裁决的问题

1. 哪些环境字段构成“可比较”硬键，哪些变化只触发重新观察，哪些变化直接使 Run `not-comparable`。
2. 每个平台至少需要多少次独立 hosted Run，单个 Run 内继续使用哪些样本合同。
3. 首次 view、空闲资源、Rust 场景和构建元数据分别采用何种统计与漂移判定；单次构建是否只能观察。
4. IQR 标记如何进入统计，何时保留但不阻断，何时整次 Run 无效；禁止人工删除样本。
5. 观察、候选阻断、强制阻断三个阶段如何升级、降级和回滚。
6. 真实产品回归、证据合同错误、环境漂移和 GitHub 外部事故分别如何编码、复测和分流。
7. 预算数值由哪个后续 Issue 产生、需要何种所有者批准证据，CI 实施何时可以进入 `main` Ruleset。
8. Gate 5 解锁是否要求预算数值已批准、预算 CI 已稳定，或仅要求预算观察期已完成。

## 六、allowed_operations

规划检查点已完成：

```text
github.issue.create: Issue #50
git.branch_and_worktree.create: codex/issue-50-performance-budget-discovery
docs.write: 本任务计划、Session Plan、Runtime Workflow
validation.read_only: GitHub/API、现有性能证据、规则、维护者和仓库状态
github.issue.comment: 回写九路径、scope_hash 和批准请求
```

当前已批准：

```text
docs.write: 精确九路径
validation.local: Evidence、CI 合同、Repository Policy、scope_hash、git diff --check
git.commit_and_push: 普通提交与普通 push
github.pr.create: 关联 Issue #50 的非 Draft PR
github.review_and_ci: 根因闭环、适用 CI 和最终所有者合并决策
```

未列出的 mutation 默认禁止。

## 七、执行批次

### Batch 0：启动基线

- 创建 Issue `#50`。
- 从最新远端 `main` 创建隔离分支和工作树。
- Fresh 核对主干、上游 Release、Ruleset、维护者、性能证据和开放监控 Issue。

### Batch 1：规划检查点

- 创建三份规划控制面。
- 计算九路径 `scope_hash`。
- AGOS 默认入口 ReportOnly 已返回 `unregistered`、`needs-input` 和 owner direct write blocked；真实输出已记录并按项目原生流程绕过，不修改 AGOS。
- 回写 Issue `#50` 并等待项目所有者批准九路径。

### Batch 2：Discovery 实施

- 只在范围批准后创建 ADR 和报告并同步长期入口。
- 不新增预算数字、代码、配置或 Workflow。

### Batch 3：验证与 PR

- 运行项目定义的轻量验证。
- 创建非 Draft PR，处理全部 Review 对话和适用 CI。
- 停在项目所有者针对最终 Head 的单独 Squash Merge 授权前。

## 八、停止条件

- 项目所有者未批准九路径与 `scope_hash`。
- 需要触及九路径外文件或修改代码、基准、Workflow、Ruleset、Release、上游快照或 AGOS。
- 需要填写预算数值、运行新 hosted 测量、实施优化或迁移功能。
- 最新正式 Release、Ruleset、维护者数量或性能证据发生物质变化。
- 本地验证或 CI 失败且根因尚未闭环。
