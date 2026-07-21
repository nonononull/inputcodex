---
status: accepted
---

# 使用纯 Rust 与 Iced 构建桌面应用

`inputcodex` 的 Windows 与 macOS 桌面产品禁止使用 TypeScript、JavaScript 业务代码和 WebView，采用 Rust 与 Iced 实现原生 UI。Iced 只能存在于展示层，领域规则、用例、存储、网络和平台集成不得暴露或依赖 Iced 类型，以避免再次形成 UI 框架与业务逻辑耦合的巨型架构。

## 备选方案

- 保留上游 Tauri + React：功能接近，但继续承受 WebView、双语言和状态耦合成本。
- 继承半成品 egui：已有迁移证据，但存在巨型入口、版本落后和再次迁移成本。
- Slint：原生跨平台，但引入额外声明式语言，与纯 Rust 源码约束不完全一致。

## 影响

- Windows 与 macOS 从首版起使用同一 Rust 功能核心和 UI 状态模型。
- 所有耗时 I/O 必须离开 UI 更新路径，并支持取消、超时和过期结果丢弃。
- Iced 版本必须锁定；替换 UI 框架不得要求重写领域与应用层。
