# inputcodex 项目总计划

schema_version: inputcodex.master-plan.v1
active_task: issue-101-gate-5-context-entry-observation
active_gate: Gate 5 前八个产品切片已进入 main；Issue #100 已完成上下文能力观察与管理分离决策，Issue #101 已建立隔离分支、书面计划、Session Plan、Runtime Workflow 和二十四路径候选范围，正在等待独立实施批准
last_verified_gate: Issue #98 / PR #99 已以单父 Squash 提交 52320c2c02e19d9ffae11ccb6742a0f0fc4b71b9 进入 main；合并后 main CI Run 30449847793 七 Job与 Performance Baseline Run 30449848277 四 Job全绿且 Artifact 均为 0，Issue #98 已按 COMPLETED 关闭且 release_audit=current
next_legal_gate: 只等待项目所有者批准 Issue #101 二十四路径与 candidate_scope_hash sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372；未批准前不得开始产品 TDD、提交实现、push、PR 或 Review/CI
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/101
active_pr_ref: pending
gate_5_platform_paths_exception_ref: https://github.com/nonononull/inputcodex/issues/74
gate_5_platform_paths_scope_hash: sha256:ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37
gate_5_platform_paths_branch_ref: codex/issue-75-gate-5-platform-paths
gate_5_platform_paths_domain_commit: 67447913fa656f30dd2e6d3c65707acca7c20869
gate_5_platform_paths_application_commit: 7e52ec2c4ea2667c22a66e3bae7888eb3cb9e2ce
gate_5_platform_paths_platform_commit: 593c447262f1b1aa0ea578bb4a6a0a65037799a6
gate_5_platform_paths_parity_commit: be5673c82154fe2777046283158a152d11ead62d
gate_5_application_overview_decision_ref: https://github.com/nonononull/inputcodex/issues/77
gate_5_application_overview_issue_ref: https://github.com/nonononull/inputcodex/issues/78
gate_5_application_overview_pr_ref: https://github.com/nonononull/inputcodex/pull/79
gate_5_application_overview_merge_ref: ef69494d92c7c461b0cb858e95f6838404ae1a61
gate_5_version_startup_decision_ref: https://github.com/nonononull/inputcodex/issues/80
gate_5_version_startup_issue_ref: https://github.com/nonononull/inputcodex/issues/81
gate_5_version_startup_scope_hash: sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe
gate_5_version_startup_branch_ref: codex/issue-81-gate-5-version-startup
gate_5_version_startup_planning_commit: 72b03b1af1fd7ab1984a481af1dd30a20879bb43
gate_5_version_startup_domain_commit: 391bfe9db9348518600e14c912333f221c3cfaca
gate_5_version_startup_application_commit: 1eafa90866124e4c281eba127fd48bb701817ebd
gate_5_version_startup_platform_commit: f992890611ff86f1fe6ccf5f0dd86e19d0fb07de
gate_5_version_startup_parity_commit: bee9dcb97fe9c790f45082cb23f0286c89b1d815
gate_5_version_startup_local_checkpoint: 73bd5748d2341a92577ab8273b0db6f7bdb6a265
gate_5_version_startup_pr_ref: https://github.com/nonononull/inputcodex/pull/82
gate_5_version_startup_merge_ref: da65f7d8402e4de27e2795ee8905be18ad565653
gate_5_version_startup_main_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/30338961661
gate_5_version_startup_main_performance_ref: https://github.com/nonononull/inputcodex/actions/runs/30338961713
gate_5_runtime_environment_decision_ref: https://github.com/nonononull/inputcodex/issues/85
gate_5_runtime_environment_issue_ref: https://github.com/nonononull/inputcodex/issues/86
gate_5_runtime_environment_scope_hash: sha256:dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59
gate_5_runtime_environment_branch_ref: codex/issue-86-gate-5-runtime-environment-observation
gate_5_runtime_environment_written_design_commit: 26b3b1c54dd35cc92460879483b0f9d1f9d4793f
gate_5_runtime_environment_planning_commit: 8119f921c061e5019336322a7ad4a4504ff8e16b
gate_5_runtime_environment_domain_commit: 6591882dc23596a502833d38aed08d585b4acc08
gate_5_runtime_environment_application_commit: 55b84b6c2b45d00fdf3f6e42aaa1e86d1635557e
gate_5_runtime_environment_platform_commit: cd41fa8ef739b1481cfbfc491ef42e26369f0b4e
gate_5_runtime_environment_scope_revision_commit: f177b9d6f17ee31d40bb6568f8e9bdf6bec901b5
gate_5_runtime_environment_parity_red_commit: d5c711d9071aa9e9c65d5214531a96e04dddda98
gate_5_runtime_environment_parity_green_commit: a320086f00bd16c65ae5172c28f4bd8c40a7c110
gate_5_runtime_environment_local_checkpoint: c6829fa40b7bf4cf9828f88e2dfe68c552536844
gate_5_runtime_environment_pr_ref: https://github.com/nonononull/inputcodex/pull/87
gate_5_runtime_environment_merge_ref: db0c09b9df272887deb9407a5e344cf87a59dda8
gate_5_runtime_environment_main_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/30356509847
gate_5_runtime_environment_main_performance_ref: https://github.com/nonononull/inputcodex/actions/runs/30356509131
gate_5_relay_environment_decision_ref: https://github.com/nonononull/inputcodex/issues/88
gate_5_relay_environment_issue_ref: https://github.com/nonononull/inputcodex/issues/89
gate_5_relay_environment_scope_hash: sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77
gate_5_relay_environment_branch_ref: codex/issue-89-gate-5-relay-environment-observation
gate_5_relay_environment_planning_commit: d4805d3f81801d4d685a510d31566738bc9d3ff6
gate_5_relay_environment_domain_commit: fba0e2e3e7ebb692b417b4e3a80800388f711a1f
gate_5_relay_environment_application_commit: 13e1f349b8a4d2556864e67f1066884b0e4832a4
gate_5_relay_environment_shared_platform_commit: 1d9b774a661af44f765cb8e9cb3d7223a56e594d
gate_5_relay_environment_target_platform_commit: 1dae9843618bdffc95f16069d54b7e7440d21db8
gate_5_relay_environment_parity_commit: 749d02e23f1c1fa8f042d598d8f5bb5e28a18638
gate_5_relay_environment_local_checkpoint: b33bbbb7a6b2303c7a0c60725745528c92085c66
gate_5_relay_environment_pr_ref: https://github.com/nonononull/inputcodex/pull/90
gate_5_relay_status_decision_ref: https://github.com/nonononull/inputcodex/issues/97
gate_5_relay_status_issue_ref: https://github.com/nonononull/inputcodex/issues/98
gate_5_relay_status_pr_ref: https://github.com/nonononull/inputcodex/pull/99
gate_5_relay_status_merge_ref: 52320c2c02e19d9ffae11ccb6742a0f0fc4b71b9
gate_5_relay_status_main_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/30449847793
gate_5_relay_status_main_performance_ref: https://github.com/nonononull/inputcodex/actions/runs/30449848277
gate_5_context_entry_decision_ref: https://github.com/nonononull/inputcodex/issues/100
gate_5_context_entry_issue_ref: https://github.com/nonononull/inputcodex/issues/101
gate_5_context_entry_scope_hash: sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372
gate_5_context_entry_branch_ref: codex/issue-101-gate-5-context-entry-observation
documentation_information_architecture_issue_ref: https://github.com/nonononull/inputcodex/issues/83
documentation_information_architecture_scope_hash: sha256:d8a404c19b108587a5e17b4ded454444d5e948c92410b759504a7eb7c63bed44
performance_budget_observation_issue_ref: https://github.com/nonononull/inputcodex/issues/63
performance_budget_observation_scope_hash: sha256:d5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc
performance_budget_observation_red_ref: 650040763aff07f4884ee9252c50639469622934
performance_budget_observation_green_ref: b465c660d0401ff4bff37673671147aa6b513e1a
performance_budget_observation_pr_ref: https://github.com/nonononull/inputcodex/pull/73
performance_budget_observation_merge_ref: 19d1824398d46a0d4f6b9e4805905485793d3c9d
performance_budget_observation_main_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/30242317618
performance_budget_observation_main_performance_ref: https://github.com/nonononull/inputcodex/actions/runs/30242317615
performance_remeasurement_issue_ref: https://github.com/nonononull/inputcodex/issues/54
performance_queue_discovery_issue_ref: https://github.com/nonononull/inputcodex/issues/57
performance_fixed_remeasurement_issue_ref: https://github.com/nonononull/inputcodex/issues/59
performance_budget_values_pr_ref: https://github.com/nonononull/inputcodex/pull/60
performance_budget_values_merge_ref: e225144831a0928bfa3aaa0d169a054779005812
performance_budget_values_main_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/30194897171
performance_budget_values_main_performance_ref: https://github.com/nonononull/inputcodex/actions/runs/30194897166
performance_budget_values_closeout_issue_ref: https://github.com/nonononull/inputcodex/issues/61
performance_remeasurement_entry_issue_ref: https://github.com/nonononull/inputcodex/issues/55
performance_budget_discovery_issue_ref: https://github.com/nonononull/inputcodex/issues/50
performance_budget_discovery_pr_ref: https://github.com/nonononull/inputcodex/pull/51
performance_budget_discovery_merge_ref: fea8824c652665df710a7e6ef941854060eb6e1f
performance_budget_discovery_main_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/30175592979
performance_baseline_issue_ref: https://github.com/nonononull/inputcodex/issues/32
performance_baseline_pr_ref: https://github.com/nonononull/inputcodex/pull/49
release_audit_reaudit_issue_ref: https://github.com/nonononull/inputcodex/issues/65
release_audit_reaudit_pr_ref: https://github.com/nonononull/inputcodex/pull/72
catalog_reaudit_closeout_issue_ref: https://github.com/nonononull/inputcodex/issues/47
ci_incident_issue_ref: https://github.com/nonononull/inputcodex/issues/46
release_watch_issue_ref: https://github.com/nonononull/inputcodex/issues/33
source_implementation_issue_ref: https://github.com/nonononull/inputcodex/issues/26
source_implementation_pr_ref: https://github.com/nonononull/inputcodex/pull/27
closed_gate_3_closeout_issue_ref: https://github.com/nonononull/inputcodex/issues/22
closed_implementation_issue_ref: https://github.com/nonononull/inputcodex/issues/19
gate_3_planning_issue_ref: https://github.com/nonononull/inputcodex/issues/17
upstream_watch_issue_ref: https://github.com/nonononull/inputcodex/issues/14
transition_issue_ref: https://github.com/nonononull/inputcodex/issues/8
upstream_sync_issue_ref: https://github.com/nonononull/inputcodex/issues/9
active_branch_ref: codex/issue-101-gate-5-context-entry-observation
transition_branch_ref: codex/issue-8-gate-2-transition
active_plan_ref: docs/plans/2026-07-29-issue-101-gate-5-context-entry-observation.md
active_session_plan_ref: docs/plans/sessions/2026-07-29-issue-101-gate-5-context-entry-observation.md
active_runtime_workflow_ref: docs/workflows/2026-07-29-issue-101-gate-5-context-entry-observation-runtime.md
active_pr_ref: pending
gate_3_closeout_pr_ref: https://github.com/nonononull/inputcodex/pull/23
gate_3_implementation_pr_ref: https://github.com/nonononull/inputcodex/pull/21
gate_3_planning_pr_ref: https://github.com/nonononull/inputcodex/pull/18
transition_pr_ref: https://github.com/nonononull/inputcodex/pull/10
upstream_sync_pr_ref: https://github.com/nonononull/inputcodex/pull/11
closed_delivery_ref: https://github.com/nonononull/inputcodex/pull/3, https://github.com/nonononull/inputcodex/pull/5, https://github.com/nonononull/inputcodex/pull/7, https://github.com/nonononull/inputcodex/pull/10, https://github.com/nonononull/inputcodex/pull/11, https://github.com/nonononull/inputcodex/pull/13, https://github.com/nonononull/inputcodex/pull/15, https://github.com/nonononull/inputcodex/pull/18, https://github.com/nonononull/inputcodex/pull/21, https://github.com/nonononull/inputcodex/pull/23, https://github.com/nonononull/inputcodex/pull/25, https://github.com/nonononull/inputcodex/pull/27, https://github.com/nonononull/inputcodex/pull/36, https://github.com/nonononull/inputcodex/pull/40, https://github.com/nonononull/inputcodex/pull/42, https://github.com/nonononull/inputcodex/pull/44, https://github.com/nonononull/inputcodex/pull/45
active_report_ref: docs/reports/issue-101-gate-5-context-entry-observation.md
gate_3_closeout_report_ref: docs/reports/issue-22-gate-3-closeout.md
gate_3_implementation_report_ref: docs/reports/issue-19-gate-3-rust-workspace-ci.md
gate_2_watch_report_ref: docs/reports/issue-14-gate-2-upstream-watch.md
active_ruleset_ref: https://github.com/nonononull/inputcodex/rules/19395456
active_ci_strategy_ref: docs/plans/2026-07-21-rust-ci-offload-strategy.md
active_ci_implementation_plan_ref: docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
decision_status: issue-89-pr-90-review-ci-green-final-merge-not-authorized

