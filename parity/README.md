# inputcodex 一致性事实层

## 作用

`parity/` 保存上游正式 Release `v1.2.41` 的静态审计事实，不参与桌面产品运行，也不复用 Tauri、React、WebView 或注入脚本。

## 五个领域

- `foundation-platform`：应用检测与生命周期、设置、路径、Watcher、环境冲突和诊断。
- `provider-network`：供应商、Relay、模型目录、协议代理、路由和网络诊断。
- `session-data`：本地会话、Markdown、Token 历史、Provider metadata、索引清理与备份。
- `plugin-script`：用户脚本、插件、主题、Stepwise 和界面增强的审计输入。
- `remote-install`：入口安装、应用更新、Zed Remote 与 Upstream worktree。

## source-index 边界

- 锁定来源：`BigPizzaV3/CodexPlusPlus` Release `v1.2.41`，tag commit `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 当前机器验证范围：84 个 Tauri command、45 个 `codex-plus-core` 公开模块、4 个 `codex-plus-data` 公开模块，共 133 个入口。
- 每个入口映射到稳定 feature、显式排除项或 xception-pending；当前显式排除 3 个旧适配入口。
- 当前共有 36 个 feature，其中 10 个为 xception-pending。
- 这 133 条覆盖只证明上述三类公开入口已枚举，不等于所有私有函数、React 交互或隐式副作用已经完成审计。

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
- `exception-pending` 只有在独立一致性例外 Issue 获得项目所有者决定后才能改变状态。
