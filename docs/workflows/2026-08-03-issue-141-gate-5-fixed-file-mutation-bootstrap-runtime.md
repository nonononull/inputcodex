# Issue #141 Runtime Workflow：固定文件 mutation tranche 治理 bootstrap

## Runtime Metadata

- task_id: issue-141-gate-5-fixed-file-mutation-bootstrap
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/141
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214
- retry_resume_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159471091
- baseline_ref: origin/main@42f12ead9209d6eceb7e355562fcd339f1daae81
- branch_ref: codex/issue-141-gate-5-fixed-file-mutation-bootstrap
- candidate_scope: 11
- candidate_scope_hash: sha256:9b0c24a9844ff02143624843e0762d6f0ed5cc84c618d8ff1b3bea16c76bbce3
- execution_profile: project-native-v1
- command_source: build.md Issue #141
- terminal_boundary: non-draft-pr-final-head-no-merge

## Node Order

    startup baseline
      -> Planning Freeze on #141
      -> production mutation RED
      -> policy/verifier/live helper GREEN
      -> task-local documentation
      -> final local gates and self-review
      -> normal commit and push
      -> non-Draft PR Final Head
      -> STOP (merge forbidden)

ALLOWED_OPS：冻结十一路径内编辑、临时测试夹具、只读 Git/GitHub/#132 复验、普通 commit/push、创建和
更新 non-Draft PR。

FORBIDDEN_OPS：产品 Rust、Cargo、Parity、Workflow/Runner/Ruleset、Release/upstream、UI、AGOS、SQLite、
Zed、session-index、environment/clipboard、其他 mutation/exception、secret/network/process、广告、注入、
unsafe、FFI/VFS、新依赖、auto-merge、merge，以及任何 #132/#133 写操作。

## Node 0：Startup Baseline

状态：completed。任务 HEAD、本地主工作树、origin/main 与 GitHub main 均为 `42f12ead...`；主工作树和
任务 worktree clean，开放 PR 为 0，Release Audit current，Repository Policy 零违规，#132 指纹匹配。

## Node 1：Planning Freeze

状态：completed。编辑前已在 #141 发布 11 路径与 `sha256:9b0c24...`，并明确任何增删路径必须停止。
AGOS ReportOnly 返回 needs-input，已按项目规则绕过且没有创建 AGOS 文件。

## Node 2：Production Mutation RED

状态：completed。只修改测试后完整合同得到四个稳定失败：生产 tranche 缺失、真实策略变异无目标、
生产 helper 缺失、空闲状态仍使用泛化候选动作；既有合同继续通过。

## Node 3：Minimal GREEN

状态：completed。policy 删除旧 first candidate 并增加唯一版本化 tranche；验证器锁定 shape/type/value；
live 状态在验证器边界后再次调用生产 helper。真实策略 13 类与 helper 5 类变异全部拒绝，合同 `82/82`。

## Node 4：Documentation

状态：completed。Master Plan active task 从已完成 #128 切换到 #141；Plan、Session、Runtime、Report、
build 定向命令与 Paseo built-in loop 根因均落盘。产品计数和 Parity 文件保持不变。

## Node 5：Final Local Verification

状态：completed。已从 build.md 原文运行完整命令：CI 合同 `82/82`、policy hash `sha256:e19914...`、
live/snapshot、Release Audit、仓库政策、十一路径与 Git 空白全部通过；diff 自审为
`0 Critical / 0 Important`，main freshness 与 #132 指纹复验无漂移。

## Node 6：Remote Delivery

状态：pending。形成普通提交并普通 push，创建关联 #141 的 non-Draft PR；绑定 scope/hash、policy hash、
Final Head、RED/GREEN 和 owner refs。不得启用 auto-merge，不得合并。

## Resume Algorithm

恢复时先读本文件、Git status、#141 与 live state。脏树只允许恢复当前十一路径；clean branch 且无 PR 时
继续提交/建 PR；已有 PR 时只刷新同一 Final Head 证据。任何 base/scope/#132 漂移都停止，不自行修复。