## 当前状态

- Gate 1 已完成：Issue `#2` / PR `#3`、Issue `#4` / PR `#5`、Issue `#6` / PR `#7` 均已按治理链完成；筹备 Issue `#1` 已以 `completed` 关闭。
- PR `#7` 合并提交为 `c74b66422ba47f96bd3eb2b2385cdfb90541808e`，由 GitHub 生成有效签名；只有一个父提交 `b7404b0c63f2d2ba65474c077182c42a01cc9a64`，tree 为 `00f0f7fe0e408a1e6f218ee8e1be0d8442ed1e65`。
- PR `#7` 的 Review 对话总数、未解决数与 Checks 数量均为 `0`；`0 Checks` 只表示当前尚未配置 CI。
- `main-protection` Ruleset（ID `19395456`）仍为 `active`，只命中 `main`，禁止删除与 Force Push，要求解决全部 Review 对话，只允许 Squash Merge，单人阶段 required approvals 为 `0`。
- `main` 已将完整审计缓存与功能目录审计基线对齐 `v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c`，tree 为 `d478a9fcda7f22a7c8167cb567777ad9148cf328`；`release_audit=current`，Issue `#63/#65` 均已关闭，Gate 5 前置条件已满足。
- Issue `#9` / PR `#11` 已完成 Gate 2 上游基线导入；PR `#11` 于 `2026-07-21T19:01:02Z` Squash Merge，合并提交为 `dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`，Issue `#9` 已关闭。
- `upstream/CodexPlusPlus/` 当前包含 `277` 个审计文件，`upstream/source-lock.json` 记录 `24,196,123` 字节、manifest SHA-256 `330bee0284837c4c3a463d73ea79383cd3d01924b62f669def79276b60f21628` 和 `7` 份许可证/声明。
- Issue `#12` / PR `#13` 已完成上游基线 closeout；PR `#13` 的 Squash Merge 提交为 `5e64015075ddf2adef4bf685f50977b47b7f72e7`，Issue `#12` 已关闭。
- Issue `#14` / PR `#15` 已完成每 6 小时上游监控；最终 PR CI、两次 `main` 真实运行、唯一状态 Issue `#16`、分支清理和有效 GitHub 签名均已闭环。
- Issue `#17` / PR `#18` 已完成 Gate 3 规划交付；PR `#18` 的 Squash Merge 提交为 `477d110a9b284e127af365f5278901bcfa69e093`，Issue `#17` 已关闭。
- Issue `#19` / PR `#21` 已完成 Gate 3 实现：治理合同 `30/30`、七成员 Workspace、首版无缓存三平台 CI、五类失败语义与三平台各 `3/3` 次最低冷构建基线均已进入 `main`。
- PR `#21` 于 `2026-07-22T12:25:59Z` Squash Merge 为 `0716ec0debcd3e059cc4ca88a072232841ca73b4`；Issue `#19` 已按 `COMPLETED` 关闭，合并后 `main` 运行 `29919596057` 六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#22` / PR `#23` 已完成 Gate 3 独立 closeout；PR `#23` 于 `2026-07-22T13:05:34Z` Squash Merge 为 `f470c062037042a1f7833a29cdcf216f6c0f5601`，Issue `#22` 已按 `COMPLETED` 关闭，合并后 `main` 运行 `29922385227` 六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#24` / PR `#25` 已完成“两阶段拆分”规划合同；PR `#25` 于 2026 年 7 月 22 日 Squash Merge 为 `431682296f53e86de1184c732b0d4748857c9390`，Issue `#24` 已按 `COMPLETED` 关闭，合并后 `main` 运行 `29926710342` 六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#26` / PR `#27` 已完成功能目录执行：`133` 条入口映射为 `36` 个 feature、`3` 个排除、`10` 个 `exception-pending` 与 `0` 个覆盖缺口；`36` 份合同、`11` 个 fixture manifest 和验证器已合入 main。独立 Closeout 已由 Issue `#28` / PR `#29` 完成：Squash 提交 `c07da0cad33e09b5c54e528a8a6728a048c88c0b`、单父、tree `02ab8a3d8497ebb7b990e4078122b9bf916ef454`、GitHub 签名有效、合并后 main CI `29948874307` 六 Job 全成功且 Artifact 数为 `0`。
- Issue `#35` / PR `#36` 已解耦 Release 快照与目录审计基线，Squash 提交为 `d7438a0f2c43b7fbd2b159b3759aacea4ef1999e`；因此后续缓存可以合法进入显式 stale，而不伪造目录一致性。
- Issue `#34` / PR `#40` 已完成 `v1.2.42` 纯缓存同步，Squash 提交为 `353391424db5514d022473ba97f601486a190869`；PR 基线更新不会自动生成新的 `pull_request` Run，故经所有者授权关闭并立即重开且未改 Head、文件或提交，Run `30147559602` 全绿后才合并，合并后 main Run `30147841226` 七 Job 全绿。
- Issue `#41` / PR `#42` 已修复固定 Release 与 stale 语义的错误耦合，Squash 提交为 `8aa1d4c96b0543e766b477b1b8e9652968b55f92`；合并后 main Run `30147071062` 七 Job 全绿，Issue 已关闭。
- Issue `#43` / PR `#44` 已完成 `v1.2.42` 缓存与 CI 合同状态收口，PR `#44` 以单父 Squash 提交 `fdb2f98c701800969fc478f95cd2539be598faaa` 合并；合并后 main Run `30152001233` 成功。
- Issue `#38` / PR `#45` 已完成二十六路径重新审计：最终 Head `d3df8759bdb9c6378497a3a0c8f409c3968f4d4f` 精确包含 `26` 路径，CI Run `30157623932` 七 Job 全绿且 Artifact 为 `0`；单父 Squash 提交 `5fd337fb7ceb9b0ef53e2e694cc5ddd81ea0a98c` 的 tree 为 `5273d0e42483bbb5c629a2243fc24f0a892b3db3`、GitHub 签名 `valid`，Issue `#38` 已按 `COMPLETED` 关闭。
- Issue `#64` / PR `#66` 已将 `v1.2.43` 完整快照缓存为只读审计输入；Squash 提交为 `15e91708b41548f523e26ede4c7ca4de41badf77`，主干 CI Run `30214249228` 七 Job 全绿且 Artifact 为 `0`。
- Issue `#65` / PR `#72` 已完成二十四路径与 `sha256:82234e7aacce0bd6c57994529ccf74371052ed906dc8371324b90e41f697d7b7` 的 TDD、Review/CI、全部对话闭环与单独 Squash 授权；合并提交为 `fc1683aabda4afb27ca333387ec954b6a405d2df`，Issue 已按 `COMPLETED` 关闭。
- Issue `#74` 已确认平台路径安全差异：无效显式路径与非空 `CODEX_HOME` 必须失败，禁止相对目录或静默回退；Issue 已按 `COMPLETED` 关闭。
- Issue `#75` / PR `#76` 已在三十路径与 `sha256:ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37` 内完成首个 Gate 5 平台路径迁移；Final Head `ba082669cd6d491cce26e29efcaa249786973a39` 以单父 Squash 提交 `a06a97fd59ce125306a13202c8f1a07656c797a0` 进入 `main`，tree 为 `b669aa6610e976542a74f404ff4f87b36864816b`，Issue 已关闭。合并后主干 CI Run `30276472184` 七 Job、Performance Baseline Run `30276476891` 四 Job全绿且 Artifact 均为 `0`。
- Issue `#77` 已批准应用概览方案 A 并按 `COMPLETED` 关闭；历史启动记录不得冒充实时运行状态，损坏状态不得静默视为无记录。
- Issue `#78` / PR `#79` 已完成二十九路径应用概览只读事实迁移；Final Head `1bae5d51850c3538c3e161e73e266ac19f7406b3` 以单父 Squash 提交 `ef69494d92c7c461b0cb858e95f6838404ae1a61` 进入 `main`，tree 为 `936cc74fbceae2a3ee8d98b924c836e13d9f7ae3`，GitHub 签名 `valid`，Issue 已按 `COMPLETED` 关闭。合并后主干 CI Run `30289461278` 七 Job、Performance Baseline Run `30289461109` 四 Job全绿且 Artifact 均为 `0`。
- Issue `#80` 已批准版本与启动意图方案 A并按 `COMPLETED` 关闭；版本来源、合法输入、非法值优先级、禁止副作用与非目标平台错误已经冻结。
- Issue `#81` / PR `#82` 已完成二十三路径版本与启动意图迁移；Final Head `adc91bafd850fd054346b44e8b79a42bb7b00f71` 以单父 Squash 提交 `da65f7d8402e4de27e2795ee8905be18ad565653` 进入 `main`，tree 为 `0aad4659daaf9e07f3e62d1fd1ad9dfea38fd604`，GitHub 签名 `valid`，Issue 已按 `COMPLETED` 关闭。合并后主干 CI Run `30338961661` 七 Job、Performance Baseline Run `30338961713` 四 Job全绿且 Artifact 均为 `0`。
- Issue `#83` / PR `#84` 已完成 README 与文档导航重构，并以单父 Squash 提交 `3f2914cd81ace7afe28e0137c867c20fd346c3f9` 进入 `main`；该任务没有迁移产品能力。
- Issue `#85` 已批准运行时环境观察与破坏性清理分离并按 `COMPLETED` 关闭；原环境冲突总功能继续为 `unassessed`。
- Issue `#86` / PR `#87` 已完成第四个 Gate 5 运行时环境冲突只读观察切片；Final Head `4799d40aedbf1ea4e95fbbf97fc2ed5cf72e5379` 以单父 Squash 提交 `db0c09b9df272887deb9407a5e344cf87a59dda8` 进入 `main`，tree 为 `7429a3cf1705239a47ac5bf7536e5541da401c51`，GitHub 签名 `valid`，Issue 已按 `COMPLETED` 关闭。合并后主干 CI Run `30356509847` 七 Job、Performance Baseline Run `30356509131` 四 Job全绿且 Artifact 均为 `0`。
- Issue `#88` 已批准 Relay 环境方案 A 并按 `COMPLETED` 关闭；独立只读观察只覆盖固定代理环境来源、`CODEX_HOME/.env` 存在状态和四个 Clash Verge TUN 候选，网络测试、配置修改与原网络环境总功能继续未评估。
- Issue `#89` 已在批准的三十路径与 `sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77` 内完成 Planning、Domain、Application、共享 Platform、双平台适配、Parity、项目控制面、本地验证、非 Draft PR `#90`、Review/CI 根因闭环与 Artifact 核验；动态证据保留在 GitHub，当前只等待独立 Squash Merge 授权。
- 合并后 main CI Run `30158058627` 的 Attempt `1/2` 因 GitHub Actions major outage 在创建 Job 前失败；服务恢复后的同一 Run Attempt `3` 绑定同一提交并七 Job 全绿、Artifact 为 `0`，Issue `#46` 已按 `COMPLETED` 关闭，仓库代码与 CI 合同无需为该外部事故修改。
- Issue `#47` 以 `sha256:dd612ef0c2e5f0f830c40f161b1ef1a85bc58cd1d85458a758c3905ade8db03e` 冻结微型五路径 Closeout；本报告只收口长期状态与事故复用知识，其自身动态 Review、CI 与合并证据保留在 GitHub Issue/PR，不再创建二次 Closeout。
- Issue `#8` 的过渡交付为 PR `#10`；该 PR 只包含文档与验证控制面，并按项目所有者明确授权执行 Squash Merge。
- AGOS 仍是可选外部辅助；本仓库可用原生控制面时不运行它，不在本任务中修改或优化它。

