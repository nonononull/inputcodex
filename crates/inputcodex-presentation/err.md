# inputcodex-presentation 排错记录

## 当前边界

- 当前只提供 application 状态到 Iced 生命周期的最小集成，不建立颜色、排版、组件或交互设计系统。
- UI、视觉和交互方案仍由 Gemini 实现或审阅。
- `iced-runtime` 未启用时只能验证纯状态映射；这不等于桌面运行时已经通过。

## 排错顺序

先运行无 Iced 的定向测试；Iced API、渲染后端或系统库问题交给三平台 Actions 取证，禁止把 Iced 类型下沉到 application。
