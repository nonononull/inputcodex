# inputcodex-parity 排错记录

## 当前限制

- 当前提供稳定错误签名、功能目录/行为合同/fixture manifest 解析、纯内存验证与 fixture 载荷安全检查。
- 当前不读取 `upstream/` 或仓库 `parity/` 数据，不实现业务映射，也不依赖 Iced、platform 或 presentation。
- `catalog_repository` 在 source-index、五分域目录、合同和 fixture 数据建立前保持未运行状态。

## 排错顺序

出现一致性差异时先建立或引用一致性例外 Issue；禁止在本包直接修正产品行为或绕过项目所有者决策。

## 2026-07-22：表驱动必填字段测试的替换片段零命中

- 环境：Issue `#26` 最小 Rust schema GREEN 阶段，测试通过 `str::replace` 删除必填 YAML 片段并断言解析失败。
- 现象：合同必填字段测试报告“缺少 id 时必须拒绝解析”，但生产结构已经要求 `id`；检查发现测试使用四空格片段，而真实首个 sequence 项是两空格加 `- id`，替换没有发生。
- 根因：测试未先确认替换片段存在，零命中的 `replace` 返回原文，导致合法 YAML 被误判为生产解析缺陷。
- 处理：修正 `id` 片段缩进，并在 catalog、contract、fixture 三组表驱动测试中先断言原文包含目标片段，再执行删除。
- 验证：四个定向测试目标共 `26` 个测试全部通过；错误不涉及生产 schema、依赖或 YAML 解析器。
