# Issue #89 Session Plan：Relay 环境只读观察

## Session Contract

- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/89`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/88`
- `baseline_ref`: `origin/main@db0c09b9df272887deb9407a5e344cf87a59dda8`
- `branch_ref`: `codex/issue-89-gate-5-relay-environment-observation`
- `worktree_ref`: `.worktrees/issue-89-gate-5-relay-environment-observation`
- `planning_scope_hash`: `sha256:0a301df75edda05c8d3d1c01c91221dd9ac8ff11aeeca39fb4c26a293b0543b0`
- `candidate_scope_hash`: `sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77`
- `delivery_contract`: `Issue → 分支/worktree → Session Plan → Runtime Workflow → scope_hash 批准 → TDD → 本地验证 → PR → Review/CI → 独立 Squash Merge 授权`
- `current_phase`: `FINAL_LOCAL_VERIFICATION_PENDING`

## Global Constraints

- 全程中文；软件名称固定为 `inputcodex`。
- 性能、稳定性和可诊断性优先；Windows/macOS 功能一致但覆盖必须真实。
- 业务代码只使用 Rust；禁止 TypeScript、JavaScript、WebView 和注入。
- Iced 不进入 Domain、Application、Platform、Infrastructure 或 Parity。
- 不联网、不写文件、不改环境、不启动线程、Watcher 或子进程。
- 不读取 `.env` 内容，不返回代理变量值，不记录实际路径。
- 不修改 `upstream/`、Workflow、Ruleset、预算、Release 或 AGOS。
- 本地只执行 `build.md` 定义的轻量验证；Workspace 与双平台全量验证交给 GitHub-hosted runners。
- 所有时间证据只使用 Windows 本机 `Get-Date`。
- 最终 Squash Merge 始终保留单独授权门。

## Brainstorming 与决策

项目所有者已批准 Issue `#88` 的“Relay 环境只读观察与网络环境总功能分离”设计。当前 Session 不再重新讨论系统代理、网络测试或写入能力，只负责把批准语义转化为可测试的纯 Rust 分层实现计划。

## Local Knowledge Lookup

本次 Discovery 已读取并交叉核对：

- `README.md`、`AGENTS.md`、`build.md`、`err.md`、`CONTEXT.md`；
- `docs/plans/PROJECT-MASTER-PLAN.md`；
- Issue `#86` 的设计、Session Plan、Runtime Workflow 与实施报告；
- `parity/features/provider-network.yml`、`parity/contracts/provider-network.yml`、`parity/features/source-index.yml`；
- 上游 `relay_environment.rs`、`proxy.rs` 与 `check_relay_environment`；
- 现有平台路径和运行时环境观察实现；
- `windows-registry 0.6.1` 的官方 crate 元数据和安全 Registry API。

结论：AGOS 不是本任务执行依赖，当前项目原生控制面完整；不运行或修改外部 AGOS。

## Change Contract

### Inputs

- 零输入 `RelayEnvironmentObservationRequest`；
- 当前进程环境；
- Windows 两个固定持久化环境键；
- 解析后的用户目录与 `CODEX_HOME`；
- 四个固定 Clash Verge 配置逻辑来源。

### Outputs

- `LoadCompletion::Ready(RelayEnvironmentObservation)`；
- 或稳定 `ApplicationError`；
- 不返回变量值、`.env` 内容、实际路径或注册表值。

### Side Effects

- 只允许 `environment-read` 与 `filesystem-read`；
- Windows 注册表读取归入 `environment-read`；
- 持久化固定为 `none`。

## 当前规划写入范围

```text
docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md
docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md
docs/workflows/2026-07-28-issue-89-gate-5-relay-environment-observation-runtime.md
```

共 `3` 路径，哈希为
`sha256:0a301df75edda05c8d3d1c01c91221dd9ac8ff11aeeca39fb4c26a293b0543b0`。

## 候选实施范围

候选范围固定为设计稿列出的 `30` 路径，哈希为
`sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77`。

任何路径变化必须先更新三份规划文件、重新计算哈希并取得批准。实现可以少改，不能越界或为了匹配清单制造差异。

## Task 0：规划控制面与批准门

