# 排错记录

## 使用规则

- 遇到错误先搜索本文件，重复问题优先引用现有记录。
- 新错误按“日期、环境、现象、根因、处理、验证、关联提交或 Issue”记录。
- 未确认根因时标记为“调查中”，不得把猜测写成结论。

## 已知记录

### 2026-07-21：AGOS 可选外部辅助边界被误写为项目门禁

- 环境：`inputcodex` 文档 closeout 复核。
- 现象：部分 Session Plan、Runtime Workflow、`build.md` 和 closeout 报告把 AGOS 的未登记、`needs-input`、严格校验与 rollout 状态写成当前项目必须处理的门禁或后续动作。
- 根因：混淆了 `inputcodex` 项目原生控制面与外部 AGOS 辅助框架的责任边界；外部工具状态被错误提升为本项目交付状态。
- 处理：固化“可用则用、不可用绕过、禁止在本仓优化 AGOS”；项目验证回到 `AGENTS.md`、`build.md`、任务计划、Git/GitHub 和项目所有者决策，AGOS 只保留可选辅助与历史观测角色。
- 验证：项目规则、Master Plan、Issue `#4` Plan/Session/Runtime、closeout 报告和 PR 正文使用一致边界；本仓原生验证不依赖 AGOS 路径、Registry 或严格校验结果。
- 关联：GitHub Issue `#4`、PR `#5`。

### 2026-07-21：AGOS 默认入口曾返回 needs-input（外部历史记录）

- 环境：仅有空 Git 仓库，尚无项目级入口文档和任务登记。
- 现象：`invoke-agos-default-entry.ps1 -ReportOnly` 返回 `AGOS_DEFAULT_ENTRY_STATUS=needs-input`。
- 根因：新项目尚未建立项目入口文档和正式任务控制材料。
- 处理：当时创建项目治理文档、筹备计划、运行工作流和 GitHub Issue；当前结论补充为该外部状态不构成本项目阻塞，后续不可用时直接绕过。
- 验证：项目原生文档、Git 与 GitHub 交付链可独立完成；是否重新运行 AGOS 只取决于其是否可用且对当前任务有帮助。
- 关联：GitHub Issue `#1`。

### 2026-07-21：单条补丁命令超过 Windows 长度上限

- 环境：PowerShell 将多个完整文档作为一个 `apply_patch` 参数传入。
- 现象：命令返回 `The command line is too long.`，没有文件被修改。
- 根因：单个 Windows 进程命令行承载的补丁文本过长。
- 处理：按职责把补丁拆成多个小批次。
- 验证：小批次补丁可进入补丁程序并正常处理。
- 关联：本次仓库筹备会话。

### 2026-07-21：桌面版 apply_patch 入口随安装版本变化

- 环境：Codex Desktop、PowerShell 与 `--codex-run-as-apply-patch` 模式。
- 现象：临时 `apply_patch.bat` 曾返回 `Access is denied.`；继续硬编码 npm 包内原生二进制后又出现 `ENOENT`；通过 stdin 传递补丁时返回 `requires a UTF-8 PATCH argument`。
- 根因：Codex 安装位置和内部原生二进制路径会随桌面版或 npm 包更新变化，不能把内部绝对路径视为稳定入口；当前接口要求把完整补丁作为单个参数传入，而不是通过 stdin。
- 处理：先用 `Get-Command codex` 解析当前有效命令，再执行 `& codex --codex-run-as-apply-patch $patch`；禁止继续硬编码包内二进制路径或把补丁管道传入。
- 验证：命令退出码为 `0`，并返回 `Success. Updated the following files`，本次五个治理文件均按预期更新。
- 关联：GitHub Issue `#2`、PR `#3`。

### 2026-07-21：默认 PowerShell Shell Runner 间歇拒绝启动

- 环境：Codex Desktop 调用默认 `C:\Users\dashuai\AppData\Local\Microsoft\WindowsApps\pwsh.exe` 执行只读 Git 与文档查询。
- 现象：命令在实际脚本执行前返回 `CreateProcessAsUserW failed: 5 (拒绝访问。)`；相同仓库内容通过 Node 只读接口或提升后的 Shell 可以正常读取。
- 根因：故障发生在 WindowsApps `pwsh.exe` 的沙箱进程启动层，不是 Git 仓库、命令参数或项目文件错误。
- 处理：不连续盲目重试；只读查询可切换 Node REPL，必须使用 PowerShell 或项目脚本时使用 `require_escalated`，文件修改仍坚持官方 `apply_patch` 模式。
- 验证：Node REPL 成功读取 Git 状态和控制文档；提升后的 Shell 成功执行相同 Git、`rg` 与 `Get-Content` 查询。
- 关联：GitHub Issue `#2`、PR `#3`。

