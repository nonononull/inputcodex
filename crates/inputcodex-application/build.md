# inputcodex-application 构建说明

## 包定位

- 包名：`inputcodex-application`。
- 依赖：仅 `inputcodex-domain`。
- 定义请求标识、加载状态、取消/过期结果语义、稳定错误和端口。

## 定向验证

```powershell
cargo check -p inputcodex-application
cargo test -p inputcodex-application
```

禁止在该包中加入文件、SQLite、HTTP、进程、系统 API 或 Iced。
