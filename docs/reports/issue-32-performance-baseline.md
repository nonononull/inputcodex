# Issue #32 性能基线报告

## 一、结论

Issue `#32` / PR `#49` 已建立与根七成员 Workspace 隔离的 Rust 测量工程、opt-in 首次 view 探针、PowerShell 采集/证据验证器和 GitHub-hosted Windows/macOS 专用 Workflow。最终 Evidence Run `30170128309` 暴露 Windows Git 换行转换误报后，修复提交 `42bc2e9ce7cf2e88d0602ebdc638213854793f96` 已通过 Performance Run `30170535534` 重新采集两平台真实原始样本。

本报告结论恢复为 `ready-for-separate-budget-discovery`，不是预算批准。下列数值绑定当前实现哈希，可作为同平台、同环境指纹下的后续趋势输入；禁止把 Windows 与 macOS 数值做排名、比例换算或统一阈值，也禁止由本报告直接填写性能预算。

## 二、范围与证据身份

- Issue：`https://github.com/nonononull/inputcodex/issues/32`
- PR：`https://github.com/nonononull/inputcodex/pull/49`
- 测量提交：`42bc2e9ce7cf2e88d0602ebdc638213854793f96`
- 测量 tree：`94fc484124d1557ece1d76f27abc5ea1bc5ea592`
- Performance Run：`30170535534`，Attempt `1`
- 同提交主 CI Run：`30170535538`
- 配置哈希：`sha256:b9ed601016ececc735634aeb143965c78fbfc61819d37c7f9e584bc971642b53`
- 实现哈希：`sha256:c20d5299bed4dc14af1b8b7257b206b8a7e7a02a02835265183bf4479468da6d`
- 输入哈希：`sha256:c5b507d219ff49975c13805a2a6e036ade6c61a33a184bfffb74219ce01784b5`
- Windows 结果：`benchmarks/results/issue-32/windows.json`，SHA-256 `c8461e942da9e49ad6a62783cad2bd88076546779f22252bafe74d4af346b55a`
- macOS 结果：`benchmarks/results/issue-32/macos.json`，SHA-256 `1abd6bd0181afae9f7381049902cc35b21bbca9bd77533324c7c469ca16d4829`
- 组合 manifest：`benchmarks/results/issue-32/manifest.json`

批准范围仍为 28 路径，`scope_hash` 为 `sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505`。根 `Cargo.toml`、根 `Cargo.lock`、`apps/`、`parity/`、`upstream/`、既有 `ci.yml`、Ruleset、Release 和 AGOS 不属于本次写入面。

## 三、环境指纹

### Windows

- Runner：GitHub-hosted，`Windows/X64`
- Image：`win25-vs2026 20260714.173.1`
- OS：`Microsoft Windows 10.0.26100`
- CPU：`AMD EPYC 7763 64-Core Processor`，`4` 个逻辑处理器
- 内存：`17174360064` bytes，约 `16.00 GiB`
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
| 桌面 Release 构建 | 单次 hosted 构建元数据 | `228.118 s`；二进制 `10263552` bytes（约 `9.788 MiB`） | 不适用 |
| 隔离基准 Release 构建 | 单次 hosted 构建元数据 | `20.789 s`；二进制 `954880` bytes（约 `0.911 MiB`） | 不适用 |
| 首次 view | `5/5` 成功新进程样本 | min/median/P95/max：`264.008 / 281.025 / 586.259 / 656.112 ms` | `1` 个 |
| 空闲 Working Set | 等待 `30 s` 后以 `1 Hz` 采样 `60` 次 | min/median/P95/max：`39.516 / 39.555 / 39.594 / 39.594 MiB` | `0` 个 |
| 空闲 CPU | 同上 | min/median/P95/max 均为 `0%` | `0` 个 |
| `application-load-complete` | `3` 次预热，`20` 样本，每样本 `100000` 次 | min/P50/P95/max：`1.542 / 1.850 / 2.07915 / 2.196 ns/op` | `0` 个 |
| `application-cancel-stale` | `3` 次预热，`20` 样本，每样本 `100000` 次 | min/P50/P95/max：`1.541 / 1.8295 / 1.9198 / 1.935 ns/op` | `0` 个 |
| `parity-repository-validation` | `3` 次预热，`20` 个热文件系统样本 | min/P50/P95/max：`21.171 / 22.345 / 31.696 / 38.977 ms/op` | `1` 个 |

首次 view 没有失败或超时尝试。三个 Rust 场景各自的 `20` 个样本 checksum 均保持单一稳定值。

