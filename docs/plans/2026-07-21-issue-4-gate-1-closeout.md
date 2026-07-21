# Issue #4：Gate 1 合并证据 closeout 计划

## 基本信息

- 日期：2026-07-21。
- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/4`。
- 执行分支：`codex/issue-4-gate-1-closeout`。
- 基线提交：`0e11375997ff10fdc0c233b31c8468af2d9a4f44`。
- 范围哈希：`sha256:91d211b15a79b3795b61048db95ae5490726ceac5a25da98fda60cf693790aa8`。
- 当前状态：PR `https://github.com/nonononull/inputcodex/pull/5` 已创建并保持 `OPEN`、非 Draft、`CLEAN`，等待项目所有者 Review。

## 目标

把 Issue `#2` 与 PR `#3` 已完成的 Review、CI 边界、Squash Merge、Issue 关闭和分支清理事实写回项目控制面，消除 Master Plan 与入口文档中“PR #3 仍 OPEN”的过期状态。

## 权威输入

- PR `#3`：`MERGED`，合并时间 `2026-07-21T13:15:51Z`。
- 合并提交：`0e11375997ff10fdc0c233b31c8468af2d9a4f44`。
- PR Head：`6b090ba5aa479c714c9e231aa07787724d6a8190`。
- Issue `#2`：`CLOSED`，关闭时间 `2026-07-21T13:15:52Z`。
- Review 对话：总数 `0`，未解决数 `0`。
- Checks：`0`；Gate 1 未配置 Actions 或 required status checks。
- Squash 证据：合并提交只有父提交 `09564740b8d00a4b09630c024607cc5292d0381f`，且 merge tree 与 PR Head tree 均为 `0730422eb3fa738fe2d05a51e5191832fbfec0fe`。
- 分支清理：远端与本地 `docs/issue-2-architecture-governance` 均不存在。
- Ruleset：`main-protection`（ID `19395456`）仍为 `active`、无 bypass actor、只命中 `main`，并只允许 Squash Merge。

## 允许操作

- 新增本计划、Issue `#4` Session Plan、Runtime Workflow 和 Issue `#2` closeout 报告。
- 更新 Master Plan、README、总架构、Issue `#2` Session Plan 与 Runtime Workflow。
- 更新 `build.md` 与 `err.md` 的 closeout 验证和排错证据。
- 固化 AGOS“可用则用、不可用绕过、禁止在本仓优化”的外部辅助边界。
- Fresh 验证后提交、正常推送并创建包含 `Closes #4` 的非 Draft PR。
- PR 创建后回写真实 PR URL，再次验证、提交并推送。

## 禁止操作

- 不导入上游或半成品源码。
- 不创建 Cargo Workspace、Rust/Iced 源码、UI 或 `.github/workflows/`。
- 不修改 Ruleset、required checks、仓库级合并开关或其他分支规则。
- 不发布安装包、版本或更新资产。
- 不 Force Push，不删除 `main`，不直接写 `main`。
- 不修改、修复或优化 AGOS 的脚本、规则、Registry、Workflow、Vault 或其他跨仓控制面。
- 未经项目所有者再次明确授权，不合并本 closeout PR。

## 执行顺序

1. 记录启动 Git 状态和本地项目资料查询；AGOS 仅在可用且适用时作为可选辅助。
2. 写入 closeout 报告与 Issue `#4` 控制文档。
3. 修正 Master Plan、入口文档和 Issue `#2` 历史控制面。
4. 运行项目原生文档、Git、GitHub、Ruleset 与 Squash 证据验证。
5. 如 AGOS 可用且适用，可记录其只读辅助结果；不可用、未登记、`needs-input` 或异常时记录并绕过，不执行 AGOS 修复或 rollout 优化。
6. 创建命名 Git 提交，正常推送当前分支。
7. 创建包含 `Closes #4` 的开放 PR，回写真实 PR URL 并再次推送。
8. 复核 PR 为 OPEN、非 Draft、无未解决 Review 对话；停止在等待项目所有者 Review。

## 完成标准

- `docs/reports/issue-2-architecture-governance-closeout.md` 包含可重复核验的完整合并证据。
- Master Plan 不再把 PR `#3`、Issue `#2` 或旧功能分支描述为待处理。
- Issue `#2` Session Plan 与 Runtime Workflow 的 `review_ref`、`ci_ref`、`merge_ref` 和 `closeout_ref` 均为真实值。
- Issue `#4` 的 Session Plan、Runtime Workflow、分支和 PR 相互引用一致。
- AGOS 状态不再被描述为项目阻塞或合并门禁，且所有跨仓优化均明确排除。
- Fresh 验证通过，且改动仅覆盖本 Issue 允许的 Markdown 文件。
- closeout PR 保持开放，不自动合并。
