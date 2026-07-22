# Rust CI 无缓存冷构建基线

schema_version: inputcodex.rust-ci-cold-baseline.v1
report_status: minimum-baseline-complete
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/19
pr_ref: https://github.com/nonononull/inputcodex/pull/21
workflow_ref: .github/workflows/ci.yml
required_success_samples_per_platform: 3
accepted_success_samples_linux: 3
accepted_success_samples_windows: 3
accepted_success_samples_macos: 3
cache_policy: disabled-and-out-of-scope-for-issue-19
dependency_package_count: 336
workspace_package_count: 7
external_package_count: 329
failure_semantics_status: verified-5-of-5
latest_full_green_ci_ref: https://github.com/nonononull/inputcodex/actions/runs/29917649550

## 一、测量边界

- 当前只测公开仓库标准 GitHub-hosted runners；禁止 Larger Runner、self-hosted runner 和本地全 Workspace/Iced 重型编译。
- `.github/workflows/ci.yml` 没有 Cargo Cache；每个被接受样本都来自新的普通提交运行，不使用 rerun 旧失败。
- 排队时间定义为平台 Job `startedAt - run.createdAt`；Job 执行时间定义为 `completedAt - startedAt`。GitHub API 的步骤时间精度为整秒，Workflow 内部 stopwatch metrics 从后续样本开始由普通日志保留三位小数。
- 成功样本必须对应平台 Job 成功且 `required` 成功；失败恢复运行单独记录，不计入三次成功样本。
- Gate 3 只建立 CI 冷构建基线，不据单次最快结果设置最终产品性能预算，也不在本 Issue 引入 Cache。

## 二、已接受成功样本

### 样本 1：运行 29911337652

- 时间：`2026-07-22T10:17:23Z` 至 `2026-07-22T10:21:19Z`。
- Head：`bd4610f6e98dc597bddf02c584d0f0fc616cac7b`。
- 事件：Draft PR `#21` 的 `pull_request` synchronize。
- 结果：classify、governance、linux-quality、windows、macos、required 六个 Job 全部成功；成功 Artifact 数为 `0`。

| 平台 | 排队时间 | Job 执行时间 | 关键冷步骤 | 结果 |
| --- | ---: | ---: | --- | --- |
| Linux | 17 秒 | 112 秒 | Rust 安装 8 秒；Clippy 31 秒；Workspace 全目标测试 50 秒 | 成功 |
| Windows | 16 秒 | 211 秒 | Rust 安装 12 秒；桌面冷构建 121 秒；Workspace 全目标测试 4 秒 | 成功 |
| macOS | 17 秒 | 94 秒 | Rust 安装 9 秒；桌面冷构建 50 秒；Workspace 全目标测试 3 秒 | 成功 |

辅助 Job：classify 执行 `10` 秒，governance 执行 `28` 秒，required 在前置 Job 完成后执行 `4` 秒。required 的 `231` 秒开始延迟来自依赖等待，不解释为 Runner 排队时间。

### 首样本 metrics 缺口

- Linux stopwatch 秒数、Windows/macOS stopwatch 秒数与二进制字节数曾写入各 Runner 的 `metrics.txt`，但摘要步骤只追加到 Step Summary。
- Check Run 与 Actions API 在运行完成后没有返回该 Step Summary，成功策略又不上传 Artifact，因此这些精确值无法复取。
- 本报告不猜测缺失值；样本 1 只采用 GitHub API 的 Job/步骤时间。
- 已用合同 RED→GREEN 修改三个摘要步骤，使后续样本把同一 metrics 同时写入控制台日志与 Step Summary。

### 样本 2：运行 29913139948

- 时间：`2026-07-22T10:45:42Z` 至 `2026-07-22T10:49:45Z`。
- Head：`ca6d0216115059146c8e59d4fae52d3d96d06dc4`。
- 结果：六个 Job 全部成功；成功 Artifact 数为 `0`；三平台 metrics 均可从普通 Job 日志复取。

| 平台 | 排队时间 | Job 执行时间 | Workflow 内部 metrics | 二进制字节数 | 结果 |
| --- | ---: | ---: | ---: | ---: | --- |
| Linux | 17 秒 | 133 秒 | Clippy `38.732` 秒 | 不适用 | 成功 |
| Windows | 18 秒 | 213 秒 | 桌面冷构建 `117.053` 秒 | `26,347,520` | 成功 |
| macOS | 18 秒 | 152 秒 | 桌面冷构建 `78.163` 秒 | `53,510,976` | 成功 |

辅助 Job：classify 执行 `9` 秒，governance 执行 `28` 秒，required 在前置 Job 完成后执行 `8` 秒。required 的 `234` 秒开始延迟仍是依赖等待。

