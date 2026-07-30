# Issue #109 Session Plan：受控会话 Markdown 生成

```yaml
schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-109-gate-5-markdown-generation
work_class: standard
decision_status: approved
approval_source: direct-user
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/108#issuecomment-5130112866
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/109
delivery_contract: agos.issue-pr-merge.v1
selected_business_path: gate-5/session-data/markdown-generation
mutation_intent: source
allowed_operations:
  - planning-doc-write
  - bounded-source-write
  - bounded-test-write
  - parity-contract-write
  - local-verification
  - git-checkpoint
  - normal-push
  - pull-request
executor_enforcement: fail-closed-on-scope-or-semantic-drift
```

## Session Decision

项目所有者对 Discovery #108 明确回复 `A`。批准范围是“受控 Markdown 内存生成”，不是文件保存：

- 输入既有本地会话 ID；禁止任意路径。
- 严格只读 SQLite 与受控 rollout。
- 用户/助手文本、UTC、LF、不可联网图片占位。
- 返回建议文件名与内存 Markdown。
- 文件写入、保存对话框、覆盖确认和 UI 后置独立评审。

此前项目所有者已说明本项目属于重构项目、技术细节按推荐自主收敛；本次范围未越过 A，因此不重复请求中间批准。最终 Squash Merge 仍保留独立授权门。

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - "inputcodex Markdown 导出 会话 SQLite rollout 确定性 只读" -> no-results
    - "Markdown 导出 rollout SQLite" -> no-results
  vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/07-Workflows/Core/AI-Growth-OS-Brainstorming-Gate-And-Session-Plan.md
    - D:/Android_source/ai-growth-os/components/vault/08-Skills/AI-Growth-OS.md
  rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
    - AGENTS.md
  project_refs:
    - README.md
    - build.md
    - err.md
    - CONTEXT.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - parity/features/session-data.yml
    - parity/contracts/session-data.yml
    - parity/features/source-index.yml
    - upstream/CodexPlusPlus/crates/codex-plus-data/src/markdown.rs
    - upstream/CodexPlusPlus/crates/codex-plus-data/tests/markdown.rs
    - upstream/CodexPlusPlus/assets/inject/renderer-inject.js
  missing_coverage:
    - GBrain 与 vault 无 inputcodex Markdown 专项知识；锁定上游快照、项目原生控制面和 GitHub 决策作为事实真源
```

仓库没有 `.codegraph/`，未初始化索引。AGOS 仅 ReportOnly；任何 `needs-input`、未登记或接口异常记录后绕过。

本次实际 ReportOnly 结果为 `needs-input / unregistered / blocked`：外部 Registry 不存在
`issue-109-gate-5-markdown-generation`，mutation admission 不识别项目自定义 operation，并要求
protected-feature replay。该结果不修改本项目已批准范围，也不授权跨仓修复；按 `inputcodex`
项目规则记录为外部缺口后绕过，继续使用本 Session Plan、Runtime Workflow、Issue #109 与
`build.md` 门禁。

## Superpowers Method Discipline

```yaml
superpowers_method_discipline:
  using_superpowers: superpowers:using-superpowers
  brainstorming: superpowers:brainstorming
  writing_plans: superpowers:writing-plans
  worktree_isolation: superpowers:using-git-worktrees + Paseo native worktree
  test_driven_development: superpowers:test-driven-development
  verification_before_completion: superpowers:verification-before-completion
  security_review: security-review
```

## Change Contract

- `target_contract`: 按受控会话 ID 生成确定性 Markdown 内存文档。
- `owner`: Issue `#109`；语义决策为 Issue `#108`。
- `preserved_invariants`:
  - `release_audit=current`
  - Windows/macOS 同一领域、应用、错误与字节语义
  - Iced 只在展示层
  - Issue #104 目录分页、去重、隐私与只读行为不变
  - 文件保存、renderer 注入和原子写入未实现
  - 无广告、遥测、网络、子进程或 `unsafe`
- `adjacent_surfaces`:
  - `SystemPlatformPaths`
  - `local_session_directory_observation`
  - session-data Parity 目录
  - renderer-enhancements 例外证据
- `historical_state_refs`:
  - Issue `#26` Gate 4 目录基线
  - Issue `#65` v1.2.43 重审
  - Issue `#103/#104` 本地会话目录
  - Issue `#108` Markdown 生成/保存分离决策
