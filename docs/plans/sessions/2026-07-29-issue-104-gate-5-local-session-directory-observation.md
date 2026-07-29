# Issue #104 Session Plan：本地会话目录只读观察

## Session Metadata

- `task_id`: `issue-104-gate-5-local-session-directory-observation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/104`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/103#issuecomment-5121451656`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/104#issuecomment-5121508761`
- `decision_status`: `approved`
- `selected_business_path`: `gate-5/local-session-directory-observation`
- `baseline_ref`: `origin/main@4032b051f0f18be71d344eded2d6e79595233b65`
- `branch_ref`: `codex/issue-104-gate-5-local-session-directory-observation`
- `worktree_ref`: `.worktrees/issue-104-gate-5-local-session-directory-observation`
- `work_class`: `standard`
- `delivery_contract`: `issue-pr-squash-merge`
- `planning_scope_hash`: `sha256:8b9295414fde23c7ea9c9c53a47acfb7a70b1f1bcd3255f81cb011ba5b624a51`
- `candidate_scope_hash`: `sha256:47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d`
- `normal_scope_without_err`: `sha256:acee55e9539631f6eca4fb557d27999c7815d20ae15a4b4ea932db243079cb8c`
- `planning_validation`: `PASSED`
- `execution_state`: `PLANNING_VERIFIED`
- `planning_checkpoint_ref`: `pending`
- `domain_checkpoint_ref`: `pending`
- `application_checkpoint_ref`: `pending`
- `platform_checkpoint_ref`: `pending`
- `parity_checkpoint_ref`: `pending`
- `local_verified_checkpoint_ref`: `pending`
- `agos_report_only`: `blocked-unregistered-missing-owner-scope-manifest-bypassed`

## Approved Decision

项目所有者依次选择候选 A、批准推荐 A1、授权执行者在重构边界内自主收敛，并明确下达“开始吧”。集中授权证据已写入 Issue #103 与 #104。

批准内容：

- 只接管 `tauri-command:list_local_sessions` 的安全只读事实。
- 每条会话只返回 ID、有界标题、标题截断、归档状态和更新时间。
- 支持受控多数据库排序、去重、分页和来源覆盖状态。
- 使用严格只读 SQLite；删除、备份、恢复、修复和 grouped undo 继续未评估。
- 实施细节无需逐项请示；越界语义和最终 Squash Merge 必须重新请求项目所有者授权。

## Mutation Intent

- `mutation_intent`: `source`
- `mutation_target`: `inputcodex Issue #104 精确二十九路径`
- `requested_operations`: `plan, source-edit, test, commit, push, pr, review-response`
- `forbidden_operations`: `main-write, force-push, merge-without-final-head-approval, scope-expansion, AGOS-edit`
- `allowed_operations`: 仅在冻结路径内修改项目原生控制面、Domain、Application、Platform、Parity、锁文件和合成 fixture。

## Executor Enforcement

- 当前阶段：`planning-freeze`。
- 产品源代码 TDD 仅在 planning 七路径、hash、AGOS report-only 和 planning checkpoint 完成后开始。
- 所有生产实现遵循 `superpowers:test-driven-development`：先写测试、亲眼观察预期 RED、再写最小 GREEN。
- 错误或异常先查 `err.md`，重复根因只引用；新可复用根因才修改 `err.md`。
- 不派发 subagent：用户未明确授权代理委派；独立核验由本地主线程、新鲜 GitHub CI 和 Review 证据承担。
- 不创建第二套活跃 `docs/superpowers/*` 控制面。
- 每个执行批次后运行 Git snapshot checkpoint 并建立命名 Git 提交。

## Local Knowledge Lookup

已完成的本地查询：

