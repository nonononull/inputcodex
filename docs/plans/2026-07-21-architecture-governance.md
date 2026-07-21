# inputcodex 重构、上游同步与发布治理总方案

## 文档状态

- 状态：已批准方案，PR `#3` 已创建并等待项目所有者审阅与合并。
- 决策日期：2026 年 7 月 21 日。
- 功能真源：`BigPizzaV3/CodexPlusPlus` 最新正式 Release。
- 起始上游基线：`v1.2.41`。
- 起始上游提交：`3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 半成品参考：`zsr131550/CodexPlusPlus`，不作为代码底座或功能真源。
- 跟踪 Issue：`#2`。
- 交付 PR：`#3`。

本文件是当前重构方案的单一真源。聊天记录、临时评论和旧筹备计划不能覆盖本文件中的已批准决策；方案变化必须通过新 Issue 和关联 PR 修改本文件或对应 ADR。

## 一、目标与优先级

项目按以下优先级排序：

1. **性能优先**：启动、交互、加载、内存、CPU 和后台行为必须可测量，并作为合并门禁。
2. **功能一致**：有效功能必须与上游稳定基线保持行为一致。
3. **跨平台一致**：Windows 与 macOS 从首版起同步交付，不允许长期存在功能残缺平台。
4. **可持续同步**：持续发现上游变化，判断是否需要缓存、迁移、适配或重新设计。
5. **可维护性**：禁止再次形成巨型入口、UI 与业务耦合、不可取消加载和隐式副作用。

性能优先不代表可以静默删除功能。功能一致与性能发生冲突时，必须通过 Issue 提供数据和候选方案，由项目所有者决定。

## 二、硬约束

- 产品名称固定为 `inputcodex`。
- Windows 与 macOS 从首版起保持功能一致。
- 产品业务代码和 UI 使用 Rust。
- 禁止 TypeScript、JavaScript 业务代码和 WebView。
- UI 主框架采用 Iced。
- Iced 只能存在于展示层，其他层不得依赖 Iced 类型。
- 禁止广告、推广位、广告 SDK、付费导流和隐蔽遥测进入最终运行面。
- 上游完整源码进入本仓库缓存和审计范围，包括 Tauri、React、TypeScript、注入脚本、远程列表、广告代码及素材。
- 上游缓存不直接参与新产品构建，也不能成为新架构的运行时依赖。
- 上游 Tauri/React 管理界面、现有注入脚本和远程推荐列表不得直接进入新架构或最终运行面；其背后的有效能力只能通过独立功能或一致性例外 Issue 重新设计。
- 所有正式工作执行 `Issue → 分支 → 验证证据 → 关联 PR → Review/CI → Merge`。
- 禁止直接向 `main` 写功能或发布改动。

## 三、仓库目标结构

```text
inputcodex/
├─ apps/
│  └─ inputcodex-desktop/             # Iced 桌面入口，仅组合功能
├─ crates/
│  ├─ inputcodex-domain/              # 领域模型、规则和稳定值对象
│  ├─ inputcodex-application/         # 用例、任务编排、端口接口
│  ├─ inputcodex-infrastructure/      # SQLite、文件、HTTP、更新和缓存适配
│  ├─ inputcodex-platform/            # Windows/macOS 平台能力
│  ├─ inputcodex-presentation/        # Iced 状态、消息、视图和主题
│  └─ inputcodex-parity/              # 功能合同、差异和上游映射工具
├─ upstream/
│  ├─ CodexPlusPlus/                  # 上游完整快照，不含上游 .git
│  ├─ source-lock.json                # 上游版本、提交、来源和校验信息
│  └─ sync-reports/                   # 每次检测与同步报告
├─ parity/
│  ├─ features/                       # 功能矩阵
│  ├─ contracts/                      # 输入、输出、副作用和错误合同
│  ├─ fixtures/                       # 脱敏测试夹具
│  └─ exceptions/                     # 已批准一致性例外引用
├─ benchmarks/                        # 性能场景和基准定义
├─ docs/
│  ├─ adr/
│  ├─ plans/
│  ├─ reports/
│  └─ upstream/
└─ xtask/                             # 构建、校验、同步和发布辅助命令
```