## 五、macOS 原始样本摘要

| 对象 | 样本合同 | 结果摘要 | IQR 标记 |
| --- | --- | --- | --- |
| 桌面 Release 构建 | 单次 hosted 构建元数据 | `142.604 s`；二进制 `10305648` bytes（约 `9.828 MiB`） | 不适用 |
| 隔离基准 Release 构建 | 单次 hosted 构建元数据 | `11.345 s`；二进制 `1228672` bytes（约 `1.172 MiB`） | 不适用 |
| 首次 view | `5/5` 成功新进程样本 | min/median/P95/max：`287.639 / 349.474 / 1394.224 / 1644.147 ms` | `1` 个 |
| 空闲 Working Set | 等待 `30 s` 后以 `1 Hz` 采样 `60` 次 | min/median/P95/max：`82.266 / 82.266 / 82.359 / 82.359 MiB` | `10` 个 |
| 空闲 CPU | 同上 | min/median/P95/max 均为 `0%` | `0` 个 |
| `application-load-complete` | `3` 次预热，`20` 样本，每样本 `100000` 次 | min/P50/P95/max：`0.73375 / 0.943125 / 1.993441 / 2.02709 ns/op` | `0` 个 |
| `application-cancel-stale` | `3` 次预热，`20` 样本，每样本 `100000` 次 | min/P50/P95/max：`0.7225 / 0.79229 / 0.9269855 / 0.9725 ns/op` | `0` 个 |
| `parity-repository-validation` | `3` 次预热，`20` 个热文件系统样本 | min/P50/P95/max：`7.498 / 8.391 / 14.223 / 14.561 ms/op` | `4` 个 |

首次 view 没有失败或超时尝试。三个 Rust 场景各自的 `20` 个样本 checksum 均保持单一稳定值。

## 六、异常值与失败处理

所有 IQR 标记样本均保留在原始 JSON 中，没有删除、平滑或重跑单个样本。Windows 首次 view 与 parity 场景各有一个标记样本；macOS 首次 view、空闲 Working Set、parity 场景分别有 `1`、`10`、`4` 个标记样本；两个应用微场景没有 IQR 标记。它们均属于正式证据的一部分。

- Run `30168805192` 在 Workflow 装载阶段零 Job 失败。根因是 Job 级 `env` 非法使用 `runner.temp`；修复为 step 内读取 `RUNNER_TEMP`。
- Run `30168904725` 暴露微场景精度缺陷。根因是 `u128` 整数除法把亚纳秒批量结果截断为 `0`；该 Run 的测量结果整批拒绝入库，修复为 `f64` 与六位小数 CSV，并增加 `SCENARIO_PRECISION_INVALID` 证据门禁。
- Run `30169262247` 是首次完整入库 Run，但后续验证器修复改变了 `implementation_sha256`，因此只保留为历史审计证据。
- Run `30170128309` 在 Windows Evidence 暴露 `core.autocrlf` 原始哈希误报；失败诊断 Artifact `8622687822` 按 7 天合同保留。
- Run `30170535534` 是当前实现的有效入库 Run；`contract`、`windows`、`macos`、`required` 四 Job 全部成功。

错误根因、处理和验证细节同时保留在 `err.md`、提交历史与 GitHub Run 中，没有通过修改预算、删除样本或运行上游/半成品来掩盖失败。

## 七、Artifact 闭环

- 有效 Windows Artifact：ID `8622843440`，名称 `performance-windows-30170535534-1`。
- 有效 macOS Artifact：ID `8622830369`，名称 `performance-macos-30170535534-1`。
- 下载文件分别匹配固定结果 SHA-256；组合 manifest 保存原始 Artifact ID、名称、Run 与 Attempt。
- 两个当前有效临时成功 Artifact 与此前四个旧临时成功 Artifact 均已删除。
- Run `30170535534` 当前 Artifact 数为 `0`；失败 Run `30170128309` 仅保留诊断 Artifact `8622687822`，仓库和 Workflow 均未上传整个 `target/`。

## 八、可比性限制

