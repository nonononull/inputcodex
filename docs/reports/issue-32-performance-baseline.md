# Issue #32 性能基线报告

## 一、结论

Issue `#32` / PR `#49` 已建立与根七成员 Workspace 隔离的 Rust 测量工程、opt-in 首次 view 探针、PowerShell 采集/证据验证器和 GitHub-hosted Windows/macOS 专用 Workflow。初始 Performance Run `30169262247` Attempt `1` 已产生两平台真实原始样本；最终 Evidence Run `30170128309` 随后暴露 Windows Git 换行转换会让原始工作树文件哈希误报。

当前阶段结论为 `remeasure-required-before-budget-discovery`，不是预算批准。下列数值保留为初始 Run 的历史审计证据，但绑定旧 `implementation_sha256`，不得作为当前实现的最终基线；修复后必须重新 hosted 测量。禁止把 Windows 与 macOS 数值做排名、比例换算或统一阈值，也禁止由本报告直接填写性能预算。

## 二、范围与证据身份

- Issue：`https://github.com/nonononull/inputcodex/issues/32`
- PR：`https://github.com/nonononull/inputcodex/pull/49`
- 测量提交：`1974577837de97a74d7b980e8106d5e2f4a4de2e`
- 测量 tree：`0efa97728a3e8d0e8bf11a8bed75b4886cdce91a`
- Performance Run：`30169262247`，Attempt `1`
- 同提交主 CI Run：`30169262250`
- 配置哈希：`sha256:b9ed601016ececc735634aeb143965c78fbfc61819d37c7f9e584bc971642b53`
- 实现哈希：`sha256:3d69735d1d02b3cd09316e892e104a55c9e3b605035ef155861cbc3c12705d21`
- 输入哈希：`sha256:c5b507d219ff49975c13805a2a6e036ade6c61a33a184bfffb74219ce01784b5`
- Windows 结果：`benchmarks/results/issue-32/windows.json`，SHA-256 `652d913ac29453acd4ce0a00cd5a7b3ab39d47f88e4ec0146d30f72e56df4952`
- macOS 结果：`benchmarks/results/issue-32/macos.json`，SHA-256 `068165593728b81a5c8c089b09bbd6bb6c931d63d3a4f0d791d3b02e3d10a22e`
- 组合 manifest：`benchmarks/results/issue-32/manifest.json`

批准范围仍为 28 路径，`scope_hash` 为 `sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505`。根 `Cargo.toml`、根 `Cargo.lock`、`apps/`、`parity/`、`upstream/`、既有 `ci.yml`、Ruleset、Release 和 AGOS 不属于本次写入面。

## 三、环境指纹

### Windows

- Runner：GitHub-hosted，`Windows/X64`
- Image：`win25-vs2026 20260714.173.1`
- OS：`Microsoft Windows 10.0.26100`
- CPU：`AMD EPYC 7763 64-Core Processor`，`4` 个逻辑处理器
- 内存：`17178693632` bytes，约 `16.00 GiB`
- Rust：`rustc 1.97.1`，host `x86_64-pc-windows-msvc`
- Cargo：`1.97.1`
- PowerShell：`7.6.3`

### macOS

- Runner：GitHub-hosted，`macOS/ARM64`
- Image：`macos26 20260720.0258.1`
- OS：`macOS 26.4.0`
- CPU：`Apple M1 (Virtual)`，`3` 个逻辑处理器
- 内存：`7516192768` bytes，约 `7.00 GiB`
- Rust：`rustc 1.97.1`，host `aarch64-apple-darwin`
- Cargo：`1.97.1`
- PowerShell：`7.6.3`

## 四、Windows 原始样本摘要

