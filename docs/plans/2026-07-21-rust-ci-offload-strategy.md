# Rust CI 云端编译卸载方案

status: approved
decision_date: 2026-07-21
owner: nonononull
scope: 设计与治理约束；当前不创建 GitHub Actions Workflow
implementation_gate: Gate 2 上游监控与 Gate 3 Rust 工作区

## 一、决策

`inputcodex` 采用“本地轻量验证 + GitHub Actions 全量验证”的混合模式：

- 本地机器不承担每个 PR 的全量 Rust Workspace、Windows/macOS 双平台或安装包编译。
- 本地只运行当前任务必要的快速检查、定向 `cargo check/test` 和问题复现命令，具体入口由各阶段 `build.md` 定义。
- 全量 Workspace 检查、双平台编译、完整测试和发布构建交给公开仓库的标准 GitHub-hosted runners。
- 本方案只冻结边界、作业分层和安全约束；当前 Issue `#2` 不创建 Workflow、不引入源码、不启用 required status checks。

## 二、为什么选择混合模式

### 方案 A：本地全量构建

- 优点：反馈不依赖云端排队，调试环境完全可控。
- 缺点：Rust 与 Iced 双平台构建持续占用项目所有者本地 CPU、内存、磁盘和时间，无法验证真实 macOS 环境。
- 结论：拒绝作为默认流程；只保留为自愿调试手段。

### 方案 B：本地轻量 + 云端全量

- 优点：本地反馈足够快，Windows/macOS 由真实标准 Runner 验证，资源成本与开发机隔离。
- 缺点：依赖 GitHub 排队与缓存质量，需要控制重复运行、超时和 Artifact。
- 结论：采用；这是性能、双平台一致性和维护成本之间的最小可行平衡。

### 方案 C：完全 CI-only

- 优点：本地资源占用最低。
- 缺点：格式错误、简单编译错误也必须等待云端，调试循环过慢并浪费 CI 队列。
- 结论：拒绝；本地仍保留低成本、定向的快速检查。

## 三、硬约束

- 公开仓库默认只使用标准 GitHub-hosted runners；禁止 Larger Runner。
- 禁止默认使用项目所有者本地机器作为 self-hosted runner。
- 启用任何收费 Runner、self-hosted runner 或新的付费资源前，必须建立独立 Issue 并取得项目所有者批准。
- 本地未运行全量构建不能代替 CI 证据；需要全量验证的 PR 必须等待对应云端检查完成。
- CI 失败、超时或取消不能被解释为通过；重复点击重新运行不构成根因解决。
- Windows 与 macOS 从首个 Rust 骨架开始使用真实平台 Runner 验证，不以交叉编译结果代替平台验收。
- GitHub Actions Workflow 必须由独立 Issue/分支/PR 引入；当前治理 PR 只记录设计，不创建 `.github/workflows/` 文件。

## 四、本地与云端职责

### 4.1 本地默认职责

- 读取 `build.md` 和 `err.md`。
- 执行格式检查或格式化。
- 对当前修改的 crate 或模块执行定向 `cargo check`、单元测试或复现命令。
- 在提交前执行 Git diff、Session Plan 和项目定义的轻量验证。
- 性能问题只在具备可比环境时本地测量；本地结果不得冒充跨平台 CI 结果。

当前仓库尚无 Cargo Workspace，因此本方案不虚构具体包名、Feature 或 Cargo 参数。Gate 3 创建 Workspace 的同一 PR 必须把准确命令写入 `build.md`。

### 4.2 PR 云端职责

Gate 3 的首版 PR CI 至少拆成以下稳定职责，而不是把所有工作塞入单个不可诊断 Job：

- Linux 质量 Job：格式、Clippy、单元测试和与平台无关的确定性检查。
- Windows Job：Workspace 编译、Windows 平台测试和 Iced 平台集成检查。
- macOS Job：Workspace 编译、macOS 平台测试和 Iced 平台集成检查。
- 治理 Job：许可证、依赖、更新源归属、禁止广告与禁止 WebView/JavaScript 业务代码检查。

PR 默认不构建最终安装包，不读取发布密钥，也不上传整个 `target/`。

### 4.3 `main` 云端职责

- 运行与 PR 相同或更严格的全量检查。
- 验证合并后的 Workspace 状态，防止多个 PR 组合后产生回归。
- 仅在 Job 名称、触发条件和失败语义稳定后，才通过独立 Issue/PR 将其加入 `main` Ruleset 的 required status checks。

### 4.4 Release 云端职责

