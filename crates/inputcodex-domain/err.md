# inputcodex-domain 排错记录

## 当前限制

- 当前只包含稳定诊断码值对象，不包含上游业务实体。
- 任何 I/O、平台或 Iced 依赖都属于架构越层，应由仓库政策脚本直接拒绝。

## 排错顺序

1. 先查仓库根 `err.md` 是否已有 Rust、Cargo 或 Windows 执行环境结论。
2. 运行 `cargo test -p inputcodex-domain` 取得最小失败证据。
3. 只修复 domain 根因，不通过增加适配层依赖绕过。
