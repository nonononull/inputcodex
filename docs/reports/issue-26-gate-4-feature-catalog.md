# Issue #26：Gate 4 功能目录、行为合同与脱敏夹具报告

schema_version: inputcodex.report.v1
report_status: source-pr-squash-merged-independent-closeout-in-progress
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/26
branch_ref: codex/issue-26-gate-4-feature-catalog
baseline_ref: 431682296f53e86de1184c732b0d4748857c9390
approved_decision_ref: user-message:create-issue-26-session-plan-runtime-scope-hash-2026-07-22
scope_hash: sha256:e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772
control_plane_checkpoint_ref: commit:80e0ddbb734496e95e89fe57fd89ddb668c8c276;issuecomment:5047590347
implementation_decision_ref: user-message:approve-issue-26-implementation-2026-07-22
implementation_approval_ref: issuecomment:5047650154
feature_catalog_checkpoint_ref: commit:87537e6e4a0e6911dd1427cc23f52dcb805a4679;issuecomment:5048930060
source_pr_ref: https://github.com/nonononull/inputcodex/pull/27
source_pr_head_ref: 1d1bf32cdc4edc45e2d28f1047604222ebdb51e4
source_pr_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/29942593564
main_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/29943399832
review_ref: github-pr-27-review-threads-0
merge_ref: commit:a9b20f00ae069aedd42c8124d2789b230187258c;parent-count:1;tree:205c24e05e0451a3aa39af4f43f0d9853cc7a6a2;github-signature:valid
owner_merge_authorization_ref: issuecomment:5049442605
branch_cleanup_ref: issuecomment:5049570338
closeout_issue_ref: https://github.com/nonononull/inputcodex/issues/28

## 一、当前结论

- Issue `#26` 已以 `CLOSED` 状态关闭，标签为 `type:architecture`、`gate:4`。
- PR `#27` 已以最终 Head `1d1bf32cdc4edc45e2d28f1047604222ebdb51e4` Squash Merge 至 `main`，合并提交为 `a9b20f00ae069aedd42c8124d2789b230187258c`。
- Squash 提交只有一个父提交，merge/head tree 均为 `205c24e05e0451a3aa39af4f43f0d9853cc7a6a2`，GitHub 签名为 `valid`。
- PR `#27` 的 `classify`、`governance`、`linux-quality`、`windows`、`macos` 与 `required` 均成功；合并后 main CI 同样六 Job 成功，两个运行的 Artifact 数均为 `0`。
- PR `#27` Review 对话总数与未解决数均为 `0`；项目所有者授权证据为 Issue 评论 `5049442605`。
- 来源功能分支的远端、本地与远端跟踪引用均已清理；远端删除证据为 Issue 评论 `5049570338`。
- 动态合并证据只能通过独立 Issue `#28` Closeout PR 回写；该 Closeout 不新增功能、性能基线、优化或一致性例外。

## 二、Fresh 基线

- Gate 4 规划 PR `#25` 已 Squash Merge 为 `431682296f53e86de1184c732b0d4748857c9390`，Issue `#24` 已 COMPLETED。
- 合并后 main CI 运行 `29926710342` 六 Job 全绿，成功 Artifact 数为 `0`。
- GitHub API 于 2026 年 7 月 22 日复核最新正式 Release 为 `v1.2.41`，发布时间 `2026-07-20T01:48:40Z`。
- Release tag 提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`；本仓缓存与 source-lock 继续作为审计输入。
- 上游 `main` 为 `91376ee3518cb5fe5ec8eead179418f706c25870`，只由 Issue `#20` 预警。
- 搜索引擎一度返回陈旧 `v1.1.7` 页面；直接 GitHub API 与 tag ref 一致证明这是搜索缓存陈旧，不构成功能真源漂移。
- `.codegraph/` 不存在，未初始化索引。
- AGOS default entry 已使用 report-only 运行，返回 `needs-input / unregistered`；根据 `inputcodex` 合同记录并绕过，未修改外部 Registry、Workflow、Rules 或 Vault。

## 三、范围与哈希

