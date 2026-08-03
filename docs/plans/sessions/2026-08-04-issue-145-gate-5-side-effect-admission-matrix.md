# Issue #145 Session Plan：Gate 5 副作用准入矩阵

## Session Metadata

- task_id: issue-145-gate-5-side-effect-admission-matrix
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/145
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5168939607
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/145#issuecomment-5169299292
- session_plan_ref: docs/plans/sessions/2026-08-04-issue-145-gate-5-side-effect-admission-matrix.md
- baseline_ref: main@42c73f401e7a758cdc5eca374613625dad46340b
- branch_ref: codex/issue-145-gate-5-side-effect-admission-matrix
- scope_count: 20
- scope_hash: sha256:b7955bb33dac2a5f58990dfbe2aff22cc6145a2b60e601e6de255bc0a8f4360f
- allowed_operations: task-local docs, autonomous policy/state/validator TDD, Parity admission schema/matrix/validation TDD, local verification, normal commit/push, non-Draft PR, read-only review, exact Squash
- mutation_intent: governance-discovery-only
- executor_enforcement: single-writer / exact-scope / implementation-authorized-false

## Approved Decision

本批只建立“准入事实层”：当前 83 个 unassessed source 全部保持 blocked 且未获实现授权。完成矩阵后必须重新找 owner，不能从 owner 状态、能力桶或 blocker 数量自动推出产品候选。

## Source Buckets

- write：16 feature / 70 source
- process：2 feature / 5 source
- network：4 feature / 8 source

Feature 级聚合优先级固定为 `write > process > network`，由 source-index 的完整 side-effect 集合动态复算。

## Verification Strategy

- PowerShell：真实 policy/state helper 变异，拒绝 consumed/matrix 字段的类型、值、大小写、集合和额外字段漂移。
- Rust：严格反序列化、83-source 精确集合、feature 映射、桶优先级、owner/blocker/admission/authorization 与完整仓库接线。
- Repository：保持现有 135/46/46/12/11/3/0 目录计数，Release Audit current，产品/Cargo/Workflow/upstream 零 diff。
- Delivery：双独立 Final Head 复审、PR/main 双 Workflow、Artifact 0、Review thread 0、单父/tree/signature。

## External Controls

- `superpowers:*` 当前会话未暴露，使用 `karpathy-guidelines`、`domain-modeling` 与项目原生 TDD。
- `.codegraph` 不存在，未初始化。
- AGOS 仅执行一次 default-entry `ReportOnly`；needs-input 或异常按项目规则绕过，禁止修改 AGOS。
- 两个 bounded reviewer 仅提供只读设计复核，不拥有 scope 或写入权限。

## Execution Checkpoint

- control_plane_red: `75 pass / 7 fail`
- control_plane_green: `CI_CONTRACT_GREEN passed=84`
- parity_red: admission 公共 API 与 9 个 ValidationCode 缺失
- parity_green: `admission_matrix 5/5`、`catalog_repository 33/33`、all-targets 与 Clippy 通过
- product_delta: `0`
- next_node: 二十路径本地门禁与独立 Final Head 复审
