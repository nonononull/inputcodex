# Session Plan：Issue #19 Gate 3 纯 Rust Workspace 与首版三平台 CI

schema_version: inputcodex.session-plan.v1
task_id: 2026-07-22-issue-19-gate-3-rust-workspace-ci
work_class: major
task_status: first-ci-real-red-root-cause-fix-in-progress
task_summary: 按已批准的 Gate 3 合同建立七成员纯 Rust Workspace、Iced 展示层隔离、最小加载/平台语义、治理脚本与首版无缓存三平台 CI，不迁移任何上游业务功能。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/19
planning_issue_ref: https://github.com/nonononull/inputcodex/issues/17
planning_pr_ref: https://github.com/nonononull/inputcodex/pull/18
branch_ref: codex/issue-19-gate-3-rust-workspace-ci
baseline_ref: 477d110a9b284e127af365f5278901bcfa69e093
decision_status: approved-implementation-merge-authorization-pending
approved_decision_ref: user-message:approve-gate-3-implementation-2026-07-22
session_plan_ref: docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md
implementation_plan_ref: docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
architecture_plan_ref: docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md
report_ref: docs/reports/issue-19-gate-3-rust-workspace-ci.md
scope_hash: sha256:2e101627480012d57d6d0472a08cfbe03fc401f6ac74ef3ae1e6a42929ed61ba
mutation_intent: 先提交项目原生控制面，再按 RED 治理合同、GREEN Workspace、三平台 CI、真实失败恢复和冷构建基线分批实现；任何批次不得越过 23 条允许路径模式。
executor_enforcement: 每个执行批次前后核对分支、允许路径、禁止依赖、测试状态和 Git 快照；本地只跑轻量定向验证，Workspace 与 Windows/macOS 全量验证交给标准 GitHub-hosted runners。
delivery_contract: inputcodex.issue-pr-merge.v1
external_agos_policy: available-optional-unavailable-bypass-no-optimization
external_agos_execution: needs-input-unregistered-recorded-and-bypassed
red_contract_status: verified
red_contract_exit_code: 10
red_contract_marker: CI_CONTRACT_RED_MISSING_IMPLEMENTATION
red_checkpoint_ref: commit:67fe99457e1aa2717cc29c70d51114028d68dafd;issuecomment:5043557146
green_contract_status: verified
green_contract_passed: 23
green_checkpoint_ref: commit:be9259f55b32014e918113936e6e6ddfdd16765f;issuecomment:5043682396
governance_alignment_status: verified
governance_alignment_passed: 27
governance_alignment_checkpoint_ref: commit:2d8a1466ae42d6b208258bee3d0cb6bd5647bb12;issuecomment:5043770155
current_governance_contract_passed: 29
workspace_status: seven-members-local-light-green
workspace_member_count: 7
lock_package_count: 336
lock_external_package_count: 329
local_toolchain_evidence: rust-1.93.1-ignore-rust-version-only
exact_toolchain_evidence: pending-github-actions
desktop_iced_compile: pending-github-actions
workspace_checkpoint_ref: commit:f93372fdc63cf8c628007117be4a8b222510957b;issuecomment:5044073911
ci_workflow_status: local-static-green-awaiting-github-actions
ci_checkpoint_ref: commit:f3107dd16705dd3a25bc8c3acc540a3c6c6990a3;issuecomment:5044470597

## 一、批准决策

- Decision：Issue `#17` / PR `#18` 的规划已 Squash Merge，项目所有者批准创建 Workspace 与首版三平台 CI。
- Decision：采用“RED 治理合同 → GREEN 最小分层 Workspace → 无缓存三平台 CI → 真实失败恢复 → 冷构建基线”的顺序，不做大爆炸式脚手架。
- Decision：当前批准覆盖本 Issue 的实现与验证，不包含最终 PR Squash Merge；合并前必须再次取得项目所有者明确授权。
- Decision：Iced 只允许直接存在于 `inputcodex-presentation`，最小窗口不形成 UI 设计系统；视觉与交互由 Gemini 实现或审阅。
- Decision：不迁移上游功能，不实现数据库、网络、安装、更新、注入、远程推荐、广告、推广或遥测。
- Reason：先让禁止表面和依赖方向以失败测试落地，才能阻止上游架构问题、Iced 越层和平台语义分叉进入第一个可编译稳定面。