- 项目：`AGENTS.md`、`README.md`、`build.md`、`err.md`、`CONTEXT.md`、Master Plan。
- 近期事实：Issue #100/#101 文档与实现模式、PR #102 合并后主干证据。
- 上游：`codex_sqlite.rs`、`storage.rs`、`commands.rs::list_local_sessions` 和相关测试。
- Parity：session-data feature、contract、source-index 与原管理 fixture。
- 架构：七成员依赖方向、Platform observation 模式、LoadCompletion/LoadCoordinator。
- AGOS：auto-application、brainstorming gate、runtime workflow、协议观察治理、代码质量与测试规则。
- 依赖文档：rusqlite 官方 API，确认 `0.40.1`、`bundled`、`hooks`、只读 OpenFlags 与 progress handler。

结论：没有可直接复用的本地会话目录实现；可以复用既有观察分层、平台路径严格失败、LoadCompletion、错误脱敏和 Parity 拆分模式。CodeGraph CLI 报告当前工作区不存在有效索引；按项目规则不得擅自初始化，继续使用普通源码检索并在收口记录跳过。

## Change Contract

### Owner-visible behavior

- 返回可识别但最小披露的本地会话清单。
- 不返回任何真实路径、Provider 或对话内容。
- 部分来源失败必须显式标记，不能伪造完整结果。
- 没有数据库或完整可读但零会话返回 Empty。

### Data contract

- `session_id`: 非空、有界字符串；不出现在 Debug/错误。
- `display_title`: 可选、空白规范化、控制字符清理、最多 256 字符。
- `title_truncated`: 标题是否被截断。
- `archived`: bool。
- `updated_at_ms`: 可选 i64。
- 分页：offset、limit、has_more。
- 覆盖：discovered/readable/failed 与 Complete/Partial。

### Source contract

- `CODEX_HOME` 来自现有 `SystemPlatformPaths`。
- 非空 `CODEX_SQLITE_HOME` 必须为合法受支持目录，否则失败且不回退。
- 只观察 `sqlite/` 直接普通文件和 `state_5.sqlite`，候选最多 32。
- 只读 SQLite、query_only、busy timeout、deadline/cancellation。

### Sibling regression guard

以下已实现能力必须保持现有测试全绿：平台路径、应用概览、版本启动、运行时环境观察、Relay 环境、设置、诊断日志、Relay 状态和上下文条目观察。禁止为 Issue #104 修改其公开语义。

## Security Contract

- SQL 只使用固定模板和参数，不拼接用户值、路径或字段名。
- 只从受支持 schema 的白名单列生成查询；可选列由已发现列集合选择固定表达式。
- 不记录标题、ID、路径、文件名或 rusqlite 原始错误文本。
- 合成 fixture 不含真实用户路径或真实会话内容。
- 依赖必须锁定、许可证可接受，并由 Cargo.lock 保证可重复构建。
- 禁止网络、凭据、账号、UI、子进程、写入和永久后台线程。

## Planning Allowlist

```text
AGENTS.md
build.md
CONTEXT.md
docs/plans/2026-07-29-issue-104-gate-5-local-session-directory-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-29-issue-104-gate-5-local-session-directory-observation.md
docs/workflows/2026-07-29-issue-104-gate-5-local-session-directory-observation-runtime.md
```

- `count`: `7`
- `hash`: `sha256:8b9295414fde23c7ea9c9c53a47acfb7a70b1f1bcd3255f81cb011ba5b624a51`

## Candidate Implementation Allowlist

```text
AGENTS.md
build.md
Cargo.lock
Cargo.toml
CONTEXT.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/local_session_directory_observation.rs
crates/inputcodex-application/tests/local_session_directory_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/local_session_directory_observation.rs
crates/inputcodex-domain/tests/local_session_directory_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/local_session_directory_observation.rs
crates/inputcodex-platform/tests/local_session_directory_observation.rs
docs/plans/2026-07-29-issue-104-gate-5-local-session-directory-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-29-issue-104-gate-5-local-session-directory-observation.md
docs/reports/issue-104-gate-5-local-session-directory-observation.md
docs/workflows/2026-07-29-issue-104-gate-5-local-session-directory-observation-runtime.md
err.md
parity/README.md
parity/contracts/session-data.yml
parity/features/session-data.yml
parity/features/source-index.yml
parity/fixtures/feature.session-data.local-session-directory-observation/baseline.yml
parity/fixtures/feature.session-data.local-session-directory-observation/manifest.yml
README.md
```

