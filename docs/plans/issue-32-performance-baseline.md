# Issue #32 性能基线实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. 当前任务未授权 subagent，所有写入由当前执行器完成。

**目标：** 在不改变功能语义、不污染七成员主 Workspace 的前提下，建立可重复、可审计的 Windows/macOS 性能基线与原始样本。

**架构：** 新建一个带独立 `[workspace]` 的 Rust 测量工程，只通过路径依赖调用现有应用层与 Parity 验证器；展示层仅增加环境变量显式开启的首次 `view` 标记。PowerShell 负责进程测量、环境指纹、统计汇总与 JSON 证据，专用 GitHub Actions Workflow 负责 Windows/macOS 真实运行。

**技术栈：** Rust `1.97.1`、PowerShell `7`、Iced `0.14.0`、GitHub Actions 标准 hosted runner、JSON 原始证据。

## 全局约束

- 批准范围固定为 28 条路径，`scope_hash` 为 `sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505`。
- 根 `Cargo.toml`、根 `Cargo.lock`、`apps/`、`parity/`、`upstream/`、既有 `.github/workflows/ci.yml` 和 AGOS 零差异。
- 上游 `v1.2.42@657cd33e009ad02515d30db6492cd4e669b06318` 与半成品参考只作静态来源/许可证证据，不运行、不打包、不进行数值比较。
- 本地只运行定向离线测试与合同验证；Windows/macOS GUI、Release 构建和完整测量交给公开 GitHub-hosted runner。
- 本 Issue 不填写数值预算、不实施优化、不迁移功能、不设计 UI；最终 Squash Merge 另行授权。

## 文件职责

- `benchmarks/inputcodex-baseline/`：隔离 Rust 测量工程、单元/合同测试及独立构建排错文档。
- `benchmarks/config/issue-32-baseline.json`：唯一测量参数真源。
- `scripts/performance/Invoke-InputcodexBaseline.ps1`：构建、启动、采样、环境指纹和原始 JSON 生成。
- `scripts/performance/Test-InputcodexBaseline.ps1`：配置、结果、哈希和禁止面的合同验证。
- `.github/workflows/performance-baseline.yml`：两阶段 Windows/macOS hosted 测量与最终证据校验。
- `crates/inputcodex-presentation/src/lib.rs`：仅增加 `INPUTCODEX_PERFORMANCE_PROBE=1` 时的一次性 stdout 标记。
- `benchmarks/results/issue-32/`：固定 Windows、macOS 和组合 manifest 原始证据。
- `docs/reports/issue-32-performance-baseline.md`：样本摘要、限制和预算就绪性，不包含预算数值。

## Task 1：冻结项目原生控制面

**文件：**
- 修改：`AGENTS.md`
- 修改：`README.md`
- 修改：`build.md`
- 修改：`err.md`
- 修改：`docs/plans/PROJECT-MASTER-PLAN.md`
- 创建：`docs/plans/issue-32-performance-baseline.md`
- 创建：`docs/plans/sessions/issue-32-performance-baseline.md`
- 创建：`docs/workflows/issue-32-performance-baseline-runtime.md`

- [x] 回写 Issue #32 所有者实施授权与 Fresh 基线。
- [x] 创建 `codex/issue-32-performance-baseline` 隔离工作树。
- [x] 运行起点定向验证：Parity `12/12`、CI 合同 `32/32`、仓库政策零违规。
- [x] 运行 Session Plan 与范围哈希验证。
- [x] 形成首个普通提交：`docs: 建立 Issue 32 性能基线控制面`。

## Task 2：用 TDD 建立隔离 Rust 测量核心

**文件：**
- 创建：`benchmarks/inputcodex-baseline/Cargo.toml`
- 创建：`benchmarks/inputcodex-baseline/Cargo.lock`
- 创建：`benchmarks/inputcodex-baseline/src/lib.rs`
- 创建：`benchmarks/inputcodex-baseline/src/main.rs`
- 创建：`benchmarks/inputcodex-baseline/tests/baseline_contract.rs`
- 创建：`benchmarks/inputcodex-baseline/build.md`
- 创建：`benchmarks/inputcodex-baseline/err.md`
- 创建：`benchmarks/config/issue-32-baseline.json`
- 修改：`.gitignore`

