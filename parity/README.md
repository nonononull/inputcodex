# inputcodex 一致性事实层

## 作用

`parity/` 保存上游正式 Release `v1.2.43` 的静态审计事实，不参与桌面产品运行，也不复用 Tauri、React、WebView 或注入脚本。

## 五个领域

- `foundation-platform`：应用检测与生命周期、设置、路径、Watcher、环境冲突和诊断。
- `provider-network`：供应商、Relay、模型目录、协议代理、路由和网络诊断。
- `session-data`：本地会话、Markdown、Token 历史、Provider metadata、索引清理与备份。
- `plugin-script`：用户脚本、插件、主题、Stepwise 和界面增强的审计输入。
- `remote-install`：入口安装、应用更新、Zed Remote 与 Upstream worktree。

## source-index 边界

- 锁定来源：`BigPizzaV3/CodexPlusPlus` Release `v1.2.43`，tag commit `5036ff056b5c629f19356396b17d6eeb70da664c`。
- 当前机器验证范围：84 个 Tauri command、45 个 `codex-plus-core` 公开模块、4 个 `codex-plus-data` 公开模块，共 133 个入口。
- 每个入口映射到稳定 feature、显式排除项或 `exception-pending`；当前显式排除 3 个旧适配入口。
- 当前共有 43 个 feature，其中 10 个为 `exception-pending`。
- 这 133 条覆盖只证明上述三类公开入口已枚举，不等于所有私有函数、React 交互或隐式副作用已经完成审计。

## `v1.2.43` 定向复审结论

