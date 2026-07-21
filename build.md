# 构建说明

## 当前状态

截至 2026 年 7 月 21 日，仓库仅包含许可证和筹备文档，没有可构建的应用源码，因此当前不存在产品构建命令。

## 准备阶段验证

在仓库根目录执行：

```powershell
git status --short --branch
git diff --check
gh repo view nonononull/inputcodex --json nameWithOwner,visibility,url,defaultBranchRef,licenseInfo
```

预期结果：

- 当前分支为 `main`。
- `git diff --check` 无输出且退出码为 `0`。
- GitHub 仓库为 `nonononull/inputcodex`，可见性为 `PUBLIC`。
- 许可证为 GNU AGPLv3。

## 后续维护规则

- 确定技术栈并加入首个可构建项目时，必须在同一变更中补齐环境要求、依赖安装、开发构建、发布构建、测试和产物位置。
- 新增可独立构建的子项目时，在子项目根目录新增独立 `build.md`。
- 构建失败先查阅并更新 `err.md`。
