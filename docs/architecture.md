# 仓库结构与统计

> `AGENTS.md` 只保留协作硬规则; 细节放这里。

## 结构

- `tela-ui/` — 组件库本体, `build.rs` 注册 `@tela-ui/...`; 按 `base/` `input/` `select/` `date-time/` `display/` `content/` `notice/` `overlay/` `navigation/` `experimental/` 分目录, 顶层同名聚合器导出; `common.slint` 仅作兼容出口。
- `tela-ui/structs/` — 共享枚举与主题结构; `tela-ui/colors.slint` 是亮/暗色板。
- `ui/` + `src/` — gallery (bin `tela`); `ui/MainWindow.slint` 用 `if` 懒挂 11 个页面 (10 个分类页 + 设置页)。
- 构建: Slint 1.18.1 / edition 2024 / 样式 `fluent` / accessibility + backend-winit + renderer-skia/software。

## 统计 (2026-10-25, 第四批前三项滚动增强落地后)

- 126 个 `.slint` / 19947 行: `tela-ui/` 113 个 16235 行, `ui/` 13 个 3712 行。
- 115 个导出组件 = 100 个 UI 组件 (基础 12 / 输入 14 / 选择 11 / 日期时间 5 / 展示 12 / 内容 20 / 提示 4 / 浮层 6 / 导航 15 / 实验性 1) + 8 个服务 overlay + 5 个库内基类 (TStrokedShape / TInputFrame / TModalSurface / TPopupSurface / TPopupStackLayer) + 2 个库内部件 (私有 TNavLabel / 内部 TScrollThumb, 不在聚合出口)。
- 设计令牌: `TTheme` 151 个顶层转发 → `TAppThemeConfig` 103 字段; 色板在 `colors.slint` (亮/暗两套 ramp); `TIconSet` 46 个字形。
- 滚动条: `TScrollBar` (scroll-bar.slint, 经 display.slint 公开) — 自带视口与滚动进度的滚动容器 (内建 Flickable 视口 + 集成滑块 + `scroll-to` 缓动), 替代 std ScrollView, 用于六个下拉弹层 / 两菜单 / 命令面板 / gallery 页面滚动; 滚动缓动一律组件内 Timer 逐帧写入 (slint-semantics 第 36 条); TList / TTable 保留 ListView 行回收并接内部 `TScrollThumb` (scroll-thumb.slint) 叠加条。
- 浮层体系: `TPopupSurface` 锚定弹层 + `TPopupStack` 层计数与 ESC + z 序令牌 (`z-popup` / `z-modal` / `z-message`) + 服务型 global (`TDialogService` / `TLoadingService` / `TMessage` / `TNotification` / `TTooltip` / `TImageViewerService` / `TActionSheetService` / `TCommandPaletteService`)。

## 相关文档

- `docs/slint-semantics.md` — Slint 1.18 语义实证。
- `docs/decisions.md` — 已定取舍。
- `docs/components.md` — 组件 API。
- `docs/harness.md` — 自证配方与探针踩坑。
- `docs/components-plan.md` — 本地待办 (gitignored)。
