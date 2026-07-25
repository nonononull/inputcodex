# Issue #32 性能基线 Runtime Workflow

schema_version: inputcodex.runtime-workflow.v1
workflow_id: inputcodex.issue-32.performance-baseline
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/32
session_plan_ref: docs/plans/sessions/issue-32-performance-baseline.md
task_plan_ref: docs/plans/issue-32-performance-baseline.md
approved_decision_ref: user-message:进入-Issue-分支-Session-Plan-Runtime-Workflow-实现与验证-PR-2026-07-25
branch: codex/issue-32-performance-baseline
baseline_ref: f81f457f615bed3d0f177aae52516824651abd12
scope_hash: sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505
execution_mode: single-executor, no-subagents, local-light-validation, github-hosted-measurement
mutation_intent: 建立隔离、可重复、可审计的 inputcodex 基线采集能力与真实样本；不改变功能语义，不实施优化，不制定数值预算。
executor_enforcement: exact-twenty-eight-path-set, isolated-benchmark-workspace, github-hosted-windows-macos-only, no-upstream-runtime, no-budget, no-optimization, normal-push-only, squash-merge-only
agos_status: bypassed-report-only-unregistered-needs-input-no-cross-repo-mutation

## 1. 启动基线

1. 读取 `AGENTS.md`、`README.md`、`build.md`、`err.md`、Master Plan、Session Plan 与本 Workflow。
2. 确认当前工作树为 `codex/issue-32-performance-baseline`，HEAD 为 `f81f457f615bed3d0f177aae52516824651abd12`，范围外零差异。
3. 使用 GitHub API Fresh 核对远端 `main`、Issue `#32`、Release `v1.2.42`、Ruleset `19395456` 与维护者数量；Git smart-HTTP 超时按 `err.md` 既有结论绕过，不盲目重试。
4. 运行启动 checkpoint：

```powershell
pwsh -NoProfile -File D:\Android_source\ai-growth-os\components\rules\scripts\verify-git-snapshot-governance.ps1 `
  -Root D:\Android_source\ai-growth-os `
  -ProjectRoot . `
  -TaskId issue-32-performance-baseline `
  -Checkpoint `
  -CheckpointReason startup-baseline `
  -ReportOnly
```

本次 AGOS 默认入口 ReportOnly 已返回 `TASK_REGISTRATION_STATUS=unregistered` 与 `DEFAULT_ENTRY_ROUTE_STATUS=needs-input`；项目 Git snapshot checkpoint 因控制面尚未提交而阻断，完整 AGOS Session Plan schema 不适用于本项目原生计划。现记录 `agos_status=bypassed` 并继续项目原生流程；禁止伪造外部 schema、修复或修改 AGOS。

## 2. 计划与范围门

1. 验证 Session Plan 包含 `approved_decision_ref`、`scope_hash`、`allowed_operations`、`mutation_intent` 与 `executor_enforcement`。
2. 重新计算 28 路径哈希，必须等于批准值。
3. 检查实际差异只能是批准集合的子集；最终 PR Head 必须精确覆盖 28 路径，缺失结果文件时仍处于测量前阶段，不得请求合并。
4. AGOS 默认入口 ReportOnly 已按以下命令尝试并按上述状态绕过；后续不重复调用、不修改外部控制面：

```powershell
pwsh -NoProfile -File D:\Android_source\ai-growth-os\components\rules\scripts\invoke-agos-default-entry.ps1 `
  -Root D:\Android_source\ai-growth-os `
  -ProjectRoot . `
  -TaskId issue-32-performance-baseline `
  -SelectedBusinessPath performance-baseline `
  -SessionPlanRef docs/plans/sessions/issue-32-performance-baseline.md `
  -ApprovedDecisionRef user-message:进入-Issue-分支-Session-Plan-Runtime-Workflow-实现与验证-PR-2026-07-25 `
  -ReportOnly
