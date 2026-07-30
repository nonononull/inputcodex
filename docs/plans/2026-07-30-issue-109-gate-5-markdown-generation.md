# Issue #109 受控会话 Markdown 生成实施计划

> **执行要求：** 按项目原生 Runtime Workflow 分批实施；生产代码必须先有可观察 RED，再写最小 GREEN。`docs/superpowers/*` 不是本任务控制面。

**目标：** 从受控本地会话 ID 严格只读生成确定性 Markdown 内存结果，不保存文件、不创建 UI。

**架构：** Domain 负责消息、UTC 时间戳、建议文件名和逐字节确定性渲染；Application 负责请求、取消、Port 与加载完成态；Platform 复用第十切片的 SQLite 根和候选语义，负责只读查询、受控 rollout 定位、流式 JSONL 解析及错误脱敏；Parity 把误命名的后端“导出”重新分类为生成能力，并将注入保存继续留在既有 renderer 例外中。

**技术栈：** Rust 2024、`rusqlite 0.40.1`、`serde_json 1.0.149`、项目既有加载协调器与 GitHub-hosted CI；不新增依赖。

## 全局约束

- 基线固定为 `origin/main@88eaf6301f1897cadaf4da830db998078fb06e97`。
- 决策证据为 Issue `#108` 的项目所有者回复 `A`。
- 候选范围为 29 路径，`sha256:b113da5d41514f50e36cef7d4eb9ade89e2562cbcfe8d392a5173d38fd0ebaac`；其中旧/new fixture 各两份路径均显式计入。
- SQLite 候选最多 `32`；rollout 目录深度最多 `4`、枚举项最多 `8192`、JSONL 候选最多 `4096`。
- rollout 发现累计最多读取 `32 MiB`，单候选 metadata 前缀最多 `8 KiB`。
- 选中 rollout 最大 `16 MiB`、最多 `100000` 个非空 JSONL 记录、最多 `20000` 条导出消息。
- Markdown 最大 `16 MiB`，建议文件名最大 `160` UTF-8 字节。
- SQLite busy timeout `50 ms`、progress interval `1000`、整体 deadline `2 s`。
- 只输出用户/助手文本和 `> Image attachment omitted`；不输出内部角色、data URL、远程图片 URL 或图片字节。
- 不修改 Cargo、UI、Workflow、Ruleset、Release、上游缓存或 AGOS。

## 文件职责

- `crates/inputcodex-domain/src/markdown_generation.rs`：纯领域消息、时间戳、文件名和渲染结果。
- `crates/inputcodex-application/src/markdown_generation.rs`：请求验证、取消、Port 和 UseCase。
- `crates/inputcodex-platform/src/markdown_generation.rs`：受控数据源、流式解析与系统适配器。
- 三个同名测试文件：分别锁定 Domain、Application、Platform 行为。
- `parity/*session-data*` 与 fixture：锁定能力重分类和稳定合同。
- `parity/features/plugin-script.yml`：仅补充 `saveMarkdown` 仍属于既有 renderer 注入例外的证据。

### Task 1：Domain RED → GREEN

**文件：**
- 新建：`crates/inputcodex-domain/tests/markdown_generation.rs`
- 新建：`crates/inputcodex-domain/src/markdown_generation.rs`
- 修改：`crates/inputcodex-domain/src/lib.rs`

**接口：**

```rust
pub enum MarkdownMessageRole { User, Assistant }
pub struct MarkdownUtcTimestamp;
pub struct MarkdownMessage;
pub struct SessionMarkdownDocument;

impl MarkdownUtcTimestamp {
    pub fn new(value: String) -> Result<Self, MarkdownGenerationError>;
}
impl MarkdownMessage {
    pub fn new(
        role: MarkdownMessageRole,
        timestamp: Option<MarkdownUtcTimestamp>,
        body: String,
    ) -> Result<Self, MarkdownGenerationError>;
}
impl SessionMarkdownDocument {
    pub fn generate(
        title: Option<&LocalSessionTitle>,
        messages: Vec<MarkdownMessage>,
    ) -> Result<Self, MarkdownGenerationError>;
}
```

- [x] 先写测试：角色标题、UTC 行、LF、图片占位正文、文件名清洗/截断、零消息、消息/输出上限和脱敏 Debug。
- [x] 运行 `cargo test --locked --offline -p inputcodex-domain --test markdown_generation`，确认因目标 API 缺失而 RED。
- [x] 实现最小领域类型；文件名固定 `session-<clean-title>.md`，不使用 session ID。
- [x] 运行 Domain 测试、Clippy 与 fmt，确认 GREEN 后建立 `issue-109-domain-green` checkpoint。

### Task 2：Application RED → GREEN

**文件：**
- 新建：`crates/inputcodex-application/tests/markdown_generation.rs`
- 新建：`crates/inputcodex-application/src/markdown_generation.rs`
- 修改：`crates/inputcodex-application/src/lib.rs`

**接口：**

```rust
pub struct MarkdownGenerationRequest;
pub struct MarkdownGenerationCancellation;
pub trait MarkdownGenerationPort {
    fn generate(
        &self,
        request: &MarkdownGenerationRequest,
        cancellation: &MarkdownGenerationCancellation,
    ) -> Result<Option<SessionMarkdownDocument>, ApplicationError>;
}
pub struct GenerateSessionMarkdown<P>;
```

