# Issue #83：README 信息架构与文档入口实施报告

## 当前结论

方案 B“稳定首页 + 文档门户”已在批准的十路径内完成内容实施。README 已从项目流水账
收敛为用户优先的公开首页，分类文档门户、职责边界、PR #82 后稳定状态和新排错根因均已
落盘；本地内容、范围、链接、CI 合同、Release Audit、仓库政策、暂存级 diff check 和
官方纯文档分类均已通过，当前只剩提交、普通推送、非 Draft PR 与 Hosted Review/CI。

## 基线与批准

- Tracking Issue：<https://github.com/nonononull/inputcodex/issues/83>
- 基线：`main@da65f7d8402e4de27e2795ee8905be18ad565653`
- Planning checkpoint：`c313cba06484eb00ccc373e63419602d13192f7c`
- 批准范围评论：<https://github.com/nonononull/inputcodex/issues/83#issuecomment-5101769289>
- candidate scope count：`10`
- candidate scope hash：
  `sha256:d8a404c19b108587a5e17b4ded454444d5e948c92410b759504a7eb7c63bed44`

## README 收敛结果

| 指标 | 修改前 | 修改后 |
| --- | ---: | ---: |
| 行数 | 193 | 87 |
| 字节 | 23,425 | 4,073 |
| 列表项 | 158 | 21 |
| Issue 编号引用 | 121 | 0 |
| 完整提交 SHA | 34 | 0 |
| Actions Run 编号 | 18 | 0 |

README 字节减少约 `82.6%`，同时保留项目定位、开发阶段警告、三个已迁移能力、七成员
Workspace、构建入口、上游边界、贡献流程和许可证。

## 文档门户

新增 `docs/README.md`，共 `76` 行，提供：

- 项目入口；
- 四份 ADR；
- 三个 Gate 5 已迁移能力的设计与报告；
- 上游审计和 Parity 入口；
- 性能基线与预算入口；
- Plan、Session Plan、Runtime Workflow 和报告目录导航；
- 面向新用户、贡献者和 Reviewer 的阅读顺序。

初步链接检查覆盖 README 与文档门户的 `42` 个 Markdown 链接，失效数量为 `0`。

## 职责边界

- AGENTS 固定 README、docs portal、Master Plan、build、err、task-local 文档和 GitHub
  动态证据的唯一职责；
- Master Plan 已把产品活动面切换为“第四个 Gate 5 切片尚未选择”，并记录 PR #82
  合并后的稳定事实；
- build.md 顶部改为可重复命令职责，并增加通用纯文档轻量验证入口；
- README 不再因当前 Head、Review、CI 或下一操作发生变化。

## 排错复用

- Issue 创建正文首次命中 PowerShell 双引号 Markdown 反引号解析，复用 err.md 既有
  “单引号 here-string + token 替换”结论，失败发生在 API 调用前；
- 默认 `apply_patch.bat` 命中既有 WindowsApps 权限根因，复用 npm 官方
  `bin/codex.js --codex-run-as-apply-patch` 入口；
- 新增根因：`git check-ignore .worktrees` 对尚不存在目录本身不命中 `.worktrees/`，
  应检查 `.worktrees/` 或目录内探针路径。

## 非目标确认

本任务没有修改 Rust、Cargo、UI、CI Workflow、Ruleset、性能预算、Parity、`upstream/`、
第四个 Gate 5 功能或 AGOS，也没有删除任何历史文档或创建递归 Closeout。

## 本地验证证据

本机时间 `2026-07-28 16:39:15 +08:00` 至 `2026-07-28 16:39:45 +08:00`：

- 实际差异与候选范围均为 `10` 路径；
- candidate scope hash 为
  `sha256:d8a404c19b108587a5e17b4ded454444d5e948c92410b759504a7eb7c63bed44`；
- README 为 `87` 行、`4,073` 字节、`21` 个列表项；
- README 的 Issue 编号、完整提交 SHA 和 Actions Run 引用均为 `0`；
- README 与文档门户共检查 `42` 个 Markdown 链接，失效数量为 `0`；
- 十路径非法控制字节数量为 `0`；
- CI 合同输出 `CI_CONTRACT_GREEN passed=35`；
- Release Audit 输出 `status=current`、`requires_reaudit=false`；
- Repository Policy 输出 `violation_count=0`；
- `origin/main` 仍为批准基线，Issue #83 保持 OPEN。

本机时间 `2026-07-28 16:41:23 +08:00` 至 `2026-07-28 16:41:49 +08:00`：

- 十路径全部进入暂存区，未暂存和未跟踪差异均为 `0`；
- `git diff --cached --check origin/main` 通过；
- 官方 `Collect-Changes.ps1` 与 `Classify-Changes.ps1` 输出
  `docs_only=true`、`heavy=false`、`change_count=10`、`errors=[]`；
- CI 合同再次输出 `CI_CONTRACT_GREEN passed=35`；
- Release Audit 再次为 `current`；
- Repository Policy 再次为 `0` 违规。

## 剩余门禁

1. Git checkpoint、提交、普通推送和非 Draft PR；
2. Hosted Review/CI 根因闭环；
3. 项目所有者独立 Squash Merge 授权。
