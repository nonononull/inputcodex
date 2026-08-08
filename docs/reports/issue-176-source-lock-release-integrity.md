# Issue #176 source-lock 与 Release 身份完整性报告

## 当前状态

- state: `LOCAL_VERIFICATION_COMPLETE_PR_PENDING`
- baseline: `5a7465252b56f7e90673e72d3e02881ac9238141`
- baseline_tree: `348a55ce78d1c9da408238b6d9b63cb2e49e32ba`
- scope: `11`
- scope_hash: `sha256:973cd1dd29f70ab6cf3f8bd2c82f61c1673adf09eec7f15db3c76dbd93e97001`
- product_delivery: `0`

## 根因

既有 Release Audit 能复算仓库中的上游快照，却没有把 source-lock 全结构、远端 Release/tag/commit/tree、
codeload archive 与许可证 API 组成同一条严格身份闭包；Rust 投影也只读取少量字段，未知或缺失嵌套字段未统一拒绝。

## 当前证据

- 中断恢复身份：HEAD/base 均为冻结基线，未发现第二 writer 或第十二路径。
- PowerShell AST：已修改脚本为零解析错误。
- 完整 CI 合同：`CI_CONTRACT_GREEN passed=99`；其中六个 Release Audit 分组全部通过。
- 真实仓库离线 Release Audit：`current / requires_reaudit=false / errors=[]`。
- 固定上游 live Release/tag/commit/tree/license 与有界 codeload archive 验证：
  `current / requires_reaudit=false / errors=[]`。
- 自治策略 `0` 违规；仓库政策 `0` 违规；`cargo fmt --check`、Git 空白检查及 Ordinal 十一路径
  `sha256:973cd1dd29f70ab6cf3f8bd2c82f61c1673adf09eec7f15db3c76dbd93e97001` 通过。
- Rust source-lock 全结构负例已实现并通过 `cargo fmt`；本机缺少 MSVC `link.exe`，定向测试尚未启动，
  必须由 exact-head Hosted CI 提供运行证据。

## 待完成证据

- exact-head Hosted Rust 定向与 Workspace 门禁
- Final Head Hosted CI、Performance 与 Artifact
- 双独立复审、Squash Merge 与 fresh main 复验

本报告不得在上述证据形成前宣称通过、合并或启动批次 2。