上游快照目录必须通过构建配置明确排除在 Cargo Workspace、打包输入和最终安装包之外。

## 四、依赖方向

```text
presentation(Iced)
        ↓
application
        ↓
domain

infrastructure ──implements──> application ports
platform       ──implements──> application ports
```

约束：

- `domain` 不依赖 UI、网络、数据库、操作系统或异步运行时。
- `application` 定义用例和端口，不直接访问文件、SQLite、HTTP 或系统 API。
- `infrastructure` 和 `platform` 实现端口，并将外部错误转换为稳定错误类型。
- `presentation` 只发送意图和显示状态，不直接执行磁盘、网络、进程或数据库操作。
- `apps/inputcodex-desktop` 只负责依赖组装、启动和生命周期，不容纳业务逻辑。

## 五、Iced 状态与加载架构

每个功能域拥有独立模块：

```text
feature/
├─ state.rs
├─ message.rs
├─ update.rs
├─ view.rs
├─ effect.rs
└─ tests.rs
```

规则：

- 根应用只负责路由和功能模块组合，禁止形成全功能巨型 `app.rs`。
- 每个加载操作具备 `Idle / Loading / Ready / Empty / Failed / Cancelling` 等明确状态。
- 每次异步请求包含请求标识；过期结果不得覆盖新状态。
- 路由切换、窗口关闭、参数变化和用户取消必须能终止或废弃在途任务。
- 磁盘、SQLite、网络、压缩、进程和系统 API 调用禁止阻塞 UI 更新路径。
- 后台并发必须有界，禁止无上限创建任务或重复刷新。
- 错误信息对用户稳定，对诊断日志保留必要上下文，但不得记录密钥和敏感内容。

## 六、功能一致性治理

### 6.1 功能矩阵

每个上游功能必须登记唯一标识和以下状态之一：

- `unassessed`：尚未审计。
- `planned`：已有迁移 Issue。
- `implementing`：关联 PR 正在实施。
- `implemented`：代码完成，尚未双平台验收。
- `verified`：功能、错误、副作用和双平台行为均验收通过。
- `exception-pending`：存在争议，等待项目所有者决定。
- `exception-approved`：允许与上游存在明确差异。
- `retired`：经项目所有者批准确认不进入产品。

### 6.2 一致内容

默认要求一致：

- 输入与允许值。
- 输出与用户可见结果。
- 配置、文件、SQLite 和其他数据兼容性。
- 持久化结果和恢复行为。
- 网络、文件、进程和系统修改等副作用。
- 错误分类、失败提示、重试和部分成功语义。
- 操作流程和关键确认步骤。

UI 允许重新设计，不要求像素一致，但不得隐藏必要信息、确认和失败状态。

### 6.3 一致性例外

出现以下情况时必须建立 `parity-exception` Issue，禁止开发者自行决定：

- 上游功能已经失效或没有实际效果。
- 功能产生数据破坏、配置污染、无意义后台任务或额外网络请求。
- 功能包含广告、推广、导流或遥测。
- 功能依赖上游现有注入脚本、远程推荐列表或旧 Tauri/React 运行面。
- 上游错误语义明显有争议。
- 保持一致会造成显著性能或安全风险。
- Windows 与 macOS 无法在相同语义下实现。

Issue 必须包含复现步骤、上游版本与提交、实际行为、预期行为、副作用、兼容性影响、性能或安全证据以及候选方案。没有项目所有者批准，不允许合并差异实现。

## 七、上游快照和持续监控

### 7.1 起始快照

