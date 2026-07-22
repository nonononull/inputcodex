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
