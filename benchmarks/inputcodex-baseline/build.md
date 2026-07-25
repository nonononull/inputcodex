# inputcodex-baseline 构建与验证说明

## 定位

本目录是 Issue `#32` 的独立 Rust 测量 Workspace，不属于仓库根七成员 Workspace。它只依赖现有应用层与 Parity crate，不得反向修改根 `Cargo.toml` 或根 `Cargo.lock`。

## 本地轻量验证

从仓库根目录执行：

```powershell
cargo fmt --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --check
cargo test --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --locked --offline
```

本地只验证合同和场景正确性，不运行 Release 性能采集，不启动桌面窗口，不执行 Windows/macOS 完整基线。

## CLI 合同

CLI 输出一行稳定 CSV：

```text
scenario,iterations,total_nanoseconds,nanoseconds_per_operation,checksum
```

调用格式：

```powershell
cargo run --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --locked --offline -- `
  application-load-complete `
  . `
  1
```

真实样本只能由 `.github/workflows/performance-baseline.yml` 在 GitHub-hosted Windows/macOS runner 上采集。

## 锁文件

依赖变化必须先获得 Issue 范围批准，再从仓库根执行：

```powershell
cargo generate-lockfile --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --offline
```

生成后必须确认根 `Cargo.toml` 与根 `Cargo.lock` 零差异。
