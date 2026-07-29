# Issue #98 Session Plan：Relay 认证与配置状态只读观察

## Session Metadata

- `task_id`: `issue-98-gate-5-relay-status-observation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/98`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/97#issuecomment-5115176838`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/98#issuecomment-5115957943`
- `selected_business_path`: `gate-5/relay-status-observation`
- `baseline_ref`: `origin/main@b7c4174671caba806162a42e82b7bc0b20f73bf5`
- `branch_ref`: `codex/issue-98-gate-5-relay-status-observation`
- `worktree_ref`: `.worktrees/issue-98-gate-5-relay-status-observation`
- `planning_scope_hash`: `sha256:ec6c88d4a96c351fee85d6c416b04c95b27050893ccbe55b4ad55edfd8d95051`
- `candidate_scope_hash`: `sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5`
- `planning_validation`: `PASSED`
- `execution_state`: `IMPLEMENTATION_APPROVED_DOMAIN_RED_PENDING`
- `agos_report_only`: `needs-input-unregistered-bypassed`

## Mutation Intent

- `mutation_intent`: `source`
- 产品目标：把上游 `tauri-command:relay_status` 拆为独立、只读、最小披露的结构观察能力。
- 当前实际写入：只允许四份 planning 控制面。
- 后续候选写入：仅在所有者批准二十七路径与 candidate hash 后开放。
- 禁止把凭据读取、Provider 配置管理、网络测试或 UI 借本切片带入新架构。

## Executor Enforcement

- 当前阶段：`implementation-tdd`
- 当前写入 allowlist：已批准二十七路径，实际路径必须为其子集。
- 当前允许操作：二十七路径内 TDD、稳定控制面更新、本地轻量验证、Git checkpoint、提交、
  普通 push、非 Draft PR、Review/CI。
- 当前禁止操作：范围外写入、Squash Merge、force push、`main` 写入和任何 AGOS 控制面修改。
- 产品实现前置门：`PASSED`，证据为 `implementation_scope_approval_ref`。
- 最终合并前置门：具体 PR Final Head、Review 根因闭环、Hosted CI 全绿和独立 Squash Merge 授权。

## 本地知识与来源

已读取并比对：

- `AGENTS.md`、`README.md`、`build.md`、`err.md`、`CONTEXT.md`、
  `docs/plans/PROJECT-MASTER-PLAN.md`
- 前七个 Gate 5 切片的 Domain/Application/Platform/Parity 分层模式
- `upstream/CodexPlusPlus/crates/codex-plus-core/src/relay_config.rs`
- 上游 `RelayStatus`、`relay_status_from_home`、`chatgpt_auth_status_from_home`、
  `relay_config_status_from_home` 和 `auth_json_chatgpt_account_label`
- 当前 `SystemPlatformPaths`、设置观察与诊断日志观察的固定路径和有界读取模式
- 本地锁定 `toml_edit 0.25.13+spec-1.1.0` 源码、最低 Rust 与 parse API

仓库没有 `.codegraph/`，未初始化新索引；GBrain 针对 Relay 状态、认证结构和配置完整性的三次查询
均无项目结果。本次 `local_knowledge_lookup` 使用项目原生文档、代码、Git/GitHub 事实和锁定依赖源码
完成。AGOS 只允许 ReportOnly；未登记、`needs-input` 或不兼容时按项目规则记录并绕过。

## 已批准语义

- 新功能：`feature.provider-network.relay-status-observation`
- 唯一来源：`tauri-command:relay_status`
- 固定文档：`CODEX_HOME/auth.json`、`CODEX_HOME/config.toml`
- 单文件上限：`256 KiB`；总预算：`512 KiB`
- 文档状态：`Missing / Valid / Invalid / TooLarge / Unreadable`
- 凭据状态：`Present / Absent / NotObserved`
- 配置状态：`NotConfigured / Complete / Incomplete / NotObserved`
- 两文件均缺失：`LoadCompletion::Empty`
- 至少一份存在：`LoadCompletion::Ready`，并保留两份文档状态
- 平台不支持或路径解析失败：`LoadCompletion::Failed`
- 禁止返回账号标签、Token、Provider ID、Base URL、字段、内容、认证来源和实际路径

## Planning Allowlist

```text
docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/reports/issue-98-gate-5-relay-status-observation.md
docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md
```

- `count`: `4`
- `hash`: `sha256:ec6c88d4a96c351fee85d6c416b04c95b27050893ccbe55b4ad55edfd8d95051`