- 只由受保护的自主版本标签触发。
- 构建 Windows 与 macOS 正式安装包，执行签名、校验摘要、更新清单和发布资产验证。
- 发布密钥只进入 Release 环境；Fork PR、普通 PR 和上游监控 Job 不得获得密钥。
- 发布资产和下载地址只能属于 `nonononull/inputcodex`。

## 五、分阶段落地

### Gate 1：当前治理阶段

- 冻结本方案并纳入项目硬约束。
- 不创建 Workflow，不增加 required status checks。

### Gate 2：上游快照与监控

- 使用标准 `ubuntu-latest` Runner 建立每 6 小时上游监控。
- 单次任务只读取上游元数据并管理 Issue，不编译 Rust、不上传 Artifact、不自动同步或合并。

### Gate 3：Rust Workspace 与构建 CI

- 创建 Cargo Workspace 和 Iced 双平台骨架。
- 在同一 Issue/PR 中写明 Rust 工具链、系统依赖、本地轻量命令和云端全量命令。
- 创建 Linux、Windows、macOS 标准 Runner Job。
- 先以非 required 状态观察 Job 名称、耗时与失败语义；稳定后再单独更新 Ruleset。

### Gate 4 及以后：性能与功能迁移

- 稳定微基准和确定性合同测试可以成为 required checks。
- 受 Runner 抖动影响的端到端性能只作趋势证据，除非已经建立可重复的统计门槛。
- 每项迁移功能只触发必要的定向检查，同时保留合并前全量门禁。

## 六、资源与缓存治理

- PR Workflow 使用 `concurrency`，同一 PR 新提交到达后取消旧运行。
- 避免同一提交同时被 `push` 与 `pull_request` 重复执行相同全量矩阵。
- 每个 Job 必须声明有限 `timeout-minutes`；具体数值由 Gate 3 冷构建基线决定，禁止直接沿用平台最大时长。
- 首版 CI 先记录无缓存冷构建基线，再决定是否缓存 Cargo registry、Git 依赖、编译器缓存或 `target/` 子集。
- Cache key 必须至少隔离操作系统、Rust 工具链、构建配置和 `Cargo.lock`。
- Cache 总量必须保持在仓库当前免费额度内；GitHub 政策变化时采用更严格限制。
- 禁止把整个 `target/` 作为 Artifact 上传。
- 非 Release Artifact 最长保留 7 天，只保存失败诊断、测试报告或确有审计价值的产物。
- Release 安装包进入 GitHub Release，不依赖短期 Actions Artifact 作为正式下载源。

## 七、公开仓库安全边界

- Fork PR Job 使用最小权限，只读仓库内容，不授予发布、Issue 写入或包发布权限。
- Fork PR、普通 PR 与上游监控 Job 不得读取签名密钥、更新密钥或发布 Token。
- 需要写 Issue 的上游监控 Workflow 必须限制输入来源并使用最小 `issues: write` 权限。
- 第三方 Action 必须确认来源、许可证和维护状态，并固定到不可变提交；优先使用 GitHub 官方 Action。
- Workflow 不执行来自 PR 内容拼接出的未经验证 Shell 命令、路径或发布参数。
- Release Job 与普通 CI 分离，并通过受保护环境和项目所有者批准控制密钥访问。

## 八、失败语义

- 格式、编译、测试、许可证或治理检查失败时，PR 状态为失败，不允许静默降级为警告。
- 平台偶发故障必须先保留日志并判断根因；只有证据表明是 Runner 或外部服务瞬时故障时才允许重试。
- 不稳定测试必须建立 Issue，记录复现率、根因调查和修复证据；禁止长期使用忽略列表掩盖。
- 某平台暂时无法验证时必须标记阻塞，不得用另一平台成功推定双平台通过。

## 九、required checks 升级条件

只有同时满足以下条件，才允许修改 `main-protection` Ruleset：

- 对应 Workflow 已通过独立 Issue/PR 合并。
- Job 名称和触发条件稳定，不会因为矩阵命名变化导致 Ruleset 悬空。
- Windows 与 macOS 均有真实 Runner 成功证据。
- 失败输出能定位到具体阶段，不依赖盲目重跑。
- `build.md` 已记录本地复现或诊断方式。
- 项目所有者批准将该 Job 设为 required check。

## 十、完成标准

- `AGENTS.md`、Master Plan、总架构方案和 Runtime Workflow 均引用本策略且语义一致。
- 当前 PR 不出现 `.github/workflows/` 或 Rust 源码改动。
- Gate 2 与 Gate 3 明确使用标准 GitHub-hosted runners，并禁止默认 Larger/self-hosted runner。
- 本地轻量验证与云端全量验证的责任边界可由 `build.md` 复现。
- CI 稳定前不向 Ruleset 添加不存在或易漂移的 required status checks。
