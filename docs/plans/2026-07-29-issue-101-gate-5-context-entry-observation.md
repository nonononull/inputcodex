# Issue #101 Gate 5 上下文能力只读目录观察实施计划

> **状态：** `implementation-in-progress`
>
> **实施纪律：** 项目所有者已批准二十四路径与 `candidate_scope_hash`，允许按本计划执行 TDD、
> 本地验证、checkpoint、普通推送、非 Draft PR 与 Review/CI；最终 Squash Merge 仍保留独立授权门。

## 任务元数据

- `tracking_issue_ref`: https://github.com/nonononull/inputcodex/issues/101
- `approved_decision_ref`: https://github.com/nonononull/inputcodex/issues/100#issuecomment-5118455158
- `implementation_scope_approval_ref`: https://github.com/nonononull/inputcodex/issues/101#issuecomment-5119498422
- `decision_issue_ref`: https://github.com/nonononull/inputcodex/issues/100
- `branch_ref`: `codex/issue-101-gate-5-context-entry-observation`
- `baseline_ref`: `52320c2c02e19d9ffae11ccb6742a0f0fc4b71b9`
- `superpowers_skill`: `superpowers:brainstorming`
- `planning_skill`: `superpowers:writing-plans`
- `candidate_scope_hash`: `sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372`
- `candidate_scope_count`: `24`
- `planning_validation`: `PASSED`

## 目标

建立第九个 Gate 5 产品切片 `feature.provider-network.context-entry-observation`：只读观察
固定 `CODEX_HOME/config.toml` 中声明的 MCP Server、Skill 和 Plugin，返回条目 ID、稳定种类、
启用状态及分类计数，不返回配置正文或可推导敏感内容。

本切片只接管 `tauri-command:read_live_context_entries`。原上下文增加、删除、同步、提取、
设置正文解析和完整 `feature.provider-network.context-entry-management` 继续保持 `unassessed`。

## 已批准语义

### 加载结果

- 固定配置文件不存在：`LoadCompletion::Empty`。
- 文件存在且是合法空 TOML，或不含三类上下文 table：`LoadCompletion::Ready`，条目数为 `0`。
- 文件合法且存在条目：`LoadCompletion::Ready`，保留 TOML 源顺序。
- 元数据失败、符号链接、非普通文件、超限、读取失败、读取增长越界、非法 UTF-8、非法 TOML、
  重复定义、错误 table 类型、非 table 子项、空 ID 或错误布尔类型：
  `LoadCompletion::Failed`，只携带稳定诊断码。

### 条目投影

- `ContextEntryKind`: `McpServer / Skill / Plugin`。
- `ContextEntryObservation`: `id`、`kind`、`enabled`。
- `ContextEntryCategorySummary`: `total`、`enabled`、`disabled`。
- `ContextEntryCatalogObservation`: 有序条目与三类汇总。
- `enabled = false` 或 `disabled = true` 时条目禁用；两个字段均缺失时默认启用。
- `enabled` 或 `disabled` 存在但不是布尔值时整个配置无效，禁止默认成启用。

### 最小披露

允许：条目 ID、稳定种类、启用状态和分类计数。

禁止：原始 TOML、`toml_body`、`summary`、命令、参数、环境变量、Header、URL、Token、账号、
实际路径、用户名、机器名、内容片段或可逆派生值。

## 架构

### Domain

新增纯领域类型和不变量：

- ID 必须非空且只作为用户可见标识，不参与路径解析或命令执行。
- 分类计数只能由条目集合计算，调用方不能注入与条目不一致的计数。
- Domain 不依赖 `PathBuf`、`toml_edit`、文件系统、Iced 或平台类型。

### Application

新增零字段 `ContextEntryObservationRequest`、`ContextEntryObservationPort` 和
`ObserveContextEntries<P>`：

- `Ok(Some(value)) -> Ready(value)`；
- `Ok(None) -> Empty`；
- `Err(error) -> Failed(error)`；
- 旧设置、Relay 状态或任意路径请求不得调用本 UseCase。

### Platform

新增 `SystemContextEntryObservation`：