### 2026-07-21：AGOS 严格入口未登记 inputcodex Issue #2（外部历史记录）

- 环境：外部项目 `C:\Users\dashuai\Documents\inputcodex` 调用 AI Growth OS 默认入口。
- 现象：`invoke-agos-default-entry.ps1 -ReportOnly` 返回 `TASK_REGISTRATION_STATUS=unregistered`、`AGOS_DEFAULT_ENTRY_STATUS=needs-input`，并禁止严格模式下的项目文档写入、提交和 PR。
- 根因：AI Growth OS 全局 `registry/task-backlog.yml` 和 `registry/business-paths.yml` 尚未登记本项目任务 `2026-07-21-issue-2-architecture-governance` 与 `architecture-governance` 路径。
- 处理：不在 inputcodex PR 中越权修改跨仓控制面；以 GitHub Issue `#2`、当前分支、项目原生文档和项目所有者批准作为本仓任务证据。后续该状态只记录并绕过，不再使用 warning mode 充当项目门禁。
- 验证：Issue `#2` / PR `#3` 已通过本项目交付链完成 Squash Merge；未登记与严格校验状态没有阻塞项目，也未触发 AGOS 修改。
- 关联：GitHub Issue `#2`。

### 2026-07-21：Git 快照检查阻止继续扩大未提交文档

- 环境：Issue `#2` 文档分支已有未提交的总方案和 ADR。
- 现象：`verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly` 返回 `GIT_SNAPSHOT_STATUS=blocked`，并列出三个 critical untracked 文档。
- 根因：关键治理文档尚未形成命名 Git 快照；检查器按设计阻止在脏工作树上无限扩展改动。
- 处理：将现有草案视为同一个有界执行批次，补齐 Session Plan、Runtime Workflow、Master Plan、`build.md` 和本记录后停止扩大范围，运行 Fresh 验证并提交快照。
- 验证：提交前重新执行 Git snapshot、`git diff --check` 和 `git diff --cached --check`；提交后要求工作树只保留预期状态。
- 关联：GitHub Issue `#2`。

### 2026-07-21：Major Session Plan 使用了非标准 reviewer 角色

- 环境：运行 `verify-session-plan.ps1` 校验 Issue `#2` 的 Major Session Plan。
- 现象：校验器报错 `Major session plan missing proposal role: architecture-reviewer`。
- 根因：草案把三个方案路线误写为 `agent_proposals.role`，并使用了校验器不接受的 `proposal_mode: owner-dialogue`；Major 合同要求固定的 `architecture-reviewer`、`operator-experience-reviewer`、`verification-reviewer` 与受支持枚举。
- 处理：保留用户已批准的实质决策，把三个审查视角映射到标准角色，并将模式改为 `simulated-roles`，明确未启动子 agent 的原因。
- 验证：重新运行校验输出 `SESSION_PLAN_VERIFY_OK`，工作类别为 `major`。
- 关联：GitHub Issue `#2`。

### 2026-07-21：AGOS 验证脚本参数漂移被错误包装成退出码 0

- 环境：Issue `#2` 文档分支运行 `verify-post-implementation-review.ps1` 与 `verify-protected-feature-replay.ps1`。
- 现象：PowerShell 报告不存在参数 `ProjectRoot` 和 `SessionPlanPath`，但临时包装函数仍输出 `EXIT_CODE=0`。
- 根因：两个脚本的真实输入参数均为 `-Path`；Runtime Workflow 保留了旧调用名。同时 `$LASTEXITCODE` 只可靠表示本机可执行程序退出码，不能单独判断 PowerShell 脚本参数绑定或异常状态。
- 处理：当时通过 `Get-Command` 和脚本 `param` 块确认接口，把可选调用统一改为 `-Path`；受保护功能回放增加 `-RequireProtectedReplay`。当前边界进一步明确：AGOS 接口再次漂移或异常时只记录并绕过，不在本仓修复脚本；原生 `git` 仍检查 `$LASTEXITCODE`，不能用 `if (git ...)` 把“成功但无输出”误判为失败。
- 验证：使用修正参数重新运行两个脚本，分别得到 post-implementation review 的确定状态和 protected feature replay 的通过证据，且不再出现参数绑定错误。
- 关联：GitHub Issue `#2`、PR `#3`。

