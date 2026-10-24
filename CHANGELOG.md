# Changelog

本文件记录 tela-ui 的可见变更。格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### Added
- 组件库: 84 个导出组件 = 71 个 UI 组件 + 8 个服务 overlay (TDialogService / TLoadingService / TMessage / TNotification / TTooltip / TImageViewerService / TActionSheetService / TCommandPaletteService) + 5 个库内基类 (TStrokedShape / TInputFrame / TModalSurface / TPopupSurface / TPopupStackLayer)。
- `TTable`: 列定义 + 行模型, 表头排序回调 (`sort-changed`), 行点击回调, ListView 虚拟滚动与空态; 列宽超出时水平滚动且表头同步; 排序只上报, 行重排由宿主完成。
- `TCalendar` / `TDatePicker`: 月历面板 (闰年/大小月/首日星期在 Slint 内计算), 单选与区间选择, 只读触发器 + 锚定弹层; 区间底色为连通色带, 起点/终点按 50% 圆角收口 (与日期圆圈一致); `TDate` 结构承载年/月/日。
- `TTimePicker`: 时间选择, 只读触发器 + 锚定时/分(秒)面板; 整列数据带动画滚动, 选中框画在选中行上随数据移动, 点击或滚轮逐格改值, 无确认按钮 (点击外部 / ESC 关闭); `TTime` 结构承载时/分/秒 (`hour < 0` = 未选); `TIconSet` 新增 `Clock`。
- `TAutoComplete`: 输入联想, TInput + 锚定建议列表; 按标签做不区分大小写的子串过滤 (行内塌陷, 不重建模型), 聚焦或键入展开, 高亮首条可选匹配, 回车/点击选中, 无匹配显示 `empty-text`; `TAutoCompleteItem` 承载 label/value/disabled。
- `TInput`: 新增公开函数 `move-caret-to-end()`, 宿主写入 `text` 后可把光标放到末尾; 新增框内文字装饰 `prefix-text` / `suffix-text`, 可点击后缀渲染成贴右缘的内嵌芯片 (次级底色 + 左侧 1px 细线与正文分隔, TInputNumber 步进器同款; 文本行整体缩进边框内缘绘制, 芯片贴住边框且边框保持完整; 禁用态守卫, 读屏走 button 角色; 芯片在行内占位, 正文不会钻到芯片底下), 以及 `always-clearable` (有文本即常显清空钮, 不等悬停)。
- `TSearch`: 搜索框, TInput 的搜索预设 (内置搜索图标 + 默认可清空), 右缘 x 图标即清空/取消 (经 `always-clearable` 在有文本时常显, 不等悬停), 回车触发 `accepted`。
- `TInputAdornment`: 输入装饰, `addon-before` / `addon-after` 附着段与输入框共用一套状态边框 (内层 TInput 去边框, 外层统一画), 分段间 1px 细线分隔; 附着段 stretch 0 贴合文本宽度, 剩余宽度全归输入框 (内层布局不写 `width: parent.width`, 否则盒子首选尺寸成环被压没)。
- `TForm` / `TFormItem`: 轻量表单脚手架, 标签列 (可固定宽 / 必填星标) + 控件行 + help/error 行 (error 优先且染红, 固定标签列时对齐控件列); 无校验引擎, 校验由 Rust 侧回调后写入状态。
- `TPopover`: 点击触发的富内容气泡 (`title` + `text`), 复用 TPopupSurface 锚定 (bottom-start 等四角), 点击外部 / ESC 关闭; 与悬停型 TTooltipArea 区分。
- `TContextMenu`: 右键菜单, 菜单行样式与 TDropdownMenu 一致 (图标/禁用/危险项/分隔线), 右键按下在指针位置弹出 (TPopupSurface 新增 `popup-x` / `popup-y` 显式定位), 点击外部 / ESC / 选中后关闭; 交互式触发器可关 `auto-area` 自行调 `open-at()`。
- `TSplitter`: 可拖拽分栏 (折叠后窗格内容以 `visible: false` 整体隐藏, 不留残影), `TSplitter` + `TSplitterPane` + `TSplitterPanePrimary` (及垂直变体 `TSplitterVertical` + `TSplitterPanePrimaryVertical`): 主栏拖拽夹紧 `min-size` 且被布局钳在容器宽度内 (无显式 width, preferred + min 0, 超容时布局向 min 压缩; 拖拽基准取显示尺寸, 钳制后回拖无死区), 16px 拖拽条 + 同宽圆形折叠钮, 折叠报 `changed`, 尺寸在组件自身维护 (一窗多分栏互不干扰); 拖拽按窗口坐标取增量 (分隔条随拖动移动, 局部坐标不动)。
- `TTree`: 树形控件, 嵌套声明式结构 (参考 sleek-ui `UMenu`): `TTree` + `TTreeItem` + `TTreeBranch`, 分支用 `@children` 承载子节点, 深度不限且无需组件递归; 分支折叠靠 `clip` + 高度动画 (收起后子节点保留实例); 叶子点击写入 `TTreeLogic.current-value` (全局选中值, 与 sleek-ui `UMenuLogic` 同构); `TIconSet` 新增 `Folder` / `File`。
- `TCodeBlock`: 代码改为只读 TextInput, 可鼠标划选并 Ctrl+C 复制 (Slint 未暴露应用层剪贴板写入, 纯 Slint 做不了复制按钮); 外观不变。
- `common.slint`: 补齐 `TTable` / `TSkeleton` / `TCalendar` / `TDatePicker` / `TTimePicker` / `TAutoComplete` / `TTree` 导出, 与分类聚合器保持一致。
- 浮层体系: `TPopupSurface` 锚定弹层 + `TPopupStack` 层计数 / ESC / z 序令牌。
- 设计令牌: `TTheme` global 151 个顶层转发, `TAppThemeConfig` 103 字段, 亮/暗两套色板 ramp。
- 无障碍: 可点击元素覆盖 `accessible-role` / `accessible-label`, 浮层文本带 `accessible-live-region`, 禁用态显式 `accessible-enabled` 并在动作回调自带守卫。
- 自证工具链: `docs/harness.md` 的 `--check` 全量扫描与亮/暗渲染对照配方。
- gallery 程序 (bin `tela`): 11 个页面 (10 个分类页 + 设置页), 按 `if` 懒挂载, 亮/暗主题经 `TELA_THEME` 环境变量选择。

### Changed
- TDropdownMenu / TContextMenu: 分隔线行高收窄为 5px (细线上下各 spacing-1 间距), 条目行仍为 item-height; 弹层高度改为各行 `init` 自报累加, 且在每次 show/toggle/open-at 先清零 (弹层行每次打开都重建, 不清零会翻倍把弹层顶出窗口), 上限仍为 max-visible-items 个整行。
- **BREAKING**: 分类聚合器按交互类型拆分 — `data-entry.slint` → `input.slint` / `select.slint` / `date-time.slint`, `data-display.slint` → `display.slint` / `content.slint`, `feedback.slint` → `notice.slint` / `overlay.slint`; 组件文件移入对应目录, 旧聚合器删除, 导入路径改为 `@tela-ui/input.slint` 等; `@tela-ui/common.slint` 仍汇总全部导出。