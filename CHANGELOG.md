# Changelog

本文件只记录版本级总体变更; 逐组件细节、机制与取舍见 `docs/components.md`、`docs/slint-semantics.md` 与 `docs/decisions.md`。

## [Unreleased]

### Changed
- 文档分层: README 只留入门, 统计进 docs/architecture.md, 已知限制进 docs/components.md, 本文件只记版本级变更。
- 统一 Slint 源码格式: 长 import/export 列表与单行组件声明展开为多行, 并清理多余空行, 不涉及行为变更。