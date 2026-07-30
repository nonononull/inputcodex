# Issue #105 quick-xml RustSec 高危公告修复实施计划

## 控制元数据

```yaml
task_id: issue-105-quick-xml-rustsec-remediation
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/105
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/105#issuecomment-5129028445
branch: codex/issue-105-quick-xml-rustsec-remediation
baseline_main: 060ca045d2c134f8be3c9adc8cdb038842fc3243
work_class: standard
selected_business_path: maintenance/dependency-security/quick-xml-rustsec-remediation
planning_scope_count: 7
planning_scope_hash: sha256:25337a6cb90386439878af2bd8be7d00af0276e102663ab6add7e3d3b4621a09
candidate_scope_count: 9
candidate_scope_hash: sha256:0c90d018e06aa640d33a4c65c75aea45c89eb0e365b91fadee803b0426c8c58f
delivery_contract: agos.issue-pr-merge.v1
allowed_operations: plan,lockfile-update,documentation-edit,local-light-verification,git-checkpoint,commit,push,non-draft-pr,review-ci
mutation_intent: config
executor_enforcement: exact-path-and-exact-package-diff-hard-stop
final_merge_authorization: pending-separate-final-head-gate
```

## 问题与风险事实

- `origin/main@060ca045d2c134f8be3c9adc8cdb038842fc3243` 的 `Cargo.lock` 锁定
  `quick-xml 0.39.4` 与 `wayland-scanner 0.31.10`。
- fresh `cargo audit` 扫描 `350` 个锁定依赖后以退出码 `1` 报告两条漏洞：
  `RUSTSEC-2026-0194` 与 `RUSTSEC-2026-0195`，CVSS 均为 `7.5 high`，修复要求均为
  `quick-xml >=0.41.0`。
- 依赖路径为 `quick-xml -> wayland-scanner -> Wayland/winit/Iced ->
  inputcodex-presentation -> inputcodex-desktop`。`wayland-scanner` 用于从受依赖版本控制的
  Wayland XML 生成 API，不是 inputcodex 面向用户暴露的 XML 解析入口；这降低了当前直接输入面，
  但不能替代可用的最小补丁升级。
- `wayland-scanner 0.31.10` 精确依赖 `quick-xml ^0.39`，不能只提高叶子锁定版本；
  `wayland-scanner 0.31.11` 将约束提高到 `quick-xml ^0.41`。
- `wayland-scanner 0.31.11` 为 MIT、MSRV `1.71`；`quick-xml 0.41.0` 为 MIT、
  MSRV `1.79`。两者均低于仓库固定工具链 Rust `1.97.1`。
- 同次扫描的 `paste 1.0.15` 与 `ttf-parser 0.25.1` 是两条允许的
  `unmaintained` 警告，不是本任务的高危漏洞，也不允许借此升级 Iced/wgpu 字体或图形依赖链。

## 已批准方案

只执行一个精确锁文件更新：

```powershell
cargo update -p wayland-scanner --precise 0.31.11
```

预期结果只有：

1. `wayland-scanner 0.31.10 -> 0.31.11`；
2. `quick-xml 0.39.4 -> 0.41.0`；
3. 两个 package block 的 crates.io checksum 随版本更新；
4. `Cargo.toml`、Iced、winit、smithay-client-toolkit、产品行为与 Workflow 全部不变。

不选择以下方案：

- 直接锁定 `quick-xml 0.41.0`：被 `wayland-scanner 0.31.10` 的 `^0.39` 约束拒绝；
- 升级 Iced/winit：范围与跨平台回归面远大于已知最小补丁路径；
- 仅记录不可达性并保留漏洞：已有兼容补丁版本，风险收益不成立；
- 同时处理两条 unmaintained warning：需要评估不同上游链，不得与已确认漏洞捆绑。

## Change Contract

### Target contract

`Cargo.lock` 不再解析到 `quick-xml 0.39.4`，fresh RustSec 扫描不再包含
`RUSTSEC-2026-0194/0195`，并且漏洞总数为 `0`。

### Preserved invariants

- 根 `Cargo.toml` 与所有成员 manifest 不变；
- `iced 0.14.0`、`iced_winit 0.14.0`、`winit 0.30.13` 和现有两代
  `smithay-client-toolkit` 不变；
- 七成员依赖方向、Iced 展示层边界和所有 Gate 5 产品语义不变；
- Windows/macOS/Linux 继续由标准 GitHub-hosted runners 构建；
- CI、Performance Baseline、Ruleset、Runner、Artifact 合同与 `upstream/` 不变；
- `paste` 与 `ttf-parser` 警告只记录，不误报为已修复或本任务漏洞。

### Adjacent surfaces

- Linux Wayland 协议代码生成与桌面编译；
- Windows/macOS 的 Cargo feature/target 解析；
- 锁文件可重复性、crates.io 来源、checksum、许可证和 MSRV；
- Release Audit 与仓库政策验证；
- Issue #104 已合并的本地会话目录能力及其他九个 Gate 5 切片。

### Historical state and stale verdict invalidation