| 对象 | 样本合同 | 结果摘要 | IQR 标记 |
| --- | --- | --- | --- |
| 桌面 Release 构建 | 单次 hosted 构建元数据 | `227.947 s`；二进制 `10263552` bytes（约 `9.788 MiB`） | 不适用 |
| 隔离基准 Release 构建 | 单次 hosted 构建元数据 | `20.505 s`；二进制 `954880` bytes（约 `0.911 MiB`） | 不适用 |
| 首次 view | `5/5` 成功新进程样本 | min/median/P95/max：`265.111 / 287.540 / 710.812 / 808.425 ms` | `1` 个 |
| 空闲 Working Set | 等待 `30 s` 后以 `1 Hz` 采样 `60` 次 | min/median/P95/max：`39.555 / 39.598 / 39.633 / 39.633 MiB` | `0` 个 |
| 空闲 CPU | 同上 | min/median/P95/max 均为 `0%` | `0` 个 |
| `application-load-complete` | `3` 次预热，`20` 样本，每样本 `100000` 次 | min/P50/P95/max：`1.541 / 1.6905 / 2.14085 / 2.195 ns/op` | `0` 个 |
| `application-cancel-stale` | `3` 次预热，`20` 样本，每样本 `100000` 次 | min/P50/P95/max：`1.541 / 1.700 / 1.90645 / 2.143 ns/op` | `0` 个 |
| `parity-repository-validation` | `3` 次预热，`20` 个热文件系统样本 | min/P50/P95/max：`20.559 / 22.093 / 32.728 / 34.321 ms/op` | `4` 个 |

首次 view 没有失败或超时尝试。三个 Rust 场景各自的 `20` 个样本 checksum 均保持单一稳定值。

## 五、macOS 原始样本摘要

| 对象 | 样本合同 | 结果摘要 | IQR 标记 |
| --- | --- | --- | --- |
| 桌面 Release 构建 | 单次 hosted 构建元数据 | `84.319 s`；二进制 `10305648` bytes（约 `9.828 MiB`） | 不适用 |
| 隔离基准 Release 构建 | 单次 hosted 构建元数据 | `8.297 s`；二进制 `1228672` bytes（约 `1.172 MiB`） | 不适用 |
| 首次 view | `5/5` 成功新进程样本 | min/median/P95/max：`150.836 / 196.978 / 1002.830 / 1203.179 ms` | `2` 个 |
| 空闲 Working Set | 等待 `30 s` 后以 `1 Hz` 采样 `60` 次 | min/median/P95/max：`83.625 / 83.625 / 83.672 / 83.672 MiB` | `4` 个 |
| 空闲 CPU | 同上 | min/median/P95/max 均为 `0%` | `0` 个 |
| `application-load-complete` | `3` 次预热，`20` 样本，每样本 `100000` 次 | min/P50/P95/max：`0.73458 / 0.755 / 1.1053395 / 1.30958 ns/op` | `2` 个 |
| `application-cancel-stale` | `3` 次预热，`20` 样本，每样本 `100000` 次 | min/P50/P95/max：`0.72417 / 0.72667 / 0.908191 / 0.93709 ns/op` | `2` 个 |
| `parity-repository-validation` | `3` 次预热，`20` 个热文件系统样本 | min/P50/P95/max：`7.167 / 7.354 / 7.711 / 8.128 ms/op` | `1` 个 |

首次 view 没有失败或超时尝试。三个 Rust 场景各自的 `20` 个样本 checksum 均保持单一稳定值。

## 六、异常值与失败处理

所有 IQR 标记样本均保留在原始 JSON 中，没有删除、平滑或重跑单个样本。Windows 首次 view 的慢样本、macOS 首次 view 的两个标记样本、macOS 空闲 Working Set 的四个标记样本，以及两平台 Rust 场景的标记样本均属于正式证据的一部分。

- Run `30168805192` 在 Workflow 装载阶段零 Job 失败。根因是 Job 级 `env` 非法使用 `runner.temp`；修复为 step 内读取 `RUNNER_TEMP`。
- Run `30168904725` 暴露微场景精度缺陷。根因是 `u128` 整数除法把亚纳秒批量结果截断为 `0`；该 Run 的测量结果整批拒绝入库，修复为 `f64` 与六位小数 CSV，并增加 `SCENARIO_PRECISION_INVALID` 证据门禁。
- Run `30169262247` 是修复后的唯一有效入库 Run；`contract`、`windows`、`macos`、`required` 四 Job 全部成功。

错误根因、处理和验证细节同时保留在 `err.md`、提交历史与 GitHub Run 中，没有通过修改预算、删除样本或运行上游/半成品来掩盖失败。

## 七、Artifact 闭环

- 有效 Windows Artifact：ID `8622503841`，名称 `performance-windows-30169262247-1`。
- 有效 macOS Artifact：ID `8622478720`，名称 `performance-macos-30169262247-1`。
- 下载文件分别匹配固定结果 SHA-256；组合 manifest 保存原始 Artifact ID、名称、Run 与 Attempt。
- 两个有效临时成功 Artifact 与无效 Run 的两个旧临时成功 Artifact 均已删除。
- Run `30168904725` 与 `30169262247` 当前 Artifact 数均为 `0`；仓库和 Workflow 均未上传整个 `target/`。

