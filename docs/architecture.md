# 仓库结构与统计

> `AGENTS.md` 只保留协作硬规则; 文档地图也在那里。

## 结构

- `tela-ui/` — 组件库本体, `build.rs` 注册 `@tela-ui/...`; 分类目录 + 顶层同名聚合器, `common.slint` 仅兼容出口。
- `tela-ui/structs/` — 共享枚举与主题结构; `colors.slint` 为亮/暗色板。
- `ui/` + `src/` — gallery (bin `tela`); `MainWindow.slint` 用 `if` 懒挂 11 页 (10 分类 + 设置)。设置页 `AboutSlint` 是标准库组件, 版本行默认黑字, 暗色下对比度低。
- 构建: Slint 1.18.1 / edition 2024 / `fluent` / accessibility + winit + skia/software。

## 统计 (2026-10-28)

- 127 个 `.slint` / 21951 行: `tela-ui/` 114 / 16888, `ui/` 13 / 5063。
- 116 个导出 = 100 UI (基础 12 / 输入 14 / 选择 11 / 日期时间 5 / 展示 12 / 内容 20 / 提示 4 / 浮层 6 / 导航 15 / 实验 1) + 9 服务 overlay + 5 库内基类 + 2 库内部件。
- 令牌: `TTheme` 151 顶层转发 → `TAppThemeConfig` 103 字段; 亮/暗色板; `TIconSet` 46 字形。
- `TScrollBar` 自带视口/滑块/`scroll-to` 缓动, 替代 std ScrollView; 缓动为组件内 Timer (语义 36); TList/TTable 用内部 `TScrollThumb` 叠加条。
- 浮层: `TPopupSurface` + `TPopupStack` (层计数/ESC/z 序) + 9 个服务 global。