## 二、Fresh 启动基线

- `HEAD`、`main` 与 `origin/main` 均为 `477d110a9b284e127af365f5278901bcfa69e093`，启动工作树干净。
- PR `#18` 已于 `2026-07-22T07:23:29Z` Squash Merge；Issue `#17` 已按 `COMPLETED` 关闭，合并提交单父、tree 等价且 GitHub 签名 `valid`。
- 上游正式 Release 仍为 `v1.2.41`，tag SHA 为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 上游 `main` 为 `ffb06bc83b539bee7ec2b1ee87636a5fe1a4850d`；相对规划基线仅修改 `docs/images/discussion-group-qr.jpg`，对本任务无物质影响。
- Issue `#16` 为唯一 OPEN 机器状态 Issue，最近成功观察为 `2026-07-22T04:15:38Z`；不人工编辑，后续变化由既有六小时 Workflow 分流。
- Ruleset `19395456` 为 active、无 bypass、required approvals `0`、必须解决 Review 对话并只允许 Squash Merge；具备合并权限的人类维护者仍只有 `nonononull`。
- 仓库无 `.codegraph/`，未初始化索引；当前尚无产品 Cargo、Rust、Iced 或产品 CI 文件。

## 三、Brainstorming 记录

```yaml
brainstorming:
  status: approved-existing-design-reused
  owner_intent:
    - 性能优先，同时保持上游最新正式 Release 的有效功能一致
    - 使用纯 Rust 与 Iced，禁止 TypeScript、JavaScript 业务代码和 WebView
    - Windows 与 macOS 从首版起共享功能和错误语义
    - 所有失败必须确定根因，所有 Review 对话必须闭环后才能合并
  approaches:
    - id: big-bang-workspace-and-ci
      decision: rejected
      reason: 同时创建全部源码与 Workflow，无法保留可复核 RED 证据，失败定位成本最高
    - id: governance-red-then-minimal-green
      decision: selected
      reason: 先机器化禁止表面，再建立最小分层骨架和三平台 CI，每批次都有独立验证与回滚点
    - id: cargo-only-scaffold-ci-later
      decision: rejected
      reason: 没有 Windows/macOS 与汇总 Job 的真实证据，无法证明首个稳定面跨平台一致
  approved_design_ref: docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
  approved_decision_ref: user-message:approve-gate-3-implementation-2026-07-22
```

## 四、允许操作

```yaml
allowed_operations:
  - 创建七成员 Cargo Workspace、精确 rust-toolchain.toml 与提交 Cargo.lock
  - 创建最小 domain/application/infrastructure/platform/presentation/parity/desktop 接口与测试
  - 创建 scripts/ci 的 RED/GREEN 合同、路径分类和仓库政策验证脚本
  - 创建无缓存的 Linux、Windows、macOS 与 required GitHub Actions
  - 为每个可单独构建 app/crate 创建 build.md 与 err.md
  - 运行本地轻量验证、标准 GitHub-hosted runner 全量验证和真实失败恢复
  - 记录冷构建、依赖数量、二进制体积和失败语义证据
  - 创建普通提交、普通 push、关联 PR、Review/CI 与 closeout 证据
  - 在获得新的明确授权后执行 Squash Merge
```

## 五、允许路径

```yaml
allowed_paths:
  - .gitignore
  - Cargo.toml
  - Cargo.lock
  - rust-toolchain.toml
  - .github/workflows/ci.yml
  - apps/inputcodex-desktop/**
  - crates/inputcodex-domain/**
  - crates/inputcodex-application/**
  - crates/inputcodex-infrastructure/**
  - crates/inputcodex-platform/**
  - crates/inputcodex-presentation/**
  - crates/inputcodex-parity/**
  - scripts/ci/**
  - README.md
  - build.md
  - err.md
  - docs/plans/PROJECT-MASTER-PLAN.md
  - docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
  - docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md
  - docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md
  - docs/reports/issue-17-gate-3-rust-workspace-plan.md
  - docs/reports/issue-19-gate-3-rust-workspace-ci.md
  - docs/reports/rust-ci-cold-baseline.md
allowed_path_count: 23
scope_hash: sha256:2e101627480012d57d6d0472a08cfbe03fc401f6ac74ef3ae1e6a42929ed61ba
```

