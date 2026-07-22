# Issue #19：Gate 3 纯 Rust Workspace 与首版三平台 CI 报告

report_status: first-ci-local-static-green-awaiting-checkpoint-push
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/19
branch_ref: codex/issue-19-gate-3-rust-workspace-ci
baseline_ref: 477d110a9b284e127af365f5278901bcfa69e093
session_plan_ref: docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md
scope_hash: sha256:2e101627480012d57d6d0472a08cfbe03fc401f6ac74ef3ae1e6a42929ed61ba
pr_ref: pending
ci_ref: pending
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

- 治理合同 `29/29` 与真实仓库政策 `ok=true`；Cargo metadata、rustfmt、domain check 和六个轻量 crate 测试通过。
- 本机只有 Rust `1.93.1`；轻验使用 `--ignore-rust-version`，不得解释为 Rust `1.97.1` 或桌面运行时通过。
- Rust `1.97.1` minimal 安装超过 5 分钟仍无完成证据，已精确终止本次 rustup/rustc 残留，确认工具链未安装；按 CI 卸载合同不再消耗本地机器。
- 首次非离线多包 RED 因 registry 刷新超时并留下 Cargo 进程；终止本次 PID 后用 `--offline` 立即复现批准 API 缺失，证明网络阶段与代码 RED 已分离。
- 离线 feature tree 因缺少 `arrayref` crate 源包失败；没有为本地取证下载或编译完整 `329` 个外部包图，精确 feature 解析和 desktop 编译等待 GitHub Actions。

## 八、变更收集器与首版 CI 本地证据

- 变更收集器以真实临时 Git 仓库覆盖新增、修改、删除和重命名，使用无 shell 拼接的 `ProcessStartInfo` 与 NUL 解析；收集器缺失 RED、单行输出形状错误和最终 GREEN 均有可复现根因。
- 根 `Cargo.toml` 曾误把 Iced 的 MIT 许可证沿用为本项目许可证；已与根 `LICENSE`/README 对齐为 `AGPL-3.0-only`，并用独立 RED/GREEN 合同防止再次漂移。
- 四份 PowerShell 脚本 AST 为 `0` 个错误，完整合同输出 `CI_CONTRACT_GREEN passed=29`，真实仓库政策返回 `ok=true`、`violation_count=0`。
- 首版 `CI` Workflow 固定 `classify`、`governance`、`linux-quality`、`windows`、`macos`、`required` 六个 Job；只读权限、并发取消、精确 Rust、无 Cache、失败白名单 Artifact 和 7 天保留期均已本地静态验证。两个官方 Action 均 Fresh 锁定到 `v7.0.1` 的 Node 24 不可变提交。
- 本地 YAML 解析通过，但精确 Rust `1.97.1`、Iced/desktop、Linux/Windows/macOS 及 `required` 仍是远端待验证事实，不提前宣称通过。

## 九、下一合法批次

1. 对当前 Workflow、治理 `29/29`、许可证一致性、允许路径和 Git diff 进行 Fresh 验证，形成普通提交并普通 push，在 Issue `#19` 回写 CI checkpoint。
2. 创建正文含 `Closes #19` 的关联 PR，观察标准 GitHub-hosted runners 对 Rust `1.97.1`、Iced/desktop、Linux/Windows/macOS 与 `required` 的真实结果。
3. 对每个失败先确定根因，再用后续普通提交修复；CI 稳定前不得修改 `main` Ruleset required checks，也不得在本地机器执行全 Workspace 或桌面重型编译。

## 十、收口边界

- PR、CI、Review 和 merge 字段保持 `pending`，不得提前宣称通过。
- 最终 PR 必须包含 `Closes #19`，所有适用 Job 成功、Review 对话根因闭环后，再等待项目所有者新的 Squash Merge 授权。