- 整个 Issue 最大候选写集合为 36 条路径或路径模式。
- 规范化：`StringComparer.Ordinal` 升序、UTF-8 无 BOM、LF 分隔、保留末尾 LF。
- 范围哈希：`sha256:e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772`。
- control-plane checkpoint 曾只允许 8 条路径；项目所有者批准后，实施阶段按 36 条最大范围分批写入。
- 当前实际变更仅包含已批准的 parity Rust、测试、`parity/` 功能目录、子项目 build/err 与任务控制面；CI、upstream、benchmarks、产品、Ruleset、Release 和 AGOS 保持零差异。

## 四、冻结的实现结构

1. `parity/features/source-index.yml` 记录全部已审计上游入口及其 feature/排除/exception 映射。
2. 五个 `parity/features/<domain>.yml` 保存稳定功能目录。
3. 五个 `parity/contracts/<domain>.yml` 保存行为合同。
4. `parity/fixtures/<feature-id>/` 保存合成或不可逆脱敏 fixture。
5. `inputcodex-parity` 使用纯 Rust 类型与测试验证 schema、路径、引用、状态、敏感信息和覆盖关系。
6. 目录和验证器不进入桌面发布运行面，不依赖 Iced、platform 或 presentation。

## 五、TDD 与交付批次

- Batch 1：控制面 checkpoint，提交后等待 owner 批准。
- Batch 2：依赖 Fresh 元数据与 catalog/contract/fixture RED 测试。
- Batch 3：最小 Rust schema 与验证 GREEN。
- Batch 4：source-index 与五分域功能目录。
- Batch 5：行为合同与脱敏 fixture。
- Batch 6：本地验证、PR、Review/CI 与 owner merge gate。

每个批次都必须普通提交、普通 push 并回写 Issue；禁止 amend 已推送证据或 Force Push。

## 六、依赖候选证据

- Serde 候选：`1.0.229`，MIT OR Apache-2.0，只计划启用 `derive`。
- YAML 候选：`yaml_serde 0.10.4`，MIT OR Apache-2.0，提供 Serde YAML 序列化与反序列化。
- Fresh crates.io 证据确认 `yaml_serde 0.10.4` 未撤回、MSRV `1.82`、checksum `08c7c1b1a6a7c8a6b2741a6c21a4f8918e51899b111cfa08d1288202656e3975`；Serde `1.0.229` 未撤回、MSRV `1.56`、checksum `4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba`。
- 两个依赖均使用 `MIT OR Apache-2.0`，满足项目许可证兼容要求并低于 Rust `1.97.1`。

## 七、禁止与分流

- 不迁移产品功能，不创建性能基线或预算，不修改 CI、Ruleset、Release、upstream 或 AGOS。
- Tauri/React 管理界面、注入脚本和远程推荐列表只作为审计证据，不进入运行面。
- 无效功能、有害副作用、错误语义或双平台冲突只能登记为 `exception-pending`，并候选创建独立 parity-exception。
- 未经 owner 决定，不得把条目标记为 `exception-approved` 或 `retired`。
- 证据不足时必须记录缺口，不能宣称目录完整。

## 八、RED schema 证据

- `catalog_schema --no-run`、`contract_schema --no-run` 与 `fixture_safety --no-run` 均使用既有 Rust `1.93.1` 加 `--ignore-rust-version` 进行本地定向编译，退出码均为 `1`。
- 三组首要诊断均为 `E0432`，分别证明 catalog、contract 和 fixture 的预期公开 API 尚不存在，符合 TDD RED 根因。
- 输出中的 `E0282` 是目标函数未解析后返回类型未知的级联诊断；依赖下载、Cargo 锁定、YAML 测试文本和测试语法未产生错误。
- source-index 的 6 条内存 RED/GREEN 与真实仓库对账已运行；完整合同和 fixture 仓库验证仍等待 Phase 5 数据面。

## 九、Rust schema GREEN 证据

