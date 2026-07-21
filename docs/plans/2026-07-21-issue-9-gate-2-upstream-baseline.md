# Issue #9：Gate 2 上游 v1.2.41 基线计划

## 基本信息

- 日期：2026-07-21。
- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/9`。
- 当前 Gate：Gate 2：导入上游基线。
- 当前状态：已完成；PR `#11` 已 Squash Merge，Issue `#9` 已关闭。
- 上游 Release：`v1.2.41`。
- 上游提交：`3dafffcafb2566a1e8bce4b35671656d6adb3eda`。

## 目标

为上游正式 Release 建立可审计的完整只读快照导入方案，确保同步内容可追溯、可验证且不进入产品构建运行面。

## 已完成范围

- 确认 Release、标签提交、许可证和来源声明。
- 建立 `source-lock.json`、快照目录和同步报告。
- 完成文件清单、哈希、提交 mode/blob、禁止内容和纯净性验证。
- 明确 `upstream/` 与产品 Cargo Workspace 的隔离。
- 明确 Tauri/React UI、注入脚本和远程推荐列表只作审计材料。
- 通过独立 upstream-sync PR 完成交付、Review 根因闭环与 Squash Merge。

## 持续有效边界

- 后续修改 `upstream/` 必须建立新的 upstream-sync Issue/PR 和项目所有者允许范围批准。
- 不创建 Cargo、Rust/Iced 源码、UI、GitHub Actions 或 Release。
- 不实现、修复或重构上游功能。
- 不把上游 `main` 直接当作功能真源。
- 不把快照同步 PR 与功能实现 PR 合并。

## 计划步骤

1. [x] Fresh 核验上游正式 Release 与提交。
2. [x] 完成许可证、保留声明和来源提交记录。
3. [x] 评审并生成 `source-lock.json` 与同步报告。
4. [x] 完成快照纯净性、哈希和构建隔离验证。
5. [x] 获得明确执行范围批准，创建独立同步分支与 PR `#11`。
6. [x] PR `#11` 只更新 `upstream/`、source-lock 和报告，并完成 Review、验证与 Squash Merge。

## 验收标准

- [x] Session Plan 明确允许写入路径与禁止路径。
- [x] 上游 Release、提交、许可证和声明均可复核。
- [x] 快照纯净性与哈希验证可在 Windows/macOS 复现。
- [x] 快照不参与产品构建，且同步 PR 与功能 PR 分离。
- [x] 所有差异、无效功能或错误语义继续要求独立例外 Issue。

## 交付证据

- PR：`https://github.com/nonononull/inputcodex/pull/11`。
- 获批 Head：`90d35a72cffb4a13c5f7588a147e19cbd75b14c6`。
- Squash Merge：`dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`。
- 合并时间：`2026-07-21T19:01:02Z`。
- 同步报告：`docs/reports/2026-07-21-upstream-v1.2.41-sync.md`。
- 后续控制面收口：Issue `#12`，不得回写或改写已合并快照。
