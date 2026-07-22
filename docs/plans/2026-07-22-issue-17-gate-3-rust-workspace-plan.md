# Issue #17：Gate 3 纯 Rust Workspace 骨架规划

## 基本信息

- 日期：2026-07-22。
- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/17`。
- 规划分支：`codex/issue-17-gate-3-rust-workspace-plan`。
- 基线提交：`113476fb96623452f9a69526edabc73a57d812a1`。
- 上游功能真源：`BigPizzaV3/CodexPlusPlus v1.2.41`。
- 决策引用：`user-message:approve-gate-3-planning-2026-07-22`。
- 范围哈希：`sha256:0c4fc5017aed0b5883b5cb597b2afc2680646479de32916cc4e720aff67dfd05`。
- 当前状态：规划已批准，源码实现尚未授权。

## 一、目标

本任务只建立 Gate 3 的项目原生规划控制面，冻结后续“纯 Rust Workspace 与首版三平台 CI”实施任务的边界、依赖方向、验证证据和停止条件。规划 PR 合并前后，仓库都必须继续保持没有产品 `Cargo.toml`、`Cargo.lock`、`.rs`、Iced UI 或新的产品 CI Workflow。

本计划解决的根因不是“缺少目录”，而是缺少可阻止以下失败模式的任务合同：

- Iced、平台 API、存储或网络类型向领域和应用层倒灌。
- 桌面入口再次承载业务逻辑、阻塞 I/O 或不可取消加载。
- Windows 与 macOS 通过条件编译形成两套语义不一致的产品。
- 先创建源码、后补 CI，导致未经真实平台验证的骨架进入 `main`。
- 为追求速度引入 TypeScript、JavaScript、WebView、广告、遥测或上游旧运行面。

## 二、本次批准范围

### 允许

- 创建 Issue `#17` 的主计划、Session Plan、Runtime Workflow 和初始报告。
- 更新 `README.md`、Master Plan、总架构方案、Rust CI 实施计划、`build.md` 与 `err.md` 中的当前状态或既有排错引用。
- 把 Issue `#14` / PR `#15` 已完成的动态证据回写到 Gate 2 交付报告。
- 创建文档规划 PR，并通过 Review、现有只读 CI 和 Squash Merge 门禁收口。

### 禁止

- 创建或修改任何产品 `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml`、`.rs` 或 Iced 文件。
- 创建 `.github/workflows/ci.yml`、发布 Workflow、安装包、签名、更新清单或 required check。
- 修改 `upstream/`、`source-lock.json`、机器维护的 Issue `#16`、Ruleset 或 AGOS 控制面。
- 迁移任何上游功能、数据格式、注入脚本、远程推荐列表或 Tauri/React 界面。
- 把本次规划批准解释为后续源码实现或合并授权。

## 三、方案比较与决定

### 方案 A：直接创建完整 Workspace 与最小窗口

优点是最快得到可编译项目；缺点是 crate 边界、Iced 特性、平台端口和 CI 失败语义会在未经评审时成为事实，返工成本最高，不采用。

### 方案 B：合同先行，规划与实现拆分

先由 Issue `#17` 冻结设计和门禁；规划 PR Squash Merge 后，再创建独立实现 Issue，把 Workspace、最小双平台窗口和首版三平台 CI 放在同一个实现 PR 中。该方案多一个文档 PR，但范围最可审查，采用。

### 方案 C：只补概念 ADR

改动最少，但没有任务级允许路径、验证命令、Runtime Workflow 和 GitHub 交付证据，无法直接驱动实现，不采用。

## 四、后续 Workspace 合同

后续实现 Issue 的 Workspace 成员固定为：

```text
apps/inputcodex-desktop
crates/inputcodex-domain
crates/inputcodex-application
crates/inputcodex-infrastructure
crates/inputcodex-platform
crates/inputcodex-presentation
crates/inputcodex-parity
```

`upstream/`、`parity/` 文档数据、`benchmarks/` 场景文件和未来 `xtask/` 不得被隐式通配加入 Workspace。首版不创建没有立即用途的 `xtask` crate；需要时另以具体 Issue 证明用途。

### 4.1 依赖方向

```text
inputcodex-presentation(Iced) -> inputcodex-application -> inputcodex-domain
inputcodex-infrastructure     -> inputcodex-application
inputcodex-platform           -> inputcodex-application
inputcodex-parity             -> inputcodex-application + inputcodex-domain
inputcodex-desktop            -> presentation + application + infrastructure + platform
```

