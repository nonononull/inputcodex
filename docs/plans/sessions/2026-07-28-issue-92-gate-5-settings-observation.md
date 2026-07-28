# Issue #92 Session Plan：设置只读观察

## Session Contract

- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/92`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/91`
- `approved_design_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107059878`
- `owner_planning_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107673827`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5108221117`
- `baseline_ref`: `origin/main@a5559f4a873a81d91ed09b571503523a78a45118`
- `branch_ref`: `codex/issue-92-gate-5-settings-observation`
- `worktree_ref`: `.worktrees/issue-92-gate-5-settings-observation`
- `planning_scope_hash`: `sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`
- `candidate_scope_hash`: `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`
- `delivery_contract`: `Issue → 设计批准 → 分支/worktree → Session Plan → Runtime Workflow → scope_hash 批准 → TDD → 本地验证 → PR → Review/CI → 独立 Squash Merge 授权`
- `current_phase`: `REMOTE_DELIVERY`
- `mutation_intent`: `approved-product-implementation`
- `executor_enforcement`: `二十七路径候选 allowlist；路径扩大必须重新批准`

## Global Constraints

- 全程中文；软件名称固定为 `inputcodex`。
- 性能、稳定性、隐私和可诊断性优先；Windows/macOS 语义一致。
- 业务代码只使用 Rust；禁止 TypeScript、JavaScript、WebView 和注入。
- Iced 不进入 Domain、Application、Platform、Infrastructure 或 Parity。
- 只读一个由平台路径能力解析的设置文件，不接受任意路径。
- 不返回设置键、值、原始 JSON、私人路径、凭据或脚本内容。
- 不联网、不写文件、不改环境、不启动线程、Watcher 或子进程。
- 不修改 `upstream/`、Workflow、Ruleset、预算、Release 或 AGOS。
- 本地只执行轻量验证；Workspace 与双平台全量验证交给 GitHub-hosted runners。
- Git 时间使用 Windows 本机默认时间，不设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。
- 最终 Squash Merge 始终保留单独授权门。

## 决策与知识查询

Issue `#91` 已批准“设置只读观察与设置管理总功能分离”；Issue `#92` 的方案 A 已获批准。
本 Session 不重新引入完整设置字段、保存、重置、Provider、Relay、脚本或皮肤。

本次 Discovery 已读取：

- `README.md`、`AGENTS.md`、`build.md`、`err.md`、`CONTEXT.md`；
- `docs/plans/PROJECT-MASTER-PLAN.md`；
- Issue `#86/#89` 的设计、Session Plan、Runtime Workflow 与报告；
- `PlatformPathsSnapshot::settings_file()`、`PrivatePath`、`LoadCompletion`、`LoadCoordinator`；
- Domain/Application/Platform 的既有只读观察模式；
- foundation-platform feature/contract/source-index；
- 上游 `SettingsStore::load` 和三个设置 command；
- 上游 Release 锁文件中的 `serde_json 1.0.149`。

项目原生控制面足以执行。AGOS 只可作为可选 `ReportOnly` 辅助；不可用或 `needs-input`
立即记录并绕过，禁止跨仓修复或优化 AGOS。

## Change Contract

### Inputs

- 零字段 `SettingsObservationRequest`；
- 既有平台路径能力解析出的 `settings_file`；
- 单个设置文件的元数据和最多 `256 KiB + 1` 字节。

### Outputs

- `LoadCompletion::Ready(SettingsDocumentObservation)`；
- `LoadCompletion::Empty`，语义为 `NotConfigured`；
- 或稳定 `ApplicationError`；
- 不返回路径、键、值、原始 JSON、凭据或脚本内容。

### Side Effects

- 只允许 `filesystem-read`；持久化固定为 `none`。
- 禁止 filesystem-write、environment-write、network、process-control、UI 和注入。

## 当前允许操作

1. 使用批准基线上的隔离分支/worktree。
2. 修改四份 task-local 规划控制面。
3. 复算 planning/candidate scope hash。
4. 执行 Markdown、范围、占位符、受保护路径和 Git diff 轻量检查。
5. 在 Issue `#92` 回写范围与证据并请求下一批准。

