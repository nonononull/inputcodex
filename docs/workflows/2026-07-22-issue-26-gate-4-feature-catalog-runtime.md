# Runtime Workflow：Issue #26 Gate 4 功能目录、行为合同与脱敏夹具

schema_version: inputcodex.runtime-workflow.v1
task_id: 2026-07-22-issue-26-gate-4-feature-catalog
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/26
branch_ref: codex/issue-26-gate-4-feature-catalog
baseline_ref: 431682296f53e86de1184c732b0d4748857c9390
approved_decision_ref: user-message:create-issue-26-session-plan-runtime-scope-hash-2026-07-22
implementation_decision_ref: user-message:approve-issue-26-implementation-2026-07-22
implementation_approval_ref: issuecomment:5047650154
scope_hash: sha256:e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772
scope_path_count: 36
session_plan_ref: docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md
implementation_plan_ref: docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md
report_ref: docs/reports/issue-26-gate-4-feature-catalog.md
pr_ref: pending

## 当前 checkpoint

- Issue `#26` 已创建，标签和首次授权边界已验证。
- Fresh 上游功能真源仍为 `v1.2.41`，tag 提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`；上游 main `91376ee3518cb5fe5ec8eead179418f706c25870` 只作预警。
- 已创建分支 `codex/issue-26-gate-4-feature-catalog`。
- 36 条最大范围使用 Ordinal 排序、UTF-8 无 BOM、LF 和末尾 LF 计算，哈希为 `sha256:e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772`。
- AGOS default entry report-only 返回 `needs-input / unregistered`，已按项目原生合同绕过；本任务不修复或登记 AGOS。
- control-plane checkpoint 已以普通提交 `80e0ddbb734496e95e89fe57fd89ddb668c8c276` 推送，并回写 Issue 评论 `5047590347`。
- 项目所有者已通过 Issue 评论 `5047650154` 批准实施。
- Phase 2 已完成依赖锁定和三组 RED 定向编译：`catalog_schema`、`contract_schema`、`fixture_safety` 均以退出码 `1` 失败在 crate root 缺少预期 API；`E0282` 为未解析返回类型的级联诊断，未发现依赖、YAML、fixture 或测试语法错误。
- RED checkpoint 已以普通提交 `532fba89d882862438345788ed2fdd73faede507` 推送，并回写 Issue 评论 `5048079257`。
- 生产实现仍为零；RED 门禁已经满足，当前进入 Phase 3 最小 Rust schema GREEN。

## Phase 0：startup-baseline

1. 确认启动基线为 `main@431682296f53e86de1184c732b0d4748857c9390`，工作树干净。
2. 确认 Issue `#26` 为 OPEN，Issue `#24` 已 COMPLETED，PR `#25` 已 MERGED。
3. Fresh 查询 Release `v1.2.41`、tag SHA、上游 main、source-lock、Issue `#16/#20`、Ruleset和维护者数量。
4. 确认 `.codegraph/` 不存在后不初始化；项目文档和缓存上游是本任务本地知识输入。
5. AGOS 只允许 report-only；不可用、needs-input、未登记或接口异常时记录并绕过，禁止修改外部仓库。

## Phase 1：control-plane-checkpoint

1. 只创建任务计划、Session Plan、Runtime Workflow 和初始报告，并更新 AGENTS、README、build 与 Master Plan。
2. 当前变更必须恰好位于 8 条控制面路径；不得创建 Cargo、Rust、`parity/`、测试或 Workflow 变更。
3. 校验四份控制面包含真实 Issue、分支、基线、批准引用、36 条范围、scope hash、TDD 批次、验证、停止与回滚合同。
4. 运行 Issue `#26` 规划 checkpoint 验证、治理合同、仓库政策和 `git diff --check`。
5. 创建普通提交并普通 push，禁止 amend、Force Push 或删除 `main`。
6. 在 Issue `#26` 回写 commit、分支、计划引用、scope hash 和“实现仍待批准”。
7. 停止，等待项目所有者明确批准实现；不得把 control-plane checkpoint 自动延伸为 Batch 2。

## Phase 2：dependency-and-red-schema

1. Fresh 复核 Serde/YAML crate 的官方版本、许可证、撤回状态、checksum 与 Rust `1.97.1` 兼容性。
2. 仅在 owner 批准范围内修改根 Cargo、Cargo.lock 和 parity crate Cargo。
3. 先创建 catalog、contract、fixture 的失败测试；RED 必须来自目标类型/验证能力缺失。
4. RED 覆盖：非法 feature/contract/fixture ID、未知 domain/status、缺失字段、重复 ID、悬空引用、非法加载状态、路径穿越、绝对路径、敏感值和跨 feature fixture 引用。
5. 保存精确命令、退出码和根因，形成普通 RED checkpoint 并回写 Issue。

执行结果：

- `catalog_schema --no-run`：退出码 `1`，根因 `E0432`，缺少 catalog 稳定类型、解析与验证 API。
- `contract_schema --no-run`：退出码 `1`，根因 `E0432`，缺少 contract 加载状态、解析与验证 API。
- `fixture_safety --no-run`：退出码 `1`，根因 `E0432`，缺少 fixture manifest、载荷安全与验证 API。
- 三组测试均已通过依赖解析和测试文件语法阶段；`catalog_repository` 延后到 Phase 4 数据面建立后执行。

