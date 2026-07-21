# Runtime Workflow：inputcodex 仓库筹备

```yaml
task_id: 2026-07-21-inputcodex-bootstrap
session_plan_ref: docs/plans/sessions/2026-07-21-inputcodex-bootstrap.md
approved_decision_ref: user:2026-07-21#continue-preparation
selected_business_path: project-bootstrap
node_order:
  - 读取项目与全局规则
  - 完成本地知识查询
  - 核对两份参考仓库元数据与许可证
  - 创建 GitHub 公开仓库
  - 关联本地 main 与 origin/main
  - 创建筹备 Issue
  - 写入项目原生控制文档
  - 运行格式、GitHub 与 Git 快照验证
  - 提交并推送准备基线
subagent_roles: []
skill_tree_nodes:
  - superpowers:using-superpowers
  - superpowers:brainstorming
  - superpowers:writing-plans
  - superpowers:systematic-debugging
  - karpathy-guidelines
code_authoring_quality_rules:
  - 仅改批准范围
  - 不导入应用源码
  - 不提前决定架构
  - 所有结论有命令或文档证据
model_drift_guards:
  - 名称必须保持 inputcodex
  - 无广告原则不得弱化
  - 架构讨论前不得开始实现
err_md_correction_watchlist:
  - AGOS 默认入口缺少项目入口文档
  - 新旧远端分支关联错误
  - 许可证与参考项目不一致
  - Windows 命令行长度上限
  - 桌面版 apply_patch 包装器拒绝启动
stop_gates:
  - 用户改变准备范围
  - GitHub 仓库名已被占用
  - 参考仓库许可证核验失败
verification_gates:
  - git diff --check
  - git status --short --branch
  - gh repo view nonononull/inputcodex
  - AGOS Git snapshot checkpoint
rollout_draft:
  reusable_path: 新项目公开仓库最小治理初始化
  record_at_closeout: false
  reason: 单次准备流程尚未达到 workflow candidate 重复阈值
```
