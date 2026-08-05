# Issue #161 嵌套 snapshot 对象边界报告

## 当前状态

- state: `LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`
- baseline: `f3e7d6f873f59399e71b602e1a9fbdee71760d64`
- scope: `8`
- scope_hash: `sha256:8f336603e25d05f96a45c9e57e0adb4724ebbfe19e6194097cc780d036d1ce40`
- product_delivery: `0`

## 已确认根因

共享 `Get-PropertyValue` 通过 PowerShell 输出管道返回 `.Value`，会把单元素对象数组展开为内部对象。两个
gate 若先使用该 getter，就无法辨认 `evidence`、`planning_evidence`、`review_attestation` 与
`post_merge` 的原始 JSON 容器形状。

## 修复

- 两个 gate 对四个命名属性使用 `Get-PropertyProjection` 保留原始值。
- 只接受具体 `System.Management.Automation.PSCustomObject`。
- 不修改共享 getter，不扩大其他 snapshot schema。
- task kind 使用直接属性赋值，避免 `if` 语句输出再次展开单元素数组。

## 本地证据

- RED：既有 `85` 个合同通过，新增 `9` 个合同精确复现旁路。
- GREEN：规范对象接受，单元素、空、多元素数组及缺失属性全部 fail closed；`CI_CONTRACT_GREEN passed=94`。
- 两份 PowerShell AST：零错误。
- 产品、Parity、Cargo、Workflow、Release 与 upstream：零改动。

## 待完成证据

- Final Head 独立复审：pending
- Hosted CI / Performance / Artifact：pending
- Squash 与 fresh main：pending

本报告不得在证据形成前宣称通过、合并、Gate 5 完成或启动 admission matrix successor。
