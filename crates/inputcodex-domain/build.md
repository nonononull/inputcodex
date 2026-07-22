# inputcodex-domain 构建说明

## 包定位

- 包名：`inputcodex-domain`。
- 只允许标准库和纯领域类型；不得依赖 I/O、异步运行时、平台或 UI。

## 本地轻量验证

从仓库根执行：

```powershell
cargo check -p inputcodex-domain
cargo test -p inputcodex-domain
```

仓库固定 Rust `1.97.1`。本地缺少该工具链时不要改成浮动 `stable`；全量与精确工具链验证交给 GitHub Actions。