## Phase 3：rust-schema-green

1. 在 `catalog.rs` 实现 feature/domain/status/来源证据类型。
2. 在 `contract.rs` 实现行为合同、六加载状态、请求标识、超时、取消、错误隔离、可观测和平台字段。
3. 在 `fixture.rs` 实现 fixture manifest、仓库相对路径和结构化敏感数据边界。
4. 在 `validation.rs` 实现唯一性、引用、来源路径、fixture 归属和验证错误聚合。
5. `lib.rs` 只公开稳定验证 API；不得加入 Iced、platform、presentation 或产品调用。
6. 定向测试 GREEN 后形成普通 checkpoint；任何绕过失败测试的实现必须回退。

## Phase 4：source-index-and-feature-catalog

1. 只读审计 `upstream/CodexPlusPlus/`，不运行或构建上游 Tauri/React 管理界面、注入脚本和远程推荐列表。
2. `source-index.yml` 记录上游公开命令、核心入口、持久化/网络/进程/安装更新副作用和平台条件。
3. 每个入口必须映射到 feature ID、明确排除项或 `exception-pending`；未映射入口使仓库验证失败。
4. 五个 feature 文件只使用批准 domain 和八种状态；首次状态只允许 `unassessed` 或 `exception-pending`。
5. 每条 feature 记录 Release/tag、证据路径、入口、平台适用性和决策引用。
6. 无法证明完整性时记录缺口并停止完整性宣称，不使用空目录或总数猜测伪造覆盖。

## Phase 5：contracts-and-fixtures

1. 按 domain 建立行为合同；每个合同 ID 必须与已有 feature ID 和 scenario 对齐。
2. 合同覆盖前置、输入、输出、数据、持久化、副作用、错误、加载、超时、取消、隔离、可观测和双平台语义。
3. 只有需要结构数据的场景创建 fixture；无 fixture 场景必须显式说明原因。
4. fixture 只用合成或不可逆脱敏数据，保留类型、长度等级、边界值和关联关系。
5. 真实秘密、私人路径、签名材料、生产数据库、路径逃逸或跨 feature 引用必须由测试拒绝。
6. 争议项只转为 `exception-pending` 并候选创建独立 parity-exception；禁止在本 PR 偷偷批准。

## Phase 6：local-verification

1. 核对实际变更全部位于 36 条最大范围，产品、CI、Ruleset、Release、upstream、benchmarks 和 AGOS 零差异。
2. 运行 `cargo metadata --locked --offline --no-deps --format-version 1`。
3. 运行 `cargo fmt --all -- --check`。
4. 本机缺少 Rust `1.97.1` 时，只允许用既有 `1.93.1 --ignore-rust-version` 执行 parity 定向 check/test；结果不能替代精确工具链证据。
5. 运行治理合同、仓库政策和 `git diff --check`。
6. 更新报告中的 feature、contract、fixture、排除项、exception-pending 和缺口数量。

## Phase 7：pr-review-ci

1. 创建关联 Issue `#26` 的非 Draft PR，正文包含 `Closes #26`、scope hash、路径范围、RED/GREEN、目录覆盖、敏感数据验证和回滚证据。
2. Fresh 核对 PR 文件、最终 Head、自动合并关闭、Ruleset 无漂移和 Review 对话状态。
3. Linux、Windows、macOS 和 required 的适用 Job 必须成功；非法 skipped、0 Checks 或只靠 rerun 不算通过。
4. Artifact 只允许 Workflow 既有失败白名单，成功运行不得上传整个 target 或敏感夹具。
5. 所有 Review 对话必须记录根因、处理和验证证据后解决。
6. 最终 Head 全绿后停止，等待项目所有者对具体 PR 的 Squash Merge 授权。

## Phase 8：merge-closeout

1. 只在具体 PR、最终 Head、CI 与 owner 授权一致时执行 Squash Merge，禁止 `--admin`。
2. 合并后验证 Issue `#26` 按 COMPLETED 关闭、merge commit 单父、tree 与最终 Head 等价、签名 `valid`、远端分支删除。
3. 等待合并后 main CI 全绿并确认成功 Artifact 数符合合同。
4. 在已关闭 Issue `#26` 回写最终 closeout；不直接修改 main 制造递归 closeout PR。
5. 功能目录收口后才允许创建独立性能基线 Issue；Gate 5 继续锁定。

## 停止条件

- Release/tag/source-lock、依赖元数据、许可证、Ruleset、维护者数量、Head 或批准范围发生物质变化。
- 需要运行真实账号、写入敏感材料、修改范围外路径、产品、CI、upstream、benchmarks、Release、Ruleset 或 AGOS。
- RED 根因不成立、GREEN 依赖跳过、source-index 无法证明覆盖却准备宣称完整。
- 出现无效功能、有害副作用、错误语义或双平台冲突，但没有独立 parity-exception 与 owner 决定。
- Review 对话未根因闭环、适用 CI 未成功或缺少具体 PR 的 Squash Merge 授权。
