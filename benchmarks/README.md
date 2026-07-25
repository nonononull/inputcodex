# inputcodex 性能基线

## 边界

本目录只保存 Issue `#32` 的测量配置、独立 Rust 测量工程与经 GitHub-hosted Windows/macOS runner 采集的原始证据。它不是根 Workspace 成员，不提供性能预算，也不承载优化实现。

上游 `BigPizzaV3/CodexPlusPlus` 与半成品参考只提供静态来源和许可证审计输入；本基线禁止运行、打包或拿它们做数值排名。

## 唯一配置

`config/issue-32-baseline.json` 固定以下协议：

- 首次 view：至少 5 个成功新进程样本，最多 7 次尝试，单次 15 秒超时。
- 空闲资源：首次 view 后等待 30 秒，以 1 Hz 采样 60 秒 Working Set 与 CPU。
- Rust 场景：3 次预热、20 个原始样本；IQR 只标记异常值，不删除样本。
- 成功临时 Artifact 保留 1 天，失败诊断 Artifact 最长保留 7 天。

## 本地允许命令

```powershell
cargo test --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --locked --offline
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Contract
```

禁止在项目所有者本机运行 `Invoke-InputcodexBaseline.ps1`。该脚本会验证 `GITHUB_ACTIONS=true` 与 `RUNNER_ENVIRONMENT=github-hosted`，只允许标准 GitHub-hosted Windows/macOS runner 执行完整 Release 构建、窗口探针和资源采样。

## 证据模式

初次 PR Head 缺少 `results/issue-32/` 三个固定文件时，专用 Workflow 进入 `measure` 模式并分别上传 Windows/macOS 一天临时 Artifact。下载、核验并写入固定结果与组合 manifest 后，后续 Head 进入 `evidence` 模式，只校验哈希、原始样本和 Artifact 引用，成功运行不再上传 Artifact。
