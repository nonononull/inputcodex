# `main` 保护 Ruleset 落地报告

## 基本信息

- 日期：2026-07-21。
- 仓库：`nonononull/inputcodex`。
- 跟踪 Issue：`#2`。
- 关联 PR：`#3`。
- Ruleset：`main-protection`。
- Ruleset ID：`19395456`。
- Ruleset 地址：`https://github.com/nonononull/inputcodex/rules/19395456`。

## 变更前基线

- 默认分支为 `main`，仓库可见性为 public。
- 仓库只有 `nonononull` 一名具备 `write`、`maintain` 或 `admin` 权限的人类维护者。
- 仓库 Ruleset 数量为 `0`，`main` 未配置传统 Branch Protection。
- 仓库级界面允许 Squash Merge、Merge Commit 和 Rebase Merge；为满足“只作用于 `main`”的边界，本次不修改仓库级合并开关，而在 `main` Ruleset 内限定只允许 Squash Merge。

## 已应用规则

- `target`: `branch`。
- `enforcement`: `active`。
- `include`: 仅 `refs/heads/main`。
- `exclude`: 空。
- `bypass_actors`: 空，项目所有者与管理员无例外。
- `deletion`: 禁止删除 `main`。
- `non_fast_forward`: 禁止对 `main` Force Push。
- `pull_request`: 所有变更必须通过 PR。
- `required_approving_review_count`: `0`，符合当前单人维护阶段规则。
- `required_review_thread_resolution`: `true`。
- `allowed_merge_methods`: 仅 `squash`。

## 明确未配置

- 未配置 required status checks，因为仓库当前没有 Actions 或稳定 CI 检查。
- 未启用 Code Owner Review、最后推送者外的额外批准或 stale review 自动失效；这些规则尚未获得项目所有者批准。
- 未修改功能分支、发布分支或其他未来分支的 Force Push 策略。
- 未创建 GitHub Actions、发布资产，也未合并 PR `#3`。

## Fresh 验证证据

- `GET /repos/nonononull/inputcodex/rulesets/19395456` 返回 `active`，且分支条件只包含 `refs/heads/main`。
- `GET /repos/nonononull/inputcodex/rules/branches/main` 返回三个有效规则，均来自 Ruleset `19395456`。
- 传统 `GET /repos/nonononull/inputcodex/branches/main/protection` 在未配置旧式 Branch Protection 时可能返回 `404`，不能据此判定 Ruleset 未生效。
- Ruleset 无 bypass actor；删除保护、非快进保护、PR 门禁、Review 对话解决和 Squash-only 参数均与批准决策一致。
- Ruleset 生效后，PR `#3` 仍为 `OPEN`、非 Draft、`mergeStateStatus=CLEAN`，Review 对话总数与未解决数量均为 `0`。

## 后续计划

1. 项目所有者完成 PR `#3` 的正式 Review；任何 Review 对话必须先完成根因、处理与验证闭环。
2. PR `#3` 通过后使用 Squash Merge，禁止 Merge Commit、Rebase Merge、Force Push 或直接修改 `main`。
3. 以独立 Issue/PR 建立 Issue 模板、PR 模板和标签体系，完成 Gate 1 剩余治理工作。
4. Gate 1 全部完成后，才允许新建 Gate 2 的 `upstream-sync` Issue，导入上游 `v1.2.41` 快照。
5. CI 稳定后再以独立 Issue/PR 把 required status checks 加入 Ruleset。

## 变更控制

- 放宽、停用、删除或扩大本 Ruleset 的范围，必须先建立治理 Issue、记录项目所有者批准并通过关联 PR 留下证据。
- 紧急情况不能绕过该流程；误配置应先冻结合并，记录事故 Issue，再以最小变更恢复已批准规则。