## 项目不变量

- 软件名称固定为 `inputcodex`。
- 性能优先不能通过静默删除有效功能实现。
- 有效功能默认与上游最新正式 Release 保持行为一致。
- Windows 与 macOS 从首版起功能一致。
- 禁止 TypeScript、JavaScript 业务代码和 WebView；产品使用 Rust 与 Iced。
- Iced 只能存在于展示层，领域、应用、基础设施和平台层不得依赖 Iced 类型。
- 广告、推广、导流和隐蔽遥测不得进入最终运行面。
- 上游完整快照只用于审计、映射和重构追踪，不参与新产品构建。
- 上游 Tauri/React 管理界面、现有注入脚本和远程推荐列表不得直接进入新架构或最终运行面。
- 无效功能、有害副作用和错误语义争议必须进入 `parity-exception` Issue，由项目所有者决定。
- 客户端更新、安装包、签名与下载地址只属于 `nonononull/inputcodex`。
- 所有 PR 合并到 `main` 只允许 Squash Merge；禁止 Merge Commit 和 Rebase Merge。
- `main` 永久禁止 Force Push、删除和绕过 Ruleset；错误历史只能通过关联 Issue/PR 的 revert 处理。
- 所有 Review 对话必须在确定根因、完成处理并回写验证证据后才能解决和合并。
- 单人维护阶段 required approvals 为 `0`，但必须保留所有者决策证据；第二名具备合并权限的人类维护者加入后，在下一次合并前提升为 `1`。
- Rust 全量编译与双平台验证默认在标准 GitHub-hosted runners 完成；Issue `#19` 是唯一获批的 Gate 3 Workspace 与首版 CI 实现任务。
- 权威控制面是 `AGENTS.md`、`README.md`、`build.md`、`err.md`、本 Master Plan、任务计划和 GitHub 证据；外部框架只能提供可选辅助。