1. 通过 `SystemPlatformPaths.resolve` 得到固定 `codex_home()`。
2. 只派生 `config.toml`，不接受调用方路径或配置正文。
3. 使用 `symlink_metadata` 拒绝符号链接与非普通文件。
4. 元数据长度和实际读取均执行 `256 KiB` 上限，读取 `limit + 1` 字节识别增长竞态。
5. 使用 `toml_edit::DocumentMut` 严格解析，不调用上游重复项删除或文本修复函数。
6. 三类根项存在时必须是 table；每个子项必须是 table；布尔字段必须类型正确。
7. 原始字节、TOML 文档和配置字段在平台函数内部销毁，只返回领域投影。

### Parity

- 新增 `feature.provider-network.context-entry-observation`，状态为 `implemented`。
- 只把 `tauri-command:read_live_context_entries` 重映射到新子能力，副作用固定为
  `[filesystem-read]`。
- `list_context_entries`、`extract_relay_common_config`、`upsert_context_entry`、
  `delete_context_entry`、`sync_live_context_entries` 继续归原总功能。
- 原 `feature.provider-network.context-entry-management` 必须继续 `unassessed`。

## TDD 执行批次

以下批次只有在项目所有者批准路径与哈希后才能执行。

### Batch 1：Domain RED → GREEN

1. 新增领域失败测试，引用尚不存在的种类、条目、汇总和目录类型。
2. 验证 RED 原因只能是目标类型缺失。
3. 实现最小领域类型、构造器、getter、计数不变量和脱敏 Debug。
4. 运行 Domain tests、Clippy 和格式检查。
5. 建立 `issue-101-domain-green` Git checkpoint。

### Batch 2：Application RED → GREEN

1. 新增 Request、Port、UseCase 与 Some/None/Err 映射测试。
2. 验证旧请求类型不能调用新 UseCase。
3. 实现最小应用层合同。
4. 运行 Application tests、Clippy 和格式检查。
5. 建立 `issue-101-application-green` Git checkpoint。

状态：`completed`。Application RED 精确命中 Request、Port 与 UseCase 缺失；GREEN 后 6 个新增测试与 Application 全目标测试、Clippy、fmt 通过。

### Batch 3：Platform RED → GREEN

RED 矩阵至少覆盖：

- 文件缺失、合法空文件、无目标 table、三类合法条目和源顺序。
- 启用/禁用默认值、两个布尔字段组合和错误布尔类型。
- 非 table 根项、非 table 子项、空 ID、重复 table、非法 TOML、非法 UTF-8。
- 元数据超限、读取增长越界、符号链接、非普通文件、元数据失败和读取失败。
- 错误与 Debug 不包含正文、路径、命令、URL 或凭据片段。

GREEN 仅实现固定路径、有界读取、严格解析和最小投影。随后运行 Platform tests、Clippy 和格式
检查，并建立 `issue-101-platform-green` Git checkpoint。

状态：`completed`。Platform RED 覆盖固定路径、文件门禁、双重上限、严格 TOML、顺序、启用状态和隐私矩阵；GREEN 后 Platform 全目标测试、Clippy、fmt 通过。`DocumentMut` 清除 span 的新可复用根因已记录到 `err.md`。

### Batch 4：Parity RED → GREEN

1. 先写 feature/contract/source mapping RED 断言。
2. 新增 observation 子能力，只移动 `read_live_context_entries`。
3. 固定原总功能和五个剩余入口仍未评估。
4. 运行 Parity tests、Release Audit、Clippy 和格式检查。
5. 建立 `issue-101-parity-green` Git checkpoint。

状态：`completed`。Parity RED 精确命中新 feature/contract/source mapping 缺失；GREEN 只移动 `read_live_context_entries`，目录测试 `21/21`、Parity 全目标测试、Clippy、fmt 与 Release Audit `current` 通过。

### Batch 5：文档与本地收口

- 更新稳定产品说明、术语、Gate 状态、Parity 导航和任务报告。
- `build.md` 固化本地轻量验证、范围哈希、禁止能力和隐私扫描。
- 只运行四个受影响 crate 的定向测试/Clippy、格式、Release Audit、CI 合同、仓库政策、
  Cargo metadata、范围和隐私门禁。