- [x] 创建并关闭一致性决策 Issue `#88`；
- [x] 创建实现 Issue `#89`；
- [x] 从最新 `origin/main` 建立隔离分支和 worktree；
- [x] 完成上游、目录、架构与依赖 Discovery；
- [x] 落盘设计、Session Plan 与 Runtime Workflow；
- [x] 运行规划范围、哈希、占位符、受保护路径和 `git diff --check`；
- [x] 回写 Issue `#89` 候选范围与验证证据；
- [x] 取得项目所有者对 `30` 路径和哈希的明确批准。

## Task 1：Domain TDD

- [x] 为五个固定代理变量名称编写 RED；
- [x] 为来源、覆盖状态、稳定排序和去重编写 RED；
- [x] 为 `.env` 三态编写 RED；
- [x] 为四类 Clash 来源和五种候选状态编写 RED；
- [x] 为聚合风险、空事实 Ready 和隐私 Debug 编写 RED；
- [x] 最小实现并运行 Domain tests/Clippy；
- [x] 建立 Domain Git checkpoint。

## Task 2：Application TDD

- [x] 为零输入 Request、Port 和 UseCase 编写 RED；
- [x] 固定 Ready、局部不可用 Ready、硬失败 Failed；
- [x] 复用请求身份、取消、超时和过期结果合同；
- [x] 运行 Application tests/Clippy；
- [x] 建立 Application Git checkpoint。

## Task 3：共享 Platform TDD

- [x] 从 `platform_paths` 提取窄 `CODEX_HOME` 解析入口并保持既有测试；
- [x] 固定进程环境单次扫描和名称规范化；
- [x] 固定 `.env` 元数据检查且不读取内容；
- [x] 固定四候选去重、有界读取、UTF-8 和顶层布尔解析；
- [x] 固定非目标平台 unsupported；
- [x] 建立共享 Platform checkpoint。

## Task 4：Windows/macOS Platform TDD

- [x] Windows target 加入精确版本 `windows-registry = 0.6.1`；
- [x] 更新 `Cargo.lock`，禁止其他无关依赖漂移；
- [x] Windows 固定当前进程、用户级、系统级来源及 Unavailable；
- [x] macOS 固定当前进程来源和两个 NotObserved；
- [x] 双平台固定 Clash 候选路径规则，不返回路径；
- [x] 运行 Platform tests/Clippy 和格式；
- [x] 建立 Platform checkpoint。

## Task 5：Parity TDD

- [x] 先让目录测试因缺少新子能力失败；
- [x] 新增 feature 与 contract；
- [x] 迁移 `relay_environment` 和 `check_relay_environment` 两个来源；
- [x] 原总功能只保留 `proxy` 且继续 `unassessed`；
- [x] 固定 implemented 数、来源归属、副作用和 Release current；
- [x] 运行 Parity tests/Clippy；
- [x] 建立 Parity checkpoint。

## Task 6：项目控制面收口

- [x] 更新 README、CONTEXT、AGENTS、Master Plan 与 parity README；
- [x] 在 `build.md` 固定 Issue `#89` 轻量命令和禁止面；
- [x] 只有新根因才更新 `err.md`，重复问题引用既有条目；
- [x] 创建实施报告并写入静态本地证据；
- [x] 不写递归 Closeout 或动态合并证据。

## Task 7：验证与远端交付

- [ ] 四 crate 定向 tests/Clippy 与 rustfmt；
- [ ] 目录、合同、来源、范围、隐私和禁止能力验证；
- [ ] Git snapshot governance 与最终本地 checkpoint；
- [ ] 普通提交、普通推送和非 Draft PR；
- [ ] Review 对话逐条根因闭环；
- [ ] Hosted CI、Performance observation 和 Artifact 核验；
- [ ] Final Head 独立 Squash Merge 授权。

## 当前授权

项目所有者已批准 `30` 路径与 `candidate_scope_hash`，允许 Task `1-7` 的 TDD、实现、本地轻量验证、Git checkpoint、提交、普通推送、非 Draft PR、Review/CI。最终 Squash Merge 继续保留单独授权门。

## 停止条件

- 精确范围未批准；
- 需要新增第 `31` 条路径；
- 需要网络、写入、子进程、线程、UI、注入或 `unsafe`；
- `windows-registry` 产生不兼容 Rust、许可证或无关锁文件漂移；
- 无法在不泄露值/路径的前提下表达结果；
- Windows/macOS 无法提供同一产品结构和真实覆盖；
- `release_audit` 不再为 `current`；
- Review 或 CI 根因未闭环；
- 未取得独立 Squash Merge 授权。