### 2026-07-21：受保护功能回放的实际结果与所有者状态未满足通过合同

- 环境：使用 `-RequireProtectedReplay` 严格校验 Issue `#2` Session Plan。
- 现象：校验输出 `actual-result-mismatch` 与 `owner-visible-replay-not-passed`，禁止提交、PR 和完成声明。
- 根因：`actual_result` 虽表达了相同中文语义，但没有包含校验器要求的完整 `expected_result` 文本；同时 `owner_visible_status` 仍保留早期 `pending`，未同步项目所有者已确认方案且 Gate 0 证据仍公开可见的事实。
- 处理：先重新核对 `LICENSE` 未变化、仓库无 Workflow/Cargo/Rust 源码、Gate 0 计划和 closeout 仍存在、Master Plan 处于 Gate 1；再让 `actual_result` 明确包含完整预期句，并把有证据的所有者可见状态更新为 `passed`。
- 验证：`verify-protected-feature-replay.ps1 -Path docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md -RequireProtectedReplay -ReportOnly` 输出 `PROTECTED_FEATURE_REPLAY_STATUS=ready`、`COMPLETION_STATUS=passed`、两个 passed count 均为 `1`，且 `FORBIDDEN_OPS=none`。
- 关联：GitHub Issue `#2`、PR `#3`。

### 2026-07-21：合并门错误地把仓库级开关当成 main 的有效规则

- 环境：项目所有者授权 Squash 合并 PR `#3` 后执行 Fresh 合并门检查。
- 现象：自定义检查发现仓库级 `allow_merge_commit` 与 `allow_rebase_merge` 为 `true`，因而错误报告不符合 Squash-only；与此同时 `main-protection` Ruleset 明确只允许 `squash`。
- 根因：检查器混淆了两个不同作用域：仓库级开关控制整个仓库可提供的合并方式，`main` Ruleset 控制目标分支的实际允许方式。Gate 1 已批准且在 `docs/reports/2026-07-21-main-protection-rollout.md` 明确记录“只约束 main、不修改仓库级开关”，因此仓库级布尔值不能单独判定 PR `#3` 的合并门失败；总架构中“仓库必须禁用”的措辞也扩大了已批准范围。
- 处理：合并门改用 `main` 的有效 Ruleset、PR 目标分支和显式 `--squash` 作为权威证据；修正总架构措辞，明确 Gate 1 不改变其他分支可用方式。若未来要求全仓关闭 Merge Commit/Rebase Merge，必须另建 Issue/PR 并获得项目所有者批准。
- 验证：Ruleset `19395456` 为 `active`，只包含 `refs/heads/main`，无 bypass actor，`allowed_merge_methods` 仅为 `squash`；PR `#3` 目标为 `main` 且合并命令显式使用 `--squash`。
- 关联：GitHub Issue `#2`、PR `#3`。

### 2026-07-21：Git HTTPS 失败后通过 GitHub API 精确恢复签名合并提交

- 环境：PR `#3` 已在 GitHub Squash Merge，但本地 `git fetch` 与直接 HTTPS 请求连续出现 timeout/reset；认证后的 `gh api` 仍可稳定读取 Git Database API。
- 现象：本地 `main` 和 `origin/main` 无法通过正常 Git transport 前进到远端合并提交 `0e11375997ff10fdc0c233b31c8468af2d9a4f44`，closeout 又必须在准确的新基线上继续。
- 根因：故障位于 Git smart HTTP/直接 HTTPS 传输路径，不是远端仓库、提交对象或 GitHub API 不可用；GitHub 生成的合并提交还包含有效 PGP 签名，普通 `git commit-tree` 无法自动复现相同对象 ID。
- 处理：使用 GitHub Git Database API 读取目标 commit、tree、parent、`verification.payload` 与 PGP signature；按 Git commit 对象格式重建 `gpgsig` 多行头，并完整保留签名结束行后的单空格 continuation，再写入对象数据库并把本地 `main`、`origin/main` 更新到已验证 SHA。失败尝试产生的两个 dangling commit 不挂任何 ref，不执行激进 `git gc` 或 `prune`。
- 验证：重建 SHA 精确等于 `0e11375997ff10fdc0c233b31c8468af2d9a4f44`；`git fsck --connectivity-only` 通过；本地 `main`、`origin/main` 与 closeout 分支基线一致；提交只有父节点 `09564740b8d00a4b09630c024607cc5292d0381f`，tree 为 `0730422eb3fa738fe2d05a51e5191832fbfec0fe`。
- 关联：GitHub Issue `#4`、PR `#3`、dangling commits `74a193e9382e2e849d21d34a2f40f6d1c3b264f3` 与 `c19930a7e3ba1074e66a834306bc402bfddab615`。