- `domain` 只允许 Rust 标准库和经单独评审的纯领域依赖，不依赖异步运行时、I/O、操作系统或 UI。
- `application` 定义用例、端口、加载状态和稳定错误语义；不得直接访问文件、SQLite、HTTP、进程或系统 API。
- `infrastructure` 与 `platform` 只能实现 application 端口，不得依赖 presentation。
- `presentation` 只依赖 application 和 Iced，不直接依赖 infrastructure、platform 或上游快照。
- `desktop` 只负责依赖组装、进程入口和生命周期；业务规则进入其他 crate。
- `parity` 只服务 application/domain 行为合同、上游映射和差异证据，默认不链接进桌面发布二进制。
- 反向依赖、环依赖、Iced 越层和 `upstream/` 入图必须由机器检查失败关闭。

### 4.2 Iced 隔离

- Iced 精确候选版本锁定为 `0.14.0`，官方 Release 发布于 `2025-12-07`，crates.io 标记未撤回，许可证为 MIT，声明的最低 Rust 版本为 `1.88`。
- 后续实现必须在创建 `Cargo.lock` 前 Fresh 复核版本、checksum `000e01026c93ba643f8357a3db3ada0e6555265a377f6f9291c472f6dd701fb3`、许可证和所选 feature；不得使用通配版本或浮动 Git 依赖。
- `inputcodex-presentation` 是唯一可直接依赖 `iced` 的 crate；`inputcodex-desktop` 只能调用 presentation 暴露的启动入口，不得导入 Iced 类型。
- 最小窗口只证明双平台事件循环、状态更新和关闭生命周期，不建立颜色、排版、组件或交互设计系统。
- 最小窗口的视觉与交互实现或审阅默认交给 Gemini；当前执行者只负责非 UI 架构、合同和集成边界。
- 禁止启用 Web、WebGL、WebView 或与当前桌面骨架无关的 Iced feature。精确 feature 集必须在实现 Issue 中以三平台编译证据确定。

### 4.3 Rust 工具链

- 规划时官方 stable 为 `Rust 1.97.1 (8bab26f4f 2026-07-14)`；后续实现候选工具链固定为 `1.97.1`，不得写浮动 `stable`。
- 实现 Issue 创建 `rust-toolchain.toml` 前必须 Fresh 复核该版本仍可由标准 GitHub-hosted runners 安装，且 Iced `0.14.0` 的 MSRV 合同未变化。
- 初始组件仅允许 `rustfmt` 与 `clippy`；额外 target、component 或 Cargo 工具必须说明用途、版本、许可证和资源成本。
- 根 Workspace 使用显式成员、resolver、统一 edition、license、repository 和依赖版本；`Cargo.lock` 随桌面应用提交。

## 五、平台抽象合同

- application 定义平台端口和稳定错误类别；platform 按目标系统提供 Windows、macOS 与非发布目标的明确实现。
- Windows 与 macOS 必须共享同一 application 用例和返回语义；条件编译只允许位于 platform 或极薄的启动适配层。
- Linux Runner 只承担质量与依赖图验证，不构成 Linux 产品支持承诺；若需要非发布目标占位实现，必须明确返回 unsupported，不得伪造成功。
- Gate 3 只证明平台边界和最小生命周期，不实现应用路径修改、进程注入、远程列表、安装、更新或其他业务平台能力。
- 平台错误必须转换为稳定类型并保留可诊断上下文；日志不得包含密钥、Token、用户文件内容或隐蔽遥测。

## 六、加载、取消与可诊断性

- application 首版只建立可复用的加载状态合同：`Idle / Loading / Ready / Empty / Failed / Cancelling`。
- 每次加载必须有请求标识；过期结果不能覆盖新状态，取消后结果必须失效。
- presentation 不执行磁盘、网络、SQLite、压缩、进程或系统调用；这些能力只能经 application 端口调用。
- 后台任务必须有界，禁止无上限 spawn、隐式轮询或重复刷新。
- Gate 3 不拍脑袋设定最终性能预算；只记录冷构建耗时、最小窗口启动测量方法、依赖数量和二进制体积的初始证据。上游/半成品/inputcodex 可比运行时预算在 Gate 4 性能 Issue 中批准。
- 可诊断输出只用于本地故障证据，不联网回传；任何遥测或远程日志提议都必须单独建立 Issue，默认拒绝。

## 七、本地与云端验证合同

### 7.1 本地轻量验证

后续实现默认只在本地运行：

```powershell
cargo fmt --all -- --check
cargo metadata --no-deps --format-version 1
cargo check -p inputcodex-domain
cargo test -p inputcodex-domain
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
```

