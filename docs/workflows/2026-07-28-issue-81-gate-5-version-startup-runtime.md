# Issue #81 Runtime Workflow：版本与启动意图

## 运行元数据

```yaml
task_id: issue-81-gate-5-version-startup
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/81
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/80#issuecomment-5095090077
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/81#issuecomment-5100612634
branch: codex/issue-81-gate-5-version-startup
baseline_main: ef69494d92c7c461b0cb858e95f6838404ae1a61
candidate_scope_count: 23
candidate_scope_hash: sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe
final_merge_authorization: pending-separate-gate
```

## 节点图

```text
ISSUE_80_DECISION_COMPLETED
  -> ISSUE_81_CREATED
  -> OWNER_SCOPE_APPROVED
  -> ISOLATED_BRANCH_CREATED
  -> PLANNING_CONTROL_PLANE_CREATED
  -> PLANNING_SCOPE_VERIFIED
  -> DOMAIN_RED
  -> DOMAIN_GREEN
  -> APPLICATION_RED
  -> APPLICATION_GREEN
  -> PLATFORM_RED
  -> PLATFORM_GREEN
  -> PARITY_RED
  -> PARITY_GREEN
  -> CONTROL_PLANE_CLOSEOUT
  -> LOCAL_LIGHTWEIGHT_VERIFICATION
  -> FINAL_GIT_CHECKPOINT
  -> COMMIT
  -> NORMAL_PUSH
  -> NON_DRAFT_PR
  -> FINAL_HEAD_REVIEW_CI
  -> OWNER_SQUASH_MERGE_APPROVAL
  -> SQUASH_MERGE
  -> POST_MERGE_VERIFICATION
```

当前授权允许执行到 `FINAL_HEAD_REVIEW_CI`；`SQUASH_MERGE` 仍被独立授权门锁定。

## 输入合同

- 当前进程参数：只匹配精确 `--show-update`；
- 环境变量：只读取 `INPUTCODEX_SHOW_UPDATE`；
- 编译版本：只读取 `CARGO_PKG_VERSION`；
- 禁止读取旧变量、文件、网络、数据库、注册表、历史状态或 UI 状态。

## 输出合同

```text
Ready(VersionStartupSnapshot {
  inputcodex_version: ApplicationVersion,
  startup_intent: Default | ShowUpdate
})

Failed(ApplicationError {
  kind: InvalidInput | Internal | Unsupported,
  code: stable DiagnosticCode
})
```

不得产生 `Empty`。

## 解析状态机

```text
读取 INPUTCODEX_SHOW_UPDATE
  -> 未设置 / 空 / 0: ENV_DEFAULT
  -> 1: ENV_SHOW_UPDATE
  -> 其他值: INVALID_STARTUP_OPTION

ENV_DEFAULT
  -> 参数含 --show-update: ShowUpdate
  -> 否则: Default

ENV_SHOW_UPDATE
  -> ShowUpdate

INVALID_STARTUP_OPTION
  -> Failed(InvalidInput)
```

非法环境值先于命令行结果失败，不允许用 `--show-update` 掩盖配置错误。

## 写入强制

允许路径固定为设计文档中的 `23` 路径。每个 checkpoint 执行：

```powershell
$changed = @(
  git diff --name-only origin/main...HEAD
  git diff --name-only
  git ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique

$outside = @($changed | Where-Object { $_ -notin $candidate })
if ($outside.Count -ne 0) {
  throw "Issue #81 越界路径：$($outside -join ', ')"
}
```

其中 `$candidate` 必须使用设计文档中的完整路径清单，并复算
`sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe`。

## 禁止能力扫描

生产 Rust 路径必须拒绝：

- `CODEX_PLUS_SHOW_UPDATE`；
- `reqwest`、`ureq`、socket 或 HTTP 客户端；
- `std::fs`、文件打开和写入；
- `std::thread`、后台运行时或定时任务；
- shell/PowerShell/cmd 子进程；
- `unsafe`；
- Iced、Tauri、WebView 或 JavaScript。

文档与 Parity 合同可以提及禁止字符串以记录决策；扫描必须限定生产路径，不能把控制面证据误报为运行能力。

## Checkpoint 合同

### Planning

- 只允许三份规划文件；
- planning hash 必须为
  `sha256:707b8a43199ffb69b71f18a9681e9432b02c94b8533dd7dcbc4cf2b1ad758579`；
- 形成规划提交后才进入 Domain RED。

### Domain

- 先看到测试因缺失类型失败；
- GREEN 后只允许 Domain 三路径与既有规划路径变化；
- 版本类型必须复用，不得移动应用概览代码。

### Application

- RED 必须证明缺少 Port/用例或 InvalidInput；
- GREEN 后 Ready/Failed 与过期结果测试通过；
- 不得把非法显式值归类为 Internal 或 Unavailable。

### Platform

- RED 覆盖默认、参数、环境、非法值、非 Unicode 和 unsupported；
- GREEN 后生产实现只读取批准的进程输入；
- Linux Clippy 不得产生 dead code，不得使用 `allow(dead_code)` 掩盖。

### Parity

- RED 证明目录或合同仍为旧语义；
- GREEN 后 feature 为 implemented、合同不含 Empty 和禁止能力；
- `source-index.yml` 必须保持未修改。

### Closeout

- 更新控制面与实施报告；
- 运行完整本地轻量链和扫描；
- 创建最终 Git checkpoint；
- 提交、普通推送与 PR 后只做 Review/CI 根因闭环；
- 未获授权不得 Squash Merge。

## 错误恢复

1. 先读取 `err.md` 查重；
2. 复现失败并固定命令、退出码和受影响路径；
3. 找到根因后只修改批准范围内的最小路径；
4. 新根因写入 `err.md`，重复根因引用既有记录；
5. 修复后重跑最小失败命令，再重跑所属 Batch；
6. 路径变化必须重新审批 scope hash；
7. CI 外部事故不得通过修改产品代码规避。

## 远端交付合同

- 分支普通推送，禁止 force push；
- PR 必须非 Draft，并关联 Issue #80/#81；
- Review 对话必须逐条记录根因、处理与验证；
- 标准 CI 和 Performance Baseline 必须针对 Final Head；
- 成功 Run Artifact 必须为 `0`；
- Final Head 变化后重新核验全部门禁；
- 最终停在项目所有者单独 Squash Merge 授权门。
