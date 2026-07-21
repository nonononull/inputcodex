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

### 2026-07-21：桌面版 apply_patch 包装器拒绝启动

- 环境：桌面版临时 `apply_patch.bat` 指向 WindowsApps 中的 Codex 可执行文件。
- 现象：运行包装器返回 `Access is denied.`。
- 根因：当前沙箱进程无法嵌套启动 WindowsApps 内的桌面版可执行文件。
- 处理：改为直接调用本机 npm 安装的 Codex 官方原生二进制，并继续使用 `--codex-run-as-apply-patch` 模式。
- 验证：最小 `README.md` 补丁返回 `Success. Updated the following files`。
- 关联：本次仓库筹备会话。

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
