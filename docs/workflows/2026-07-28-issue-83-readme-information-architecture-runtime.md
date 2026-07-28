# Issue #83 Runtime Workflow：README 信息架构与文档入口

## 运行元数据

```yaml
task_id: issue-83-readme-information-architecture
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/83
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/83
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/83#issuecomment-5101769289
branch: codex/issue-83-readme-information-architecture
baseline_main: da65f7d8402e4de27e2795ee8905be18ad565653
planning_scope_count: 3
planning_scope_hash: sha256:83c915a75626bbfb31d9520a519dba3a5a210adc8b47a535f46fc21412c3a95f
candidate_scope_count: 10
candidate_scope_hash: sha256:d8a404c19b108587a5e17b4ded454444d5e948c92410b759504a7eb7c63bed44
allowed_operations: plan,documentation-edit,local-light-verification,git-checkpoint,commit,push,non-draft-pr,review-ci
mutation_intent: separate-stable-readme-from-dynamic-governance-history
executor_enforcement: exact-path-scope-and-no-product-surface-hard-stop
final_merge_authorization: pending-separate-gate
```

## 节点图

```text
Issue #83 owner decision
  -> latest main worktree
  -> baseline verification
  -> planning control plane
  -> candidate scope approval
  -> stable README
  -> docs portal
  -> document responsibility boundaries
  -> err/report evidence
  -> scope/link/policy verification
  -> Git checkpoint
  -> commit and ordinary push
  -> non-Draft PR
  -> Review/CI root-cause closure
  -> separate Squash Merge authorization
```

## 输入合同

- Issue #83 必须保持 OPEN；
- 项目所有者批准方案 B 的证据必须保留在 Issue #83；
- 分支基线必须是 `main@da65f7d8402e4de27e2795ee8905be18ad565653`；
- planning scope 必须是三份控制面文档；
- candidate scope 必须是设计文件列出的十路径；
- 精确范围批准前禁止修改 README、AGENTS、build、Master Plan、err、report 和 docs portal；
- AGOS 不参与本仓写入，不得成为停止项目原生交付的条件。

## 输出合同

- 稳定、用户优先的 `README.md`；
- 分类导航 `docs/README.md`；
- 文档职责与防回退规则；
- PR #82 合并后的最小稳定状态修正；
- 新 worktree ignore 查询根因；
- Issue #83 实施报告；
- 十路径范围、链接、CI 合同、仓库政策和纯文档差异证据；
- 关联 Issue #83 的非 Draft PR；
- Review/CI 全部根因闭环后的独立合并授权门。

## 状态机

| 状态 | 进入条件 | 允许写入 | 退出条件 |
| --- | --- | --- | --- |
| `DISCOVERY` | 用户提出 README 混乱 | 无 | 方案 B 获批 |
| `PLANNING` | Issue #83 与隔离分支存在 | 三份 planning 文档 | planning scope 验证并 checkpoint |
| `SCOPE_PENDING` | candidate scope 已计算 | 仍仅三份 planning 文档 | 所有者批准十路径和哈希 |
| `EXECUTING` | 精确范围批准 | 十路径 | 所有内容完成 |
| `VERIFYING` | 实施完成 | 仅十路径内纠错 | 本地轻量门禁全绿 |
| `PR_OPEN` | 已提交和普通推送 | Review 根因修复仍限十路径 | Review/CI 全绿 |
| `MERGE_GATE` | Final Head 稳定 | 禁止继续写入 | 所有者单独授权 Squash Merge |

## 写入强制

Planning 允许集合：

```text
docs/plans/2026-07-28-issue-83-readme-information-architecture.md
docs/plans/sessions/2026-07-28-issue-83-readme-information-architecture.md
docs/workflows/2026-07-28-issue-83-readme-information-architecture-runtime.md
```

Planning scope hash：

`sha256:83c915a75626bbfb31d9520a519dba3a5a210adc8b47a535f46fc21412c3a95f`。

批准后的完整允许集合：

```text
AGENTS.md
README.md
build.md
docs/README.md
docs/plans/2026-07-28-issue-83-readme-information-architecture.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-83-readme-information-architecture.md
docs/reports/issue-83-readme-information-architecture.md
docs/workflows/2026-07-28-issue-83-readme-information-architecture-runtime.md
err.md
```

Candidate scope hash：

`sha256:d8a404c19b108587a5e17b4ded454444d5e948c92410b759504a7eb7c63bed44`。