- 导入 `BigPizzaV3/CodexPlusPlus v1.2.41` 的完整源码内容。
- 不导入上游 `.git` 和历史提交。
- 保留所有源码、脚本、资源、远程列表、广告代码和历史 UI，用于审计和重构。
- `source-lock.json` 记录仓库、标签、提交、获取时间、文件数量、校验摘要和许可证。
- 当前上游约 277 个文件、23.06 MiB，没有超过 10 MiB 的单文件，起始阶段不使用 Git LFS。

### 7.2 云端监控

GitHub Actions 使用标准 `ubuntu-latest` Runner：

- 每 6 小时扫描一次，避开整点。
- 支持 `workflow_dispatch` 手动扫描。
- 单次超时 5 分钟。
- 使用并发锁防止重复运行。
- 不上传 Artifact，不使用 Larger Runner。
- 只读取上游提交、标签和 Release 元数据。
- 发现变化时创建或更新 `upstream-watch` Issue。
- 不自动修改缓存、不自动创建实现 PR、不自动合并。

### 7.3 同步流程

```text
检测到上游变化
  → upstream-watch Issue
  → 人工分类：忽略 / 缓存 / 迁移 / 重新设计 / 例外讨论
  → upstream-sync Issue
  → sync/upstream-<版本或提交> 分支
  → 仅更新 upstream/ 与同步报告
  → 关联 PR 与快照纯净性 CI
  → 合并缓存更新
  → 分别创建功能迁移或一致性例外 Issue
```

上游同步 PR 禁止同时修改 `apps/`、`crates/` 和产品功能。功能迁移 PR 禁止顺带更新上游快照。

## 八、性能治理

### 8.1 必测指标

- 冷启动到首个可交互界面的耗时。
- 热启动耗时。
- 空闲 RSS、峰值 RSS 和空闲 CPU。
- UI 帧耗时的 P50、P95 和 P99。
- 主要路由首次加载和重复加载的 P50、P95。
- 大量会话、供应商、脚本和插件数据下的响应时间。
- 取消操作到任务停止或结果失效的时间。
- 后台网络请求数量、重复请求和失败重试次数。
- 安装包体积与首次启动缓存体积。

### 8.2 基线建立

第一项性能实施 Issue 必须在相同测试数据和可比环境下测量：

- 上游稳定基线 `v1.2.41`。
- 半成品参考的原生实现。
- `inputcodex` 当前实现。

绝对预算数值必须由性能基线 Issue 和关联 PR 固化，不能在缺乏实测时拍脑袋填写。预算确定后，性能回退必须阻止合并；因 Runner 抖动导致的端到端数据只能作为趋势证据，稳定微基准和确定性状态测试作为 CI 硬门禁。

### 8.3 性能 PR 要求

- 功能 PR 必须提供受影响场景的前后数据。
- 禁止用减少功能、隐藏错误或跳过持久化伪造性能提升。
- 缓存必须定义生命周期、容量、失效和敏感数据边界。
- 性能优化造成行为差异时，进入一致性例外流程。

## 九、版本、发布和更新

### 9.1 版本格式

```text
Git 标签与机器版本：v<上游基础版本>-inputcodex.<修订号>
界面显示：inputcodex <上游基础版本>-r<修订号>
```

示例：

- `v1.2.41-inputcodex.1`
- `v1.2.41-inputcodex.2`
- 上游升级后：`v1.2.42-inputcodex.1`

同一上游版本上的自有修复只递增修订号。上游基础版本升级后，修订号重置为 `1`。

正式版必须完成对应上游稳定版本的功能矩阵、双平台验收和性能门禁。开发预览版本可以使用额外的 `alpha` 或 `beta` 标识，但不得冒充功能一致正式版。

### 9.2 发布资产

```text
inputcodex-<version>-windows-x64-setup.exe
inputcodex-<version>-windows-x64.zip
inputcodex-<version>-macos-arm64.dmg
inputcodex-<version>-macos-arm64.zip
inputcodex-<version>-macos-x64.dmg
inputcodex-<version>-macos-x64.zip
latest.json
latest.json.sig
SHA256SUMS
```

