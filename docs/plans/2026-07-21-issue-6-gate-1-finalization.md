# Issue #6：Gate 1 仓库治理基线最终收口计划

## 基本信息

- 日期：2026-07-21。
- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/6`。
- 执行分支：`codex/issue-6-gate-1-finalization`。
- 基线提交：`b7404b0c63f2d2ba65474c077182c42a01cc9a64`。
- 范围哈希：`sha256:bd271f22bfb20eb32a78d99ae89e9554cc2de17c8486d1f4386b8980068659bd`。
- 决策状态：项目所有者已批准直接开始 Gate 1 治理收口，无需追加澄清。
- 当前状态：PR `https://github.com/nonononull/inputcodex/pull/7` 已创建并保持 `OPEN`、非 Draft、`CLEAN`，等待项目所有者 Review；未启用自动合并。

## 目标

消除 PR `#5` 已合并后仍残留的过期状态，补齐仓库级 Issue Forms、PR 模板、标签、closeout 报告和项目原生验证入口，使 Gate 1 具备可审计、可重复验证的最终治理基线。

## 权威输入

- Issue `#4`：`CLOSED`，关闭时间 `2026-07-21T15:22:00Z`。
- PR `#5`：`MERGED`，合并时间 `2026-07-21T15:21:58Z`。
- PR `#5` 合并提交：`b7404b0c63f2d2ba65474c077182c42a01cc9a64`。
- PR `#5` 最终 Head：`ecd34360ae5f6c0d1f2995ccc6724fe39bf95381`。
- Squash tree：合并提交与最终 Head 均为 `af186e05673b441a936199e55c7d632cd06ea929`，合并提交只有一个父提交。
- Review 对话：总数 `0`，未解决数 `0`。
- Checks：`0`；当前没有 GitHub Actions 或 required status checks。
- Ruleset：`main-protection`（ID `19395456`）为 `active`，只命中 `main`，无 bypass，只允许 Squash Merge，required approvals 为 `0`，并要求解决全部 Review 对话。
- 上游最新正式 Release：`v1.2.41`，提交 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。

## 允许操作

- 更新 `README.md`、`build.md`、Master Plan 与架构治理状态。
- 新增本计划、Session Plan、Runtime Workflow 和 Issue `#4` closeout 报告。
- 新增八类 Issue Forms、Issue 模板配置和 PR 模板。
- 在 GitHub 创建已批准的类型、Gate、平台和状态标签；保留默认标签。
- 给 Issue `#6` 添加 `type:architecture` 与 `gate:1` 标签。
- 在确认 Squash tree 完全一致后删除 PR `#5` 的远端、本地与跟踪旧分支。
- Fresh 验证后提交、正常推送并创建包含 `Closes #6` 的非 Draft PR。
- PR 创建后向 Issue `#1` 回写追踪证据，但不提前关闭 Issue `#1`。

## 禁止操作

- 不导入上游或半成品源码。
- 不创建 Cargo Workspace、Rust/Iced 源码、UI 或 `.github/workflows/`。
- 不修改 Ruleset、required checks 或仓库级合并开关。
- 不创建 Release、安装包、签名或更新资产。
- 不 Force Push，不删除 `main`，不直接向 `main` 写入。
- 不修改、修复或优化 AGOS；本任务不运行 AGOS。
- 未经项目所有者再次明确授权，不合并本任务 PR。

## 执行批次

1. 核验本地、GitHub、PR `#5`、Ruleset 和上游 Release 事实。
2. 创建 Issue `#6` 与专用分支，记录启动快照。
3. 修正 README、Master Plan、架构治理状态并补齐 closeout 控制文档。
4. 创建 Issue Forms、PR 模板和项目标签。
5. 核验 PR `#5` Squash tree 后安全清理旧分支。
6. 更新 `build.md`，运行本仓项目原生 Fresh 验证。
7. 暂存、提交并正常推送当前分支。
8. 创建包含 `Closes #6` 的开放非 Draft PR，回写 Issue `#1` 与控制面 URL。
9. 复核 PR、Review 对话与远端提交一致后停止，等待项目所有者 Review。

## 完成标准

- README、Master Plan、架构治理和 `build.md` 不再把 Issue `#4` 或 PR `#5` 描述为待处理。
- 八类 Issue Forms、`config.yml` 与 PR 模板全部存在并可被 YAML/文本验证。
- 已批准标签全部存在，默认标签未删除。
- Issue `#4` closeout 报告包含 PR `#5` 的 Review、Checks、Squash tree 与分支清理证据。
- 仓库仍无 Rust 源码、`Cargo.toml`、GitHub Actions Workflow 和 Release。
- Ruleset 仍保持批准值，且 Review 根因闭环要求已进入模板。
- 当前分支通过 `build.md` 的全部 Fresh 验证。
- 关联 PR 保持开放，不自动合并；Issue `#1` 只回写追踪证据，不提前关闭。
