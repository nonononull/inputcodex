# Issue #86 Session Plan：运行时环境冲突只读观察

> **执行要求：** 默认使用 `superpowers:executing-plans` 在当前隔离工作树逐任务执行；只有项目
> 所有者另行明确授权时才允许 `superpowers:subagent-driven-development`。每个 Task 必须完成
> RED、最小 GREEN、定向验证和 Git checkpoint 后才能进入下一 Task。

**Goal:** 以纯 Rust 实现当前进程 `OPENAI_*` 环境冲突只读观察，并保持环境实际值、持久化
来源和破坏性清理完全不进入运行面。

**Architecture:** Domain 保存规范名称、值存在状态和来源覆盖；Application 通过同步 Port 与现有
加载协调器返回 `Ready` 或稳定失败；Platform 仅调用一次 `std::env::vars_os()` 并把注入样本交给
纯函数；Parity 新增已实现子能力，同时保持原环境冲突总功能为 `unassessed`。

**Tech Stack:** Rust 2024、标准库 `OsString`/`BTreeMap`、现有四 crate 分层、YAML Parity 合同、
PowerShell 项目验证器、GitHub-hosted Windows/macOS CI。

## Global Constraints

- 软件名称固定为 `inputcodex`。
- 只允许 Rust；禁止 TypeScript、JavaScript、Tauri、WebView 和业务层 Iced。
- Workspace 保持 `#![forbid(unsafe_code)]`，不得增加 `unsafe`、直接 Win32 FFI 或新依赖。
- 当前切片不联网、不读写文件、不缓存、不启动线程、定时器、shell 或子进程。
- 禁止 `std::env::set_var`、`std::env::remove_var` 和任何持久化环境修改。
- 禁止返回、记录或持久化环境变量实际值；诊断证据只记录计数、覆盖、耗时和结果分类。
- Windows 按环境变量大小写不敏感语义识别；macOS 按大小写敏感语义识别。
- 只观察当前进程；用户级和系统级持久化来源必须明确为 `NotObserved`。
- 原 `feature.foundation-platform.environment-conflicts` 必须保持 `unassessed`。
- `Cargo.toml`、`Cargo.lock`、UI、预算、Release、`upstream/`、Ruleset 和 AGOS 均为受保护面；
  `parity/features/source-index.yml` 只允许修正 `tauri-command:check_env_conflicts` 的副作用和归属。
- 时间只使用 Windows 本机 `Get-Date`；禁止覆写 Git author/committer 日期。
- `24` 路径与修订候选哈希已获批准；最终 Squash Merge 仍单独授权。

---

## 会话控制

```yaml
task_id: issue-86-gate-5-runtime-environment-observation
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/86
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/85#issuecomment-5102509300
written_review_ref: https://github.com/nonononull/inputcodex/issues/86#issuecomment-5102713590
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/86#issuecomment-5103198917
branch: codex/issue-86-gate-5-runtime-environment-observation
baseline_main: 3f2914cd81ace7afe28e0137c867c20fd346c3f9
planning_scope_count: 3
planning_scope_hash: sha256:c3d16ff75e79d9fd2866db1bd59f4259089b7398bce46b20e2766fc2bccc6d34
candidate_scope_count: 24
candidate_scope_hash: sha256:dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59
allowed_operations: tdd,local-light-verification,git-checkpoint,commit,normal-push,non-draft-pr,review-ci
mutation_intent: add-runtime-environment-observation-slice-without-remediation
executor_enforcement: exact-path-scope-secret-value-and-side-effect-hard-stop
implementation_authorization: authorized
commit_push_pr_authorization: authorized
final_merge_authorization: pending-separate-gate
agos_status: bypassed-project-native-control-plane
execution_status: local-verified-ready-for-push-pr
domain_checkpoint: 6591882dc23596a502833d38aed08d585b4acc08
application_checkpoint: 55b84b6c2b45d00fdf3f6e42aaa1e86d1635557e
platform_checkpoint: cd41fa8ef739b1481cfbfc491ef42e26369f0b4e
scope_revision_checkpoint: f177b9d6f17ee31d40bb6568f8e9bdf6bec901b5
parity_red_checkpoint: d5c711d9071aa9e9c65d5214531a96e04dddda98
parity_green_checkpoint: a320086f00bd16c65ae5172c28f4bd8c40a7c110
local_verification_checkpoint: c6829fa40b7bf4cf9828f88e2dfe68c552536844
```

