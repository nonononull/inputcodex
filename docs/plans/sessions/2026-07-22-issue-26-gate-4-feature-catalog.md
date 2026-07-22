# Session Plan：Issue #26 Gate 4 功能目录、行为合同与脱敏夹具

schema_version: inputcodex.session-plan.v1
session_status: control-plane-verified-commit-pending
task_id: 2026-07-22-issue-26-gate-4-feature-catalog
work_class: major
task_summary: 建立上游 v1.2.41 功能目录、行为合同、脱敏夹具和纯 Rust parity 验证器，不迁移产品功能，不进行性能优化。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/26
planning_issue_ref: https://github.com/nonononull/inputcodex/issues/24
planning_pr_ref: https://github.com/nonononull/inputcodex/pull/25
branch_ref: codex/issue-26-gate-4-feature-catalog
baseline_ref: 431682296f53e86de1184c732b0d4748857c9390
approved_decision_ref: user-message:create-issue-26-session-plan-runtime-scope-hash-2026-07-22
implementation_decision_ref: pending-owner-review
session_plan_ref: docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md
implementation_plan_ref: docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-26-gate-4-feature-catalog-runtime.md
report_ref: docs/reports/issue-26-gate-4-feature-catalog.md
pr_ref: pending
scope_hash: sha256:e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772
scope_path_count: 36
control_plane_path_count: 8
mutation_intent: 先建立并提交项目原生控制面；项目所有者批准实现后，再按 RED schema、GREEN Rust 类型、source-index、功能目录、合同夹具和 PR closeout 分批执行。
executor_enforcement: 每个批次前后核对分支、36 条最大路径、当前批次子集、上游锁、敏感信息边界、定向测试和 Git 快照；未批准批次禁止写入。
delivery_contract: inputcodex.issue-pr-merge.v1
external_agos_policy: available-optional-unavailable-bypass-no-optimization
external_agos_execution: needs-input-unregistered-recorded-and-bypassed

## 一、当前批准状态

- 项目所有者已要求为 Issue `#26` 建立独立 Session Plan、Runtime Workflow、精确写入范围和新 `scope_hash`。
- 当前批准允许：创建独立分支、写入 8 条控制面路径、本地只读/轻量验证、普通提交、普通推送和 Issue checkpoint。
- 当前批准不允许：创建 `parity/` 数据、修改 Cargo/Rust 代码、开始上游功能审计、创建实现 PR 或执行合并。
- 用户关于“该提交的提交、该合并的合并”只作为遵循正常交付链的指令；未知 PR、未知最终 Head 和未知 CI 不能取得空白合并授权。
- 实现开始前必须由项目所有者明确批准本 Session Plan、Runtime Workflow、36 条范围和 `scope_hash`。
- 最终 Squash Merge 前仍需对具体 PR 和最终 Head 取得新的明确授权。

## 二、Fresh 基线

- `main`、`origin/main` 与启动 Head 均为 `431682296f53e86de1184c732b0d4748857c9390`，启动工作树干净。
- Issue `#26` 为 OPEN，标签为 `type:architecture` 与 `gate:4`；正文已冻结首次授权、目标、边界和停止条件。
- PR `#25` 已 Squash Merge，Issue `#24` 已按 COMPLETED 关闭；合并后主干运行 `29926710342` 六 Job 全绿，成功 Artifact 数为 `0`。
- GitHub API Fresh 核对最新正式 Release 仍为 `v1.2.41`，tag 提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 上游 `main` 仍为 `91376ee3518cb5fe5ec8eead179418f706c25870`，只由 Issue `#20` 预警；Issue `#16/#20` 不得手工修改。
- `.codegraph/` 不存在，不初始化索引。
- AGOS default entry 已 report-only 返回 `needs-input / unregistered`；按项目合同记录并绕过，未修改 AGOS Registry、Workflow、Rules 或 Vault。
- 仓库仍只有一名具备合并权限的人类维护者 `nonononull`；required approvals 为 `0`，但所有者决策证据仍是硬门禁。

## 三、allowed_operations

### 当前 control-plane checkpoint

```text
git.branch.create: codex/issue-26-gate-4-feature-catalog
docs.write: 8 条 control-plane 路径
docs.validate: 路径、哈希、占位符、治理合同、仓库政策、git diff --check
git.commit: 普通提交，禁止 amend 已推送证据
git.push: 普通 push，禁止 force/force-with-lease
github.issue.comment: 回写计划 checkpoint 和批准边界
```

