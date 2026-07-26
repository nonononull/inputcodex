# Issue #67：解耦固定目录证据与活动快照状态执行报告

## 元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/67`
- 基线：`66543faf948375afdc26e51015f2270a48b8bb4f`
- 分支：`codex/issue-67-release-audit-contract-v2`
- 精确范围：六路径
- `scope_hash`：`sha256:b6295dabd39f0cba7c4f13bd3d35ff8b0433e1fb95de98e6fc5f2cf0c1eb6b9f`
- 阻塞 PR：`#66`，Final Head `4e419586fc89b1bbdd79d20b7179f017070052fb`
- 原 PR：`#68`，Final Head `159abbf45dfdc29a277cb152af7368868f2f618d`，保持只读且禁止 force push。
- 当前状态：已基于最新 `main` 等价迁移六路径；重新验证、替代 PR 与 Review/CI 待完成。

## 替代 PR 迁移

- PR `#70` 合并后，PR `#68` 与当前 `main` 唯一重叠路径 `err.md` 产生真实内容冲突，GitHub 状态为 `CONFLICTING/DIRTY`。
- 旧 PR merge ref 仍绑定旧基线，且 `workflow_dispatch` 不接受 `refs/pull/68/merge`；直接 rerun 旧 Performance Run 会继续检出未含 Issue `#69` 修复的原 Head。
- 项目所有者批准从当前 `main` 建立六路径等价替代分支；测试语义保持一致，`scope_hash` 不变，PR `#68` 不修改、不 force push。

## Discovery 与 RED

- CI Run `30202056781` 的 `linux-quality`、`windows`、`macos` 均失败于 `catalog_repository.rs:395`；`required` 因依赖失败，`classify`、`governance`、`release-audit` 成功。
- `git blame` 将永久 `current` 断言定位到 `5fd337fb7ceb9b0ef53e2e694cc5ddd81ea0a98c`。
- 干净 `main` 在清理错误复用的本地 Rust 测试产物后通过定向测试；未修改 PR `#66` 工作树稳定 RED，退出码 `101`。
- 已有专项状态机测试证明 `current` 与 `stale-re-audit-required` 均为合法状态，根因不是生产验证器或 `v1.2.43` 缓存。

## 外部治理与实现

- Session Plan 通过 AGOS `verify-session-plan.ps1`，输出 `SESSION_PLAN_VERIFY_OK`。
- AGOS Default Entry 只运行一次 `ReportOnly`，返回 `needs-input`、`unregistered`、`doctor=blocked`；项目 Git 与入口文档均为 `ready`。按 inputcodex 规则记录后绕过，没有登记、修复或优化 AGOS。
- 仓库实例测试改名为 `仓库v1_2_42目录重新审计证据保持固定`，继续调用 `validate_repository` 并固定目录/合同/source-index 证据。
- 删除唯一的永久 `!summary.requires_reaudit()` 断言和固定 `upstream/source-lock.json` snapshot/status 文本断言；生产验证器、专项状态机和 PR `#66` 均未修改。

## GREEN 与保护回放

- 本分支 main/current：精确新测试 `1/1`，完整 `catalog_repository` `12/12`。
- 临时 detached 工作树从 PR `#66` Head `4e419586fc89b1bbdd79d20b7179f017070052fb` 创建，只叠加本任务测试文件补丁；精确新测试 `1/1`，完整 `catalog_repository` `12/12`。
- stale 输入实值为 snapshot `v1.2.43`、catalog `v1.2.42`、status `stale-re-audit-required`、`re_audit_issue_ref=#65`。
- 临时工作树验证后通过 `git worktree remove --force` 清理，目标路径已预先限制在 `inputcodex-worktrees`，清理后不存在。

## 提交前轻量门禁

- Session Plan：`SESSION_PLAN_VERIFY_OK`；Protected Feature Replay：`STATUS=ready`、`COMPLETION_STATUS=passed`、两项 owner/regression 状态全部 passed。
- Rust：完整 `catalog_repository` `12/12`。
- CI 合同：原提交为 `CI_CONTRACT_GREEN passed=34`；替代基线包含 Issue `#69` 新合同，重新验证目标为 `passed=35`。
- Release Audit Gate：`ok=true`、`status=current`、`blocked_paths=[]`、`errors=[]`。
- Repository Policy：`ok=true`、`violation_count=0`。
- `cargo fmt --all -- --check` 与 `git diff --check` 通过。
- 六路径精确匹配，`scope_hash=sha256:b6295dabd39f0cba7c4f13bd3d35ff8b0433e1fb95de98e6fc5f2cf0c1eb6b9f`；六文件均无非法控制字节。
- PR `#66` 仍为 OPEN、非 Draft、35 文件，Head 仍为 `4e419586fc89b1bbdd79d20b7179f017070052fb`。
- AGOS Git snapshot checkpoint 返回 `GIT_SNAPSHOT_STATUS=ready`、`GIT_COMMIT_DISCIPLINE_STATUS=ready`、`GIT_SOURCE_EDIT_ADMISSION_STATUS=ready`；首次通过 `pwsh -File` 传数组的失败已按 `err.md` 记录并改用脚本对象直接调用。
- 提交前 changed-surface 自审确认：生产验证器未改、专项状态机未削弱、PR `#66` 未改、删除内容只包含永久 current 与固定 snapshot/status 的重复断言，无新增 Review 发现。

## 待写入证据

- 替代提交、替代 PR、Review、两套 Workflow、PR `#68` superseded 状态和最终 Head。