- 完整 Workspace 与 Windows/macOS 编译交给 GitHub-hosted CI。

### Batch 6：远端交付

仅在本地收口通过后：普通 push、非 Draft PR、Review 根因闭环、标准 CI、Performance
observation、Artifact 核验，并绑定 Final Head 请求独立 Squash Merge 授权。

## 精确路径范围

1. `AGENTS.md`
2. `build.md`
3. `CONTEXT.md`
4. `crates/inputcodex-application/src/context_entry_observation.rs`
5. `crates/inputcodex-application/src/lib.rs`
6. `crates/inputcodex-application/tests/context_entry_observation.rs`
7. `crates/inputcodex-domain/src/context_entry_observation.rs`
8. `crates/inputcodex-domain/src/lib.rs`
9. `crates/inputcodex-domain/tests/context_entry_observation.rs`
10. `crates/inputcodex-parity/tests/catalog_repository.rs`
11. `crates/inputcodex-platform/src/context_entry_observation.rs`
12. `crates/inputcodex-platform/src/lib.rs`
13. `crates/inputcodex-platform/tests/context_entry_observation.rs`
14. `docs/plans/2026-07-29-issue-101-gate-5-context-entry-observation.md`
15. `docs/plans/PROJECT-MASTER-PLAN.md`
16. `docs/plans/sessions/2026-07-29-issue-101-gate-5-context-entry-observation.md`
17. `docs/reports/issue-101-gate-5-context-entry-observation.md`
18. `docs/workflows/2026-07-29-issue-101-gate-5-context-entry-observation-runtime.md`
19. `err.md`
20. `parity/contracts/provider-network.yml`
21. `parity/features/provider-network.yml`
22. `parity/features/source-index.yml`
23. `parity/README.md`
24. `README.md`

哈希算法固定为 Windows PowerShell `Sort-Object`、UTF-8 无 BOM、LF 拼接和末尾换行：

`sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372`

`err.md` 是批准上限中的新根因应急路径：没有形成新可复用根因时必须保持不变，最终实际范围
精确使用其余二十三路径，哈希为
`sha256:08b223934a07a66d91e5cf2e1b340a243ea460d6c4edc266f58d30101c478d47`。

## 禁止范围

- `Cargo.toml`、`Cargo.lock`、`.github/`、`apps/`、`upstream/`、`scripts/`、`benchmarks/`。
- `rusqlite`、网络客户端、系统命令、线程、Watcher、缓存或新运行时依赖。
- UI、Iced 交互、WebView、TypeScript、JavaScript、注入、广告或遥测。
- 修改 AGOS、Ruleset、性能预算、发布资产或上游缓存。

## 成功标准

- 二十四路径批准上限与哈希不漂移，无越界文件；没有新根因时实际范围精确排除 `err.md`，
  使用二十三路径和 `sha256:08b223934a07a66d91e5cf2e1b340a243ea460d6c4edc266f58d30101c478d47`。
- Domain/Application/Platform/Parity 定向测试和 Clippy 全绿，`cargo fmt --check` 通过。
- Release Audit 保持 `current`，CI/仓库政策通过。
- 新入口只读一个固定文件且最大 `256 KiB`，没有新增依赖或副作用。
- 原上下文管理总功能和剩余入口继续 `unassessed`。
- 新增行不包含真实账号、凭据、URL、绝对用户路径或配置正文。

当前 planning gate 已通过：七路径实际集合与
`sha256:0393705157d30192e317a8158686baf6c2a79483abab1e5a7a5b109d30923dbd` 精确一致；
Release Audit 为 `current`，CI 脚本合同 `35/35`、仓库政策、Cargo metadata、Parity baseline 和
`git diff --check` 均通过。

## 停止门

- 任何需要新增依赖、读取第二个文件、联网、写入、子进程、线程、UI 或原始配置返回的需求，
  必须停止并回到 Issue `#101`。
- 任何路径或语义新增必须重新计算 `candidate_scope_hash` 并取得项目所有者批准。
- Review 反馈必须确定根因、完成修复和验证后才能解决对话；最终 Squash Merge 保留独立授权门。
