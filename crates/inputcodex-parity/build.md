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
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_schema --test contract_schema --test fixture_safety --test error_signature
```

## 阶段边界

- 本地已有工具链为 Rust `1.93.1` 时，只能配合 `--ignore-rust-version` 生成定向证据；精确 Rust `1.97.1` 与 Workspace 全量验证由 GitHub Actions 执行。
- `catalog_repository` 必须等 `parity/features/`、`parity/contracts/` 与 `parity/fixtures/` 数据建立后再运行；数据面尚不存在时不得用空目录伪造通过。
- 禁止在本包引入 Iced、platform、presentation、产品调用或真实私人数据。
