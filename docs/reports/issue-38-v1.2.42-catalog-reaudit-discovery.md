# Issue #38：`v1.2.42` 功能目录重新审计发现报告

```yaml
report_status: implementation-green-awaiting-pr-review-ci
issue_ref: https://github.com/nonononull/inputcodex/issues/38
baseline_ref: fdb2f98c701800969fc478f95cd2539be598faaa
branch_ref: codex/issue-38-v1.2.42-catalog-reaudit
local_discovery_time: 2026-07-25 18:35:54 +08:00
planning_scope_hash: sha256:c7c32b7d07f5f1b04acba9c465e1bc4bc5021228b18c438e85b40d7db5f56add
implementation_scope_hash: sha256:a384353e947bcb9d95b51ac5ccce49ef9558ca34580c130307a64b6d868819af
implementation_approval_status: approved
approval_ref: https://github.com/nonononull/inputcodex/issues/38#issuecomment-5078279459
pr_ref: pending-creation
merge_ref: pending-separate-owner-authorization
```

## 结论

Issue `#38` 的硬前置已全部解除。正式 Release `v1.2.42` 相对当前功能目录基线 `v1.2.41` 的真实差异可收敛为四个行为复审域；其余变化属于版本元数据、上游 UI 布局或非功能图片。

“**全目录 Release 对齐 + 受影响行为定向复审**”已经按获批二十六路径实施：五域目录、五域合同和 source-index 对齐 `v1.2.42`，两组 fixture 固定受影响语义，`release_audit` 已恢复 `current`。产品运行面、缓存文件、Cargo、CI Workflow、性能、UI 与 AGOS 保持零差异；当前等待提交、PR、Review/CI。

## 输入与前置证据

| 输入 | 复核结果 |
| --- | --- |
| Issue `#34` / PR `#40` | `v1.2.42` 完整缓存已进入 `main`，`release_audit` 诚实保持 stale。 |
| Issue `#37` / PR `#39` | Windows CRLF 下的 CI 合同假失败已独立修复。 |
| Issue `#41` / PR `#42` | 固定 Release 值与合法 stale 状态已解耦。 |
| Issue `#43` / PR `#44` | 长期控制面已收口；当前基线为 `fdb2f98c701800969fc478f95cd2539be598faaa`，合并后 Run `30152001233` 成功。 |
| 正式 Release | `v1.2.42 @ 657cd33e009ad02515d30db6492cd4e669b06318`。 |
| Git tree | `be938b3cfa7db919c6c17322f4617ab286f280d2`。 |
| Release 差异 | 17 个提交、18 个修改文件、0 个新增、0 个删除。 |
| 当前目录基线 | `v1.2.42 @ 657cd33e009ad02515d30db6492cd4e669b06318`，状态 `current`。 |

发现使用仓库锁定报告、只读缓存、GitHub Compare API 和现有目录/合同/fixture 完成；没有运行上游 Tauri、React、JavaScript 或 Rust 产品代码。GitNexus 当前没有已索引仓库，项目根也不存在 `.codegraph`，因此没有擅自初始化代码图谱。

## 18 个文件逐项分类

| # | 路径 | 分类 | 处置 |
| --- | --- | --- | --- |
| 1 | `Cargo.lock` | 依赖/版本元数据 | 只保留来源完整性证据。 |
| 2 | `Cargo.toml` | Workspace 版本元数据 | 不形成目录语义变化。 |
| 3 | `apps/codex-plus-launcher/src/main.rs` | 本地会话行为 | 纳入允许数据库路径和多库删除复审。 |
| 4 | `apps/codex-plus-manager/package-lock.json` | 前端依赖锁 | 不进入产品或目录语义。 |
| 5 | `apps/codex-plus-manager/package.json` | 前端版本元数据 | 不进入产品或目录语义。 |
| 6 | `apps/codex-plus-manager/src-tauri/tauri.conf.json` | Tauri 版本元数据 | 不进入 Rust/Iced 运行面。 |
| 7 | `apps/codex-plus-manager/src/App.tsx` | 确认框布局 | UI 交互变化不进入本任务。 |
| 8 | `apps/codex-plus-manager/src/dream-skin.test.ts` | companion 行为证据 | 支持 library 配置审计，同时证明显示依赖注入。 |
| 9 | `apps/codex-plus-manager/src/dream-skin.ts` | companion 配置结构 | 纳入 Dream Skin 本地主题库合同。 |
| 10 | `apps/codex-plus-manager/src/styles.css` | 确认框样式 | 不进入本任务。 |
| 11 | `assets/inject/renderer-inject.js` | renderer 注入 | 只补例外证据，继续 `exception-pending`。 |
| 12 | `crates/codex-plus-core/src/app_paths.rs` | Windows 包身份 | 纳入平台路径复审。 |
| 13 | `crates/codex-plus-core/src/assets.rs` | renderer revision | 只支持注入变化证据，继续例外。 |
| 14 | `crates/codex-plus-core/src/codex_sqlite.rs` | SQLite 根目录与多库枚举 | 纳入本地会话管理复审。 |
| 15 | `crates/codex-plus-core/tests/cdp_bridge.rs` | 注入桥验证 | 只支持 renderer 例外证据。 |
| 16 | `crates/codex-plus-data/src/storage.rs` | 多库删除与撤销 | 纳入本地会话管理复审。 |
| 17 | `crates/codex-plus-data/tests/storage_adapter.rs` | 多库行为测试 | 用于更新输入、输出、失败与副作用证据。 |
| 18 | `docs/images/discussion-group-qr.jpg` | 非功能图片 | 不进入功能目录。 |

