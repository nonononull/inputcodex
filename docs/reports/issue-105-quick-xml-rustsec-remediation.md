# Issue #105 quick-xml RustSec 高危公告修复报告

## 当前状态

`LOCAL_VERIFIED_PR_PENDING`。批准的最小锁文件修复已经实施，fresh `cargo audit` 的漏洞数量
从 `2` 降为 `0`；九路径完整本地轻量验证已通过。普通 push、非 Draft PR、Review/CI、Artifact
与 Final Head 独立合并授权尚未开始，不能把当前分支状态表述为 `main` 已修复。

## 基线与批准

- Tracking Issue：<https://github.com/nonononull/inputcodex/issues/105>
- 批准与范围冻结：
  <https://github.com/nonononull/inputcodex/issues/105#issuecomment-5129028445>
- 基线：`origin/main@060ca045d2c134f8be3c9adc8cdb038842fc3243`
- Planning checkpoint：`c266cab2570ce477bc6079b951cd9e79f5abe4a0`
- Security checkpoint：`7c890e3137a503cac334e2802bd2441feae41052`
- candidate scope：`9` 路径，
  `sha256:0c90d018e06aa640d33a4c65c75aea45c89eb0e365b91fadee803b0426c8c58f`

## 输入面与修复决策

`quick-xml` 通过 `wayland-scanner` 进入 Wayland/winit/Iced 构建图，服务于依赖版本控制的
Wayland XML 协议代码生成；inputcodex 没有向用户暴露该 XML parser。当前直接输入面有限，
但两条公告均为 `7.5 high`，且存在不升级 Iced 的兼容 patch，因此不采用“仅记录不可达性”路线。

批准命令只有：

```powershell
cargo update -p wayland-scanner --precise 0.31.11
```

`wayland-scanner 0.31.10` 的 manifest 约束 `quick-xml ^0.39`；候选 `0.31.11` 将其提高为
`^0.41`。这解释了为什么必须先更新最小上游 package，而不能直接强锁叶子版本。

## Security RED

首次 direct fetch 在 advisory 分析前因网络失败，不能作为 RED。确认本机代理 TCP 可达后，只在
当前 shell 设置代理并重跑 fresh audit，得到真实证据：

- advisory database：`1173` 条；
- 锁定依赖：`350`；
- 退出码：`1`；
- `RUSTSEC-2026-0194`：`quick-xml 0.39.4`，`7.5 high`；
- `RUSTSEC-2026-0195`：`quick-xml 0.39.4`，`7.5 high`；
- 修复要求：两条均为 `>=0.41.0`；
- informational warning：`paste 1.0.15` 与 `ttf-parser 0.25.1`，单独记录。

## 最小锁文件差异

`Cargo.lock` 的 numstat 精确为 `4` 行新增、`4` 行删除：

| package | 基线 | 目标 | 新 checksum |
| --- | --- | --- | --- |
| `quick-xml` | `0.39.4` | `0.41.0` | `e660451e55124f798a69a5af3f49ccfbefbd41910eefd25caf2393e1f3473ec1` |
| `wayland-scanner` | `0.31.10` | `0.31.11` | `338e30461b3a2b67d70eb30a6d89f8e0c93a833e07d2ae89085cd070c4a00ac0` |

两个 package 的 `source` 均继续为 crates.io registry，dependency 数组没有变化；不存在第三个
package、manifest、Iced/winit/smithay 或产品源码变化。

## Security GREEN

精确更新后 fresh `cargo audit --json` 得到：

```text
CARGO_AUDIT_EXIT=0
DEPENDENCIES=350
VULNERABILITIES=0
UNMAINTAINED_WARNINGS=2
WARNING=RUSTSEC-2024-0436 crate=paste version=1.0.15
WARNING=RUSTSEC-2026-0192 crate=ttf-parser version=0.25.1
SECURITY_GREEN
```

目标两条 high 漏洞已经从当前分支锁文件解析结果中消失。两条 warning 没有被降格、隐藏或宣称
已修复；它们需要不同的上游依赖评估，不进入本任务实现范围。

## 许可证与工具链

通过精确版本 `cargo info` 与 registry manifest 复核：

- `wayland-scanner 0.31.11`：MIT，MSRV `1.71`；
- `quick-xml 0.41.0`：MIT，MSRV `1.79`；
- 仓库工具链：Rust `1.97.1`；
- 无许可证或 MSRV 阻塞。

## AGOS 边界

Planning checkpoint 后的干净 Git snapshot 返回 `GIT_SNAPSHOT_READY`。AGOS default-entry
`-ReportOnly` 将本任务识别为 `unregistered`，并因缺它自己的 owner-scope manifest、mutation
admission 与 unknown requested ops 返回 `blocked/needs-input`。inputcodex 项目规则明确要求此类
外部状态记录后绕过；本分支没有修改 AGOS Registry、Workflow、Vault、规则或脚本。

## 范围确认

最终范围固定为：

```text
AGENTS.md
build.md
Cargo.lock
docs/plans/2026-07-30-issue-105-quick-xml-rustsec-remediation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-30-issue-105-quick-xml-rustsec-remediation.md
docs/reports/issue-105-quick-xml-rustsec-remediation.md
docs/workflows/2026-07-30-issue-105-quick-xml-rustsec-remediation-runtime.md
err.md
```

本任务没有修改 README、`Cargo.toml`、Rust 源码、UI、CI、Ruleset、Runner、性能预算、
`upstream/` 或 AGOS，也没有开始新的 Gate 5 产品切片。

## 本地验证证据

本机时间 `2026-07-30 17:49:18 +08:00` 开始执行 `build.md` Issue #105 完整命令链：

- 当前分支精确为 `codex/issue-105-quick-xml-rustsec-remediation`；
- Cargo metadata 识别七成员 Workspace；
- 两个 package 的版本、source 与 checksum 精确匹配；
- `Cargo.lock` 相对 `origin/main` 为 `4/4`，实际变化行与批准集合完全一致；
- `cargo info` 复核 MIT 与 MSRV `1.71/1.79`；
- fresh audit 输出 `vulnerabilities=0`、两条分离的 unmaintained warning；
- `cargo fmt --all -- --check` 通过；
- CI contract 输出 `CI_CONTRACT_GREEN passed=35`；
- Repository Policy 输出 `ok=true`、`violation_count=0`；
- Release Audit 输出 `status=current`、`requires_reaudit=false`；
- 实际九路径与
  `sha256:0c90d018e06aa640d33a4c65c75aea45c89eb0e365b91fadee803b0426c8c58f`
  精确一致；
- `git diff --check origin/main` 通过；
- 最终标记为 `ISSUE_105_LOCAL_GREEN ... vulnerabilities=0`。

## 剩余门禁

1. 建立 local-verified checkpoint；
2. 普通 push 与关联 Issue #105 的非 Draft PR；
3. Review conversations 根因闭环；
4. Final Head Hosted CI、Performance Baseline 与 Artifact 核验；
5. 项目所有者绑定 Final Head 的独立 Squash Merge 授权。
