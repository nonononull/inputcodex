# Issue #24：Gate 4 功能目录与性能基线规划

plan_status: candidate-review-ci-green-final-seal-pending
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/24
branch_ref: codex/issue-24-gate-4-planning
pr_ref: https://github.com/nonononull/inputcodex/pull/25
approved_decision_ref: user-message:approve-gate-4-option-2-planning-2026-07-22
scope_hash: sha256:72e2f5d774080a55599297909600aba3c9f58470710b71db25d3690a61a1cbf0
baseline_ref: f470c062037042a1f7833a29cdcf216f6c0f5601
upstream_release_ref: v1.2.41
upstream_tag_commit_ref: 3dafffcafb2566a1e8bce4b35671656d6adb3eda
upstream_main_alert_ref: 91376ee3518cb5fe5ec8eead179418f706c25870

## 一、目标

本任务只把 Gate 4 从一句总目标变成可执行、可审查、可停止的规划合同。规划 PR 不创建真实功能矩阵数据、合同测试、性能基准或产品行为；合并后再分别创建两个独立执行 Issue：

1. 功能矩阵、行为合同与脱敏夹具。
2. 性能基线、可比测量与预算批准。

这种拆分避免功能审计、争议决策、上游运行、基准建设和产品修改混入一个不可独立验收的大 PR。

## 二、Fresh 基线

- Gate 3 实现由 Issue `#19` / PR `#21` 完成，独立 closeout 由 Issue `#22` / PR `#23` 完成。
- `main` 权威提交为 `f470c062037042a1f7833a29cdcf216f6c0f5601`；合并后运行 `29922385227` 六 Job 全绿，成功 Artifact 数为 `0`。
- 最新正式功能真源仍为 `BigPizzaV3/CodexPlusPlus v1.2.41`，缓存 tag 提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 上游 `main` 当前为 `91376ee3518cb5fe5ec8eead179418f706c25870`，只作为 Issue `#20` 的变化预警，不改变 Gate 4 基线。
- 半成品参考尚未导入；后续性能执行 Issue 必须先固定来源、提交、许可证和可复现构建方式，不能临时拉取浮动分支形成比较结论。

## 三、已批准设计

采用“两阶段拆分”：

- **当前规划阶段**：冻结数据结构、证据规则、测量协议、执行 Issue 边界和停止条件。
- **后续执行阶段 A**：创建首版功能矩阵、行为合同、脱敏夹具和相应验证器，不迁移产品功能。
- **后续执行阶段 B**：建立性能场景、测量三套实现、形成原始样本和预算提案，不顺带优化产品。

规划 PR 的创建已获批准；规划 PR 合并、阶段 A、阶段 B 和任何 `parity-exception` 决策均需要新的项目所有者授权。

## 四、功能矩阵合同

### 4.1 稳定标识与分域

- 功能标识使用 `feature.<domain>.<slug>`，一经进入 `main` 不因文件移动或 UI 重设计改变。
- `domain` 只允许：`foundation-platform`、`provider-network`、`session-data`、`plugin-script`、`remote-install`。
- 每项至少记录名称、上游 Release/tag 提交、证据路径、入口、Windows/macOS 适用性和关联决策引用。

### 4.2 行为字段

每项必须能映射到：

- 输入、允许值和默认值。
- 输出、用户可见结果和数据格式。
- 文件、配置、SQLite、网络、进程和系统修改等副作用。
- 持久化、恢复、重试、部分成功和失败后的状态。
- 错误分类、用户提示、诊断上下文和敏感信息边界。
- `Idle / Loading / Ready / Empty / Failed / Cancelling` 加载语义、请求标识、超时和取消。
- Windows 与 macOS 的共同语义；平台差异不能隐式藏在实现备注中。

### 4.3 一致性状态

只使用架构单一真源已批准的状态：`unassessed`、`planned`、`implementing`、`implemented`、`verified`、`exception-pending`、`exception-approved`、`retired`。

- 执行阶段首次登记默认为 `unassessed`。
- 发现无效功能、有害副作用、广告/遥测、旧注入/远程推荐依赖、错误语义或跨平台冲突时，只能转为 `exception-pending` 并创建候选 `parity-exception` Issue。
- `exception-approved` 与 `retired` 只能引用项目所有者的明确决定。

## 五、行为合同与脱敏夹具

- 合同标识使用 `contract.<feature-id>.<scenario>`；每个合同包含前置状态、输入、输出、副作用、错误、超时、取消、可观测证据和平台期望。
- 夹具标识使用 `fixture.<feature-id>.<scenario>`，只允许合成数据或不可逆脱敏数据。
- 禁止真实 API Token、Cookie、账号、会话内容、设备标识、私人绝对路径、签名材料和生产数据库进入仓库、日志或 Artifact。
- 脱敏后仍必须保留数据类型、长度等级、边界值和关联关系；不能用空文件或无结构假数据伪造合同通过。
- 执行阶段拟使用 `parity/features/*.yml`、`parity/contracts/*.yml` 与 `parity/fixtures/<feature-id>/`；本规划任务不得创建这些路径。

