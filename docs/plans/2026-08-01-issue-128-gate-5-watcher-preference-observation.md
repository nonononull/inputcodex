# Issue #128 Gate 5 Watcher 偏好状态只读观察实施计划

> **状态：** `planning-frozen-implementation-active`
>
> **执行纪律：** 当前会话按 `superpowers:test-driven-development` 执行 RED -> GREEN -> VERIFY，
> 不向其他 agent 委派写入；Final Head 必须另行独立只读复审。

**目标：** 只读观察固定 `inputcodex_state_root/watcher.disabled` 的元数据，把缺失与普通文件映射为稳定偏好状态，不泄露路径，也不接管完整 Watcher 管理。

**架构：** Domain 只保存 `EnabledByDefault / ExplicitlyDisabled`；Application 提供零字段请求、Port 和 UseCase；Platform 复用 `SystemPlatformPaths` 并只调用一次 `symlink_metadata`。Parity 只移动 `tauri-command:load_watcher_state`。

**技术栈：** Rust 1.97.1、现有七成员 Workspace、标准库文件元数据、现有 `LoadCoordinator`、YAML Parity 目录与 PowerShell 治理脚本；不新增依赖。

## 任务元数据

- `tracking_issue_ref`: https://github.com/nonononull/inputcodex/issues/128
- `approved_decision_ref`: https://github.com/nonononull/inputcodex/issues/127#issuecomment-5151021881
- `implementation_scope_approval_ref`: https://github.com/nonononull/inputcodex/issues/128#issuecomment-5151032720
- `standing_authorization_ref`: https://github.com/nonononull/inputcodex/issues/111
- `branch_ref`: `codex/issue-128-gate-5-watcher-preference-observation`
- `baseline_ref`: `43ace17de1505f251812e4ead3035ef3274a8455`
- `planning_scope_count`: `7`
- `planning_scope_hash`: `sha256:359bfb23f99ed52174d8489271f19d597435bd271d426e1dd04ec4237064d018`
- `candidate_scope_count`: `24`
- `candidate_scope_hash`: `sha256:0be3dd45ed7e91d1cb7f633da369bb2b9052cefc512852d80362775da7e81699`

## 已批准语义

- 固定标记缺失：`LoadCompletion::Ready(EnabledByDefault)`，不是 `Empty`。
- 固定标记是普通文件：`LoadCompletion::Ready(ExplicitlyDisabled)`。
- 符号链接、目录或其他类型：`LoadCompletion::Failed(WATCHER_PREFERENCE_INVALID)`。
- 元数据 I/O 失败：`LoadCompletion::Failed(WATCHER_PREFERENCE_UNREADABLE)`。
- 非 Windows/macOS：`WATCHER_PREFERENCE_UNSUPPORTED`。
- 返回值不包含路径、文件内容、用户名、进程、debug port、安装或运行状态。
- 不读取内容、不写入、不联网、不调用子进程、不启动线程或真实 Watcher。

## Task 1：Domain RED -> GREEN

**文件：**

- 新增：`crates/inputcodex-domain/tests/watcher_preference_observation.rs`
- 新增：`crates/inputcodex-domain/src/watcher_preference_observation.rs`
- 修改：`crates/inputcodex-domain/src/lib.rs`

- [x] RED：测试引用尚不存在的 `WatcherPreference` 与 `WatcherPreferenceObservation`，分别断言两个状态和脱敏 Debug。
- [x] 运行 `cargo test -p inputcodex-domain --test watcher_preference_observation --offline`，确认只因目标 API 缺失失败。
- [x] GREEN：实现两个 `Copy + Eq` 领域类型、构造器和只读 getter，不引入路径或字符串字段。
- [x] 运行 Domain 专项、全目标 Clippy 和 rustfmt。
- [x] 建立 `issue-128-domain-green` checkpoint。

目标 API：

```rust
pub enum WatcherPreference {
    EnabledByDefault,
    ExplicitlyDisabled,
}

pub struct WatcherPreferenceObservation {
    preference: WatcherPreference,
}
```

## Task 2：Application RED -> GREEN

**文件：**

- 新增：`crates/inputcodex-application/tests/watcher_preference_observation.rs`
- 新增：`crates/inputcodex-application/src/watcher_preference_observation.rs`
- 修改：`crates/inputcodex-application/src/lib.rs`

- [ ] RED：测试 Request、Port、UseCase 的 Ready、Failed 和取消后迟到结果 `Stale`。
- [ ] 运行 Application 专项，确认只因目标 API 缺失失败。
- [ ] GREEN：实现零字段 request、同步只读 port 和 `ObserveWatcherPreference<P>`。
- [ ] 端口返回领域值而不是 `Option`，避免把缺失标记错误映射为 `Empty`。
- [ ] 运行 Application 专项、全目标 Clippy 和 rustfmt。
- [ ] 建立 `issue-128-application-green` checkpoint。

