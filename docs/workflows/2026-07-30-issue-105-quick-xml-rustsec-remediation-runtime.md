# Issue #105 Runtime Workflow：quick-xml RustSec 高危公告修复

## Runtime Metadata

- `task_id`: `issue-105-quick-xml-rustsec-remediation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/105`
- `session_plan_ref`: `docs/plans/sessions/2026-07-30-issue-105-quick-xml-rustsec-remediation.md`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/105#issuecomment-5129028445`
- `branch_ref`: `codex/issue-105-quick-xml-rustsec-remediation`
- `baseline_ref`: `origin/main@060ca045d2c134f8be3c9adc8cdb038842fc3243`
- `planning_scope_hash`: `sha256:25337a6cb90386439878af2bd8be7d00af0276e102663ab6add7e3d3b4621a09`
- `candidate_scope_hash`: `sha256:0c90d018e06aa640d33a4c65c75aea45c89eb0e365b91fadee803b0426c8c58f`
- `mutation_intent`: `config`
- `executor_enforcement`: `exact-path-and-exact-package-diff-hard-stop`
- `current_node`: `remote-delivery`

## Node Graph

```text
Issue #105 owner-approved security route
  -> clean origin/main worktree
  -> fresh cargo audit RED
  -> nine-path scope freeze
  -> project-native planning checkpoint
  -> AGOS report-only boundary
  -> exact wayland-scanner cargo update
  -> package-level Cargo.lock guard
  -> fresh cargo audit GREEN
  -> local lightweight verification
  -> report and local-verified checkpoint
  -> ordinary push and non-Draft PR
  -> Review/CI/Performance/Artifact closure
  -> Final Head owner Squash Merge authorization
```

## Current Gate

允许：七路径 planning 写入、验证、Git snapshot 和命名 planning commit。

禁止：planning checkpoint 前修改 `Cargo.lock`；任何时候修改 `Cargo.toml`、产品源码、UI、Iced、
CI、Ruleset、Runner、`upstream/` 或 AGOS；创建新 Gate 5 产品功能；force push；未授权合并。

## Node Order

### Node 0：Startup Baseline

1. 确认 Issue #105 OPEN、分支名、worktree 与 `origin/main` 基线。
2. 确认工作树干净并运行 Git snapshot startup checkpoint。
3. 读取 AGENTS、README、build、err、Master Plan 与 AGOS 规则入口。

状态：`completed`。基线为 `060ca045...`，startup snapshot 为 `GIT_SNAPSHOT_READY`。

### Node 1：Knowledge 与 Decision Evidence

1. 查询 GBrain、vault/rules 与项目文档；记录没有 task-specific 知识命中。
2. 复核 `wayland-scanner 0.31.10` 的 `quick-xml ^0.39` 约束。
3. 复核候选 `0.31.11` 使用 `quick-xml ^0.41`、MIT、MSRV `1.71`；
   `quick-xml 0.41.0` 为 MIT、MSRV `1.79`。
4. 在 Issue #105 冻结批准路线、九路径与 final merge gate。

状态：`completed`。批准引用为 Issue 评论 `5129028445`。

### Node 2：Security RED

1. fresh `cargo audit` 必须真实更新 advisory database。
2. 如果直连 fetch 失败，先确认错误位于网络层，不把网络退出码当漏洞 RED。
3. 按 `err.md` 的已验证当前 shell 代理恢复后重试。
4. 保存两条目标公告、`350` 个依赖、严重度与退出码 `1`。

状态：`completed`。RED 为 `RUSTSEC-2026-0194/0195`，两条均 `7.5 high`；另有两条允许
unmaintained warning。

### Node 3：Planning Freeze

1. 写入 Plan、Session Plan、Runtime Workflow。
2. 更新 AGENTS、build、Master Plan 与 err。
3. 验证七路径 planning count/hash 和 Markdown 基础质量。
4. 运行 Git snapshot checkpoint，建立 `issue-105-planning-freeze` 提交。
5. 在干净 checkpoint 上运行 AGOS default-entry `-ReportOnly`。

状态：`completed`。七路径 planning checkpoint 为
`c266cab2570ce477bc6079b951cd9e79f5abe4a0`；Git snapshot 为 `GIT_SNAPSHOT_READY`。
AGOS default-entry 返回 `needs-input/unregistered/missing-owner-scope-manifest`，按项目规则绕过，
没有修改 AGOS。

### Node 4：Lockfile GREEN

1. 执行 `cargo update -p wayland-scanner --precise 0.31.11`。
2. 立即检查 `git diff -- Cargo.lock`。
3. 要求只出现 quick-xml 与 wayland-scanner 的 version/checksum 变化，共 `4/4` 行。
4. 检查当前图包含 `quick-xml 0.41.0` 与 `wayland-scanner 0.31.11`，不含旧版本。
5. 运行 fresh `cargo audit --json`，要求 `vulnerabilities.count=0`。

状态：`completed`。Security checkpoint 为
`7c890e3137a503cac334e2802bd2441feae41052`；锁文件差异精确为 `4/4`，fresh audit 扫描
`350` 个依赖后得到漏洞 `0`、两条单列 unmaintained warning。

