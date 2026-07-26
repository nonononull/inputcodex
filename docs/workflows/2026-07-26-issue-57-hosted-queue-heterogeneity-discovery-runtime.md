# Issue #57 Hosted 队列异构性 Discovery Runtime Workflow

## 运行前置

- Tracking Issue：`#57`。
- 基线：`main@325bb2419548bc076502065dc583f54f4fddd582`。
- 范围：八路径，`scope_hash=sha256:26cc8ba51b7926c0898be56f1cec23c623963b2e944295d14d3d46bf650cd953`。
- 决策：当前会话的项目所有者直接授权仅覆盖 Discovery 文档交付；不覆盖 A/B/C 任何实质路径选择。

## 节点

1. **启动快照**：读取 Windows 本机时间、`git status --short --branch`、`git fetch --prune origin`、`git fsck --no-reflogs --connectivity-only`；对象悬挂但 `fsck` 成功仅作为 Squash 历史现象记录，不做清理。
2. **事实核验**：读取 Issue `#54`、`manifest.json`、停止报告和 ADR `0004`；核对 `run-01` 至 `run-08` 的成功 Job、Windows `2/4/1/1` CPU 队列分布、macOS `8` 次可比结果与 `max_serial_runs=8`。
3. **边界判定**：若需要 `run-09`、预算、合同/ADR 变更或 Runner 资源，立即停止并保留给项目所有者；不以“推荐 A”替代决定。
4. **文档写入**：只修改八路径，保存三条候选路径、推荐理由、风险、停止条件和“PR 不关闭 Issue #57”的语义。
5. **轻量验证**：运行 Evidence、CI 合同、仓库政策、`git diff --check` 与八路径/哈希审计；失败时先在 `err.md` 查重并记录根因，不扩展范围修复。
6. **GitHub 交付**：普通提交、普通 push、创建非 Draft PR（`Refs #57`）；Review 对话全部根因闭环，PR CI 通过且不存在未解决对话后才可 Squash Merge。
7. **交接**：将最终 PR/CI/合并事实写入 GitHub Issue/PR 评论；Issue `#57` 保持开放并明确等待所有者选择 A、B、C 或“不继续”。

## 失败恢复

- Git 远端不可用：只记录本机时间、精确错误和本地状态，恢复网络后以同一分支普通 push；禁止改 remote、Force Push 或改写提交时间。
- AGOS `needs-input`/未登记：记录外部缺口后绕过；不修改 AGOS registry、脚本、规则、workflow 或 vault。
- 验证失败：先复现、定位根因，再决定是否仍能在八路径内修复；否则停止，不能用临时放宽或跳过门禁掩盖失败。
