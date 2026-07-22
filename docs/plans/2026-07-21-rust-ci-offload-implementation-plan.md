# Rust CI 云端卸载实施计划

> **执行要求：** 每个任务先建立独立 Issue、Session Plan 与 Runtime Workflow，再创建分支和关联 PR。Issue `#17` 只冻结 Gate 3 实施合同，不创建 Workspace、Rust 源码或产品 CI；后续实现仍需新的 Issue 和项目所有者批准。

status: gate-3-implementation-completed-closeout-active
decision_date: 2026-07-21
owner: nonononull
owner_decision_ref: 当前任务对话中的“方案确认”, user-message:approve-gate-3-planning-2026-07-22, user-message:authorize-squash-merge-pr-21-2026-07-22
strategy_ref: docs/plans/2026-07-21-rust-ci-offload-strategy.md
current_scope: Gate 3 实现已通过 Issue #19 / PR #21 合并；Issue #22 只持久化 closeout 证据，Gate 4 与 Cache/Ruleset 调优仍需独立批准

**目标：** 在不占用项目所有者本地机器承担全量 Rust 编译的前提下，分阶段建立每 6 小时上游监控、纯 Rust/Iced 双平台工作区、标准 GitHub-hosted runners 全量 CI，以及证据充分后的 `main` required check。

**架构：** Gate 2 先用单独的 `ubuntu-latest` 工作流读取上游元数据并管理 GitHub Issue，不编译 Rust、不修改仓库内容。Gate 3 将 Cargo Workspace、最小双平台骨架和首版 Linux/Windows/macOS CI 放在同一个 Issue/PR 中，使新骨架首次进入仓库前就由云端真实平台验证；缓存优化和 Ruleset 升级再使用独立 Issue/PR，避免把不稳定 Job 名称提前写进保护规则。

**技术栈：** Rust、Cargo、Iced、PowerShell 7、GitHub Actions YAML、GitHub CLI/API、标准 `ubuntu-latest`、`windows-latest`、`macos-latest` Runner。

## 全局约束

- 软件名称固定为 `inputcodex`。
- 产品业务代码只使用 Rust；禁止 TypeScript、JavaScript 业务代码和 WebView。
- Iced 只允许出现在展示层；领域、应用、基础设施、平台层不得依赖 Iced 类型。
- Windows 与 macOS 从首个可运行骨架起保持功能一致。
- 本地默认只执行格式检查、定向 `cargo check/test` 和问题复现；Workspace 全量、双平台与发布构建交给标准 GitHub-hosted runners。
- 禁止 Larger Runner 和项目所有者本机 self-hosted runner；改变此约束必须建立新 Issue 并取得项目所有者批准。
- Fork PR 和普通 PR 只获得 `contents: read`；不得获得 Issue 写权限、发布权限、签名密钥或更新密钥。
- 上游监控只管理 Issue，不自动提交、同步、合并、发布或编译 Rust。
- 第三方 Action 必须核对许可证、维护状态并固定到不可变提交；优先使用 GitHub 官方 Action。
- 所有 Job 必须有有限超时和稳定失败语义；失败、超时、取消或无证据跳过都不能解释为通过。
- 禁止上传整个 `target/`；非 Release Artifact 最长保留 7 天。
- 每个任务都必须执行 `Issue → 分支 → 验证证据 → 关联 PR → Review/CI → Squash Merge`，禁止直接写 `main`、Force Push 或删除 `main`。
- 所有 Review 对话必须先记录根因、处理方式与验证证据，再由 reviewer 或项目所有者确认解决。
- Gate 2 已完成；Issue `#17` 的 Gate 3 规划活动，但 Task 2 的 Workspace 与 CI 实现仍锁定，直到规划 PR Squash Merge 且项目所有者批准新的实现 Issue。

## 交付拆分

