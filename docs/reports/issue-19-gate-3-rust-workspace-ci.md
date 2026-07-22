# Issue #19：Gate 3 纯 Rust Workspace 与首版三平台 CI 报告

report_status: failure-semantics-and-cold-baseline-in-progress
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/19
branch_ref: codex/issue-19-gate-3-rust-workspace-ci
baseline_ref: 477d110a9b284e127af365f5278901bcfa69e093
session_plan_ref: docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md
scope_hash: sha256:2e101627480012d57d6d0472a08cfbe03fc401f6ac74ef3ae1e6a42929ed61ba
pr_ref: https://github.com/nonononull/inputcodex/pull/21
ci_ref: https://github.com/nonononull/inputcodex/actions/runs/29911337652
review_ref: pending
merge_ref: pending

## 一、批准范围

- 项目所有者已批准创建七成员纯 Rust Workspace、最小分层骨架、治理脚本与首版无缓存三平台 CI。
- 批准引用为 `user-message:approve-gate-3-implementation-2026-07-22`；该批准不包含最终 PR Squash Merge。
- 本任务不迁移上游业务功能，不实现数据库、网络、安装、更新、注入、远程推荐、广告、推广、遥测或发布流程。
- UI 只允许最小生命周期集成，不建立设计系统；视觉与交互由 Gemini 实现或审阅。

## 二、Fresh 基线

- PR `#18` 已 Squash Merge，Issue `#17` 已关闭；当前 `main` 基线为 `477d110a9b284e127af365f5278901bcfa69e093`。
- 上游正式 Release 为 `v1.2.41`，tag SHA 为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`；上游 `main` 的二维码图片变化对本任务无物质影响。
- Rust 为 `1.97.1 (8bab26f4f 2026-07-14)`；Iced 为 `0.14.0`、MIT、MSRV `1.88`、checksum `000e01026c93ba643f8357a3db3ada0e6555265a377f6f9291c472f6dd701fb3`、未撤回。
- Ruleset `19395456` 保持 active、无 bypass、required approvals `0`、解决 Review 对话和 Squash-only。
- 本地 GBrain 查询无结果；AGOS report-only 返回 `needs-input/unregistered`，已按项目合同绕过且未修改外部仓库。

## 三、控制面与 RED checkpoint

- 已创建公开 Issue `#19` 与分支 `codex/issue-19-gate-3-rust-workspace-ci`。
- Issue 正文已固定 23 条允许路径模式、RED/GREEN 合同、三平台 CI、停止条件、回滚和独立合并授权边界。
- Session Plan 与 Runtime Workflow 已落盘，控制面提交 `03b68584add4e43291818376a2a85a696ea1e688` 已与远端分支精确一致。
- `scripts/ci/Test-CiScripts.ps1` 已预置空 diff、文档、重型路径、删除、重命名、非法路径、Iced 越层、Workspace 越界、脚本语言、WebView/Tauri、广告/遥测、更新源和依赖方向夹具。
- PowerShell AST 解析为 `0` 个错误；实际 RED 执行退出码为 `10`，稳定标记 `CI_CONTRACT_RED_MISSING_IMPLEMENTATION` 恰好出现一次。
- RED 根因是 `scripts/ci/Classify-Changes.ps1` 与 `scripts/ci/Verify-RepositoryPolicy.ps1` 尚不存在，不是路径、拼写、解析或夹具错误。
- RED 提交 `67fe99457e1aa2717cc29c70d51114028d68dafd`、tree `21bc79da1e402c4d7266d4d8b54af6866a685463` 已通过普通 fast-forward push，并回写 Issue 评论 `5043557146`。

## 四、治理 GREEN 证据

- 路径分类器输出确定 JSON，区分空 diff、纯文档和重型路径，并审计删除、重命名的新旧路径。
- 仓库政策验证器固定七成员显式 Workspace、包名与依赖方向；Iced 只能直接存在于 presentation，并拒绝生产 TypeScript/JavaScript、Tauri/WebView、广告/遥测和外部更新源。
- 初始 GREEN 自测暴露测试夹具的零结果数组形状和空集合参数绑定问题；均确定根因并修复测试，不降低实现门禁。
- 安全复核先新增 TOML 表形式的 Tauri 别名与逆向依赖 RED，再修复依赖解析器，防止通过 `[dependencies.alias]` 绕过政策。
- 首个合同测试 `23/23` 通过，三份 PowerShell 脚本 AST 均为 `0` 个错误，测试退出码为 `0`；提交 `be9259f55b32014e918113936e6e6ddfdd16765f` 已推送并回写 Issue 评论 `5043682396`。

