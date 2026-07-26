# Runtime Workflow：Issue #54 五次性能复测与数值批准

## 1. 元数据

```yaml
workflow_id: issue-54-performance-remeasurement-budget-approval-runtime
task_id: issue-54-performance-remeasurement-budget-approval
issue: 54
branch: codex/issue-54-performance-remeasurement-budget-approval
base_commit: 325bb2419548bc076502065dc583f54f4fddd582
scope_hash: sha256:32c818aaf99efe550e9afc45d5871f9dceeef50ba314aaa91b617cd76158a38e
approved_decision_ref: user-message:按你推荐来-不用我二次批准-直接安排后续-2026-07-26
delivery_contract: issue -> branch -> session-plan -> runtime-workflow -> serial-hosted-samples -> offline-statistics -> validation -> pr -> review-ci -> squash-merge
```

## 2. 启动前检查

```powershell
$branch = git branch --show-current
if ($branch -ne 'codex/issue-54-performance-remeasurement-budget-approval') { throw "分支不正确：$branch" }
git status --short --branch
git merge-base --is-ancestor 325bb2419548bc076502065dc583f54f4fddd582 HEAD
if ($LASTEXITCODE -ne 0) { throw '当前分支未建立在 Issue #55 合并后的 main。' }
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
```

启动阶段只允许计划/Session/Runtime/控制面文档写入。先普通提交并推送，使 hosted 样本的 `source.commit` 固定到可复核 Head；禁止本机运行桌面 Release、完整 Workspace 或真实性能采样。

## 3. 串行 hosted 采样

每次只运行以下一条触发命令；下一次必须等待当前 Run 的四个 Job 成功或完成分类后才可开始：

```powershell
gh workflow run 'Performance Baseline' `
  --repo nonononull/inputcodex `
  --ref codex/issue-54-performance-remeasurement-budget-approval `
  -f mode=measure
```

对每次新 `run_id` 执行：

1. `gh run view <run-id>` 核验 `workflow_dispatch`、精确 branch Head、`contract`、`windows`、`macos`、`required` 四 Job。
2. 使用 Actions API 核验恰好两个成功临时 Artifact，名称为 `performance-windows-<run-id>-1` 与 `performance-macos-<run-id>-1`，不得包含 `target/`。
3. 下载 Artifact 到项目外临时目录；每个平台 JSON 必须满足 `status=complete`、`environment.runner_environment=github-hosted`、同一 `github.run_id`、同一 source commit/tree、有效 schema 和真实 Artifact ID。
4. 计算 LF 归一化 SHA-256；只经 `apply_patch` 缓存到下一个未使用的固定 `run-01` 至 `run-08` 槽位，并在 `manifest.json` 写入原始 run、attempt、Artifact、哈希、环境、队列、分类和处理证据。
5. 若 Job 失败，保留 GitHub URL、attempt 与失败日志索引；只有外部服务证据充分时分类 `external-incident`，否则 `evidence-invalid`。不重跑同一 attempt 伪造新样本。

达到每个平台五个 `comparable-valid` 后停止触发；达到第八次仍不足五个时停止本 Issue 的数值生成。

## 4. 队列和分类

每个平台独立计算：

```text
hard_key = schema_version + platform + runner_arch + runner_environment
         + image_os + rust_release_and_host + config_sha256 + input_sha256
         + sample_contract
fingerprint = image_version + os_description + processor
            + logical_processor_count + total_memory_bytes
```

- 第一个证据完整的 Run 建立该平台候选队列。
- `hard_key` 或 `fingerprint` 改变时标记 `new-cohort-valid`，保留原始结果但不纳入旧队列。
- schema、成功状态、哈希、checksum、Artifact、双平台 source 或样本合同失配时标记 `evidence-invalid`。
- 可复核 GitHub 外部故障才标记 `external-incident`。
- 同平台相同 hard key/fingerprint 的完整结果标记 `comparable-valid`；IQR 标记和数值波动只能记录，不能删除。

## 5. 离线统计与数值控制面

`Build-InputcodexBudgetApproval.ps1` 只能读取 `benchmarks/results/issue-54/manifest.json` 和缓存的 JSON，输出确定性 JSON，不访问网络、不执行 Rust、不修改 CI。

```text
center = median(run_level_values)
mad = median(abs(value - center))
warning = round_up(center + max(3 * mad, 10% * center), quantum)
blocking = round_up(center + max(5 * mad, 20% * center), quantum)
```

预算数据必须为每个平台的首次 view、Working Set 和三个 Rust 场景分别保存 median/P95 lane；还须保存 min/max/MAD、全部来源 run、队列指纹、单位、quantum、安全裕量、公式版本和预授权引用。CPU 与桌面二进制大小只记录为观察，构建元数据只记录为诊断。

`Test-InputcodexBudgetApproval.ps1` 复算上述统计和公式，并拒绝：少于五个样本、重复 run、混合队列、`evidence-invalid` 进入统计、checksum 不一致、丢失 IQR、数值/单位/裕量不匹配、把观察或诊断指标写为阻断预算，或 `budget_ci_enabled=true`。

## 6. 本地验证与范围审计

```powershell
pwsh -NoProfile -File scripts/performance/Test-InputcodexBudgetApproval.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

范围审计必须把 `325bb2419548bc076502065dc583f54f4fddd582...HEAD` 的已提交差异、未暂存、已暂存和未跟踪路径合并后与二十九路径集合比较，并重算 `sha256:32c818aaf99efe550e9afc45d5871f9dceeef50ba314aaa91b617cd76158a38e`。

## 7. PR、Review 与停止

创建 `Closes #54` 的非 Draft PR，PR 说明必须包含每个 Run 的 GitHub URL、分类、队列、统计、数值、安全裕量和预授权。Review 对话必须记录根因、处理和验证；所有 PR CI 通过后只能 Squash Merge。不得 Force Push、删除 `main`、删除分支、修改预算 CI、优化或 Gate 5。

遇到第九个样本需求、队列混合、不能解释的 hosted 失败、范围漂移或任何禁止面需求时，停止并建立新的 Issue/范围，不通过删除样本、修改历史或放宽验证继续。

## 8. 实际停止记录

运行至 `run-08` 后，Windows 只有 `2` 个初始队列 `comparable-valid` 样本，macOS 有 `8` 个；所有八次 Artifact 完整且远端核验成功。运行时工作流已按停止分支结束：没有触发 `run-09`，没有创建预算验证器或预算 JSON，后续只建立 Issue `#57` 的 Discovery，不在本工作流中修订队列合同。
