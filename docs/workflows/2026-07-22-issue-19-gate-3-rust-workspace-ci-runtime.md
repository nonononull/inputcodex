# Issue #19 Runtime Workflow：Gate 3 纯 Rust Workspace 与首版三平台 CI

workflow_status: failure-semantics-and-cold-baseline-in-progress
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/19
session_plan_ref: docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md
implementation_plan_ref: docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
architecture_plan_ref: docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
approved_decision_ref: user-message:approve-gate-3-implementation-2026-07-22
scope_hash: sha256:2e101627480012d57d6d0472a08cfbe03fc401f6ac74ef3ae1e6a42929ed61ba

## 当前执行 checkpoint

- Phase 0 Fresh 基线已完成，未发现分支、Issue、Ruleset、Release/tag、Rust/Iced 元数据或 Issue `#16` 漂移。
- Phase 2 RED 合同已通过 AST 验证并以退出码 `10`、唯一标记 `CI_CONTRACT_RED_MISSING_IMPLEMENTATION` 证明两个治理实现入口缺失。
- RED checkpoint 已以提交 `67fe99457e1aa2717cc29c70d51114028d68dafd` 推送并回写 Issue `#19`。
- Phase 3 首个 GREEN 提交 `be9259f55b32014e918113936e6e6ddfdd16765f` 已推送并回写 Issue。
- Phase 4 复核发现依赖白名单比批准箭头更宽；四条新增 RED 已证明根因，政策已收紧并达到 `27/27` GREEN。
- 依赖方向纠偏提交 `2d8a1466ae42d6b208258bee3d0cb6bd5647bb12` 已推送并回写 Issue。
- Phase 4 七成员 Workspace 已完成 RED→GREEN；Phase 5 子项目文档、metadata、fmt、domain check 和六个轻量 crate 测试已完成，Workspace checkpoint `f93372fdc63cf8c628007117be4a8b222510957b` 已推送并回写 Issue。
- Phase 6 已完成变更收集器、许可证一致性门禁和首版六 Job Workflow；当前合同为 `30/30`。CI checkpoint `f3107dd16705dd3a25bc8c3acc540a3c6c6990a3` 已普通 push 并回写 Issue 评论 `5044470597`，Draft PR 为 `#21`。
- Draft PR `#21` 的运行 `29910132968` 与 `29910379208` 证明 job 级 `env` 不支持 `runner.temp`；运行 `29910847062` 进一步证明 Linux 条件导入根因，`required` 对真实失败正确阻断。两类问题均用后续普通提交修复，未 rerun 旧失败。
- 提交 `bd4610f6e98dc597bddf02c584d0f0fc616cac7b` 触发运行 `29911337652`，classify、governance、linux-quality、windows、macos、required 全绿，成功 Artifact 数为 `0`；精确 Rust `1.97.1` 与 Iced/desktop 三平台证据已成立。
- 首个成功样本的 metrics 只写 Step Summary，当前 API 无法在完成后复取二进制字节数；已用合同 RED→GREEN 要求后续 metrics 同时写控制台日志与 Step Summary，当前转入 Phase 7。

## Phase 0：startup-baseline

1. 确认分支为 `codex/issue-19-gate-3-rust-workspace-ci`，基线为 `main@477d110a9b284e127af365f5278901bcfa69e093`，工作树只含本任务变更。
2. 确认 Issue `#19` 为 OPEN，标签为 `type:architecture` 与 `gate:3`，Issue 正文包含 23 条允许路径和当前所有者批准引用。
3. 确认 PR `#18` 已 MERGED、Issue `#17` 已按 COMPLETED 关闭、远端旧分支已删除。
4. Fresh 核对上游 Release `v1.2.41`、tag SHA、Issue `#16`、Ruleset `19395456`、Rust `1.97.1` 与 Iced `0.14.0` 元数据。
5. 确认仓库无 `.codegraph/`，不初始化索引；本地知识查询无结果时不补写推测。
6. AGOS 只运行 report-only；返回 `needs-input/unregistered` 后记录并绕过，不修改外部控制面。

## Phase 1：control-plane-checkpoint

1. 只创建 Session Plan、Runtime Workflow、初始报告并更新必要项目入口；不得在本 checkpoint 创建 Cargo、Rust、Iced 或产品 Workflow。
2. 校验新增文档包含真实 Issue、分支、基线、批准引用、scope hash、RED/GREEN、CI、停止与回滚合同。
3. 运行 `git diff --check`、允许路径和占位符扫描；形成普通提交并普通 push，作为实现前命名 Git checkpoint。
4. 在 Issue `#19` 回写 checkpoint commit、分支、Session Plan、Runtime Workflow 和下一合法批次。

## Phase 2：governance-red

1. 创建 `scripts/ci/Test-CiScripts.ps1`，先调用尚不存在的治理入口并要求非零退出。
2. RED 夹具覆盖：Iced 越层、`upstream/` 入 Workspace、生产 `.ts/.js`、Tauri/WebView、广告/遥测依赖、非本仓更新源、路径穿越、空 diff、重命名与删除。
3. 运行测试并保存精确失败命令、退出码和根因；RED 必须因为目标能力缺失而失败，不能因拼写、路径或测试夹具错误失败。
4. 形成 RED checkpoint；未取得可信 RED 前不得创建 Workspace。

