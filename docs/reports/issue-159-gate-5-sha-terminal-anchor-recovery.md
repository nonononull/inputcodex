# Issue #159 SHA 绝对终止恢复报告

## 当前状态

- state: `LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`
- baseline: `f3e7d6f873f59399e71b602e1a9fbdee71760d64`
- scope: `8`
- scope_hash: `sha256:f56d3b5f45794641ddb1186b485bdc23c9c583068d15dbae61ce8f958225bddf`
- product_delivery: `0`

## 已确认根因

原始标量类型校验不能证明字符串词法规范；.NET `$` 允许在末尾单个 LF 前匹配。当 `expected_base` 与 `parent_oid` 同时携带同一尾随换行时，精确字符串相等反而掩盖了非法形状。

## 实现

- snapshot `expected_base` 使用 `\A[0-9a-f]{40}\z`。
- post-merge collector 对每个 parent SHA 使用同一绝对终止合同。
- post-merge gate 独立验证 `expected_base` 与 `parent_oid` 的规范 SHA，再做精确相等比较。
- 手工重建 Review ref 绝对终止、当前 PR 编号、原始 `expected_base/parent_count` 类型与唯一 `parent_oid` 绑定。

## 本地证据

- RED：九个精确合同失败；新增三项分别命中 snapshot、collector 与 gate。
- GREEN：`CI_CONTRACT_GREEN passed=93`。
- 两份 PowerShell AST：零错误。
- 实际变更限制在冻结八路径；产品交付与目录计数均为零。

## 待完成证据

- Final Head 双路独立复审：pending
- Hosted CI / Performance / Artifact：pending
- Squash 与 fresh main：pending

任一复审 Critical/Important 必须立即 hard stop；本报告不得在证据形成前宣称通过或合并。
