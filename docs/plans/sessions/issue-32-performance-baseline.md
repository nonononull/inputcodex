# Session Plan：Issue #32 性能基线实施

schema_version: inputcodex.session-plan.v1
task_id: issue-32-performance-baseline
task_summary: 建立隔离 Rust 测量工程、opt-in 首次 view 标记、GitHub-hosted Windows/macOS 实测与可审计原始样本；不制定预算、不做优化。
task_class: Standard
decision_status: approved
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/32
baseline_ref: f81f457f615bed3d0f177aae52516824651abd12
baseline_tree_ref: b41db92d93032adf2ed77b2e01f785b072ea58ae
branch: codex/issue-32-performance-baseline
session_plan_ref: docs/plans/sessions/issue-32-performance-baseline.md
task_plan_ref: docs/plans/issue-32-performance-baseline.md
runtime_workflow_ref: docs/workflows/issue-32-performance-baseline-runtime.md
report_ref: docs/reports/issue-32-performance-baseline.md
approved_decision_ref: user-message:进入-Issue-分支-Session-Plan-Runtime-Workflow-实现与验证-PR-2026-07-25
scope_hash: sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505
allowed_operations: create-isolated-worktree, edit-exact-approved-paths, local-light-validation, github-hosted-windows-macos-measurement, normal-commit-push, non-draft-pr, review-ci
mutation_intent: 建立隔离、可重复、可审计的 inputcodex 基线采集能力与真实样本；不改变功能语义，不实施优化，不制定数值预算。
executor_enforcement: exact-twenty-eight-path-set, isolated-benchmark-workspace, github-hosted-windows-macos-only, local-light-validation-only, no-upstream-or-reference-runtime, no-budget-or-optimization, normal-push-only, squash-merge-only, no-force-push, no-main-delete, review-root-cause-closure-required
time_source: Windows Get-Date for local Git operations; GitHub timestamps remain service-side evidence
agos_status: bypassed-report-only-unregistered-needs-input-no-cross-repo-mutation

## 一、所有者批准与 Fresh 基线

- Discovery 冻结评论：`https://github.com/nonononull/inputcodex/issues/32#issuecomment-5079349474`。
- 实施授权评论：`https://github.com/nonononull/inputcodex/issues/32#issuecomment-5079389518`。
- 本地 `main`、远端 GitHub API `main` 与工作树起点均为 `f81f457f615bed3d0f177aae52516824651abd12`。
- 上游最新正式 Release 仍为 `v1.2.42`，tag commit 为 `657cd33e009ad02515d30db6492cd4e669b06318`，`release_audit.status=current`。
- Ruleset `19395456` 仍只命中 `main`，禁止删除/非快进，PR 只允许 Squash，Review 对话必须解决；人类维护者数量为 `1`，required approvals 为 `0`。
- 起点验证：Parity 定向测试 `12/12`、CI 合同 `32/32`、Repository Policy `violation_count=0`、`git diff --check` 通过。

## 二、local_knowledge_lookup

- 项目控制面：`AGENTS.md`、`README.md`、`build.md`、`err.md`、Master Plan、Issue #24/#32 计划与报告。
- 代码入口：应用层 `LoadCoordinator`、Parity `validate_repository`、展示层 Iced `view`、既有三平台 CI 冷构建指标。
- 来源与许可证：`upstream/source-lock.json`、上游 `v1.2.42` AGPL-3.0、半成品参考固定 commit 与 AGPL-3.0 元数据。
- 外部规则：AGOS brainstorming/runtime、生成代码、注释、复杂度和测试规则；没有发现可直接复用且适配本仓的性能基线 workflow。
- 实际结果：AGOS 默认入口 ReportOnly 返回 `TASK_REGISTRATION_STATUS=unregistered` 与 `DEFAULT_ENTRY_ROUTE_STATUS=needs-input`；Git snapshot checkpoint 因计划文件尚未形成项目 Git 快照而阻断，Session Plan 验证器要求的完整 AGOS schema 不适用于本项目原生控制面。
- 结论：按 `AGENTS.md` 采用项目原生流程并绕过 AGOS；不伪造 `mutation_intent`、reviewer/subagent 或外部 schema，不修改 AGOS Registry、Workflow、Vault、规则或脚本。

## 三、已批准写集合

```text
.github/workflows/performance-baseline.yml
.gitignore
AGENTS.md
benchmarks/config/issue-32-baseline.json
benchmarks/inputcodex-baseline/build.md
benchmarks/inputcodex-baseline/Cargo.lock
benchmarks/inputcodex-baseline/Cargo.toml
benchmarks/inputcodex-baseline/err.md
benchmarks/inputcodex-baseline/src/lib.rs
benchmarks/inputcodex-baseline/src/main.rs
benchmarks/inputcodex-baseline/tests/baseline_contract.rs
benchmarks/README.md
benchmarks/results/issue-32/macos.json
benchmarks/results/issue-32/manifest.json
benchmarks/results/issue-32/windows.json
build.md
crates/inputcodex-presentation/build.md
crates/inputcodex-presentation/src/lib.rs
docs/plans/issue-32-performance-baseline.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/issue-32-performance-baseline.md
docs/reports/issue-32-performance-baseline.md
docs/workflows/issue-32-performance-baseline-runtime.md
err.md
README.md
scripts/ci/Test-CiScripts.ps1
scripts/performance/Invoke-InputcodexBaseline.ps1
scripts/performance/Test-InputcodexBaseline.ps1
```