## 阶段索引

### Gate 0：仓库准备（已验证）

- 已建立本地与 GitHub 公开仓库、许可证、根文档、Issue #1 和筹备 closeout。

### Gate 1：方案与治理冻结（已完成）

- [x] 冻结纯 Rust/Iced、性能优先、功能一致、双平台一致和无广告硬约束。
- [x] 完成架构治理、Ruleset、CI 云端卸载策略和项目原生验证入口。
- [x] PR `#3`、`#5`、`#7` 均以 Squash Merge 合入 `main`。
- [x] Issue `#1`、`#2`、`#4`、`#6` 均完成关闭证据；PR #7 旧分支已清理。
- [x] Issue Forms、PR 模板与项目标签已进入 `main`。

### Gate 2：导入上游基线与监控（已完成）

- [x] 创建 Issue `#9`，锁定当前上游正式 Release `v1.2.41` 与提交。
- [x] 创建 Gate 2 Session Plan、Runtime Workflow 和来源/许可证/纯净性验证范围。
- [x] 通过 Issue `#8` / PR `#10` 完成 Gate 1→2 控制面过渡。
- [x] 获得 Issue `#9` 的快照导入范围和项目所有者合并批准。
- [x] 通过独立 upstream-sync PR `#11` 只更新 `upstream/`、source-lock 和同步报告，并 Squash Merge 到 `main`。
- [x] 通过 Issue `#12` / PR `#13` 回写 merge ref、`build.md`、`err.md` 和最新控制面。
- [x] 通过 Issue `#14` / PR `#15` 建立每 6 小时只管理 Issue 的上游监控，并完成两次真实 Actions 幂等验证。

