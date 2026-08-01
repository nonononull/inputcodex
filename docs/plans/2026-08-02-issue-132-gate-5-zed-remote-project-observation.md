# Issue #132 Gate 5 Zed 远程项目只读观察实施计划

> **状态：** `planning-completed-domain-red-next`
>
> **执行纪律：** 使用项目原生 TDD 与 `build.md` 定向命令执行 RED -> GREEN -> VERIFY；当前 Codex 执行器保持唯一写入，Final Head 必须另行独立只读复审。

**目标：** 从固定 Codex 全局状态与受控 SQLite 中识别 Zed 远程项目，只返回稳定假名、来源、选择提示和来源覆盖，不接管完整 Zed Remote 工作流。

**架构：** Domain 固定最小输出；Application 提供零字段请求、Cancellation、Port 和 UseCase；Platform 使用有界 no-follow JSON 与严格只读 SQLite；Parity 只移动 `tauri-command:list_zed_remote_projects`。

**技术栈：** Rust 1.97.1、现有七成员 Workspace、`serde_json`、`rusqlite`、精确锁定 `sha2 0.10.9`、现有 `LoadCoordinator` 与 Parity YAML；不新增异步运行时、URL、UUID、HMAC 或网络依赖。

## 任务元数据

- `tracking_issue_ref`: https://github.com/nonononull/inputcodex/issues/132
- `approved_decision_ref`: https://github.com/nonononull/inputcodex/issues/131#issuecomment-5152838531
- `implementation_scope_approval_ref`: https://github.com/nonononull/inputcodex/issues/132#issuecomment-5152853748
- `standing_authorization_ref`: https://github.com/nonononull/inputcodex/issues/111
- `branch_ref`: `codex/issue-132-gate-5-zed-remote-project-observation`
- `baseline_ref`: `da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4`
- `planning_scope_count`: `7`
- `planning_scope_hash`: `sha256:543b946874f01849a30f1098ab298cb86d7d24df9cc97b6c9082c27d71d27de5`
- `candidate_scope_count`: `28`
- `candidate_scope_hash`: `sha256:7ee5d47dca72d0d2f1ec683cc45e4bbac0e3ce40af1e57417d39608c8c0c26bb`

## 已批准语义

- 零字段请求；固定读取 `CODEX_HOME/.codex-global-state.json` 与受控 SQLite `threads.cwd`。
- 排除 legacy `~/.codex-session-delete/zed_remote_projects.json`，不依赖 open/remember/forget 写入结果。
- 输出只含完整 SHA-256 稳定假名、origin、`SelectedHostHint / NotObserved` 与来源覆盖。
- `SelectedHostHint` 只表示全局状态选择提示，不表示 Zed 运行、项目打开或远端可达。
- `Partial` 作为 `Ready(snapshot.coverage=Partial)`；取消、超时、碰撞和总量越界始终 `Failed`。
- Windows/macOS 使用相同 hash vector、排序、去重、资源和错误语义。

## Task 1：Planning Freeze

- [x] Issue #130 性能优先延期和 Issue #131 方案 B 决策已完成。
- [x] 冻结 7 条 planning 与 28 条 candidate Ordinal 路径 hash。
- [x] live 自治状态为 `active-issue-planning`，base/head、单 writer、Release Audit 和仓库政策通过。
- [x] 写入七路径项目原生控制面并完成 CI 合同 `76/76`、策略、Release Audit、仓库政策、范围和空白检查。

## Task 2：Domain RED -> GREEN

- [ ] RED：固定 ID 格式、三种 origin、两种选择提示、来源覆盖、项目/来源计数和脱敏 Debug。
- [ ] GREEN：实现纯领域值；禁止原始身份材料、label、路径、URL 或时间戳字段。
- [ ] 运行 Domain 专项、全目标 Clippy 与 rustfmt，建立 domain checkpoint。

## Task 3：Application RED -> GREEN

- [ ] RED：固定零字段 Request、Cancellation、Port、Ready/Partial/Empty/Failed 与取消后迟到 `Stale`。
- [ ] GREEN：复用 `LoadCompletion` 与 `LoadCoordinator`，不新增通用 Partial 状态。
- [ ] 运行 Application 专项、全目标 Clippy 与 rustfmt，建立 application checkpoint。

## Task 4：Platform RED -> GREEN

- [ ] RED：SHA-256 向量、None/22 区分、碰撞、敏感值零输出、JSON/SQLite 资源矩阵、NOFOLLOW、超时取消和 WAL 只读证据。
- [ ] GREEN：实现固定路径、有界同句柄 JSON、严格只读 SQLite、确定性匹配/排序和完整性分类。
- [ ] 仅添加 `sha2 0.10.9` 依赖闭包；运行 Platform 专项、Clippy 与 rustfmt，建立 platform checkpoint。

## Task 5：Parity RED -> GREEN

- [ ] RED：要求新 feature/contract/fixture 与 `[filesystem-read, database-read]`；旧 Zed 总功能不得再包含 list。
- [ ] GREEN：只移动 `list_zed_remote_projects`；旧 core/open/forget 与旧完整 fixture 保持。
- [ ] 计数固定为 `135/46/46/13/11/3/0`，建立 parity checkpoint。

## Task 6：文档、本地门禁与远端交付

- [ ] 更新稳定 README、术语、主计划、报告和构建命令，不在 README 写动态 Head/CI。
- [ ] 运行 `build.md` Issue #132 全部轻量门禁，实际路径精确为 28。
- [ ] 普通 push、非 Draft PR、独立 Final Head 只读复审、CI 7/7、Performance 4/4、Artifact 0、Review thread 0。
- [ ] 精确 Head Squash Merge；验证单父、tree、签名与 main 双 Workflow；关闭 Issue 并归档。

## 停止门

- 需要读取 legacy registry、SSH config、认证材料，返回任何原始远程身份/路径/URL/内容，或引入持久化密钥。
- 需要网络、子进程、写入、Zed 启动、UI、unsafe、第二个 feature 或 `sha2 0.10.9` 外的直接依赖。
- Release、Ruleset、许可证、付费资源或精确 Head/Review/CI/Artifact 门禁失败。