- [x] 先写测试：合法/非法 session ID、请求 Debug 脱敏、克隆取消、`Some/None/Err` 到 `Ready/Empty/Failed`、执行前取消和旧结果隔离。
- [x] 运行定向测试，确认只因 Application API 缺失而 RED。
- [x] 实现最小 Request/Cancellation/Port/UseCase；复用 `MAX_LOCAL_SESSION_ID_BYTES`。
- [x] 运行 Application 测试、Clippy 与 fmt，确认 GREEN 后建立 `issue-109-application-green` checkpoint。

### Task 3：Platform RED → GREEN

**文件：**
- 新建：`crates/inputcodex-platform/tests/markdown_generation.rs`
- 新建：`crates/inputcodex-platform/src/markdown_generation.rs`
- 修改：`crates/inputcodex-platform/src/lib.rs`

**接口：**

```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemMarkdownGeneration;

impl MarkdownGenerationPort for SystemMarkdownGeneration { /* 系统入口 */ }
```

- [x] 先写合成测试：零来源、threads/automation_runs、跨库去重、显式/发现 rollout、根越界、符号链接种类、只读/query_only、WAL 业务不变、malformed JSON、角色过滤、图片占位、UTC 正负偏移与日期边界、所有资源上限、超时/取消和错误脱敏。
- [x] 运行 `cargo test --locked --offline -p inputcodex-platform --test markdown_generation`，确认因平台模块/API 缺失而 RED。
- [x] 实现 SQLite 精确参数查询；任何候选数据库失败都阻断生成，避免较旧重复记录冒充权威记录。
- [x] 实现固定根候选发现和流式 JSONL 解析；不得使用 `read_to_string`。
- [x] 实现不依赖本机时区和新依赖的 RFC 3339 → UTC 规范化，并覆盖闰年、跨日/月/年与正负 offset。
- [x] 运行 Platform 测试、Clippy 与 fmt；确认 GREEN 后建立 `issue-109-platform-green` checkpoint。

### Task 4：Parity RED → GREEN

**文件：**
- 修改：`crates/inputcodex-parity/tests/catalog_repository.rs`
- 修改：`parity/features/session-data.yml`
- 修改：`parity/contracts/session-data.yml`
- 修改：`parity/features/source-index.yml`
- 修改：`parity/features/plugin-script.yml`
- 新建：`parity/fixtures/feature.session-data.markdown-generation/baseline.yml`
- 新建：`parity/fixtures/feature.session-data.markdown-generation/manifest.yml`
- 删除：`parity/fixtures/feature.session-data.markdown-export/baseline.yml`
- 删除：`parity/fixtures/feature.session-data.markdown-export/manifest.yml`

**说明：** 删除旧 fixture 与新增新 fixture 均计入两个目标路径，不扩大 allowlist；Git 预期识别为 rename。现有 Source Index 对公开模块是一对一映射，因此将误命名的 `markdown-export` 条目重分类为 `markdown-generation`；真实保存代码继续由既有 `feature.plugin-script.renderer-enhancements` 的 `exception-pending` 注入证据承接，不能把完整导出标为已实现。

- [x] 先写目录测试，要求 generation 为 implemented、data module 仅有 database/filesystem read、保存仍 exception-pending、总数保持 `43/43/12`。
- [x] 运行 Parity 定向测试，确认旧目录使新断言 RED。
- [x] 重分类 feature/contract/fixture/source mapping，并补 `saveMarkdown` 注入证据。
- [x] 运行 Parity 全目标测试、Clippy、fmt 与 Release Audit，确认 GREEN 后建立 `issue-109-parity-green` checkpoint。

### Task 5：稳定文档与本地收口

**文件：**
- 修改：`README.md`、`AGENTS.md`、`build.md`、`CONTEXT.md`、Master Plan 与三份任务控制文档。
- 新建：`docs/reports/issue-109-gate-5-markdown-generation.md`
- 可选修改：`err.md`，仅限不存在等价条目的新可复用根因。

- [x] 更新公开稳定能力，明确“生成不等于保存”。
- [x] 运行 `build.md` 的 Issue #109 完整本地轻量门禁。
- [x] 执行安全审查：路径、SQL 参数、边界、Debug/错误、真实数据和禁止能力逐项核对。
- [x] 执行 AGOS ReportOnly；`needs-input/unregistered` 记录后绕过，不修改外部控制面。
- [x] 建立 `issue-109-local-verified` checkpoint `623e35a`。
- [x] 执行 `origin/main..623e35a` 独立只读评审，取得 `0 Critical / 5 Important / 3 Minor`。
- [x] 以 RED/GREEN 修复六项成立反馈：目录预分配、SQLite 文本、路径打开竞态、重复 rollout、显式深度与闰秒位置。
- [x] 有证据驳回两项：生成层不得篡改批准保留的用户 Markdown 文本；精确根比较保持 fail-closed，不在大小写敏感卷上放宽。
- [x] 重跑完整本地门禁，确认 `29 / sha256:b113da...` 与四 crate 全绿。
- [ ] 建立 review-correction checkpoint，普通 push 并创建关联 #109 的非 Draft PR。

## 成功标准

- TDD RED/GREEN 证据覆盖四层。
- 相关四 crate tests/Clippy、fmt、CI 合同、仓库政策、Release Audit、Cargo metadata 与 diff check 全绿。
- 实际范围精确为含 `err.md` 的 29 路径，或无新根因时排除它的 28 路径。
- GitHub-hosted CI 与 Performance Baseline 合同通过，成功 Artifact 为 0。
- Final Head 合并前取得项目所有者独立 Squash Merge 授权。
