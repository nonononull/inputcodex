# Issue #14 Runtime Workflow：Gate 2 上游变化监控

workflow_status: active
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/14
session_plan_ref: docs/plans/sessions/2026-07-22-issue-14-gate-2-upstream-watch.md
approved_decision_ref: user-message:create-upstream-watch-full-delivery-2026-07-22

## Phase 1：startup-baseline

1. 确认分支为 `codex/issue-14-gate-2-upstream-watch`，基线为 `main@5e64015075ddf2adef4bf685f50977b47b7f72e7`。
2. 确认 Issue `#14` 为 OPEN，标签为 `type:upstream-watch` 与 `gate:2`。
3. Fresh 读取上游最新正式 Release、标签提交和 `main`，与 Session Plan 基线对账。
4. 确认工作树变更只在允许路径中。

## Phase 2：red-contract

1. 创建 `.github/scripts/tests/test_upstream_watch.py`，先覆盖纯比较、状态初始化、事件指纹、Issue 标记安全和 Workflow 静态合同。
2. 运行：

   ```powershell
   python -m unittest discover -s .github/scripts/tests -p 'test_*.py' -v
   ```

3. 预期在模块或 Workflow 尚不存在时失败；把失败原因记录为测试先行证据，不把语法或环境错误当成合格 RED。

## Phase 3：minimal-implementation

1. 实现 `.github/scripts/upstream_watch.py`，保持纯比较逻辑与 GitHub API 适配器分离。
2. 实现严格输入校验、分页读取、精确机器标记、重复标记拒绝、稳定事件指纹和相同指纹无写入。
3. 状态 Issue 只在全部告警写入成功后更新；异常尽力写入唯一异常 Issue并退出非零。
4. 创建 `.github/workflows/upstream-watch.yml`：
   - `pull_request`：只运行无网络合同、Python 编译与冻结基线验证，不授予 Issue 写权限；
   - `schedule: 17 */6 * * *`；
   - `workflow_dispatch`；
   - `ubuntu-latest`
   - `contents: read`
   - `issues: write`
   - `concurrency.group: upstream-monitor-${{ github.event.pull_request.number || github.ref }}`，同一 PR 或同一运行引用互斥；
   - `cancel-in-progress: true`
   - `timeout-minutes: 10`

## Phase 4：local-verification

依次运行：

```powershell
python -m unittest discover -s .github/scripts/tests -p 'test_*.py' -v
python -m py_compile .github/scripts/upstream_watch.py .github/scripts/tests/test_upstream_watch.py
python -c "import pathlib,yaml; yaml.safe_load(pathlib.Path('.github/workflows/upstream-watch.yml').read_text(encoding='utf-8')); print('WORKFLOW_YAML_OK')"
git diff --check -- .github/scripts .github/workflows README.md build.md err.md docs
```

再使用 Session Plan 的允许路径集合核对 `git diff --name-only main...HEAD` 与未提交路径，结果必须为零越界。

## Phase 5：commit-push-pr

1. 精确暂存允许路径并提交，推荐提交主题：`ci: 建立上游变化监控`。
2. 普通 push 分支；禁止 Force Push。
3. 创建关联 Issue `#14` 的非 Draft PR，明确 `0 Checks` 与实际 CI 状态，不把未配置检查说成通过。
4. 在 PR 和 Issue 回写 Head、验证命令、允许路径和所有者条件式合并授权。

## Phase 6：review-and-actions

1. 检查 PR 文件、Review 对话、Ruleset、合并状态和 Actions 运行。
2. PR 通过只读 `validate` Job 形成 GitHub Actions 检查；`watch` Job 在 PR 事件中必须为 skipped，且不得获得 Issue 写权限。
3. Squash Merge 后立即在 `main` 手动运行两次：首次建立单一状态 Issue，第二次复用该 Issue且不创建重复告警。
4. 每个失败先查 `err.md`，确定根因、修复并 Fresh 重验；Review 对话按根因、处理、验证、确认四项闭环。
5. 若合并后真实运行发现根因缺陷，立即建立事故 Issue 和修复 PR；不得把失败运行解释为完成，也不得 Force Push 回退 `main`。

## Phase 7：squash-merge-closeout

1. Fresh 核对 PR Head、允许路径、Checks、Review 对话、mergeable 状态、Ruleset和项目所有者授权。
2. 以 Squash Merge 合并；禁止 Merge Commit、Rebase Merge和自动合并。
3. 在 `main` 手动执行两次 Workflow，确认成功、状态 Issue 唯一和零重复告警。
4. 确认 Issue `#14` 关闭、`main` 指向新的单父合并提交、分支删除状态和 GitHub 签名。
5. 将 PR/Issue/Actions 最终证据写入 GitHub 评论；合并后的动态 merge SHA 和运行链接不创建递归 closeout PR。

## 停止条件

- 上游正式 Release 或冻结基线发生变化，需要建立独立 upstream-sync 或一致性决策。
- 真实 Workflow 请求额外写权限、密钥、付费 Runner、Artifact 或仓库内容写入。
- 允许路径越界、测试不稳定、异常被静默吞掉、Review 对话未闭环或 Ruleset 不允许 Squash Merge。
- 需要进入 Gate 3、迁移功能、实现 UI 或修改 AGOS。
