# Changelog

本文件记录 tela-ui 的可见变更。格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### Fixed
- gallery 全部 83 个代码样本修复为可独立编译的自包含片段 — 原 72 个不可编译: 枚举/全局未 import (TThemeRole / TSize / TIconSet / TTheme / TMessage 等), `…` 省略号当数组占位与回调体留 `{ … }`, 引用页面私有状态 (root.input-text / 中文标识符 状态文本 / 未声明的 nav、v、page 等), 全局服务调用 (TMessage.success 等) 裸写在组件顶层; 现统一为完整 import + 完整模型数组 + 演示状态以 property 声明 + 服务调用包进按钮回调, TBorderlessWindow 样本改教 `export component MainWindow inherits TBorderlessWindow` 形式 (Window 作子元素放置会被编译器警告); 沉淀 `scripts/check-code-samples.py` 样本门禁 (每个 code: 字符串包成独立组件逐个 --check, 样本字符串不参与日常编译, 必须单独把关), 配方同步 harness.md 第 4 条。
- 第三批修复轮 (用户验收反馈): TTabBar 标签内容改为固定几何定位 — 原内层布局读自身 preferred 尺寸, 在重页面首帧整个不渲染 (图标消失), 现图标/文字/角标按固定 y 槽位排布; TCell 右侧 note/箭头垂直居中 (`width: self.preferred-width` 钉宽后显式 y 生效, 原写法顶对齐); TFab 子操作行 `alignment: end` (无标签行的圆钮不再左贴, 与上方按钮成一列), 标签胶囊经拉伸包装盒垂直居中; TSortable 演示钉死固定高度 (模型长度绑定的首选高度在深页首帧饿死, 行渲染不完整, slint-semantics 第 33 条)。
- gallery 内容页移除第二个 TTable 演示实例 (空态演示, 文档保留) — 页面实例化链恰好被它推过运行时 10 轮收尾预算, 触发 "long callback/instantiation chain" 警告; 移除后 0 警告。
- TNoticeBar / TComment 演示反馈改为组件内纯文本状态行, 不再弹 TMessage。
- TSortable 恢复让位行的 `animate y` (拖拽行关断, 其余行 duration-fast 缓动到新槽位); TTabBar 演示改为居中手机宽度 (520px) — 整宽演示条的右端落在页面级回到顶部悬浮钮热区里被挡住, 居中后任何滚动位置都不相交; TTabBar 演示反馈同步改纯文本状态行。
- TTabBar 演示包装盒塌缩修复 (720px 高窗口报障) — 无显式高度的 `Rectangle { width: 520px }` 靠固定高子元素撑 preferred, 重页实例化预算外饿死成 8px, 56px 标签栏被 clip 成 3px 缝贴在卡片标题下且不自愈 (slint-semantics 第 33 条第四实例); 改为包装盒显式 `height: bar.height + 2px` (2px 是描边厚度), bar 内缩 1px 避免盖住描边。

