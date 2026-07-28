# inputcodex 项目规则

## 沟通

- 所有项目沟通、文档和代码注释使用中文。
- 软件正式名称固定为 `inputcodex`；如需修改，必须先获得项目所有者批准。

## 产品约束

- 禁止加入广告、推广位、广告 SDK、付费导流或隐蔽遥测。
- 性能、稳定性和可诊断性优先于功能数量。
- Windows 与 macOS 从首版起保持功能一致。
- 禁止 TypeScript、JavaScript 业务代码和 WebView；桌面产品使用 Rust 与 Iced。
- Iced 只能存在于展示层，领域、应用、存储、网络和平台层不得依赖 Iced 类型。
- 功能加载必须具备明确状态、超时、取消、错误隔离和可观测证据。
- 未经方案评审，不照搬 `BigPizzaV3/CodexPlusPlus` 或 `zsr131550/CodexPlusPlus` 的架构。
- `BigPizzaV3/CodexPlusPlus` 最新正式 Release 是唯一功能真源，`main` 仅作为变化预警源。
- 无效功能、有害副作用或错误语义争议必须建立一致性例外 Issue，由项目所有者决定后才能实现差异。
- 上游 Tauri/React 管理界面、现有注入脚本和远程推荐列表只能作为完整快照中的审计输入，不得直接进入新架构或最终运行面；需要保留其背后的有效能力时，必须建立独立功能或一致性例外 Issue。

## 开发约束