### Node 5：Local Closeout

1. 创建 `docs/reports/issue-105-quick-xml-rustsec-remediation.md`。
2. 执行 `build.md` 的 Issue #105 完整本地轻量命令链。
3. 验证九路径/hash、许可证/MSRV、仓库政策、Release Audit 与 diff check。
4. 运行 Git snapshot checkpoint并建立 `issue-105-local-verified` 提交。

状态：`completed`。`build.md` 完整命令链输出 `ISSUE_105_LOCAL_GREEN`；fresh audit 漏洞 `0`、
CI contract `35/35`、Repository Policy `0` 违规、Release Audit `current`、九路径/hash 精确。
报告已落盘，local-verified checkpoint 为 `2d3f37a0b31fcfe10501e14033665f0c14ae4ffd`。

### Node 6：Remote Delivery

1. 使用本机默认时间提交，普通 push，禁止 force push。
2. 创建关联 Issue #105 的非 Draft PR。
3. 请求 Review；每条反馈按根因、处理和 fresh 证据闭环。
4. 核验 Final Head 的 CI 七 Job、Performance Baseline 四 Job、Artifact 与 conversations。
5. 只在所有门禁通过后请求绑定 Final Head 的独立 Squash Merge 授权。

状态：`pending`。

## Scope Enforcement

最终允许路径固定为：

```text
AGENTS.md
build.md
Cargo.lock
docs/plans/2026-07-30-issue-105-quick-xml-rustsec-remediation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-30-issue-105-quick-xml-rustsec-remediation.md
docs/reports/issue-105-quick-xml-rustsec-remediation.md
docs/workflows/2026-07-30-issue-105-quick-xml-rustsec-remediation-runtime.md
err.md
```

范围 hash 为 `sha256:0c90d018e06aa640d33a4c65c75aea45c89eb0e365b91fadee803b0426c8c58f`。
范围漂移必须停止、重新计算、回写 Issue 并重新取得批准。

## Package Diff Contract

基线 package 元数据：

```text
quick-xml 0.39.4 checksum cdcc8dd4e2f670d309a5f0e83fe36dfdc05af317008fea29144da1a2ac858e5e
wayland-scanner 0.31.10 checksum 9c324a910fd86ebdc364a3e61ec1f11737d3b1d6c273c0239ee8ff4bc0d24b4a
```

目标 package 元数据：

```text
quick-xml 0.41.0 checksum e660451e55124f798a69a5af3f49ccfbefbd41910eefd25caf2393e1f3473ec1
wayland-scanner 0.31.11 checksum 338e30461b3a2b67d70eb30a6d89f8e0c93a833e07d2ae89085cd070c4a00ac0
```

任何第三个 package、dependency 数组、source 或 manifest 变化均为硬停止。

## Verification Gates

### Planning Gate

- 七路径精确匹配 `sha256:25337a6c...`；
- CI contract、Repository Policy、Release Audit 与 `git diff --check` 通过；
- planning commit 后工作树干净；
- AGOS 仅 report-only，不写外部控制面。

### Implementation Gate

- security RED 已观察且归因于旧锁版本；
- cargo update 命令与批准命令完全一致；
- package diff 为两个 crate；
- fresh audit 漏洞总数为 `0`，warning 与漏洞分离。

### Delivery Gate

- 九路径/hash、许可证、MSRV 与本地轻量门禁通过；
- 非 Draft PR 关联 Issue #105；
- 所有 Review conversations 根因闭环；
- Hosted CI/Performance 成功，成功 Artifact 为 `0`；
- Final Head 取得单独 Squash Merge 授权。

## Error Watchlist

- advisory-db fetch 失败：先判断网络层，复用 `err.md` 当前 shell 代理恢复；
- `cargo tree --offline` 因本机 package cache 不完整失败：不把缺缓存误判为 lockfile 回归，
  使用批准的在线 audit/解析与 Hosted CI；
- cargo update 出现第三个 package：停止，不使用宽泛 `cargo update`；
- warning 漂移：重新审查 advisory 类型，禁止把 informational warning 伪装成漏洞或已修复；
- Review/CI 失败：确定根因后最小修复，不靠无证据重跑；
- AGOS unregistered/needs-input/incompatible：记录并绕过，不修改 AGOS。

## AGOS Boundary

AGOS 只提供 report-only 辅助。任何 unregistered、needs-input、missing manifest 或接口异常都按
inputcodex 项目规则记录后绕过，不得阻塞 Issue/PR/Review/CI，也不得在本分支修改外部仓库。

## Rollout Draft

- `business_path`: `maintenance/dependency-security/quick-xml-rustsec-remediation`
- `node_order`: `red -> scope-freeze -> exact-parent-patch -> package-diff -> green -> hosted-ci`
- `skills`: `brainstorming, writing-plans, tdd, systematic-debugging, security-review, verification`
- `failure_recovery`: `network-layer-separation, proxy-retry, exact-package-stop, review-root-cause-closure`
- `closeout_condition`: `final-head-review-ci-artifact-green-and-owner-merge-authorization`
