# Issue #14：Gate 2 上游变化监控实施计划

> 本计划由 Issue `#14` 批准，执行时以项目原生 Session Plan 与 Runtime Workflow 为准。

## 目标

在标准 GitHub-hosted `ubuntu-latest` Runner 上建立每 6 小时执行一次、并支持手动触发的上游变化监控。监控只读取 `BigPizzaV3/CodexPlusPlus` 和本仓库冻结基线，只管理带 inputcodex 机器标记的 GitHub Issue，不修改快照、产品源码、Release 或 Ruleset。

## 架构

- `.github/scripts/upstream_watch.py` 使用 Python 标准库实现 GitHub API 适配器、输入校验、纯比较决策和 Issue 幂等写入。
- `upstream/source-lock.json` 是已导入 Release 的冻结基线；上游最新正式 Release 是功能真源，上游 `main` 仅作为变化预警源。
- 单一状态 Issue 保存最近成功观察值；告警 Issue 使用稳定机器标记和事件指纹去重。脚本只更新标记完全匹配且唯一的 Issue，遇到重复标记或不完整响应立即失败。
- Workflow 使用 `pull_request`、`17 */6 * * *` 与 `workflow_dispatch`：PR 只运行只读验证，只有定时/手动 `watch` Job 获得 `issues: write`；所有 Job 使用标准 Runner、并发取消和有限超时，不执行 Rust、Cargo、Artifact 或仓库写入。

## 全局约束

- 软件名称固定为 `inputcodex`。
- 禁止广告、推广、付费导流和隐蔽遥测。
- 禁止 TypeScript、JavaScript 业务代码和 WebView。
- 不修改 `upstream/`、`upstream/source-lock.json`、Ruleset、Release、产品源码或 AGOS。
- 只使用标准 GitHub-hosted Runner；禁止 self-hosted、Larger Runner 和付费资源。
- 所有失败必须保留可诊断证据，不得把 API、权限或格式异常解释为“无变化”。
- 所有 Review 对话必须确定根因、处理并完成 Fresh 验证后才能解决。
- 合并方式只允许 Squash Merge，禁止 Force Push 和删除 `main`。

## 文件结构

- `.github/scripts/upstream_watch.py`：纯比较逻辑、GitHub API 客户端、Issue 状态机和命令入口。
- `.github/scripts/tests/test_upstream_watch.py`：无网络单元测试与 Workflow 静态合同测试。
- `.github/workflows/upstream-watch.yml`：定时与手动触发入口。
- `docs/plans/sessions/2026-07-22-issue-14-gate-2-upstream-watch.md`：会话批准范围和允许路径。
- `docs/workflows/2026-07-22-issue-14-gate-2-upstream-watch-runtime.md`：执行阶段、验证命令与停止条件。
- `docs/reports/issue-14-gate-2-upstream-watch.md`：PR、CI、Review、合并和运行证据。
- `README.md`、`build.md`、`err.md`、`docs/plans/PROJECT-MASTER-PLAN.md`：项目入口与长期维护说明。

## Task 1：控制面与失败合同

- [x] 创建 Issue `#14`、分支、Session Plan、Runtime Workflow 和初始报告。
- [x] 先写无网络测试，覆盖相同 Release 无告警、首次状态初始化、新 Release、标签漂移、元数据变化、`main` 前进、重复输入指纹稳定、非机器 Issue 不修改、重复机器标记失败和 Workflow 最小权限合同。
- [x] 在实现不存在时运行测试，确认以明确缺失模块失败。

## Task 2：最小监控实现

- [x] 实现严格解析 `upstream/source-lock.json` 与 GitHub API 响应，校验仓库名、Release tag、40 位提交 SHA、时间和 URL。
- [x] 实现纯比较函数；相同输入必须产生相同事件键与指纹。
- [x] 实现分页读取 Issue、精确机器标记匹配、重复标记拒绝和相同指纹无写入。
- [x] 状态写入只能发生在所有告警写入成功之后，避免失败扫描吞掉变化。
- [x] 监控异常时尽力更新唯一异常 Issue，然后退出非零；日志不得包含 Token 或完整请求头。

## Task 3：Workflow 与本地验证

- [x] 创建 `.github/workflows/upstream-watch.yml`：`pull_request` 只运行只读验证 Job，`schedule` 与 `workflow_dispatch` 才运行 Issue 写入 Job。
- [x] 固定 GitHub 官方 checkout Action 到不可变提交；不使用第三方 Action。
- [x] 运行 Python 单元测试、编译检查、Workflow YAML 解析、受控路径检查和 `git diff --check`。
- [x] 更新 `README.md`、`build.md`、`err.md`、Master Plan 和报告。

## Task 4：PR、真实运行与合并

- [ ] 普通 push 功能分支并创建关联 PR，禁止自动合并。
- [ ] 观察 PR 上的 Workflow 语法和仓库门禁；定时 Workflow 不作为 `main` required check。
- [ ] 合并前通过只读 PR `validate` Job；合并后在 `main` 连续手动运行两次，验证 Issue 写权限、状态 Issue 唯一性和重复运行幂等性。
- [ ] 完成所有 Review 对话的根因闭环，核对 PR Head、Checks、Ruleset 和允许路径。
- [ ] 使用 Issue `#14` 中的项目所有者条件式授权执行 Squash Merge，关闭 Issue，并回写最终合并与运行证据。

## 回滚

若 Workflow 因 GitHub API、权限或错误去重造成持续失败，建立事故 Issue，并通过新的受审 PR 禁用或删除 `.github/workflows/upstream-watch.yml`；保留历史状态与告警 Issue，不修改 `upstream/source-lock.json`，不 Force Push，不删除 `main`。

## 完成定义

- Issue、分支、计划、实现、测试、PR、Review/CI、Squash Merge 和关闭证据齐全。
- 首次运行建立且只建立一个状态 Issue；相同输入重复运行不创建重复告警。
- Release、标签、元数据和 `main` 变化具有确定、可复核、可去重的 Issue 语义。
- 异常明确失败且不泄露 Token；所有修改路径均在 Session Plan 允许范围内。
- Gate 2 标记完成，但 Gate 3 继续锁定，直到项目所有者批准新的 Session Plan。
