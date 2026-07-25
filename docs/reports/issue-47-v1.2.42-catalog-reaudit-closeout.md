# Issue #47：`v1.2.42` 功能目录重新审计 Closeout

## 任务元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/47`
- 任务分类：`micro-closeout`
- 权威基线：`5fd337fb7ceb9b0ef53e2e694cc5ddd81ea0a98c`
- 来源 Issue：`https://github.com/nonononull/inputcodex/issues/38`
- 来源 PR：`https://github.com/nonononull/inputcodex/pull/45`
- CI 事故 Issue：`https://github.com/nonononull/inputcodex/issues/46`
- 待消费 Release 预警：`https://github.com/nonononull/inputcodex/issues/33`
- 项目所有者批准：`user-message:批准-Issue-47-五路径-scope_hash-与远端写入范围-2026-07-25`
- 精确范围哈希：`sha256:dd612ef0c2e5f0f830c40f161b1ef1a85bc58cd1d85458a758c3905ade8db03e`
- 本机授权登记时间：`2026-07-25 23:36:39 +08:00`，来自 Windows `Get-Date`
- 本机证据复核时间：`2026-07-25 23:39:22 +08:00`，来自 Windows `Get-Date`

## 一、收口结论

Issue `#38` / PR `#45` 已完成最新正式 Release `v1.2.42` 的二十六路径功能目录重新审计。五域功能目录、行为合同、source-index 与 `release_audit` 已对齐 `v1.2.42@657cd33e009ad02515d30db6492cd4e669b06318`，定向 `catalog_repository` 为 `12/12`，`release_audit` 为 `current`。

PR `#45` 与合并后主干 CI 均已成功。合并后 Run 的前两次 Attempt 失败已经 Issue `#46` 证明为 GitHub Actions 平台事故，不是项目代码、CI 合同或 Runner 回退。本 Closeout 只修正长期控制面和复用排错知识，不改变来源实现或历史阶段快照。

## 二、来源交付证据

- PR `#45` 最终 Head：`d3df8759bdb9c6378497a3a0c8f409c3968f4d4f`。
- 精确变更路径：`26` 条，与 Issue `#38` 获批范围一致。
- 来源范围哈希：`sha256:a384353e947bcb9d95b51ac5ccce49ef9558ca34580c130307a64b6d868819af`。
- PR CI Run：`30157623932`，事件为 `pull_request`，`governance`、`classify`、`release-audit`、`windows`、`macos`、`linux-quality`、`required` 七 Job 全部成功，Artifact 为 `0`。
- GitHub 服务端合并时间：`2026-07-25T12:29:10Z`。
- Squash 提交：`5fd337fb7ceb9b0ef53e2e694cc5ddd81ea0a98c`。
- 父提交：`fdb2f98c701800969fc478f95cd2539be598faaa`，父提交数为 `1`。
- Tree：`5273d0e42483bbb5c629a2243fc24f0a892b3db3`。
- GitHub 签名：`verified=true`、`reason=valid`。
- Issue `#38` 已按 `COMPLETED` 关闭。

## 三、主干 CI 事故与恢复证据

合并后 `main` 触发 Run `30158058627`。Attempt `1/2` 均未创建 Job、Check Run 或日志，页面显示 `Internal server error` 与 Correlation ID `c5f5fd38-d868-4f2a-bd19-4452000f21c6`；GitHub Status 同期报告 Actions `major_outage`。

Issue `#46` 冻结恢复范围后，在 GitHub Actions 恢复为 operational 前没有盲目重跑，也没有修改仓库、Workflow、Ruleset 或 `main`。恢复后只重跑同一 Run：

- Attempt：`3`
- Head：`5fd337fb7ceb9b0ef53e2e694cc5ddd81ea0a98c`
- 事件：`push`
- 结果：七 Job 全部成功
- Artifact：`0`
- Issue `#46`：已按 `COMPLETED` 关闭

同一提交、同一 Workflow 在零代码变更下恢复成功，构成“外部平台事故而非仓库失败”的根因证据。以后遇到零 Job、零日志和平台内部错误时，必须先核验官方状态与 Run 元数据，禁止用代码改动掩盖外部故障。

## 四、长期控制面漂移根因

PR `#45`、Issue `#38` 与 Issue `#46` 已完成，但 `AGENTS.md`、`README.md` 和 Master Plan 仍声称 Issue `#38` 等待 PR、Review/CI，导致项目入口与 GitHub 权威事实不一致。根因是来源实现 PR 只负责获批二十六路径，没有把合并后动态事实反向写入长期控制面。

