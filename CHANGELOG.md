# Changelog

本文件只记录版本级总体变更; 逐组件细节、机制与取舍见 `docs/components.md`、`docs/slint-semantics.md` 与 `docs/decisions.md`。

## [Unreleased]

### Breaking Changes
- Removed `TNotificationData.stamp`, `TNotification.seq`, and the `MainWindow` `page-scroll-y` out property

### Added
- Accessibility support: accessible roles, labels, enabled state, and default actions across input, select, context-menu, tag, and navigation components

### Changed
- Renamed the library crate from `tela_lib` to `tela_ui` and moved bootstrap, panic reporting, and `TELA_THEME` handling into the binary target
- `TLoading.size` and `TEmpty.icon-size` are now input properties
- `TSelect` popup item height now derives from `TTheme.height-m`

### Fixed
- `TInput` placeholder indent now tracks the prefix icon size
- Re-picking the already selected `TSelect` option no longer fires `selected` again