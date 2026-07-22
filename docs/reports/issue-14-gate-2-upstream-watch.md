# Issue #14：Gate 2 上游变化监控交付报告

report_status: pr-open-final-ci-pending
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/14
branch_ref: codex/issue-14-gate-2-upstream-watch
pr_ref: https://github.com/nonononull/inputcodex/pull/15
ci_ref: https://github.com/nonononull/inputcodex/actions/runs/29889749336
review_ref: https://github.com/nonononull/inputcodex/pull/15#issuecomment-5041714708
merge_ref: pending

## 一、批准范围

- 每 6 小时和手动触发的 GitHub Actions 上游监控。
- 只读取公开上游元数据与冻结 source-lock，只管理带精确机器标记的 Issue。
- 标准 `ubuntu-latest`、最小权限、10 分钟超时、并发取消、无 Rust 编译和 Artifact。
- 执行批准和条件式 Squash Merge 授权来自 Issue `#14` 记录的项目所有者指令。

## 二、Fresh 基线

- `main` 基线：`5e64015075ddf2adef4bf685f50977b47b7f72e7`。
- 最新正式 Release：`v1.2.41`，发布时间 `2026-07-20T01:48:40Z`。
- Release 标签提交：`3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 上游 `main`：`6fa0a57decbb3382771a981247e6922799e97f5d`，仅作变化预警源。

## 三、实现证据

- `.github/scripts/upstream_watch.py` 使用 Python 标准库实现纯比较逻辑、GitHub API 适配器、Issue 状态机与命令入口，不引入 TypeScript、JavaScript、Rust 编译或第三方运行时依赖。
- 监控事件固定为 `new-release`、`release-tag-drift`、`release-metadata-change`、`main-change` 与 `monitor-error`；变化事件使用稳定 SHA-256 指纹，同一物质输入不会因扫描时间变化而产生新指纹。
- 状态与告警 Issue 的机器标记必须精确位于首行；非机器 Issue 永不修改，重复机器标记、重复指纹字段、损坏状态 JSON 或无效 GitHub API 响应均失败关闭。
- 告警写入全部成功后才推进状态 Issue；状态 Issue 被人工关闭时会恢复为 OPEN，告警指纹变化时更新并重新打开，相同指纹不写入。
- GitHub API 主机、目标仓库、上游仓库、Release URL、tag、UTC 时间和 40 位小写 SHA 均使用白名单或严格格式校验；Token 只从环境变量读取，日志和异常摘要执行脱敏。
- `.github/workflows/upstream-watch.yml` 在 PR 中只运行 `validate`，只有定时或手动 `watch` Job 获得 `issues: write`；使用标准 `ubuntu-latest`、10 分钟超时、并发取消、`actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1`、`persist-credentials: false` 和工作区外 Python 缓存目录。该固定提交已 Fresh 核对为签名通过的 `v7.0.1`。

## 四、验证证据

- RED：测试文件先于实现创建，首次运行因 `.github/scripts/upstream_watch.py` 不存在而以明确 `FileNotFoundError` 失败，证明测试真实约束缺失实现。
- 首轮修正：伪造 Issue URL 与生产端精确 URL 校验不一致；动态输入正则误伤只用于 concurrency 的 PR 编号。修正测试夹具和正则边界后，生产安全校验保持不降级。
- 自审补强：增加“关闭的状态 Issue 必须恢复 OPEN”和“Issues API 非对象条目必须失败”合同，修复原实现中状态不恢复及损坏条目静默过滤风险。
- 本地 Fresh：`python -m unittest discover -s .github/scripts/tests -p 'test_*.py' -v` 共 `28` 项通过；`py_compile`、`--validate-only` 与 Workflow YAML 合同均通过。
- 路径门禁首次正确发现仓库内 `__pycache__`；根因是 Python 默认缓存目录。验证入口改用工作区外 `PYTHONPYCACHEPREFIX` 后重新运行，测试与编译通过且未产生未跟踪 `.pyc`。
- Fresh 远端对账仍为 Release `v1.2.41`、标签提交 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`、上游 `main` `6fa0a57decbb3382771a981247e6922799e97f5d`，未触发停止条件。
- 真实 Actions、首次状态 Issue 与第二次幂等运行必须在 Squash Merge 后补充；合并前不得把这些待验证项描述为已通过。

## 五、Review 与合并证据

- PR `#15` 为非 Draft、目标 `main`，首轮 Head 为 `43707034caa2e7b51ec011ce5fbbb61578a0afc3`，远端文件列表与 Session Plan 的 `11` 条允许路径完全一致。
- 首轮 Actions `29889749336` 在该 Head 上完成：`validate=SUCCESS`、`watch=SKIPPED`；这证明 PR 事件只运行只读验证，没有进入 Issue 写入 Job。
- Ruleset `19395456` Fresh 状态为 active、无 bypass actor、required approvals `0`、必须解决 Review 对话且只允许 Squash Merge；PR 自动合并未启用，首轮状态为 `MERGEABLE/CLEAN`。
- 首轮 Review 对话为 `0`；Owner Review 跟踪锚点为 `https://github.com/nonononull/inputcodex/pull/15#issuecomment-5041714708`，当前明确标记为“尚未最终通过”。
- 本次文档回写提交后必须重新核对最终 Head、最终 CI、Review 对话和 Fresh 上游基线，再把同一 Owner Review 评论原位更新为最终通过证据。
- Squash Merge、两次真实运行、Issue 关闭和签名证据尚未发生，不得提前填充或解释为已完成。
- 项目所有者已通过 `user-message:create-upstream-watch-full-delivery-2026-07-22` 提供执行批准和门禁满足后的条件式 Squash Merge 授权；该授权不能替代 CI、Review 对话闭环或合并前 Fresh 核对。