- 基线 RED、旧版本和 Issue #105 安全暂停状态是历史证据，不能在 GREEN 后继续描述为当前锁文件状态；
- 只有 Final Head 的 fresh audit、本地门禁与 GitHub CI 全部通过后，才能解除本任务的实现暂停；
- #105 合并前不得把“本地 GREEN”解释为 `main` 已修复，也不得开始新的 Gate 5 产品 PR。

### Regression checks

- `git diff origin/main -- Cargo.lock` 精确为两个 package block、`4` 行新增与 `4` 行删除；
- `cargo audit --json`：`vulnerabilities.count=0`；
- `cargo metadata --locked --offline --no-deps --format-version 1`；
- `cargo fmt --all -- --check`；
- CI 合同、Repository Policy、Release Audit 与 `git diff --check`；
- GitHub-hosted CI 七 Job、Performance Baseline 四 Job与成功 Artifact 数量。

`sibling_regression_guard` 在 Final Head CI、Review conversation 与 Artifact 核验前保持
`pending`，不得提前写成 `passed`。

## 精确路径范围

```text
AGENTS.md
build.md
Cargo.lock
docs/plans/2026-07-30-issue-105-quick-xml-rustsec-remediation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-30-issue-105-quick-xml-rustsec-remediation.md
docs/reports/issue-105-quick-xml-rustsec-remediation.md
docs/workflows/2026-07-30-issue-105-quick-xml-rustsec-remediation-runtime.md
err.md
```

Ordinal 排序、UTF-8 无 BOM、末尾单个 LF 的路径清单哈希固定为：

`sha256:0c90d018e06aa640d33a4c65c75aea45c89eb0e365b91fadee803b0426c8c58f`。

`err.md` 用于记录本次已真实复现的 advisory-db 直连失败与代理恢复根因，因此最终实际范围
必须精确等于上述九路径；任何新增、删除或重命名都停止执行并重新取得批准。

## TDD 执行批次

### Batch 0：Planning Freeze

- 创建 Plan、Session Plan、Runtime Workflow；
- 更新 AGENTS、build、Master Plan 与 err；
- 验证七路径 planning scope 并建立命名 Git checkpoint；
- 运行 AGOS default-entry 与 Git snapshot `-ReportOnly`，异常按项目规则记录后绕过。

状态：`completed`。Planning checkpoint 为
`c266cab2570ce477bc6079b951cd9e79f5abe4a0`；干净状态 Git snapshot 为
`GIT_SNAPSHOT_READY`。AGOS default-entry 因 task `unregistered`、缺 owner-scope manifest 与其
自身 mutation admission 不兼容而返回 `blocked/needs-input`，已按 inputcodex 规则绕过且未修改
外部仓库。

### Batch 1：Security RED -> GREEN

- 已观察 fresh `cargo audit` 因两条目标公告以退出码 `1` 失败；
- 执行唯一批准的 `cargo update`；
- 检查锁文件只修改两个 package block；
- 再运行 fresh `cargo audit`，要求漏洞总数为 `0`。

状态：`completed`。Security checkpoint 为
`7c890e3137a503cac334e2802bd2441feae41052`；锁文件精确 `4/4`，fresh audit 扫描 `350`
个依赖后退出 `0`、漏洞 `0`，保留两条单独的 unmaintained warning。

### Batch 2：本地轻量验证与报告

- 运行 `build.md` 的 Issue #105 完整命令链；
- 记录许可证、MSRV、warning 分离、范围与安全结果；
- 创建报告并建立 local-verified checkpoint。

状态：`completed`。本机 `2026-07-30 17:49:18 +08:00` 开始的完整九路径门禁输出
`ISSUE_105_LOCAL_GREEN`：audit 漏洞 `0`、warning `2`、CI contract `35/35`、Repository
Policy `0` 违规、Release Audit `current`、实际 scope/hash 与批准值精确一致。Local-verified
checkpoint 为 `2d3f37a0b31fcfe10501e14033665f0c14ae4ffd`。

### Batch 3：远端交付

- 使用本机默认时间提交，普通 push，创建关联 Issue #105 的非 Draft PR；
- 处理 Review，对每条反馈记录根因、处理和 fresh 证据；
- 核验 CI、Performance Baseline、Artifact 与 Final Head；
- 停在项目所有者绑定 Final Head 的独立 Squash Merge 授权门。

状态：`github-dynamic-after-local-verified`。本地控制面冻结于 local-verified checkpoint；普通 push、
PR、Review/CI、Artifact、Final Head 与授权状态只写入 GitHub，不再用递归文档提交追逐远端事件。

## 验收与停止门

验收要求：九路径精确匹配，锁文件仅有两个批准版本/checksum 变化，fresh audit 漏洞为 `0`，
本地轻量门禁、许可证和三平台 Hosted CI 全部通过，Review conversations 与 Artifact 均闭环。

遇到以下任一情况立即停止：

- 需要修改 `Cargo.toml`、Iced/winit/smithay、产品源码、UI、CI、Ruleset、Runner 或 AGOS；
- `cargo update` 解析出第三个 package 变化；
- 许可证不再是 MIT、MSRV 高于 Rust `1.97.1` 或三平台构建失败；
- fresh audit 仍有目标公告或出现新的漏洞；
- 实际范围/hash 漂移、Release Audit stale、工作树污染或基线变化；
- Final Head 未完成 Review/CI/Artifact 闭环却请求 Squash Merge。
