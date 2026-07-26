# Issue #67：解耦固定目录证据与活动快照状态计划

## 元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/67`
- 阻塞 PR：`https://github.com/nonononull/inputcodex/pull/66`
- 失败 Run：`https://github.com/nonononull/inputcodex/actions/runs/30202056781`
- 基线：`a2e5e5a6728200739a4acb85042ba7831ac6295b`
- 分支：`codex/issue-67-release-audit-repository-contract`
- 批准证据：`https://github.com/nonononull/inputcodex/issues/67#issuecomment-5083523560`
- 范围：六路径，`scope_hash=sha256:b6295dabd39f0cba7c4f13bd3d35ff8b0433e1fb95de98e6fc5f2cf0c1eb6b9f`
- 最终 Squash Merge：未授权，继续绑定最终 PR Head 单独审批。

## 根因

1. PR `#66` 合法把活动快照推进到 `v1.2.43`，功能目录仍为 `v1.2.42`，状态为 `stale-re-audit-required` 并指向 Issue `#65`。
2. 提交 `5fd337fb7ceb9b0ef53e2e694cc5ddd81ea0a98c` 新增的 `仓库v1_2_42目录重新审计恢复current`，把“当时为 current”误设为永久不变量：无条件断言 `!summary.requires_reaudit()`，并固定 snapshot/status 必须为 `v1.2.42/current`。
3. 同文件专项测试已覆盖正确状态机：快照与目录相同必须 `current`；快照新于目录必须 `stale-re-audit-required`；非法组合继续返回 `ReleaseMismatch`。
4. 新鲜复现：干净 `main` 在 `cargo clean -p inputcodex-parity` 后通过；未修改 PR `#66` 工作树稳定失败于旧断言，和三平台 CI 同构。

## 采用方案

- 仓库实例测试继续要求 `validate_repository` 成功，并固定 `v1.2.42` 功能目录、合同、source-index 与受影响行为证据。
- 删除实例测试对活动 snapshot/status 的重复固定；current/stale 语义继续由专项状态机测试负责。
- 不新增依赖，不修改生产验证器、PR `#66`、`upstream/`、Workflow、Cargo、预算、Ruleset、Gate 5 或 AGOS。
- 拒绝动态重复状态推导、把固定值更新为 `v1.2.43`、把修复混入 PR `#66`。

## 精确范围

```text
crates/inputcodex-parity/tests/catalog_repository.rs
docs/plans/2026-07-26-issue-67-release-audit-repository-contract.md
docs/plans/sessions/2026-07-26-issue-67-release-audit-repository-contract.md
docs/reports/issue-67-release-audit-repository-contract.md
docs/workflows/2026-07-26-issue-67-release-audit-repository-contract-runtime.md
err.md
```

```text
scope_hash: sha256:b6295dabd39f0cba7c4f13bd3d35ff8b0433e1fb95de98e6fc5f2cf0c1eb6b9f
```

## 执行与验证

1. 写入并验证 Session Plan、Runtime Workflow；AGOS 只运行一次 `ReportOnly`，异常即按项目规则绕过。
2. 保留 PR `#66` RED；将仓库实例测试改为目录证据合同。
3. 在本分支复验 main/current；用临时 detached 工作树叠加 PR `#66` Head 复验 stale。
4. 运行完整目录测试、CI 合同、Release Audit Gate、仓库政策、fmt、scope hash 与 diff check。
5. 六路径精确匹配后提交、普通推送、非 Draft PR、Review/CI；最终 Squash Merge单独请求授权。

成功标准：main/current 与 PR `#66`/stale 均通过，非法状态仍被拒绝；PR `#66` 的 35 路径和 Final Head `4e419586fc89b1bbdd79d20b7179f017070052fb` 不变。
