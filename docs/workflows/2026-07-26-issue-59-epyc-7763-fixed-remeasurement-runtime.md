# Issue #59 Runtime Workflow：EPYC 7763 四次固定串行复测

## Phase 0：启动与隔离

1. 使用 Windows `Get-Date` 记录本机时间。
2. 确认 worktree 位于 `C:\Users\dashuai\Documents\inputcodex-worktrees\issue-59-epyc-7763-fixed-remeasurement`，分支为 `codex/issue-59-epyc-7763-fixed-remeasurement`，基线为 `d9d1ed77b9796ac6a99e250d1547217a39426aa9`。
3. 确认 Issue `#59` 开放、Issue `#57` 已按 `COMPLETED` 关闭、Issue `#54` 未被恢复。
4. 运行 AGOS 与 Git snapshot ReportOnly；`needs-input/unregistered` 只记录并绕过。

## Phase 1：冻结控制面与历史缓存

1. 写入三十八路径和 `scope_hash=sha256:d0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570`。
2. 通过 `apply_patch` 从 Issue `#54` 分支缓存 manifest 与十六份原始 JSON；不得重新序列化或修改原始分类。
3. 验证历史 manifest 归一化 SHA-256 为 `sha256:72567fe96f61d19d4eca8a5347e3d3fcea7df823975946ec3f464a43d229f1ae`，并逐份复算 manifest 中记录的结果哈希。
4. 创建 Issue `#59` 初始 manifest，固定目标处理器、四槽位和硬停止条件。
5. 运行 `build.md` 的控制面轻量命令，普通提交并普通 push；该提交是四次测量共同基线。

## Phase 2：每个槽位的串行测量循环

对 `run-01`、`run-02`、`run-03`、`run-04` 依次执行，禁止并发和跳号：

```powershell
gh workflow run 'Performance Baseline' --repo nonononull/inputcodex --ref codex/issue-59-epyc-7763-fixed-remeasurement -f mode=measure
```

1. 记录触发前分支 Head；从 Actions API 取得新 `run_id`，确认事件为 `workflow_dispatch`、分支和 Head 一致。
2. 使用 `gh run watch <run_id> --repo nonononull/inputcodex --exit-status` 等待终态；只有 `contract`、`windows`、`macos`、`required` 全部成功才进入 Artifact 缓存。
3. 下载成功 Artifact 到仓库外临时目录；只把双平台原始 JSON 通过 `apply_patch` 写入当前固定槽位。
4. 核验平台、`status=complete`、`runner_environment=github-hosted`、source commit/tree、配置/实现/输入哈希、样本合同、checksum、Artifact ID、名称、Run 和 attempt。
5. Windows 处理器精确等于 `AMD EPYC 7763 64-Core Processor` 时分类为目标队列 `comparable-valid`；其他完整结果分类为 `new-cohort-valid`。不得改写 Issue `#54` 原始分类。
6. macOS 与历史目标硬键和环境指纹匹配时分类为 `comparable-valid`，否则建立自己的新队列。
7. 更新 manifest 与报告，运行轻量验证；普通提交并 push 后，删除该 Run 的成功 Artifact，再确认 Artifact 数为 `0`。
8. 当前槽位完全闭环后才进入下一槽位；即使提前达到五份目标样本，也必须执行剩余固定槽位。

## Phase 3：四次后的确定性判定

1. 汇总 Issue `#54` 的四个历史 EPYC 7763 Windows 样本与 Issue `#59` 的新命中样本；macOS 独立汇总全部同队列有效样本。
2. 若 Windows 目标队列和 macOS 均至少五次：按 ADR `0004` 计算 median-of-medians、median-of-P95、min、max、MAD、warning 与 blocking，创建预算 JSON、构建脚本和验证脚本。
3. 若 Windows 目标队列仍不足五次：状态固定为 `STOPPED_AFTER_FOUR_RUNS_INSUFFICIENT_TARGET_COHORT`，不创建三个可选文件，不创建 `run-05`。
4. 本 Issue 无论哪条分支都不修改 `.github/workflows/`，不实施预算 CI。

## Phase 4：Fresh 验证与 PR

1. 运行 `build.md` 中 Issue `#59` 的范围、历史证据、manifest、Evidence、CI 合同、Repository Policy、JSON/PowerShell 和空白验证。
2. 运行 Git snapshot ReportOnly，形成最终普通提交并普通 push。
3. 创建 `Closes #59` 的非 Draft PR；正文列出四个 Run、CPU、分类、Artifact 删除、统计或硬停止结果。
4. 所有 Review 对话必须写明根因、处理方式和验证证据；PR CI 全部通过后等待 Final Head Squash Merge 授权。

## 禁止面

- `run-05`、Issue `#54/run-09`、选择性停止、选择性删除、混合 CPU 队列、追溯改写原始标签。
- TypeScript、WebView、产品功能、性能优化、预算 CI、Ruleset、Release、上游或 AGOS 改动。
- force push、rebase merge、merge commit、删除 `main`。