### 样本 3：运行 29914029406

- 时间：`2026-07-22T10:59:53Z` 至 `2026-07-22T11:04:02Z`。
- Head：`d474c47f5ab02ef9ed9804b208a739823819c9e9`。
- 结果：治理探针删除后六个 Job 全部成功；成功 Artifact 数为 `0`。

| 平台 | 排队时间 | Job 执行时间 | Workflow 内部 metrics | 二进制字节数 | 结果 |
| --- | ---: | ---: | ---: | ---: | --- |
| Linux | 22 秒 | 141 秒 | Clippy `39.965` 秒 | 不适用 | 成功 |
| Windows | 23 秒 | 212 秒 | 桌面冷构建 `118.349` 秒 | `26,347,520` | 成功 |
| macOS | 22 秒 | 96 秒 | 桌面冷构建 `50.326` 秒 | `53,510,976` | 成功 |

辅助 Job：classify 执行 `14` 秒，governance 执行 `32` 秒，required 在前置 Job 完成后执行 `8` 秒。required 的 `240` 秒开始延迟仍是依赖等待。

## 三、失败恢复样本

| 运行 | 结果 | 根因 | 处理 | 是否计入成功样本 |
| --- | --- | --- | --- | --- |
| `29910132968` | Workflow 文件级失败，0 Job | job 级 `env` 使用不可用的 `runner.temp` | 后续普通提交改为 `RUNNER_TEMP` + `GITHUB_ENV` | 否 |
| `29910379208` | Workflow 文件级失败，0 Job | 同上；用于取得官方行级注解 | 不 rerun，继续由修复提交触发新运行 | 否 |
| `29910847062` | Linux Clippy 与 required 失败 | Linux 下无条件导入仅供 Windows/macOS 使用的 `PlatformKind` | 后续普通提交按 cfg 收紧导入 | 否 |
| `29913582488` | governance 与 required 失败，三平台成功 | 受控 `.ts` 探针触发 `SCRIPT_LANGUAGE_FORBIDDEN` | `d474c47` 删除探针，运行 `29914029406` 六 Job 全绿 | 否 |
| `29914734781` | linux-quality 与 required 失败，Windows/macOS 成功 | 受控单行格式差异使 `cargo fmt --check` 失败 | `71be06a` 恢复格式，运行 `29915134906` 六 Job 全绿 | 否 |
| `29915537702` | Linux、Windows、macOS 与 required 失败 | 全平台 `compile_error!` 探针触发稳定标记 `GATE3_GENERIC_RUST_COMPILE_FAILURE` | `3ca5866` 删除探针，运行 `29915879951` 六 Job 全绿 | 否 |
| `29916309635` | Windows 与 required 失败，Linux/macOS 成功 | Windows cfg `compile_error!` 探针触发 `GATE3_WINDOWS_CONDITIONAL_COMPILE_FAILURE` | `436f727` 删除探针，运行 `29916670916` 六 Job 全绿 | 否 |
| `29917061781` | macOS 与 required 失败，Linux/Windows 成功 | macOS cfg `compile_error!` 探针触发 `GATE3_MACOS_CONDITIONAL_COMPILE_FAILURE` | `41c0cc2` 删除探针，运行 `29917649550` 六 Job 全绿 | 否 |

## 四、样本槽位

| 平台 | 样本 1 | 样本 2 | 样本 3 | 当前状态 |
| --- | --- | --- | --- | --- |
| Linux | `29911337652` | `29913139948` | `29914029406` | `3/3` |
| Windows | `29911337652` | `29913139948` | `29914029406` | `3/3` |
| macOS | `29911337652` | `29913139948` | `29914029406` | `3/3` |

后续附加样本仍必须补：run/Head、排队时间、Job 时间、内部 metrics、Windows/macOS 二进制字节数、Artifact 数量、结论和异常说明。

## 五、当前超时依据

| Job | Workflow 超时 | 三样本中位数 | 三样本范围 | 当前结论 |
| --- | ---: | ---: | ---: | --- |
| linux-quality | 30 分钟 | 133 秒 | 112–141 秒 | 保持；最低基线与失败语义均已完成 |
| windows | 45 分钟 | 212 秒 | 211–213 秒 | 保持；Windows 为稳定最慢平台 |
| macos | 45 分钟 | 96 秒 | 94–152 秒 | 保持；样本 2 存在明显但未超时的波动 |

三平台各三次成功样本已满足 Gate 3 最低基线：排队时间中位数分别为 Linux `17` 秒、Windows `18` 秒、macOS `18` 秒；Windows/macOS 两次精确二进制字节数分别稳定为 `26,347,520` 与 `53,510,976`。Cache、P95、七天观测和至少十次重型运行仍属于后续独立调优 Issue。
