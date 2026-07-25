# Issue #41：解耦上游快照同步与 CI 基线断言 Runtime Workflow

```yaml
task_id: issue-41-ci-contract-decoupling
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/41
session_plan_ref: docs/plans/sessions/2026-07-24-issue-41-ci-contract-decoupling.md
implementation_plan_ref: docs/plans/2026-07-24-issue-41-ci-contract-decoupling.md
approved_decision_ref: user-message:批准独立-CI-合同-Issue-PR-2026-07-24
selected_business_path: inputcodex.ci-contract-decoupling
baseline_ref: 317349a2cee1d2777472c8ccbd55204e570176c4
branch: codex/issue-41-ci-contract-decoupling
scope_hash: sha256:ada2baa0a524b2c8f0831d946236197b056513981c30b4530d903114b709c1b8
scope_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/41#issuecomment-5067938336
local_time_source: Windows Get-Date
```

## 工作流节点

1. `startup-baseline`：确认 `main`、远端 `main`、实现分支和 Git 状态；运行 `verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly`。
2. `root-cause`：只读查看 PR #40 CI 日志、两个失败断言、`Baseline` 验证链、Rust stale 专项合同、Git 历史和 #40 路径差异。
3. `red`：在 #40 未修改工作树运行 Python 上游监控全量单测和 Rust 精确失败测试，记录退出码与失败文本。
4. `scope-proposal`：写入发现计划、Session Plan、Runtime Workflow，按七条路径重算 `scope_hash`，在 Issue #41 请求项目所有者明确批准。
5. `owner-gate`：项目所有者已通过 `https://github.com/nonononull/inputcodex/issues/41#issuecomment-5067938336` 批准七路径和 `scope_hash`；最终 Squash Merge仍需单独授权。
6. `green`：修改两个测试合同、`err.md` 和报告；运行本地轻量验证、路径/哈希检查和 Git checkpoint。
7. `merge-simulation`：用临时 detached 工作树无提交合入 #40 Head，复验合并后的 stale 输入，再完整 abort/remove。
8. `delivery`：普通提交、正常推送、创建非 Draft PR、处理所有 Review 对话、等待 GitHub-hosted CI；最终 Head 全绿后请求 Squash Merge 授权。
9. `closeout`：独立回写 Issue #41、主计划和执行报告；确认 #40 使用新 `main` 自动重跑。不得顺手合并或改写 #40。

## 允许与禁止

| 阶段 | 允许 | 禁止 |
| --- | --- | --- |
| 当前执行 | 七路径内 TDD、轻量验证、临时合并模拟、普通提交、推送、PR、Review/CI | 扩范围、force push、删除 `main`、Merge/Rebase Merge、未解决对话合并、未经授权 Squash Merge |
| Closeout | 合并后事实回写和 #40 自动重跑观察 | 修改 #40、上游缓存、Workflow、Cargo、产品代码、AGOS |

## 停止门

- 路径集合或 `scope_hash` 与本文件不一致；
- 项目所有者没有明确授予实现/提交/PR/Review/CI/Squash Merge 权限；
- RED 失去可重复性，或最小修复需改变 Workflow、来源锁、Cargo、产品运行面；
- 合并模拟、Release Audit Gate、仓库政策、CI 或 Review 任一失败；
- AGOS 继续返回 `needs-input`、接口不可用或未登记：记录后绕过，不得阻塞 inputcodex 原生流程，也不得修改 AGOS。

## 定向命令

```powershell
# RED：仅在 #40 现有工作树执行
python -m unittest discover -s .github/scripts/tests -p 'test_upstream_watch.py' -v
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository '仓库功能目录通过完整引用与安全验证' -- --exact

# GREEN：仅在精确范围获批并实施后执行
python -m unittest discover -s .github/scripts/tests -p 'test_upstream_watch.py' -v
python .github/scripts/upstream_watch.py --validate-only
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

## 外部治理边界

- AGOS ReportOnly：`DEFAULT_ENTRY_ROUTE_STATUS=needs-input`、`DEFAULT_ENTRY_TASK_REGISTRATION_STATUS=unregistered`；只作为审计记录。
- 当前 MCP resources 与 resource templates 为空；没有可调用的 `local_knowledge_lookup` 查询面，Session Plan 已记录缺口。
- 本项目继续依赖 `AGENTS.md`、`build.md`、`err.md`、本计划、Session Plan、Issue #41、Git 和 GitHub CI；不得修改外部规则、Registry、Workflow 或 Vault。
