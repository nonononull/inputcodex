# Issue #69：性能实现哈希路径边界 Discovery 报告

## 元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/69`
- 基线：`a2e5e5a6728200739a4acb85042ba7831ac6295b`
- 分支：`codex/issue-69-performance-hash-boundary`
- 阻塞 PR：`#68`，Final Head `159abbf45dfdc29a277cb152af7368868f2f618d`
- 失败 Run：`30203435572`
- 十路径提案：`sha256:6392a1b4150f2aae0c34c285e83b6870f47e3bdc57def6308d0a26ddd158911d`
- PR：`https://github.com/nonononull/inputcodex/pull/70`
- 当前状态：实现、Hosted 测量、Evidence 入库、本地最终门禁与非 Draft PR 已完成；等待最终 Head Review/CI，Squash Merge 仍单独授权。

## GitHub 事实

- PR `#68` 为 OPEN、非 Draft、可合并，Head 保持 `159abbf45dfdc29a277cb152af7368868f2f618d`。
- 标准 CI Run `30203435576` 七 Job 全绿，成功 Artifact 数为 `0`。
- PR `#68` Review 对话总数 `0`、未解决数 `0`。
- Performance Baseline Run `30203435572` 的 `contract` 成功；`windows`、`macos` 和 `required` 失败。

## 可重复根因

- 本地在 PR `#68` 工作树运行 Evidence，稳定输出三个 `HASH_MISMATCH`。
- 当前计算哈希为 `sha256:174f8aec273a5b7490aa833e7554e26edda996a649105ce916d9cc4873ea8bd7`；三份入库 Evidence 均为 `sha256:e4c9265396476c918112f553d846239ed21f84c7432e6a34ddc8f55293d64e48`。
- `Test-InputcodexBaseline.ps1` 第 263—266 行递归读取七个产品组件目录中的全部 `.rs`，因此 `crates/inputcodex-parity/tests/catalog_repository.rs` 被错误归入性能实现。
- PR `#68` 没有改变产品 `src/**`、Cargo、性能夹具、采集器或 Workflow；失败是哈希边界错误，不是性能回归。

## 二阶语义核验

```json
{
  "stored_evidence": "sha256:e4c9265396476c918112f553d846239ed21f84c7432e6a34ddc8f55293d64e48",
  "main_old_algorithm": "sha256:e4c9265396476c918112f553d846239ed21f84c7432e6a34ddc8f55293d64e48",
  "pr68_old_algorithm": "sha256:174f8aec273a5b7490aa833e7554e26edda996a649105ce916d9cc4873ea8bd7",
  "main_narrowed_surface": "sha256:39fcc5593ada18c4b42daf2e94556a97dda1e90385f146184178418a79b38a7e",
  "pr68_narrowed_surface": "sha256:39fcc5593ada18c4b42daf2e94556a97dda1e90385f146184178418a79b38a7e",
  "narrowed_surfaces_equal": true,
  "stored_matches_narrowed": false,
  "validator_edit_changes_hash": true
}
```

收窄路径可以正确解耦 PR `#68`，但无法让旧 Evidence 自动通过：路径集合本身已变化，且两个待修改脚本属于固定实现输入。

## 历史合同

- `err.md` 已明确记录：`Test-CiScripts.ps1` 属于 `implementation_sha256` 输入，审查修复后旧 Evidence 必须由同一精确 Head 的 hosted Artifact 刷新。
- `docs/workflows/issue-32-performance-baseline-runtime.md` 明确记录：验证器变化后旧结果必须重新 measure，禁止直接替换元数据。
- 因此“只改两个脚本、保持三份旧 Evidence 原样 GREEN”与当前严格合同互相矛盾。

## 外部治理

- GitNexus `list_repos` 返回空列表，仓库根不存在 `.codegraph`；未擅自初始化代码图谱。
- AGOS Default Entry 只运行一次 `ReportOnly`，返回 `needs-input`、`unregistered`、`doctor=blocked`；项目 Git 与入口文档为 `ready`。
- 按 `inputcodex` 项目规则立即绕过 AGOS；没有登记、修复、优化或修改 `D:\Android_source\ai-growth-os`。
- 后续提交前误向 `verify-session-plan.ps1` 传入其不支持的 `-ReportOnly`；该接口问题已在 `err.md` 有既有记录，因此不继续消耗本任务修复外部治理，改用项目原生合同完成门禁。

## 方案结论

- 推荐 A2：收窄动态实现路径，并从修复后精确 Head 做一次正式双平台重新测量；十路径包含三份 Evidence。
- 不采用 B：从历史提交重建版本化兼容哈希，依赖 Git 历史和 checkout 深度，复杂且扩大 CI 面。
- 禁止 C：直接改写哈希、忽略当前实现哈希或复用旧 Artifact。