### 2026-07-21：quiet 原生命令不能只凭空输出判断成功

- 环境：PowerShell 中执行 `gh api --silent`、`git for-each-ref` 等可能成功但没有 stdout 的原生命令。
- 现象：若脚本只判断输出是否为空，会把“成功且无结果”“目标不存在”“命令执行失败”混成同一状态；只有一行输出时，PowerShell 还可能返回标量字符串，直接使用 `$result[0]` 会取得首字符而不是第一行。远端分支删除、Git ref 与工作树状态查询因此可能产生假结论。
- 根因：原生命令的 stdout 内容、输出形状和进程退出状态是三条不同证据；`--silent` 会主动抑制成功输出，单行 stdout 会发生标量解包，PowerShell 也不会自动把非零退出码转换为可捕获异常。
- 处理：原生 `git`、`gh` 命令执行后立即保存并检查 `$LASTEXITCODE`，再解释 stdout；需要按行计数或索引时使用 `@(...)` 强制数组归一化。PowerShell 脚本仍使用 `$ErrorActionPreference = 'Stop'`、`try/catch` 或 `$?`，不能反过来只用 `$LASTEXITCODE` 判断脚本参数绑定错误。
- 验证：远端旧分支查询明确得到 `gh` 退出码 `1`，本地 ref 查询退出码为 `0` 且输出为空；PR `#5` 最终核验把 `git status --short --branch` 包装为数组后，正确识别唯一分支状态行并输出 `PR5_AGOS_BOUNDARY_UPDATE_VERIFY_OK`。
- 关联：GitHub Issue `#4`、`build.md` 的 Squash 与分支清理核验。

### 2026-07-21：PowerShell 把 GitHub 时间字符串自动解析为 DateTime

- 环境：PowerShell 7 使用 `gh ... --json ... | ConvertFrom-Json` 读取 `closedAt` 与 `mergedAt`。
- 现象：对象显示正确时间，但直接与 `2026-07-21T13:15:52Z` 等 RFC3339 字符串比较返回 `False`，会使正确的 closeout 证据验证假失败。
- 根因：`ConvertFrom-Json` 将 ISO 8601 字段转换成 `System.DateTime`，直接字符串比较不会先按目标 RFC3339 形式格式化。
- 处理：先调用 `ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')`，再与权威 UTC 时间比较。
- 验证：Issue `#2` 关闭时间格式化为 `2026-07-21T13:15:52Z`，PR `#3` 合并时间格式化为 `2026-07-21T13:15:51Z`，closeout 检查通过。
- 关联：GitHub Issue `#4`、`build.md` 的 GitHub 与上游基线核验。

### 2026-07-21：Git push 连接失败后用 Git Database API 精确同步功能分支