| 顺序 | 独立 Issue 标题 | 建议分支 | 单一交付物 | 准入条件 |
| --- | --- | --- | --- | --- |
| 1 | `[Gate 2] 建立每 6 小时上游 Release 与 main 变化监控` | `codex/issue-14-gate-2-upstream-watch` | 已通过 PR `#15` 完成，只读上游、只写 Issue 的监控工作流 | 已完成 |
| 2 | `[Gate 3] 冻结纯 Rust Workspace 骨架规划` | `codex/issue-17-gate-3-rust-workspace-plan` | 仅规划、Session、Runtime、报告和项目入口 | 已批准规划；禁止源码实现 |
| 3 | `[Gate 3] 建立纯 Rust Workspace 与三平台首版 CI` | `codex/issue-<真实编号>-gate-3-rust-workspace-ci` | 分层 Workspace、最小双平台骨架、无缓存首版 CI | Issue `#17` 规划 PR 已合并；项目所有者批准新实现 Issue |
| 4 | `[Gate 3] 基于冷构建证据优化 Cargo Cache 与诊断 Artifact` | `codex/gate-3-ci-cache-tuning` | 有测量证据的缓存配置或明确维持无缓存的报告 | 至少 10 次源码影响运行且覆盖三个 Runner；观测期不少于 7 天 |
| 5 | `[Gate 3] 将稳定 CI 汇总检查加入 main-protection` | `codex/gate-3-required-check` | Ruleset required check 与落地报告 | Job 名称、触发和失败语义稳定；项目所有者再次批准 |

Issue `#17` 的规划 PR 不包含 Workspace。规划合并后，Gate 3 的 Workspace 与首版 CI 必须处于同一个实现 PR：Workspace 没有云端三平台证据不能合并，CI 没有真实 Workspace 也无法被验收。缓存调优与 Ruleset 升级可以被独立拒绝，因此必须拆成后续 PR。

## 文件责任地图

### 当前计划 PR 只修改

- `docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md`：未来实施任务、接口、验证和回滚的单一执行计划。
- `docs/plans/2026-07-21-rust-ci-offload-strategy.md`：增加实施计划引用，不改变已批准策略。
- `docs/plans/PROJECT-MASTER-PLAN.md`：登记实施计划并保持 Gate 2/3 锁定。
- `docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md`：记录本次所有者确认与计划证据。
- `docs/workflows/2026-07-21-issue-2-architecture-governance-runtime.md`：增加计划编写窗口和验证门。
- `docs/plans/2026-07-21-architecture-governance.md`：把 Gate 3 的执行入口指向本计划。
- `build.md`：增加当前 docs-only 范围和计划一致性检查。

### Gate 2 未来创建

- `.github/workflows/upstream-monitor.yml`：定时与手动触发、最小权限、并发和超时。
- `scripts/upstream-monitor/UpstreamMonitor.psm1`：纯函数比较 Release、main、source lock 与现有 Issue 状态。
- `scripts/upstream-monitor/Invoke-UpstreamMonitor.ps1`：调用 GitHub API 并执行 Issue 创建、更新或关闭。
- `scripts/upstream-monitor/Test-UpstreamMonitor.ps1`：使用本地夹具运行无网络合同测试。
- `scripts/upstream-monitor/fixtures/source-lock.json`：已同步基线夹具。
- `scripts/upstream-monitor/fixtures/latest-release.json`：正式 Release API 夹具。
- `scripts/upstream-monitor/fixtures/main-ref.json`：上游 `main` 引用 API 夹具。
- `scripts/upstream-monitor/fixtures/issues.json`：已有监控 Issue 夹具。
- `docs/upstream/monitor-contract.md`：Issue 标记、状态转换、失败语义和人工处置入口。
- `build.md`、`err.md`：监控测试、手动运行和排错证据。

### Gate 3 未来创建

