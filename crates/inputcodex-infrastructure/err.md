# inputcodex-infrastructure 排错记录

## 当前限制

- `UnconfiguredLoadPort` 返回稳定的 `Unavailable / LOAD_SOURCE_UNCONFIGURED`，这是有意的失败语义，不得改为空结果或成功占位。
- 任何真实存储、网络或文件适配器必须由后续独立功能 Issue 批准。

## 排错顺序

先运行 `cargo test -p inputcodex-infrastructure`，再检查 application 端口是否变化；禁止直接依赖 domain、presentation 或 platform。