- catalog schema 覆盖稳定 domain/status、完整 ID、重复 ID、必填元数据和 Windows/macOS 字段。
- contract schema 覆盖六加载状态、请求标识、必填行为段、合同身份、重复 ID、悬空 feature/fixture 与跨 feature fixture 引用。
- fixture schema 覆盖 manifest 必填字段、重复/归属、相对路径、反斜杠、Windows/POSIX 私人绝对路径和结构化敏感值占位策略。
- 四个定向测试目标共 `26` 个测试通过；格式、离线库 check 与 Clippy `-D warnings` 均通过。
- Phase 4 数据面已由真实 source-lock 和上游入口枚举验证为 `133/133`；尚未把缺失合同/fixture 伪造成完整仓库 GREEN。

## 十、Phase 5 本地验证证据

- 8 条控制面路径、36 条范围、scope hash、占位符、治理合同、仓库政策和空白检查已通过。
- 普通 control-plane checkpoint `80e0ddbb734496e95e89fe57fd89ddb668c8c276` 已 push。
- Issue `#26` 评论 `5047590347` 已回写 commit、计划引用、范围哈希和实现待批准边界。
- RED checkpoint `532fba89d882862438345788ed2fdd73faede507` 已普通 push，并通过 Issue 评论 `5048079257` 回写。
- GREEN checkpoint `8b18f0a2a37829af3338edba34454eb6690af77a` 已普通 push，并通过 Issue 评论 `5048438316` 回写。
- source-index 与五域功能目录 checkpoint `87537e6e4a0e6911dd1427cc23f52dcb805a4679` 已普通 push，并通过 Issue 评论 `5048930060` 回写。
- 五个 contract 文件通过 domain、stable ID、fixture policy、引用完整性和每 feature 合同覆盖验证；合同总数为 `36`。
- fixture 目录与 manifest feature ID 一致，`11` 个 manifest 的所有 payload 都是声明文件、非符号链接、非空、仓库内路径且不含真实敏感值或私人绝对路径。
- `仓库功能目录通过完整引用与安全验证`、完整 parity 包测试、格式、离线 check 和 Clippy 严格门禁均已取得本地 GREEN；项目级 CI 合同 `30/30`、仓库政策 `0` 违规和 `git diff --check` 也已通过。
- 文本控制字节扫描确认仅发现 Phase 4 提交遗留的两处 ESC；PowerShell 双引号反引号转义是根因。已修复、在 `err.md` 记录，并通过先 RED 后 GREEN 的 `parity_文本文件不包含非法控制字节` 回归覆盖。
- 本来源实现已完成；PR、CI、Review、Squash 和分支清理证据由独立 Issue `#28` 继续回写。性能基线必须在该 Closeout 合并后使用新的独立 Issue 建立。

## 十一、来源 PR 合并与 Closeout 状态

以下字段均已通过 Issue `#28` 的 Fresh GitHub/Git 复核：

```text
feature_count: 36
contract_count: 36
fixture_count: 11
fixture_manifest_count: 11
source_entry_count: 133
excluded_entry_count: 3
exception_pending_count: 10
coverage_gap_count: 0
red_checkpoint_ref: commit:532fba89d882862438345788ed2fdd73faede507;issuecomment:5048079257
green_checkpoint_ref: commit:8b18f0a2a37829af3338edba34454eb6690af77a;issuecomment:5048438316
feature_catalog_checkpoint_ref: commit:87537e6e4a0e6911dd1427cc23f52dcb805a4679;issuecomment:5048930060
phase5_checkpoint_ref: commit:c50ec7b911f72fa935dd033cfef4050876548c21;issuecomment:5049288893
pr_ref: https://github.com/nonononull/inputcodex/pull/27
final_pr_head_ref: 1d1bf32cdc4edc45e2d28f1047604222ebdb51e4
pr_ci_ref: run:29942593564;status:success;jobs:6;artifacts:0
main_ci_ref: run:29943399832;status:success;jobs:6;artifacts:0
review_ref: total:0;unresolved:0
merge_ref: commit:a9b20f00ae069aedd42c8124d2789b230187258c;parent-count:1;tree:205c24e05e0451a3aa39af4f43f0d9853cc7a6a2;github-signature:valid
owner_merge_authorization_ref: issuecomment:5049442605
branch_cleanup_ref: remote:absent;local:absent;remote-tracking:absent;issuecomment:5049570338
closeout_issue_ref: https://github.com/nonononull/inputcodex/issues/28
```
