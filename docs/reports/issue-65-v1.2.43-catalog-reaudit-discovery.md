# Issue #65：`v1.2.43` 功能目录重新审计 Discovery 报告

## 结论

`v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c` 相对 `v1.2.42` 有 `33` 个变化文件，但公开入口仍为 `133`，功能、合同与 fixture 仍为 `36/36/11`。本轮不新增产品能力，只推进 Release 锚点、复审两个受支持行为并补充两个既有例外的证据。

## 来源基线

- 快照合并提交：`15e91708b41548f523e26ede4c7ca4de41badf77`
- Release tag：`v1.2.43`
- Release commit：`5036ff056b5c629f19356396b17d6eeb70da664c`
- Release tree：`d478a9fcda7f22a7c8167cb567777ad9148cf328`
- 归档 SHA-256：`5612fbdc60244b9080823b596745c6e88f8cc5fa1996015b143b44ba6e51dd7f`
- Manifest SHA-256：`5af73a428ab5fb14d102ea7f788e6878fc29e203ab5f551770a5871161edb239`
- Discovery 评论：`https://github.com/nonononull/inputcodex/issues/65#issuecomment-5084841145`
- 实施批准评论：`https://github.com/nonononull/inputcodex/issues/65#issuecomment-5087166207`

## 来源入口重枚举

| 类型 | 数量 | 结论 |
| --- | ---: | --- |
| Tauri command | 84 | 无增删；仍只作为审计入口，不进入最终运行面。 |
| Core public module | 45 | 无增删。 |
| Data public module | 4 | 无增删。 |
| 合计 | 133 | `source-index` 不新增条目。 |

## 受影响语义

### Watcher

- macOS 新增 launcher 与 Codex 主进程终止和等待语义。
- 重启可按 `remote-debugging-port=<port>` 精确筛选 Codex 主进程，排除 helper 进程。
- 等待超时会写入可诊断证据，不应静默宣称退出成功。
- 目录结论：更新 `feature.foundation-platform.watcher` 和合同；Windows/macOS 共享“精确目标、等待退出、失败可诊断”的语义，不照搬上游启动器实现。

### Provider Sync

- 当 `local_thread_catalog` 缺少行时，可从 `threads` 表复制标题、时间、工作区、来源、Provider、分支和 thread source。
- 同步结果新增 `sqlite_catalog_rows_inserted`，Tauri 输出为 `sqliteCatalogRowsInserted`。
- 同步状态维护 watermark、initial build、observation sequence 与最后完整复核时间。
- 目录结论：更新 feature、合同和一组合成 fixture；数据库写入必须事务化，计数与状态可观测。

## 继续保持例外

### User Scripts

上游可将 renderer 记录的 `status` 与 `error` 合并到脚本库存，但真实执行状态仍来自注入页面。该能力继续保持 `exception-pending`，只补充来源证据和“不得伪造运行成功”的合同语义。

### 广告与 sponsor

上游新增 sponsor、到期时间和本地素材。项目无广告硬规则不变，功能继续保持 `exception-pending`，不得加载、下载、展示或执行这些内容。

## 不进入产品事实层

- Tauri/React 管理界面与 CSS。
- renderer/inject、Dream Skin runtime 和宠物鼠标注入。
- 远程推荐列表与 sponsor 图片。
- 上游 `docs/superpowers` 计划文档。
- Cargo/npm/Tauri 版本元数据与讨论群图片。

## 推荐方案与批准

采用方案 A：二十四路径全目录 Release 对齐加受影响行为定向复审。项目所有者已批准 `sha256:82234e7aacce0bd6c57994529ccf74371052ed906dc8371324b90e41f697d7b7`，允许实施到非 Draft PR 与 Review/CI；最终 Squash Merge 保留单独授权门。

## 实施状态

```yaml
status: local-verification-passed-pr-pending
baseline_test: 12/12 passed
red_evidence: 9/12 passed; 3 expected failures from old release metadata and missing new semantics
green_evidence: 12/12 passed; CI contract 35/35; repository policy ok with 0 violations; git diff --check passed at Windows local time 2026-07-27 12:46:06 +08:00
review_ci: pending
final_merge: not-authorized
```