### Gate 3：纯 Rust 工作区骨架（已完成）

- [x] 通过 Issue `#17` / PR `#18` 冻结分层 Workspace、Iced 隔离、平台端口、加载状态、性能诊断和三平台 CI 合同。
- [x] 创建实现 Issue `#19`、独立分支、Session Plan、Runtime Workflow 与初始报告，并取得项目所有者实现批准。
- [x] 先建立 `scripts/ci` 的可信 RED/GREEN 治理合同，再创建七成员 Workspace；当前合同为 `30/30`，七成员 Workspace checkpoint 已推送。
- [x] 在同一实现 PR 中通过标准 Linux、Windows、macOS Runner、`required` 汇总、五类真实失败恢复和三平台各 `3/3` 次无缓存冷构建基线。
- [x] PR `#21` 最终 Head 六 Job 全绿、Review 对话为 `0`，已按明确授权 Squash Merge；Issue `#19` 已关闭，合并后主干 CI 全绿。
- [x] Issue `#22` / PR `#23` 已完成独立 closeout，merge/tree/签名/Issue/CI/Review/Ruleset 和分支删除证据均已闭环。
- 不迁移业务功能，不创建临时 UI 事实标准；最小窗口的视觉和交互默认由 Gemini 实现或审阅。

### Gate 4：功能目录已收口，性能基线正在 PR 收口

