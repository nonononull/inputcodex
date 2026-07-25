# inputcodex-presentation 构建说明

## 包定位

- 包名：`inputcodex-presentation`。
- 直接依赖仅为 `inputcodex-application`；Iced `0.14.0` 只通过可选 `iced-runtime` 特性启用。
- 不执行磁盘、网络、SQLite、进程或平台调用。

## 本地轻量验证

```powershell
cargo check -p inputcodex-presentation --no-default-features
cargo test -p inputcodex-presentation --no-default-features
```

## 云端桌面运行时验证

```powershell
cargo check -p inputcodex-presentation --features iced-runtime
```

Iced 运行时编译属于 GitHub Actions 全量验证；本地默认不编译重型渲染依赖。

## Issue #32 首次 view 性能探针

性能探针只在环境变量 `INPUTCODEX_PERFORMANCE_PROBE=1` 时启用，并在首次构建 Iced `view` 后向标准输出写入单条稳定标记。它不发送网络请求、不写用户文件、不改变界面或加载语义；本地轻量验证不运行 Iced 窗口，Windows/macOS 实测只由专用 GitHub-hosted workflow 执行。