## 六、禁止操作

- 不修改 `upstream/`、`source-lock.json`、Issue `#16`、Ruleset、Release、签名、安装包或外部 AGOS。
- 不实现业务功能、数据库、网络、安装、更新、注入、远程推荐、广告、推广、遥测或发布流程。
- 不创建 TypeScript、JavaScript 业务代码、WebView、Tauri、浮动 Git 依赖、通配依赖版本或未说明的 Cargo 工具。
- 不让 `upstream/`、未来 `benchmarks/`、`parity/` 数据目录或 `xtask/` 被隐式通配加入 Workspace。
- 不默认使用 Cache、整个 `target/` Artifact、Larger Runner、self-hosted runner 或付费 CI 资源。
- 不创建临时 UI 事实标准；最小窗口以生命周期和集成边界为验收目标。
- 不 Force Push、不删除 `main`、不启用自动合并，不把实现批准解释为合并授权。

## 七、本地知识与外部辅助

```yaml
local_knowledge_lookup:
  query: inputcodex Gate 3 纯 Rust Iced Workspace 七个 crate 分层 加载状态 取消 稳定错误 三平台 GitHub Actions 无缓存 CI
  tool: gbrain 0.41.14.0
  result: no-results
  conclusion: 继续使用已合并架构、CI 计划、官方元数据和 GitHub Fresh 事实，不用空结果补写设计
external_agos:
  command_status: report-only-completed
  result: needs-input
  registration_status: unregistered
  project_git_foundation: ready
  project_entry_docs: ready
  action: 按项目规则记录并绕过，继续项目原生流程
  forbidden_followup: 不在 inputcodex 中登记、修复或优化 AGOS
```

## 八、工具链与依赖证据

- Rust 固定为 `1.97.1 (8bab26f4f 2026-07-14)`；官方 channel 日期为 `2026-07-16`，`rust-toolchain.toml` 不得使用浮动 `stable`。
- Iced 固定为 `0.14.0`，checksum `000e01026c93ba643f8357a3db3ada0e6555265a377f6f9291c472f6dd701fb3`，MIT，MSRV `1.88`，未撤回。
- Iced 首轮候选使用 `default-features = false`，只评估 `wgpu`、`thread-pool` 与 Linux 编译所需 `x11`；任何 feature 调整必须用三平台编译根因和资源成本证明。
- 根 Workspace 统一 edition、license、repository、resolver 与依赖版本；`Cargo.lock` 随桌面应用提交。
- 本地机器只执行轻量定向验证，不承担全 Workspace、Windows/macOS 双平台或安装包构建。

## 九、执行批次

1. **控制面 checkpoint**：创建 Issue、分支、Session Plan、Runtime Workflow、初始报告并更新 Master Plan/README/build.md。
2. **RED 治理合同**：先提交失败的 `Test-CiScripts.ps1`，保留脚本缺失与违规夹具的可复现非零证据。
3. **GREEN 治理脚本**：实现路径分类与仓库政策验证，使合同测试通过。
4. **GREEN Workspace**：按 domain → application → adapters → presentation → desktop 顺序创建最小接口、状态语义和单元测试。
5. **子项目文档与本地轻验**：补齐每个 app/crate 的 build.md、err.md，执行 fmt、domain check/test 与 CI 脚本测试。
6. **首版三平台 CI**：创建无缓存 Workflow，先完成正常编译，再用普通提交制造并修复治理、格式和平台失败。
7. **冷构建基线**：每个平台记录至少三次无缓存成功样本，不基于单次最快结果引入 Cache。
8. **PR / Review / closeout**：创建关联 PR，解决全部 Review 对话，Fresh 复核后等待新的 Squash Merge 授权。

## 十、验证合同

