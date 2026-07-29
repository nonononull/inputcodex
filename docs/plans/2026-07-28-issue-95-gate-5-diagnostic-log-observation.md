# Issue #95 Gate 5 诊断日志只读结构观察设计与实施计划

## 文档状态

- `status`: `IMPLEMENTATION_AUTHORIZED`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/95`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/94#issuecomment-5109582183`
- `design_ref`: `https://github.com/nonononull/inputcodex/issues/95`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/95#issuecomment-5113647548`
- `baseline_ref`: `origin/main@9587549c3f1bb334507075499f806485d83fce6a`
- `branch_ref`: `codex/issue-95-gate-5-diagnostic-log-observation`
- `worktree_ref`: `.worktrees/issue-95-gate-5-diagnostic-log-observation`
- `planning_scope_hash`: `sha256:14d78bf1a92f5b8db58650b501fb0cebee59a329823ff49e7b8ff3e93e0b7231`
- `candidate_scope_hash`: `sha256:8d407c269436c655e12ff94035183de6aa50dc7759fbc75f9cb7b6f9b0349d38`

## 目标

以纯 Rust 迁移第七个 Gate 5 产品切片：只读观察既有平台路径能力定位的诊断日志文件，
仅返回有界尾部样本的结构计数与截断事实，不返回日志正文、路径或事件内容。

本切片新增 `feature.foundation-platform.diagnostic-log-observation`，只接管
`tauri-command:read_latest_logs`。原 `feature.foundation-platform.diagnostics` 继续
`unassessed`。

## 已批准设计边界

1. 请求为零字段，不接受任意路径、读取长度、过滤器或事件条件。
2. 只通过现有 `SystemPlatformPaths` 取得私有诊断日志路径。
3. 文件不存在返回 `LoadCompletion::Empty`，稳定原因是 `NoDiagnosticLog`。
4. 合法空普通文件返回 `Ready`，所有计数为 `0`。
5. 非空文件只读取尾部最多 `256 KiB`，禁止读取完整大日志。
6. 从文件中部开始采样时，丢弃第一个可能不完整的记录片段并显式留证。
7. 完整非空记录只有解析为 JSON object 才计为合法；空行、非法 UTF-8、损坏 JSON 和
   非 object JSON 均计为 malformed。
8. 返回值不得包含正文、字段、事件名、detail、PID、时间戳、实际路径、用户名、机器名或凭据。
9. 非普通文件、元数据失败、Seek/Read 失败必须明确失败，不得伪造成缺失或空日志。
10. 不联网、不写文件、不调用子进程、不启动线程/Watcher、不打开 UI、不注入、不使用 `unsafe`。
11. 不新增依赖，复用 Workspace 已固定的 `serde_json = "=1.0.149"`。
12. 产品实现、提交、推送、PR、Review/CI 与最终 Squash Merge 均保留后续独立批准门。

## 上游根因

- 上游 `read_latest_logs` 返回诊断日志实际路径和原始文本，扩大隐私与凭据暴露面。
- 上游尾部读取使用 `2 MiB`，并通过 lossy UTF-8 把损坏字节替换为文本，无法区分合法记录与损坏记录。
- `clear_logs`、`copy_diagnostics`、`write_diagnostic_event` 与只读读取耦合在同一诊断总功能中。
- `copy_diagnostics` 会聚合设置、安装路径、日志路径与状态，不符合本切片的最小披露边界。

新架构只保留可诊断的结构观察，写入、清理、复制报告和完整诊断总功能继续等待独立评估。

## 功能身份

### 新子能力

- ID：`feature.foundation-platform.diagnostic-log-observation`
- 实现后状态：`implemented`
- 来源：`tauri-command:read_latest_logs`
- 副作用：`filesystem-read`
- 持久化：`none`

### 原总功能

- ID：`feature.foundation-platform.diagnostics`
- 状态：继续 `unassessed`
- 剩余来源：`core-module:diagnostic_log`、`tauri-command:clear_logs`、
  `tauri-command:copy_diagnostics`、`tauri-command:write_diagnostic_event`