1. Windows 与 macOS 的 CPU 架构、虚拟化资源、镜像和总内存不同，因此跨平台结果为 `not-comparable`。
2. 每个平台当前只有一个 hosted Run；Runner 背景负载、调度和虚拟化抖动未被多日重复样本覆盖。
3. 首次 view 标记表示 Iced 首次 `view` 构建完成，不等同于用户可交互、首帧呈现或业务数据加载完成。
4. 空闲 CPU 样本全部为 `0%`，只能说明当前采样分辨率下未观察到持续 CPU 使用，不能证明进程绝对零消耗。
5. `parity-repository-validation` 明确是热文件系统场景；不得把结果解释为冷磁盘、首次克隆或上游运行性能。
6. 亚纳秒微场景来自批量迭代换算，只能在相同实现、工具链、配置与环境指纹下观察回归趋势。
7. Release 构建耗时和二进制大小是本次 Runner 元数据，不是产品预算，也不是发布资产承诺。

## 九、预算就绪性

当前证据已具备独立预算 Discovery 所需的最小输入：稳定测量对象、唯一配置、完整环境指纹、原始样本、异常标记、失败保留、当前实现/输入哈希和双平台 hosted 执行链。下一独立性能预算 Issue 可基于这些输入决定同平台重复次数、可比 Runner 类别、统计窗口、回归阈值和所有者批准方式。

本 Issue 不给出任何绝对预算数值。PR `#49` 合并前仍必须完成最终 Head 的 Evidence 模式、主 CI、全部 Review 对话根因闭环，并取得项目所有者针对最终 Head 的单独 Squash Merge 授权；Gate 5 在此之前继续锁定。

## 十、本地 Fresh 验证

结果入库并完成本报告后，项目所有者本机只执行 `build.md` 允许的轻量、定向命令：

- 隔离 Rust 测量工程：`7/7` 测试通过。
- 展示层无默认特性：`3/3` 测试通过，未出现编译警告。
- Evidence：`ok=true`、`violation_count=0`，配置、实现与输入哈希均匹配 manifest。
- CI 合同：`CI_CONTRACT_GREEN passed=34`。
- Repository Policy：`ok=true`、`violation_count=0`。
- PowerShell 三脚本、Workflow YAML 与四个 JSON 文件解析通过。
- 实际差异并集精确等于批准的 `28` 路径，重算 `scope_hash` 仍为 `sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505`；根 Cargo 和全部禁止面零差异。
- `git diff --check` 通过。

本地没有运行完整 Workspace、Iced 桌面 Release、真实性能采集、上游/半成品或任何收费/self-hosted runner。最终 Head 的跨平台 Evidence 与主 CI 继续由公开 GitHub-hosted runner 执行。

## 十一、Windows Evidence 换行根因与重测闭环

- 失败 Run：`30170128309`，Head `e679eee64442f0ae4db97b4e9cdbfab6780ea1de`。
- 结果：`contract` 与 `macos` 成功；`windows` 因 `WINDOWS_RESULT_HASH_INVALID`、`MACOS_RESULT_HASH_INVALID` 失败，`required` 正确阻断。
- 失败诊断 Artifact：`8622687822`，按 7 天合同保留，不删除以掩盖失败。
- 同 Head 主 CI：Run `30170128326` 七 Job 全绿，证明产品构建、Workspace 测试和仓库治理没有同步失败。
- 可重复根因：仓库没有结果 JSON 的 `eol=lf` 属性，Windows Git 配置为 `core.autocrlf=true`；fresh checkout 会把两份 LF JSON 改写为 CRLF。验证器的配置、实现和输入哈希已归一化换行，只有结果文件仍使用 `Get-FileHash` 原始工作树字节，因此同时误报两平台文件哈希。
- TDD：新增“性能 Evidence 对 Git 换行转换保持稳定”合同后得到期望 RED `2 -> 0`；生产修复改用 `Get-NormalizedTextHash` 并删除原始文本哈希入口，合同达到 `34/34` GREEN。
- 提交级回归：结果提交 `151011de62c36cf6b9af1bbdc81c9b7a7422abfc` 在 fresh `core.autocrlf=true` 检出后，Windows/macOS 工作树原始哈希分别变为 `sha256:aa2c995cc369aa2f382dcb4efe22188ea08c5f3617a720b5e4f3a23e6d5384d3`、`sha256:7db611791054a4db3aee74420de1145cce2ff649d6ebb486c6ad7f3f62244f8e`，但 Evidence 仍零违规；`core.autocrlf=false` 保持 manifest 原始哈希并同样通过。
- 证据纪律：验证器属于 `implementation_sha256`，修复后旧结果没有被改写元数据。三份固定结果删除后由提交 `42bc2e9ce7cf2e88d0602ebdc638213854793f96` 触发 Run `30170535534` 重新 measure；四 Job 全绿，新结果本地 Evidence 零违规，两个成功 Artifact 已删除且 Run Artifact 数为 `0`。
