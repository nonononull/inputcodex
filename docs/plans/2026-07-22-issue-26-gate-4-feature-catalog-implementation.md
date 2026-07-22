# Issue #26 Gate 4 功能目录、行为合同与脱敏夹具实现计划

> 执行要求：本计划在项目所有者批准后按批次实施；每个批次必须先取得可复核失败或缺口证据，再实现最小通过面。禁止把计划批准解释为最终 PR 合并授权。

**目标：** 为上游正式 Release `v1.2.41` 建立可审计、可机器验证的功能目录、行为合同与脱敏夹具事实层，为后续 Rust 功能重构和性能基线提供同一组稳定输入。

**架构：** 静态事实文件保存在 `parity/`，只由 `inputcodex-parity` 读取和验证；目录与验证器不得进入桌面发布运行面，也不得依赖 Iced、platform 或 presentation。五个分域文件保存功能和合同，`source-index.yml` 负责证明上游入口覆盖，夹具按稳定 feature ID 分目录保存并由 Rust 递归检查引用、安全和路径边界。

**技术栈：** Rust `1.97.1`、edition `2024`、Serde `1.0.229`、`yaml_serde 0.10.4`、项目现有 GitHub Actions 三平台 CI。依赖在实现开始时必须重新核对 crates.io/docs.rs 元数据、许可证、撤回状态和 Rust 版本兼容性；物质漂移时停止，不静默换包。

## 全局硬约束

- 软件名称固定为 `inputcodex`；禁止广告、推广、隐蔽遥测、TypeScript、JavaScript 业务代码和 WebView。
- 上游 `v1.2.41` 是唯一功能真源；上游 `main` 只作 Issue `#20` 预警。
- 只做审计事实、数据合同、脱敏夹具和验证器，不迁移或实现 `apps/` 产品功能。
- Windows 与 macOS 使用共同语义；差异必须显式记录，不能藏在实现备注中。
- 无效功能、有害副作用、错误语义或跨平台冲突只能进入 `exception-pending`，并分流到独立 `parity-exception`。
- `inputcodex-parity` 继续只依赖 domain/application 与纯数据解析依赖，不得依赖 infrastructure、platform、presentation 或 Iced。
- 本地只执行 `build.md` 定义的轻量定向命令；Workspace、Windows、macOS 和精确 Rust `1.97.1` 验证交给标准 GitHub-hosted runners。
- 所有普通提交必须可独立审查；禁止 amend 已推送证据、Force Push、删除 `main`、管理员绕过、Merge Commit 或 Rebase Merge。

## 一、Fresh 基线

- 追踪 Issue：`https://github.com/nonononull/inputcodex/issues/26`。
- 规划来源：Issue `#24`、PR `#25`、合并提交 `431682296f53e86de1184c732b0d4748857c9390`。
- 分支：`codex/issue-26-gate-4-feature-catalog`。
- 上游 Release：`v1.2.41`；tag 提交 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 上游 `main`：`91376ee3518cb5fe5ec8eead179418f706c25870`，只进入 Issue `#20`。
- 当前批准引用：`user-message:create-issue-26-session-plan-runtime-scope-hash-2026-07-22`。
- 实现批准引用：`user-message:approve-issue-26-implementation-2026-07-22`；Issue 评论 `5047650154`。
- 当前批准覆盖 36 条范围内实现、验证、普通提交、普通推送和 PR 创建；最终合并仍需具体 PR 与最终 Head 的独立授权。

## 二、精确最大写入范围

以下 36 条路径或路径模式是整个 Issue `#26` 的最大候选写集合；项目所有者批准实现前只有其中 8 条控制面路径可写。

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

规范化方式：使用 `StringComparer.Ordinal` 升序排列，UTF-8 无 BOM 编码，以 LF 分隔并保留末尾 LF。新范围哈希为：

```text
sha256:e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772
```

当前 control-plane checkpoint 只允许写入：

```text
AGENTS.md
README.md
build.md
docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md
docs/reports/issue-26-gate-4-feature-catalog.md
docs/workflows/2026-07-22-issue-26-gate-4-feature-catalog-runtime.md
```

## 三、文件职责

- `parity/features/source-index.yml`：列出被审计的上游入口、证据路径和最终映射；每个入口必须映射到稳定 feature ID、明确排除项或 `exception-pending`。
- `parity/features/<domain>.yml`：按五个批准 domain 保存功能事实、入口、双平台适用性、状态和决策引用。
- `parity/contracts/<domain>.yml`：保存每个场景的行为合同；合同通过 `feature_id` 与目录关联。
- `parity/fixtures/<feature-id>/`：保存夹具 manifest 与合成/不可逆脱敏数据；禁止跨目录引用和路径穿越。
- `catalog.rs`：定义 feature ID、domain、状态、来源证据与目录文档类型。
- `contract.rs`：定义合同 ID、场景、加载语义、超时、取消、错误隔离和平台期望类型。
- `fixture.rs`：定义 fixture ID、manifest、仓库相对路径和安全边界类型。
- `validation.rs`：执行跨文件唯一性、引用完整性、证据路径、状态、fixture 安全和 source-index 覆盖验证。
- 四个集成测试：分别约束目录 schema、合同 schema、夹具安全和仓库完整目录。