### 9.3 自主更新源

- 客户端只检查 `nonononull/inputcodex` 的 Release 和更新清单。
- 禁止客户端从 `BigPizzaV3/CodexPlusPlus` 下载安装包、脚本、远程列表或运行资源。
- 需要动态更新的上游资源必须先镜像进本仓库或本仓库 Release，并纳入签名与版本治理。
- `latest.json` 必须包含版本、平台、架构、资产地址、大小、SHA-256、发布时间和最低兼容版本。
- 更新清单使用独立签名；应用内置公钥并在下载和安装前同时校验清单签名与资产摘要。
- Windows 和 macOS 发布签名密钥只保存在 GitHub Environments/Secrets，不进入源码、日志或 Artifact。
- 发布工作流使用最小权限和受保护环境；第三方 Action 固定到可信提交或稳定版本。

### 9.4 发布流程

```text
Release Issue
  → release/v<版本> 分支
  → 版本与更新清单 PR
  → Windows/macOS 构建
  → 功能一致性检查
  → 性能预算检查
  → 无广告和更新源检查
  → 人工 Review
  → 合并 main
  → 创建受保护标签
  → GitHub Actions 发布到 nonononull/inputcodex
  → 安装和自更新冒烟测试
```

Release Notes 必须列出上游版本和提交、功能一致范围、自有修复、已批准差异、双平台验证、性能结果和资产校验值。

## 十、GitHub 治理

### 10.1 Issue 类型

- `upstream-watch`：自动记录上游变化。
- `upstream-sync`：更新完整上游快照。
- `feature-parity`：迁移和验收上游功能。
- `parity-exception`：讨论无效功能、副作用或错误语义。
- `performance`：建立基线或优化性能。
- `architecture`：修改模块边界或硬约束。
- `release`：一次正式或预览发布。
- `bug`：已迁移功能的缺陷。

### 10.2 分支命名

- `sync/upstream-v1.2.42`
- `feature/issue-123-provider-switch`
- `parity/issue-145-error-semantics`
- `perf/issue-160-session-loading`
- `docs/issue-2-architecture-governance`
- `release/v1.2.41-inputcodex.1`

### 10.3 PR 必填证据

- `Closes #<issue>` 或明确的 Issue 关联。
- 上游版本、提交和源文件引用。
- 功能矩阵状态变化。
- 测试命令和结果。
- Windows 与 macOS 影响和验证结果。
- 性能数据或不适用原因。
- 副作用、错误语义和数据迁移说明。
- 一致性例外批准引用，若存在差异。
- 更新源、广告和敏感信息检查。

### 10.4 `main` 保护

- 禁止直接推送功能与发布改动。
- 必须通过关联 PR。
- 永久禁止对 `main` 使用 Force Push，包括 `--force` 和 `--force-with-lease`；项目所有者与管理员也不得例外。
- 已进入 `main` 的错误历史只能通过 `revert` 提交和关联 Issue/PR 修正，紧急修复不得重写历史。
- 永久禁止删除 `main`，项目所有者与管理员也不得例外。
- 若 `main` 被意外删除，只能从删除前最后一个权威提交恢复同名分支并建立事故 Issue；恢复过程不得改写或替换既有历史。
- 仓库规则必须启用合并前解决所有 Review 对话。
- 每条 Review 对话只有在根因已确定、处理已完成且验证证据已回写后才可解决；若反馈不成立，必须给出可复核证据并取得 reviewer 或项目所有者确认。
- 禁止仅点击 `Resolve conversation`、以关闭对话代替解决问题，或带着任何未解决对话合并 PR。
- 只允许 Squash Merge；仓库必须禁用 Merge Commit 和 Rebase Merge。
- 每个 Issue 合并后在 `main` 上只形成一条可独立追踪和回滚的提交。
- 单人维护阶段 required approvals 为 `0`，但关联 Issue 或 PR 必须存在项目所有者明确批准证据。
- 当第二名具备 `write`、`maintain` 或 `admin` 权限的人类维护者加入时，必须在下一次 PR 合并前把 required approvals 提升为 `1`；Bot、GitHub App 和自动化账号不计入人数。
- 必须通过要求的 CI 检查。
- 架构、一致性例外和发布 PR 必须由项目所有者批准。
- 合并后删除功能分支。
- `main` 应保持可构建；正式发布仍需单独 Release Gate。

