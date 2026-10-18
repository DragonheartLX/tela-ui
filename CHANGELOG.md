# Changelog

本文件记录 tela-ui 的可见变更。格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### Added

- 组件库: 65 个导出组件 = 52 个 UI 组件 + 8 个服务 overlay (TDialogService / TLoadingService / TMessage / TNotification / TTooltip / TImageViewerService / TActionSheetService / TCommandPaletteService) + 5 个库内基类 (TStrokedShape / TInputFrame / TModalSurface / TPopupSurface / TPopupStackLayer)。
- 浮层体系: `TPopupSurface` 锚定弹层 + `TPopupStack` 层计数 / ESC / z 序令牌。
- 设计令牌: `TTheme` global 151 个顶层转发, `TAppThemeConfig` 103 字段, 亮/暗两套色板 ramp。
- gallery 程序 (bin `tela`): 7 个页面, 按 `if` 懒挂载, 亮/暗主题经 `TELA_THEME` 环境变量选择。
- 无障碍: 可点击元素覆盖 `accessible-role` / `accessible-label`, 浮层文本带 `accessible-live-region`, 禁用态显式 `accessible-enabled` 并在动作回调自带守卫。
- 自证工具链: `docs/harness.md` 的 `--check` 全量扫描与亮/暗渲染对照配方。

### Fixed

- P1 缺陷修复: TButton / TLink 无障碍动作守卫 disabled / loading 状态。
- 库内文本统一走 `TText`, 未覆盖字号时不再落到宿主窗口默认字体。
- 几何全部对齐令牌刻度, 阴影走 `shadow-*` 三件套令牌。

### Removed

- `TButton` 的 `block` 参数: 显式 `width` 绑定压过 `horizontal-stretch`, 该参数从未真正生效; 整行按钮改由外层布局撑满。