- `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml`：固定 Workspace、依赖锁和 Rust 工具链。
- `apps/inputcodex-desktop/Cargo.toml`、`apps/inputcodex-desktop/src/main.rs`：只负责组装与生命周期的桌面入口。
- `crates/inputcodex-domain/Cargo.toml`、`crates/inputcodex-domain/src/lib.rs`：零 UI、零平台、零网络的领域层。
- `crates/inputcodex-application/Cargo.toml`、`crates/inputcodex-application/src/lib.rs`：用例与端口接口。
- `crates/inputcodex-infrastructure/Cargo.toml`、`crates/inputcodex-infrastructure/src/lib.rs`：外部适配骨架。
- `crates/inputcodex-platform/Cargo.toml`、`crates/inputcodex-platform/src/lib.rs`：Windows/macOS 平台端口实现骨架。
- `crates/inputcodex-presentation/Cargo.toml`、`crates/inputcodex-presentation/src/lib.rs`：唯一允许依赖 Iced 的展示层。
- `crates/inputcodex-parity/Cargo.toml`、`crates/inputcodex-parity/src/lib.rs`：功能合同与上游映射骨架。
- 各 `apps/` 与 `crates/` 子项目根目录的 `build.md`、`err.md`：满足可单独构建项目的文档硬约束。
- `scripts/ci/Classify-Changes.ps1`：确定是否需要运行 Rust 重型 Job。
- `scripts/ci/Verify-RepositoryPolicy.ps1`：验证分层、禁止 WebView/JS 业务代码、更新源和广告边界。
- `scripts/ci/Test-CiScripts.ps1`：使用临时目录和夹具验证分类与治理脚本。
- `.github/workflows/ci.yml`：PR、`main` 和手动触发的稳定 CI。
- `docs/reports/rust-ci-cold-baseline.md`：三个 Runner 的无缓存基线、超时选择和失败分类。

## Task 1：Gate 2 上游监控

**接口：**

- `Get-UpstreamDecision([pscustomobject]$SourceLock, [pscustomobject]$LatestRelease, [pscustomobject]$MainRef, [pscustomobject[]]$OpenIssues)` 返回 `NoChange`、`ReleaseAvailable`、`MainAdvanced` 或 `InvalidEvidence`。
- `Invoke-UpstreamIssueSync([pscustomobject]$Decision, [string]$Repository)` 的 `$Repository` 固定为 `nonononull/inputcodex`，只允许创建、编辑、评论或关闭带固定机器标记的监控 Issue。
- Release Issue 机器标记固定为 `<!-- inputcodex-upstream-release-monitor:v1 -->`；main 预警标记固定为 `<!-- inputcodex-upstream-main-monitor:v1 -->`。
- 最新正式 Release 是功能真源；`main` 变化只产生预警，不能自动改变 `upstream/source-lock.json` 或功能基线。

- [ ] **步骤 1：建立 Gate 2 Issue 和执行控制面**

  Issue 正文写明跟踪基线、允许写入路径、`issues: write` 原因、禁止 Rust 编译和回滚方式；创建真实 Issue 后再生成包含真实编号的 Session Plan 与 Runtime Workflow。

- [ ] **步骤 2：先写无网络失败合同**

  `scripts/upstream-monitor/Test-UpstreamMonitor.ps1` 必须先覆盖：相同 Release 无写入、新正式 Release 创建一次 Issue、重复运行更新同一 Issue、main 前进只更新预警 Issue、API JSON 缺字段失败、非本仓机器标记 Issue 不得修改。

  运行：`pwsh -NoProfile -File scripts/upstream-monitor/Test-UpstreamMonitor.ps1`

  预期：在模块实现前退出非零，并明确报告缺少 `Get-UpstreamDecision`。

- [ ] **步骤 3：实现纯比较模块并使合同通过**

  模块不得访问网络、环境变量或 GitHub；全部输入通过参数传入，便于重复测试。

  运行：`pwsh -NoProfile -File scripts/upstream-monitor/Test-UpstreamMonitor.ps1`

  预期：全部本地夹具合同通过，重复输入产生相同决策。

- [ ] **步骤 4：实现 GitHub Issue 适配器**

  适配器只读取 `GITHUB_TOKEN` 和 GitHub 上下文；对 tag、SHA、Issue 编号和机器标记做白名单验证，禁止把上游响应拼接成可执行 Shell。网络失败、权限失败和响应结构变化必须退出非零。