## 当前停止门

项目所有者已通过 `https://github.com/nonononull/inputcodex/issues/69#issuecomment-5083808758` 批准十路径、`scope_hash`、TDD、Hosted 重新测量、Evidence 入库、提交、推送、PR 与 Review/CI。实施仍受十路径、PR `#68` Head 不变、普通推送和最终 Squash Merge 单独授权约束。

## TDD 实施证据

- 新增真实隔离 Contract 夹具，分别修改普通 `crates/inputcodex-parity/tests/**` 与产品 `crates/inputcodex-parity/src/**`。
- RED：旧脚本下普通测试变化将实现哈希从 `sha256:5bfa9444ffb93d9d55271cefe969e4a11eb75b4307610c0d91dc3adcb99cfc68` 改为 `sha256:8a1a819157bef90f992ce86fceb7587d4159908c95d258d788ea32dbf3e792e9`，CI 合同退出码 `1`。
- GREEN：动态实现路径只纳入七个组件根 `Cargo.toml` 与 `src/**/*.rs`；普通 tests 变化哈希不变，产品 src 变化哈希改变。
- 完整 CI 合同达到 `35/35` GREEN；性能 Contract `ok=true`、`violation_count=0`，当前实现哈希为 `sha256:ed9a8c27972a8b99b331031af171fbf348587481079d62f05f2dc54a88536faa`。
- Hosted 重新测量前，旧 Windows、macOS、manifest Evidence 按预期产生三个 `HASH_MISMATCH`，没有手工改写或删除证据。
- 实现检查点原生门禁通过：CI 合同 `35/35`、性能 Contract 零违规、Repository Policy 零违规、精确七路径、批准十路径 `scope_hash`、非法控制字节与 `git diff --check` 均通过。

## Hosted 重新测量与 Evidence 入库

- 实现检查点提交为 `3d25fafe8b9c085aaaa0069d8e5c93e6afc63eac`，Tree 为 `733319d274a854248d7c706a74333ab8df8d2232`，父提交为 `a2e5e5a6728200739a4acb85042ba7831ac6295b`；普通推送后从该精确 Head 触发 Run `30205624985` Attempt `1`。
- `contract`、`windows`、`macos`、`required` 四 Job 全绿；Windows 用时约 `6m20s`，macOS 用时约 `3m33s`。
- Windows Artifact `8633022467`，名称 `performance-windows-30205624985-1`，结果归一化 SHA-256 为 `sha256:d8737f5039fd58b00f8d9d542fff3deb54b512ed0aa5ec145771726e9f79dd00`。
- macOS Artifact `8632997457`，名称 `performance-macos-30205624985-1`，结果归一化 SHA-256 为 `sha256:36b2d08185144617c5ea785b86731771127b9601184b8e64e4e971959944a17d`。
- 两平台均绑定提交 `3d25fafe8b9c085aaaa0069d8e5c93e6afc63eac`、Tree `733319d274a854248d7c706a74333ab8df8d2232`、实现哈希 `sha256:ed9a8c27972a8b99b331031af171fbf348587481079d62f05f2dc54a88536faa`、配置哈希 `sha256:b9ed601016ececc735634aeb143965c78fbfc61819d37c7f9e584bc971642b53` 与输入哈希 `sha256:c5b507d219ff49975c13805a2a6e036ade6c61a33a184bfffb74219ce01784b5`。
- 每个平台均包含 `5` 个首次 view 成功样本、`60` 个空闲资源样本和 `3` 个 Rust 场景；下载文件与入库文件哈希完全一致。
- Windows、macOS 与 manifest 已通过 `apply_patch` 写入；本地 Evidence 验证为 `ok=true`、`violation_count=0`，没有手工替换单一哈希字段。
- 两个成功 Artifact 在 Evidence 提交并普通推送后删除，Run `30205624985` Artifact 数为 `0`；本机临时下载目录已安全清理。

## PR 与最终本地门禁

- PR `#70` 为 OPEN、非 Draft，base 为 `main`，首次 PR Head 为 `4a5bf6747b549856315ffaa5dfb76766d36fa0e7`，变更恰好十路径。
- 最终本地门禁：CI 合同 `35/35`、性能 Evidence 零违规、预算合同 `10/10`、Release Audit `current`、Repository Policy 零违规、十路径 `scope_hash`、JSON/Artifact 身份、PR `#68` Head 不变、差异与控制字节检查全部通过。
- PR `#70` 最终 Head 的标准 CI、Performance Baseline、Review 对话与 Artifact 数待 GitHub 回写；这些证据只写入 Issue/PR 评论，避免为记录 Final Head 再次改变 Final Head。