### 项目所有者批准实现后

```text
rust.write: 仅 crates/inputcodex-parity 的批准文件
parity.data.write: 五个 feature 文件、source-index、五个 contract 文件和 parity/fixtures/**
cargo.write: 根 Cargo.toml/Cargo.lock 与 parity crate Cargo.toml
tests.run.local: parity 定向测试、格式、治理合同、仓库政策、空白检查
github.actions.use: 现有标准 GitHub-hosted runners，不修改 Workflow
github.pr.create: 关联 Issue #26 的非 Draft PR
github.issue_or_pr.comment: 回写 RED/GREEN、Review、CI 与 owner 决策证据
```

未列出的 mutation 默认禁止。特别禁止产品、CI、Ruleset、Release、upstream、benchmarks、AGOS、Force Push、删除 `main` 和管理员绕过。

## 四、精确最大写集合

```text
AGENTS.md
Cargo.lock
Cargo.toml
README.md
build.md
crates/inputcodex-parity/Cargo.toml
crates/inputcodex-parity/build.md
crates/inputcodex-parity/err.md
crates/inputcodex-parity/src/catalog.rs
crates/inputcodex-parity/src/contract.rs
crates/inputcodex-parity/src/fixture.rs
crates/inputcodex-parity/src/lib.rs
crates/inputcodex-parity/src/validation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-parity/tests/catalog_schema.rs
crates/inputcodex-parity/tests/contract_schema.rs
crates/inputcodex-parity/tests/fixture_safety.rs
docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md
docs/reports/issue-26-gate-4-feature-catalog.md
docs/workflows/2026-07-22-issue-26-gate-4-feature-catalog-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/contracts/plugin-script.yml
parity/contracts/provider-network.yml
parity/contracts/remote-install.yml
parity/contracts/session-data.yml
parity/features/foundation-platform.yml
parity/features/plugin-script.yml
parity/features/provider-network.yml
parity/features/remote-install.yml
parity/features/session-data.yml
parity/features/source-index.yml
parity/fixtures/**
```

规范化算法：`StringComparer.Ordinal` 升序、UTF-8 无 BOM、LF 分隔、保留末尾 LF。结果为 `sha256:e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772`。

当前 8 条控制面路径是：AGENTS、README、build、Master Plan、任务计划、Session Plan、Runtime Workflow 和初始报告。`err.md` 只在真实新根因出现时使用；本 checkpoint 的重复 Windows 沙箱启动错误复用既有记录，不新增模板化条目。

## 五、设计决定

1. 数据按五个 domain 分文件，避免单一巨型 YAML，同时保证 ID 不受 UI 或文件移动影响。
2. `source-index.yml` 是完整性证据入口：每个已审计上游入口必须映射到 feature、明确排除项或 `exception-pending`。
3. Rust 类型是 schema 真源，不另建第二套 JSON Schema；集成测试直接读取仓库 `parity/` 数据。
4. `inputcodex-parity` 不进入桌面发布运行面，不依赖 Iced/platform/presentation，不执行上游功能。
5. 行为合同按 domain 保存，fixture 按 feature ID 分目录；合同引用必须可解析且不能跨 feature。
6. fixture 安全使用结构化键值、路径边界和显式占位策略，避免简单关键词扫描造成高误报。
7. 首次目录登记只使用 `unassessed` 或 `exception-pending`；本任务不宣称产品功能已经 implemented/verified。
8. 性能基线、预算和优化留给后续独立 Issue，不能在本任务中顺手加入。

## 六、执行批次

### Batch 0：startup-baseline

- Fresh 验证 Git、Issue、Release/tag、上游 main、source-lock、Ruleset、维护者数量和主干 CI。
- 确认 `.codegraph/` 缺失后不初始化；AGOS 只允许 report-only，失败或 needs-input 即记录并绕过。

### Batch 1：control-plane-checkpoint

- 创建任务计划、Session Plan、Runtime Workflow 和初始报告。
- 更新 AGENTS、README、build 和 Master Plan；产品、Cargo、parity、CI、upstream、benchmarks 与 AGOS 零差异。
- 验证 8 条当前路径、36 条最大范围、哈希算法和无占位符。
- 普通提交、普通推送并在 Issue `#26` 回写 checkpoint，然后停止等待实现批准。