```yaml
local_lightweight_commands:
  - pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
  - cargo fmt --all -- --check
  - cargo check -p inputcodex-domain
  - cargo test -p inputcodex-domain
  - git diff --check
cloud_full_commands:
  - cargo clippy --workspace --all-targets -- -D warnings
  - cargo test --workspace --all-targets
  - cargo check -p inputcodex-desktop
  - scripts/ci/Test-CiScripts.ps1
ci_jobs:
  - classify
  - governance
  - linux-quality
  - windows
  - macos
  - required
ci_policy:
  - Workflow 名称固定 CI，汇总 Job 名称固定 required
  - PR 避免与同分支 push 重复触发全量编译
  - 首版无 Cache，成功默认无 Artifact，失败 Artifact 白名单且 retention-days 为 7
  - CI 未稳定前不修改 main Ruleset required checks
```

## 十一、停止条件

- 正式 Release/tag、Iced checksum/许可证/MSRV、Rust 工具链、Ruleset 或 Issue `#16` 发生未解释的物质变化。
- 需要扩展 23 条允许路径、修改 `upstream/`、实现业务功能、创建发布资产或修改 AGOS。
- RED 证据无法复现、GREEN 只靠跳过测试、平台 Job 被关闭、失败只靠重新运行或 Force Push 掩盖。
- Iced 或平台类型越过分层边界，Windows/macOS 返回语义分叉，Linux 占位伪造成功。
- Review 对话未完成根因闭环、CI 失败、冷构建证据不足或缺少项目所有者合并授权。

## 十二、交付证据

tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/19
review_ref: pending
pr_ref: pending
ci_ref: pending
merge_ref: pending
owner_merge_authorization_ref: pending-new-owner-authorization-required

## 十三、治理 RED 执行记录

- Fresh 基线复核确认本地与远端分支 Head 均为 `03b68584add4e43291818376a2a85a696ea1e688`，Issue `#19` 为 OPEN 且标签为 `type:architecture`、`gate:3`。
- Ruleset `19395456` 仍为 active、无 bypass、required approvals `0`、必须解决 Review 对话且只允许 Squash Merge；上游正式 Release 仍为 `v1.2.41`，tag SHA 仍为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- `scripts/ci/Test-CiScripts.ps1` 已预置路径分类和仓库政策 GREEN 夹具，但首先检查两个实现入口是否存在。
- PowerShell AST 解析结果为 `0` 个错误；实际执行退出码为 `10`，唯一错误标记为 `CI_CONTRACT_RED_MISSING_IMPLEMENTATION`。
- RED 输出精确列出缺失的 `scripts/ci/Classify-Changes.ps1` 与 `scripts/ci/Verify-RepositoryPolicy.ps1`；当前仍不存在产品 Workspace、Rust/Iced 源码或 `.github/workflows/ci.yml`。

## 十四、治理 GREEN 执行记录

- `scripts/ci/Classify-Changes.ps1` 只读取 JSON 变更记录，拒绝路径穿越、绝对路径、反斜杠、控制字符和不完整重命名，不执行构建或网络调用。
- `scripts/ci/Verify-RepositoryPolicy.ps1` 固定七成员 Workspace、依赖方向、Iced 展示层边界、禁止脚本语言、WebView/Tauri、广告/遥测和更新源合同，不执行仓库代码或网络调用。
- 首轮 GREEN 启动失败复用了 `err.md` 已有“零结果必须用 `@(...)` 归一化”结论；第二轮空 diff 夹具通过 `[AllowEmptyCollection()]` 修复，均未降低生产检查。
- 安全复核新增 TOML 表形式依赖夹具，先稳定复现 Tauri 别名与依赖方向绕过，再修复解析器根因。
- 三份 PowerShell 文件 AST 均为 `0` 个错误，首个 GREEN checkpoint 输出 `CI_CONTRACT_GREEN passed=23` 并已推送。

## 十五、批准依赖方向纠偏记录

- Phase 4 架构复核发现首个治理夹具错误允许 infrastructure/platform/presentation 直接依赖 domain，并允许 parity 依赖 platform；这些关系比已批准架构箭头更宽。
- 先移除合法夹具中的越层依赖，并新增 `infrastructure → domain`、`platform → domain`、`presentation → domain`、`parity → platform` 四条拒绝测试；旧政策稳定错误返回 `ok=true`。
- 政策白名单已精确收紧为：application → domain；infrastructure/platform/presentation → application；parity → application + domain；desktop → presentation + application + infrastructure + platform。
- 完整合同最终输出 `CI_CONTRACT_GREEN passed=27`；纠偏 checkpoint 推送并回写 Issue 前仍不得创建 Cargo Workspace。

