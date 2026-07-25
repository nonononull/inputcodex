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
- Issue `#32` 的 28 路径实施范围与 `sha256:857f6a8a2070d5ddcb43eaf237448d30302d59e39e1dbb910724cfac2fc81505` 已获项目所有者批准；PR `#49` 已建立隔离 Rust 测量工程、opt-in 首次 view 探针、PowerShell 证据验证器与 Windows/macOS hosted Workflow。
- 初始 Performance Run `30169262247` Attempt `1` 已生成两平台样本；最终 Head `e679eee64442f0ae4db97b4e9cdbfab6780ea1de` 的 Run `30170128309` 随后证明 Windows 新鲜检出会因 `core.autocrlf=true` 改写 JSON 工作树字节，导致原始文件哈希误报，而同 Run macOS Evidence 与主 CI Run `30170128326` 均成功。
- CRLF 根因已通过 fresh checkout 复现，并以 TDD 将结果校验改为换行归一化文本哈希；旧结果绑定旧 `implementation_sha256`，不得篡改元数据冒充当前实现，固定结果已移除以触发新一轮 hosted 重测。最终样本仍只能用于同平台、同环境指纹趋势；禁止跨平台排名、数值预算、性能优化、功能迁移、上游/半成品运行和范围外修改，Gate 5 保持锁定。