- 清理、写入、报告复制、剪贴板和路径披露语义均不进入本切片。

## 分层设计

### Domain

新增 `DiagnosticLogObservation`，只保存：

- `file_size_bytes: u64`
- `sampled_record_count: usize`
- `valid_object_record_count: usize`
- `malformed_record_count: usize`
- `truncated: bool`
- `partial_record_discarded: bool`

所有字段私有，只提供构造函数与只读 getter。构造必须维护不变量：

```text
sampled_record_count = valid_object_record_count + malformed_record_count
```

`NoDiagnosticLog` 只作为 Parity 与文档中的稳定空状态语义；Application 仍使用无载荷的
`LoadCompletion::Empty`，不为单一用例扩展全局完成态。领域类型不保存路径、字节、文本、
JSON 值或底层 I/O 错误。

### Application

新增零字段 `DiagnosticLogObservationRequest`、`DiagnosticLogObservationPort` 与
`ObserveDiagnosticLog<P>`。Port 返回
`Result<Option<DiagnosticLogObservation>, ApplicationError>`，映射固定为：

- `Some` → `LoadCompletion::Ready`；
- `None` → `LoadCompletion::Empty`；
- `Err` → `LoadCompletion::Failed`。

请求身份、取消、超时和过期结果隔离继续复用 `LoadCoordinator`，不新增线程或异步运行时。

### Platform

新增 `SystemDiagnosticLogObservation`，公开面只实现 Application Port；任意路径探针保持模块私有。

读取算法：

1. 非 Windows/macOS 返回 `DIAGNOSTIC_LOG_OBSERVATION_UNSUPPORTED`。
2. 通过 `SystemPlatformPaths` 解析固定诊断日志文件；路径不进入返回值或错误信息。
3. `symlink_metadata` 的 `NotFound` 映射为 `Ok(None)`；其他失败映射为稳定 unavailable。
4. 拒绝符号链接、目录、设备和其他非普通文件。
5. 记录元数据字节数；当文件大于 `256 KiB` 时 Seek 到尾部窗口起点并标记 `truncated`。
6. 截断窗口从文件中部开始时，丢弃首个换行前片段并标记 `partial_record_discarded`。
7. 按换行切分剩余记录；EOF 前的最后一条非空记录即使没有结尾换行也参与分类。
8. 每条记录独立执行严格 UTF-8 与 JSON 解析；只接受 object 根，不保存解析结果。
9. 空行、非法 UTF-8、损坏 JSON 与非 object 根各计一条 malformed。
10. 所有 Seek/Read/metadata 错误只公开稳定 kind/code。

## 稳定错误

| 场景 | `ErrorKind` | `DiagnosticCode` |
| --- | --- | --- |
| 非支持平台 | `Unsupported` | `DIAGNOSTIC_LOG_OBSERVATION_UNSUPPORTED` |
| 元数据、打开、Seek 或读取失败 | `Unavailable` | `DIAGNOSTIC_LOG_OBSERVATION_UNAVAILABLE` |
| 符号链接或非普通文件 | `InvalidInput` | `DIAGNOSTIC_LOG_OBSERVATION_INVALID_FILE_TYPE` |

损坏记录属于成功观察中的计数事实，不把单条记录损坏提升为整个文件失败。

## 性能与隐私

- 单次文件内容读取上限固定 `256 KiB`；大文件只 Seek 到尾部窗口。
- 结构计数使用单次线性扫描，内存上限与窗口上限同阶。
- 禁止日志正文进入 Domain、Application、Debug、错误、报告或测试快照。
- 任何错误信息不得拼接真实路径、用户名、机器名、凭据或底层解析文本。
- Windows/macOS 使用同一领域结构、窗口、分类和错误码；平台差异只存在于路径解析。

## Parity 设计

- 新增一个 feature 与一个无 fixture 行为合同，预期 feature/contract 为 `40/40`。
- `tauri-command:read_latest_logs` 从原诊断总功能移动到新子能力，副作用收敛为
  `[filesystem-read]`。