- [ ] **步骤 5：创建最小权限 Workflow**

  `.github/workflows/upstream-monitor.yml` 仅使用 `schedule` 与 `workflow_dispatch`，计划表达式固定为 `17 */6 * * *`；Runner 固定 `ubuntu-latest`，权限固定 `contents: read` 与 `issues: write`，并发组固定 `upstream-monitor`，`cancel-in-progress: true`，超时固定 10 分钟。禁止 `pull_request_target`、Rust 安装、Cargo 命令、Artifact 和仓库写入。

- [ ] **步骤 6：验证权限与幂等性**

  手动运行两次相同输入，第二次不得创建重复 Issue；使用临时失效仓库引用验证 Job 失败且不修改无关 Issue；确认运行日志不包含 Token 或完整请求头。

- [ ] **步骤 7：更新文档并提交 PR**

  更新 `build.md`、`err.md`、`docs/upstream/monitor-contract.md` 和 Master Plan；PR 正文关联 Gate 2 Issue，并附两次幂等运行链接、权限截图或 API 输出及失败合同证据。

  提交信息：`ci: 建立上游变化监控`

**回滚：** 通过新 PR 删除或禁用 `.github/workflows/upstream-monitor.yml`，保留历史 Issue 和报告；不得 Force Push、删除 `main` 或自动回退 `source-lock.json`。

## Task 2：Gate 3 Workspace 与首版三平台 CI（Issue #19 实现活动）

**接口：**

- Workspace 成员固定为 `apps/inputcodex-desktop` 与六个 `crates/inputcodex-*` 包；`upstream/` 明确排除在 Workspace 和打包输入之外。
- 依赖方向固定为 `presentation → application → domain`，`infrastructure` 与 `platform` 实现 application 端口；反向依赖检查失败必须阻止合并。
- CI 工作流名称固定为 `CI`，稳定汇总 Job 名称固定为 `required`，GitHub 检查上下文预期为 `CI / required`。
- 重型路径固定包括根 Cargo 文件、`rust-toolchain.toml`、`apps/**`、`crates/**`、`xtask/**`、`benchmarks/**`、`parity/**`、`scripts/ci/**` 和 `.github/workflows/ci.yml`。

- [x] **步骤 1：建立 Gate 3 实现 Issue、工具链决策和执行控制面**

  Issue `#17` / PR `#18` 已完成规划 Squash Merge；项目所有者以 `user-message:approve-gate-3-implementation-2026-07-22` 批准实现。Issue `#19`、分支 `codex/issue-19-gate-3-rust-workspace-ci`、Session Plan、Runtime Workflow 和初始报告已建立。Fresh 候选固定为 Rust `1.97.1` 与 Iced `0.14.0`，Iced checksum、MIT、MSRV `1.88` 和未撤回状态已复核；本地资源边界仍为轻量定向验证。禁止使用浮动 `stable`、通配依赖版本或未锁定依赖。Iced 展示文件由 Gemini 负责实现或审阅，当前执行者不得自行确立视觉设计系统。

- [x] **步骤 2：先写分层和治理失败测试**

  `scripts/ci/Test-CiScripts.ps1` 先构造违规夹具，证明以下情况会失败：领域层依赖 Iced、生产目录出现 `.ts/.js` 业务文件、Cargo 依赖包含 WebView/Tauri、更新地址不属于 `nonononull/inputcodex`、`upstream/` 被加入 Workspace、广告或遥测依赖进入生产清单。

  运行：`pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1`

  预期：治理脚本尚未实现时退出非零。

- [x] **步骤 3：创建最小分层 Workspace**

  每个 crate 只建立其职责所需的最小公开接口和单元测试，不迁移上游功能、不引入数据库、网络、更新、广告、注入或远程推荐实现。`apps/inputcodex-desktop` 只组装依赖并打开最小 Iced 窗口；窗口之外不确立视觉规范。