任何范围漂移必须硬停止并重新批准。不得把“顺手修正文档”作为越界理由。

## README 内容合同

README 必须包含：

- 项目定位和开发阶段警告；
- 无广告、性能优先、Rust/Iced、双平台一致原则；
- 三个已迁移能力及其无副作用边界；
- 七成员 Workspace 架构表；
- 构建与完整文档入口；
- 上游正式 Release 真源、一致性例外和运行面禁止项；
- Issue/PR/Squash Merge 贡献流程；
- AGPL-3.0-only 许可证。

README 必须拒绝：

- `40` 位提交 SHA；
- Actions Run 数字流水账；
- `Final Head`、`merge tree`、Artifact 取证；
- “待推送”“创建非 Draft PR”“等待合并”等活动任务文本；
- 全量 Issue/PR 时间线；
- 全量任务文档清单；
- 完整 UI、更新、安装或发布已完成的误导性描述。

## 链接验证

只检查本任务新增或重写的 Markdown 入口：

```powershell
$files = @('README.md', 'docs/README.md')
$failures = [Collections.Generic.List[string]]::new()
foreach ($file in $files) {
  $text = Get-Content -LiteralPath $file -Raw -Encoding UTF8
  $directory = Split-Path -Parent (Resolve-Path -LiteralPath $file)
  foreach ($match in [regex]::Matches($text, '\[[^\]]+\]\((?<target>[^)]+)\)')) {
    $target = $match.Groups['target'].Value.Trim()
    if ($target -match '^(https?://|mailto:|#)') { continue }
    $path = ($target -split '#', 2)[0]
    if ([string]::IsNullOrWhiteSpace($path)) { continue }
    $resolved = Join-Path $directory $path
    if (-not (Test-Path -LiteralPath $resolved)) {
      $failures.Add("$file -> $target")
    }
  }
}
if ($failures.Count -gt 0) { throw "Markdown 链接失效：$($failures -join '; ')" }
```

## Checkpoint 合同

### Planning

- 只允许三份 planning 文档；
- planning count/hash 精确匹配；
- 基线 CI 合同与 Repository Policy 保持绿色；
- 使用本机默认 Git 时间创建 checkpoint；
- 回写 Issue #83 并等待十路径批准。

### Implementation

- 每批次前后检查 `git status --short` 与实际路径；
- README、docs portal、边界状态和证据按顺序实施；
- 不运行 Cargo 全量编译；
- 新错误先查 err.md，相同根因只引用。

### Verification

- 十路径与 candidate hash；
- README 行数和动态内容守卫；
- README/docs portal 链接；
- `scripts/ci/Test-CiScripts.ps1`；
- `scripts/ci/Verify-RepositoryPolicy.ps1`；
- `git diff --check`；
- 禁止路径和受保护能力扫描。

### Delivery

- 提交时间来自 Windows 本机 `Get-Date`；
- 普通推送，禁止 force push；
- 非 Draft PR 关联 Issue #83；
- 所有 Review 对话根因闭环；
- Hosted CI 按纯文档合同分类；
- 合并方式只能是 Squash Merge，且需要单独授权。

## 错误恢复

- PowerShell Markdown 反引号：复用 err.md 既有单引号 here-string + token 替换结论；
- 默认 apply_patch 权限：复用 npm 官方 `bin/codex.js --codex-run-as-apply-patch`；
- worktree ignore 查询：对目录规则使用尾斜杠或目录下探针路径，记录到 err.md；
- 链接失败：定位具体来源和目标，不通过删除链接掩盖缺失文档；
- 范围漂移：停止、重新计算、回写并重新批准；
- Review/CI 失败：确定根因、最小修复、重跑验证，禁止仅重跑碰运气；
- AGOS 异常：记录为外部缺口后绕过，不修改 AGOS。

## 当前执行进度

- `DISCOVERY`：完成；
- `PLANNING`：完成，checkpoint 为 `c313cba06484eb00ccc373e63419602d13192f7c`；
- `SCOPE_PENDING`：完成，十路径与 candidate scope hash 已批准；
- `EXECUTING`：完成；
- `VERIFYING`：完成；十路径、链接、CI 合同、Release Audit、仓库政策、暂存级 diff check 和官方纯文档分类均已通过；
- `PR_OPEN`：未开始；
- `MERGE_GATE`：未开始。