## 成功标准

1. Domain 只保存规范名称、`Empty | NonEmpty`、固定运行时来源和覆盖状态；
2. Application 成功只产生 `Ready`，零冲突也不得产生 `Empty`；
3. 调用方超时使用 `RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT`，迟到结果不得应用；
4. Platform 只执行一次进程环境采集，测试不修改真实环境；
5. Windows/macOS 名称比较遵守各自真实语义，结果排序去重稳定；
6. 实际变量值不进入领域、错误、诊断或生产代码格式化路径；
7. 新子能力为 `implemented`，原总功能仍为 `unassessed`；
8. 24 路径、隐私、禁止能力、四 crate、本地轻量验证与 Hosted CI 全部通过；
9. 非 Draft PR 停在独立 Squash Merge 授权门。

## Task 0：规划控制面与范围冻结

**Files:**

- Modify: `docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md`
- Create: `docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md`
- Create: `docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md`

**Interfaces:**

- Consumes: Issue `#85` 决策、Issue `#86` 书面设计批准、`main@3f2914c`。
- Produces: 3 路径 planning hash、24 路径 candidate hash、可执行批次和硬停止条件。

- [x] **Step 1: 建立并关闭一致性决策 Issue #85**

  Expected: `CLOSED/COMPLETED`，决定“观察与清理分离”。

- [x] **Step 2: 创建 Issue #86 与隔离分支**

  Expected branch: `codex/issue-86-gate-5-runtime-environment-observation`。

- [x] **Step 3: 落盘并提交书面设计**

  Commit: `26b3b1c54dd35cc92460879483b0f9d1f9d4793f`。

- [x] **Step 4: 记录书面审阅批准**

  Reference: `https://github.com/nonononull/inputcodex/issues/86#issuecomment-5102713590`。

- [x] **Step 5: 验证三路径 planning hash**

  ```powershell
  $planning = [string[]]@(
    'docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md',
    'docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md',
    'docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md'
  )
  [Array]::Sort($planning, [StringComparer]::Ordinal)
  $text = [string]::Join("`n", $planning) + "`n"
  $hash = [Convert]::ToHexString(
    [Security.Cryptography.SHA256]::HashData(
      [Text.UTF8Encoding]::new($false).GetBytes($text)
    )
  ).ToLowerInvariant()
  if ($planning.Count -ne 3) { throw 'Issue #86 planning scope count 漂移。' }
  if ($hash -ne 'c3d16ff75e79d9fd2866db1bd59f4259089b7398bce46b20e2766fc2bccc6d34') {
    throw "Issue #86 planning hash 漂移：sha256:$hash"
  }
  ```

