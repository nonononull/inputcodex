# Issue #115：`v1.2.44` 功能目录重新审计 Discovery 报告

## 结论

`v1.2.44 @ 77091ccaee4423f35a1b2c51c4ecd703e6201092` 相对 `v1.2.43` 有 `35` 个提交、`42` 个变化文件和 `+3161/-445`。公开入口由 `133` 增至 `135`，唯一枚举缺口为：

- `core-module:sub2api`
- `tauri-command:fetch_sub2api_billing`

其余变化均归入既有 feature 或既有例外。重审后目标为 `135` source、`44` feature、`44` contract、`13` fixture manifest、`10` exception、`3` excluded、`0` gap。

## 来源基线

- 快照合并：`057c7e08de5c1e30198881a7f548a49453fb1ac6`
- Release tree：`d417f2775adfe61798c6676024edce1a236e576f`
- 完整快照：`281` blob、`70` 目录、`0` submodule、`24,481,110` blob bytes
- 归档：`18,987,824` bytes，SHA-256 `2c9a1900b24e838ed7b9405534be15efc81a670636cd97d4de8a16cab17a73cb`
- Release delta：`41 modified / 1 added / 0 deleted`
- 同步报告：`docs/reports/2026-07-31-upstream-v1.2.44-sync.md`

## Sub2API 归类

| 方案 | 结论 |
| --- | --- |
| 独立账单倍率观察 | 采用。具有独立网络、凭据、schema、倍率、超时和错误边界。 |
| 并入 Provider diagnostics | 拒绝。账单倍率不是连接测试或 Provider Doctor。 |
| 并入 Relay profile management | 拒绝。读取本身不写配置，会污染既有文件副作用合同。 |

新 feature 只登记上游事实并保持 `unassessed`。合同不接受独立任意 URL，不返回或持久化 API key、账号、Header、原始响应或实际端点。

## 行为变化

| 上游变化 | 目录归属 | 处理 |
| --- | --- | --- |
| URL provider import sanitization | provider-import | pending config/auth 不落盘，URL 凭据脱敏。 |
| provider credentials/common config isolation | relay-profile-management | provider 字段留在 profile，不进入 common config。 |
| Web Search / Responses Lite | model-catalog | capability 由 provider wire 语义决定。 |
| Responses compact / zstd / WebSocket | protocol-proxy | 保留 compact 路径，有界解码，unsupported upgrade 明确失败。 |
| mixed relay 与 switching disabled | provider-configuration-application | Remote Control 仍维持所需代理。 |
| reserved ports / existing CDP / reinjection | application-lifecycle | 复用经验证 CDP，data bridge 不因重注入丢失。 |
| provider sync honors CODEX_HOME | provider-metadata-maintenance | 默认路径使用平台 CODEX_HOME 语义。 |
| remote plugin auth fallback | plugin-marketplace | 本地 fallback 可审计，远程推荐/注入仍为例外。 |
| Quick Chat / app-server / Fast / model whitelist | renderer-enhancements | 继续 renderer 注入例外。 |
| companion rendered height | dream-skin-runtime | 继续 renderer/DOM/UI 例外。 |
| sponsor 文案与素材哈希 | advertising | 继续受无广告硬规则阻断。 |

## 排除与保留

- Tauri/React、样式、UI 和注入脚本不进入 inputcodex 运行面。
- 广告/赞助、远程推荐与插件注入不改变 `exception-pending`。
- `core-module:launcher` 与 `core-module:routes` 继续显式 excluded；`tauri-command:open_external_url` 继续 excluded。
- 上游 Cargo/npm/README、图片和资产哈希变化不形成新的产品能力。

## 证据质量

三个独立只读 Agent 在输出最终报告前均被本机 AI proxy `HTTP 503` 中断。活动日志已经确认两条枚举缺口和主要行为分类，但不作为独立 Review 结论；主线程重新读取 42 文件 diff、35 条 commit 标题、目录/合同/fixture 和 live GitHub 证据后形成本文。

## 冻结范围

- 路径数：`32`
- Ordinal hash：`sha256:b8d42285dc7cfca080f9fbf683c9c8176a0faae633c5971b1827837059898b83`
- Planning Freeze：`https://github.com/nonononull/inputcodex/issues/115#issuecomment-5150369262`
- 明确不含：README、err、产品 Rust/Cargo、Workflow、Ruleset、上游快照、UI、Release 与 AGOS。