项目所有者已批准候选范围，现允许在二十七路径 allowlist 内执行产品 TDD、本地轻量验证、
Git checkpoint、普通提交、普通推送、非 Draft PR 和 Review/Hosted CI。最终 Squash Merge 仍禁止。

## 当前规划范围

```text
docs/plans/2026-07-28-issue-92-gate-5-settings-observation.md
docs/plans/sessions/2026-07-28-issue-92-gate-5-settings-observation.md
docs/reports/issue-92-gate-5-settings-observation.md
docs/workflows/2026-07-28-issue-92-gate-5-settings-observation-runtime.md
```

- `count`: `4`
- `hash`: `sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`
- 算法：`StringComparer.Ordinal` 排序、UTF-8、LF 拼接、末尾保留 LF。

## 候选实施范围

候选范围为主设计稿列出的 `27` 路径：

`sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`

任何路径变化必须同步四份规划文件、复算哈希并重新批准；实际实现可少改，不能越界。

## 执行批次

### Batch 0：规划冻结

- [x] Issue、设计批准、基线、分支/worktree 和四份规划控制面就绪。
- [x] 完成范围、哈希、占位符、受保护路径和 diff 检查。
- [x] AGOS 命中 `err.md` 已记录的外部任务未登记同根因，按项目规则绕过。
- [x] 回写 Issue 并停在范围批准门。

### Batch 1：Domain RED → GREEN

- 测试先证明 `SettingsDocumentObservation` 不存在。
- 最小实现零/非零条目数量和隐私 Debug。
- 运行 Domain tests/Clippy，建立 `issue-92-domain-green` checkpoint。

### Batch 2：Application RED → GREEN

- 测试先证明 Request/Port/UseCase 不存在。
- 固定 Some→Ready、None→Empty、Err→Failed；合法零条目仍为 Ready。
- 验证 LoadCoordinator 兼容，运行 tests/Clippy，建立 checkpoint。

### Batch 3：Platform 与 Cargo RED → GREEN

- 使用模块私有探针覆盖缺失、文件类型、超限、I/O、JSON 和错误根。
- 增加精确 `serde_json 1.0.149`，通过 Cargo 更新锁文件并拒绝无关漂移。
- 实现平台路径、`symlink_metadata`、双重上限、解析和稳定错误。
- 集成测试只验证公开 Port，不公开任意路径 API。
- 运行 tests/Clippy/fmt/offline，建立 `issue-92-platform-green` checkpoint。

### Batch 4：Parity RED → GREEN

- 先写缺失子能力、错误来源归属和计数 RED。
- 新增 feature/contract，只移动 `load_settings`。
- 固定原总功能 `unassessed`、`39/39`、source `133`、fixture `11`。
- 运行 tests/Clippy，建立 `issue-92-parity-green` checkpoint。

### Batch 5：项目控制面与本地收口

- 更新稳定项目入口、Gate 事实、构建命令和静态证据。
- README 禁止写 Head、Run、Artifact 或合并流水账。
- `err.md` 没有新根因则保持不变。
- 运行四 crate 轻量验证、格式、范围、隐私和禁止面检查。
- 建立 `issue-92-local-verified` checkpoint。

### Batch 6：远端交付

- 仅普通提交、普通 push 和非 Draft PR；禁止 force push。
- Review 对话逐条记录根因、处理和验证后解决。
- Hosted CI 与 Performance observation 核验预期 Job 和 Artifact。
- Final Head 全绿后只请求独立 Squash Merge 授权，不创建递归 Closeout。

## 文件职责

- `AGENTS.md`、`CONTEXT.md`、Master Plan：实现完成后固化稳定 Gate 事实。
- `README.md`：只增加稳定用户能力。
- `Cargo.toml`、`Cargo.lock`、Platform Cargo：只增加批准依赖及必要锁项。
- Domain/Application/Platform：按分层边界实现，不泄露框架或路径类型。
- Parity：新增子能力、合同、来源映射和仓库级断言。
- `build.md`：记录可重复轻量命令；`err.md`：只记录新根因。
- task-local docs：保存本 Issue 计划、运行合同和静态证据。