## 四、数据合同

### 功能目录

- ID 固定为 `feature.<domain>.<slug>`。
- domain 只允许 `foundation-platform`、`provider-network`、`session-data`、`plugin-script`、`remote-install`。
- 状态只允许 `unassessed`、`planned`、`implementing`、`implemented`、`verified`、`exception-pending`、`exception-approved`、`retired`。
- 首次登记默认 `unassessed`；后四种已实现状态不得由本任务伪造。
- 每项至少包含名称、Release、tag 提交、证据路径、入口、Windows/macOS 适用性和决策引用。

### 行为合同

- ID 固定为 `contract.<feature-id>.<scenario>`。
- 必须表达前置状态、输入、输出、数据格式、持久化、副作用、错误、加载、超时、取消、隔离、可观测和双平台语义。
- 加载状态固定为 `Idle / Loading / Ready / Empty / Failed / Cancelling`，并显式记录请求标识和旧请求失效规则。
- 合同可以声明“无 fixture”，但必须给出原因；引用 fixture 时必须存在且属于同一 feature ID。

### 脱敏夹具

- ID 固定为 `fixture.<feature-id>.<scenario>`。
- 只允许仓库相对路径；拒绝 `..`、绝对 Windows/UNC/POSIX 路径、控制字符和符号链接逃逸。
- 拒绝真实 Token、Cookie、账号、会话内容、设备标识、私人路径、签名材料和生产数据库。
- 安全验证基于结构化键和值、路径边界和显式占位策略，不使用“出现 token 单词即失败”的高误报规则。

## 五、实施任务

### Task 0：控制面 checkpoint

**文件：** 当前 8 条 control-plane 路径。

- [x] 创建 Issue `#26` 并锁定只创建 Issue 的首次授权。
- [x] 创建分支 `codex/issue-26-gate-4-feature-catalog`。
- [x] 计算 36 条最大范围和新 `scope_hash`。
- [x] 创建 Session Plan、Runtime Workflow、任务计划和初始报告。
- [x] 更新 AGENTS、README、build 与 Master Plan 的活动状态。
- [x] 运行规划 checkpoint 验证，确认 8/8 路径、36 条范围和治理合同通过。
- [x] 创建普通 checkpoint 提交 `80e0ddbb734496e95e89fe57fd89ddb668c8c276` 并普通推送。
- [x] 在 Issue `#26` 回写 checkpoint `5047590347`，进入实现批准等待状态。
- [x] 项目所有者通过 Issue 评论 `5047650154` 批准 36 条范围和实施顺序。

### Task 1：依赖元数据与 RED schema

**文件：** `Cargo.toml`、`Cargo.lock`、`crates/inputcodex-parity/Cargo.toml`、四个测试文件。

- [x] Fresh 核对 `serde 1.0.229` 与 `yaml_serde 0.10.4` 的版本、许可证、撤回状态和 Rust `1.97.1` 兼容性。
- [x] 在根 Workspace 固定依赖版本，在 parity crate 只启用必要 feature。
- [x] 先编写目录 ID/domain/status 的失败测试，证明当前 crate 不具备解析与验证能力。
- [x] 编写合同必填字段、六状态加载机、请求标识和 fixture 引用失败测试。
- [x] 编写路径穿越、真实秘密、私人绝对路径和跨 feature fixture 引用失败测试。
- [x] 保存 RED 命令、退出码和根因；三组定向编译均以退出码 `1` 失败在 crate root 缺少预期 API，`E0282` 仅为未解析返回类型导致的级联错误，依赖下载、YAML 文本和测试语法均已通过此前编译阶段。
- [x] 形成独立 RED checkpoint 提交 `532fba89d882862438345788ed2fdd73faede507`，并回写 Issue 评论 `5048079257`。

### Task 2：最小 Rust 类型与解析 GREEN

**文件：** `catalog.rs`、`contract.rs`、`fixture.rs`、`validation.rs`、`lib.rs`。

- [x] 实现无 I/O 的 ID/domain/status 值类型和反序列化。
- [x] 实现 feature、contract、fixture manifest 的最小数据结构。
- [x] 实现仓库相对路径规范化，拒绝绝对路径、反斜杠、控制字符和 `..`。
- [x] 实现唯一性、引用完整性、同 feature fixture 归属和平台字段验证。
- [x] 实现结构化敏感键和值检查，测试通过后再扩展规则。
- [x] 运行 parity crate 定向测试并形成 GREEN checkpoint `8b18f0a2a37829af3338edba34454eb6690af77a`，并回写 Issue 评论 `5048438316`。

### Task 3：上游入口清单与功能目录

**文件：** `parity/README.md`、`source-index.yml`、五个 feature 文件、目录仓库测试。

