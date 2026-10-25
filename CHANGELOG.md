# Changelog

本文件记录 tela-ui 的可见变更。格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### Added
- 组件库 (`@tela-ui/...`): 108 个导出组件 = 93 个 UI 组件 + 8 个服务型浮层 (对话框 / 加载 / 消息 / 通知 / 提示 / 图片预览 / 命令面板 / 动作面板, 全局调用 + 窗口根部挂一次 overlay) + 5 个库内基类 (TStrokedShape / TInputFrame / TModalSurface / TPopupSurface / TPopupStackLayer) + 2 个库内部件 (TNavLabel / TScrollThumb, 不在聚合出口)。
- 组件按交互类型分九组聚合: 基础 12 (按钮 / 图标 / 排版 / 分割线 / 加载 / 键位等), 输入 14 (文本 / 数字 / 滑块 / 评分 / 标签录入 / 提及 / 表单等), 选择 11 (下拉 / 复选 / 单选 / 开关 / 分段 / 树选 / 级联等), 日期时间 5 (日历 / 日期 / 时间 / 滚轮面板 / 日期时间), 展示 12 (标签 / 徽标 / 头像 / 时间线 / 统计 / 滚动容器 / 评论 / 倒计时等), 内容 16 (表格 / 列表 / 树 / 折叠 / 走马灯 / 分栏 / 拖拽排序等), 提示 4 (警告 / 通告栏 / 进度 / 结果页), 浮层 6 (对话框 / 抽屉 / 底部弹层 / 气泡 / 命令面板 / 图片预览), 导航 12 (选项卡 / 分页 / 导航栏 / 菜单 / 面包屑 / 步骤条 / 索引条等); 另有实验性 1 (TSkeleton); `@tela-ui/common.slint` 汇总全部导出。
- 设计令牌: `TTheme` global 151 个顶层转发, `TAppThemeConfig` 103 字段, 亮/暗两套 TDesign 色板; 模式偏好直接读写标准库 `Palette.color-scheme` (light / dark 强制, unknown 跟随系统且 tela 令牌默认暗色), tela 令牌与标准库组件同步切换。
- 浮层体系: `TPopupSurface` 锚定弹层 + `TPopupStack` 层计数 / ESC / z 序令牌。
- 无障碍: 可点击元素覆盖 `accessible-role` / `accessible-label`, 浮层文本带 `accessible-live-region`, 禁用态显式 `accessible-enabled` 且动作回调自带守卫。
- gallery 程序 (bin `tela`): 11 个页面 (10 分类 + 设置) 按 `if` 懒挂载, 页面切换各自记忆滚动位置, 亮暗经 `TELA_THEME` 选择, 设置页含标准库 `AboutSlint` 徽标。
- 自证工具链: `docs/harness.md` 的 `--check` 全量扫描 / 亮暗渲染对照 / 代码样本门禁 (`scripts/check-code-samples.py`) 配方; 83 个 gallery 代码样本均为可独立编译的自包含片段。
- 新增 TSticky (内容) 与 TAffix / TAnchor / TStickyTool (导航) 组件, 支持吸顶吸底、锚点导航与固定侧边工具栏; 已从分类聚合文件及 `@tela-ui/common.slint` 导出。
- gallery 新增对应演示页与代码样本, 并补充 TSticky / TAffix / TAnchor / TStickyTool 的使用限制说明。

### Changed
- TScrollBar / TAnchor / TBackTop 的滚动缓动改为组件内 16ms 定时器驱动的 glide, 缓动期内覆盖外部写入。
- TScrollBar 拖拽保留滑块抓取偏移, 轨道点击先将滑块居中再滚动。