### Added
- 第三批中频组件 8 个: `TComment` / `TCountDown` (display), `TCell` + `TCellGroup` / `TSortable` (content), `TNoticeBar` (notice), `TTabBar` / `TIndexes` / `TFab` (navigation)。
- `TComment`: 评论条目, TAvatar 头像 + 作者/时间/正文 + 点赞切换与回复链接; 点赞先自翻转 `liked` 再回调 `like-changed`; 纯展示, 回复列表由宿主嵌套。
- `TCountDown`: 倒计时, `remaining` (duration) 双向绑定 + `running`, 每秒递减并 `changed`, 归零自停并 `finished`; 无目标时刻解析, 时位不按 24 回绕 (25h 显示 25:00:00)。
- `TNoticeBar`: 通告栏, 主题色浅底 + 前置图标 + 单行 elide; 多条 `contents` 整条垂直轮播 (TCarousel 镜像动画法, `changed` 只在真切换时触发); `closable` + 公开函数 `close()` (关闭走 visible, TAlert 同款)。
- `TCell` / `TCellGroup`: 设置页单元行 (图标/标题/说明/右侧 note/箭头, 行高 48/70px) 与带标题的分组卡片容器; 行分隔线自绘 (组内末行 divider: false); 点击只上报 `clicked`。
- `TSortable`: 拖拽排序列表 — 拖拽行淡出并以带阴影浮层跟随指针, 其余行实时让位预览插入槽; 松手上报 `reorder(from, to)`, 行序由宿主重建模型 (1.18 无循环语句); 拖拽状态机为 public function (`begin-drag`/`drag-to`/`end-drag`), 探针走同一代码路径。
- `TTabBar`: 底部标签栏 (图标 + 文案 + TBadge 角标), 等宽拉伸, 选中品牌色, 顶部分隔线内建; `changed` 上报, 与 TTabs 语义区分, 贴底定位由宿主负责。
- `TIndexes`: 字母索引条, 点击与拖拽统一走 `pick(y)` 命中夹紧 (`changed` 上报), 按住期间气泡回显当前字母; 分组列表滚动由宿主执行。
- `TFab`: 浮动操作按钮 — 无 `actions` 即悬浮圆钮; 有则主钮切换向上展开的子操作面板 (clip 高度动画, TTree 折叠盒同款), 主钮图标切 X, 子项点击先收起再 `action-clicked(i)`; 公开函数 `toggle-open()`; 面板随宿主绝对定位 (TBackTop 同款), 非弹出层。
- `TNoticeBar` / `TComment` / `TCell` 修复过程中沉淀 slint-semantics 第 32 条: 全定宽子元素的嵌套行会把拉伸列塌缩到首选宽, 行尾空 `Rectangle` 吸收位是修法。
- `TAvatarGroup`: 头像组, 成员按声明顺序叠放 (重叠 spacing-4), `max-count` 截断并渲染 `+N` 尾像; 成员 `clicked(index)`, 尾像 `overflow-clicked()`; `TAvatarGroupItem` 承载 text/source。
- `TRangeInput`: 数值区间输入, 双 decimal 格共享状态边框 (`~` 分隔); 沿用 TInputNumber 缓冲语义 (空缓冲不触发 changed, 失焦重排), 一侧变更夹紧另一侧合并发一次 `changed(min, max)`, Up/Down 步进。
- `TTagInput`: 标签录入, 模型渲染为可关闭 TTag + 行尾输入格; 回车添加、空缓冲退格删末位、标签 x 删除; 增删只上报 (`tag-added` / `tag-removed(i, tag)`), 模型由宿主改写。
- `TList`: 通用虚拟滚动列表, ListView 行回收 (TTable 同款); 行结构为图标/标题/可选说明, `selectable` + `current-index` 选中高亮, `row-clicked` / `changed` 回调, 空态 TEmpty。
- `TSelectInput`: 可输入筛选的下拉选择, TSelect 语义 + TAutoComplete 过滤机 (行内塌陷不重建模型); 聚焦列全部, 键入子串过滤, 选中带 ✓ 并回填 `text`, 再次键入不清空已选。
- `TTreeSelect`: 树选择下拉, TSelect 触发器 + 弹层内 TTree (`@children` 承载子节点); 选中值走 `TTreeLogic` 全局, 面板打开期间叶子选中即上报 `selected` 并关闭; 弹层高度固定 `panel-height` (默认 8 行, PopupWindow 读不到树的首选高度, 取内容会塌成 0), 并提供 `show()` / `close()` / `toggle()`。
- `TCascader`: 级联选择, 嵌套声明式 (TTree 同构); 单列逐级原位展开 (clip 折叠动画), 叶子选中写 `TCascaderLogic.current-value` 并上报 `selected`; 触发器回显叶子 label (完整路径回显暂不支持); 弹层高度固定 `panel-height` (同 TTreeSelect), 提供 `show()` / `close()` / `toggle()`。
- `TMention`: 提及输入, TInput + 宿主驱动的建议面板; `@` 解析与文本拼接在宿主 (1.18 无字符串切片), 组件上报 `changed`/`selected` 并提供 `show-suggestions()` / `close-suggestions()`; 面板锚输入框不锚光标。
- `TPicker`: 通用滚轮多列选择面板 (纯面板, 无触发器); 每列整列渲染 + 动画偏移 + 行内选中框 (TTimePicker 方案), `value: [int]` 逐列存下标并回写 (请与列数等长初始化; 短模型只有恰好补到末位的拾取能 push 追加), `set-value(col, idx)` 编程写入。
- `TDateTimePicker`: 日期时间选择, 只读触发器 + 年/月/日/时/分五列滚轮; 日列随年月收放 (TCalendar 同款闰月运算), `TDateTime { date, hour, minute }` 承载选中值, `hour < 0` 表示未选时间。
- `TOtpInput`: 分段验证码输入 (降级形态), 自动跳格 + 空段退格回退 + 全满 `completed()`; 已填段吞掉后续可打印键 (换字符先退格清除, 方向键/编辑键透传), 每段缓冲全文上报 `changed(i, text)`, 段内溢出与粘贴分发由宿主改写 `values` (Slint 1.18 无 max-length 与字符串切片); 初始聚焦经段自身 init 兜底 (仅靠 root init 的 `changed seq` 会在子段创建前跑完而不聚焦, 探针实测)。
- `TScrollBar`: 重写为自带视口与滚动进度的滚动容器 (替代 std ScrollView) — `@children` 进内建 Flickable 视口, 滚轮 / 拖拽 / 点击轨道驱动同一 `content-y`, 滑块始终是页面位置的精确映射; 6px 胶囊滑块静止 gray-7 / 悬停 gray-6 / 按压 gray-5, 拖拽即时跟手 (滑块中心对齐指针), 点击轨道 duration-base 缓动定位; `content-height` 默认取子元素首选高 (可钉住), `content-y` 双向暴露 (TBackTop 直绑 / 编程滚动), `thumb-inset` 避开窗口原生拉伸热区, `enabled` 关滑块交互。仅垂直滚动、非虚拟化内容; TList / TTable 保留 ListView 行回收并改接内部 `TScrollThumb` 叠加条 (scroll-thumb.slint, 不在公共出口)。库内全部滚动面已迁移, 展示页介绍卡同步新用法。
- gallery: 页面切换各自记忆滚动位置 (切走时存 `content-y`, 切回时恢复), 每页独立。
- `TAvatar`: 短暂加过的 1px 边框移除, 回到 TDesign 无边框样式。
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
- 组件库: 108 个导出组件 = 93 个 UI 组件 + 8 个服务 overlay (TDialogService / TLoadingService / TMessage / TNotification / TTooltip / TImageViewerService / TActionSheetService / TCommandPaletteService) + 5 个库内基类 (TStrokedShape / TInputFrame / TModalSurface / TPopupSurface / TPopupStackLayer) + 2 个库内部件 (私有 TNavLabel / 内部 TScrollThumb, 不在聚合出口)。