**接口：**
- `run_scenario(name, repository_root, iterations) -> Result<ScenarioMeasurement, BaselineError>`。
- 支持 `application-load-complete`、`application-cancel-stale`、`parity-repository-validation`。
- CLI 输出稳定 CSV 行，PowerShell 负责 JSON 封装；Rust 端不引入统计或 JSON 新依赖。

- [x] 先写合同测试，断言未知场景、零迭代和无效仓库根失败。
- [x] 运行 `cargo test --manifest-path benchmarks/inputcodex-baseline/Cargo.toml`，确认 RED。
- [x] 实现最小场景与 checksum，使用 `std::hint::black_box` 防止优化删除。
- [x] 运行隔离工程测试，确认 GREEN。
- [x] 生成独立 `Cargo.lock`，确认根 Cargo 文件零差异；随本批次快照提交。

## Task 3：用 TDD 增加 opt-in 首次 view 标记

**文件：**
- 修改：`crates/inputcodex-presentation/src/lib.rs`
- 修改：`crates/inputcodex-presentation/build.md`

**接口：**
- 环境变量：`INPUTCODEX_PERFORMANCE_PROBE=1`。
- stdout 标记：`INPUTCODEX_PERFORMANCE_READY_V1`。
- 标记最多输出一次；未启用时零输出、零文件、零网络副作用。

- [x] 先在展示层单元测试中固定 opt-in 与单次输出判定。
- [x] 运行无默认特性测试确认 RED。
- [x] 实现最小探针状态与首次 `view` 调用。
- [x] 运行无默认特性测试确认 GREEN 且无编译警告；Iced runtime 编译留给云端。

## Task 4：建立 PowerShell 测量与 CI 合同

**文件：**
- 创建：`scripts/performance/Invoke-InputcodexBaseline.ps1`
- 创建：`scripts/performance/Test-InputcodexBaseline.ps1`
- 创建：`.github/workflows/performance-baseline.yml`
- 修改：`scripts/ci/Test-CiScripts.ps1`
- 创建：`benchmarks/README.md`

**接口：**
- `Invoke-InputcodexBaseline.ps1 -RepositoryRoot -Platform -OutputPath`。
- `Test-InputcodexBaseline.ps1 -RepositoryRoot -Mode Contract|Evidence`。
- Workflow 初次运行在结果文件缺失时测量并上传 1 天 Artifact；结果入库后只验证证据，成功不再上传 Artifact。

- [x] 先扩展 CI 合同测试，要求 pinned actions、只读权限、concurrency、超时、无 Cache、成功 Artifact 1 天和失败 Artifact 7 天。
- [x] 运行 `scripts/ci/Test-CiScripts.ps1` 确认 RED。
- [x] 实现配置/结果验证器与测量脚本。
- [x] 实现 Windows/macOS Workflow 与 required 汇总。
- [x] 运行性能 Contract、33 项 CI 合同、Evidence 正例与篡改反例确认 GREEN。

## Task 5：采集、核验并入库真实样本

**文件：**
- 创建：`benchmarks/results/issue-32/windows.json`
- 创建：`benchmarks/results/issue-32/macos.json`
- 创建：`benchmarks/results/issue-32/manifest.json`
- 创建：`docs/reports/issue-32-performance-baseline.md`

- [x] 普通提交并推送测量实现，创建非 Draft PR 触发初次 hosted 测量。
- [x] 下载 Windows/macOS 成功 Artifact，核对 run、Head、平台、配置哈希和文件哈希。
- [x] 生成组合 manifest；原始样本、失败尝试与异常值标记全部保留。
- [x] 删除已入库的临时成功 Artifact，再普通提交结果与报告。
- [x] 运行最终证据模式验证，确认结果文件与当前测量实现哈希一致。

## Task 6：验证、Review 与 PR 收口

- [x] 本地执行 `build.md` 的 Issue #32 轻量验证和 `git diff --check`。
- [x] 核对实际差异严格等于批准 28 路径，范围外零差异。
- [ ] 推送最终 Head，等待主 CI 与性能 Workflow 全绿。
- [ ] 检查 Review 对话；逐条记录根因、处理与验证证据。
- [ ] 在 Issue/PR 回写 Head、CI、Artifact、样本和 Review 证据。
- [ ] 停在项目所有者最终 Squash Merge 授权前。