- 修改前先读取 `README.md`、`build.md`、`err.md` 和当前任务计划。
- 遇到错误先查 `err.md`，重复问题优先复用已有结论。
- 可单独构建的项目或子项目必须在其根目录维护 `build.md` 和 `err.md`。
- AGOS 仅作为可选外部治理辅助：可用且适合当前任务时可以使用，其输出只能补充本项目证据，不能替代项目原生控制面。
- AGOS 不可用、未登记、返回 `needs-input`、接口不兼容或执行异常时，记录原因后立即绕过，继续执行 `inputcodex` 的项目原生流程；这些状态不得阻塞本项目 Issue、PR、Review、验证或合并决策。
- `inputcodex` 的 Issue、分支和 PR 禁止修改、修复或优化 AGOS 的脚本、规则、Registry、Workflow、Vault 或其他跨仓控制面；发现问题只记录为外部缺口，任何 AGOS 改动必须由项目所有者另行批准为独立跨仓任务。
- 改动保持小而可验证；禁止顺手重构无关代码。
- 导入第三方或参考项目代码前，必须确认许可证、来源、提交和保留声明要求。
- 架构与功能实现使用测试或可重复测量证据驱动，完成前运行项目定义的验证命令。
- Rust 开发默认采用“本地轻量验证 + GitHub Actions 全量验证”：本地只执行 `build.md` 定义的快速、定向命令，Workspace 全量检查、Windows/macOS 编译测试和发布构建交给公开仓库的标准 GitHub-hosted runners。
- Git 提交、分支操作和本地审计的时间判定以项目所有者 Windows 本机时间为唯一来源，使用 `Get-Date` 读取；禁止用助手会话元数据、网络/API、CI 服务端或推测时间替代或覆写本机时间。
- 未经项目所有者明确批准，禁止设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE` 覆写 Git 默认本机时间；GitHub 事件时间必须按服务端原值单独记录，不得与本机时间混写。
- 禁止默认使用 Larger Runner 或项目所有者本地机器作为 self-hosted runner；任何收费 Runner、self-hosted runner 或新的付费 CI 资源必须先建立独立 Issue 并获得项目所有者批准。
- CI 必须限制重复运行、超时、Cache 和 Artifact；禁止上传整个 `target/`，非 Release Artifact 最长保留 7 天。CI 尚未稳定前不得把检查加入 `main` Ruleset。
- 所有正式工作必须执行 `Issue → 分支 → 验证证据 → 关联 PR → Review/CI → Merge`，禁止直接向 `main` 写功能。
- 所有 PR 合并到 `main` 必须使用 Squash Merge；禁止 Merge Commit 和 Rebase Merge，确保每个 Issue 在 `main` 上形成一条可独立追踪和回滚的提交。
- 永久禁止对 `main` 使用 `git push --force` 或 `git push --force-with-lease`，项目所有者和管理员也不例外；历史错误必须通过 `revert` 提交和关联 Issue/PR 修正，紧急情况不能绕过该规则。
- 永久禁止删除 `main`，项目所有者和管理员也不例外；若发生误删，必须从删除前最后一个权威提交恢复同名分支并建立事故 Issue，不得借恢复改写历史。
- 所有 Review 对话必须在合并 PR 前解决；每条解决记录必须写明根因、处理方式和验证证据。若反馈被判定为不成立，必须提供可复核证据并取得 reviewer 或项目所有者确认；禁止仅点击 `Resolve conversation`、忽略根因或带着未解决对话合并。
- 仓库只有一名具备合并权限的人类维护者时，平台 required approvals 设为 `0`，但每次合并前必须在关联 Issue 或 PR 中保留项目所有者的明确决策证据。
- 当第二名具备 `write`、`maintain` 或 `admin` 权限的人类维护者加入时，必须在下一次 PR 合并前把 required approvals 提升为 `1`；Bot、GitHub App 和自动化账号不计入维护者人数。
- 上游缓存同步 PR 只能更新 `upstream/` 与同步报告；功能重构必须使用独立 Issue 和 PR。
- 客户端更新、安装包、签名和下载地址只能指向 `nonononull/inputcodex`。

## UI 边界

- UI、视觉和交互方案默认交给 Gemini；只有用户明确要求当前助手实现 UI 时才执行。
- 未确定设计系统前，不创建临时 UI 作为事实标准。

## 当前 Gate 边界

- Gate 3 七成员纯 Rust Workspace、首版三平台 CI、失败语义和冷构建最低基线已通过 Issue `#19` / PR `#21` 进入 `main`；仓库已包含最小应用骨架，但尚未迁移任何上游业务功能。
- Gate 3 合并证据已通过 Issue `#22` / PR `#23` 完成独立 closeout；合并提交为 `f470c062037042a1f7833a29cdcf216f6c0f5601`，Issue `#22` 已按 `COMPLETED` 关闭，合并后 `main` CI 六 Job 全绿。
- Issue `#24` / PR `#25` 已完成 Gate 4 规划合同并 Squash Merge 为 `431682296f53e86de1184c732b0d4748857c9390`；Issue `#24` 已按 `COMPLETED` 关闭。
- Issue `#26` / PR `#27` 已完成 Gate 4 功能目录执行、Review/CI 与 Squash Merge；来源事实只能通过独立 Closeout 回写，不得改写来源提交。
- Issue `#28` / PR `#29` 已完成 Gate 4 独立 Closeout；PR `#29` 以单父 Squash 提交 `c07da0cad33e09b5c54e528a8a6728a048c88c0b` 进入 `main`，Issue `#28` 已关闭，合并后主干 CI 六 Job 全绿且 Artifact 数为 `0`。
- Issue `#35` / PR `#36` 已将活动上游快照与功能目录审计基线解耦，并以 Squash 提交 `d7438a0f2c43b7fbd2b159b3759aacea4ef1999e` 进入 `main`。
- Issue `#34` / PR `#40` 已将最新正式 Release `v1.2.42` 缓存为只读审计输入；PR `#40` 以 Squash 提交 `353391424db5514d022473ba97f601486a190869` 合并，合并后主干 CI Run `30147841226` 七 Job 全绿。
- Issue `#41` / PR `#42` 已修复快照值与 CI 基线断言的错误耦合，并以 Squash 提交 `8aa1d4c96b0543e766b477b1b8e9652968b55f92` 合并；合并后主干 CI Run `30147071062` 七 Job 全绿。
- Issue `#43` / PR `#44` 已完成 `v1.2.42` 缓存与 CI 合同状态收口；PR `#44` 以单父 Squash 提交 `fdb2f98c701800969fc478f95cd2539be598faaa` 合并，合并后主干 CI Run `30152001233` 成功。
- Issue `#38` / PR `#45` 已完成 `v1.2.42` 二十六路径功能目录重新审计；最终 Head `d3df8759bdb9c6378497a3a0c8f409c3968f4d4f` 以单父 Squash 提交 `5fd337fb7ceb9b0ef53e2e694cc5ddd81ea0a98c` 进入 `main`，tree 为 `5273d0e42483bbb5c629a2243fc24f0a892b3db3`，GitHub 签名 `valid`，Issue `#38` 已按 `COMPLETED` 关闭。
- PR `#45` 的最终 Head CI Run `30157623932` 与合并后 `main` CI Run `30158058627` Attempt `3` 均为七 Job 全绿且 Artifact 数为 `0`；Attempt `1/2` 的零 Job 失败已由 Issue `#46` 证明为 GitHub Actions 外部事故，Issue `#46` 已按 `COMPLETED` 关闭。
- Issue `#47` 被指定为上述稳定事实的微型五路径 Closeout；其 GitHub Issue/PR 保存自身动态 Review、CI 与合并证据，不得再为本次状态回写创建二次 Closeout，也不得借机修改产品、缓存、CI 或历史任务快照。
- Issue `#32` / PR `#49` 已完成隔离 Rust 测量工程、opt-in 首次 view 探针、PowerShell 证据验证器和 Windows/macOS hosted Workflow；PR `#49` 以单父 Squash 提交 `fd9db9ca1c150b7db34dda8acc09b6f0cc357a17` 进入 `main`，tree 为 `3fc4a5a7697850f048edcedf6a9ec5e4f76c847c`，Issue `#32` 已按 `COMPLETED` 关闭。
- PR `#49` 合并后主 CI Run `30171903289` 七 Job 全绿且 Artifact 为 `0`；Performance Run `30171903279` 在 Evidence 模式四 Job 全绿且 Artifact 为 `0`。Windows `core.autocrlf` 原始哈希误报已通过 fresh checkout 和 TDD 根因闭环，失败诊断证据按既有保留合同处理。
- Issue `#32` 的完成只表示性能基线和可审计样本就绪，不表示任何性能预算数值已经批准；现有样本只能用于同平台、同可比环境趋势，禁止跨平台排名或把单次 Run 直接设为 required budget。
- Issue `#50` / PR `#51` 已完成九路径性能预算 Discovery；最终 Head `e0154c61d8b05835db10437c79f029909516eac1` 以单父 Squash 提交 `fea8824c652665df710a7e6ef941854060eb6e1f` 进入 `main`，tree 为 `9fb518cda8b35a9388fb9fce0a1ff6ba976d80cb`，GitHub 签名 `valid`，Issue `#50` 已按 `COMPLETED` 关闭。
- PR `#51` Final Head CI Run `30174131581` 的七个预期 Job 均为成功或合同性跳过，`required` 成功且 Artifact 为 `0`；合并后主干 CI Run `30175592979` 七 Job 全绿且 Artifact 为 `0`。
- 性能预算 Discovery 的稳定结论是“方法已批准、预算数值未批准”：Windows/macOS 必须分别收集至少五次独立可比 Run，下一合法工作只能是独立性能复测与数值批准 Issue；预算 CI、性能优化和 Gate 5 产品迁移必须继续使用不同 Issue/PR。
- Issue `#55` 为 Issue `#54` 修复唯一的前置合同缺口：`Performance Baseline` 的手工触发必须显式选择 `evidence` 或 `measure`，默认 `evidence`，自动 PR/push 语义、hosted Runner、超时、并发、Artifact 保留和 `target/` 禁止保持不变；因 Workflow/CI 合同属于实现哈希，合并前只能用本 Issue 成功 Artifact 刷新三份 Issue `#32` Evidence。该入口本身不是预算 CI。
- Issue `#54` 已在 Issue `#55` 合并后完成八次严格串行 GitHub-hosted 复测；Windows 仅得到 `2` 次初始可比队列与 `4` 次次级队列，未满足五次硬约束，数值预授权未触发。该 Issue 保持 `STOPPED_AT_EIGHT_RUN_CAP`，不得创建 `run-09`、预算值、预算 CI、优化或 Gate 5 工作。
- Issue `#57` / PR `#58` 已完成 Hosted Windows CPU 队列异构性 Discovery；PR `#58` 以 Squash 提交 `d9d1ed77b9796ac6a99e250d1547217a39426aa9` 进入 `main`，合并后主干 CI Run `30185092327` 七 Job 全绿且 Artifact 为 `0`，Issue `#57` 已在所有者选择方案 A 后按 `COMPLETED` 关闭。
- Issue `#59` 已完成四次固定串行复测：`run-02` 与 `run-04` 精确命中 AMD EPYC 7763 完整环境指纹 `sha256:f3954543f3cec519568345d9f40341ddeb8991a7d93b3a274cc324b047fb00cb`，Windows 严格目标队列为历史 `3` + 新命中 `2` = `5`，macOS 同队列为 `12`；四个 Run 的成功 Artifact 均已删除并复核为 `0`，不存在 `run-05`。
- Issue `#59` / PR `#60` 已完成预算数值交付；Final Head `61c088d74d61a329fbe67e14b8280dfa9701c6b2` 以单父 Squash 提交 `e225144831a0928bfa3aaa0d169a054779005812` 进入 `main`，tree 为 `56eb1e8d95dfce22726c1aef1bdde1c353af055e`，GitHub 签名 `valid`，Issue `#59` 已按 `COMPLETED` 关闭。
- PR `#60` Final Head CI Run `30194465259` 七 Job、Performance Baseline Run `30194465231` 四 Job均全绿；合并后主干 CI Run `30194897171` 七 Job、Performance Baseline Run `30194897166` 四 Job均全绿，四个成功 Run 的 Artifact 数均为 `0`。
- `benchmarks/budgets/issue-59-approved-observation.json` 已进入 `main`，归一化 SHA-256 为 `sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5`，离线构建与复算合同为 `BUDGET_APPROVAL_GREEN passed=10`；`budget_ci_enabled=false`、`gate_5_unlocked=false` 继续保持。
- Issue `#54` 的十六份原始 JSON 与 manifest 在 Issue `#59` 中只能作为只读历史证据缓存；不得修改其原始 `new-cohort-valid` 分类、来源 Run、哈希或任何样本内容。
- 合并后稳定状态由 `docs/reports/issue-52-performance-budget-closeout.md` 固化；本 Closeout 自身的动态 Head、CI、Review、授权与合并证据只保留在 GitHub Issue/PR 评论，不得递归创建同类状态 Closeout。
- Issue `#61` 是 Issue `#59` / PR `#60` 稳定事实的八路径反递归 Closeout；其自身动态 Head、CI、Review、授权与合并证据只保留在 GitHub Issue/PR 评论，合并后不得再次创建同类 Closeout。
- Issue `#64` / PR `#66` 已将最新正式 Release `v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c` 缓存为完整只读审计输入；单父 Squash 提交为 `15e91708b41548f523e26ede4c7ca4de41badf77`，合并后主干 CI Run `30214249228` 七 Job 全绿且 Artifact 为 `0`。
- Issue `#63` / PR `#73` 已完成非 required `approved-observation` 预算 CI；Final Head `e742227086a3457ea876a91fc84c2d115257cf40` 以单父 Squash 提交 `19d1824398d46a0d4f6b9e4805905485793d3c9d` 进入 `main`，tree 为 `8e44390053b9a839a19be6357b833583fbbb6a5a`，GitHub 签名 `valid`。PR `#72` 合并后的主干 CI Run `30244760762` 七 Job、Performance Baseline Run `30244760739` 四 Job全绿且 Artifact 均为 `0`；Issue `#63` 已按 `COMPLETED` 关闭。
- `Performance Baseline` 的自动 PR/push 固定使用 `observation`；四种阈值分类均不得阻断，预算或结果合同错误必须阻断，成功 observation 禁止上传 Artifact，失败只允许保留七天最小诊断且不得包含 `target/`。
- Issue `#65` / PR `#72` 已在批准的二十四路径与 `sha256:82234e7aacce0bd6c57994529ccf74371052ed906dc8371324b90e41f697d7b7` 内完成 `12/12` GREEN；Final Head `75d6b67deecaa84f05272427cfbcc46b9753b903` 以单父 Squash 提交 `fc1683aabda4afb27ca333387ec954b6a405d2df` 进入 `main`，Issue `#65` 已按 `COMPLETED` 关闭，`release_audit=current`。
- Issue `#74` 已批准并关闭平台路径一致性例外：显式应用路径或非空 `CODEX_HOME` 无效时必须明确失败，禁止回退到相对目录、当前目录或其他自动探测结果。
- Issue `#75` / PR `#76` 已完成首个 Gate 5 平台路径迁移；Final Head `ba082669cd6d491cce26e29efcaa249786973a39` 以单父 Squash 提交 `a06a97fd59ce125306a13202c8f1a07656c797a0` 进入 `main`，tree 为 `b669aa6610e976542a74f404ff4f87b36864816b`，Issue `#75` 已按 `COMPLETED` 关闭。合并后主干 CI Run `30276472184` 七 Job、Performance Baseline Run `30276476891` 四 Job全绿且 Artifact 均为 `0`。
- Issue `#77` 已批准应用概览方案 A 并按 `COMPLETED` 关闭：应用概览按领域拆分，历史启动记录不得冒充实时运行状态，损坏状态不得静默视为无记录。
- Issue `#78` / PR `#79` 已完成第二个 Gate 5 应用概览只读事实迁移；Final Head `1bae5d51850c3538c3e161e73e266ac19f7406b3` 以单父 Squash 提交 `ef69494d92c7c461b0cb858e95f6838404ae1a61` 进入 `main`，tree 为 `936cc74fbceae2a3ee8d98b924c836e13d9f7ae3`，GitHub 签名 `valid`，Issue `#78` 已按 `COMPLETED` 关闭。合并后主干 CI Run `30289461278` 七 Job、Performance Baseline Run `30289461109` 四 Job全绿且 Artifact 均为 `0`。
- Issue `#80` 已批准版本与启动意图方案 A 并按 `COMPLETED` 关闭：版本只来自编译期 `CARGO_PKG_VERSION`；精确 `--show-update` 或 `INPUTCODEX_SHOW_UPDATE=1` 请求展示更新；非法显式环境值必须以 `INVALID_STARTUP_OPTION` 失败。
- Issue `#81` 是当前第三个 Gate 5 产品切片；二十三路径与 `sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe` 已获批准。规划、Domain、Application、Platform 与 Parity TDD checkpoint 已完成；当前切片只读取当前进程参数、`INPUTCODEX_SHOW_UPDATE` 和编译版本，不打开 UI、不联网、不检查或执行更新、不写文件、不缓存、不启动线程、不引入新依赖，并且 `source-index.yml` 保持不变。四 crate 测试/Clippy、格式、CI 合同 `35/35`、`release_audit=current`、仓库政策 `0` 违规、候选范围 `23`/实际差异 `22`、受保护路径、隐私、旧变量和禁止能力检查已通过；`err.md` 因无新根因保持未修改。本地验证 checkpoint 为 `73bd5748d2341a92577ab8273b0db6f7bdb6a265`，当前只剩普通推送、非 Draft PR 与 Hosted Review/CI，最终 Squash Merge 仍须单独授权。
- Gate 5 只能按独立功能 Issue/PR 串行推进；Issue `#81` 未完成前不得夹带第四个产品 feature、UI、预算、Release、`upstream/`、Ruleset 或 AGOS 改动。
