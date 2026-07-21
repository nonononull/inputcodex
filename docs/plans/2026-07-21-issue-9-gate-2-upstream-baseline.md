# Issue #9：Gate 2 上游 v1.2.41 基线计划

## 基本信息

- 日期：2026-07-21。
- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/9`。
- 当前 Gate：Gate 2：导入上游基线。
- 当前状态：规划与来源锁定，尚未批准快照写入。
- 上游 Release：`v1.2.41`。
- 上游提交：`3dafffcafb2566a1e8bce4b35671656d6adb3eda`。

## 目标

为上游正式 Release 建立可审计的完整只读快照导入方案，确保同步内容可追溯、可验证且不进入产品构建运行面。

## 当前允许范围

- 确认 Release、标签提交、许可证和来源声明。
- 设计 `source-lock.json` 字段、快照目录和同步报告格式。
- 设计文件清单、哈希、禁止内容和纯净性验证。
- 明确 `upstream/` 与产品 Cargo Workspace 的隔离。
- 明确 Tauri/React UI、注入脚本和远程推荐列表只作审计材料。
- 准备后续 upstream-sync PR 的验收清单。

## 当前禁止范围

- 未经新的 Session Plan 与项目所有者允许写入范围批准，不创建或修改 `upstream/`。
- 不创建 Cargo、Rust/Iced 源码、UI、GitHub Actions 或 Release。
- 不实现、修复或重构上游功能。
- 不把上游 `main` 直接当作功能真源。
- 不把快照同步 PR 与功能实现 PR 合并。

## 计划步骤

1. Fresh 核验上游正式 Release 与提交。
2. 完成许可证、保留声明和来源提交记录。
3. 评审 `source-lock.json` 与同步报告格式。
4. 评审快照纯净性、哈希和构建隔离验证。
5. 获得明确的执行范围批准后，创建独立同步分支与 PR。
6. 同步 PR 只更新 `upstream/`、source-lock 和报告，并完成 Review/CI/合并证据。

## 验收标准

- [ ] Session Plan 明确允许写入路径与禁止路径。
- [ ] 上游 Release、提交、许可证和声明均可复核。
- [ ] 快照纯净性与哈希验证可在 Windows/macOS 复现。
- [ ] 快照不参与产品构建，且同步 PR 与功能 PR 分离。
- [ ] 所有差异、无效功能或错误语义均有独立例外 Issue。