## 八、可比性限制

1. Windows 与 macOS 的 CPU 架构、虚拟化资源、镜像和总内存不同，因此跨平台结果为 `not-comparable`。
2. 每个平台当前只有一个 hosted Run；Runner 背景负载、调度和虚拟化抖动未被多日重复样本覆盖。
3. 首次 view 标记表示 Iced 首次 `view` 构建完成，不等同于用户可交互、首帧呈现或业务数据加载完成。
4. 空闲 CPU 样本全部为 `0%`，只能说明当前采样分辨率下未观察到持续 CPU 使用，不能证明进程绝对零消耗。
5. `parity-repository-validation` 明确是热文件系统场景；不得把结果解释为冷磁盘、首次克隆或上游运行性能。
6. 亚纳秒微场景来自批量迭代换算，只能在相同实现、工具链、配置与环境指纹下观察回归趋势。
7. Release 构建耗时和二进制大小是本次 Runner 元数据，不是产品预算，也不是发布资产承诺。

## 九、预算就绪性

初始证据已经证明测量对象、唯一配置、完整环境指纹、原始样本、异常标记、失败保留和双平台 hosted 执行链可工作，但最终 Evidence 的 Windows 换行根因改变了验证器 `implementation_sha256`。在新 Run 结果入库前，预算就绪性保持暂停；不得把旧样本重新标记为当前实现。

本 Issue 不给出任何绝对预算数值。PR `#49` 合并前仍必须完成最终 Head 的 Evidence 模式、主 CI、全部 Review 对话根因闭环，并取得项目所有者针对最终 Head 的单独 Squash Merge 授权；Gate 5 在此之前继续锁定。

## 十、本地 Fresh 验证

结果入库并完成本报告后，项目所有者本机只执行 `build.md` 允许的轻量、定向命令：

- 隔离 Rust 测量工程：`7/7` 测试通过。
- 展示层无默认特性：`3/3` 测试通过，未出现编译警告。
- Evidence：`ok=true`、`violation_count=0`，配置、实现与输入哈希均匹配 manifest。
- CI 合同：`CI_CONTRACT_GREEN passed=33`。
- Repository Policy：`ok=true`、`violation_count=0`。
- PowerShell 三脚本、Workflow YAML 与四个 JSON 文件解析通过。
- 实际差异并集精确等于批准的 `28` 路径，重算 `scope_hash` 仍为 `sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505`；根 Cargo 和全部禁止面零差异。
- `git diff --check` 通过。

本地没有运行完整 Workspace、Iced 桌面 Release、真实性能采集、上游/半成品或任何收费/self-hosted runner。最终 Head 的跨平台 Evidence 与主 CI 继续由公开 GitHub-hosted runner 执行。

## 十一、Windows Evidence 换行根因与重测

- 失败 Run：`30170128309`，Head `e679eee64442f0ae4db97b4e9cdbfab6780ea1de`。
- 结果：`contract` 与 `macos` 成功；`windows` 因 `WINDOWS_RESULT_HASH_INVALID`、`MACOS_RESULT_HASH_INVALID` 失败，`required` 正确阻断。
- 失败诊断 Artifact：`8622687822`，按 7 天合同保留，不删除以掩盖失败。
- 同 Head 主 CI：Run `30170128326` 七 Job 全绿，证明产品构建、Workspace 测试和仓库治理没有同步失败。
- 可重复根因：仓库没有结果 JSON 的 `eol=lf` 属性，Windows Git 配置为 `core.autocrlf=true`；fresh checkout 会把两份 LF JSON 改写为 CRLF。验证器的配置、实现和输入哈希已归一化换行，只有结果文件仍使用 `Get-FileHash` 原始工作树字节，因此同时误报两平台文件哈希。
- TDD：新增“性能 Evidence 对 Git 换行转换保持稳定”合同后得到期望 RED `2 -> 0`；生产修复改用 `Get-NormalizedTextHash` 并删除原始文本哈希入口，合同达到 `34/34` GREEN。
- 证据纪律：验证器属于 `implementation_sha256`，修复后旧结果不再代表当前实现。三份固定结果已删除以触发下一 Head 的 `measure` 模式；禁止直接改写旧 JSON/manifest 的实现哈希。
