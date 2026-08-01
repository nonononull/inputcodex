# Issue #128 Session Plan：Watcher 偏好状态只读观察

## Session Metadata

- `task_id`: `issue-128-gate-5-watcher-preference-observation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/128`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/127#issuecomment-5151021881`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/128#issuecomment-5151032720`
- `standing_authorization_ref`: `https://github.com/nonononull/inputcodex/issues/111`
- `selected_business_path`: `gate-5/watcher-preference-observation`
- `baseline_ref`: `origin/main@43ace17de1505f251812e4ead3035ef3274a8455`
- `branch_ref`: `codex/issue-128-gate-5-watcher-preference-observation`
- `worktree_ref`: `C:/Users/dashuai/.paseo/worktrees/1takg4n7/issue-128-gate-5-watcher-preference-observation`
- `planning_scope_hash`: `sha256:359bfb23f99ed52174d8489271f19d597435bd271d426e1dd04ec4237064d018`
- `candidate_scope_hash`: `sha256:0be3dd45ed7e91d1cb7f633da369bb2b9052cefc512852d80362775da7e81699`
- `execution_state`: `IMPLEMENTATION_ACTIVE`

## Mutation Intent

- `mutation_intent`: `source`
- 只在批准的二十四路径内新增一个只读领域/应用/平台能力并修正一条 Parity source mapping。
- 不修改 Cargo、Workflow、Ruleset、upstream、UI、Release、性能预算或 AGOS。

## Executor Enforcement

- 当前允许：隔离 worktree 内 TDD、定向轻量验证、普通 Git checkpoint、push、PR、Review/CI 与精确门禁合并。
- 当前禁止：陈旧 `main` 写入、范围外修改、force push、GitHub 原生 auto-merge 与任何硬停止表面。
- 当前 writer：本会话唯一 writer；其他 agent 只允许 Final Head 只读复审。

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  memory_refs:
    - inputcodex autonomous refactor interrupt recovery
    - inputcodex Gate 5 observation planning
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - CONTEXT.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - parity/features/foundation-platform.yml
    - parity/contracts/foundation-platform.yml
    - parity/features/source-index.yml
    - upstream/CodexPlusPlus/crates/codex-plus-core/src/watcher.rs
    - upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs
  github_refs:
    - https://github.com/nonononull/inputcodex/issues/111
    - https://github.com/nonononull/inputcodex/issues/127
    - https://github.com/nonononull/inputcodex/issues/128
  codegraph: absent-not-initialized
  agos: project-native-bypass-no-cross-repo-mutation
```

## Change Contract

- `target_contract`: 固定标记文件元数据的只读、单探针、最小披露观察。
- `owner`: Issue `#128`；一致性决策为 Issue `#127`。
- `preserved_invariants`:
  - `release_audit=current`
  - Windows/macOS 使用相同领域和错误语义
  - Iced 只存在于展示层
  - 完整 Watcher 管理继续 `unassessed`
  - 无广告、遥测、注入、WebView 或脚本业务代码
- `adjacent_surfaces`:
  - `SystemPlatformPaths`
  - `LoadCoordinator`
  - `feature.foundation-platform.watcher`
  - `tauri-command:load_watcher_state`

## Planning Allowlist

```text
AGENTS.md
CONTEXT.md
build.md
docs/plans/2026-08-01-issue-128-gate-5-watcher-preference-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-08-01-issue-128-gate-5-watcher-preference-observation.md
docs/workflows/2026-08-01-issue-128-gate-5-watcher-preference-observation-runtime.md
```

- `count`: `7`
- `hash`: `sha256:359bfb23f99ed52174d8489271f19d597435bd271d426e1dd04ec4237064d018`

## Candidate Implementation Allowlist

完整二十四路径以 Issue #128 正文和实施计划为准。

- `count`: `24`
- `hash`: `sha256:0be3dd45ed7e91d1cb7f633da369bb2b9052cefc512852d80362775da7e81699`
- 哈希：`StringComparer.Ordinal` 去重排序，UTF-8 无 BOM，LF 拼接并保留末尾 LF。

## Execution Batches

### Batch 0：Planning Freeze

- 写入七路径项目原生控制面。
- 复算 planning/candidate hash，运行策略、Release Audit、仓库政策与空白检查。
- 建立 planning checkpoint。

状态：`completed`。

### Batch 1：Domain TDD

- 先取得目标类型缺失 RED，再实现两个纯领域类型。
- 状态：`pending`。

### Batch 2：Application TDD

- 先取得 Request/Port/UseCase 缺失 RED，再实现稳定映射。
- 状态：`pending`。

### Batch 3：Platform TDD

- 先取得固定文件名、元数据类型和 fail-closed 错误矩阵 RED，再实现单探针适配器。
- 状态：`pending`。

### Batch 4：Parity TDD

- 先取得新 feature/contract/source 缺失 RED，再只移动一个 command。
- 状态：`pending`。

### Batch 5：Local Closeout

- 更新稳定文档/报告，运行完整本地轻量门禁并建立 named checkpoint。
- 状态：`pending`。

### Batch 6：Review / CI / Merge

- 普通 push、非 Draft PR、独立只读复审、CI/Performance/Artifact、精确 Head Squash 与 main 验证。
- 状态：`pending`。

## Checkpoint Rules

- startup baseline：`43ace17de1505f251812e4ead3035ef3274a8455`
- checkpoints：planning、domain-green、application-green、platform-green、parity-green、local-verified
- 每次提交前运行范围核对和 `git diff --check`；禁止 amend、rebase、force push 或改写 `main`
- Git 时间只用本机 `Get-Date`，禁止设置作者/提交者时间环境变量

## Stop Conditions

- 读取内容、任意路径、路径回显、写入、进程控制、网络、子进程、线程、UI、依赖或范围扩张。
- Release、Ruleset、secret、收费资源或许可证硬停止。
- 测试失败根因不明、Review 对话未闭环、Final Head/CI/Artifact 证据不一致。

## Next Gate

完成 planning checkpoint 后进入 Domain RED，不再等待常规人工意见。
