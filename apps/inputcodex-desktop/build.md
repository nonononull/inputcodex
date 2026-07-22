# inputcodex-desktop 构建说明

## 包定位

- 包名：`inputcodex-desktop`。
- 只负责组装 application、infrastructure、platform 与 presentation，并启动展示层生命周期。
- 不实现业务规则、安装、更新、注入或发布逻辑。

## 本地默认

本地只解析元数据，不默认编译 Iced 桌面依赖：

```powershell
cargo metadata --no-deps --format-version 1
```

## GitHub Actions 全量验证

```powershell
cargo check -p inputcodex-desktop
```

Windows 与 macOS 必须分别在标准 GitHub-hosted runner 成功；Linux 只承担质量验证，不代表产品支持。