## Candidate Implementation Allowlist

```text
AGENTS.md
CONTEXT.md
Cargo.lock
Cargo.toml
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/relay_status_observation.rs
crates/inputcodex-application/tests/relay_status_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/relay_status_observation.rs
crates/inputcodex-domain/tests/relay_status_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/relay_status_observation.rs
crates/inputcodex-platform/tests/relay_status_observation.rs
docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/reports/issue-98-gate-5-relay-status-observation.md
docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md
err.md
parity/README.md
parity/contracts/provider-network.yml
parity/features/provider-network.yml
parity/features/source-index.yml
```

- `count`: `27`
- `hash`: `sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5`
- 哈希合同：Ordinal 排序、UTF-8、LF 拼接并保留末尾 LF。

## 执行批次

### Batch 0：Planning Freeze

1. 写入四份 planning 控制面。
2. 复算 planning/candidate 两个 hash。
3. 验证实际路径集精确等于 Planning allowlist。
4. 运行 CI 脚本合同、仓库政策、Cargo metadata、占位符和 diff 检查。
5. 运行 AGOS ReportOnly，并记录可用或绕过事实。
6. 建立本地 planning checkpoint，Issue 回写后停止。

### Batch 1：Domain RED → GREEN

- 先测试三个封闭枚举和聚合值不存在。
- 再实现私有字段、构造与 getter。
- 证明 Debug、类型和 API 不承载字符串、路径或解析器节点。

### Batch 2：Application RED → GREEN

- 先测试零字段 Request、Port、UseCase 和完成态映射。
- 再实现 `ObserveRelayStatus<P>`。
- 保持旧请求类型无法调用新 UseCase。

### Batch 3：Platform RED → GREEN

- 先测试两文件状态矩阵、有界读取、严格解析与最小披露。
- 再实现固定路径、文件类型门禁、双重上限和结构判定。
- 文件级错误进入文档状态；平台/路径级错误进入 Failed。

### Batch 4：Parity RED → GREEN

- 先测试新 feature/contract/source mapping 缺失。
- 再只移动 `tauri-command:relay_status`。
- 原 Relay 总功能与其余入口保持 `unassessed`。

### Batch 5：Local Closeout

- 更新稳定控制面；`err.md` 只记录新根因。
- 运行四 crate tests/Clippy、fmt、Release Audit、范围、隐私和仓库政策验证。
- 建立最终本地 checkpoint，普通 push 和非 Draft PR。

### Batch 6：Review / CI

- 所有 Review 对话根因闭环。
- Hosted CI、Performance observation 和 Artifact 合同全绿。
- 绑定 Final Head 请求独立 Squash Merge 授权。

## Checkpoint 规则

- startup baseline：`b7c4174671caba806162a42e82b7bc0b20f73bf5`
- planning checkpoint：四路径验证通过后建立，仅记录到 Issue `#98`
- implementation checkpoints：Domain、Application、Platform、Parity、local-verified
- 每个 checkpoint 前后核对 `git status --short`、路径 allowlist、scope hash 和 `git diff --check`
- Git 时间只读 `Get-Date`，禁止设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`
- 禁止 amend、rebase 或 force push 改写已公开检查点

## 当前成功标准

- 四份 planning 文件全部存在。
- 实际差异精确为四路径。
- 两个 hash 由独立脚本复算一致。
- CI 脚本合同、仓库政策、Cargo metadata 和 `git diff --check` 通过。
- 无占位符、无产品源码、Cargo、Parity 或稳定项目文档改动。
- Issue `#98` 获得 planning 证据和下一批准原文。

## Stop Conditions

- 未获得二十七路径批准时出现任何产品写入。
- 实际路径不等于 Planning allowlist，或实现后不属于 Candidate allowlist。
- 需要公开路径输入、返回敏感内容或产生写入/网络/子进程/线程/UI 副作用。
- 需要未批准依赖或 `toml_edit` 合同与锁定源码不一致。
- 基线、scope hash 或验证结果不一致。
- AGOS 试图要求修改其自身控制面；此时记录并绕过，不修 AGOS。

## 下一批准原文

```text
批准 Issue #98 二十七路径范围与 candidate_scope_hash
sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5，允许 TDD 实施、
本地轻量验证、Git checkpoint、提交、普通推送、非 Draft PR、Review/CI；最终 Squash Merge
保留单独授权门。
```