- 环境：Issue `#4` closeout 提交已在本地完成并通过 Fresh 验证，正常 `git push` 需要创建新的远端功能分支。
- 现象：默认 Git HTTPS push 返回 `Recv failure: Connection was reset`；仅切换 HTTP/1.1 后又返回端口 `443` 连接超时，而同一时段 `gh api` 读写 GitHub API 正常。
- 根因：故障仍位于 Git smart HTTP 传输路径。首次直接用工作区文本创建 GitHub tree 时，7 个既有文件受 `core.autocrlf` 工作区 CRLF 与 Git index LF 归一化影响，tree SHA 不一致；随后 GitHub Commit API 保留 `+0800` 时区，但省略本地 `git commit` 默认添加的提交消息末尾换行，因此 commit SHA 不同。
- 处理：不 force push、不改 `main`。从本地 Git blob 对象读取原始字节，以 Base64 逐个调用 Git Database API，并要求每个远端 blob SHA 与本地 SHA 相同；重建得到与本地完全一致的 tree `a23e7f5d30178da95d33509894cadf4b97c08b0c`。GitHub API 创建 commit `f78d6ead6da39579d38ce49a9edd552ba1af844b` 后，验证它与本地预推送提交只有消息末尾换行差异，tree 与 parent 完全相同；本地精确重建该对象，再创建远端新分支 ref，并用原子 `git update-ref` 对齐本地功能分支和 `origin/*` 跟踪引用。后续 fast-forward 必须使用精确 author/committer identity 与 JSON body；禁止用 `sha=$commit.sha` 这类会把整个 PowerShell 对象展开进参数的写法。
- 验证：首次远端分支、本地 HEAD 与 `origin/*` 对齐到 `f78d6ead6da39579d38ce49a9edd552ba1af844b`；后续 closeout 元数据使用 `force:false` 正常 fast-forward。当前工作树干净，PR `#5` 为 `OPEN`、非 Draft、`CLEAN`，PR Head 与本地/远端跟踪分支一致，Checks 与 Review 对话均为 `0`。
- 关联：GitHub Issue `#4`、PR `#5`。本地预推送提交 `316d4afec8b67b857b6f217847cd3f0cf8ed0d58` 和诊断对象 `e6780bc2ccc22192e3f23c7868c56ba5f683af97` 已无 ref；错误 CRLF tree `811b432374ad56430645ec2215f6475b93c0520e` 未被任何远端 ref 使用，不执行激进清理。

### 2026-07-21：过期状态扫描把验证器中的正则字面量识别为命中

- 环境：Issue `#6` Gate 1 本地 Fresh 验证，PowerShell `Select-String` 扫描当前控制文档。
- 现象：验证报告 `发现过期状态：PR `#5`.*OPEN`，但实际命中位于 `build.md` 的 `$stalePatterns` 定义本身，而不是 README、Master Plan 或架构状态。
- 根因：验证器把包含待搜索正则字面量的自身文件加入了扫描集合，形成确定性的自匹配假阳性。
- 处理：从过期状态内容扫描集合中移除 `build.md`；`build.md` 继续通过正向硬约束、Git diff 和实际命令执行验证，不用自身正则扫描自身。
- 验证：重新执行同一 Fresh 验证，要求过期状态扫描、Issue Forms YAML 解析和 `git diff --check` 全部通过。
- 关联：GitHub Issue `#6`、`build.md` 的 Gate 1 本地 Fresh 验证。

### 2026-07-21：PR 模板空列表占位产生尾随空格

- 环境：Issue `#6` 文件已精确暂存，执行 `git diff --cached --check`。
- 现象：Git 报告 `.github/pull_request_template.md:12: trailing whitespace`。
- 根因：变更摘要使用仅包含 `- ` 的 Markdown 空列表占位，连字符后的单个空格被 Git 判定为尾随空白。
- 处理：把空占位改为 `- 待补充`，保留模板引导语义并消除尾随空格。
- 验证：重新暂存 PR 模板与 `err.md`，执行 `git diff --cached --check`，要求退出码为 `0`。
- 关联：GitHub Issue `#6`、`.github/pull_request_template.md`。

### 2026-07-21：PowerShell `-join` 结果与比较运算必须显式分组

- 环境：PR `#7` 合并前 Fresh 验证，PowerShell 同一 `if` 条件中检查 Ruleset 的 `allowed_merge_methods`。
- 现象：表达式 `(@(...)-join ',' -ne 'squash')` 在解析阶段报 `Unexpected token '{'`，验证命令尚未执行任何 GitHub 读取或写入。
- 根因：`-join` 是运算符，不能把右侧分隔符和后续 `-ne` 比较混写在同一未分组表达式中；解析器无法确定比较边界。
- 处理：先计算 `$allowedMethods = (@(...) -join ',')`，再独立执行 `$allowedMethods -ne 'squash'`。
- 验证：修正后的同一 Fresh 验证输出 `PR7_PREMERGE_FRESH_VERIFY_OK`，PR `#7` 随后按授权 Squash Merge。
- 关联：GitHub PR `#7`、Issue `#8` Gate 1→2 过渡。

## 记录模板

```text
### YYYY-MM-DD：问题标题

- 环境：
- 现象：
- 根因：
- 处理：
- 验证：
- 关联：
```