- [x] 从缓存 `upstream/CodexPlusPlus/` 静态枚举公开命令、核心能力入口、持久化/网络/进程/安装更新副作用和平台条件。
- [x] 每个入口映射到一个 feature ID、明确排除项或 `exception-pending`；禁止静默丢弃 Tauri/React、注入或远程推荐背后的能力证据。
- [x] 按五个 domain 建立稳定 feature ID；首次状态只允许 `unassessed` 或 `exception-pending`。
- [x] 为每条 feature 保存 `v1.2.41`、tag SHA、缓存证据路径、入口和双平台适用性。
- [x] 对无法证明完整性的模块记录缺口，不宣称“全部功能已盘点”。
- [x] 运行 source-index 覆盖与目录仓库测试，形成目录 checkpoint `87537e6e4a0e6911dd1427cc23f52dcb805a4679`，并通过 Issue 评论 `5048930060` 回写。

### Task 4：行为合同与脱敏夹具

**文件：** 五个 contract 文件、`parity/fixtures/**`、合同与夹具测试。

- [ ] 为每个可描述场景建立 contract ID，覆盖输入、输出、持久化、副作用、错误、加载、超时、取消、隔离和可观测证据。
- [ ] 明确 Windows/macOS 共同语义；差异只能作为显式平台字段或 `exception-pending`。
- [ ] 只在场景需要结构数据时建立 fixture；纯状态/无数据场景必须明确声明无需 fixture。
- [ ] 夹具只使用合成或不可逆脱敏数据，并保留类型、长度等级、边界值和关联关系。
- [ ] 运行 fixture 安全、引用完整性和全仓目录验证，形成合同 checkpoint。

### Task 5：文档、验证与 PR

**文件：** AGENTS、README、build、两个 err、Master Plan、Session Plan、Runtime Workflow 和报告。

- [ ] 更新 `build.md` 与 parity crate `build.md`，写明本地轻量命令和 GitHub 全量验证边界。
- [ ] 仅在真实新根因出现时更新对应 `err.md`；重复 Windows 沙箱错误引用既有记录。
- [ ] 更新报告中的功能数量、合同数量、fixture 数、排除项、exception-pending 和未覆盖缺口。
- [ ] 运行允许范围检查、`cargo fmt`、parity 定向测试、治理合同、仓库政策和 `git diff --check`。
- [ ] 创建关联 Issue `#26` 的非 Draft PR，正文包含 `Closes #26`、范围哈希、RED/GREEN 和未决缺口。
- [ ] 所有 Review 对话完成根因闭环，最终 Head CI 全绿后停止，等待项目所有者按 PR 号明确授权 Squash Merge。

## 六、验证命令

规划 checkpoint：

```powershell
& .\scripts\ci\Test-CiScripts.ps1
& .\scripts\ci\Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
git status --short --branch
```

实现阶段本地轻量验证：

```powershell
$env:RUSTUP_TOOLCHAIN = '1.93.1-x86_64-pc-windows-msvc'
cargo metadata --locked --offline --no-deps --format-version 1
cargo fmt --all -- --check
cargo check --locked --offline --ignore-rust-version -p inputcodex-parity
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity
& .\scripts\ci\Test-CiScripts.ps1
& .\scripts\ci\Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

GitHub Actions 必须证明：精确 Rust `1.97.1`、Linux Clippy/Workspace 全目标测试、Windows/macOS 编译与测试、`required` 汇总和 Artifact 边界。禁止为此把项目所有者本地机器变成 self-hosted runner。

## 七、完成定义

- 36 条最大范围无越界；产品源码、CI、Ruleset、Release、upstream、benchmarks 和 AGOS 零差异。
- source-index 对审计入口有明确映射或缺口，不使用无证据的完整性宣称。
- 五个 domain、八种状态、稳定 ID、合同和 fixture 规则均由 Rust 测试确定性验证。
- 真实敏感信息、绝对路径、路径穿越、跨 feature 引用和无 owner 决策的例外均被拒绝。
- 本地轻量验证和 GitHub 全量 CI 成功，成功 Artifact 数符合 Workflow 合同。
- Review 对话全部解决，根因、处理和验证证据完整。
- 最终 Squash Merge 只能在项目所有者对具体 PR 与最终 Head 明确授权后执行。

## 八、回滚

- 未合并前通过普通 revert 提交撤销具体批次，禁止改写分支历史。
- 合并后发现错误时建立事故/纠错 Issue，通过关联 PR revert `main` 上的 Squash 提交。
- 不允许为保留目录数据而跳过失败验证；证据不足的条目退回 `unassessed` 或 `exception-pending`。

## 九、停止条件

- 最新正式 Release/tag 或 source-lock 发生漂移。
- 需要修改 36 条范围外文件，或需要修改 CI、Ruleset、Release、upstream、benchmarks、产品源码或 AGOS。
- 依赖元数据、许可证、撤回状态或 Rust `1.97.1` 兼容性无法证明。
- 需要真实凭据、私人数据、签名材料或生产数据库才能建立夹具。
- 发现无效功能、有害副作用、错误语义或双平台冲突，但没有独立 `parity-exception` 与 owner 决定。
- RED 根因错误、GREEN 依赖跳过、Review 对话未闭环、适用 CI 未成功或缺少具体 PR 的 Squash Merge 授权。