## 五、批准依赖方向纠偏

- 架构复核发现首个合法夹具与政策白名单错误允许 infrastructure/platform/presentation 直连 domain，并允许 parity 依赖 platform，违反已批准依赖图。
- 四条新增测试先稳定复现政策错误返回 `ok=true`，证明不是文档措辞问题，而是机器门禁真实放宽。
- 允许图已收紧为批准箭头，完整合同达到 `27/27` GREEN；当前仍没有产品 Cargo、Rust/Iced 或产品 CI Workflow。
- 纠偏提交 `2d8a1466ae42d6b208258bee3d0cb6bd5647bb12` 已普通 push，并回写 Issue 评论 `5043770155`。

## 六、最小 Workspace 证据

- 七成员路径与批准合同一致，根清单使用 resolver `3`、edition `2024`、Rust `1.97.1`、与根 `LICENSE` 一致的 `AGPL-3.0-only` 和本仓 repository；每个 app/crate 均有独立 `build.md` 与 `err.md`。
- domain/application 的编译 RED 分别由缺失 `DiagnosticCode` 与加载状态 API 触发；最终 domain `1` 项、application `3` 项集成测试通过。
- infrastructure、platform、parity、presentation 的编译 RED 分别由缺失批准类型触发；最终四个包各 `1` 项集成测试通过。
- application 固化 `Idle / Loading / Ready / Empty / Failed / Cancelling`；新请求使旧结果过期，取消后结果失效，取消完成需要匹配请求标识。
- infrastructure 未配置端口返回 `Unavailable / LOAD_SOURCE_UNCONFIGURED`，不伪造空结果；非 Windows/macOS 返回 `Unsupported / PLATFORM_UNSUPPORTED`。
- presentation 只依赖 application，Iced `0.14.0` 为可选 `iced-runtime`；desktop 直接依赖 application/infrastructure/platform/presentation，但不直接依赖 Iced；parity 不链接 desktop。
- `Cargo.lock` 包含 `336` 个 package 记录，其中 `329` 个外部包、`7` 个 Workspace 包；Iced checksum 为批准值；直接 features 为 `wgpu`、`thread-pool`、`x11`、`wayland`，未启用默认 features 或 Web/WebGL 相关 feature。

## 七、本地验证与资源边界

- 治理合同 `30/30` 与真实仓库政策 `ok=true`；Cargo metadata、rustfmt、domain check 和六个轻量 crate 测试通过。
- 本机只有 Rust `1.93.1`；轻验使用 `--ignore-rust-version`，不得解释为 Rust `1.97.1` 或桌面运行时通过。
- Rust `1.97.1` minimal 安装超过 5 分钟仍无完成证据，已精确终止本次 rustup/rustc 残留，确认工具链未安装；按 CI 卸载合同不再消耗本地机器。
- 首次非离线多包 RED 因 registry 刷新超时并留下 Cargo 进程；终止本次 PID 后用 `--offline` 立即复现批准 API 缺失，证明网络阶段与代码 RED 已分离。
- 离线 feature tree 因缺少 `arrayref` crate 源包失败；没有为本地取证下载或编译完整 `329` 个外部包图。远端运行 `29911337652` 已完成精确 Rust `1.97.1`、Iced/desktop 与三平台编译测试。

## 八、变更收集器与首版 CI 本地证据

