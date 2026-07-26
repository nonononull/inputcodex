# Issue #69 性能实现哈希路径边界 Implementation Plan

> **执行要求：** 实施时必须使用 `superpowers:test-driven-development`、`superpowers:systematic-debugging` 与 `superpowers:verification-before-completion`；当前主执行器逐项完成，不启用未获用户请求的 subagent。

**目标：** 让 `implementation_sha256` 只绑定真实被测生产源码、Cargo 合同和显式性能基础设施，不再因普通 crate `tests/**` 变化而失效，同时保持真实实现变化必须触发 Evidence 失效。

**架构：** 保留 Issue `#32` 的严格 Evidence 绑定语义，将七个产品组件的动态 Rust 输入从“组件目录全部 `.rs`”收窄为“组件根 `Cargo.toml` + `src/**/*.rs`”。由于哈希输入集合与验证脚本本身都会变化，旧 Evidence 不得直接改写元数据，必须由修复后精确 Head 在 GitHub-hosted Windows/macOS 上重新测量并刷新三份结果。

**技术栈：** PowerShell 7、Rust Workspace 文本合同、GitHub Actions hosted runners、JSON Evidence、Git/GitHub CLI。

## 全局约束

- 本任务不修改 PR `#68` 的六路径或 Final Head `159abbf45dfdc29a277cb152af7368868f2f618d`。
- 禁止修改预算数值、预算公式、Ruleset、Gate 5、上游缓存、功能目录或产品运行面。
- 本机只做合同与证据轻量验证；真实性能采集只允许使用公开仓库标准 GitHub-hosted Windows/macOS runner。
- 三份 Evidence 只能来自修复后精确 Head 的成功 `mode=measure` Artifact；禁止手工替换哈希、复用旧 Artifact 或删除失败证据掩盖问题。
- 最终 Squash Merge 必须基于最终 PR Head 获得项目所有者单独授权。

## 已确认事实

- `main` 旧算法哈希等于入库 Evidence：`sha256:e4c9265396476c918112f553d846239ed21f84c7432e6a34ddc8f55293d64e48`。
- PR `#68` 仅修改普通测试后，旧算法哈希变为 `sha256:174f8aec273a5b7490aa833e7554e26edda996a649105ce916d9cc4873ea8bd7`。
- 仅收窄动态路径时，`main` 与 PR `#68` 都得到 `sha256:39fcc5593ada18c4b42daf2e94556a97dda1e90385f146184178418a79b38a7e`，证明普通测试已被排除。
- 新路径集合哈希不等于旧 Evidence；且两个待修改脚本本身属于实现哈希固定输入，因此真实修复会再次改变哈希。
- `err.md` 与 Issue `#32` Runtime Workflow 已有同类先例：验证器或 CI 合同变化后必须重新 measure，禁止直接替换元数据。

## 精确范围提案

按 Windows `Sort-Object` 排序固定为十路径：

```text
benchmarks/results/issue-32/macos.json
benchmarks/results/issue-32/manifest.json
benchmarks/results/issue-32/windows.json
docs/plans/2026-07-26-issue-69-performance-implementation-hash-boundary.md
docs/plans/sessions/2026-07-26-issue-69-performance-implementation-hash-boundary.md
docs/reports/issue-69-performance-implementation-hash-boundary.md
docs/workflows/2026-07-26-issue-69-performance-implementation-hash-boundary-runtime.md
err.md
scripts/ci/Test-CiScripts.ps1
scripts/performance/Test-InputcodexBaseline.ps1
```

```text
scope_hash: sha256:6392a1b4150f2aae0c34c285e83b6870f47e3bdc57def6308d0a26ddd158911d
```

项目所有者已通过 Issue 评论 `https://github.com/nonononull/inputcodex/issues/69#issuecomment-5083808758` 明确批准上述十路径、`scope_hash`、TDD 实施、Hosted 重新测量、Evidence 入库、提交、推送、PR 与 Review/CI；最终 Squash Merge 仍保留单独授权门。

---

### Task 1：固定 RED 与哈希集合证据

**文件：**
- 修改：`docs/reports/issue-69-performance-implementation-hash-boundary.md`
- 修改：`err.md`

- [x] **Step 1：复现 PR #68 Evidence RED**

```powershell
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 `
  -RepositoryRoot C:\Users\dashuai\Documents\inputcodex-worktrees\issue-67-release-audit-repository-contract `
  -Mode Evidence
```

预期：退出码 `1`，三份 Evidence 各出现一个 `HASH_MISMATCH`，当前哈希为 `sha256:174f8aec273a5b7490aa833e7554e26edda996a649105ce916d9cc4873ea8bd7`。

- [x] **Step 2：记录路径集合迁移证据**

预期：`main_narrowed_surface == pr68_narrowed_surface == sha256:39fcc5593ada18c4b42daf2e94556a97dda1e90385f146184178418a79b38a7e`，同时 `stored_matches_narrowed == false`。

### Task 2：先写哈希边界合同 RED

**文件：**
- 修改：`scripts/ci/Test-CiScripts.ps1`

- [x] **Step 1：增加隔离性能 Contract 夹具**