- [x] 创建 Issue `#24`，批准采用“规划合同 → 两个独立执行 Issue”的拆分方案。
- [x] 冻结功能矩阵的稳定标识、证据路径、行为字段、既有一致性状态和决策引用。
- [x] 冻结行为合同、脱敏夹具规则、性能测量协议和可比环境要求，不填写未经实测的预算。
- [x] 定义“功能矩阵/行为合同/脱敏夹具”执行 Issue 与“性能基线/预算批准”执行 Issue 的互斥边界。
- [x] PR `#25` 通过 Review/CI 并按项目所有者授权 Squash Merge；Issue `#24` 已完成独立 closeout。
- [x] 创建功能目录执行 Issue `#26` 与独立分支。
- [x] 提交 Issue `#26` 任务计划、Session Plan、Runtime Workflow、36 条范围和新 scope hash checkpoint，并取得项目所有者实现批准。
- [x] 完成 RED schema、GREEN Rust 验证器与 source-index/五域功能目录 checkpoint；不得迁移产品功能。
- [x] 建立五域 `36` 份行为合同与必要的 `11` 个脱敏 fixture manifest，并完成完整本地仓库验证；产品、CI、Ruleset、Release、`upstream/`、`benchmarks/` 和 AGOS 保持零差异。
- [x] PR `#27` 已完成 Review/CI 并按项目所有者对具体 PR 与最终 Head 的授权 Squash Merge；Issue `#26` 已关闭，来源分支本地、远端和远端跟踪引用均已清理。
- [x] Issue `#28` / PR `#29` 已以独立 Closeout 回写来源 Issue、PR、Review、CI、Squash、签名、tree 与分支清理证据；PR `#29` 已按项目所有者授权 Squash Merge，Issue `#28` 已关闭，本任务未创建性能基线或优化。
- [x] Issue `#35` / PR `#36` 已将完整快照与功能目录审计基线解耦。
- [x] Issue `#34` / PR `#40` 已缓存 `v1.2.42` 并保持显式 `stale-re-audit-required`；Issue `#41` / PR `#42` 已使该合法状态通过 CI 合同验证。
- [x] Issue `#43` / PR `#44` 已将缓存、CI 合同与下一合法工作状态收口到 `main`。
- [x] Issue `#38` / PR `#45` 已完成二十六路径、Review/CI、全部对话闭环与授权 Squash Merge；Issue 已按 `COMPLETED` 关闭，功能目录和 `release_audit` 均对齐 `v1.2.42`。
- [x] Issue `#64` / PR `#66` 已缓存 `v1.2.43`，保持 `stale-re-audit-required` 并正确指向 Issue `#65`，合并后主干 CI 七 Job 全绿。
- [x] Issue `#65` / PR `#72` 已完成二十四路径本地 TDD、目录/合同/fixture/source-lock、Review/CI、全部对话闭环与独立 Squash Merge；`release_audit` 恢复 `current`，Issue 已关闭。
- [x] Issue `#46` 已证明主干 Run `30158058627` Attempt `1/2` 为 GitHub Actions 外部事故；同一 Run Attempt `3` 七 Job 全绿且 Artifact 为 `0`，事故已按 `COMPLETED` 关闭。
- [x] Issue `#47` 已冻结五路径 Closeout、范围哈希、禁止面和所有者批准；其报告进入长期控制面，动态 PR/CI/合并证据只保留在 GitHub，不再递归创建 Closeout。
- [x] Issue `#32` 已完成 Discovery，并冻结 28 路径、测量对象、可比环境与 `sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505`；项目所有者已授权进入分支、Session Plan、Runtime Workflow、实现、验证与 PR。
- [x] Issue `#32` / PR `#49` 已在隔离工作树中实现性能测量入口、opt-in 首次 view 探针与证据验证器；初始 Run `30169262247` 已产生两平台样本。
- [x] 最终 Evidence Run `30170128309` 已证明 Windows `core.autocrlf=true` 会使原始工作树 JSON 哈希失配；fresh checkout 可重复，TDD 回归合同已从 RED 转为 `34/34` GREEN，主 CI Run `30170128326` 七 Job 全绿。
- [x] 删除绑定旧实现哈希的固定结果后，修复 Head `42bc2e9ce7cf2e88d0602ebdc638213854793f96` 已由 Performance Run `30170535534` 重新 hosted 测量并入库当前实现样本；同 Head 主 CI Run `30170535538` 七 Job 全绿，成功 Run Artifact 数为 `0`。
- [x] PR `#49` 最终 Head `78f8eec2fdea0ae33b02c7478923e1cbaf223d69` 已完成 Evidence、Review/CI、全部对话闭环和项目所有者独立授权，并以单父 Squash 提交 `fd9db9ca1c150b7db34dda8acc09b6f0cc357a17` 进入 `main`；Issue `#32` 已按 `COMPLETED` 关闭，合并后主 CI 七 Job 与 Performance Evidence 四 Job 全绿且 Artifact 数均为 `0`。
- [x] Issue `#32` 的稳定语义是“性能基线完成、预算未批准”；现有 Windows/macOS 样本只能用于同平台、同可比环境趋势，不能跨平台排名或直接生成 required budget。
- [x] Issue `#50` 已冻结九路径、`sha256:af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f` 和 ADR `0004`：每个平台至少五次独立可比 Run，采用 run-level 稳健汇总并从观察阶段逐步升级门禁。
- [x] PR `#51` Final Head `e0154c61d8b05835db10437c79f029909516eac1` 已完成 Review/CI、全部对话闭环和项目所有者独立授权，并以单父 Squash 提交 `fea8824c652665df710a7e6ef941854060eb6e1f` 进入 `main`；tree 为 `9fb518cda8b35a9388fb9fce0a1ff6ba976d80cb`，GitHub 签名 `valid`。
- [x] PR `#51` 合并后主干 CI Run `30175592979` 七 Job 全绿且 Artifact 为 `0`，Issue `#50` 已按 `COMPLETED` 关闭；稳定语义仍是“预算方法已批准、预算数值未批准”。
- [x] Issue `#55` 为已有证据锁定 `evidence` 的 Workflow 合同增加显式手工 `measure` 入口，并以同一 Issue 成功 Artifact 刷新因实现哈希漂移而失效的三份 Evidence；该最小前置修复不改变采集器、结果 schema、预算数值、预算 CI、优化或 Gate 5。
- [x] Issue `#54` 已在 Issue `#55` 合并后完成八次严格串行 GitHub-hosted Run；Windows 的初始 AMD EPYC 9V74 队列只有 `2` 次、次级 AMD EPYC 7763 队列只有 `4` 次，macOS 为 `8` 次可比样本。五次硬约束未满足，数值预授权未触发，Issue 保持 `STOPPED_AT_EIGHT_RUN_CAP`，不得创建 `run-09`。
- [x] Issue `#57` / PR `#58` 已产出 Hosted Windows CPU 队列异构性的决策材料并以 Squash 提交 `d9d1ed77b9796ac6a99e250d1547217a39426aa9` 进入 `main`；项目所有者选择方案 A 后，Issue `#57` 已按 `COMPLETED` 关闭。
- [x] Issue `#59` 已在 38 路径与 `sha256:d0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570` 内完成四次固定串行槽位，Windows 严格目标队列达到 `5`、macOS 达到 `12`；预算 JSON 与离线构建/验证脚本为 `10/10` GREEN，不存在 `run-05`。
- [x] PR `#60` Final Head `61c088d74d61a329fbe67e14b8280dfa9701c6b2` 已完成 Review/CI 和项目所有者独立授权，并以单父 Squash 提交 `e225144831a0928bfa3aaa0d169a054779005812` 进入 `main`；tree 为 `56eb1e8d95dfce22726c1aef1bdde1c353af055e`，GitHub 签名 `valid`，Issue `#59` 已按 `COMPLETED` 关闭。
- [x] PR `#60` Final Head CI/Performance Run `30194465259` / `30194465231` 与合并后主干 Run `30194897171` / `30194897166` 均全绿且 Artifact 为 `0`；Issue `#61` 仅以反递归 Closeout 固化该稳定事实，其自身动态交付证据只保留在 GitHub。
- [x] Issue `#63` / PR `#73` 已在十三路径与 `sha256:d5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc` 内完成观察器 `12/12`、CI 合同 `35/35`、双平台 observation、Review/CI 与独立 Squash 授权，并以提交 `19d1824398d46a0d4f6b9e4805905485793d3c9d` 进入 `main`；PR `#72` 合并后的主干 observation 已通过，Issue `#63` 已按 `COMPLETED` 关闭。

### Gate 5：分域迁移（进行中）

