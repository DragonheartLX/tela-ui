# Changelog

本文件只记录版本级总体变更; 逐组件细节、机制与取舍见 `docs/components.md`、`docs/slint-semantics.md` 与 `docs/decisions.md`。

## [Unreleased]

### Added
- 漫游引导: `TGuideService` + 每步一个 `TGuideOverlay` (高亮遮罩 + 自动选边气泡, 状态在全局, 动作走回调)。