- [x] **步骤 4：补齐所有子项目构建与排错文档**

  每个可单独构建的 app/crate 根目录创建 `build.md` 与 `err.md`，写明定向 `cargo check -p 包名`、单元测试和已知平台依赖；根 `build.md` 记录本地轻量命令与云端全量命令的责任边界。

- [x] **步骤 5：实现分类与仓库治理脚本**

  `Classify-Changes.ps1` 只解析 Git diff 路径并输出布尔结果，不执行构建；`Verify-RepositoryPolicy.ps1` 解析 Workspace 清单和受控源目录。脚本必须对路径穿越、空 diff、重命名和删除场景有确定结果。

- [x] **步骤 6：运行本地轻量验证**

  运行：

  ```powershell
  cargo fmt --all -- --check
  cargo check -p inputcodex-domain
  cargo test -p inputcodex-domain
  pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
  ```

  预期：本地只验证最小领域包和 CI 脚本，不运行全 Workspace 或双平台安装包构建。

- [x] **步骤 7：创建无缓存首版 CI**

  `.github/workflows/ci.yml` 只监听目标为 `main` 的 `pull_request`、`main` 的 `push` 与 `workflow_dispatch`，避免同一功能分支同时由 push 和 PR 重复跑全量。顶层权限固定 `contents: read`；并发键按 PR 编号或 ref 隔离，`cancel-in-progress: true`。

  Job 固定如下：

  | Job ID | 显示名称 | Runner | 超时 | 职责 |
  | --- | --- | --- | --- | --- |
  | `classify` | `classify` | `ubuntu-latest` | 5 分钟 | 路径分类，不读取密钥 |
  | `governance` | `governance` | `ubuntu-latest` | 10 分钟 | CI 脚本测试、许可证、架构、广告、WebView/JS、更新源检查 |
  | `linux-quality` | `linux-quality` | `ubuntu-latest` | 30 分钟 | `fmt`、Clippy、Workspace 单元测试 |
  | `windows` | `windows` | `windows-latest` | 45 分钟 | Workspace 测试、桌面包编译、Windows 平台合同 |
  | `macos` | `macos` | `macos-latest` | 45 分钟 | Workspace 测试、桌面包编译、macOS 平台合同 |
  | `required` | `required` | `ubuntu-latest` | 5 分钟 | `if: always()` 汇总全部前置结果，非法跳过或失败均退出非零 |

  PR 的非重型文档改动可以跳过三个编译 Job，但 `governance` 与 `required` 必须始终运行；`main` push 与手动运行必须执行全部 Job。首版禁止 Cargo Cache，用真实冷构建数据填写 `docs/reports/rust-ci-cold-baseline.md`。

- [x] **步骤 8：约束 Artifact 与日志**

  成功运行默认不上传 Artifact。失败时只允许上传测试报告、治理报告和经过脱敏的命令日志，`retention-days: 7`；上传路径必须显式列举，禁止 `target/`、Cargo registry、Git 凭据、环境转储和整个工作区。

- [x] **步骤 9：验证真实三平台失败语义**

  在 PR 分支使用普通提交分别制造并修复格式、Rust 编译、Windows 条件编译、macOS 条件编译和治理违规；每次保留失败运行链接，再用后续普通提交修复。禁止 Force Push 或只靠重新运行掩盖失败。

  已完成：治理 `29913582488`→`29914029406`、rustfmt `29914734781`→`29915134906`、通用 Rust `29915537702`→`29915879951`、Windows `29916309635`→`29916670916`、macOS `29917061781`→`29917649550`；五类失败均先确定根因，再由普通修复提交恢复全绿。

- [x] **步骤 10：记录冷构建基线并提交 PR**

  报告记录每个 Job 至少三次无缓存运行的耗时、排队时间、成功/失败原因和所选超时依据；PR 必须等待 Linux、Windows、macOS 和 `required` 成功，所有 Review 对话完成根因闭环后才能 Squash Merge。

  已完成：运行 `29911337652`、`29913139948`、`29914029406` 使三平台各达到 `3/3`；Job 执行时间中位数为 Linux `133` 秒、Windows `212` 秒、macOS `96` 秒，报告保留无 Cache 边界。Draft PR `#21` 已创建，最终 Squash Merge 仍需新的项目所有者授权。

  提交信息：`build: 建立 Rust 工作区与三平台 CI`

