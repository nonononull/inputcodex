# inputcodex-desktop 排错记录

## 当前限制

- 当前入口只组装最小服务并调用 presentation；没有安装包、签名、更新、数据源或上游功能。
- 启动失败只输出展示层的稳定本地错误，不联网回传。
- desktop 禁止直接依赖 Iced；若出现 Iced 类型，必须退回 presentation 边界修复。

## 排错顺序

1. 先查根 `err.md` 和对应 crate 的 `err.md`。
2. 使用三平台 CI 区分 Iced、系统库、条件编译和应用状态根因。
3. 禁止通过 WebView、Tauri、JavaScript、跳过 Job 或平台分叉绕过。
