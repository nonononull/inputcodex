# inputcodex-platform 构建说明

## 包定位

- 包名：`inputcodex-platform`。
- 依赖：仅 `inputcodex-application`。
- Windows 与 macOS 返回统一 `PlatformKind`；其他目标明确返回 unsupported。

## 定向验证

```powershell
cargo check -p inputcodex-platform
cargo test -p inputcodex-platform
```

Windows/macOS 双平台完整验证由 GitHub Actions 标准 runner 执行，本地结果不能替代另一平台证据。