- 按基础能力、供应商与网络、会话与数据、插件与脚本、远程集成与安装分域迁移。
- 每个可独立验收功能使用独立 Issue 和 PR，上游同步与功能重构永远分离。
- `release_audit` 不是 `current` 时，任何 Gate 5 产品迁移 PR 都必须被门禁阻断。
- [x] 首个 Gate 5 Issue 的预算复测、数值批准、非 required `approved-observation`、Windows/macOS 主干观察与 `release_audit=current` 前置条件已满足。
- [x] Issue `#74` 已批准平台路径安全例外并关闭。
- [x] Issue `#75` / PR `#76` 已完成平台路径迁移、Review/CI、独立 Squash 授权与合并后主干验证。
- [x] Issue `#77` 已完成应用概览语义例外决策并关闭。
- [x] Issue `#78` / PR `#79` 已完成应用概览只读事实迁移、Review/CI、独立 Squash 授权与合并后主干验证。
- [x] Issue `#80` 已完成版本与启动意图方案 A 决策并关闭。
- [x] Issue `#81` / PR `#82` 已完成版本与启动意图迁移、Review/CI、独立 Squash 授权与合并后主干验证。
- [x] Issue `#85` 已完成运行时环境观察与清理分离决策并关闭。
- [x] Issue `#86` / PR `#87` 已完成运行时环境冲突只读观察迁移、Review/CI、独立 Squash 授权与合并后主干验证。
- [x] Issue `#88` 已完成 Relay 环境只读观察与网络环境总功能分拆决策并关闭。
- [x] Issue `#89` / PR `#90` 已完成第五个 Gate 5 Relay 环境只读观察切片、独立 Squash Merge 与 Issue 关闭。
- [x] Issue `#91` 已完成设置只读观察与设置写入管理的方案 A 决策并关闭。
- [x] Issue `#92` / PR `#93` 已完成第六个 Gate 5 设置只读观察迁移、独立 Squash Merge 与主干落盘；只接管 `load_settings`，原设置管理总功能继续 `unassessed`。
- [x] Issue `#94` 已完成设置管理写入、重置与损坏语义的方案 1 决策并关闭。
- [x] Issue `#95` / PR `#96` 已完成第七个 Gate 5 诊断日志只读结构观察迁移、独立 Squash Merge 与主干落盘；只接管 `read_latest_logs`，原诊断总功能继续 `unassessed`。
- [x] Issue `#97` 已完成 Relay 认证与配置状态观察方案 A 决策并关闭；最小披露的只读状态观察与配置管理总功能正式分离。
- [x] Issue `#98` / PR `#99` 已完成第八个 Gate 5 Relay 认证与配置状态只读观察迁移、独立 Squash Merge 与主干验证；只接管 `relay_status`，原 Relay 配置管理总功能继续 `unassessed`。
- [x] Issue `#100` 已完成上下文能力只读目录观察与完整管理分离决策并按 `COMPLETED` 关闭。
- Issue `#101` 是第九个 Gate 5 上下文能力只读目录观察切片：只接管 `read_live_context_entries`，固定单文件 `256 KiB` 上限，只返回条目 ID、稳定种类、启用状态和分类计数；当前停在二十四路径与 `candidate_scope_hash` 的独立实施批准门。
- 第十个产品切片必须等待 Issue `#101` 完成后重新建立独立 Issue、书面设计、精确范围与实现授权；不得复用当前范围扩展上下文写入、完整 TOML、网络、SQLite、Zed Remote、Token 用量、UI 或其他总功能。

### Gate 6：首个正式版本（锁定）

- 完成功能矩阵、双平台、性能预算、差异批准、签名、安装、升级、回滚和自主更新源。
- 首个目标版本不硬编码旧上游版本；遵循 ADR `0002` 的 `v<获批上游版本>-inputcodex.<修订号>`，具体版本、签名和资产由独立 `type:release` Issue 冻结。

## 当前验证入口

