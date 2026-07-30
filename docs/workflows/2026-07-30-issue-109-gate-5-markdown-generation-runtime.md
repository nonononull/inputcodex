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
- `runtime_state`: `pr-110-corrective-delivery-dynamic`
- `paseo_workspace_ref`: `wks_495c490aa88811f1`
- `agos_status`: `blocked-needs-input-unregistered-bypassed`

## Current Gate

```text
Issue #108 owner decision A
  -> Issue #109 + isolated worktree
  -> planning freeze
  -> Domain RED/GREEN
  -> Application RED/GREEN
  -> Platform RED/GREEN
  -> Parity RED/GREEN
  -> local closeout
  -> independent review + correction RED/GREEN
  -> final local gate
  -> PR #110 incremental review correction
  -> corrective full local gate
  -> corrective source checkpoint
  -> NOW: github-dynamic-see-pr-110 Review/CI/Artifact
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

状态：`completed`。缺失 API 的 RED 后，专项 `7/7` 与 Domain 全目标测试、Clippy、fmt 通过；
checkpoint `16d0816`。

### Node 4：Application TDD

1. RED：Request、Cancellation、Port、UseCase 缺失。
2. GREEN：输入验证和完成态映射。
3. VERIFY：Application tests、Clippy、fmt。
4. CHECKPOINT：`issue-109-application-green`。

状态：`completed`。缺失 API 的 RED 后，专项 `6/6` 与 Application 全目标测试、Clippy、fmt 通过；
checkpoint `60da5b1`。

### Node 5：Platform TDD

1. RED：合成 SQLite/JSONL、安全、资源和 UTC 矩阵失败。
2. GREEN：严格只读查询、受控发现、流式解析和错误脱敏。
3. VERIFY：Platform tests、Clippy、fmt，WAL 业务数据不变。
4. CHECKPOINT：`issue-109-platform-green`。

状态：`completed`。缺失模块的 RED 后，专项 `17/17` 覆盖只读 SQLite、受控 rollout、WAL、UTC、
资源上限、取消/超时与脱敏；Platform 全目标测试、Clippy、fmt 通过；checkpoint `b4a66b5`。

### Node 6：Parity TDD

1. RED：generation feature/contract/fixture 和保存例外证据缺失。
2. GREEN：误命名后端条目重分类；完整导出不标 implemented。
3. VERIFY：Parity tests、Clippy、fmt、Release Audit。
4. CHECKPOINT：`issue-109-parity-green`。

状态：`completed`。缺失 generation 条目的 RED 后，完整目录验证保持 `43/43/12`，Parity 全目标
测试、Clippy、fmt 与 Release Audit 通过；checkpoint `338359f`。

### Node 7：Local Closeout

1. 更新 README 和任务报告；`err.md` 只记录新可复用根因。
2. 运行 `build.md` Issue #109 全门禁和 security review。
3. AGOS ReportOnly 与 Git pre-delivery checkpoint。
4. 建立 `issue-109-local-verified` named snapshot。

状态：`completed`。Issue #109 本地轻量门禁、安全审查、二十八路径 scope hash、CI 脚本合同、
仓库政策、Release Audit、Cargo metadata 与 Git 空白检查全部通过；`err.md` 无新根因而未修改。

### Node 7.5：Independent Review Correction

1. 只读 reviewer 审查 `origin/main@88eaf630..623e35a`，不取得写入所有权。
2. 父线程逐条复核 `0 Critical / 5 Important / 3 Minor`，禁止盲目照单修改。
3. 接受六项并分别保留 RED/GREEN：目录预分配、SQLite 文本、路径打开竞态、重复 rollout、
   显式深度和闰秒位置。
4. 驳回两项：用户/助手文本按批准合同原样保留，未来 renderer 独立禁网；精确根比较保持
   fail-closed，避免在大小写敏感卷上放宽越界接受。
5. `err.md` 记录新可复用根因，实际范围切换为完整 `29 / sha256:b113da...`。

状态：`completed`。Domain `7/7`、Platform `21/21`、四 crate 全目标 tests/Clippy、rustfmt、
CI 合同 `35/35`、仓库政策、Release Audit、Cargo metadata、静态安全门与
`29 / sha256:b113da...` 范围门全部通过；review-correction named snapshot 待建立。

### Node 7.6：PR 后增量评审修正

1. Fresh reviewer 复核 PR `#110`，返回 `0 Critical / 3 Important / 2 Minor`。
2. 接受 SQLite C 层文本物化与发现式 rollout 按路径重开两个 Important；分别以 SQL 返回前
   限长和同一 `File` 句柄消费测试完成 RED/GREEN。
3. Windows 强文件 ID 与 SQLite 连接句柄同一性记录为残余边界：Rust 1.97.1 稳定 API 与
   `unsafe`/依赖/Cargo 禁令下不可在本切片证明，不伪装成已修复。
4. 驳回静态闰秒历史表；接受动态交付证据归位，统一使用 `github-dynamic-see-pr-110`。
5. VERIFY：Platform Markdown 专项、Platform all-targets Clippy、完整 Issue #109 本地门禁。

状态：`completed`。两项 RED 已确认，最小 GREEN 后 Platform `23/23` 与 Clippy 通过；完整本地
门禁再次达到四 crate 全绿、CI 合同 `35/35`、仓库政策零违规、Release Audit current 及
`29 / sha256:b113da...`。本提交作为 corrective source checkpoint；普通 push 及后续远端状态
只见 `github-dynamic-see-pr-110`。

### Node 8：Remote Delivery

1. 已创建关联 #109 的非 Draft PR `#110`；动态状态只见 GitHub。
2. 完整本地门禁后建立 corrective checkpoint，并普通 push 到现有 PR。
3. Review 对话逐条记录根因、处理和验证证据。
4. 核验 CI、Performance Baseline 与 Artifact 0。
5. 绑定 Final Head 请求项目所有者独立 Squash Merge 授权。

## Error Watchlist

- `err.md` 先查重；已知 PowerShell `rg` glob、scope hash、WAL SHM 协调和 GitHub 多行正文问题直接复用。
- `fs::read_to_string`、普通 `Connection::open`、真实路径化错误和本机时区一旦出现立即停止。
- 目录项必须在 `Vec` 扩张前收到剩余上限；SQLite 文本必须先在 SQL C 层通过 `octet_length`
  与 `?2` 分类，再由借用 `ValueRef` 二次检查。
- SQLite/rollout 打开必须保留 no-follow、组件复验与打开前后文件一致性；发现式匹配必须沿用
  已验证句柄，重复匹配必须失败。
- Windows 稳定 metadata 比较不能表述成强文件 ID；SQLite handle 同一性需要 `unsafe`，在本任务
  禁令内作为显式 ABA 残余边界保留。
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
- `skill_usage`: brainstorming、writing-plans、using-git-worktrees、test-driven-development、requesting-code-review、receiving-code-review、security-review、verification-before-completion
- `record_after_closeout`: 仅在合并后形成可复用增量时评估 rollout 记录