## 行为发现

### 1. 平台路径

`app_paths.rs` 新增 Windows 包身份 `OpenAI.ChatGPT-Desktop`，仍使用 `App` 作为应用 ID。它扩展 Windows 安装检测覆盖面，不改变 macOS 的路径语义；目录必须明确这是平台特有输入、双平台等价能力，而不是照搬上游启动器。

### 2. 本地会话管理

`CODEX_SQLITE_HOME` 在值非空且目录真实存在时成为 SQLite 根目录。会话删除从单数据库扩展到候选数据库集合，删除结果可携带组合 undo token；撤销必须先对全部数据库、恢复行和文件做预检，随后才执行写回，并拒绝来源数据库超出允许路径集合。上游管理面还保持删除期间撤销窗口并在成功撤销后刷新列表。

这些变化需要更新本地会话管理的输入、输出、文件/数据库副作用、部分失败防线和脱敏 fixture。fixture 不能包含项目所有者真实路径、真实会话或真实数据库内容。

### 3. Dream Skin 本地主题库

主题配置新增可选 companion：`dataUrl` 只接受 `png/jpeg/webp/gif` 的 base64 图片，可声明宽度、左右侧与偏移。该配置结构是可审计的本地主题库能力，可以进入 library 合同与合成 fixture；实现时不得复制 TypeScript 类型或引入 WebView。

### 4. Dream Skin 运行时与 renderer 增强

companion 的显示依赖 `renderer-inject.js` 创建 DOM 节点、查找 composer、监听变化和清理注入元素，并伴随 renderer revision 更新。该机制违反本项目禁止 JavaScript 业务代码、WebView 和直接注入迁移的约束，因此 `dream-skin-runtime` 与 `renderer-enhancements` 继续保持 `exception-pending`。

本 Issue 只能补充例外理由和来源证据。是否以后用 Rust/Iced 原生方式提供等价能力，必须另建功能或一致性例外 Issue 决策。

## 无功能影响与拒绝输入

- Cargo、npm 和 Tauri 版本字段不会单独触发功能目录语义变化。
- 上游 React 确认框布局、CSS 和讨论群二维码不进入新架构。
- 上游 `main` 只作为预警源；任何只存在于 `main` 的 sponsor、广告或推广变化都不是本次正式 Release 功能真源，并且受项目无广告规则阻断。
- 远程推荐列表、现有注入脚本和 Tauri/React 管理界面不进入最终运行面。

## 当前八路径规划范围

```text
AGENTS.md
build.md
docs/plans/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md
docs/reports/issue-38-v1.2.42-catalog-reaudit-discovery.md
docs/workflows/2026-07-25-issue-38-v1.2.42-catalog-reaudit-runtime.md
README.md
```

```text
scope_hash: sha256:c7c32b7d07f5f1b04acba9c465e1bc4bc5021228b18c438e85b40d7db5f56add
```

## 已批准实施范围

候选实施范围共 26 条：八条控制面路径、`catalog_repository.rs`、Parity README、五份 domain contract、五份 feature 目录、source-index、Dream Skin library 与 local-session-management 各两份 fixture，以及 `upstream/source-lock.json`。

```text
scope_hash: sha256:a384353e947bcb9d95b51ac5ccce49ef9558ca34580c130307a64b6d868819af
approval_status: approved
approval_ref: user-message:批准-Issue-38-二十六路径实施范围与-scope_hash-2026-07-25
```

完整路径清单位于实现计划与 Runtime Workflow；本次实际变更并集必须严格等于该二十六路径集合。

## 实施与验证证据

- RED：提交 `4206ef66076a4c9e9a19ce014a20f78cb3b73163` 中两条真实仓库测试均失败；失败原因为目录仍 stale、平台路径缺少 `OpenAI.ChatGPT-Desktop`。
- GREEN：`cargo test -p inputcodex-parity --test catalog_repository --offline -- --nocapture` 输出 `12 passed; 0 failed`。
- 目录计数保持 `133` 条来源、`36` 个 feature、`36` 份合同、`11` 个 fixture、`0` 个覆盖缺口和 `10` 个 `exception-pending`。
- Release Audit 输出 `status=current`、`requires_reaudit=false`；Repository Policy 输出 `violation_count=0`。
- 本地仅运行定向轻量 Rust；完整 Workspace 与 Windows/macOS/Linux 验证留给标准 GitHub-hosted runners。

## 剩余验收标准

1. 五域 feature、五域 contract 与 source-index 全部对齐 `v1.2.42`，不存在混合 Release。
2. 四个受影响行为域的输入、输出、副作用、错误语义和 Windows/macOS 结论可逐项复核。
3. Dream Skin runtime 与 renderer 注入保持 `exception-pending`，没有 JavaScript、WebView、CDP 注入或 UI 迁移。
4. fixture 只含合成、脱敏、可重复验证的数据。
5. `catalog_repository`、Release Audit、仓库政策、格式和空白门禁通过；`release_audit.status = current` 且 stale 字段合法清空。
6. 最终 PR Head 的 GitHub-hosted CI 全绿、全部 Review 对话解决，并取得项目所有者单独 Squash Merge 授权。

## 当前交付状态

二十六路径实施授权已经满足。下一步创建关联 Issue `#38` 的非 Draft PR，完成最终 Head Review、GitHub-hosted CI 和全部 Review 对话闭环。

```text
最终 Squash Merge 尚未授权；必须在 PR Review/CI 全部通过后由项目所有者单独批准。
```
