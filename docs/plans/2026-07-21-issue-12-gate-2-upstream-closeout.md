# Issue #12：Gate 2 上游基线合并证据收口计划

## 基本信息

- 日期：2026-07-21。
- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/12`。
- 当前 Gate：Gate 2：上游基线合并证据收口。
- 基线：`main` / `dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`。
- 分支：`codex/issue-12-gate-2-upstream-closeout`。
- 上游同步交付：Issue `#9` / PR `#11`。

## 目标

将 PR `#11` 的 Squash Merge、Issue `#9` 关闭、快照完整性、Review、Ruleset 和排错证据回写到项目原生控制面，使 Gate 2 当前状态可复核且不依赖已关闭 Issue 的动态评论。

## 允许范围

- 更新 `README.md`、`build.md`、`err.md` 与 `docs/plans/PROJECT-MASTER-PLAN.md`。
- 将 Issue `#9` 的计划、Session Plan 和 Runtime Workflow 封存为已完成历史证据。
- 新增本 Issue 的计划、Session Plan、Runtime Workflow 与 closeout 报告。
- 创建关联 closeout PR，并回写真实 PR、Review 和验证引用。

## 禁止范围

- 不修改 `upstream/`、`upstream/source-lock.json` 或上游快照字节。
- 不创建 Cargo/Rust/Iced 产品源码、UI、WebView、GitHub Actions、Release 或发布资产。
- 不修改 Ruleset、仓库权限、合并方式或 AGOS。
- 不自动进入 Gate 3，不创建功能迁移或性能实现。
- closeout PR 不自动合并，必须等待项目所有者 Review 与明确 Squash Merge 授权。

## 执行步骤

1. Fresh 验证 PR `#11`、Issue `#9`、`main` merge commit、父提交、tree 和 279 条合并差异。
2. 创建 Issue `#12` 与独立 closeout 分支。
3. 更新 README、Master Plan、Issue `#9` 历史控制面、`build.md` 和 `err.md`。
4. 新增本任务 Session Plan、Runtime Workflow 与 closeout 报告。
5. 执行允许路径、快照不变、Markdown diff、Git 状态和 GitHub Fresh 验证。
6. 精确暂存、提交、普通 push 并创建关联 PR。
7. 回写 PR Head、Review、Checks 和自动合并状态，等待项目所有者决策。

## 验收标准

- [ ] PR `#11` merge commit、父提交和 tree 与 GitHub/本地 Git 完全一致。
- [ ] Issue `#9` 已关闭，Issue `#12` 是唯一活动 closeout 任务。
- [ ] `upstream/` 相对 `origin/main` 无任何差异。
- [ ] 当前仓库仍无 Cargo Workspace、产品 Rust/Iced 源码、Workflow 或 Release。
- [ ] `build.md` 提供当前可复现验证，旧规划命令明确标记为历史证据。
- [ ] `err.md` 记录超大 PR diff API 与 Git 中文路径转义的根因和恢复命令。
- [ ] closeout PR 仅修改批准路径，0 未解决 Review 对话，自动合并关闭。