```

## 3. TDD 执行批次 A：隔离 Rust 测量工程

1. 创建独立 `[workspace]` manifest、配置、测试与空实现；先运行目标测试取得 RED。
2. 实现以下纯 Rust 场景：
   - `application-load-complete`
   - `application-cancel-stale`
   - `parity-repository-validation`
3. 每个场景返回稳定名称、迭代数、总纳秒、每操作纳秒与 checksum；未知场景、零迭代和无效仓库根必须失败。
4. 运行 GREEN：

```powershell
cargo test --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --locked --offline
```

5. 确认根 `Cargo.toml` 与 `Cargo.lock` 零差异；生成 execute-batch checkpoint。

## 4. TDD 执行批次 B：展示层首次 view 探针

1. 在展示层测试中先固定环境变量判定、稳定标记和单次输出语义。
2. 仅在 `INPUTCODEX_PERFORMANCE_PROBE=1` 时，于首次 Iced `view` 构建后输出 `INPUTCODEX_PERFORMANCE_READY_V1` 并 flush stdout。
3. 未启用时不得输出标记、写文件、发网络请求或改变状态机/界面。
4. 本地只运行：

```powershell
cargo test -p inputcodex-presentation --no-default-features --offline
```

Iced runtime 编译和真实窗口只在 GitHub-hosted Windows/macOS 运行。

## 5. TDD 执行批次 C：PowerShell 与 Workflow

1. 先扩展 `scripts/ci/Test-CiScripts.ps1`，要求性能 Workflow 具有：只读权限、concurrency、固定 SHA Action、Windows/macOS、Job 超时、无 Cache、成功 Artifact 1 天、失败 Artifact 7 天、禁止 `target/`。
2. 运行测试取得 RED。
3. 实现配置验证器、结果验证器和测量脚本：
   - 读取唯一 JSON 配置；
   - 构建 Release 桌面和隔离测量工程；
   - 采集首次 view、空闲资源、Rust 场景、构建耗时、二进制大小和完整环境指纹；
   - 保留失败尝试、超时和全部原始样本；
   - 计算最小值、中位数、P50/P95、最大值与 IQR 异常标记，不删除数据。
4. 实现 `.github/workflows/performance-baseline.yml`：
   - PR 中结果文件缺失时运行 Windows/macOS 测量并上传 1 天临时成功 Artifact；
   - 结果文件存在时只运行证据验证；
   - 失败时上传最多 7 天诊断 Artifact；
   - `required` 汇总必须阻断任一平台/合同失败。
5. 运行 CI 合同测试取得 GREEN，并生成 execute-batch checkpoint。

## 6. 远端测量与证据入库

1. 在完成本地轻量验证后创建普通提交并普通推送；不得 Force Push。
2. 创建关联 Issue `#32` 的非 Draft PR，保持自动合并关闭。
3. 等待初次性能 Workflow：Windows/macOS 必须成功生成各自 JSON Artifact；主 CI 也必须通过。
4. 使用 `gh run download` 下载精确 run/attempt 的 Artifact，运行 `Test-InputcodexBaseline.ps1 -Mode Evidence` 校验后写入固定路径。
5. 生成 `manifest.json`，记录测量 commit/tree、配置/实现/输入哈希、run/attempt、Artifact ID 与两平台结果哈希。
6. 删除已入库的临时成功 Artifact；不得删除失败诊断 Artifact 来掩盖问题。
7. 普通提交原始结果与报告，推送最终 Head；最终 Workflow 应进入证据验证模式，成功 Artifact 数为 `0`。

## 7. Fresh 验证门

```powershell
cargo test --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --locked --offline
cargo test -p inputcodex-presentation --no-default-features --offline
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

完整 Workspace、Iced runtime、Windows/macOS Release 与测量由 GitHub-hosted runner 执行，本地禁止补跑。

## 8. Review、PR 与停止条件

- PR 必须非 Draft、自动合并关闭、Head 与远端分支一致。
- 所有 Review 对话必须在合并前完成根因、处理方式和验证证据闭环；仅点击 Resolve 不成立。
- 若来源、环境或样本不可比较，报告必须明确 `not-comparable`，不得补造预算或删除样本。
- 若需要范围外文件、根 Workspace 变更、上游运行、收费/self-hosted runner、优化或预算，立即停止并回到 Issue #32 请求新决策。
- 最终 Head CI/Review/Artifact/原始样本未闭环前禁止请求 Squash Merge；完成后仍必须等待项目所有者单独授权。

## 9. Closeout 草案

- `workflow_lookup_mode`: project-native-first, agos-report-only-optional
- `static_workflow_refs`: 本 Runtime Workflow、Issue #24 性能协议、Gate 3 冷构建报告
- `dynamic_workflow_gap_summary`: 标准 hosted runner 只能形成严格指纹下的同平台趋势，不能形成跨平台或跨实现排名
- `reusable_lesson`: `measure` 与 `evidence` 双模式必须用测量 commit/tree、配置/实现/输入哈希和结果文件哈希闭环；IQR 只标记异常值，不得删除样本
- `rollout_status`: project-native-evidence-imported-awaiting-final-pr-verification

## 10. 已执行远端证据

- Run `30168805192` 在 Workflow 装载阶段零 Job 失败；根因为 Job 级 `env` 非法引用 `runner.temp`，修复后未再复现。
- Run `30168904725` 暴露 `u128` 整数除法把亚纳秒批量结果截断为 `0` 的精度缺陷；结果未入库，修复为 `f64` 并增加 `SCENARIO_PRECISION_INVALID` 后重新测量。
- 有效 Run `30169262247` Attempt `1` 在提交 `1974577837de97a74d7b980e8106d5e2f4a4de2e` 上四 Job 全绿；Windows/macOS 原始结果已按 manifest 导入固定路径。
- 四个临时成功 Artifact 均已删除；Run `30168904725` 与 `30169262247` 当前 Artifact 数均为 `0`。失败根因和无效结果均保留在 GitHub Run、提交历史与 `err.md`，没有通过删除样本或改写语义掩盖问题。
- 结果入库后的本地 Fresh 门已通过：隔离基准测试 `7/7`、展示层定向测试 `3/3`、Evidence 零违规、CI 合同 `33/33`、Repository Policy 零违规、PowerShell/YAML/JSON 解析和差异空白检查均成功；批准范围并集为 `28/28`，scope hash 保持批准值。
