# Changelog

本文件只记录版本级总体变更; 逐组件细节、机制与取舍见 `docs/components.md`、`docs/slint-semantics.md` 与 `docs/decisions.md`。

## [Unreleased]


## [0.1.0] - 2026-11-05

首个版本: 面向 Slint 1.18.1 的 TDesign 风格组件库。

- 116 个导出: 100 个 UI 组件 (基础 12 / 输入 14 / 选择 11 / 日期时间 5 / 展示 12 / 内容 20 / 提示 4 / 浮层 6 / 导航 15 / 实验 1) + 9 个服务浮层 + 5 个库内基类 + 2 个库内部件; 按交互类型聚合导出 (`@tela-ui/base` 等), `@tela-ui/common.slint` 为全量兼容出口。
- 服务型浮层: 对话框 / 加载 / 消息 / 通知 / 图片预览 / 命令面板 / 动作面板 / 提示 / 漫游引导; `TPopupSurface` + `TPopupStack` 统一管层计数、ESC 与 z 序。
- 主题: `TTheme` 全局 151 个顶层令牌转发至 `TAppThemeConfig` 103 字段; 亮/暗色板见 `colors.slint`; 组件模式偏好走标准库 `Palette.color-scheme`; `TIconSet` 内置 46 字形 (Lucide 几何数据)。
- 滚动: `TScrollBar` 滚动容器 + 共享滑块 (视口同步 / `scroll-to` 缓动), 替代 std ScrollView; `TList` / `TTable` 用内部 `TScrollThumb` 叠加条。
- 可访问性: 组件 accessible role / label / enabled 与默认动作, 浮层消息用 accessible-live-region。
- gallery (bin `tela`): 11 页组件陈列, 亮/暗双主题, `TELA_THEME` 可强制主题; 随包附带亮/暗截图基线与变改探针配方 (`docs/harness.md`)。
