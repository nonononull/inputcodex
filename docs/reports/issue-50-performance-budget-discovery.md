# Issue #50：性能预算 Discovery 报告

## 任务元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/50`
- 分支：`codex/issue-50-performance-budget-discovery`
- 基线提交：`fd9db9ca1c150b7db34dda8acc09b6f0cc357a17`
- 规划提交：`414047d5bacd34e5c6f3abf63cb1b4813ae9cc3a`
- 批准决策：`user-message:按A方案开始-2026-07-25`
- 九路径批准证据：`https://github.com/nonononull/inputcodex/issues/50#issuecomment-5080410894`
- `scope_hash`：`sha256:af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f`

## 一、结论

Issue `#32` / PR `#49` 已证明 `inputcodex` 能在 GitHub-hosted Windows 与 macOS 上产生结构化、绑定实现和环境指纹的性能证据，但当前每个平台只有一次独立有效 Run，因此状态是 `baseline-only`，不是预算已批准。

本 Discovery 采用“同平台可比队列、至少五次独立 Run、run-level 稳健统计、先观察后阻断”的方案。具体预算数值、预算验证器和 Gate 5 产品迁移继续使用不同 Issue/PR。

## 二、现有证据

- 有效测量提交：`42bc2e9ce7cf2e88d0602ebdc638213854793f96`。
- 有效测量 tree：`94fc484124d1557ece1d76f27abc5ea1bc5ea592`。
- Performance Run：`30170535534` Attempt `1`。
- 配置哈希：`sha256:b9ed601016ececc735634aeb143965c78fbfc61819d37c7f9e584bc971642b53`。
- 实现哈希：`sha256:c20d5299bed4dc14af1b8b7257b206b8a7e7a02a02835265183bf4479468da6d`。
- 输入哈希：`sha256:c5b507d219ff49975c13805a2a6e036ade6c61a33a184bfffb74219ce01784b5`。
- Windows/macOS 固定结果、manifest 和失败诊断均已按 Issue `#32` 报告保留；成功 Run Artifact 数为 `0`。

现有对象包括：

- 桌面与隔离基准 Release 构建耗时和二进制大小。
- 首次 view 的五个成功新进程样本。
- 等待三十秒后的六十个空闲 Working Set 与 CPU 样本。
- 三个 Rust 场景各二十个样本及稳定 checksum。

这些样本足以定义方法，但不能替代多个独立 Runner 执行形成的跨 Run 分布。

## 三、可比环境决定

预算比较必须先匹配结果 schema、平台、架构、Runner 类型、镜像系列、Rust release/host、配置哈希、输入哈希和样本合同。镜像精确版本、OS、处理器、逻辑处理器数量和内存档位组成队列指纹。

若队列指纹变化：

- 结果仍可通过 Evidence，并记录为 `new-cohort-valid`。
- 不与旧队列预算做阻断比较。
- 新队列重新收集至少五次独立 Run。
- 禁止为了延续旧预算而删除环境字段或放宽证据哈希。

## 四、指标分类

| 类别 | 指标 | Discovery 决定 |
| --- | --- | --- |
| 阻断候选 | 首次 view median/P95 | 五次可比 Run 后提出独立 warning/blocking 数值 |
| 阻断候选 | 空闲 Working Set median/P95 | 同平台独立预算，不跨平台换算 |
| 阻断候选 | 三个 Rust 场景 median/P95 | checksum 必须稳定，保留所有样本 |
| 观察 | 空闲 CPU median/P95 | 精度与非零变化稳定前不进入 required 阻断 |
| 观察 | 桌面二进制大小 | 可报告趋势，升级阻断需新决策 |
| 诊断 | 桌面与隔离基准构建耗时 | 标准共享 Runner 上不作为强制预算 |
| 诊断 | 隔离基准二进制大小 | 用于工具链和依赖变化解释 |

## 五、统计与异常处理

- 每个平台、每个队列至少收集五次不同 `run_id` 的独立成功 Run。
- 每个指标使用 run-level median 和 P95；跨 Run 分别取这些统计量的中位数作为参考中心和尾部。
- 同时保存最小值、最大值和 MAD，供后续所有者批准安全裕量。
- IQR 标记全部保留，不自动删除，也不自动使 Run 无效。
- 样本合同、checksum、schema、哈希或成功状态失配时，整次 Run 为 `evidence-invalid`。
- GitHub 外部事故必须有服务状态、同一 Run attempt 或其他可复核证据，不能仅凭结果慢或 Job 失败推定。

## 六、方案比较

### 方案 A：先 Discovery，再数值批准，再实施 CI

采用。优点是预算方法、数值和执行器分离，每一步都有独立证据和回滚面；Gate 5 解锁条件可检查，不会把单次基线误写成 required budget。