- `regression_checks`:
  - 四个相关 crate tests/Clippy
  - rustfmt、CI 合同、仓库政策、Release Audit、Cargo metadata
  - scope hash、隐私扫描、禁止能力扫描和 `git diff --check`
- `sibling_regression_guard`: 必须在 local-verified 前为 passed。

## 精确资源常量

| 常量 | 值 |
| --- | ---: |
| SQLite 候选 | 32 |
| rollout 最大深度 | 4 |
| rollout 枚举项 | 8192 |
| JSONL 候选 | 4096 |
| 单候选 metadata 前缀 | 8 KiB |
| 发现累计读取 | 32 MiB |
| 选中 rollout | 16 MiB |
| 非空 JSONL 记录 | 100000 |
| 导出消息 | 20000 |
| Markdown 输出 | 16 MiB |
| 建议文件名 | 160 UTF-8 bytes |
| SQLite busy timeout | 50 ms |
| progress interval | 1000 |
| 整体 deadline | 2 s |

## Planning Allowlist

```text
AGENTS.md
build.md
CONTEXT.md
docs/plans/2026-07-30-issue-109-gate-5-markdown-generation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-30-issue-109-gate-5-markdown-generation.md
docs/workflows/2026-07-30-issue-109-gate-5-markdown-generation-runtime.md
```

- `count`: `7`
- `hash`: `sha256:41598221ce6fc613299d7cc881e57c5472c9a937072f93a2c8edbf821e7029a3`

## Candidate Implementation Allowlist

```text
AGENTS.md
build.md
CONTEXT.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/markdown_generation.rs
crates/inputcodex-application/tests/markdown_generation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/markdown_generation.rs
crates/inputcodex-domain/tests/markdown_generation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/markdown_generation.rs
crates/inputcodex-platform/tests/markdown_generation.rs
docs/plans/2026-07-30-issue-109-gate-5-markdown-generation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-30-issue-109-gate-5-markdown-generation.md
docs/reports/issue-109-gate-5-markdown-generation.md
docs/workflows/2026-07-30-issue-109-gate-5-markdown-generation-runtime.md
err.md
parity/contracts/session-data.yml
parity/features/plugin-script.yml
parity/features/session-data.yml
parity/features/source-index.yml
parity/fixtures/feature.session-data.markdown-export/baseline.yml
parity/fixtures/feature.session-data.markdown-export/manifest.yml
parity/fixtures/feature.session-data.markdown-generation/baseline.yml
parity/fixtures/feature.session-data.markdown-generation/manifest.yml
parity/README.md
README.md
```

- `count`: `29`
- `hash`: `sha256:b113da5d41514f50e36cef7d4eb9ade89e2562cbcfe8d392a5173d38fd0ebaac`
- 无新根因时排除 `err.md`：`28` 路径，`sha256:3d19b05918c8294f050f3f0c83f9118453705afd8aad016827fafb477e74a50b`。
- 范围命令固定使用 `git diff --no-renames --name-only`，保证旧/new fixture 路径分别计数。

## Execution Batches

1. Planning freeze：七路径、两个 hash、Release Audit 与基线验证。
2. Domain RED/GREEN：纯类型、渲染、文件名与 Debug。
3. Application RED/GREEN：Request、Cancellation、Port、UseCase。
4. Platform RED/GREEN：SQLite、rollout、JSONL、UTC、边界与错误。
5. Parity RED/GREEN：后端生成重分类，renderer 保存继续 exception-pending。
6. Local closeout：稳定文档、报告、安全审查、完整轻量门禁。
7. Remote delivery：普通 push、非 Draft PR、Review/CI/Artifact、Final Head 授权。

## Agent Lifecycle

- `agent_strategy`: parent-only
- `open_agent_count_before_dispatch`: `0`
- `reclaim_before_spawn`: `not-required`
- `claimed_file_owners`: 父任务独占 29 路径
- `completion_status`: `no-subagents-used`

## Stop Conditions

- Release/audit 漂移、范围外路径、新依赖或第二个功能。
- 文件写入、UI、网络、子进程、永久线程/Watcher、注入或 `unsafe`。
- 任意路径、内部角色、远程图片 URL、完整 session ID、真实路径或原始错误披露。
- 无法证明只读 SQLite、rollout 根归属、资源上限、超时取消或跨平台逐字节确定性。
- Review 未闭环、Hosted CI/Artifact 未通过或缺 Final Head 独立合并授权。