- 变更收集器以真实临时 Git 仓库覆盖新增、修改、删除和重命名，使用无 shell 拼接的 `ProcessStartInfo` 与 NUL 解析；收集器缺失 RED、单行输出形状错误和最终 GREEN 均有可复现根因。
- 根 `Cargo.toml` 曾误把 Iced 的 MIT 许可证沿用为本项目许可证；已与根 `LICENSE`/README 对齐为 `AGPL-3.0-only`，并用独立 RED/GREEN 合同防止再次漂移。
- 四份 PowerShell 脚本 AST 为 `0` 个错误，完整合同输出 `CI_CONTRACT_GREEN passed=30`，真实仓库政策返回 `ok=true`、`violation_count=0`。
- 首版 `CI` Workflow 固定 `classify`、`governance`、`linux-quality`、`windows`、`macos`、`required` 六个 Job；只读权限、并发取消、精确 Rust、无 Cache、失败白名单 Artifact 和 7 天保留期均已本地静态验证。两个官方 Action 均 Fresh 锁定到 `v7.0.1` 的 Node 24 不可变提交。
- 本地 YAML 解析通过，但精确 Rust `1.97.1`、Iced/desktop、Linux/Windows/macOS 及 `required` 仍是远端待验证事实，不提前宣称通过。
- Draft PR `#21` 首轮运行 `29910132968` 与 `29910379208` 均在创建 Job 前失败；GitHub 注解将根因定位到三个平台 Job 的 job 级 `env` 使用了不可用的 `runner.temp` 上下文。
- 修复改为平台 Job 首步从 `RUNNER_TEMP` 计算报告目录并写入 `GITHUB_ENV`，不改变 Artifact 白名单、权限、超时、Runner 或构建命令；本地 YAML 门禁同步拒绝未来 job 级 `runner.*` 回归。
- 上下文修复后的运行 `29910847062` 已证明 classify、governance、Windows 与 macOS 成功；Linux Clippy 唯一根因是 `platform_contract.rs` 无条件导入只在 Windows/macOS 条件断言使用的 `PlatformKind`，`required` 按设计阻断。
- 条件导入修复提交 `bd4610f6e98dc597bddf02c584d0f0fc616cac7b` 触发运行 `29911337652`：六个 Job 全绿，成功 Artifact 数为 `0`；Linux、Windows、macOS Job 分别执行 `112`、`211`、`94` 秒。
- 首个全绿样本的 Linux Clippy、Windows/macOS 桌面冷构建 metrics 只写入 Step Summary；Check Run 与 Actions API 完成后均未返回该摘要，因此二进制字节数保持“未知”，禁止按文件类型或历史值猜测。
- 已按 TDD 增加“冷构建指标同时写入日志与摘要”合同：旧 Workflow 稳定 RED 为读取 metrics 数量 `0`、期望 `3`；最小修复后合同恢复 `CI_CONTRACT_GREEN passed=30`。后续运行可从普通 Job 日志复取精确秒数与二进制字节数。
- 运行 `29913139948` 六 Job 全绿，成功 Artifact 数为 `0`；第二个无缓存样本已复取 Linux Clippy `38.732` 秒、Windows 冷构建 `117.053` 秒/`26,347,520` 字节、macOS 冷构建 `78.163` 秒/`53,510,976` 字节。
- 治理失败语义运行 `29913582488` 中，临时生产目录 `.ts` 探针使 governance 以唯一违规码 `SCRIPT_LANGUAGE_FORBIDDEN` 失败，`required` 精确报告 `governance=failure`；classify 与三平台 Job 均成功。
- 该失败只上传 `governance-failure-29913582488-1` 与 `required-failure-29913582488-1`，内容白名单为 `contract.log`、`policy.json`、`required.json`，不存在 `target/`、环境转储或整个工作区；当前修复删除探针，不降低治理规则。
- 治理修复提交 `d474c47f5ab02ef9ed9804b208a739823819c9e9` 触发运行 `29914029406`，六 Job 全绿且 Artifact 数为 `0`；三平台无缓存成功样本由此达到最低 `3/3`。
- rustfmt 失败语义提交 `743da60b81161f2c18d6db9b0a1b03f976b04cea` 只改变 `DiagnosticCode::new` 的空格形状；运行 `29914734781` 在 Linux “检查 Rust 格式”步骤失败，Windows/macOS 与治理成功，`required` 精确报告 `linux-quality=failure`。
- 对应失败 Artifact 只有 `fmt.log`、`toolchain.txt` 与 `required.json`；当前修复恢复 rustfmt 标准格式，不跳过格式检查，也不改变 Rust 语义。

## 九、下一合法批次

1. 提交当前 metrics 日志合同、状态回写和 `docs/reports/rust-ci-cold-baseline.md`，普通 push 触发新的无缓存三平台运行。
2. 按 Runtime Workflow 用普通提交依次制造并修复治理、rustfmt、通用 Rust、Windows 条件编译和 macOS 条件编译失败；每次读取日志确认根因，禁止 rerun 旧失败。
3. Linux、Windows、macOS 各收集至少三次无缓存成功样本后，更新 PR 正文、Issue 评论和 Review 证据；CI 稳定前不得修改 `main` Ruleset required checks，也不得在本地机器执行全 Workspace 或桌面重型编译。

## 十、收口边界

- PR 与首轮全绿 CI 已有真实引用；Review 与 merge 字段继续保持 `pending`，不得提前宣称可合并。
- 最终 PR 必须包含 `Closes #19`，所有适用 Job 成功、Review 对话根因闭环后，再等待项目所有者新的 Squash Merge 授权。