- 原诊断总功能保留其余四个入口并继续 `unassessed`。
- source index 总数保持 `133`，fixture manifest 保持 `11`。
- 仓库测试必须证明没有把写日志、清理日志、复制诊断或完整 core 模块误标为 implemented。

## TDD 验收

### Domain RED → GREEN

- 类型和六个只读事实最初不存在。
- 覆盖零值、合法混合计数、不变量、截断与首片段标志。
- `Debug` 与公开 API 不包含文本、路径或 JSON 内容。

### Application RED → GREEN

- Request/Port/UseCase 最初不存在。
- 覆盖 `Some → Ready`、`None → Empty`、错误 `→ Failed`。
- 验证旧请求隔离、取消和超时语义继续由现有协调器承担。

### Platform RED → GREEN

- 覆盖不存在、空文件、合法 JSON Lines、混合损坏、非法 UTF-8、空行、非 object JSON。
- 覆盖超限尾部窗口、首片段丢弃、无结尾换行、符号链接/非普通文件和 I/O 失败。
- 集成测试只能经公开 Port 观察固定平台路径；任意路径测试入口保持模块私有。

### Parity RED → GREEN

- 先证明新子能力、合同和来源映射不存在。
- 再固定 `40/40`、source `133`、fixture `11` 和原诊断总功能 `unassessed`。

## 当前规划写入范围

```text
docs/plans/2026-07-28-issue-95-gate-5-diagnostic-log-observation.md
docs/plans/sessions/2026-07-28-issue-95-gate-5-diagnostic-log-observation.md
docs/reports/issue-95-gate-5-diagnostic-log-observation.md
docs/workflows/2026-07-28-issue-95-gate-5-diagnostic-log-observation-runtime.md
```

- `count`: `4`
- `hash`: `sha256:14d78bf1a92f5b8db58650b501fb0cebee59a329823ff49e7b8ff3e93e0b7231`

## 候选完整实施范围

候选范围固定为 `24` 路径，完整清单由 Runtime Workflow 作为执行 allowlist 保存；哈希为：

`sha256:8d407c269436c655e12ff94035183de6aa50dc7759fbc75f9cb7b6f9b0349d38`

范围不包含 Cargo、UI、Workflow、Ruleset、Release、`upstream/` 或 AGOS 文件。

## 实施任务

1. 冻结四份规划控制面并完成范围、哈希、占位符和仓库政策验证。
2. 获得 `24` 路径与候选哈希的独立所有者批准。
3. Domain TDD：实现六字段结构、不变量和隐私边界。
4. Application TDD：实现零字段请求、Port、UseCase 和完成态映射。
5. Platform TDD：实现固定路径、有界尾读、记录分类和稳定错误。
6. Parity TDD：新增子能力/合同并精确移动单一来源入口。
7. 更新稳定项目控制面，执行本地轻量验证与 Git checkpoint。
8. 普通推送、非 Draft PR、Review 根因闭环和 Hosted CI。
9. 绑定 Final Head 后请求独立 Squash Merge 授权。

## 当前授权

项目所有者已批准 `24` 路径与候选哈希，当前允许按本计划执行 TDD、稳定项目控制面、本地
轻量验证、Git checkpoint、普通推送、非 Draft PR 与 Review/CI。最终 Squash Merge 仍未授权。

规划阶段的 CI 脚本合同、仓库政策、四路径 allowlist、两个哈希、占位符和 `git diff --check`
均已通过。AGOS ReportOnly 返回既有 `needs-input`/未登记边界，已按 `err.md` 记录绕过，不构成
本项目门禁。

## 停止条件

- 实际路径超出当前四路径或后续批准的二十四路径。
- 需要读取任意路径、提高窗口、返回正文或新增依赖。
- 需要网络、写入、子进程、线程、Watcher、UI、注入或 `unsafe`。
- 发现损坏记录、截断或空文件语义需要改变。
- 基线漂移、哈希不一致、Issue 状态异常或仓库政策验证失败。

任一条件触发时必须停止并回到 Issue `#95` 重新决策，禁止静默扩大范围。
