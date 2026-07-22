# Runtime Workflow：Issue #28 Gate 4 功能目录执行合并证据收口

workflow_status: executing-control-plane-checkpoint
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/28
source_issue_ref: https://github.com/nonononull/inputcodex/issues/26
source_pr_ref: https://github.com/nonononull/inputcodex/pull/27
branch_ref: codex/issue-28-gate-4-feature-catalog-closeout
baseline_ref: a9b20f00ae069aedd42c8124d2789b230187258c
approved_decision_ref: user-message:approve-gate-4-closeout-issue-2026-07-22
scope_amendment_decision_ref: user-message:approve-issue-28-scope-amendment-agents-md-2026-07-22
scope_hash: sha256:a6559a9538dc16908d76c6a0360c1b7c7cb3323158764d244ca891f49d587e1a
allowed_operations: project-doc-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 将已经发生的 Issue #26 / PR #27 合并事实回写项目原生控制面。
executor_enforcement: 当前分支、工作树、允许路径、GitHub Fresh 事实和验证结果任一项异常即停止。
agos_status: bypassed-needs-input-unregistered

## Phase 0：启动与 Fresh 基线

1. 确认隔离工作树分支为 `codex/issue-28-gate-4-feature-catalog-closeout`，工作树干净，基线为 `a9b20f00ae069aedd42c8124d2789b230187258c`。
2. 确认主工作树仍在 `main`；禁止在主工作树写入本 Issue 文件。
3. 读取 `AGENTS.md`、`README.md`、`build.md`、`err.md`、Master Plan、来源报告与本 Session Plan。
4. 运行 AGOS `-ReportOnly`；若为 `needs-input`、`unregistered`、接口不兼容或异常，记录原因并立即使用项目原生流程，禁止修改 AGOS。
5. Fresh 查询 Issue `#26`、PR `#27`、PR CI `29942593564`、main CI `29943399832`、最终 Head、Squash 提交与 Review 对话。

## Phase 1：控制面 checkpoint

1. 新建实施计划、Session Plan、Runtime Workflow 与初始 Closeout 报告。
2. 计算且复算八路径 `scope_hash`；仅允许以下集合：

```text
AGENTS.md
README.md
docs/plans/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md
docs/reports/issue-26-gate-4-feature-catalog.md
docs/reports/issue-28-gate-4-feature-catalog-closeout.md
docs/workflows/2026-07-22-issue-28-gate-4-feature-catalog-closeout-runtime.md
```

3. 项目所有者批准将 `AGENTS.md` 纳入状态回写范围后，只更正该文件中的当前 Gate、Closeout PR 与性能基线锁定状态；不得扩展到产品、性能、上游、CI 或 AGOS 表面。
4. 运行路径、范围哈希、文本控制字节和 `git diff --check` 自检。
5. 普通提交、普通推送、在 Issue `#28` 创建 checkpoint 评论；评论必须包含 commit、`AGENTS.md`、四份控制面文档、范围哈希、范围扩展批准引用和 AGOS 绕过原因。

## Phase 2：来源事实回写

1. 仅修改 README、Master Plan、Issue `#26` 报告与 Issue `#28` Closeout 报告。
2. 回写以下证据：Issue `#26` 关闭、PR `#27` 最终 Head、Squash 提交、单父与 tree 等价、GitHub 签名、PR CI、合并后 main CI、Review 对话、项目所有者授权、远端与本地功能分支清理。
3. 解释控制面漂移根因：来源 PR 合并后其 commit 不可改写，故独立 Closeout PR 是唯一合规的回写路径。
4. 下一步仅指向新的性能基线 Issue；不创建 `benchmarks/`、样本、预算、优化或产品功能。

## Phase 3：本地轻量验证与 PR

1. 使用 `git diff --name-only <baseline>` 与 `git ls-files --others --exclude-standard` 的并集运行八路径白名单与范围哈希验证。
2. 运行来源事实 Fresh 验证、`scripts/ci/Verify-RepositoryPolicy.ps1`、文本控制字节扫描与 `git diff --check`。
3. 运行 `git status --short --branch`，确认无意外文件；普通提交并推送。范围扩展后的新 Head 必须重新触发 CI，旧 Head CI 不得复用。
4. 更新关联的非 Draft PR 正文，列出八路径、最新 `scope_hash`、范围扩展批准、禁止表面、验证命令和“需要独立 owner Squash Merge 授权”。
5. 新 Head push 后回写 Issue `#28`：PR、候选 Head、CI 与 Review 初始状态。

## Phase 4：Review、CI 与合并前门槛

1. 只接受最终 PR Head 的 GitHub-hosted CI 作为合并证据；不得用旧 Head 或来源 PR CI 替代。
2. 每条 Review 对话写明根因、处理方式与验证证据后才可解决；不成立的反馈必须由 reviewer 或项目所有者确认。
3. Fresh 核对 Ruleset、维护者数量、无 bypass、审批数、Squash-only、自动合并关闭、路径集合、CI 和 Review 对话。
4. 取得项目所有者针对该 PR 与最终 Head 的明确 Squash Merge 授权后，才可执行合并。
5. 合并后再次核对 main CI、Issue 关闭、Squash 提交、分支清理和 Master Plan；若需要最终动态回写，创建新的独立收口 Issue，不改写本已合并 PR。

## 失败恢复

- `err.md` 已记录 PowerShell 双引号将 `` `e`` 写成 ESC 的根因；生成 Markdown 时使用补丁文本或单引号字符串，并运行控制字节扫描。
- `err.md` 已记录 Windows 沙箱启动 PowerShell 的偶发拒绝；优先短命令，必要时使用受控提权，不改远端、Git 配置、系统 ACL 或 AGOS。
- 任何范围外变更、网络/CLI 异常或 GitHub 状态不一致都不得用推测补齐；保留证据、停止当前批次并报告。
