# inputcodex-application 排错记录

## 当前合同

- 状态固定为 `Idle / Loading / Ready / Empty / Failed / Cancelling`。
- 新请求会使旧请求结果过期；进入 `Cancelling` 后的完成结果必须返回 `Stale`。
- 取消完成只接受当前请求标识，其他请求不得改变状态。

## 排错顺序

先运行 `cargo test -p inputcodex-application`。若状态断言失败，必须检查请求标识和当前状态，不得用忽略测试、全局可变状态或平台条件编译绕过。