目标 Port：

```rust
pub trait WatcherPreferenceObservationPort {
    fn observe(
        &self,
        request: &WatcherPreferenceObservationRequest,
    ) -> Result<WatcherPreferenceObservation, ApplicationError>;
}
```

## Task 3：Platform RED -> GREEN

**文件：**

- 新增：`crates/inputcodex-platform/tests/watcher_preference_observation.rs`
- 新增：`crates/inputcodex-platform/src/watcher_preference_observation.rs`
- 修改：`crates/inputcodex-platform/src/lib.rs`

- [ ] RED：内存 probe 覆盖 NotFound、普通文件、symlink、other、PermissionDenied；记录被访问文件名与 metadata 调用次数。
- [ ] RED：断言固定文件名只能是 `watcher.disabled`，任何路径与错误 Debug 均不得包含私有片段。
- [ ] 运行 Platform 专项，确认只因目标 API 缺失失败。
- [ ] GREEN：System adapter 先解析平台路径，再拼接固定文件名并执行一次 `symlink_metadata`。
- [ ] 非发布平台返回稳定 Unsupported；没有文件内容读取 API。
- [ ] 运行 Platform 专项、全目标 Clippy 和 rustfmt。
- [ ] 建立 `issue-128-platform-green` checkpoint。

## Task 4：Parity RED -> GREEN

**文件：**

- 修改：`crates/inputcodex-parity/tests/catalog_repository.rs`
- 修改：`parity/features/foundation-platform.yml`
- 修改：`parity/contracts/foundation-platform.yml`
- 修改：`parity/features/source-index.yml`
- 修改：`parity/README.md`

- [ ] RED：新增测试要求新 feature/contract、单一入口、单一 `filesystem-read`，并禁止原 Watcher feature/contract 残留 `load_watcher_state`。
- [ ] 运行 Parity 目录专项，确认新目录条目缺失导致 RED。
- [ ] GREEN：新增 implemented 子能力和无 fixture 合同，只重映射一个 command。
- [ ] 原 Watcher 总功能保持 `unassessed`，其 core 与四个写入/进程入口不变。
- [ ] 运行 Parity 专项、全目标 Clippy、Release Audit 和 rustfmt。
- [ ] 建立 `issue-128-parity-green` checkpoint。

## Task 5：文档与本地收口

**文件：**

- 修改：`README.md`、`AGENTS.md`、`CONTEXT.md`、`build.md`、`docs/plans/PROJECT-MASTER-PLAN.md`
- 新增：`docs/reports/issue-128-gate-5-watcher-preference-observation.md`
- 修改：`err.md`，记录已确认的 `Path::exists` fail-open 与目录副作用误分类根因

- [ ] 更新稳定产品能力、术语、Gate、构建命令、Parity 计数和报告。
- [ ] 运行四 crate tests/Clippy、rustfmt、CI 合同、仓库政策、Release Audit 和 Cargo metadata。
- [ ] 复算实际 24 路径 Ordinal hash，执行隐私/禁止能力扫描与 `git diff --check`。
- [ ] 建立 `issue-128-local-verified` checkpoint。

## Task 6：远端交付

- [ ] 普通 push，创建 `Closes #128` 的非 Draft PR。
- [ ] 回写 schema v1 PR evidence，绑定 policy hash、24 路径 hash、Final Head 与 standing authorization。
- [ ] 由独立只读 reviewer 检查 Final Head；Critical/Important 必须清零。
- [ ] 核验 Review thread 为 0、CI 7/7、Performance 4/4、Artifact 为 0、mergeable=CLEAN。
- [ ] 精确 Head Squash Merge；验证单父、tree 等价、GitHub 签名与 main 两套 Workflow。
- [ ] 关闭 Issue 并归档工作区/reviewer。

## 精确路径范围

候选范围固定为 Issue #128 正文列出的二十四路径，Ordinal hash 为
`sha256:0be3dd45ed7e91d1cb7f633da369bb2b9052cefc512852d80362775da7e81699`。

## 停止门

- 需要新增依赖、读取内容或第二个文件、接受任意路径、写入、网络、子进程、线程、UI 或进程控制。
- `release_audit` 不为 `current`，上游正式 Release 变化，或实际路径/hash 漂移。
- Review finding 未关闭、Hosted 门禁未绑定同一 Final Head，或出现 secret/Release/Ruleset/收费资源硬停止。