## Phase 3：governance-green

1. 实现 `scripts/ci/Classify-Changes.ps1`，只解析输入路径并输出确定分类，不执行构建或网络调用。
2. 实现 `scripts/ci/Verify-RepositoryPolicy.ps1`，解析 Workspace 和受控源码目录，执行依赖方向、禁止文件、禁止依赖和更新源合同。
3. 使 RED 夹具全部 GREEN；为发现的每个测试设计错误先记录根因，再修复夹具，不降低生产检查。
4. 运行脚本自测、`git diff --check` 和路径门禁，形成 GREEN checkpoint。

## Phase 4：minimal-workspace-green

1. 创建根 `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml` 与 `.gitignore`；成员必须显式列出七个路径，禁止通配。
2. 按 `domain → application → infrastructure/platform/parity → presentation → desktop` 顺序创建最小接口和测试。
3. application 固化请求标识、`Idle / Loading / Ready / Empty / Failed / Cancelling` 与稳定错误语义；过期结果和取消后结果必须失效。
4. Iced 只存在于 presentation；desktop 只组装依赖并调用展示层启动入口，UI 视觉与交互由 Gemini 实现或审阅。
5. Windows/macOS 共享 application 合同；Linux 非发布实现明确返回 unsupported，不伪造成功。
6. 每个 crate 只实现当前测试需要的最小代码，不迁移任何上游功能。

## Phase 5：subproject-docs-and-local-lightweight

1. 为 app 与六个 crate 分别创建 `build.md`、`err.md`，写明包名、定向 check/test、平台依赖和已知限制。
2. 更新根 `build.md`：本地仅运行 fmt、domain check/test 和 CI 脚本；全 Workspace、Windows/macOS、冷构建交给 Actions。
3. 依次运行：
   - `pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1`
   - `cargo fmt --all -- --check`
   - `cargo check -p inputcodex-domain`
   - `cargo test -p inputcodex-domain`
4. 任何失败先查对应 `err.md`；重复问题引用既有结论，不盲目重跑。

## Phase 6：first-ci

1. 创建 `.github/workflows/ci.yml`，Workflow 名称固定为 `CI`，顶层权限固定 `contents: read`。
2. 事件只允许 `pull_request`（目标 `main`）、`push`（`main`）和 `workflow_dispatch`；避免功能分支 push 与 PR 双重全量运行。
3. 创建 `classify`、`governance`、`linux-quality`、`windows`、`macos`、`required` 六个 Job，按批准超时运行。
4. `required` 使用 `if: always()` 汇总所有前置结果；非法 skipped、cancelled 或 failure 必须退出非零。
5. 首版禁止 Cache；成功默认不上传 Artifact，失败只上传白名单报告且 `retention-days: 7`，禁止 `target/` 和环境转储。
6. CI 稳定前不修改 Ruleset required checks。

## Phase 7：failure-semantics-and-cold-baseline

1. 使用普通提交分别制造并修复治理、格式、Rust、Windows 条件编译和 macOS 条件编译失败；禁止 Force Push 和只 rerun。
2. 每次失败先确定根因，在 `err.md` 或任务报告记录失败运行、修复提交和验证运行。
3. Linux、Windows、macOS 各记录至少三次无缓存成功样本，包含排队、执行、依赖下载、编译和总耗时。
4. 记录依赖数量、最小桌面二进制体积和测量方法；Gate 3 不设最终运行性能预算。
5. Cache 调优必须另建 Issue；当前任务只保留无缓存基线。

## Phase 8：pr-review-closeout

1. PR 正文包含 `Closes #19`、范围哈希、RED/GREEN、三平台运行、失败恢复、冷构建和回滚证据。
2. Fresh 核对 PR 文件全部位于允许路径、Head 未漂移、自动合并关闭、Ruleset 无漂移、Review 对话全部闭环。
3. 所有适用 Job 必须成功；`0 Checks`、非法 skipped 或仅 rerun 不得解释为通过。
4. 获得项目所有者新的明确授权后，只执行 Squash Merge；禁止 Merge Commit、Rebase Merge、Force Push 和删除 `main`。
5. 合并后验证 Issue `#19` 按 COMPLETED 关闭、merge commit 单父、tree 与最终 Head 相同、签名 `valid`、远端分支删除，并回写最终报告。

## 停止条件

- 需要扩展允许路径、实现业务功能、修改 `upstream/`、Ruleset、Release、签名、安装包或 AGOS。
- Release/tag、Rust/Iced 元数据、许可证、Issue `#16` 或 Ruleset 发生未解释的物质漂移。
- RED 证据根因错误、GREEN 通过依赖跳过、Iced 越层、平台语义分叉或 UI 边界未经 Gemini 处理。
- CI Job 被关闭、失败仅靠 rerun/Force Push 掩盖、Artifact 越界、Cache 未经独立批准或本地机器被迫承担全量构建。
- Review 对话未根因闭环、适用 CI 未成功、冷构建证据不足或缺少新的 Squash Merge 授权。
