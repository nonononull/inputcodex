# Issue #63：预算 CI Observation 报告

## 当前结论

方案 A 与十三路径已获批准。观察器、自动 observation Workflow 与 CI 合同已完成本地 RED→GREEN；预算 JSON、预算数值、公式、历史 Evidence、Ruleset、产品与 Gate 5 均保持只读。当前等待十三路径最终验证、非 Draft PR、双平台 hosted observation 与 Review/CI。

## 已确认根因

PR `#72` 的 `Performance Baseline` 失败只来自旧 Issue `#32` Evidence 的 `input_sha256` 与当前 `parity/**` 输入不一致；产品 `implementation_sha256` 未变化。自动事件继续使用 Evidence 会永久阻断合法目录更新，因此必须改为当前 Head 临时测量加只读 observation。

## 固定预算

- 路径：`benchmarks/budgets/issue-59-approved-observation.json`
- 归一化 SHA-256：`sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5`
- `status=approved-observation`
- `budget_ci_enabled=false`
- `gate_5_unlocked=false`

## 实施状态

```yaml
status: local-green-hosted-pending
red_test: observer implementation missing at Windows local time 2026-07-27 13:21:15 +08:00; expected failure confirmed
red_commit: 650040763aff07f4884ee9252c50639469622934
green_commit: b465c660d0401ff4bff37673671147aa6b513e1a
green_test: BUDGET_OBSERVATION_GREEN passed=12
workflow_contract: CI_CONTRACT_GREEN passed=35
repository_policy: ok=true violation_count=0
diff_check: passed
scope_verification: 13 paths, sha256:d5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc
session_plan: SESSION_PLAN_VERIFY_OK
stored_windows_observation: within-budget, observations=10
stored_macos_observation: within-budget, observations=10
actionlint: not-installed; GitHub-hosted workflow parse pending
hosted_observation: pending
review_ci: pending
final_merge: not-authorized
```

## 已实现语义

- 手工模式保留默认 `evidence`，并允许显式 `measure` 与 `observation`；自动 PR/push 固定进入 `observation`。
- Windows/macOS observation 复用当前 Head 临时测量，只读消费固定预算哈希；成功只写日志和 Step Summary，不上传 Artifact。
- `within-budget`、`warning-observed`、`blocking-observed`、`not-comparable` 均退出 `0`；合同错误输出结构化 `violations` 并退出非零。
- `budget_ci_enabled` 与 `gate_5_unlocked` 必须作为显式布尔字段存在且均为 `false`；缺失、类型错误或被启用都以 `BUDGET_GUARD_INVALID` 阻断。
- 可比性只使用 GitHub-hosted、固定样本合同与完整环境指纹，不使用包含历史 `input_sha256` 的 hard key 阻断合法代码变化。
