# Issue #63：非 required `approved-observation` 预算 CI 实施计划

## 任务元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/63`
- 基线：`15e91708b41548f523e26ede4c7ca4de41badf77`
- 分支：`codex/issue-63-performance-budget-ci-observation`
- 方案：A，在现有 `Performance Baseline` 增加 `observation` 模式
- 已批准范围：十三路径
- `scope_hash`：`sha256:d5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc`
- 实施批准：`https://github.com/nonononull/inputcodex/issues/63#issuecomment-5087488361`
- 最终 Squash Merge：保留项目所有者对最终 PR Head 的单独授权门

## 目标

1. 自动 `pull_request` / `push` 使用 `observation`，不再用历史 Issue `#32` Evidence 阻断合法的 `parity/**` 变化。
2. Windows 与 macOS 在标准 GitHub-hosted Runner 上生成当前 Head 临时测量，只读比较 `benchmarks/budgets/issue-59-approved-observation.json`。
3. `within-budget`、`warning-observed`、`blocking-observed`、`not-comparable` 都是非阻断观察分类。
4. 预算文件缺失或哈希漂移、schema/status 错误、平台/单位/指标/样本合同/checksum 错误必须失败。
5. 成功 observation 不上传 Artifact；失败只上传最小诊断，保留七天且禁止 `target/`。

## 架构

- `Invoke-InputcodexBudgetObservation.ps1` 负责单平台只读验证、指标提取、环境可比性判断和阈值分类；不写预算、不改结果、不执行构建。
- `Test-InputcodexBudgetObservation.ps1` 使用合成 fixture 完成 RED→GREEN，覆盖四种分类与合同失败。
- `Performance Baseline` 在 observation 模式先复用 `Invoke-InputcodexBaseline.ps1` 生成当前 Head 临时结果，再调用观察器；阈值分类不改变退出码，合同错误返回非零。
- `scripts/ci/Test-CiScripts.ps1` 固定 mode、Runner、权限、超时、Action SHA、Artifact 和非 required 边界。

## 关键语义

- 可比性只由平台、GitHub-hosted、固定测量合同和预算保存的完整环境指纹决定；预算中的历史 hard key 含旧 `input_sha256`，只能作为来源证据，不能让所有合法代码变化永久变成 `not-comparable`。
- 同平台且完整环境指纹一致时比较十个候选：首次 view、空闲 Working Set 与三个 Rust 场景的 median/p95。
- 任一候选达到 blocking limit 时分类 `blocking-observed`；否则任一达到 warning limit 时为 `warning-observed`；其余为 `within-budget`。
- 环境指纹不一致时分类 `not-comparable`，不得伪造预算回归。

## TDD 顺序

1. **RED**：先创建观察器测试，确认因实现脚本不存在而失败。
2. **GREEN-合同**：实现预算哈希、schema、平台、单位、唯一指标、样本合同和 checksum 验证。
3. **GREEN-分类**：实现四种非阻断分类和紧凑 JSON 输出。
4. **GREEN-Workflow**：自动模式改为 observation，双平台测量并观察，成功 Artifact 为零。
5. **VERIFY**：运行观察器测试、CI 合同、仓库政策、范围哈希和空白检查。

## 精确十三路径

```text
.github/workflows/performance-baseline.yml
AGENTS.md
build.md
docs/plans/2026-07-26-issue-63-performance-budget-ci-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-63-performance-budget-ci-observation.md
docs/reports/issue-63-performance-budget-ci-observation.md
docs/workflows/2026-07-26-issue-63-performance-budget-ci-observation-runtime.md
err.md
README.md
scripts/ci/Test-CiScripts.ps1
scripts/performance/Invoke-InputcodexBudgetObservation.ps1
scripts/performance/Test-InputcodexBudgetObservation.ps1
```

## 交付顺序

1. Issue `#63` 实现 PR 在当前 `main` 上完成 TDD、标准 CI 与双平台真实 observation。
2. 本 PR 使用 `Refs #63`，合并后 Issue `#63` 保持开放，等待 `release_audit=current` 的主干验收。
3. 重新触发 PR `#72` 同一 Final Head；新的自动 observation 必须双平台执行成功，Artifact 为零。
4. PR `#72` 经单独授权 Squash Merge 后，`main` 因 `parity/**` 变化自动运行 observation；确认 `release_audit=current`、双平台成功和 Artifact 为零后关闭 Issue `#63`。

## 停止门

- 十三路径集合或 `scope_hash` 漂移。
- 需要修改预算 JSON、预算数值/公式、历史 Evidence、Ruleset、required checks、产品行为、Gate 5、`upstream/` 或 AGOS。
- 阈值分类导致非零退出码。
- observation 成功 Run 上传 Artifact，或失败 Artifact 包含 `target/`。
- Review/CI 失败但根因未解决，或最终 Squash Merge 尚未单独授权。

## 当前状态

RED 已于 Windows 本机时间 `2026-07-27 13:21:15 +08:00` 确认，并以提交 `650040763aff07f4884ee9252c50639469622934` 普通推送。最小观察器、自动 observation Workflow 与 CI 合同已以初始 GREEN 提交 `b465c660d0401ff4bff37673671147aa6b513e1a` 普通推送；自审追加“预算锁字段缺失必须失败”回归后，本地结果为观察器 `12/12`、CI 合同 `35/35`、仓库政策零违规、`git diff --check` 通过。下一步完成十三路径最终验证、创建非 Draft PR，并取得 Windows/macOS 真实 observation 与 Review/CI 证据；最终 Squash Merge 仍未授权。