- `count`: `29`
- `hash`: `sha256:47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d`
- 无新根因时实际范围排除 `err.md`，固定为 `28` 路径与 `sha256:acee55e9539631f6eca4fb557d27999c7815d20ae15a4b4ea932db243079cb8c`。

## Execution Batches

### Batch 0：Planning Freeze

状态：`ready-for-checkpoint`。

验证：planning 七路径与 hash、Markdown 链接、CI 合同 `35/35`、仓库政策、Release Audit、Cargo metadata 和 diff check 已通过。AGOS default-entry 返回 `blocked/unregistered`、`missing-owner-scope-manifest`；按 inputcodex 项目规则记录后绕过，不修改 AGOS。

### Batch 1：Domain RED → GREEN

状态：`pending`。

产出：领域条目、标题、分页和覆盖状态；先失败测试后最小实现。

### Batch 2：Application RED → GREEN

状态：`pending`。

产出：请求、取消标记、Port、UseCase 与完成态映射。

### Batch 3：Platform RED → GREEN

状态：`pending`。

产出：锁定 rusqlite、严格路径、只读 SQLite、schema 投影、多库分页/去重、超时取消和脱敏错误。

### Batch 4：Parity RED → GREEN

状态：`pending`。

产出：新 feature/contract/fixture、source remap 与原总功能继续未评估。

### Batch 5：Local Closeout

状态：`pending`。

产出：README、报告、完整定向验证、安全审查、范围证据和知识图谱状态。

### Batch 6：Review / CI

状态：`pending`。

产出：非 Draft PR、Review 根因闭环、CI/Performance/Artifact 证据和 Final Head。

## Agent Lifecycle

- `subagents`: `not-used`
- `reason`: 用户未明确授权 subagent；本任务保持单主线程，避免跨批次写入冲突。
- `independent_verification`: 本地主线程 fresh verification + GitHub-hosted CI + GitHub Review。
- `model_drift_guard`: 不切换模型，不把 reviewer 意见自动转为范围或实现授权。

## Skill Tree

- `superpowers:using-superpowers`
- `superpowers:brainstorming`
- `superpowers:writing-plans`
- `superpowers:using-git-worktrees`
- `superpowers:test-driven-development`
- `karpathy-guidelines`
- `domain-modeling`
- `security-review`
- `tdd-workflow`
- `knowledge-graph-auto-update`
- 完成前启用 `superpowers:verification-before-completion`、`requesting-code-review` 和 `finishing-a-development-branch`。

## Checkpoint Rules

- startup baseline 已由 AGOS Git snapshot report 证明干净。
- planning、domain、application、platform、parity、local-verified 各自形成命名提交。
- 每次 commit/push/PR 前重新检查 Git diff、范围与新鲜验证。
- Git 时间使用本机默认时间；禁止设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。
- 永久禁止对 `main` force push 或删除 `main`。

## Stop Conditions

- 变更超出二十九路径或 hash 漂移。
- 需要写数据库、修复 schema、删除、备份、恢复或输出私人数据。
- 需要新增第二个依赖家族、异步 runtime、连接池、UI、网络、子进程、Watcher 或 AGOS 改动。
- 依赖许可证/安全/跨平台构建不满足。
- RED 不是由缺失行为导致，或 GREEN 需要扩大语义。
- Release Audit stale、基线变化、工作树污染或验证失败。
- Final Head 未完成 Review/CI/Artifact 闭环却请求合并。

## Next Gate

完成 Planning Freeze 后直接进入 Domain RED；不再逐项请求项目所有者批准。只有触发 Stop Conditions 或最终 Squash Merge 时重新请求决策。