### Batch 2：dependency-and-red-schema

- Fresh 复核依赖元数据后，只添加固定 Serde/YAML 解析依赖。
- 先写 catalog/contract/fixture 的失败测试，保存根因正确的 RED 证据。
- 形成独立 RED checkpoint；禁止因格式错误或缺测试数据伪造 RED。

### Batch 3：rust-schema-green

- 实现稳定 ID、domain、状态、合同、fixture manifest 与验证错误类型。
- 实现路径、唯一性、引用、加载语义、平台字段和敏感信息验证。
- 定向测试 GREEN 后形成普通提交并回写 Issue。

### Batch 4：source-index-and-features

- 静态审计缓存上游，建立 source-index 和五个 feature 文件。
- 每个入口映射到 feature、排除项或 `exception-pending`；缺口必须显式报告。
- 不运行或移植 Tauri/React、注入脚本和远程推荐列表。

### Batch 5：contracts-and-fixtures

- 建立五个 contract 文件和必要 fixture。
- 验证双平台语义、引用完整性、路径安全和敏感信息边界。
- 无 owner 决策的争议项只能保持 `exception-pending`。

### Batch 6：verification-pr-closeout

- 更新 build/report/Master Plan 和真实 err 记录。
- 运行本地轻量验证，创建非 Draft PR 并等待现有 CI 的全量三平台验证。
- Review 对话全部根因闭环；最终 Head 全绿后等待具体 PR 的 Squash Merge 授权。

## 七、依赖与工具链证据

- Rust 固定 `1.97.1`，不得改成浮动 `stable`。
- Serde 候选固定 `1.0.229`，MIT OR Apache-2.0；只启用 `derive`。
- YAML 候选固定 `yaml_serde 0.10.4`，MIT OR Apache-2.0；实现开始前必须再次确认未撤回且能由 Rust `1.97.1` 构建。
- 依赖版本、feature 或包名发生变化时必须更新计划、重新计算影响并取得 owner 批准；不能因为本机下载超时静默换库。

## 八、验证矩阵

```text
本地规划 checkpoint:
  - 8 条路径，无越界
  - scope_hash 重算一致
  - CI_CONTRACT_GREEN passed=30
  - repository policy ok=true / violation_count=0
  - git diff --check

本地实现轻量验证:
  - cargo metadata --locked --offline --no-deps
  - cargo fmt --all -- --check
  - cargo check --locked --offline --ignore-rust-version -p inputcodex-parity
  - cargo test --locked --offline --ignore-rust-version -p inputcodex-parity
  - scripts/ci/Test-CiScripts.ps1
  - scripts/ci/Verify-RepositoryPolicy.ps1
  - git diff --check

GitHub 全量验证:
  - classify
  - governance
  - linux-quality: Rust 1.97.1 / fmt / clippy / workspace tests
  - windows: 构建与 workspace tests
  - macos: 构建与 workspace tests
  - required
```

## 九、完成定义

- source-index、五个 feature 文件、五个 contract 文件和必要 fixture 均通过 Rust 仓库级验证。
- 每条目录记录有上游证据；未覆盖和争议项明确可见，没有虚假完整性声明。
- ID、domain、八种状态、六加载状态、平台、超时、取消、隔离和可观测字段均被验证。
- 产品、性能、CI、Ruleset、Release、upstream 和 AGOS 零差异。
- 本地与 GitHub 适用验证全绿，Review 对话全部闭环，最终合并有具体 owner 授权证据。

## 十、停止条件

- Release/tag/source-lock、许可证、Ruleset、维护者数量或批准范围发生物质变化。
- 需要运行真实私人账号、写入真实秘密、修改产品/CI/upstream/benchmarks/AGOS 或越过 36 条范围。
- 依赖元数据无法确认，RED/GREEN 根因不成立，或目录完整性无法证明却准备宣称完成。
- 出现无效功能、副作用、错误语义或双平台冲突，但没有独立 `parity-exception` 与 owner 决定。
- PR Head 漂移、适用 CI 失败、Review 根因未解决或缺少具体 PR 的 Squash Merge 授权。
