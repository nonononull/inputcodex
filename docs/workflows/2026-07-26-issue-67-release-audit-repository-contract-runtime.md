# Issue #67：解耦固定目录证据与活动快照状态 Runtime Workflow

```yaml
task_id: issue-67-release-audit-repository-contract
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/67
session_plan_ref: docs/plans/sessions/2026-07-26-issue-67-release-audit-repository-contract.md
implementation_plan_ref: docs/plans/2026-07-26-issue-67-release-audit-repository-contract.md
approved_decision_ref: session-plan:issue-67#decision
selected_business_path: inputcodex.release-audit-repository-contract
baseline_ref: a2e5e5a6728200739a4acb85042ba7831ac6295b
branch: codex/issue-67-release-audit-repository-contract
scope_hash: sha256:b6295dabd39f0cba7c4f13bd3d35ff8b0433e1fb95de98e6fc5f2cf0c1eb6b9f
scope_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/67#issuecomment-5083523560
local_time_source: Windows Get-Date
blocking_pr_ref: https://github.com/nonononull/inputcodex/pull/66
blocking_pr_head: 4e419586fc89b1bbdd79d20b7179f017070052fb
pr_ref: pending
ci_ref: pending
```

## 工作流节点

1. `startup-baseline`：确认 `origin/main`、隔离分支、六路径范围、本机时间与日期覆写变量。
2. `root-cause`：读取 Run `30202056781`、失败测试、Git blame、Issue `#41` / PR `#42` 和已有 current/stale 状态机测试。
3. `control-plane`：写入计划与本 Runtime Workflow；运行 AGOS Session Plan verifier 和一次 Default Entry ReportOnly。
4. `red`：在未修改的 PR `#66` Head 保留旧测试退出码 `101` 与 line `395` 证据。
5. `green`：将仓库实例测试收敛为固定目录证据，不再固定活动 snapshot/status；不修改生产验证器。
6. `replay`：在本分支复验 main/current；用临时 detached 工作树叠加 PR `#66` Head 复验 stale，完成后安全删除临时工作树。
7. `verify`：执行目录测试、CI 合同、Release Audit Gate、仓库政策、fmt、六路径 scope hash 与 diff check。
8. `delivery`：提交、普通推送、非 Draft PR、Review/CI；最终 Squash Merge 保留单独授权门。
9. `resume-pr66`：修复 PR 合并后，不改变 PR `#66` Head，基于新 `main` 重新触发并核验七 Job、Artifact 与 Review 对话。

## 允许与禁止

| 阶段 | 允许 | 禁止 |
| --- | --- | --- |
| 当前执行 | 六路径内控制面与测试合同修复、本地轻量验证、临时 PR #66 叠加、普通提交/推送、非 Draft PR、Review/CI | 扩范围、修改 PR #66、force push、删除 main、Merge/Rebase Merge、未授权 Squash Merge |
| 外部治理 | 一次 AGOS ReportOnly 与只读规则/知识查询 | 登记、修复、优化或提交 AGOS |
| 产品边界 | 无产品运行面写入 | 预算、Ruleset、Gate 5、上游缓存、Iced/业务代码 |

## 停止门

- 六路径或 `scope_hash` 变化；
- 修复需要修改生产验证器、Cargo、Workflow、`upstream/` 或 PR `#66`；
- main/current 或 PR `#66`/stale 任一回放失败；
- Review 对话未解决、最终 CI 未全绿或成功 Artifact 非零；
- 最终 Squash Merge 未获得项目所有者对最终 Head 的单独授权。

## 验证命令

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
cargo fmt --all -- --check
git diff --check
```

## 外部治理边界

- `local_knowledge_lookup`：GitNexus 当前无已索引仓库；已读取 AGOS vault/rules 与项目原生文档，不伪造 GBrain 结果。
- AGOS 只允许运行一次 `invoke-agos-default-entry.ps1 -ReportOnly`；若 `needs-input`、`unregistered`、接口不兼容或异常，记录后立即绕过。
- 不修改 `D:\Android_source\ai-growth-os` 的任何文件；本项目 Issue、Session Plan、Runtime Workflow、Git 与 GitHub CI 仍为执行真源。
