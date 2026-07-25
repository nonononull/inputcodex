---
status: accepted
---

# 性能预算采用同平台可比队列与分阶段门禁

## 背景

Issue `#32` 已建立 Windows 与 macOS GitHub-hosted 性能基线，覆盖桌面与隔离基准 Release 构建元数据、首次 view、空闲 Working Set、空闲 CPU，以及三个 Rust 场景。当前证据只有每个平台一次独立 hosted Run，足以证明测量合同和环境指纹可用，但不足以批准稳定预算。

共享 GitHub-hosted Runner 会受到镜像、硬件、系统负载和服务事故影响。若直接把单次结果或跨平台差异写成 required check，会把环境噪声误判为产品回归；若完全不设预算，又违背 `inputcodex` 性能优先约束。

## 决策

### 1. 预算按平台和可比队列独立管理

- Windows 与 macOS 使用不同预算，不做跨平台排名、比例换算或共享阈值。
- 比较对象必须属于同一可比队列。硬键包括结果 schema、平台、架构、`github-hosted` Runner 类型、镜像系列、Rust release 与 host、配置哈希、输入哈希和样本合同。
- 镜像精确版本、OS 描述、处理器、逻辑处理器数量和内存档位组成队列指纹。任一字段变化时，本次结果仍可作为新队列观察值，但不得用于旧队列阻断判断。
- `implementation_sha256` 用于证明结果绑定具体实现，允许在被比较提交之间变化；它不是环境可比键。

### 2. 指标分为阻断候选、观察指标和诊断元数据

阻断候选：

- 首次 view 的 run-level median 与 P95。
- 空闲 Working Set 的 run-level median 与 P95。
- `application-load-complete`、`application-cancel-stale`、`parity-repository-validation` 的 run-level median 与 P95；checksum 必须稳定。

观察指标：

- 空闲 CPU 的 median 与 P95。
- 桌面二进制大小。

诊断元数据：

- 桌面 Release 构建耗时。
- 隔离基准 Release 构建耗时与二进制大小。

构建耗时在标准共享 Runner 上不进入强制预算；它只用于发现明显环境或依赖变化。观察指标只有在独立证据证明精度和稳定性后，才能通过新的所有者决策升级为阻断候选。

### 3. 数值批准前至少需要五次独立可比 Run

- 每个平台、每个可比队列至少需要 `5` 次独立成功的 GitHub-hosted Run。
- Run 必须具有不同 `run_id`；单个 Run 的 rerun attempt 不能冒充新的独立样本。
- 因 GitHub 外部事故重跑同一 Run 时，只能保留经根因闭环后认定有效的 attempt，并在证据中保留失败 attempt。
- 当前 Issue `#32` 的有效 Run 是种子证据，不单独构成预算批准依据。

### 4. 统计使用 run-level 稳健汇总，禁止删除样本

- 单个 Run 继续保留既有首次 view、空闲资源和 Rust 场景样本合同。
- 跨 Run 的参考中心为各 Run median 的中位数；参考尾部为各 Run P95 的中位数。
- 同时保存跨 Run 的最小值、最大值和 MAD，作为安全裕量审批证据。
- IQR 标记只表示需要解释的离散，不自动删除样本，也不自动使 Run 无效。
- 预算数值 Issue 必须显式批准 warning limit、blocking limit、单位、队列指纹、来源 Run 和安全裕量；这些字段不得由验证器自行推导后静默生效。

### 5. Run 使用四种互斥语义

- `comparable-valid`：证据合同完整且属于已批准队列，可进入趋势和预算判断。
- `new-cohort-valid`：证据合同完整但环境队列变化，只能启动新观察窗口。
- `evidence-invalid`：样本数量、checksum、哈希、schema、成功状态或 Evidence 合同不满足；不得用于预算。
- `external-incident`：GitHub Actions 或 Runner 服务事故已有可复核外部证据；不得记为产品回归。

IQR 标记数量、单个慢样本或一次超限本身不能把 Run 改写为外部事故。事故、环境漂移和产品回归必须分别记录根因。

### 6. 预算分四阶段生效

1. `baseline-only`：当前状态；只有基线和方法，没有预算值。
2. `observation`：每个平台收集至少五次可比 Run，预算检查不阻断 PR。
3. `approved-observation`：项目所有者批准具体 warning/blocking 数值，CI 验证器以报告模式运行；至少连续五次可比执行无误报、无未解释环境漂移。
4. `enforced`：项目所有者通过独立 Issue/PR 授权将稳定预算加入 required 汇总；任何升级、降级或暂停都必须留存决策证据。

禁止从 `baseline-only` 直接进入 `enforced`。

### 7. 回归采用确认流程，不允许选择性重跑

- 首次超出 warning limit 时标记回归候选，不删除样本、不选择性重跑单个指标。
- 超出 blocking limit 时，验证器报告完整环境、原始结果和差异；只有同队列复核或可解释的实现变化才能形成产品回归结论。
- 若环境键变化，结果进入新队列；若外部服务事故成立，按事故流程重跑同一 Run；否则不得用“Runner 抖动”作为无证据豁免。
- Review 对话必须写明根因、处理方式和验证证据，不能只点击解决。

### 8. Gate 5 解锁条件

Gate 5 首个产品迁移 Issue 只能在以下条件同时满足后建立：

- 本 ADR 已进入 `main`。
- 独立预算复测与数值批准 Issue 已完成每个平台至少五次可比 Run，并取得项目所有者数值批准。
- 独立预算 CI 实施 PR 已以 `approved-observation` 模式进入 `main`，Windows 与 macOS 均至少成功执行一次。
- `release_audit.status=current`。

预算检查在 Gate 5 初期可以保持非 required 的 `approved-observation`；但在首个正式 Release 前必须通过独立决策进入稳定的 `enforced` 阶段。

## 备选方案

### 直接把 Issue #32 单次结果设为 required budget

拒绝。单次共享 Runner 结果无法区分稳定产品回归与环境噪声，且 Issue `#32` 从未获得预算数值授权。

### 先迁移 Gate 5 功能，发布前再补预算

拒绝。该方案无法在架构和功能迁移早期约束启动、空闲资源和核心场景回归，违背性能优先原则。

### 永久只做报告，不建立阻断门禁

拒绝。报告只能发现问题，不能防止已确认回归进入 `main`；但阻断必须经过观察期和所有者批准，不能一步到位。

## 影响

- Issue `#32` 被解释为“基线完成”，不是“预算批准”。
- 后续至少拆分为预算复测与数值批准、预算 CI 实施、Gate 5 解锁三个独立 Issue/PR。
- Windows 与 macOS 预算分别维护，环境漂移启动新队列而不是改写旧证据。
- 原始样本、IQR 标记、失败 attempt 和外部事故证据全部保留。
- 本 ADR 不授权任何预算数值、性能优化、产品功能迁移、收费 Runner 或 Ruleset 修改。