采用微型五路径 Closeout，而不是改写 Issue `#38` 的历史 Plan、Session Plan 或 Runtime Workflow。来源文件继续保留当时的阶段快照；长期入口只记录最终稳定事实。

## 五、精确范围与禁止面

以下路径按 Windows `Sort-Object` 的大小写不敏感字典序，以 UTF-8、LF 连接并保留末尾 LF 后计算 SHA-256：

```text
AGENTS.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/reports/issue-47-v1.2.42-catalog-reaudit-closeout.md
err.md
README.md
```

```text
scope_hash: sha256:dd612ef0c2e5f0f830c40f161b1ef1a85bc58cd1d85458a758c3905ade8db03e
```

本任务不修改 Rust、Cargo、测试、Workflow、Ruleset、Release、UI、`upstream/`、`source-lock.json`、性能测量、预算、优化、Gate 5 或 AGOS。所有 Git 提交使用 Windows 本机默认时间，不设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

## 六、验证合同

- 工作树差异的“未暂存 + 已暂存 + 未跟踪”并集必须精确等于上述五路径，重算哈希必须等于获批值。
- `Verify-RepositoryPolicy.ps1` 必须输出 `ok=true`、`violation_count=0`。
- `git diff --check` 与提交后的 `git diff --cached --check` 必须通过。
- 长期状态静态审计不得再把 Issue `#38` 写成等待 PR、Review/CI，不得把 Issue `#46` 写成未恢复，不得把 Gate 5 写成已解锁。
- 本地不运行 Rust 全量编译；最终 PR Head 由 GitHub-hosted CI 完成文档分类、治理与 required 汇总。
- 所有 Review 对话必须解决；最终 Squash Merge 必须针对最终 PR Head 另行取得项目所有者授权。

### 本地轻量验证证据

项目所有者 Windows 本机时间 `2026-07-25 23:50:54 +08:00` 完成验证：

- “未暂存 + 已暂存 + 未跟踪”路径并集精确为 `5` 条，排序后重算仍为 `sha256:dd612ef0c2e5f0f830c40f161b1ef1a85bc58cd1d85458a758c3905ade8db03e`。
- 五路径非法控制字符为 `0`、尾随空白为 `0`，长期状态字段审计通过，Closeout 报告不存在 `TBD`、`TODO` 或占位符。
- `Test-CiScripts.ps1` 输出 `CI_CONTRACT_GREEN passed=32`。
- `Verify-RepositoryPolicy.ps1` 输出 `ok=true`、`violation_count=0`。
- `Verify-ReleaseAuditGate.ps1` 输出 `ok=true`、`status=current`、`requires_reaudit=false`、`blocked_paths=[]`、`errors=[]`。
- `git diff --check` 通过；本地未运行 Rust 全量编译。
- 首轮范围验证曾在 PowerShell 数组字面量中以逗号直接串联三条原生命令，导致后续 `git` 被编组为前一条命令的参数并报 `ambiguous argument`。该问题与 `err.md` 已记录的“PowerShell 原生命令参数编组误解析”同类；处理为分别收集未暂存、已暂存和未跟踪输出后再合并，未修改任何业务或治理文件来掩盖失败。

## 七、Issue 状态合同

- Issue `#16` 是机器维护的自动监控状态，禁止手工编辑。
- Issue `#20` 继续作为上游 `main` 变化预警保持开放；`v1.2.42` 仍是最新正式 Release。
- Issue `#33` 已由 Issue `#34` / PR `#40` 的缓存同步和 Issue `#38` / PR `#45` 的目录重新审计完整消费；本 Closeout PR 合并后必须回写这些证据并按 `COMPLETED` 关闭。
- Issue `#47` 的关联 PR 使用 `Closes #47`；其自身 Review、CI、Squash 与关闭证据保留在 GitHub Issue/PR，不再创建递归 Closeout。

## 八、下一合法工作

下一项可启动工作是 Issue `#32` 的独立性能基线 Discovery：只冻结测量对象、参考来源与许可证、可比环境、精确路径、`scope_hash` 和项目所有者批准。未取得新批准前，不创建实现分支、不写测量文件、不运行本地全量 Rust 编译，也不制定预算、实施优化或启动 Gate 5 产品迁移。