- [x] **Step 6: 形成 planning checkpoint**

  ```powershell
  git add -- `
    docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md `
    docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md `
    docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md
  git commit -m "docs: 冻结运行时环境观察实施计划 (#86)"
  ```

  Expected: 工作区干净，尚未普通推送。

## Task 1：Domain TDD

**Files:**

- Create: `crates/inputcodex-domain/src/runtime_environment_observation.rs`
- Modify: `crates/inputcodex-domain/src/lib.rs`
- Create: `crates/inputcodex-domain/tests/runtime_environment_observation.rs`

**Interfaces:**

- Produces: `EnvironmentVariableName`、`EnvironmentValuePresence`、
  `EnvironmentConflictSource`、`EnvironmentObservationStatus`、`EnvironmentSourceCoverage`、
  `RuntimeEnvironmentConflict`、`RuntimeEnvironmentConflictObservation`。
- Consumed by: Application Port 和 Platform 纯观察函数。

- [x] **Step 1: 编写 Domain RED 测试**

  ```rust
  use inputcodex_domain::{
      EnvironmentObservationStatus, EnvironmentSourceCoverage, EnvironmentValuePresence,
      EnvironmentVariableName, RuntimeEnvironmentConflict,
      RuntimeEnvironmentConflictObservation,
  };

  #[test]
  fn 观察结果排序去重且非空值优先() {
      let empty = RuntimeEnvironmentConflict::runtime_process(
          EnvironmentVariableName::new("OPENAI_API_KEY".to_owned()).unwrap(),
          EnvironmentValuePresence::Empty,
      );
      let non_empty = RuntimeEnvironmentConflict::runtime_process(
          EnvironmentVariableName::new("OPENAI_API_KEY".to_owned()).unwrap(),
          EnvironmentValuePresence::NonEmpty,
      );
      let observation = RuntimeEnvironmentConflictObservation::new(7, vec![empty, non_empty]);

      assert_eq!(observation.scanned_entry_count(), 7);
      assert_eq!(observation.conflicts().len(), 1);
      assert_eq!(
          observation.conflicts()[0].value_presence(),
          EnvironmentValuePresence::NonEmpty
      );
      assert_eq!(
          observation.coverage(),
          &EnvironmentSourceCoverage::runtime_only()
      );
      assert_eq!(
          observation.coverage().persistent_user(),
          EnvironmentObservationStatus::NotObserved
      );
  }
  ```

- [x] **Step 2: 运行 RED**

  Run:

  ```powershell
  cargo test --locked --offline --ignore-rust-version -p inputcodex-domain `
    --test runtime_environment_observation
  ```

  Expected: FAIL，原因是公开类型和模块尚不存在。

- [x] **Step 3: 实现最小 Domain 类型**

  ```rust
  #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct EnvironmentVariableName(String);

  impl EnvironmentVariableName {
      pub fn new(value: String) -> Result<Self, EnvironmentVariableNameError> {
          if value.starts_with("OPENAI_") {
              Ok(Self(value))
          } else {
              Err(EnvironmentVariableNameError::InvalidPrefix)
          }
      }

      pub fn as_str(&self) -> &str {
          &self.0
      }
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum EnvironmentValuePresence {
      Empty,
      NonEmpty,
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum EnvironmentObservationStatus {
      Observed,
      NotObserved,
  }
  ```

  `RuntimeEnvironmentConflictObservation::new` 必须使用 `BTreeMap` 按名称合并重复项；同名
  冲突只要任一值为 `NonEmpty`，最终状态即为 `NonEmpty`。

- [x] **Step 4: 运行 Domain GREEN 与格式检查**

  ```powershell
  cargo test --locked --offline --ignore-rust-version -p inputcodex-domain `
    --test runtime_environment_observation
  cargo fmt --all -- --check
  ```

  Expected: Domain 目标测试全绿，格式退出码为 `0`。

- [x] **Step 5: 创建 Domain checkpoint**

  ```powershell
  git add -- `
    crates/inputcodex-domain/src/lib.rs `
    crates/inputcodex-domain/src/runtime_environment_observation.rs `
    crates/inputcodex-domain/tests/runtime_environment_observation.rs
  git commit -m "feat: 定义运行时环境冲突领域模型 (#86)"
  ```

## Task 2：Application TDD

**Files:**

- Modify: `crates/inputcodex-application/src/lib.rs`
- Create: `crates/inputcodex-application/src/runtime_environment_observation.rs`
- Create: `crates/inputcodex-application/tests/runtime_environment_observation.rs`

**Interfaces:**

- Consumes: `RuntimeEnvironmentConflictObservation`。
- Produces: `RuntimeEnvironmentObservationRequest`、`RuntimeEnvironmentObservationPort`、
  `ObserveRuntimeEnvironmentConflicts<P>`、`ApplicationError::timeout`。

