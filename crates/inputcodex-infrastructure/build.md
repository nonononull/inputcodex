# inputcodex-infrastructure 构建说明

## 包定位

- 包名：`inputcodex-infrastructure`。
- 依赖：仅 `inputcodex-application`。
- Gate 3 只提供未配置数据源适配器，不实现数据库、网络、文件或上游功能。

## 定向验证

```powershell
cargo check -p inputcodex-infrastructure
cargo test -p inputcodex-infrastructure
```