- 构建与当前 Gate 验证：`build.md`。
- 排错与已知限制：`err.md`。
- Release 审计基线解耦 ADR：`docs/adr/0003-release-snapshot-catalog-audit-decoupling.md`。
- 性能预算 ADR：`docs/adr/0004-performance-budget-policy.md`。
- Issue `#50` Discovery 计划：`docs/plans/2026-07-25-issue-50-performance-budget-discovery.md`。
- Issue `#50` Session Plan：`docs/plans/sessions/2026-07-25-issue-50-performance-budget-discovery.md`。
- Issue `#50` Runtime Workflow：`docs/workflows/2026-07-25-issue-50-performance-budget-discovery-runtime.md`。
- Issue `#50` Discovery 报告：`docs/reports/issue-50-performance-budget-discovery.md`。
- Issue `#57` Hosted 队列 Discovery 计划：`docs/plans/2026-07-26-issue-57-hosted-queue-heterogeneity-discovery.md`。
- Issue `#57` Session Plan：`docs/plans/sessions/2026-07-26-issue-57-hosted-queue-heterogeneity-discovery.md`。
- Issue `#57` Runtime Workflow：`docs/workflows/2026-07-26-issue-57-hosted-queue-heterogeneity-discovery-runtime.md`。
- Issue `#57` Discovery 报告：`docs/reports/issue-57-hosted-queue-heterogeneity-discovery.md`。
- 已完成 Issue `#35` 实施计划：`docs/plans/2026-07-22-issue-35-release-catalog-decoupling.md`。
- 已完成 Issue `#35` Session Plan：`docs/plans/sessions/2026-07-22-issue-35-release-catalog-decoupling.md`。
- 已完成 Issue `#35` Runtime Workflow：`docs/workflows/2026-07-22-issue-35-release-catalog-decoupling-runtime.md`。
- 已完成 Issue `#35` 报告：`docs/reports/issue-35-release-catalog-decoupling.md`。
- `v1.2.42` 缓存同步报告：`docs/reports/2026-07-23-upstream-v1.2.42-sync.md`。
- `v1.2.43` 缓存同步报告：`docs/reports/2026-07-26-upstream-v1.2.43-sync.md`。
- Issue `#65` 重新审计计划：`docs/plans/2026-07-27-issue-65-v1.2.43-catalog-reaudit.md`。
- Issue `#65` Session Plan：`docs/plans/sessions/2026-07-27-issue-65-v1.2.43-catalog-reaudit.md`。
- Issue `#65` Runtime Workflow：`docs/workflows/2026-07-27-issue-65-v1.2.43-catalog-reaudit-runtime.md`。
- Issue `#65` Discovery 报告：`docs/reports/issue-65-v1.2.43-catalog-reaudit-discovery.md`。
- Issue `#75` 平台路径设计：`docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md`。
- Issue `#75` Session Plan：`docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md`。
- Issue `#75` Runtime Workflow：`docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md`。
- Issue `#75` 实施报告：`docs/reports/issue-75-gate-5-platform-paths.md`。
- Issue `#78` 应用概览设计：`docs/plans/2026-07-27-issue-78-gate-5-application-overview.md`。
- Issue `#78` Session Plan：`docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md`。
- Issue `#78` Runtime Workflow：`docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md`。
- Issue `#78` 实施报告：`docs/reports/issue-78-gate-5-application-overview.md`。
- Issue `#81` 版本与启动意图设计：`docs/plans/2026-07-28-issue-81-gate-5-version-startup.md`。
- Issue `#81` Session Plan：`docs/plans/sessions/2026-07-28-issue-81-gate-5-version-startup.md`。
- Issue `#81` Runtime Workflow：`docs/workflows/2026-07-28-issue-81-gate-5-version-startup-runtime.md`。
- Issue `#81` 实施报告：`docs/reports/issue-81-gate-5-version-startup.md`。
- Issue `#86` 运行时环境观察设计：`docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md`。
- Issue `#86` Session Plan：`docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md`。
- Issue `#86` Runtime Workflow：`docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md`。
- Issue `#86` 实施报告：`docs/reports/issue-86-gate-5-runtime-environment-observation.md`。
- Issue `#89` Relay 环境观察设计：`docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md`。
- Issue `#89` Session Plan：`docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md`。
- Issue `#89` Runtime Workflow：`docs/workflows/2026-07-28-issue-89-gate-5-relay-environment-observation-runtime.md`。
- Issue `#89` 实施报告：`docs/reports/issue-89-gate-5-relay-environment-observation.md`。
- 已完成 Issue `#41` CI 合同报告：`docs/reports/issue-41-ci-contract-decoupling.md`。
- 已完成状态收口计划：`docs/plans/2026-07-25-issue-43-v1.2.42-cache-ci-closeout.md`。
- 已完成状态收口 Session Plan：`docs/plans/sessions/2026-07-25-issue-43-v1.2.42-cache-ci-closeout.md`。
- 已完成状态收口 Runtime Workflow：`docs/workflows/2026-07-25-issue-43-v1.2.42-cache-ci-closeout-runtime.md`。
- 已完成状态收口报告：`docs/reports/issue-43-v1.2.42-cache-ci-closeout.md`。
- 已完成 Issue `#38` 重新审计计划：`docs/plans/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md`。
- 已完成 Issue `#38` Session Plan：`docs/plans/sessions/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md`。
- 已完成 Issue `#38` Runtime Workflow：`docs/workflows/2026-07-25-issue-38-v1.2.42-catalog-reaudit-runtime.md`。
- 已完成 Issue `#38` 发现报告：`docs/reports/issue-38-v1.2.42-catalog-reaudit-discovery.md`。
- Gate 4 `v1.2.42` 重新审计 Closeout：`docs/reports/issue-47-v1.2.42-catalog-reaudit-closeout.md`。
- 单一架构真源：`docs/plans/2026-07-21-architecture-governance.md`。
- Gate 1 最终 closeout：`docs/reports/issue-6-gate-1-finalization-closeout.md`。
- Gate 1→2 过渡计划：`docs/plans/2026-07-21-issue-8-gate-2-transition.md`。
- 已完成 Gate 2 同步计划：`docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md`。
- 上游同步报告：`docs/reports/2026-07-21-upstream-v1.2.41-sync.md`。
- 当前 closeout 计划：`docs/plans/2026-07-21-issue-12-gate-2-upstream-closeout.md`。
- Gate 2 基线 closeout Session Plan：`docs/plans/sessions/2026-07-21-issue-12-gate-2-upstream-closeout.md`。
- Gate 2 基线 closeout Runtime Workflow：`docs/workflows/2026-07-21-issue-12-gate-2-upstream-closeout-runtime.md`。
- Gate 2 基线 closeout 报告：`docs/reports/issue-12-gate-2-upstream-closeout.md`。
- 已完成上游监控计划：`docs/plans/2026-07-22-issue-14-gate-2-upstream-watch.md`。
- Gate 2 上游监控报告：`docs/reports/issue-14-gate-2-upstream-watch.md`。
- Gate 3 架构规划：`docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md`。
- 已完成实现 Session Plan：`docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md`。
- 已完成实现 Runtime Workflow：`docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md`。
- 已完成实现报告：`docs/reports/issue-19-gate-3-rust-workspace-ci.md`。
- 无缓存冷构建基线：`docs/reports/rust-ci-cold-baseline.md`。
- 当前 Gate 3 closeout 计划：`docs/plans/2026-07-22-issue-22-gate-3-closeout.md`。
- 当前 Gate 3 closeout Session Plan：`docs/plans/sessions/2026-07-22-issue-22-gate-3-closeout.md`。
- 当前 Gate 3 closeout Runtime Workflow：`docs/workflows/2026-07-22-issue-22-gate-3-closeout-runtime.md`。
- 当前 Gate 3 closeout 报告：`docs/reports/issue-22-gate-3-closeout.md`。
- 当前 Gate 4 规划：`docs/plans/2026-07-22-issue-24-gate-4-feature-performance-plan.md`。
- 当前 Gate 4 Session Plan：`docs/plans/sessions/2026-07-22-issue-24-gate-4-feature-performance-plan.md`。
- 当前 Gate 4 Runtime Workflow：`docs/workflows/2026-07-22-issue-24-gate-4-feature-performance-runtime.md`。
- 当前 Gate 4 初始报告：`docs/reports/issue-24-gate-4-feature-performance-plan.md`。
- 已完成 Issue `#26` 实现计划：`docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md`。
- 已完成 Issue `#26` Session Plan：`docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md`。
- 已完成 Issue `#26` Runtime Workflow：`docs/workflows/2026-07-22-issue-26-gate-4-feature-catalog-runtime.md`。
- 已完成 Issue `#26` 报告：`docs/reports/issue-26-gate-4-feature-catalog.md`。
- 已完成 Gate 4 Closeout 计划：`docs/plans/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md`。
- 已完成 Gate 4 Closeout Session Plan：`docs/plans/sessions/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md`。
- 已完成 Gate 4 Closeout Runtime Workflow：`docs/workflows/2026-07-22-issue-28-gate-4-feature-catalog-closeout-runtime.md`。
- 已完成 Gate 4 Closeout 报告：`docs/reports/issue-28-gate-4-feature-catalog-closeout.md`。

## 停止条件

- 上游最新正式 Release 或已核验的 `v1.2.43` / `5036ff056b5c629f19356396b17d6eeb70da664c` / `d478a9fcda7f22a7c8167cb567777ad9148cf328` 事实发生变化。
- Issue `#89` 出现三十路径或 `sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77` 之外的新增、删除或重命名路径；把 `core-module:proxy` 或原网络环境总功能迁入新子能力；引入网络、写入、子进程、线程、UI、注入、`unsafe` 或敏感值/路径泄露；或在最终 Head Review、CI 与全部对话闭环前请求 Squash Merge。
- 需要修改 `upstream/CodexPlusPlus/` 或 `source-lock.json` 的来源快照字段，但没有新的独立 upstream-sync Issue/PR 与项目所有者批准。
- `release_audit` 为 stale 时修改 `benchmarks/`、`apps/`、产品 crate、`Cargo.toml` 或 `Cargo.lock`，或在同一 PR 同时更新实际 audit 与受阻产品路径。
- Issue `#47` 出现五路径或 `sha256:dd612ef0c2e5f0f830c40f161b1ef1a85bc58cd1d85458a758c3905ade8db03e` 之外的新增、删除或重命名路径，或在最终 Head 的 Review、CI 和全部对话闭环前请求 Squash Merge。
- Issue `#32` 出现已批准 28 路径或 `sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505` 之外的新增、删除或重命名路径，或改动根 Cargo、`apps/`、`parity/`、`upstream/`、Ruleset、发布资产或 AGOS。
- 在独立性能基线 Issue 中创建性能优化、产品迁移、`parity-exception`、运行上游/半成品或填写绝对性能预算，但没有新的独立 Issue 与项目所有者批准。
- Fresh 验证失败、Ruleset 变化、Review 对话未闭环或出现未批准的一致性差异。
