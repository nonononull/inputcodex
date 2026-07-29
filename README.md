# inputcodex

`inputcodex` 是一个面向 Codex 本地增强与管理场景的原生桌面项目。项目以性能、稳定性、
可诊断性和跨平台一致性为优先目标，使用 Rust 与 Iced 重新实现有效能力，不沿用上游的
Tauri/React 架构。

> [!IMPORTANT]
> 项目仍处于开发阶段，尚未发布面向普通用户的正式安装包。当前成果主要是领域、应用和
> 平台层能力，桌面展示层仍是最小骨架，不能代表完整管理界面已经完成。

## 核心原则

- 不加入广告、推广位、付费导流、广告 SDK 或隐蔽遥测。
- 性能、稳定性和可诊断性优先于功能数量。
- Windows 与 macOS 从首个正式版本起保持功能一致。
- 产品业务代码使用 Rust；禁止 TypeScript、JavaScript 业务代码和 WebView。
- Iced 只存在于展示层，领域、应用、基础设施和平台层不依赖 Iced 类型。
- 有效功能保持上游正式 Release 的行为语义；有害副作用或错误语义必须先做一致性例外决策。

## 已迁移能力

### 平台路径

只读解析 Codex 安装位置、`CODEX_HOME` 和 `inputcodex` 状态目录。无效显式路径会明确失败，
不会静默回退到当前目录、相对目录或其他自动探测结果。

### 应用概览

只读采集 Codex 安装事实、已安装版本或明确的版本未知原因、`inputcodex` 构建版本和采集时间。
历史启动记录不会冒充实时运行状态，损坏状态也不会被静默解释成“没有记录”。

### 版本与启动意图

版本只来自编译期 `CARGO_PKG_VERSION`。精确的 `--show-update` 或 `INPUTCODEX_SHOW_UPDATE=1`
只表达本次启动意图，不代表已经联网检查、下载或安装更新；非法显式环境值会明确失败。

### 运行时环境冲突观察

只读观察当前 `inputcodex` 进程继承的 `OPENAI_*` 环境变量名称和值存在状态，不返回原始值；
结果区分已观察和未观察来源，零冲突仍返回 `Ready`，且不执行删除、备份或持久化扫描。

### Relay 环境、认证配置与上下文能力观察

聚合固定代理环境来源、`.env`/Clash TUN 状态，并有界读取 `CODEX_HOME/auth.json` 与
`config.toml`；同时只读识别 MCP Server、Skill 和 Plugin。结果仅包含文档状态、凭据/配置完整性、
上下文条目 ID、稳定种类、启用状态和分类计数，不返回敏感内容、测试网络或修改配置。

### 设置与诊断只读观察

设置文件只返回顶层 JSON object 数量，合法空对象仍为 `Ready`；诊断日志仅观察固定文件尾部
最多 `256 KiB`，返回合法/损坏记录计数和截断事实。两者均不返回字段、正文、凭据或实际路径。

这些能力不打开 UI、不联网、不写入产品文件、不缓存、不启动后台线程，也不执行更新或环境清理。

## 架构

| 成员 | 职责 |
| --- | --- |
| `apps/inputcodex-desktop` | 桌面进程组装与启动入口 |
| `crates/inputcodex-domain` | 纯领域类型、状态和诊断语义 |
| `crates/inputcodex-application` | 用例、Port、加载协调和错误映射 |
| `crates/inputcodex-infrastructure` | 面向应用 Port 的基础设施适配 |
| `crates/inputcodex-platform` | Windows/macOS 平台读取与适配 |
| `crates/inputcodex-presentation` | 唯一允许直接依赖 Iced 的展示层 |
| `crates/inputcodex-parity` | 上游功能目录、行为合同和一致性验证 |

Workspace 禁止 `unsafe`，依赖方向和 Iced 边界由仓库政策验证器检查。

## 构建与验证

- Rust 工具链固定在 `rust-toolchain.toml`。
- 本地优先执行定向轻量验证；完整 Workspace、双平台编译和桌面构建由 GitHub-hosted runners 执行。
- 正确命令、环境要求和各类验证入口见 [build.md](build.md)。
- 已知问题和可复用根因见 [err.md](err.md)。

## 上游与一致性

- 功能真源：[BigPizzaV3/CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus) 最新正式 Release。
- 半成品参考：[zsr131550/CodexPlusPlus](https://github.com/zsr131550/CodexPlusPlus)，仅用于人工对照。
- `upstream/CodexPlusPlus/` 是只读审计快照，不参与产品构建或运行。
- 上游 Tauri/React 管理界面、注入脚本和远程推荐列表不得直接进入新架构。
- 客户端 Release、更新清单、签名和下载地址只能来自 `nonononull/inputcodex`。

## 文档

- [文档导航](docs/README.md)
- [项目总计划](docs/plans/PROJECT-MASTER-PLAN.md)
- [项目术语](CONTEXT.md)
- [构建与验证](build.md)
- [排错记录](err.md)
- [项目执行规则](AGENTS.md)

## 贡献

正式改动使用 `Issue → 分支 → 验证证据 → PR → Review/CI → Squash Merge`。禁止直接向
`main` 写功能、force push、删除 `main`，所有 Review 对话都必须在确定根因并完成验证后解决。

## 许可证

本项目采用 [GNU Affero General Public License v3.0](LICENSE)，Workspace 许可证标识为
`AGPL-3.0-only`。
