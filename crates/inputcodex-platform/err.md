# inputcodex-platform 排错记录

## 当前合同

- Windows 返回 `PlatformKind::Windows`。
- macOS 返回 `PlatformKind::Macos`。
- 非发布目标返回 `Unsupported / PLATFORM_UNSUPPORTED`，不得伪造成功。

## 排错顺序

条件编译只允许留在本包或极薄启动层。若双平台语义分叉，先检查 application 统一类型，再检查目标条件；禁止复制两套业务规则。