**回滚：** 若 Workspace 骨架有根因缺陷，使用关联 Issue/PR 的 `revert` 恢复合并提交；若只发生 CI 配置故障，使用新 PR 修复或临时禁用对应 Workflow。不得删除 `main`、Force Push 或以关闭平台 Job 冒充恢复。

### Gate 3 实施交付结果

- PR `#21` 最终 Head `9a4a4425f2fb0d8235554d3e83577111ae34efcc` 的运行 `29918843397` 六 Job 全绿，成功 Artifact 数为 `0`。
- PR `#21` 于 `2026-07-22T12:25:59Z` Squash Merge 为 `0716ec0debcd3e059cc4ca88a072232841ca73b4`，唯一父提交为 `477d110a9b284e127af365f5278901bcfa69e093`。
- Merge tree 与最终 Head tree 均为 `4881ce609370f77181d9545474c029ab0c5d4972`，GitHub 签名 `valid`；Issue `#19` 已按 `COMPLETED` 关闭。
- 合并后 `main` 运行 `29919596057` 六 Job 全绿且成功 Artifact 数为 `0`。
- Gate 3 实施步骤 `1–10` 全部完成；Cache、P95、Ruleset required check 和 Gate 4 继续使用独立 Issue/PR。

## Task 3：Cache 与诊断 Artifact 调优

**准入证据：** 至少 10 次会触发 Rust 重型 Job 的运行，覆盖 Linux、Windows、macOS，观测期不少于 7 天；每个平台至少有三次无缓存成功样本，并已排除 Runner 或 GitHub 服务事故样本。

- [ ] **步骤 1：建立独立调优 Issue 和基线报告**

  Issue 逐平台记录中位数、95 分位、依赖下载耗时、编译耗时、失败率和当前存储占用；不能用单次最快结果作为决策。

- [ ] **步骤 2：先验证 Cache key 隔离**

  为操作系统、精确 Rust 工具链、构建配置和 `Cargo.lock` 变化编写 key 合同；Windows、macOS、Linux 之间禁止交叉恢复，锁文件变化只能使用受限 restore prefix。

- [ ] **步骤 3：只缓存低风险 Cargo 下载目录**

  首个候选只允许 Cargo registry index/cache 与 Git dependency database；仍禁止缓存或上传整个 `target/`。使用的官方 Cache Action 固定到不可变提交，并在 PR 中记录来源、许可证与提交 SHA。

- [ ] **步骤 4：以数据决定保留或撤销**

  只有三个平台各自的中位总时长至少下降 15%，且没有新增错误命中、依赖污染或失败率上升，才保留 Cache。未达到门槛时，PR 只提交测量报告并明确维持无缓存，不把无效配置合并进 Workflow。

- [ ] **步骤 5：验证 Artifact 最小化**

  故意触发一次测试失败，确认只上传白名单日志且 7 天后自动过期；下载 Artifact 检查不存在密钥、环境变量转储、用户路径和 `target/`。

  提交信息：`ci: 基于基线优化构建缓存`

**回滚：** 通过新 PR 删除 Cache 步骤并恢复无缓存 CI；保留调优报告和失败证据，禁止清除历史来掩盖错误命中。

## Task 4：将稳定汇总检查加入 Ruleset

**准入证据：** `CI / required` 已在不少于 10 次源码影响 PR 运行中保持名称一致，观测期不少于 7 天；Windows 与 macOS 均有真实成功和真实失败后修复证据；不存在未关闭的不稳定测试或错误跳过 Issue。