### Changed
- 滚动条: 库内全部滚动面 (TList / TTable / 下拉弹层 / 菜单 / 命令面板) 关闭 fluent 原生滚动条, 改用内部 `TScrollBar` 覆盖条 — 原生滑块 2px 贴边不居中且对比不足; 新条为 6px 灰阶胶囊滑块, 全宽居中, 支持拖拽与点击定位, 滚轮穿透到底层滚动区; 静止 gray-7 最深, 悬停 gray-6, 按压 gray-5 (交互时逐级变浅, 内容从条下透出)。gallery 自身的页面滚动与 TBackTop 演示同款接线。
- TTree / TCascader 分支折叠动画只在本分支开合时播放 (states + `in { animate }`): 展开期间高度即时跟随内容, 嵌套子分支展开不再被逐层父级动画追赶而呈阶梯式串行。
- TOtpInput: 首次渲染不再抢焦点 (点击段聚焦), 新增 `focus-segment(index)`; 聚焦移动时光标自动落段末, 自动跳格后可直接 Backspace 清段 (探针实测: 修复前光标停在 0 位, 退格删不到字符)。
- TDropdownMenu / TContextMenu: 分隔线行高收窄为 5px (细线上下各 spacing-1 间距), 条目行仍为 item-height; 弹层高度改为各行 `init` 自报累加, 且在每次 show/toggle/open-at 先清零 (弹层行每次打开都重建, 不清零会翻倍把弹层顶出窗口), 上限仍为 max-visible-items 个整行。
- **BREAKING**: 分类聚合器按交互类型拆分 — `data-entry.slint` → `input.slint` / `select.slint` / `date-time.slint`, `data-display.slint` → `display.slint` / `content.slint`, `feedback.slint` → `notice.slint` / `overlay.slint`; 组件文件移入对应目录, 旧聚合器删除, 导入路径改为 `@tela-ui/input.slint` 等; `@tela-ui/common.slint` 仍汇总全部导出。