### 方案 B：在同一 PR 中填写预算并改 CI

拒绝。它混合方法、数值、验证器和 required 行为，任何误报都难以确定根因，也超出 Issue `#50` 九路径。

### 方案 C：Gate 5 与预算工作并行

拒绝。首批业务迁移会改变启动和资源特征；若预算方法尚未稳定，后续只能依赖主观判断，违背性能优先硬约束。

## 七、阶段合同

1. `baseline-only`：Issue `#32` 当前状态，没有预算值。
2. `observation`：独立复测 Issue 收集每个平台至少五次可比 Run。
3. `approved-observation`：项目所有者批准 warning/blocking 数值，预算 CI 以报告模式进入 `main`，连续五次可比执行无误报。
4. `enforced`：独立批准后才允许进入 required 汇总。

Gate 5 首个功能 Issue 只能在第三阶段已经进入 `main`、双平台至少成功执行一次且 `release_audit=current` 后建立。首个正式 Release 前必须进入第四阶段。

## 八、长期状态与错误语义修正

- PR `#49` 已于 `2026-07-25T19:35:18Z` Squash Merge 为 `fd9db9ca1c150b7db34dda8acc09b6f0cc357a17`；Issue `#32` 已按 `COMPLETED` 关闭。
- Issue `#32` 旧标题包含“预算批准”，但获批范围和实际交付均不含预算。长期入口必须明确“基线完成、预算未批准”。
- 本任务直接在新的实质 Discovery 中修正长期入口，不创建递归 Closeout，也不改写 Issue `#32` 的历史计划和报告。
- Master Plan 的固定 `v1.2.41-inputcodex.1` 与 ADR `0002` 的动态版本规则不一致；本任务只恢复对 ADR `0002` 的引用，具体 Release 版本和资产由独立 `type:release` Issue 决定。

## 九、后续互斥 Issue

### 性能复测与数值批准

- 运行新的 GitHub-hosted 可比测量。
- 每个平台形成至少五次独立 Run 的队列证据。
- 提出 warning/blocking 数值、单位、安全裕量和所有者批准。
- 不修改预算 CI 或产品代码。

### 性能预算 CI 实施

- 实现队列匹配、预算读取、错误码、报告和 `approved-observation` 模式。
- 使用获批数值，不重新决定预算。
- 在稳定观察完成前不加入 required checks。

### Gate 5 解锁与首个功能迁移

- 只在 ADR `0004` 的解锁条件满足后创建。
- 选择依赖最小的基础能力垂直切片，行为合同先行。
- 不与预算调整、上游同步或 Release 工作混合。

### Release 版本冻结

- 复核 ADR `0002` 的 `v<上游版本>-inputcodex.<修订号>` 规则。
- 冻结具体首版版本、签名、资产、升级和回滚合同。
- 不属于 Issue `#50` 或 Gate 5 功能迁移范围。

## 十、完成边界

本报告没有预算数值，没有运行新的真实性能采集，没有修改 `benchmarks/`、代码、Workflow、Ruleset、Release 或 AGOS，也没有解锁 Gate 5。其唯一作用是把下一阶段的输入、方法、错误语义和独立 Issue 边界冻结为可审计合同。

## 十一、本地 Fresh 验证

- 实际差异并集精确为批准的九路径，重算 `scope_hash` 为 `sha256:af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f`。
- 根 Cargo、`apps/`、`crates/`、`parity/`、`upstream/`、`benchmarks/`、Workflow、Ruleset、Release 和 AGOS 差异为 `0`。
- 长期入口必需事实全部存在，PR `#49` 待合并、Issue `#32` 预算已批准和固定 `v1.2.41-inputcodex.1` 等过期语义命中数为 `0`。
- ADR 与报告尾随空白、NUL 和常见占位标记命中数均为 `0`。
- Performance Evidence 输出 `ok=true`、`violation_count=0`，配置、实现和输入哈希均与固定 manifest 匹配。
- CI 合同输出 `CI_CONTRACT_GREEN passed=34`；Repository Policy 输出 `ok=true`、`violation_count=0`；`git diff --check` 通过。
- 本地未运行完整 Workspace、桌面 Release、新 hosted 测量、上游或半成品。

首轮过期状态扫描曾命中 `build.md` 中用于检测旧语句的 `$stalePatterns` 字面量。根因与 `err.md` 已记录的“过期状态扫描把验证器中的正则字面量识别为命中”相同；按既有结论从过期内容扫描集合移除 `build.md`，同时继续通过正向事实、九路径范围、实际命令和差异检查验证该文件。修正后同一 Fresh 门全绿，没有修改业务、性能或外部治理文件来掩盖失败。