## TDD 与 Checkpoint

| 阶段 | RED | GREEN | Checkpoint |
| --- | --- | --- | --- |
| Domain | 领域类型不存在 | 数量事实、隐私 Debug | `issue-92-domain-green` |
| Application | Request/Port/UseCase 不存在 | Ready/Empty/Failed | `issue-92-application-green` |
| Platform | 文件边界和错误不存在 | 有界单文件读取 | `issue-92-platform-green` |
| Parity | 子能力与映射不存在 | `39/39`、source `133` | `issue-92-parity-green` |
| Closeout | 稳定控制面未更新 | 静态报告与禁止面 | `issue-92-local-verified` |

## 写入强制

每批前后必须执行：

1. `git status --short`；
2. 合并 `git diff --name-only` 与未跟踪文件形成实际路径集；
3. 按 `StringComparer.Ordinal` 排序并验证 allowlist；
4. `git diff --check`；
5. 需要新路径时立即停止并重新批准。

规划阶段实际差异必须精确等于四份规划文件。

## 禁止能力与隐私扫描

实现阶段必须拒绝：网络客户端与 socket；`Command::new`、线程、Watcher、轮询；写文件、
改环境、注册表写入；Tauri/WebView/JavaScript/注入/广告/远程推荐；路径、键、值、原始 JSON、
解析消息或凭据输出；公共任意路径 API；误迁移 `core-module:settings`、save/reset；把原总功能
标记为 implemented。

## 本地验证合同

规划阶段执行 `git diff --check`、四路径精确比较、两个哈希复算和占位符扫描。

批准后的实现阶段最低执行：

```powershell
cargo test -p inputcodex-domain --offline
cargo test -p inputcodex-application --offline
cargo test -p inputcodex-platform --all-targets --offline
cargo test -p inputcodex-parity --offline
cargo clippy -p inputcodex-domain --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-application --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-platform --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-parity --all-targets --offline -- -D warnings
cargo fmt --all -- --check
```

Workspace 全量、Windows/macOS 编译和 Performance observation 交给 GitHub-hosted runners。

## 错误恢复

- 先查 `err.md`；重复 Patch、PowerShell、CRLF 和原生命令问题引用既有条目。
- Cargo 失败先确定缓存、索引或锁文件根因，禁止手写锁项。
- 难以模拟 I/O 时使用模块私有探针，禁止公开任意路径产品 API。
- 字段级设置需求回到新的功能或一致性例外 Issue。
- AGOS 不可用立即绕过，禁止跨仓修复。

## 当前进度

- [x] 决策、实现 Issue、设计批准、分支/worktree 和规划落盘。
- [x] 规划轻量验证和 AGOS 同根因绕过处置完成。
- [x] Issue 回写候选范围：`https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107913027`。
- [x] 项目所有者批准 `27` 路径和哈希：`https://github.com/nonononull/inputcodex/issues/92#issuecomment-5108221117`。
- [x] Domain/Application/Platform/Parity TDD、五个 named checkpoint 与本地轻量验证完成。
- [x] 稳定项目控制面回写完成；实际使用 `26/27` 路径，`err.md` 保持不变。
- [x] Cargo 只新增 `serde_json 1.0.149` 与必要锁包 `zmij 1.0.23`，许可证边界已核验。
- [ ] 普通推送、非 Draft PR、Review/Hosted CI 与独立 Squash Merge 授权。

## 停止条件

- 未获范围批准、基线漂移、实际 diff 越界或需要第 `28` 条路径。
- 需要公开任意路径、网络、写入、线程、子进程、UI、注入或 `unsafe`。
- 无法保护设置敏感数据，或无法区分缺失、空对象、损坏和错误根。
- Release 审计不为 current；Review/CI 根因未闭环；未获最终合并授权。