不得默认在牢大的机器上运行全 Workspace、三平台、安装包或 Release 构建。需要扩大本地命令前必须说明成本并获得批准。

### 7.2 首版 GitHub Actions

Workspace 与首版 CI 必须位于同一个后续实现 Issue/PR。Workflow 名称固定为 `CI`，汇总 Job 固定为 `required`，但在稳定观测前不得加入 Ruleset。

| Job | Runner | 上限 | 职责 |
| --- | --- | --- | --- |
| `classify` | `ubuntu-latest` | 5 分钟 | 路径分类，不执行构建 |
| `governance` | `ubuntu-latest` | 10 分钟 | 依赖方向、许可证、广告、JS/WebView、更新源和快照隔离 |
| `linux-quality` | `ubuntu-latest` | 30 分钟 | fmt、Clippy、Workspace 单元测试 |
| `windows` | `windows-latest` | 45 分钟 | Workspace 测试、桌面包编译、Windows 合同 |
| `macos` | `macos-latest` | 45 分钟 | Workspace 测试、桌面包编译、macOS 合同 |
| `required` | `ubuntu-latest` | 5 分钟 | `if: always()` 汇总，非法跳过、失败或取消均失败 |

- 只使用标准 GitHub-hosted runners；禁止 Larger Runner、self-hosted runner 和收费资源。
- PR 顶层权限固定为 `contents: read`，不读取发布密钥，不写 Issue，不构建正式安装包。
- 首版不使用 Cargo Cache，以真实冷构建数据决定后续独立 Cache Issue。
- 成功默认不上传 Artifact；失败只上传显式白名单、脱敏且最多保留 7 天的报告，禁止上传整个 `target/`。
- 第三方 Action 必须固定不可变提交；Rust 工具链优先直接由 `rustup` 安装精确版本，减少额外 Action。

## 八、后续实现 Issue 准入

规划 PR Squash Merge 后，只能在以下条件同时满足时创建 Gate 3 实现 Issue：

1. Master Plan 已指向本计划，Issue `#17` 完成 Review 根因闭环并关闭。
2. 项目所有者明确批准“创建 Workspace 与首版三平台 CI”，该批准不能从本次规划批准推导。
3. Fresh 复核上游仍为 `v1.2.41`，Ruleset 无漂移，Issue `#16` 状态正常。
4. Fresh 复核 Rust、Iced、系统依赖、许可证和 GitHub-hosted runner 可用性。
5. 实现 Issue 明确所有允许路径、RED/GREEN 证据、冷构建基线、失败语义、回滚和合并授权边界。

建议后续标题为 `[Architecture] Gate 3 纯 Rust Workspace 与首版三平台 CI 实现`，分支使用 `codex/issue-<真实编号>-gate-3-rust-workspace-ci`。真实编号必须以 GitHub 返回为准。

## 九、本规划 PR 允许路径

```text
README.md
build.md
err.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/2026-07-21-architecture-governance.md
docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
docs/plans/sessions/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
docs/workflows/2026-07-22-issue-17-gate-3-rust-workspace-plan-runtime.md
docs/reports/issue-17-gate-3-rust-workspace-plan.md
docs/reports/issue-14-gate-2-upstream-watch.md
```

## 十、完成定义

- Issue、分支、主计划、Session Plan、Runtime Workflow、报告和 Master Plan 引用一致。
- Gate 2 最终合并、两次真实运行、Issue `#16` 唯一状态和签名证据不再保留 `pending`。
- 本 PR 只有上述文档路径；仓库继续没有产品 Cargo/Rust/Iced 文件或新的产品 Workflow。
- 本地文档合同、路径门禁和 `git diff --check` 通过。
- PR Review 对话全部完成根因、处理、验证和确认闭环；现有 CI 成功后等待项目所有者新的 Squash Merge 授权。

## 十一、停止与回滚

- 若上游正式 Release、标签提交、Ruleset 或机器状态发生物质变化，停止并建立对应同步、事故或决策 Issue。
- 若 Rust/Iced 精确版本、许可证或平台依赖无法 Fresh 复核，后续实现保持锁定，不以浮动版本绕过。
- 若需要实现 UI、功能、更新、安装、注入、远程列表或 AGOS 修改，立即停止并拆分独立 Issue。
- 本规划 PR 未合并时可关闭 PR 并删除功能分支；合并后只能通过新的关联 Issue/PR `revert`，禁止 Force Push 或改写 `main` 历史。