只复制 `Contract` 模式必需的 Workflow、根 Cargo 合同、七个产品组件、性能配置/隔离基准、`parity`、`upstream/source-lock.json` 与性能脚本；不得复制 `.git`、`target/` 或完整 `upstream/CodexPlusPlus/`。

- [x] **Step 2：增加真实行为合同**

```powershell
$before = Invoke-ChildScript -Path $fixtureValidator -Arguments @('-RepositoryRoot', $fixtureRoot, '-Mode', 'Contract')
Write-Utf8File -Path (Join-Path $fixtureRoot 'crates/inputcodex-parity/tests/non-performance-contract.rs') -Content 'fn ordinary_contract_test() {}'
$afterTestChange = Invoke-ChildScript -Path $fixtureValidator -Arguments @('-RepositoryRoot', $fixtureRoot, '-Mode', 'Contract')
Add-Content -LiteralPath (Join-Path $fixtureRoot 'crates/inputcodex-parity/src/lib.rs') -Value "`npub const IMPLEMENTATION_HASH_SENTINEL: bool = true;" -Encoding utf8NoBOM
$afterSourceChange = Invoke-ChildScript -Path $fixtureValidator -Arguments @('-RepositoryRoot', $fixtureRoot, '-Mode', 'Contract')

Assert-Equal -Expected $before.Json.implementation_sha256 -Actual $afterTestChange.Json.implementation_sha256 -Message '普通 crate tests 变化不得改变性能实现哈希'
Assert-True -Condition ($before.Json.implementation_sha256 -ne $afterSourceChange.Json.implementation_sha256) -Message '产品 src 变化必须改变性能实现哈希'
```

- [x] **Step 3：运行合同并确认 RED**

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
```

预期：新合同失败，原因是普通 `tests/**` 变化仍改变哈希；不得先改生产脚本再补测试。

### Task 3：最小修正动态实现路径

**文件：**
- 修改：`scripts/performance/Test-InputcodexBaseline.ps1`

- [x] **Step 1：显式分离组件根与源码目录**

```powershell
$productImplementationRoots = @(
    'apps/inputcodex-desktop',
    'crates/inputcodex-domain',
    'crates/inputcodex-application',
    'crates/inputcodex-infrastructure',
    'crates/inputcodex-platform',
    'crates/inputcodex-presentation',
    'crates/inputcodex-parity'
)
foreach ($productImplementationRoot in $productImplementationRoots) {
    $implementationPaths.Add("$productImplementationRoot/Cargo.toml")
}
$productSourceDirectories = @($productImplementationRoots | ForEach-Object { "$_/src" })
$codePaths = Get-RelativeFilePaths -Root $resolvedRoot -Directories $productSourceDirectories -Filter { param($file) $file.Extension -eq '.rs' }
foreach ($codePath in $codePaths) { $implementationPaths.Add($codePath) }
```

固定列表中的性能 Workflow、隔离基准工程、性能夹具、CI 合同与性能脚本保持不变。

- [x] **Step 2：运行 GREEN**

运行 CI 合同和性能 `Contract`，预期全部合同通过、`ok=true`、零违规。

### Task 4：实现检查点与 hosted 重新测量

**文件：**
- 修改：`benchmarks/results/issue-32/windows.json`
- 修改：`benchmarks/results/issue-32/macos.json`
- 修改：`benchmarks/results/issue-32/manifest.json`

- [x] **Step 1：本地轻量门禁**

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Contract
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

- [x] **Step 2：提交并普通推送实现检查点**

只暂存十路径中已产生的文件，不覆写本机 Git 时间，普通推送；禁止 force push。

- [x] **Step 3：从精确实现 Head 触发测量**

```powershell
gh workflow run 'Performance Baseline' --repo nonononull/inputcodex --ref codex/issue-69-performance-hash-boundary -f mode=measure
```

预期：`contract`、`windows`、`macos`、`required` 全部成功。

- [x] **Step 4：只从同一 Run Artifact 刷新 Evidence**

核对 `measurement_commit` 等于实现 Head、两平台 `implementation_sha256` 一致、配置与输入哈希不变；禁止手工替换单一哈希字段。

### Task 5：最终验证、PR 与阻塞解除

**文件：**
- 修改：`docs/reports/issue-69-performance-implementation-hash-boundary.md`

- [x] **Step 1：重新运行 Evidence GREEN**

```powershell
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
```

预期：`ok=true`、`violation_count=0`。

- [ ] **Step 2：完成最终门禁与非 Draft PR**

执行 CI 合同、Evidence、仓库政策、十路径、`scope_hash`、`git diff --check` 和非法控制字节检查；普通推送最终 Head，创建关联 Issue `#69` 的非 Draft PR。

- [ ] **Step 3：合并后恢复 PR #68**

Issue `#69` 的最终 Squash Merge 单独授权。合并后保持 PR `#68` Head 不变，重新触发两套 Workflow；要求标准 CI `7/7`、Performance Baseline `4/4`、成功 Artifact `0`、Review 对话 `0`。

## 自审

- 同时覆盖“普通测试不漂移”和“产品源码必须漂移”，未放宽真实实现 Evidence。
- 旧样本不改元数据；新样本必须来自修复后精确 Head 的 hosted 双平台测量。
- 十路径以外任何需求均硬停止并重新审批。
