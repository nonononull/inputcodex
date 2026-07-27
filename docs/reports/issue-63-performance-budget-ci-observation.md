# Issue #63：预算 CI Observation 报告

## 当前结论

方案 A 与十三路径已获批准。任务正在 TDD 实施阶段；预算 JSON、预算数值、公式、历史 Evidence、Ruleset、产品与 Gate 5 均保持只读。

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
status: red-confirmed
red_test: observer implementation missing at Windows local time 2026-07-27 13:21:15 +08:00; expected failure confirmed
green_test: pending
workflow_contract: pending
hosted_observation: pending
review_ci: pending
final_merge: not-authorized
```
