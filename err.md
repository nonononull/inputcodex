# 排错记录

## 使用规则

- 遇到错误先搜索本文件，重复问题优先引用现有记录。
- 新错误按“日期、环境、现象、根因、处理、验证、关联提交或 Issue”记录。
- 未确认根因时标记为“调查中”，不得把猜测写成结论。

## 已知记录

### 2026-07-21：AGOS 默认入口处于筹备阻塞状态

- 环境：仅有空 Git 仓库，尚无项目级入口文档和任务登记。
- 现象：`invoke-agos-default-entry.ps1 -ReportOnly` 返回 `AGOS_DEFAULT_ENTRY_STATUS=needs-input`。
- 根因：新项目尚未建立项目入口文档和正式任务控制材料。
- 处理：创建项目治理文档、筹备计划、运行工作流和 GitHub Issue；当前不导入源码。
- 验证：文档提交后重新运行默认入口报告与 Git 快照治理检查。
- 关联：GitHub Issue `#1`。

### 2026-07-21：单条补丁命令超过 Windows 长度上限

- 环境：PowerShell 将多个完整文档作为一个 `apply_patch` 参数传入。
- 现象：命令返回 `The command line is too long.`，没有文件被修改。
- 根因：单个 Windows 进程命令行承载的补丁文本过长。
- 处理：按职责把补丁拆成多个小批次。
- 验证：小批次补丁可进入补丁程序并正常处理。
- 关联：本次仓库筹备会话。

### 2026-07-21：桌面版 apply_patch 入口随安装版本变化

- 环境：Codex Desktop、PowerShell 与 `--codex-run-as-apply-patch` 模式。
- 现象：临时 `apply_patch.bat` 曾返回 `Access is denied.`；继续硬编码 npm 包内原生二进制后又出现 `ENOENT`；通过 stdin 传递补丁时返回 `requires a UTF-8 PATCH argument`。
- 根因：Codex 安装位置和内部原生二进制路径会随桌面版或 npm 包更新变化，不能把内部绝对路径视为稳定入口；当前接口要求把完整补丁作为单个参数传入，而不是通过 stdin。
- 处理：先用 `Get-Command codex` 解析当前有效命令，再执行 `& codex --codex-run-as-apply-patch $patch`；禁止继续硬编码包内二进制路径或把补丁管道传入。
- 验证：命令退出码为 `0`，并返回 `Success. Updated the following files`，本次五个治理文件均按预期更新。
- 关联：GitHub Issue `#2`、PR `#3`。

### 2026-07-21：默认 PowerShell Shell Runner 间歇拒绝启动

- 环境：Codex Desktop 调用默认 `C:\Users\dashuai\AppData\Local\Microsoft\WindowsApps\pwsh.exe` 执行只读 Git 与文档查询。
- 现象：命令在实际脚本执行前返回 `CreateProcessAsUserW failed: 5 (拒绝访问。)`；相同仓库内容通过 Node 只读接口或提升后的 Shell 可以正常读取。
- 根因：故障发生在 WindowsApps `pwsh.exe` 的沙箱进程启动层，不是 Git 仓库、命令参数或项目文件错误。
- 处理：不连续盲目重试；只读查询可切换 Node REPL，必须使用 PowerShell 或项目脚本时使用 `require_escalated`，文件修改仍坚持官方 `apply_patch` 模式。
- 验证：Node REPL 成功读取 Git 状态和控制文档；提升后的 Shell 成功执行相同 Git、`rg` 与 `Get-Content` 查询。
- 关联：GitHub Issue `#2`、PR `#3`。

### 2026-07-21：AGOS 严格入口未登记 inputcodex Issue #2

- 环境：外部项目 `C:\Users\dashuai\Documents\inputcodex` 调用 AI Growth OS 默认入口。
- 现象：`invoke-agos-default-entry.ps1 -ReportOnly` 返回 `TASK_REGISTRATION_STATUS=unregistered`、`AGOS_DEFAULT_ENTRY_STATUS=needs-input`，并禁止严格模式下的项目文档写入、提交和 PR。
- 根因：AI Growth OS 全局 `registry/task-backlog.yml` 和 `registry/business-paths.yml` 尚未登记本项目任务 `2026-07-21-issue-2-architecture-governance` 与 `architecture-governance` 路径。
- 处理：不在 inputcodex PR 中越权修改跨仓控制面；在 Session Plan、Runtime Workflow 和 `build.md` 中明确外部项目 warning mode，以 GitHub Issue `#2`、当前分支和项目所有者批准作为本仓任务证据，并继续禁止源码实现。
- 验证：默认入口同时报告 Git foundation、入口文档和本地知识查询为 `ready`；当前不宣称 `verify-runtime-workflow.ps1` 严格校验通过。
- 关联：GitHub Issue `#2`。

### 2026-07-21：Git 快照检查阻止继续扩大未提交文档

- 环境：Issue `#2` 文档分支已有未提交的总方案和 ADR。
- 现象：`verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly` 返回 `GIT_SNAPSHOT_STATUS=blocked`，并列出三个 critical untracked 文档。
- 根因：关键治理文档尚未形成命名 Git 快照；检查器按设计阻止在脏工作树上无限扩展改动。
- 处理：将现有草案视为同一个有界执行批次，补齐 Session Plan、Runtime Workflow、Master Plan、`build.md` 和本记录后停止扩大范围，运行 Fresh 验证并提交快照。
- 验证：提交前重新执行 Git snapshot、`git diff --check` 和 `git diff --cached --check`；提交后要求工作树只保留预期状态。
- 关联：GitHub Issue `#2`。

### 2026-07-21：Major Session Plan 使用了非标准 reviewer 角色

- 环境：运行 `verify-session-plan.ps1` 校验 Issue `#2` 的 Major Session Plan。
- 现象：校验器报错 `Major session plan missing proposal role: architecture-reviewer`。
- 根因：草案把三个方案路线误写为 `agent_proposals.role`，并使用了校验器不接受的 `proposal_mode: owner-dialogue`；Major 合同要求固定的 `architecture-reviewer`、`operator-experience-reviewer`、`verification-reviewer` 与受支持枚举。
- 处理：保留用户已批准的实质决策，把三个审查视角映射到标准角色，并将模式改为 `simulated-roles`，明确未启动子 agent 的原因。
- 验证：重新运行校验输出 `SESSION_PLAN_VERIFY_OK`，工作类别为 `major`。
- 关联：GitHub Issue `#2`。

## 记录模板

```text
### YYYY-MM-DD：问题标题

- 环境：
- 现象：
- 根因：
- 处理：
- 验证：
- 关联：
```
