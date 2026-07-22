# inputcodex-parity 构建说明

## 包定位

- 包名：`inputcodex-parity`。
- 依赖：仅 `inputcodex-application` 与 `inputcodex-domain`。
- 默认不链接进桌面发布二进制，只保存稳定语义和后续一致性证据工具。

## 定向验证

```powershell
cargo check -p inputcodex-parity
cargo test -p inputcodex-parity
```
