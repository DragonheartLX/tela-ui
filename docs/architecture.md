# 仓库结构与统计

> `AGENTS.md` 只保留协作硬规则; 细节放这里。

## 结构

- `tela-ui/` — 组件库本体, `build.rs` 注册 `@tela-ui/...`; 按 `base/` `data-entry/` `data-display/` `feedback/` `navigation/` `experimental/` 分目录, 顶层同名聚合器导出; `common.slint` 仅作兼容出口。
- `tela-ui/structs/` — 共享枚举与主题结构; `tela-ui/colors.slint` 是亮/暗色板。
- `ui/` + `src/` — gallery (bin `tela`); `ui/MainWindow.slint` 用 `if` 懒挂 7 个页面。
- 构建: Slint 1.18.1 / edition 2024 / 样式 `fluent` / accessibility + backend-winit + renderer-skia/software。

## 统计 (2026-10-20)

- 81 个 `.slint` / 11,175 行: `tela-ui/` 72 个 9,320 行, `ui/` 9 个 1,855 行。
- 68 个导出组件 = 55 个 UI 组件 (含私有 TNavLabel) + 8 个服务 overlay + 5 个库内基类 (TStrokedShape / TInputFrame / TModalSurface / TPopupSurface / TPopupStackLayer)。
- 设计令牌: `TTheme` 151 个顶层转发 → `TAppThemeConfig` 103 字段; 色板在 `colors.slint` (亮/暗两套 ramp); `TIconSet` 43 个字形。
- 浮层体系: `TPopupSurface` 锚定弹层 + `TPopupStack` 层计数与 ESC + z 序令牌 (`z-popup` / `z-modal` / `z-message`) + 服务型 global (`TDialogService` / `TLoadingService` / `TMessage` / `TNotification` / `TTooltip` / `TImageViewerService` / `TActionSheetService` / `TCommandPaletteService`)。

## 相关文档

- `docs/slint-semantics.md` — Slint 1.18 语义实证。
- `docs/decisions.md` — 已定取舍。
- `docs/components.md` — 组件 API。
- `docs/harness.md` — 自证配方与探针踩坑。
- `docs/components-plan.md` — 本地待办 (gitignored)。
