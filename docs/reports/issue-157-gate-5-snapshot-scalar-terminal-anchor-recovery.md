# Issue #157 snapshot 标量与 Review ref 终端边界报告

## 当前状态

- state: `LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`
- baseline: `f3e7d6f873f59399e71b602e1a9fbdee71760d64`
- scope: `8`
- scope_hash: `sha256:caeb53933c58fc8d5ddf1a90af6274bad8ba1f9292fbaf4bcb4536ea721a41dd`
- product_delivery: `0`

## 已确认根因

Review comment ref 使用 `$` 结尾时会接受末尾单个 LF；`expected_base` 与 `parent_count` 经 `Get-PropertyValue` 返回时，PowerShell 又会把单元素数组枚举为标量。PR `#156` 未合并，因此 fresh main 还缺少当前 PR 编号和唯一父 SHA 两项基础绑定。

## 实现

- Review ref 使用大小写敏感的 `\A...\z`，并绑定当前 PR 编号。
- collector、merge gate 与 post-merge gate 共享同一 helper。
- `expected_base`、`parent_count`、Review ref 与 `parent_oid` 使用原始 property projection。
- post-merge collector 逐父对象验证规范 SHA，只在单父时返回字符串 `parent_oid`。

## 本地证据

- RED：六个精确合同失败，无无关失败。
- GREEN：`CI_CONTRACT_GREEN passed=90`。
- 两份 PowerShell AST：零错误。
- 实际变更限制在冻结八路径；产品交付与目录计数均为零。

## 待完成证据

- Final Head 双路独立复审：pending
- Hosted CI / Performance / Artifact：pending
- Squash 与 fresh main：pending

任一复审 Critical/Important 必须立即 hard stop；本报告不得在证据形成前宣称通过或合并。