#### 已落地状态（2026-07-21）

- GitHub Ruleset `main-protection` 已启用，规则集 ID 为 `19395456`。
- 规则范围只包含 `refs/heads/main`，无排除分支、无 bypass actor，项目所有者与管理员不在例外名单中。
- 有效规则为 `deletion`、`non_fast_forward` 和 `pull_request`：禁止删除、禁止 Force Push、所有变更必须通过 PR。
- `pull_request` 规则当前 required approvals 为 `0`，要求解决全部 Review 对话，且只允许 Squash Merge。
- 当前仓库尚无 required status checks；加入 CI 后必须通过独立 Issue/PR 把稳定检查加入 Ruleset，不能伪造不存在的检查。
- 落地与验证证据见 `docs/reports/2026-07-21-main-protection-rollout.md`。

## 十一、分阶段实施

### Gate 1：方案与治理冻结

- 合并本总方案、术语表和 ADR。
- 建立 Issue/PR 模板、标签和分支保护。
- 建立新的 `build.md` 和 `err.md` 执行入口。

### Gate 2：导入上游基线

- 通过独立 Issue 和 PR 导入 `v1.2.41` 完整快照。
- 创建 `source-lock.json` 和快照纯净性校验。
- 建立每 6 小时的上游监控工作流。

### Gate 3：纯 Rust 工作区骨架

- 建立分层 Cargo Workspace。
- 建立 Iced 最小双平台窗口，但不迁移业务功能。
- 建立 Windows/macOS CI、格式、测试、依赖和许可证检查。
- 建立更新源配置的仓库归属测试。

### Gate 4：功能目录与性能基线

- 从上游快照生成首版功能矩阵。
- 为数据格式、副作用和错误语义建立合同测试。
- 测量上游、半成品和新骨架的性能基线。
- 通过 Issue/PR 批准具体性能预算。

### Gate 5：分域迁移

按依赖和风险分批迁移：

1. 应用路径、配置、诊断、更新和平台基础能力。
2. 供应商、协议代理和网络配置。
3. 会话、SQLite、导入导出和数据维护。
4. 插件、皮肤及经独立 Issue 批准的脚本类有效能力；禁止直接迁移上游现有注入脚本和远程推荐列表。
5. 中继、远程集成、安装和桌面生命周期。

每个迁移项必须独立 Issue 和 PR，不能用一个“大迁移 PR”覆盖多个不可独立验收的功能域。

### Gate 6：首个正式版本

- 功能矩阵达到已批准发布范围。
- 所有未一致项均有项目所有者批准的例外。
- Windows 与 macOS 安装、启动、升级和回滚通过。
- 性能预算通过。
- 更新签名、资产摘要和自主下载源通过。
- 发布 `v1.2.41-inputcodex.1`。

## 十二、完成定义

本重构不是“界面能打开”即完成。正式完成必须同时满足：

- 对应上游稳定版本的有效功能已验证一致。
- 所有差异均有可追溯的项目所有者决策。
- Windows 与 macOS 功能一致。
- 性能预算通过且无伪优化。
- 最终运行面没有广告、推广、遥测、TypeScript、JavaScript 业务代码和 WebView。
- 客户端只使用 `nonononull/inputcodex` 的更新与资源。
- 所有交付都可由 Issue、PR、CI、Release 和签名证据追踪。