## 十六、最小 Workspace 执行记录

- 已创建七个显式成员、resolver `3`、统一 edition `2024`、Rust `1.97.1`、与根 `LICENSE` 一致的 `AGPL-3.0-only`、仓库地址和 `Cargo.lock`；`upstream/` 未进入 Workspace。
- domain/application 先以缺失 `DiagnosticCode`、`LoadCoordinator` 等符号取得编译 RED，再实现稳定诊断码、请求标识、六状态加载机、过期结果和取消后结果失效语义。
- infrastructure/platform/parity/presentation 分别先以缺失端口类型取得 RED，再实现未配置数据源明确失败、Windows/macOS 统一平台语义、非发布目标 unsupported、稳定错误签名和展示层消息映射。
- presentation 只有可选 `iced-runtime` 能直接启用 Iced `0.14.0`；desktop 仅依赖 presentation 暴露的启动入口，不直接依赖 Iced。UI 仍未建立设计系统。
- `Cargo.lock` 包含 `336` 个 package 记录，其中 `329` 个外部包、`7` 个 Workspace 包；Iced checksum 与批准值一致；直接 feature 仅为 `wgpu`、`thread-pool`、`x11`、`wayland`，默认 features、`webgl`、`web-colors`、`crisp` 未启用。
- 本地现有 Rust `1.93.1` 下，metadata、fmt、domain check 与六个轻量 crate 测试通过；命令使用 `--ignore-rust-version`，只作为逻辑证据。
- 精确 Rust `1.97.1` minimal 安装超过 5 分钟未完成，残留 rustup/rustc 进程已精确终止且工具链未安装；按本地轻验/云端全量合同不再重试，精确工具链与 Iced/desktop 编译等待 GitHub Actions。
- 离线 `cargo tree` 因未下载 `arrayref` 源包失败；未为取证下载并编译完整 `329` 个外部包图，feature 图与真实桌面编译改由 CI 验证。

## 十七、变更收集器与首版 CI 本地执行记录

- `Collect-Changes.ps1` 通过 `System.Diagnostics.ProcessStartInfo` 调用 Git，使用 `--name-status -z`、rename/copy 检测和 NUL 解析，保留新增、修改、删除及重命名新旧路径，并把 `T` 稳定映射为 `M`。
- 收集器 RED 先稳定为唯一 `CI_COLLECTOR_RED_MISSING_IMPLEMENTATION`；实现后暴露测试辅助函数把单行 Git 输出解包成字符的根因，修复返回形状后合同转绿。
- 许可证对账发现根 `LICENSE`/README 为 GNU AGPLv3，而 Workspace 元数据误写为 MIT；已按仓库真源纠正为 `AGPL-3.0-only`，并新增拒绝错误许可证的治理合同，Iced 自身仍按 MIT 依赖审计。
- 当前四份 PowerShell 脚本 AST 为 `0` 个错误，完整合同为 `CI_CONTRACT_GREEN passed=29`，真实仓库政策返回 `ok=true`、`violation_count=0`。
- `.github/workflows/ci.yml` 已创建固定六 Job、`contents: read`、PR/main/manual 事件、并发取消、无 Cache、失败白名单 Artifact 与 `retention-days: 7`；`actions/checkout` 与 `actions/upload-artifact` 均 Fresh 固定到 `v7.0.1` 的 Node 24 不可变提交。
- 本地 YAML 解析与 Workflow 静态合同已通过；这些证据不等于 Rust `1.97.1`、Iced/desktop 或三平台编译通过，下一步必须普通提交/push 后创建关联 PR 并观察 GitHub Actions。
- Draft PR `#21` 创建后，GitHub 对提交 `f3107dd` 与 `96374d9` 均生成工作流文件级失败且没有 Job；运行 `29910379208` 的官方注解精确指出三处 job 级 `env` 中 `runner.temp` 不可用。
- 根因修复不降低合同：移除三个平台 Job 的 job 级 `REPORT_DIR` 表达式，改为各 Job 首步用运行时 `RUNNER_TEMP` 创建目录并写入 `GITHUB_ENV`；本地 YAML 门禁新增禁止 job 级 `runner.*` 上下文检查。