- [x] **Step 1: 编写 Application RED 测试**

  ```rust
  #[derive(Clone)]
  struct StubPort(Result<RuntimeEnvironmentConflictObservation, ApplicationError>);

  impl RuntimeEnvironmentObservationPort for StubPort {
      fn observe(
          &self,
          _request: &RuntimeEnvironmentObservationRequest,
      ) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
          self.0.clone()
      }
  }

  #[test]
  fn 零冲突返回_ready_而不是_empty() {
      let expected = RuntimeEnvironmentConflictObservation::new(3, Vec::new());
      let use_case = ObserveRuntimeEnvironmentConflicts::new(StubPort(Ok(expected.clone())));
      let completion = use_case.execute(&RuntimeEnvironmentObservationRequest);

      assert_eq!(completion, LoadCompletion::Ready(expected));
      assert!(!matches!(completion, LoadCompletion::Empty));
  }
  ```

  同文件继续固定超时错误、取消后迟到结果和旧请求不覆盖新请求。

- [x] **Step 2: 运行 Application RED**

  ```powershell
  cargo test --locked --offline --ignore-rust-version -p inputcodex-application `
    --test runtime_environment_observation
  ```

  Expected: FAIL，原因是 Port、UseCase 和 `ApplicationError::timeout` 尚不存在。

- [x] **Step 3: 实现最小 Application 接口**

  ```rust
  #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
  pub struct RuntimeEnvironmentObservationRequest;

  pub trait RuntimeEnvironmentObservationPort {
      fn observe(
          &self,
          request: &RuntimeEnvironmentObservationRequest,
      ) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError>;
  }

  #[derive(Clone)]
  pub struct ObserveRuntimeEnvironmentConflicts<P> {
      port: P,
  }

  impl<P: RuntimeEnvironmentObservationPort> ObserveRuntimeEnvironmentConflicts<P> {
      pub fn execute(
          &self,
          request: &RuntimeEnvironmentObservationRequest,
      ) -> LoadCompletion<RuntimeEnvironmentConflictObservation> {
          match self.port.observe(request) {
              Ok(value) => LoadCompletion::Ready(value),
              Err(error) => LoadCompletion::Failed(error),
          }
      }
  }
  ```

  `ApplicationError::timeout` 必须构造 `ErrorKind::Timeout`，错误码固定由调用方传入。

- [x] **Step 4: 运行 Application GREEN**

  ```powershell
  cargo test --locked --offline --ignore-rust-version -p inputcodex-application `
    --test runtime_environment_observation
  cargo fmt --all -- --check
  ```

- [x] **Step 5: 创建 Application checkpoint**

  ```powershell
  git add -- `
    crates/inputcodex-application/src/lib.rs `
    crates/inputcodex-application/src/runtime_environment_observation.rs `
    crates/inputcodex-application/tests/runtime_environment_observation.rs
  git commit -m "feat: 添加运行时环境观察用例 (#86)"
  ```

## Task 3：Platform TDD

**Files:**

- Modify: `crates/inputcodex-platform/src/lib.rs`
- Create: `crates/inputcodex-platform/src/runtime_environment_observation.rs`
- Create: `crates/inputcodex-platform/tests/runtime_environment_observation.rs`

**Interfaces:**

- Consumes: Domain 观察类型和 Application Port。
- Produces: `SystemRuntimeEnvironmentObservation`、
  `observe_windows_runtime_environment`、`observe_macos_runtime_environment`。

- [x] **Step 1: 编写 Platform RED 测试**

  ```rust
  #[test]
  fn windows_大小写不敏感而_macos_大小写敏感() {
      let pairs = vec![
          (OsString::from("openai_api_key"), OsString::from("secret")),
          (OsString::from("OPENAI_BASE_URL"), OsString::new()),
      ];

      let windows = observe_windows_runtime_environment(pairs.clone()).unwrap();
      assert_eq!(windows.conflicts().len(), 2);
      assert_eq!(windows.conflicts()[0].name().as_str(), "OPENAI_API_KEY");

      let macos = observe_macos_runtime_environment(pairs).unwrap();
      assert_eq!(macos.conflicts().len(), 1);
      assert_eq!(macos.conflicts()[0].name().as_str(), "OPENAI_BASE_URL");
  }

  #[test]
  fn 实际环境值不会进入结果或_debug() {
      let result = observe_windows_runtime_environment(vec![(
          OsString::from("OPENAI_API_KEY"),
          OsString::from("sk-private-sentinel"),
      )])
      .unwrap();

      assert!(!format!("{result:?}").contains("sk-private-sentinel"));
  }
  ```

  继续覆盖不修剪、排除 `CUSTOM_OPENAI_*`、空值、重复项、不可表示名称和 unsupported。

- [x] **Step 2: 运行 Platform RED**

  ```powershell
  cargo test --locked --offline --ignore-rust-version -p inputcodex-platform `
    --test runtime_environment_observation
  ```

  Expected: FAIL，原因是系统适配器和纯观察函数尚不存在。