## 六、性能测量协议

### 6.1 对象与可比性

- 后续性能 Issue 在同一场景、同一夹具和可比环境下测量上游 `v1.2.41`、固定提交的半成品参考和当时的 `inputcodex`。
- 任一对象无法合法固定来源、构建或运行时，标记为不可比较并停止预算批准；禁止以缺失样本推断性能优势。
- 标准 GitHub-hosted Runner 数据只作趋势和跨提交证据；收费 Runner、self-hosted Runner 或项目所有者本机基线必须通过独立 Issue 批准。

### 6.2 场景与采样

- 冷启动和其他昂贵端到端场景：每个系统/平台至少 `5` 个成功样本，记录中位数、最小值、最大值和全部原始样本。
- 可重复加载与稳定微场景：先执行 `3` 次预热，再记录至少 `20` 个样本，报告 P50/P95；样本少于 `100` 时不把 P99 作为硬结论。
- UI 帧时间：稳定场景至少采集 `600` 帧，报告 P50/P95/P99 和卡顿帧比例。
- 空闲资源：完成 `30` 秒稳定等待后，以 `1 Hz` 采样 `60` 秒 RSS/CPU；同时记录场景峰值 RSS。
- 取消：从用户取消到任务停止或结果失效，至少记录 `20` 个样本，并证明旧请求不能覆盖新状态。
- 网络：记录请求总数、重复请求、重试次数、失败分类和取消后的额外请求；不得访问真实私人账号。

### 6.3 预算批准

- 本规划任务不填写绝对预算。
- 性能执行 Issue 必须提交环境指纹、原始样本、统计摘要、不可比较项和预算候选。
- Runner 抖动明显的端到端指标不能直接成为 CI 硬门禁；稳定微基准和确定性状态测试才可作为后续硬门禁候选。
- 禁止通过删除有效功能、隐藏错误、跳过持久化或弱化双平台语义伪造提升。

## 七、后续执行 Issue 边界

### 7.1 功能目录执行 Issue

- 允许创建 `parity/features/`、`parity/contracts/`、`parity/fixtures/` 和必要的 `inputcodex-parity` 验证代码。
- 只做审计、数据合同和验证器，不迁移 `apps/` 或其他产品功能。
- 每个争议项独立分流到 `parity-exception`，不得在目录 PR 中偷偷批准差异。

### 7.2 性能基线执行 Issue

- 允许创建 `benchmarks/`、测量脚本、原始结果报告和预算提案；具体 CI 路径必须在该 Issue 中重新批准。
- 不修改产品逻辑进行优化；基线与优化必须使用不同 Issue/PR。
- 半成品参考必须固定提交和许可证证据，不得使用浮动 `main`。

## 八、本任务允许路径

```text
AGENTS.md
README.md
build.md
err.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/2026-07-22-issue-24-gate-4-feature-performance-plan.md
docs/plans/sessions/2026-07-22-issue-24-gate-4-feature-performance-plan.md
docs/workflows/2026-07-22-issue-24-gate-4-feature-performance-runtime.md
docs/reports/issue-24-gate-4-feature-performance-plan.md
```

范围哈希为 `sha256:72e2f5d774080a55599297909600aba3c9f58470710b71db25d3690a61a1cbf0`；规范化方式是按路径升序、LF 分隔并保留末尾 LF。

## 九、禁止范围

- 不创建 `parity/`、`benchmarks/`，不修改 Cargo、Rust 产品源码、测试、`scripts/ci/`、Workflow、upstream、Ruleset、Release 或 AGOS。
- 不运行或发布上游 Tauri/React 管理界面、注入脚本和远程推荐列表。
- 不实现 UI、功能迁移、性能优化、一致性差异或绝对预算。
- 不修改 Issue `#16/#20`；上游 watch 状态由机器维护。
- 不 Force Push、不删除 `main`、不使用管理员绕过、Merge Commit、Rebase Merge 或自动合并。

## 十、交付顺序

1. Fresh 核对 `main`、Release、上游 main 预警、Ruleset 和维护者数量。
2. 创建 Issue `#24` 与分支，冻结批准引用、路径和范围哈希。
3. 落盘任务计划、Session Plan、Runtime Workflow、初始报告并更新原生控制面。
4. 运行 Issue `#24` 本地验证、治理合同、仓库政策和空白检查。
5. 普通提交、推送并创建关联规划 PR；回写 PR 引用。
6. 等待 Review/CI；规划 PR 合并前再次取得项目所有者明确授权。

## 十一、完成与停止条件

完成必须同时满足：9 条路径无越界、两阶段执行边界清晰、无占位符或未定义状态、本地验证通过、PR 非 Draft、自动合并关闭、适用 CI 成功、Review 对话全部完成根因闭环。

出现以下任一情况立即停止：最新正式 Release 变化、需要修改批准路径外文件、需要实际运行参考产品、需要创建目录/基准代码、需要预算数值或一致性决策、Ruleset/维护者数量/Head/授权发生物质漂移。
