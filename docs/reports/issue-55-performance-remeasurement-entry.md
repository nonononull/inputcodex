# Issue #55：显式性能复测入口报告

## 结论

Issue `#55` 修复了“已入库 Evidence 存在时，手工 `Performance Baseline` 无法再次测量”的合同缺口。手工触发现在必须声明 `mode`：默认 `evidence`，只有显式 `measure` 才使用既有 Windows/macOS 采集路径；自动 PR/push 继续采用原有的文件存在性语义。

本 Issue 是 Issue `#54` 五次独立采样的前置入口修复，不是预算数值、预算 CI、性能优化或 Gate 5 迁移。

## 所有者决策与范围

- 来源 Issue：`https://github.com/nonononull/inputcodex/issues/55`。
- 后续消费者：`https://github.com/nonononull/inputcodex/issues/54`。
- 所有者预授权：Issue `#55` 中记录的 `2026-07-26 06:19:35 +08:00` 本机时间决策“按你推荐来 不用我二次批准 直接安排后续”。
- 写集合：十四条固定路径，`scope_hash=sha256:372c8c3942d492a9372603f5bc6bbae42ae8013c7603a092c294d24be4edb1be`；新增的三条 Issue `#32` Evidence 路径只能使用本 Issue 显式 GitHub-hosted `measure` Run 的成功 Artifact 刷新。

## 根因闭环

1. 旧 Workflow 只有空的 `workflow_dispatch`，contract Job 仅以 Issue `#32` 三份 Evidence 文件的存在性选择模式。
2. `3/3` Evidence 存在时，任何手工 dispatch 都进入 `evidence`；删除证据以触发测量会制造隐式副作用，不能作为可审计复测入口。
3. 新合同为手工事件增加必填 choice `mode`：`measure` 直接选择既有测量路径，`evidence` 只在三份 Evidence 完整时通过，其他自动事件保留原逻辑。

## 本地验证证据

- RED：新增静态合同后，`pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1` 以“性能基线手工触发必须声明默认 evidence 的受约束 mode 输入”稳定失败，证明测试覆盖缺失能力而非环境错误。
- GREEN：最小 Workflow 修复后，同一命令输出 `CI_CONTRACT_GREEN passed=34`。
- 实现哈希：`.github/workflows/performance-baseline.yml` 与 `scripts/ci/Test-CiScripts.ps1` 是 `implementation_sha256` 输入，因此旧 Evidence 对新 Head 正确产生 `HASH_MISMATCH`；未修改 `main` 的同一命令为绿。不能弱化哈希合同，必须用新的成功 Artifact 刷新三份 Evidence。
- 当前代码不修改采集器、验证器、结果 schema、基线配置的业务语义、预算数值、预算 CI、Ruleset、上游、Release、优化或 Gate 5。

## 外部治理探测

AGOS ReportOnly 返回任务登记 `unregistered`、总体 `needs-input`、doctor/所有者直接写入 `blocked`，同时项目 Git 基础、入口文档、source edit admission 和本地知识查询为 `ready`。按项目规则已记录并绕过；没有修改任何 AGOS 文件。

## GitHub 验收证据

- 初次已在分支 `codex/issue-55-performance-remeasurement-entry` 的精确 Head `c15a9bc74c979f3de447e5b28d0ff70ff0d047c3` 触发 `workflow_dispatch mode=measure`。Run `30178268889` 的 `contract`、`windows`、`macos` 与 `required` 四个 Job 均为 `success`；触发事件没有 PR 上下文，组合 manifest 的 `pull_request` 为 `null`，GitHub 之后可能在界面中关联该分支创建的 PR。
- Windows Artifact 为 `8624873440`（`performance-windows-30178268889-1`），下载后归一化 SHA-256 为 `sha256:441d29e67749390ab2e6484810390518111b848fedac8d8a520a98978e95fc46`；macOS Artifact 为 `8624854730`（`performance-macos-30178268889-1`），归一化 SHA-256 为 `sha256:50cbb9c5deaa72b9762fb6cab01a6c934c9fed813d564e54e0a012afaf710dd4`。
- 两份 Artifact 的 `source.commit`、`source.tree`、配置、实现和输入哈希一致，分别为 `c15a9bc74c979f3de447e5b28d0ff70ff0d047c3`、`cc1e153e09cc5b73cbd5457ae502e947c5ac9210`、`sha256:b9ed601016ececc735634aeb143965c78fbfc61819d37c7f9e584bc971642b53`、`sha256:c3293b7382061cfab9734f1b393d9f7572095eb2efca7d52bb0aad478e2710e7` 与 `sha256:c5b507d219ff49975c13805a2a6e036ade6c61a33a184bfffb74219ce01784b5`。
- 已以这两份成功 Artifact 刷新 `benchmarks/results/issue-32/{windows,macos,manifest}.json`；结果文件与 Artifact 逐字归一化一致。临时 Artifact 保留到 GitHub 给出的到期时间，之后由平台自动清理。
- 独立审查补强 `Test-CiScripts.ps1` 的完整决策表断言后，该脚本的内容按既有合同进入 `implementation_sha256`；因此上述初次 Run 仍是其精确 Head 的有效审计事实，却不能继续充当新 Head 的最终 Evidence。没有放宽哈希验证，而是以新 Head 再次测量。
- 最终 `workflow_dispatch mode=measure` Run 为 `30179598622`，精确 Head 为 `427a1c6306f90cb60510200312c66ea25fa74d7a`、tree 为 `772842dc7c8ed1ed3e6dde970836b9054a9a1450`；`contract`、`windows`、`macos` 与 `required` 四个 Job 均为 `success`，事件没有 PR 上下文且组合 manifest 的 `pull_request` 仍为 `null`。
- 最终 Windows Artifact 为 `8625217348`（`performance-windows-30179598622-1`），归一化 SHA-256 为 `sha256:e68d6206b297b107f75fbe6b1dab0a6354e290ee7f0c5781cab8de8b3c68fee4`；macOS Artifact 为 `8625198125`（`performance-macos-30179598622-1`），归一化 SHA-256 为 `sha256:af1bc6a3de8c05920ae01e9459ebf45ff0b7fd8bce1785a0e8baebd9be7e7d92`。两份 Artifact 的配置、实现和输入哈希一致，分别为 `sha256:b9ed601016ececc735634aeb143965c78fbfc61819d37c7f9e584bc971642b53`、`sha256:e4c9265396476c918112f553d846239ed21f84c7432e6a34ddc8f55293d64e48` 与 `sha256:c5b507d219ff49975c13805a2a6e036ade6c61a33a184bfffb74219ce01784b5`。
- 已用最终 Run 的两份成功 Artifact 刷新 `benchmarks/results/issue-32/{windows,macos,manifest}.json`；结果文件与 Artifact 逐字归一化一致。下一步只剩本地 Evidence、CI 合同、仓库策略和范围审计，全部通过后提交、推送并等待 PR CI/Review 收口。
