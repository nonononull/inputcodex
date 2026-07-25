# Issue #55：显式性能复测入口实施计划

## 目标

修复 `Performance Baseline` Workflow 在已入库证据存在时无法再次采样的合同缺口：手工触发必须显式选择 `evidence` 或 `measure`，默认保持 `evidence`；`pull_request` 和 `push` 的既有自动语义保持不变。

## 根因与决策

- 根因：`.github/workflows/performance-baseline.yml` 仅通过 Issue `#32` 三份证据文件是否存在选择模式。当前 `main` 为 `3/3`，所以任何手工 dispatch 都进入 `evidence`；没有安全、显式的重新测量入口。
- 拒绝方案：临时删除已入库证据以迫使 `measure`。它依赖隐式副作用，会改变来源 tree，也不能为后续复测提供稳定的合同入口。
- 采用方案：为 `workflow_dispatch` 增加受约束的 `mode` 输入。只有显式 `measure` 才进入现有双平台采集路径；默认 `evidence` 仍要求三份已入库证据完整存在。
- 所有者预授权：Issue `#55` 记录项目所有者在 `2026-07-26 06:19:35 +08:00` 作出的“按你推荐来 不用我二次批准 直接安排后续”决定。本 Issue 只使用该决定完成最小复测入口，不扩大到预算数值、预算 CI、优化或 Gate 5。

## 精确范围

以下排序后的十一条路径是唯一允许写集合，`scope_hash` 为 `sha256:89a9a40c76e98d573d4f55ca7d0aa140f325c9eb908e908f9e8731c55aaf03df`：

```text
.github/workflows/performance-baseline.yml
AGENTS.md
build.md
docs/plans/2026-07-26-issue-55-performance-remeasurement-entry.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-55-performance-remeasurement-entry.md
docs/reports/issue-55-performance-remeasurement-entry.md
docs/workflows/2026-07-26-issue-55-performance-remeasurement-entry-runtime.md
err.md
README.md
scripts/ci/Test-CiScripts.ps1
```

## 实施任务

### 任务 1：先证明当前合同缺口

**文件：**

- 验证：`.github/workflows/performance-baseline.yml`
- 验证：`scripts/ci/Test-CiScripts.ps1`

1. 确认现有证据路径均存在时，模式选择为 `evidence`。
2. 用一次性本地断言验证当前 Workflow 缺少 `workflow_dispatch.inputs.mode`，该断言必须失败；不修改工作树。
3. 记录失败原因：当前手工触发没有可声明的采样意图，不能安全完成 Issue `#54` 的五次独立采样。

### 任务 2：实现显式模式选择

**文件：**

- 修改：`.github/workflows/performance-baseline.yml`

1. 在 `workflow_dispatch` 下声明必填 choice 输入 `mode`，可选值严格为 `evidence` 与 `measure`，默认值为 `evidence`。
2. 在 contract Job 中读取事件名与手工输入：
   - `workflow_dispatch + measure`：无视已入库证据的存在，进入既有测量 Job。
   - `workflow_dispatch + evidence`：只有三份证据完整存在时进入既有 Evidence 验证；否则以明确错误停止。
   - `pull_request` 与 `push`：继续沿用原先的“全缺失即 measure、全存在即 evidence、部分存在即错误”语义。
3. 保持 permissions、hosted Runner、超时、并发取消、Action SHA、Artifact 保留期与禁止上传 `target/` 的既有合同不变。

### 任务 3：以 CI 合同锁定行为

**文件：**

- 修改：`scripts/ci/Test-CiScripts.ps1`

1. 在“性能基线 Workflow 固定治理与测量合同”测试中先添加 RED 断言，要求 workflow 同时包含受约束的手工输入和显式事件分流。
2. 在 Workflow 最小改动后运行该脚本，预期 `CI_CONTRACT_GREEN`，并确认没有削弱既有 Action SHA、权限、并发、Artifact 或 Runner 断言。
3. 禁止引入 YAML 解析依赖、Cache、预算阈值、required check 或新的 Workflow。

### 任务 4：更新项目原生控制面

**文件：**

- 修改：`AGENTS.md`
- 修改：`README.md`
- 修改：`build.md`
- 修改：`err.md`
- 修改：`docs/plans/PROJECT-MASTER-PLAN.md`
- 创建：`docs/reports/issue-55-performance-remeasurement-entry.md`

1. 记录 Issue `#55` 是 Issue `#54` 的前置合同修复，而不是预算 CI 或预算数值批准。
2. 在 `err.md` 记录“证据存在导致手工 Workflow 永远 evidence”的根因、处理和验证，避免未来通过删除证据绕过。
3. 在 `build.md` 增加十一路径范围、scope hash、三条本地轻量验证和一次 hosted dispatch 验收命令；禁止在本机运行完整 Rust Workspace 或真实性能采集。
4. 在报告中仅记录可复核的 Issue、commit、Workflow Run、Artifact 保留、验证和后续交接事实。

### 任务 5：验证、提交、PR 与 hosted 验收

**文件：**

- 验证：全部十一条范围路径

1. 本地依次运行 `Test-InputcodexBaseline.ps1 -Mode Evidence`、`Test-CiScripts.ps1`、`Verify-RepositoryPolicy.ps1`、scope audit 与 `git diff --check`。
2. 使用普通提交和 SSH push 推送 `codex/issue-55-performance-remeasurement-entry`；禁止 Force Push。
3. 创建关联 `Closes #55` 的非 Draft PR。PR CI 必须保持 Evidence 语义。
4. 在已推送分支上显式 dispatch `mode=measure` 一次，等待 Windows 与 macOS 均成功，并只核验临时 Artifact 名称、1 天保留期和无 `target/` 上传；该 Run 不写预算数值、不写入 Issue `#32` 证据。
5. 回写 Issue/PR 的根因、处理、验证、hosted Run 与后续 Issue `#54` 消费关系。所有 Review 对话根因闭环后，等待项目所有者针对最终 PR Head 的单独 Squash Merge 授权。

## 停止条件

- 实际差异超出十一条路径或 scope hash 漂移。
- 需要触及采集器、验证器、结果 schema、基线配置、已入库证据、预算数值、预算 CI、Ruleset、上游、Release、优化或 Gate 5。
- 手工 `measure` 入口不能与默认 `evidence`、PR 或 push 语义清晰隔离。
- 任意本地验证、hosted Run、Review 或 CI 失败但根因未闭环。
