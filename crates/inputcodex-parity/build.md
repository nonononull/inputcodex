# inputcodex-parity 构建说明

## 包定位

- 包名：`inputcodex-parity`。
- 依赖：`inputcodex-application`、`inputcodex-domain`、固定版本 `serde 1.0.229` 与 `yaml_serde 0.10.4`。
- 默认不链接进桌面发布二进制，只保存稳定语义和后续一致性证据工具。

## 本地轻量验证

```powershell
cargo fmt --package inputcodex-parity -- --check
cargo check --locked --offline --ignore-rust-version -p inputcodex-parity --lib
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-parity --lib -- -D warnings
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity
```

## 阶段边界

- 本地已有工具链为 Rust `1.93.1` 时，只能配合 `--ignore-rust-version` 生成定向证据；精确 Rust `1.97.1` 与 Workspace 全量验证由 GitHub Actions 执行。
- `catalog_repository` 同时验证 source-index、五域 feature 目录、36 份行为合同、11 个 fixture manifest 及其完整引用、安全路径和文本控制字节；每次改动 `parity/` 后必须运行完整包测试。
- 禁止在本包引入 Iced、platform、presentation、产品调用或真实私人数据。
