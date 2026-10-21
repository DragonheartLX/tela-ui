# Changelog

本文件记录 tela-ui 的可见变更。格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### Added
- 组件库: 69 个导出组件 = 56 个 UI 组件 + 8 个服务 overlay (TDialogService / TLoadingService / TMessage / TNotification / TTooltip / TImageViewerService / TActionSheetService / TCommandPaletteService) + 5 个库内基类 (TStrokedShape / TInputFrame / TModalSurface / TPopupSurface / TPopupStackLayer)。
- `TTable`: 列定义 + 行模型, 表头排序回调 (`sort-changed`), 行点击回调, ListView 虚拟滚动与空态; 列宽超出时水平滚动且表头同步; 排序只上报, 行重排由宿主完成。
- `TCalendar` / `TDatePicker`: 月历面板 (闰年/大小月/首日星期在 Slint 内计算), 单选与区间选择, 只读触发器 + 锚定弹层; `TDate` 结构承载年/月/日。
- `TTimePicker`: 时间选择, 只读触发器 + 锚定时/分(秒)面板; 整列数据带动画滚动, 选中框画在选中行上随数据移动, 点击或滚轮逐格改值, 无确认按钮 (点击外部 / ESC 关闭); `TTime` 结构承载时/分/秒 (`hour < 0` = 未选); `TIconSet` 新增 `Clock`。
- `common.slint`: 补齐 `TTable` / `TSkeleton` / `TCalendar` / `TDatePicker` / `TTimePicker` 导出, 与分类聚合器保持一致。
- 浮层体系: `TPopupSurface` 锚定弹层 + `TPopupStack` 层计数 / ESC / z 序令牌。
- 设计令牌: `TTheme` global 151 个顶层转发, `TAppThemeConfig` 103 字段, 亮/暗两套色板 ramp。
- gallery 程序 (bin `tela`): 7 个页面, 按 `if` 懒挂载, 亮/暗主题经 `TELA_THEME` 环境变量选择。
- 无障碍: 可点击元素覆盖 `accessible-role` / `accessible-label`, 浮层文本带 `accessible-live-region`, 禁用态显式 `accessible-enabled` 并在动作回调自带守卫。
- 自证工具链: `docs/harness.md` 的 `--check` 全量扫描与亮/暗渲染对照配方。