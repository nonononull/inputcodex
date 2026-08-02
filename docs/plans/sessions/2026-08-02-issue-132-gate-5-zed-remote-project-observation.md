# Issue #132 Session Plan：Zed 远程项目只读观察

## Session Metadata

- `task_id`: `issue-132-gate-5-zed-remote-project-observation`
- `work_class`: `standard`
- `decision_status`: `approved`
- `approval_source`: `standing-authorization`
- `tracking_issue_ref`: https://github.com/nonononull/inputcodex/issues/132
- `approved_decision_ref`: https://github.com/nonononull/inputcodex/issues/131#issuecomment-5152838531
- `implementation_scope_approval_ref`: https://github.com/nonononull/inputcodex/issues/132#issuecomment-5152853748
- `standing_authorization_ref`: https://github.com/nonononull/inputcodex/issues/111
- `selected_business_path`: `gate-5/zed-remote-project-observation`
- `baseline_ref`: `origin/main@da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4`
- `branch_ref`: `codex/issue-132-gate-5-zed-remote-project-observation`
- `worktree_ref`: `C:/Users/dashuai/.paseo/worktrees/1takg4n7/issue-132-gate-5-zed-remote-project-observation`
- `planning_scope_hash`: `sha256:543b946874f01849a30f1098ab298cb86d7d24df9cc97b6c9082c27d71d27de5`
- `candidate_scope_hash`: `sha256:7ee5d47dca72d0d2f1ec683cc45e4bbac0e3ce40af1e57417d39608c8c0c26bb`
- `execution_state`: `LOCAL_VERIFIED`

## Mutation Intent

- `mutation_intent`: `source`
- `allowed_operations`: 二十八路径内项目原生计划、TDD、依赖锁定、Parity、稳定文档、验证、普通 Git/PR 与精确门禁合并。
- `forbidden_operations`: main 直写、范围外修改、原始身份披露、legacy registry、写入/网络/进程/UI/unsafe、force push、GitHub auto-merge。

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  memory_refs:
    - inputcodex autonomous refactor interrupt recovery
    - inputcodex Gate 5 observation planning
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - CONTEXT.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - crates/inputcodex-platform/src/local_session_directory_observation.rs
    - crates/inputcodex-platform/src/markdown_generation.rs
    - parity/features/remote-install.yml
    - parity/contracts/remote-install.yml
    - parity/features/source-index.yml
  github_refs:
    - https://github.com/nonononull/inputcodex/issues/111
    - https://github.com/nonononull/inputcodex/issues/130
    - https://github.com/nonononull/inputcodex/issues/131
    - https://github.com/nonononull/inputcodex/issues/132
  codegraph: absent-not-initialized
  superpowers: unavailable-in-current-session-project-native-tdd-used
  agos: optional-bypass-no-cross-repo-mutation
```

## Change Contract

- `target_contract`: 将 `list_zed_remote_projects` 从完整 Zed Remote 管理中拆为固定来源、最小披露的严格只读观察。
- `preserved_invariants`: `release_audit=current`、双平台同语义、Iced 仅展示层、无广告/遥测、完整 open/forget/core 保持 `unassessed`、旧 fixture 保持。
- `adjacent_surfaces`: 平台路径 owner、SQLite WAL/SHM 只读定义、`LoadCoordinator`、旧 `feature.remote-install.zed-remote`、Cargo 许可证与锁文件。
- `historical_state_refs`: Issue #128 的来源副作用误分类根因、Issue #104/#109 的 SQLite/NOFOLLOW/资源边界、Issue #131 决策。
- `stale_verdict_invalidation_refs`: PROJECT-MASTER-PLAN 中尚未勾选的 Issue #128 和旧 active task。
- `sibling_regression_guard`: `passed`；四 crate tests/Clippy、Parity 计数、依赖差异、禁止能力、治理和精确范围聚合门禁全部通过。

## Review Evidence

- `proposal_mode`: `delegated-agents`
- `actual_agent_count`: `2`
- `agent_result_refs`: `8f056fcd-9028-4b6a-8b9d-98ff087e7728`, `83ca859c-9cf6-4c4e-9923-837f1016d1c0`
- `divergence`: `low`
- `parent_resolution`: 采用两者更严格交集，排除 Recent、无 label、完整 SHA-256、Partial 作为 Ready coverage。
- `closed_agent_refs`: `8f056fcd-9028-4b6a-8b9d-98ff087e7728`, `83ca859c-9cf6-4c4e-9923-837f1016d1c0`
- `model_downgrade`: `forbidden`

## Execution Batches

1. Planning Freeze：7 路径控制面与 planning checkpoint。
2. Domain TDD：最小值对象、校验与脱敏。
3. Application TDD：请求、取消、Port、UseCase 与 stale result。
4. Platform TDD：稳定 ID、固定 JSON、严格 SQLite、资源和竞态。
5. Parity TDD：单 command 分拆、新 fixture 与精确计数。
6. Local Closeout：稳定文档、报告与 `build.md` 全门禁。
7. Remote Delivery：PR、Final Head reviewer、Hosted CI、精确 Squash 与 main 验证。

## Checkpoint Rules

- startup baseline：`da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4`
- checkpoints：planning、domain-green、application-green、platform-green、parity-green、local-verified
- 每次提交前核对精确路径和 `git diff --check`；禁止 amend、rebase、force push 或覆写 Git 时间。

## Stop Conditions

- 范围/hash、上游 Release、release audit、单 writer 或 origin/main 漂移。
- 需要原始身份输出、legacy registry、写入/网络/进程/UI/unsafe 或额外直接依赖。
- reviewer finding、Review thread、CI/Performance/Artifact 或许可证未闭环。
