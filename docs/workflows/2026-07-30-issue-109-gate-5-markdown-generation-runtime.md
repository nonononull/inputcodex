# Issue #109 Runtime Workflow：受控会话 Markdown 生成

## Runtime Metadata

- `task_id`: `issue-109-gate-5-markdown-generation`
- `tracking_issue_ref`: https://github.com/nonononull/inputcodex/issues/109
- `session_plan_ref`: `docs/plans/sessions/2026-07-30-issue-109-gate-5-markdown-generation.md`
- `approved_decision_ref`: https://github.com/nonononull/inputcodex/issues/108#issuecomment-5130112866
- `selected_business_path`: `gate-5/session-data/markdown-generation`
- `baseline_ref`: `88eaf6301f1897cadaf4da830db998078fb06e97`
- `branch_ref`: `codex/issue-109-gate-5-markdown-generation`
- `candidate_scope_hash`: `sha256:b113da5d41514f50e36cef7d4eb9ade89e2562cbcfe8d392a5173d38fd0ebaac`
- `runtime_state`: `planning-verified`
- `paseo_workspace_ref`: `wks_495c490aa88811f1`
- `agos_status`: `blocked-needs-input-unregistered-bypassed`

## Current Gate

```text
Issue #108 owner decision A
  -> Issue #109 + isolated worktree
  -> NOW: planning freeze
  -> Domain RED/GREEN
  -> Application RED/GREEN
  -> Platform RED/GREEN
  -> Parity RED/GREEN
  -> local closeout
  -> Review/CI
  -> Final Head independent Squash authorization
```

当前 `ALLOWED_OPS`：29 路径内的计划、TDD、文档、验证、Git checkpoint、普通 push 与 PR。

当前 `FORBIDDEN_OPS`：范围外写入、Cargo/依赖、文件保存/UI、任意路径、网络、内部角色或敏感内容披露、Squash Merge、force push、`main` 写入和 AGOS 修改。

## Node Order

### Node 0：Startup Baseline

- 本机时间、分支和 `origin/main@88eaf63` 已核验。
- 四 crate baseline tests 通过，Release Audit `current`，仓库政策零违规，Cargo metadata 与 diff check 通过。
- Git snapshot checkpoint 为 `ready`。

状态：`completed`。

### Node 1：Decision Evidence

- Issue #108 已保存项目所有者原话 `A` 并按 `COMPLETED` 关闭。
- Issue #109 已建立，保存批准范围和禁止边界。

状态：`completed`。

### Node 2：Planning Freeze

1. 写七路径项目原生控制面。
2. 复算 planning `7 / 415982...` 与 candidate `29 / b113da...`。
3. 运行 planning 范围、Release Audit、仓库政策、Cargo metadata 与 diff check。
4. 调用 AGOS default entry `-ReportOnly`；异常按项目规则绕过。
5. 回写 Issue #109 实施范围冻结并建立 planning checkpoint。

状态：`completed`。七路径与 planning hash 精确一致，CI 合同 `35/35`、仓库政策、Release Audit、
Cargo metadata 和 diff check 通过。AGOS ReportOnly 因 task 未登记、operation 不兼容与 protected
replay 要求返回 blocked，已按项目规则记录并绕过，未修改外部控制面。

### Node 3：Domain TDD

1. RED：目标领域 API 缺失。
2. GREEN：消息、UTC 时间戳、文件名、渲染结果与脱敏 Debug。
3. VERIFY：Domain tests、Clippy、fmt。
4. CHECKPOINT：`issue-109-domain-green`。

### Node 4：Application TDD

1. RED：Request、Cancellation、Port、UseCase 缺失。
2. GREEN：输入验证和完成态映射。
3. VERIFY：Application tests、Clippy、fmt。
4. CHECKPOINT：`issue-109-application-green`。

### Node 5：Platform TDD

1. RED：合成 SQLite/JSONL、安全、资源和 UTC 矩阵失败。
2. GREEN：严格只读查询、受控发现、流式解析和错误脱敏。
3. VERIFY：Platform tests、Clippy、fmt，WAL 业务数据不变。
4. CHECKPOINT：`issue-109-platform-green`。

### Node 6：Parity TDD

1. RED：generation feature/contract/fixture 和保存例外证据缺失。
2. GREEN：误命名后端条目重分类；完整导出不标 implemented。
3. VERIFY：Parity tests、Clippy、fmt、Release Audit。
4. CHECKPOINT：`issue-109-parity-green`。

### Node 7：Local Closeout

1. 更新 README 和任务报告；`err.md` 只记录新可复用根因。
2. 运行 `build.md` Issue #109 全门禁和 security review。
3. AGOS ReportOnly 与 Git pre-delivery checkpoint。
4. 建立 `issue-109-local-verified` named snapshot。

### Node 8：Remote Delivery

1. 普通 push。
2. 创建关联 #109 的非 Draft PR。
3. Review 对话逐条记录根因、处理和验证证据。
4. 核验 CI、Performance Baseline 与 Artifact 0。
5. 绑定 Final Head 请求项目所有者独立 Squash Merge 授权。

## Error Watchlist

- `err.md` 先查重；已知 PowerShell `rg` glob、scope hash、WAL SHM 协调和 GitHub 多行正文问题直接复用。
- `fs::read_to_string`、普通 `Connection::open`、真实路径化错误和本机时区一旦出现立即停止。
- 文件名按 UTF-8 字节边界截断，不得切断字符；Debug 不得派生泄露正文。
- RFC 3339 手工规范化必须用日期边界测试证明，不能靠样例推测。
- Source Index 是公开模块一对一映射；不得为保存 UI 发明虚假 source entry。
- fixture 重分类必须用 `git diff --no-renames --name-only` 计算范围，旧删除与新建路径分别计数。

## Checkpoint Rules

- 每个 execute batch、重验证、handoff/pause 前运行 Git snapshot governance `-Checkpoint -ReportOnly`。
- 每层 GREEN 后创建独立普通提交；禁止 amend、rebase、force push 或修改 `main`。
- 所有提交时间使用本机默认时间。

## AGOS Boundary

AGOS 只作外部 ReportOnly 辅助。未登记、`needs-input`、接口不兼容或执行异常时记录并绕过；Issue #109 禁止修改 AGOS Registry、规则、脚本、Workflow 或 Vault。

## Rollout Draft

- `workflow_family`: `gate-5-bounded-session-generation`
- `reusable_path`: strict session ID -> read-only SQLite -> bounded rollout -> deterministic projection
- `skill_usage`: brainstorming、writing-plans、using-git-worktrees、test-driven-development、security-review、verification-before-completion
- `record_after_closeout`: 仅在合并后形成可复用增量时评估 rollout 记录