哈希算法固定为 Windows `Sort-Object` 默认大小写不敏感排序，UTF-8 无 BOM，LF 连接并保留末尾 LF，再计算 SHA-256。

## 四、测量协议

1. `presentation-first-view`：Windows/macOS 各至少 `5` 个成功新进程样本，最多 `7` 次尝试，单次 `15` 秒超时。
2. `desktop-idle-resources`：首次 view 后等待 `30` 秒，以 `1 Hz` 采样 `60` 秒 Working Set 与 CPU。
3. `application-load-complete`：`3` 次预热、`20` 个样本、批量迭代后换算每操作耗时。
4. `application-cancel-stale`：`3` 次预热、`20` 个样本，必须证明陈旧完成不能覆盖取消状态。
5. `parity-repository-validation`：`3` 次预热、`20` 个样本，明确为热文件系统场景。
6. `release-binary-size`：Windows/macOS Release 二进制字节数及构建耗时，仅作元数据与趋势输入。

昂贵进程场景报告最小值/中位数/最大值；20 样本场景报告 P50/P95/最小值/最大值；IQR 只标记异常值，不删除样本。失败、超时和取消全部保留。

## 五、TDD 与验证顺序

1. 先为隔离 Rust 场景写失败测试，再实现最小测量核心。
2. 先为展示层 opt-in/单次标记写失败测试，再实现探针。
3. 先为专用 Workflow 合同写失败断言，再实现脚本和 Workflow。
4. 本地只运行隔离 crate、展示层无默认特性、PowerShell 合同、CI 合同、Repository Policy 与 `git diff --check`。
5. 初次 PR Head 由 GitHub-hosted runner 生成 Windows/macOS 临时成功 Artifact；下载核验后写入固定结果路径并删除临时 Artifact。
6. 最终 Head 只验证已入库证据与实现哈希一致，成功 run 不保留 Artifact。

## 六、禁止面

- 不修改根 `Cargo.toml`、根 `Cargo.lock`、`apps/`、`parity/`、`upstream/`、既有 `ci.yml`、上游 watch、Ruleset、Release 或 AGOS。
- 不运行上游/半成品，不导入其代码或数据，不进行跨实现数值对比。
- 不实现 UI、不改变视觉/交互、不加入广告、遥测、WebView 或 JavaScript/TypeScript 业务代码。
- 不填写预算数值、不做性能优化、不迁移功能、不处理 `parity-exception`。
- 不 Force Push、不删除 `main`、不使用管理员绕过、Merge Commit、Rebase Merge、self-hosted/Larger/收费 runner 或自动合并。

## 七、停止条件

- 上游最新正式 Release、`main`、Ruleset、维护者数量、批准范围或 `scope_hash` 发生变化。
- 需要触及 28 路径外文件，或隔离测量工程无法在不修改根 Workspace 的情况下构建。
- GitHub-hosted GUI 环境无法产生可解释样本，结果缺少环境指纹/输入哈希，或需要删除异常值才能形成结论。
- 本地或 CI 测试失败且根因未闭环、Review 对话未解决、Artifact 未核验入库。
- 最终 PR Head 尚未取得项目所有者单独 Squash Merge 授权。

## 八、实施与测量证据

- 非 Draft PR：`https://github.com/nonononull/inputcodex/pull/49`，自动合并保持关闭。
- 有效测量提交：`1974577837de97a74d7b980e8106d5e2f4a4de2e`；tree：`0efa97728a3e8d0e8bf11a8bed75b4886cdce91a`。
- Performance Run：`30169262247`，Attempt `1`；`contract`、`windows`、`macos`、`required` 四 Job 全部成功。
- 同一测量提交的主 CI Run：`30169262250`，七 Job 全部成功。
- Windows 与 macOS 临时成功 Artifact 分别为 `8622503841`、`8622478720`；下载后按文件 SHA-256 与 manifest 核验，固定结果入库后已删除，Run 当前 Artifact 数为 `0`。
- 配置、实现、输入哈希分别为 `sha256:b9ed601016ececc735634aeb143965c78fbfc61819d37c7f9e584bc971642b53`、`sha256:3d69735d1d02b3cd09316e892e104a55c9e3b605035ef155861cbc3c12705d21`、`sha256:c5b507d219ff49975c13805a2a6e036ade6c61a33a184bfffb74219ce01784b5`。
- `windows.json` 与 `macos.json` SHA-256 分别为 `652d913ac29453acd4ce0a00cd5a7b3ab39d47f88e4ec0146d30f72e56df4952`、`068165593728b81a5c8c089b09bbd6bb6c931d63d3a4f0d791d3b02e3d10a22e`。
- 样本只允许在同平台、同环境指纹下形成趋势输入；跨平台排名和预算数值均不属于本 Session 批准范围。
- 结果入库后的本地 Fresh 门：隔离基准测试 `7/7`、展示层定向测试 `3/3`、Evidence `violation_count=0`、CI 合同 `33/33`、Repository Policy `violation_count=0`、PowerShell/YAML/JSON 解析和 `git diff --check` 全部通过；实际差异精确为批准 28 路径。
