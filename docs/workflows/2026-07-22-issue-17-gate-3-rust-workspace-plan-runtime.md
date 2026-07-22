# Issue #17 Runtime Workflow：Gate 3 纯 Rust Workspace 骨架规划

workflow_status: active-planning-only
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/17
session_plan_ref: docs/plans/sessions/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
implementation_plan_ref: docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
approved_decision_ref: user-message:approve-gate-3-planning-2026-07-22
scope_hash: sha256:0c4fc5017aed0b5883b5cb597b2afc2680646479de32916cc4e720aff67dfd05

## Phase 1：startup-baseline

1. 确认分支为 `codex/issue-17-gate-3-rust-workspace-plan`，基线为 `main@113476fb96623452f9a69526edabc73a57d812a1`。
2. 确认 Issue `#17` 为 OPEN，标签为 `type:architecture` 与 `gate:3`。
3. Fresh 核对上游 Release `v1.2.41`、标签提交、上游 `main`、Ruleset `19395456` 和机器状态 Issue `#16`。
4. 确认 Gate 2 PR `#15`、Issue `#14`、两次真实运行、合并提交和远端分支清理均已闭环。
5. 确认仓库无 `.codegraph/`，不初始化索引；确认工作树只包含本任务允许路径。

## Phase 2：design-freeze

1. 使用项目现有架构方案、Rust CI 策略和实现计划作为真源，不另建第二控制面。
2. 比较直接实现、合同先行和 ADR-only 三种路线，采用合同先行拆分。
3. 冻结七个 Workspace 成员、依赖方向、Iced 唯一展示层依赖、双平台端口、加载状态和 CI Job 合同。
4. 锁定 Rust `1.97.1` 与 Iced `0.14.0` 为后续实现候选，要求实施前 Fresh 复核，不在当前分支创建依赖文件。
5. 明确 UI 默认由 Gemini 实现或审阅；当前 Runtime 不创建视觉或交互事实标准。

## Phase 3：project-native-docs

允许写入 Session Plan 中的 `11` 条路径。

必须完成：

- 创建主计划、Session Plan、Runtime Workflow 和初始报告。
- 更新 README、Master Plan、总架构方案、Rust CI 实施计划与 `build.md`。
- 在 `err.md` 只引用已有 WindowsApps ACL 与 AGOS `needs-input` 处理，不重复制造新根因记录。
- 把 Issue `#14` 报告中的动态 `pending` 更新为真实 PR、CI、合并、两次运行、Issue `#16` 和签名证据。

禁止创建 Cargo、Rust、Iced、产品 Workflow、功能、UI、发布资产、Ruleset 或外部 AGOS 改动。

## Phase 4：local-verification

1. 运行 `build.md` 的 Issue `#17` 文档规划验证命令。
2. 对 `main...HEAD` 与未提交文件分别执行允许路径校验，必须为 `11` 条集合的子集且零越界。
3. 检查仓库根、`apps/`、`crates/` 和其他非 `upstream/` 路径不存在产品 `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml` 或 `.rs`。
4. 确认 `.github/workflows/` 只有既有上游监控 Workflow，本任务没有创建 CI 或 Release Workflow。
5. 运行 `git diff --check`；暂存后运行 `git diff --cached --check`。
6. Fresh 核对 GitHub 状态；任何网络失败先查 `err.md` 并确定根因，不以盲目重跑冒充证据。

## Phase 5：commit-push-pr

1. 精确暂存允许路径，推荐提交主题：`docs: 规划 Gate 3 Rust Workspace 骨架`。
2. 普通 push 到 `codex/issue-17-gate-3-rust-workspace-plan`；禁止 Force Push。
3. 创建包含 `Closes #17` 的非 Draft 规划 PR，正文列出批准范围、11 条路径、验证证据和明确的源码后置边界。
4. 在 Issue 与 PR 回写 Head、验证命令、现有 CI 预期和项目所有者规划批准引用。

## Phase 6：review-and-ci

1. PR 只应触发现有 `Upstream Watch / validate`；`watch` 必须 skipped，不得写 Issue。
2. Fresh 核对 PR 文件、Head、Ruleset、Review 对话、自动合并和 mergeable 状态。
3. 每条 Review 反馈先确定根因，再处理和 Fresh 验证；若反馈不成立，提供可复核证据并取得 reviewer 或项目所有者确认。
4. CI、Review 对话、允许路径或 Fresh 基线任何一项失败均禁止合并。
5. 本次规划批准不包含合并授权；必须等待项目所有者新的明确 Squash Merge 授权。

## Phase 7：squash-merge-closeout

1. 获得授权后 Fresh 核对最终 Head、CI、Review 对话、Ruleset、上游基线和允许路径。
2. 只使用 Squash Merge；禁止 Merge Commit、Rebase Merge、Force Push 和自动合并。
3. 确认 Issue `#17` 关闭、`main` 产生单父签名提交、功能分支清理，并回写最终证据。
4. Gate 3 实现继续锁定；只有新的实现 Issue 和项目所有者批准才能创建 Workspace 与 CI。

## 停止条件

- 需要创建 Cargo、Rust、Iced、UI、功能、产品 CI、发布资产、Ruleset 或 AGOS 改动。
- 上游 Release、tag、Ruleset、Issue `#16` 或 Gate 2 closeout 事实漂移。
- 路径越界、文档互相矛盾、版本/许可证证据不确定、Review 对话未闭环或 CI 失败。
- 缺少新的规划 PR Squash Merge 授权。
