# Issue #155 Review ref 与 post-merge 父提交绑定报告

## 当前状态

- state: `LOCAL_VERIFICATION_GREEN / DELIVERY_PENDING`
- baseline: `f3e7d6f873f59399e71b602e1a9fbdee71760d64`
- scope: `8`
- scope_hash: `sha256:16fed02331530651a28e824c1ff1382478511945b4efc6892f4da7b23914247e`
- product_delivery: `0`

## 根因与处理

Review gate 原先只验证仓库级评论 URL 形状，允许其他 PR/Issue 的同值 ref；post-merge gate 只验证父提交
数量，没有保留并比较唯一父 SHA。新增严格当前 PR comment ref helper；collector 从原始属性验证每个 parent
SHA，并投影单父 `parent_oid`；merge/post-merge gate 分别绑定当前 PR 与 `expected_base`。

所有新字段均通过原始 property projection 读取，避免单元素数组被 PowerShell 管道展开为合法标量。

## 本地证据

- RED：`81 pass / 4 fail`。
- 中间态：剩余两个数组展开反例失败。
- GREEN：`CI_CONTRACT_GREEN passed=87`。
- 生产与测试 PowerShell AST：零错误。

## 待完成

- Policy、Repository Policy、Release Audit、live 与八路径最终复验
- 独立 Final Head 双复审
- Hosted CI `7/7`、Performance `4/4`、Artifact `0`、thread `0`
- 精确 Squash Merge 与 fresh-main closeout

本报告不得在证据形成前宣称合并或恢复 admission matrix。
