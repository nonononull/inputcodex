# Issue #143 Watcher 偏好固定文件 mutation 实施计划

> **状态：** `LOCAL_VERIFY_GREEN / PR_PENDING`
>
> **执行纪律：** 单 writer，按 Domain -> Application -> Platform -> Parity 的 RED -> GREEN -> VERIFY 顺序执行；Final Head 必须由独立只读 reviewer 复审。

## 目标与边界

只实现 `feature.foundation-platform.watcher-preference-mutation`，接管
`tauri-command:enable_watcher` 与 `tauri-command:disable_watcher`。固定对象只能是
`SystemPlatformPaths` 定位的 `inputcodex_state_root/watcher.disabled`；调用方不提供路径、文件名或内容。

本能力只改变 Watcher 偏好标记，不安装、卸载、启动、停止或探测 Watcher，不控制进程。

## 授权与基线

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/143
- owner_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214
- retry_resume_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5162564532
- batch_1_closeout_ref: https://github.com/nonononull/inputcodex/issues/141#issuecomment-5163639700
- standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
- baseline_ref: `main@891866a914468d3979964a2fe89066a0bbb2fe53`
- branch_ref: `codex/issue-143-gate-5-watcher-preference-mutation`
- threat_model: `cooperative-same-user-v1`
- planning_scope: `8 / sha256:7011b4faa5d331e668b2eb837dbbc42b716541f9f88431c9a773ae830c0dbe11`
- candidate_scope: `24 / sha256:f96f1a979eba89bc4de9744b3267bfd72fd81a828c9491cbfd3b95723b088ab9`

## 领域方案

- Request 同时携带 typed request identity、`expected` 与 `desired` 偏好；没有任意路径或内容。
- Receipt 固定包含 request identity、requested preference、setup commit、marker commit、final observation、outcome 与稳定 diagnostic code。
- Outcome 区分 `Applied / AlreadySatisfied / Conflict / Cancelled / Failed / Indeterminate`。
- 取消控制使用独立 mutation phase；提交前取消零副作用，任一 setup/marker 提交边界后取消返回 `TooLate`，最终 receipt 仍必须交付。
- `LoadCoordinator` 只用于读取，取消后会把完成结果降为 `Stale`，本能力禁止复用。

## 平台方案

- 即时 OS 应用数据父目录必须预先存在、是普通目录且非 symlink/junction/reparse。
- 状态根缺失时仅 Disable 可执行一次单层 `create_dir`；成功记录 `RootCreated`，后续失败不回滚空根。
- 状态根 `AlreadyExists` 后重新观察；Windows 用 safe `MetadataExt::file_attributes` 拒绝 reparse，macOS 拒绝 symlink。
- Disable 只用 `create_new` 创建空的固定 marker，不覆盖或截断。
- Enable 只在提交前再次观察为固定普通 marker 后调用 `remove_file`。
- 同进程使用单一互斥锁串行；提交后必须重新观察。合法相反状态为 `Conflict`，无法可信观察为 `Indeterminate`。
- 不防御同账号恶意进程在最终验证后的 ABA；该边界不得被描述为句柄绑定安全证明。

## TDD 批次

1. Domain：先写 receipt、outcome、commit 与脱敏 Debug RED，再实现纯领域类型。
2. Application：先写 expected/desired、取消 phase、TooLate 和 receipt 不丢失 RED，再实现 Port/UseCase。
3. Platform：先写父目录、根 setup、marker 类型、竞态、失败后复观测和固定路径矩阵 RED，再实现 safe std adapter。
4. Parity：先要求新 feature/contract 与两个 source 精确迁移 RED，再更新目录；完整 Watcher 保持 `unassessed`。
5. 文档与本地门禁：更新稳定能力、err、计数和报告，执行 `build.md` Issue #143 命令。
6. 远端交付：普通 push、non-Draft PR、独立 Final Head 复审、Hosted CI、精确 Squash 与 main 复验。

## 完成终态

- source：`135 = 19 implemented / 83 unassessed / 30 exception-pending / 3 excluded`
- feature：`46 = 13 implemented / 22 unassessed / 11 exception-pending`
- contract：`46`；fixture manifest：`12`
- `feature.foundation-platform.watcher` 继续 `unassessed`
- `gate_5_product_complete=false`；`gate_6_unlocked=false`

## 执行进度

- Planning Freeze、Domain、Application、Platform 与 Parity 的 RED/GREEN checkpoint 已完成。
- Domain 专项 `4/4`、Application 专项 `5/5`、Platform 内部矩阵 `13/13`、Platform 全包与集成测试、
  Parity 完整目录测试 `32/32` 及各受影响 crate Clippy 已通过。
- `build.md` Issue #143 二十四路径全门禁已返回 `ISSUE143_LOCAL_VERIFY_OK`；当前只允许形成 Final Head、
  普通 push 与 non-Draft PR，独立复审和 Hosted 门通过前不得合并。

## 硬停止

需要任意路径/内容、递归目录、secret、网络、进程、环境/剪贴板、UI、SQLite、VFS、FFI、`unsafe`、
新依赖、Workflow/Runner/Ruleset、Release/签名或双平台不同领域终态时，停止并按 #140 协议收口；禁止扫描其他候选。