- `feature.foundation-platform.platform-paths` 纳入 Windows 包身份 `OpenAI.ChatGPT-Desktop`，macOS 保持等价原生路径语义。
- Issue `#75` 已将 `feature.foundation-platform.platform-paths` 固定为 `implemented`：未安装受支持 Codex 时返回 `Ready + installation=None`，不使用 `Empty`；三个来源入口只增加 `process-read` 环境读取，不增加写入、网络、广告或远程推荐副作用。
- Issue `#78` 已将 `feature.foundation-platform.application-overview` 固定为只读事实切片：结果只能是 `Ready(Installed Known)`、`Ready(Installed Unknown)` 或 `Ready(NotInstalled)`，实时进程状态固定为 `LiveProcessState::NotObserved`；历史状态、写入、网络、广告与远程推荐均不进入该能力。
- Issue `#81` 已将 `feature.foundation-platform.version-and-startup` 固定为无副作用进程输入切片：版本只来自 `CARGO_PKG_VERSION`，精确 `--show-update` 或 `INPUTCODEX_SHOW_UPDATE=1` 产生 `StartupIntent::ShowUpdate`，其他合法情况为 `StartupIntent::Default`，非法显式环境值返回 `INVALID_STARTUP_OPTION`。
- Issue `#86` 已将 `feature.foundation-platform.runtime-environment-conflict-observation` 固定为只读运行时环境切片：零冲突返回 `Ready(empty)`；来源覆盖固定为 `runtime_process: Observed`、`persistent_user: NotObserved` 与 `persistent_system: NotObserved`；名称无法无损表示时返回 `RUNTIME_ENVIRONMENT_NAME_UNREPRESENTABLE`，删除、备份和持久化来源扫描仍归未评估的环境冲突总功能。
- Issue `#89` 已将 `feature.provider-network.relay-environment-observation` 固定为 Relay 环境只读观察切片：双平台只返回五个规范代理名称与来源，不返回配置值或路径；Windows 注册表单来源失败表示 `persistent_user: Unavailable` 或 `persistent_system: Unavailable`，macOS 固定 `persistent_user: NotObserved` 与 `persistent_system: NotObserved`；Codex `.env` 只查元数据，Clash Verge 单候选最多读取 `64 KiB`，无风险仍返回 `Ready`。
- Issue `#92` 已将 `feature.foundation-platform.settings-observation` 固定为设置只读观察切片：只接管 `tauri-command:load_settings`，文件缺失返回 `Empty(NotConfigured)`，合法 JSON object 只返回顶层条目数量且 `{}` 仍为 `Ready(0)`；单文件上限为 `256 KiB`，原设置管理总功能继续 `unassessed`。
- Issue `#95` 已将 `feature.foundation-platform.diagnostic-log-observation` 固定为诊断日志只读结构观察切片：只接管 `tauri-command:read_latest_logs`，最多读取固定文件尾部 `256 KiB`，只返回文件大小、合法 JSON object、malformed 与截断事实；日志正文、实际路径和其余诊断副作用不进入该能力，原诊断总功能继续 `unassessed`。
- Issue `#98` 已将 `feature.provider-network.relay-status-observation` 固定为 Relay 认证与配置状态只读观察切片：只接管 `tauri-command:relay_status`，固定观察 `CODEX_HOME/auth.json` 与 `config.toml`，单文件最多读取 `256 KiB`；结果只包含文档状态、凭据存在事实和 Relay 配置完整性，原 Relay 配置管理总功能继续 `unassessed`。
- Issue `#101` 已将 `feature.provider-network.context-entry-observation` 固定为上下文能力只读目录观察切片：只接管 `tauri-command:read_live_context_entries`，有界解析固定 `CODEX_HOME/config.toml`，只返回条目 ID、`McpServer / Skill / Plugin`、启用状态和分类计数；原上下文管理总功能与五个写入/完整管理入口继续 `unassessed`。
- Issue `#104` 已将 `feature.session-data.local-session-directory-observation` 固定为本地会话目录只读观察切片：只接管 `tauri-command:list_local_sessions`，最多观察 32 个受控 SQLite 候选，跨库排序去重后按默认 50、最大 100 分页；原删除、备份、恢复和 grouped undo 总功能继续 `unassessed`。
- `feature.session-data.local-session-management` 纳入合法 `CODEX_SQLITE_HOME`、多数据库删除、`grouped-undo-token`、全库恢复预检、允许路径保护和撤销窗口。
- `feature.plugin-script.dream-skin-library` 只纳入受限本地 companion data URL 与布局配置；fixture 使用合成图片数据。
- `feature.plugin-script.dream-skin-runtime` 与 `feature.plugin-script.renderer-enhancements` 的 companion 显示仍依赖 renderer 注入，继续保持 `exception-pending`，不得进入 Rust/Iced 运行面。
- `feature.foundation-platform.watcher` 纳入 macOS 按 `remote-debugging-port` 精确筛选主进程、等待退出和超时留证；上游 Windows 粗粒度停止差异不得成为 `inputcodex` 产品差异。
- `feature.session-data.provider-metadata-maintenance` 纳入从 `threads` 修复 `local_thread_catalog`、`sqliteCatalogRowsInserted` 计数与同步状态水位。
- `feature.plugin-script.user-scripts` 的 runtime status、`status` 与 `error` 仍来自 renderer 注入链，继续保持 `exception-pending`。
- `feature.foundation-platform.advertising` 的 sponsor、`expires_at` 与本地素材只作为审计输入，继续受无广告硬规则阻断。

## 初始状态

- 正常能力首次登记为 `unassessed`，不宣称已经实现或验证。
- 广告、远程推荐列表、renderer 注入、用户脚本注入及依赖注入的增强能力登记为 `exception-pending`。
- `apps/codex-plus-mobile-relay` 不属于该 Release 的 Workspace 与 README 正式结构，不计入 133 条入口；保留为 `not-part-of-release-workspace` 审计备注。

## 显式排除

- `tauri-command:open_external_url`：通用 Tauri 外链适配，不是独立产品能力。
- `core-module:launcher`：把进程生命周期与 renderer 注入耦合；生命周期能力已由公开命令登记。
- `core-module:routes`：旧 renderer bridge 路由器；有效数据能力已由命令和 data 模块登记。

## 后续规则

- 行为合同按同名五域文件保存于 `parity/contracts/`。
- 夹具仅允许合成或不可逆脱敏数据，保存于 `parity/fixtures/<feature-id>/`。
- 当前五域共保存 `43` 份行为合同，并为 `12` 个需要结构数据的 feature 保存 `12` 个 fixture manifest；其余场景以合同中的 `fixture_policy: none` 说明无需 fixture。
- 合同与 fixture 必须由 `inputcodex-parity` 完整仓库验证共同检查 domain、稳定 ID、引用、目录归属、路径安全、敏感 payload 和文本控制字节。
- `exception-pending` 只有在独立一致性例外 Issue 获得项目所有者决定后才能改变状态。
