# Issue #69：性能实现哈希路径边界 Runtime Workflow

```yaml
task_id: issue-69-performance-implementation-hash-boundary
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/69
session_plan_ref: docs/plans/sessions/2026-07-26-issue-69-performance-implementation-hash-boundary.md
implementation_plan_ref: docs/plans/2026-07-26-issue-69-performance-implementation-hash-boundary.md
approved_decision_ref: user-message:批准-独立性能Evidence合同方案A-2026-07-26
selected_business_path: inputcodex.performance-implementation-hash-boundary
baseline_ref: a2e5e5a6728200739a4acb85042ba7831ac6295b
branch: codex/issue-69-performance-hash-boundary
scope_hash: sha256:6392a1b4150f2aae0c34c285e83b6870f47e3bdc57def6308d0a26ddd158911d
scope_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/69#issuecomment-5083808758
local_time_source: Windows Get-Date
blocking_pr_ref: https://github.com/nonononull/inputcodex/pull/68
blocking_pr_head: 159abbf45dfdc29a277cb152af7368868f2f618d
blocking_run_ref: https://github.com/nonononull/inputcodex/actions/runs/30203435572
pr_ref: pending
ci_ref: pending
```

## 工作流节点

1. `startup-baseline`：确认 `origin/main=a2e5e5a6728200739a4acb85042ba7831ac6295b`、隔离分支、Windows 本机时间和 PR `#68` Head。
2. `root-cause`：复现三个 `HASH_MISMATCH`，追踪 `implementationPaths`，计算旧算法、收窄集合与模拟脚本改动哈希。
3. `decision-correction`：记录初始七路径方案无法保持旧 Evidence；比较真实重新测量、历史兼容层和元数据改写三案。
4. `scope-gate`：在 Issue `#69` 回写十路径与 `scope_hash`；未获项目所有者明确批准前禁止修改两个脚本和三份 Evidence。
5. `red`：在 `Test-CiScripts.ps1` 写真实行为合同，证明旧脚本下普通 tests 变化仍漂移。
6. `green`：只把七个产品组件动态输入收窄到组件根 `Cargo.toml` 与 `src/**/*.rs`；固定性能基础设施列表不变。
7. `local-verify`：运行 CI 合同、性能 Contract、仓库政策、十路径与 diff check；Evidence 在 hosted 重新测量前允许保持预期 RED。
8. `implementation-checkpoint`：提交并普通推送精确实现 Head，不 force push。
9. `hosted-measure`：从该 Head 显式运行一次 `Performance Baseline mode=measure`；Windows/macOS/required 必须全绿。
10. `evidence-import`：只从同一成功 Run 的双平台 Artifact 刷新 Issue `#32` 三份 Evidence，随后本地 Evidence 必须 GREEN。
11. `delivery`：提交 Evidence、普通推送、非 Draft PR、Review/CI；最终 Squash Merge 单独请求授权。
12. `resume-pr68`：Issue `#69` 合并后保持 PR `#68` Head 不变，重新运行两套 Workflow 并核验 Artifact/Review。

## 允许与禁止

| 阶段 | 允许 | 禁止 |
| --- | --- | --- |
| 当前 Discovery | 四份控制文档、`err.md`、只读本地/GitHub 核验、Issue 评论 | 修改脚本、刷新 Evidence、提交、推送、PR |
| 十路径批准后 | 精确十路径 TDD、轻量验证、实现检查点提交/普通推送、一次 hosted measure、Evidence 入库、PR/Review/CI | 扩范围、手改哈希、复用旧 Artifact、force push、修改 PR #68 |
| 外部治理 | 一次 AGOS ReportOnly 与只读规则查询 | 登记、修复、优化或提交 AGOS |
| 产品边界 | 无产品业务实现写入 | 预算、Ruleset、Gate 5、上游缓存、功能目录、Iced/UI |

## 停止门

- 项目所有者未明确批准十路径与 `sha256:6392a1b4150f2aae0c34c285e83b6870f47e3bdc57def6308d0a26ddd158911d`；
- 需要修改十路径外文件，或需要改动 PR `#68` Final Head；
- 新合同不能同时证明 tests 稳定与 src 敏感；
- hosted 测量任一平台失败、两平台实现哈希不一致或 Artifact 来源不唯一；
- Evidence 入库后本地验证仍非零违规；
- Review 对话未根因闭环、最终 CI 未全绿或成功 Artifact 未清零；
- 最终 Squash Merge 未获得项目所有者对最终 Head 的单独授权。

## 验证命令

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Contract
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

## 外部治理边界

- 仓库根不存在 `.codegraph`，不擅自初始化。
- AGOS 仅允许一次 `invoke-agos-default-entry.ps1 -ReportOnly`；若 `needs-input`、`unregistered`、接口不兼容或异常，记录后立即绕过。
- 不修改 `D:\Android_source\ai-growth-os` 的任何文件；Issue `#69`、本 Session Plan、Runtime Workflow、Git 与 GitHub CI 是本任务执行真源。

## 当前执行证据

- TDD RED：CI 合同唯一新增失败为“普通 crate tests 变化不得改变性能实现哈希”，旧脚本前后哈希分别为 `sha256:5bfa9444ffb93d9d55271cefe969e4a11eb75b4307610c0d91dc3adcb99cfc68` 与 `sha256:8a1a819157bef90f992ce86fceb7587d4159908c95d258d788ea32dbf3e792e9`。
- 最小 GREEN：七个产品组件改为根 `Cargo.toml + src/**/*.rs`；CI 合同 `35/35` GREEN，性能 Contract `ok=true`、零违规。
- 当前实现哈希：`sha256:ed9a8c27972a8b99b331031af171fbf348587481079d62f05f2dc54a88536faa`。
- Hosted 重新测量前，三份旧 Evidence 按预期稳定返回三个 `HASH_MISMATCH`；不得把该预期 RED 当作实现失败。
- 实现检查点：提交 `3d25fafe8b9c085aaaa0069d8e5c93e6afc63eac`，Tree `733319d274a854248d7c706a74333ab8df8d2232`，普通推送。
- Hosted 测量：Run `30205624985` Attempt `1` 的 `contract`、`windows`、`macos`、`required` 四 Job 全绿。
- Artifact：Windows `8633022467` / `sha256:d8737f5039fd58b00f8d9d542fff3deb54b512ed0aa5ec145771726e9f79dd00`；macOS `8632997457` / `sha256:36b2d08185144617c5ea785b86731771127b9601184b8e64e4e971959944a17d`。
- 三份 Evidence 已只从该 Run 入库，本地 Evidence 输出 `ok=true`、`violation_count=0`。
