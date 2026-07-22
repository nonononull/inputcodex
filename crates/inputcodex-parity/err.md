# inputcodex-parity 排错记录

## 当前限制

- 当前只把 application 错误转换为由 `ErrorKind + DiagnosticCode` 组成的稳定签名。
- 不读取 `upstream/`，不实现业务映射，也不依赖 platform 或 presentation。

## 排错顺序

出现一致性差异时先建立或引用一致性例外 Issue；禁止在本包直接修正产品行为或绕过项目所有者决策。