- [ ] **步骤 1：建立 Ruleset 升级 Issue**

  Issue 关联所有样本运行、冷构建报告、Cache 决策、失败语义和回滚命令；项目所有者必须在 Issue 或 PR 明确批准把 `CI / required` 设为 required check。

- [ ] **步骤 2：证明汇总 Job 不能伪通过**

  分别让 `governance`、`linux-quality`、`windows` 和 `macos` 失败，确认 `required` 同步失败；仅文档 PR 中三个编译 Job 合法跳过时，确认 `required` 仍要求 `governance` 成功并能解释跳过原因。

- [ ] **步骤 3：更新 `main-protection` Ruleset**

  只新增检查上下文 `CI / required`，保留 Ruleset ID `19395456` 的删除保护、非快进保护、PR 门禁、Review 对话解决、Squash-only、当前审批人数和空 bypass actor。不得用新 Ruleset 替换旧规则或扩大到其他分支。

- [ ] **步骤 4：验证有效规则和合并阻塞**

  使用 GitHub API 读取 Ruleset 和 `main` 有效规则；在测试 PR 中确认检查运行中、失败和取消都会阻止合并，成功后才解除阻塞。普通 PR、Fork PR 和文档 PR 都必须得到确定状态，不能永久等待不存在的检查。

- [ ] **步骤 5：写入落地报告并提交 PR**

  创建 `docs/reports/rust-ci-required-check-rollout.md`，更新 Master Plan、`build.md` 和 `err.md`；PR Review 对话全部完成根因闭环后才允许 Squash Merge。

  提交信息：`chore: 将稳定 CI 加入 main 保护`

**回滚：** 若 `CI / required` 因工作流重命名或 GitHub 故障造成所有 PR 永久阻塞，先建立事故 Issue，再通过受审 PR/API 仅移除该 required context；保留其他 Ruleset 规则，不得启用 bypass、Force Push 或删除 `main`。

## Release 边界

正式发布、签名、安装包、更新清单和 GitHub Release 不属于本实施计划。Gate 5 功能范围和 Gate 6 发布合同稳定后，必须另写 Release 实施计划和独立 Issue/PR；Release Workflow 与普通 CI 分离，只有受保护自主版本标签和受保护环境能够访问密钥，Fork/普通 PR 永远不能访问。

## 每个任务的统一完成定义

- 真实 Issue、分支、Session Plan、Runtime Workflow、PR 和验证链接齐全。
- 所有修改路径均在该 Issue 批准范围内，没有顺手重构。
- 本地只运行任务定义的轻量或复现命令；云端全量证据来自标准 GitHub-hosted runners。
- 每个失败先查 `err.md`，确定根因并记录修复与验证，不通过盲目重跑关闭问题。
- 所有 Review 对话均有根因、处理和验证证据；未解决对话为零。
- PR 使用 Squash Merge，合并后删除功能分支；禁止 Force Push 和删除 `main`。
- 形成新稳定面后使用项目原生 Git 状态、HEAD 与 diff 保存可复核快照；正式 closeout 补齐 `review_ref`、`ci_ref`、`merge_ref`。如外部 AGOS 可用且适用，可补充记录 rollout；不可用时绕过，不影响 closeout，且禁止在本项目任务中优化 AGOS。

## Issue #17 规划 PR 的验收

- 本文件、CI 策略、Master Plan、Issue `#17` 主计划、Session Plan、Runtime Workflow、总架构方案和 `build.md` 互相引用且语义一致。
- 当前仓库只保留既有 `.github/workflows/upstream-watch.yml`，不存在产品 `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml` 或非快照 `.rs` 文件。
- Issue `#17` 只创建规划 Issue、分支和文档；没有创建 Gate 3 实现 Issue、产品 Workflow、Ruleset required check 或发布配置。
- Gate 2 已完成；Gate 3 实现、Cache 调优和 Ruleset 升级分别使用后续独立 Issue/PR、验证、失败语义与回滚路径。
- 规划 PR 只有在项目所有者完成 Review、所有对话根因闭环、现有 CI 成功并提供新的明确授权后才允许 Squash Merge。
