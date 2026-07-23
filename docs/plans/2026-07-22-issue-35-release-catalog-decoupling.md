# Issue #35：Release 快照与功能目录审计基线解耦实施计划

plan_status: approved-scope-extended-for-local-time-rule
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/35
branch_ref: codex/issue-35-release-catalog-decoupling
baseline_ref: 939f3454b34e0faa42897be7489b344f2bec1d4c
approved_decision_ref: user-message:approve-issue-35-14-path-scope-2026-07-22
scope_extension_approval_ref: user-message:write-local-machine-time-rule-to-project-agents-2026-07-22
scope_hash: sha256:42bc11297aa5d91ff876ceb17296cef337034a11c041f565b642abae20c48a4c
allowed_operations: project-doc-write, rust-validation-write, test-write, ci-script-write, workflow-write, source-lock-metadata-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 建立可诊断的上游快照与功能目录审计基线解耦状态；不更新上游快照字节、不迁移产品功能、不建立性能基线或预算。
executor_enforcement: 仅允许下列十五路径；范围外写入、上游事实漂移、未解决 Review、CI 失败、缺少最终所有者 Squash 授权均停止推进。
agos_status: bypassed-needs-input-unregistered
owner_merge_authorization_ref: none

## 一、根因与目标

上游 `v1.2.42` 的完整缓存与现有 `v1.2.41` 功能目录存在自然的复审间隔。旧验证把 `source-lock.snapshot` 同时当作缓存真相和审计真相：纯缓存同步会令目录验证失败，反过来机械更新目录 Release 会伪造未审计的一致性。

本任务将二者解耦：缓存真相仍由 `snapshot` 表示，目录审计真相由 `release_audit.catalog_release` 表示。合法 stale 必须显式标记、具备根因和重新审计 Issue，并在合并门上阻断性能与产品路径。

## 二、Fresh 基线

- Issue `#35` 在实施开始时保持 `OPEN`；所有者批准评论为 `https://github.com/nonononull/inputcodex/issues/35#issuecomment-5051128101`，并已移除 `status:needs-owner-decision`。
- 项目所有者随后明确批准将本机时间规则写入 `AGENTS.md`；该路径扩展不改变 Release 审计语义、不授予合并权限。
- 远端 `main`、当前隔离工作树 Head 均为 `939f3454b34e0faa42897be7489b344f2bec1d4c`，分支干净且不存在该分支的既有 PR。
- 上游最新正式 Release 为 `v1.2.42`，提交为 `657cd33e009ad02515d30db6492cd4e669b06318`；当前缓存与目录审计基线均仍为 `v1.2.41` / `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- AGOS 默认入口已返回 `needs-input`、`unregistered` 且 doctor 为 `blocked`；依项目规则记录为外部缺口并绕过，不修改任何 AGOS 跨仓表面。

## 三、精确写入范围

按路径升序、LF 分隔、末尾保留 LF 后计算顶部 `scope_hash`：

1. `.github/workflows/ci.yml`
2. `AGENTS.md`
3. `build.md`
4. `crates/inputcodex-parity/src/validation.rs`
5. `crates/inputcodex-parity/tests/catalog_repository.rs`
6. `docs/adr/0003-release-snapshot-catalog-audit-decoupling.md`
7. `docs/plans/2026-07-22-issue-35-release-catalog-decoupling.md`
8. `docs/plans/PROJECT-MASTER-PLAN.md`
9. `docs/plans/sessions/2026-07-22-issue-35-release-catalog-decoupling.md`
10. `docs/reports/issue-35-release-catalog-decoupling.md`
11. `docs/workflows/2026-07-22-issue-35-release-catalog-decoupling-runtime.md`
12. `err.md`
13. `scripts/ci/Test-CiScripts.ps1`
14. `scripts/ci/Verify-ReleaseAuditGate.ps1`
15. `upstream/source-lock.json`

禁止修改 `upstream/CodexPlusPlus/` 任意快照字节、产品 `apps/`、非 parity Rust crate、`Cargo.toml`、`Cargo.lock`、`benchmarks/`、Release、Ruleset、AGOS 或任何分支删除表面。

## 四、实现合同

1. `source-lock.json` 增加 `release_audit`，当前状态固定为 `current`，目录 Release 与现有 `v1.2.41` 快照相同。
2. Rust 验证仅公开 `RepositorySummary::requires_reaudit()`；feature catalog 与 `source-index` 对齐 `catalog_release`，不再直接对齐活动 `snapshot`。
3. `current` 要求两套 Release 完全相同且 stale 字段为 `null`；`stale-re-audit-required` 要求两套 Release 不同、根因非空且 Issue URL 有效。其他组合均为验证错误。
4. PR 门禁复用 `Collect-Changes.ps1`：stale 阻断性能、应用、产品 crate 与 Cargo 路径；同一 PR 改动实际 `release_audit`、`upstream/source-lock.json` 和受阻路径必须失败。非 PR 只校验状态结构。
5. `required` Job 必须依赖独立 `release-audit` Job；所有失败证据 Artifact 最多保留七天。

## 五、TDD 与验证顺序

1. 先在 `catalog_repository.rs` 写入 current、合法 stale 与非法 stale 的临时副本用例；在 `Test-CiScripts.ps1` 写入门禁路径合同。
2. 观察 RED：门禁脚本缺失时 CI 合同退出 `10`；缺少 `requires_reaudit()` 时 Rust 定向测试以 `E0599` 失败。
3. 最小实现状态模型、验证、脚本、`source-lock` 元数据和 CI 接线。
4. 本地只运行定向 Rust 测试、CI 脚本合同、仓库策略、格式和差异检查；全 Workspace 与 Windows/macOS 构建继续交给 GitHub-hosted runners。
5. 普通 commit、普通 push、创建一个非 Draft PR；解决全部 Review 对话并等待 CI。最终 Squash Merge 必须另获所有者对最终 Head 的明确授权。

## 六、停止条件

- 上游最新正式 Release、远端 `main` 或批准范围哈希发生漂移。
- 需要更新 `upstream/CodexPlusPlus/` 快照、功能目录实质内容、性能基线、预算或产品行为。
- 任一 stale 组合无法由可复核 Issue、根因和目录复审证据解释。
- 本地定向验证、PR CI、Review 对话或仓库策略未通过。
