# Issue #151 StrictJsonObject 根类型边界报告

## 当前状态

- state: `LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`
- baseline: `c26e97ee534b74ebe1252346477640dc196f89b9`
- scope: `9`
- scope_hash: `sha256:3ea6c5882d1e5c96708a4e9cdc837ba4c67209a49ebc45d9ad19b991efe7301c`
- product_delivery: `0`

## 已确认根因

`ConvertFrom-StrictJsonObjectOutput` 已使用 `ConvertFrom-Json -NoEnumerate`，但根类型仍通过
`[pscustomobject]` 类型加速器判断。该加速器映射到宽泛 `PSObject`，使 `System.Object[]` 也通过检查，
因此三种数组根可进入原本只接受 object 的 post-merge commit 与 repository settings 读取路径。

## 本地证据

- RED：既有 `84` 个合同通过，唯一新增生产 helper 合同失败。
- GREEN：合法 `{}` 接受，`[]`、`[{}]`、`[{},{}]` 均拒绝，`CI_CONTRACT_GREEN passed=85`。
- 生产改动：仅把一个宽泛类型判断替换为具体 `System.Management.Automation.PSCustomObject`。
- PowerShell AST：两份脚本零错误。
- Release Audit：`current / requires_reaudit=false / errors=[]`。
- 自治策略与仓库政策：零违规。
- live：`active-worktree-execution / resume-worktree`，active Issue `151`，selected candidate `null`。
- scope：精确九路径，hash 与 Planning Freeze 一致，`git diff --check` 通过。

## 待完成证据

- Final Head 独立复审：pending
- Hosted CI / Performance / Artifact：pending
- Squash 与 fresh main：pending

本报告不得在证据形成前宣称通过、合并或启动 admission matrix successor。