- [x] **Step 3: 实现纯观察函数与系统适配器**

  ```rust
  #[derive(Debug, Clone, Copy, Default)]
  pub struct SystemRuntimeEnvironmentObservation;

  impl RuntimeEnvironmentObservationPort for SystemRuntimeEnvironmentObservation {
      fn observe(
          &self,
          _request: &RuntimeEnvironmentObservationRequest,
      ) -> Result<RuntimeEnvironmentConflictObservation, ApplicationError> {
          #[cfg(target_os = "windows")]
          {
              observe_windows_runtime_environment(std::env::vars_os())
          }
          #[cfg(target_os = "macos")]
          {
              observe_macos_runtime_environment(std::env::vars_os())
          }
          #[cfg(not(any(target_os = "windows", target_os = "macos")))]
          {
              Err(ApplicationError::unsupported(
                  "RUNTIME_ENVIRONMENT_OBSERVATION_UNSUPPORTED",
              ))
          }
      }
  }
  ```

  两个纯观察函数必须先按平台名称规则判断候选，再无损转换命中名称；值只调用
  `OsStr::is_empty`，不得转换或格式化。

- [x] **Step 4: 运行 Platform GREEN 与静态扫描**

  ```powershell
  cargo test --locked --offline --ignore-rust-version -p inputcodex-platform `
    --test runtime_environment_observation
  cargo fmt --all -- --check

  $forbidden = @(
    rg -n 'set_var|remove_var|std::fs|std::process::Command|std::thread|unsafe' `
      crates/inputcodex-platform/src/runtime_environment_observation.rs
  )
  if ($forbidden.Count -ne 0) { throw "Platform 出现禁止能力：$($forbidden -join '; ')" }
  ```

- [x] **Step 5: 创建 Platform checkpoint**

  ```powershell
  git add -- `
    crates/inputcodex-platform/src/lib.rs `
    crates/inputcodex-platform/src/runtime_environment_observation.rs `
    crates/inputcodex-platform/tests/runtime_environment_observation.rs
  git commit -m "feat: 观察双平台运行时环境冲突 (#86)"
  ```

## Task 4：Parity TDD

**Files:**

- Modify: `crates/inputcodex-parity/tests/catalog_repository.rs`
- Modify: `parity/features/foundation-platform.yml`
- Modify: `parity/contracts/foundation-platform.yml`
- Modify: `parity/features/source-index.yml`
- Modify: `parity/README.md`

**Interfaces:**

- Consumes: Issue `#85/#86` 稳定语义和已实现 Rust 类型。
- Produces: 新子能力 `implemented` 合同，同时保留原总功能 `unassessed`。

- [x] **Step 1: 编写 Parity RED 断言**

  ```rust
  #[test]
  fn gate5_运行时环境观察已实现但破坏性总功能仍未评估() {
      let feature_text = read_repository_text("parity/features/foundation-platform.yml");
      let observation = yaml_list_item_block(
          &feature_text,
          "feature.foundation-platform.runtime-environment-conflict-observation",
      );
      assert!(observation.contains("status: implemented"));
      assert!(observation.contains("- issue:85"));
      assert!(observation.contains("- issue:86"));

      let umbrella = yaml_list_item_block(
          &feature_text,
          "feature.foundation-platform.environment-conflicts",
      );
      assert!(umbrella.contains("status: unassessed"));
      assert!(!umbrella.contains("status: implemented"));
  }
  ```

  合同断言必须包含 `environment-read`、`Ready(empty)`、三个覆盖字段、三条稳定错误/状态语义，
  并拒绝环境写入、文件写入和变量值。

- [x] **Step 2: 运行 Parity RED**

  ```powershell
  cargo test --locked --offline --ignore-rust-version -p inputcodex-parity `
    --test catalog_repository gate5_运行时环境观察已实现但破坏性总功能仍未评估
  ```

  Expected: FAIL，新子能力和合同尚不存在。

- [x] **Step 3: 增加 Feature 与 Contract**

  Feature 必须只引用：

  ```yaml
  evidence:
    - path: upstream/CodexPlusPlus/crates/codex-plus-core/src/env_conflicts.rs
      symbol: detected_env_conflicts_from_pairs
    - path: upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs
      symbol: check_env_conflicts
  ```

  Contract 必须固定：

  ```yaml
  side_effects:
    - environment-read
  persistence: 'none'
  ```

  不得把 `remove_env_conflicts` 入口归入新子能力。

  `source-index.yml` 只允许：

  - 把 `tauri-command:check_env_conflicts` 的 `side_effects` 修正为 `[environment-read]`；
  - 把该入口映射到 `feature.foundation-platform.runtime-environment-conflict-observation`；
  - 保持 `core-module:env_conflicts` 与 `tauri-command:remove_env_conflicts` 的现有归属和读写副作用。

- [x] **Step 4: 运行 Parity GREEN**

  ```powershell
  cargo test --locked --offline --ignore-rust-version -p inputcodex-parity `
    --test catalog_repository
  $sourceIndexChanges = @(git diff --name-only origin/main...HEAD -- parity/features/source-index.yml)
  if ($sourceIndexChanges.Count -ne 1) {
    throw "Issue #86 source-index 修订缺失或漂移：$($sourceIndexChanges -join ', ')"
  }
  ```

  Expected: `catalog_repository` 全绿，`source-index.yml` 只包含批准的单入口修订。

- [x] **Step 5: 创建 Parity checkpoint**

  ```powershell
  git add -- `
    crates/inputcodex-parity/tests/catalog_repository.rs `
    parity/features/foundation-platform.yml `
    parity/contracts/foundation-platform.yml `
    parity/features/source-index.yml `
    parity/README.md
  git commit -m "feat: 记录运行时环境观察一致性合同 (#86)"
  ```

## Task 5：项目原生控制面收口

**Files:**

- Modify: `AGENTS.md`
- Modify: `CONTEXT.md`
- Modify: `README.md`
- Modify: `build.md`
- Modify if new root cause only: `err.md`
- Modify: `docs/plans/PROJECT-MASTER-PLAN.md`
- Create: `docs/reports/issue-86-gate-5-runtime-environment-observation.md`
- Modify: 本 Session Plan、设计稿和 Runtime Workflow。

**Interfaces:**

- Consumes: 四层 GREEN checkpoint 与实际验证证据。
- Produces: 当前 Gate 状态、构建命令、领域术语、实施报告和下一合法阶段。

- [x] **Step 1: 更新稳定产品文档**

  README 只增加“运行时环境冲突观察”，明确当前进程覆盖和实际值不读取；不得描述环境清理。

- [x] **Step 2: 更新项目规则与总计划**

  `AGENTS.md` 和 Master Plan 将 Issue `#86` 标为当前第四切片；下一合法阶段只能是本 Issue 的
  Review/CI 或合并后第五切片选择。

- [x] **Step 3: 更新 build.md 验证命令**

  增加四 crate 定向测试、Clippy、格式、CI 合同、Release Audit、Repository Policy、24 路径、
  隐私、禁止能力和受保护路径检查。

- [x] **Step 4: 处理 err.md**

  先查重；只有出现新的可复用根因才增加条目。重复 PowerShell here-string、未跟踪文件统计或
  原生命令退出码问题只引用既有结论，保持文件未修改。

- [x] **Step 5: 创建实施报告与控制面 checkpoint**

  ```powershell
  git add -- `
    AGENTS.md CONTEXT.md README.md build.md `
    docs/plans/PROJECT-MASTER-PLAN.md `
    docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md `
    docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md `
    docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md `
    docs/reports/issue-86-gate-5-runtime-environment-observation.md `
    err.md
  git commit -m "docs: 收口运行时环境观察控制面 (#86)"
  ```

  `err.md` 未变化时 `git add` 不会产生该路径差异，候选范围仍保持不变。

## Task 6：完整本地轻量验证

**Files:** 所有 24 条候选路径，只读验证受保护面。

- [x] **Step 1: 四 crate 测试**

  ```powershell
  cargo test --locked --offline --ignore-rust-version `
    -p inputcodex-domain `
    -p inputcodex-application `
    -p inputcodex-platform `
    -p inputcodex-parity
  ```

- [x] **Step 2: 四 crate Clippy 与格式**

  ```powershell
  cargo clippy --locked --offline --ignore-rust-version `
    -p inputcodex-domain `
    -p inputcodex-application `
    -p inputcodex-platform `
    -p inputcodex-parity `
    --all-targets -- -D warnings
  cargo fmt --all -- --check
  ```

- [x] **Step 3: 项目验证器**

  ```powershell
  pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
  pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
  pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
  git diff --check
  ```

  Expected: CI 合同 `35/35`、`release_audit=current`、政策 `0` 违规。

- [x] **Step 4: 范围与禁止能力验证**

  严格执行 Runtime Workflow 的 24 路径、保护路径、隐私和禁止能力脚本。

- [x] **Step 5: 创建最终本地验证 checkpoint**

  ```powershell
  git add -- <实际变化且位于候选范围内的路径>
  git commit -m "test: 验证运行时环境观察切片 (#86)"
  ```

## Task 7：远端交付与 Review/CI

- [x] **Step 1: 取得项目所有者 24 路径与修订哈希批准**

  批准前不得执行本 Task 或 Task 1-6 的实现写入。

- [ ] **Step 2: 普通推送**

  ```powershell
  git push -u origin codex/issue-86-gate-5-runtime-environment-observation
  ```

  禁止 `--force` 与 `--force-with-lease`。

- [ ] **Step 3: 创建非 Draft PR**

  PR 必须关联 Issue `#85/#86`，正文列出 24 路径哈希、RED/GREEN checkpoint、本地验证和硬边界。

- [ ] **Step 4: Review 根因闭环**

  每条对话记录根因、处理方式和验证证据；不成立的反馈取得 reviewer 或项目所有者确认。

- [ ] **Step 5: Hosted CI 最终核验**

  Final Head 的标准 CI 与 Performance Baseline 必须完成合同验证；成功 Run Artifact 为 `0`。

- [ ] **Step 6: 停在独立 Squash Merge 授权门**

  未收到“授权 Squash Merge PR #…”不得合并或删除分支。

## 停止条件

出现任一情况立即硬停止并回到 Issue `#86`：

- 24 路径或候选哈希发生变化；
- `source-index.yml` 超出批准的 `check_env_conflicts` 单入口修订；
- 需要 Cargo、新依赖、`upstream/`、UI、预算、Release、Ruleset 或 AGOS；
- 需要读取/修改持久化环境、执行子进程、写文件、联网、线程或 `unsafe`；
- 环境变量实际值进入领域、错误、诊断或生产 `Debug`；
- 原总功能被改为 `implemented`；
- 本地测试、Release Audit、政策或 Hosted CI 失败且根因未确定；
- Review 对话未全部解决；
- 最终 Squash Merge 未获得单独授权。
