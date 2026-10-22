# 组件文档

本文档列出 tela-ui 的全部组件, 说明每个组件的继承关系, 参数, 参数类型与回调。
组件按功能分为十类, 每类一个目录与聚合器: `base/` (基础), `input/` (输入),
`select/` (选择), `date-time/` (日期时间), `display/` (展示), `content/` (内容),
`notice/` (提示), `overlay/` (浮层), `navigation/` (导航), 另有 `experimental/`
(实验性); 分别由 `@tela-ui/base.slint`, `@tela-ui/input.slint`,
`@tela-ui/select.slint`, `@tela-ui/date-time.slint`, `@tela-ui/display.slint`,
`@tela-ui/content.slint`, `@tela-ui/notice.slint`, `@tela-ui/overlay.slint`,
`@tela-ui/navigation.slint`, `@tela-ui/experimental.slint` 聚合导出, 也可以直接从
组件文件导入 (如 `@tela-ui/base/button.slint`)。`@tela-ui/common.slint` 保留为
全量兼容出口, 汇总全部类别。主题令牌统一从 `@tela-ui/theme.slint` 导入。

文中类型均按 Slint 类型书写。未标注读写的参数为 `in` (只读输入), `in-out` 表示
可从外部读写, `out` 表示组件只写, 调用方只能读取。默认值一栏给出 Slint 默认绑定。

## 通用约定

- **语义色 `TThemeRole`**: `default, primary, success, warning, error`。`default` 渲染中性灰,
  其余映射到对应色阶。定义在 `structs/enums.slint`, 经 `@tela-ui/theme.slint` 再导出。
- **尺寸 `TSize`**: `small, medium, large`。控件高度, 字号与内边距随之一档缩放。
- **图标参数**: 所有图标入口统一为 `TIconData` 结构, 传 `TIconSet.Xxx` 的内置图标
  或自行构造的字形数据, 见 [TIconSet](#ticonset-图标数据)。
- **回调命名**: 状态变化用 `changed`, 主动点击用 `clicked`, 结果性动作按语义命名
  (`accepted`, `canceled`, `closed`, `selected` 等)。
- **枚举导入**: 共享枚举 `TThemeRole`, `TSize`, `TPopupPlacement` 从 `@tela-ui/theme.slint` 导入; 组件专属枚举
  (如 `TButtonVariant`, `TDrawerPlacement`) 从所属聚合器导入。同名类型不要经由两条
  聚合路径重复导入, 以免命名冲突。

## 继承与组合总览

Slint 组件为单继承 (`inherits`), 子类可直接使用基类的全部属性。库内的继承链:

| 组件 | 继承自 | 说明 |
| --- | --- | --- |
| TInput, TTextArea, TInputNumber | `TInputFrame` (私有) | 共享状态边框, 悬停/聚焦/禁用态与点击聚焦行为 |
| TDialog, TDrawer, TBottomSheet, TImageViewerOverlay, TCommandPaletteOverlay | `TModalSurface` (私有) | 共享遮罩, z-modal 层级, 淡入淡出, ESC 与背景点击关闭 |
| TActionSheetOverlay | `TBottomSheet` | 动作列表就是一张底部弹层 |
| TBorderlessWindow | `Window` | 在 Window 内建属性 (`title`, `no-frame` 等) 上叠加自绘标题栏 |
| TText | `Text` | 主题字体 + TDesign 正文样式 (14px/22px); 库内所有文本的基类 |
| THeading, TParagraph, TCaption | `TText` | 依次覆盖字号与行高, 保留 `text`, `color`, `wrap` 等全部内建属性 |
| TRadioGroup, TLoading | `HorizontalLayout` | 本身就是一行布局 |
| TTimeline, TStatistic, TCollapse | `VerticalLayout` | 本身即纵向布局, 首选尺寸随内容 |
| 其余全部组件 | `Rectangle` | 基础容器 |

组合关系 (has-a, 内部使用的组件):

- TButton = TIcon + TLoadingSpinner
- TLoading = TLoadingSpinner
- TDialog = TModalSurface (遮罩基类) + TButton x2 (确认/取消)
- TTable = ListView (行) + TEmpty (空态)
- TDatePicker = TInput (只读触发器) + TPopupSurface + TCalendar
- TTimePicker = TInput (只读触发器) + TPopupSurface + 私有 TTimeColumn x2/3 (整列时/分/秒数据 + 行内选中框)
- TActionSheetOverlay = TBottomSheet (底部弹层基类) + 自绘整行取消块 (`TButton` 无法横向撑满自身, 见 `docs/slint-semantics.md` 第 23 条)
- TImageViewerOverlay = TModalSurface + 私有 TViewerButton x6 (上一张/下一张/缩小/放大/适应窗口/关闭) + TViewerChip x4 (计数/缩放组/关闭/缩略图条) + 窗口宽的图片轨道
- TCommandPaletteOverlay = TModalSurface + TDivider + TIcon + ScrollView (列表)
- TNavBar, TTabs, TBreadcrumb = 私有 TNavLabel xN (文本行悬停/选中态与点击共享)
- TPopconfirm = TButton x2 (取消/确认)
- TRadioGroup = TRadio xN
- TSideNav = 私有 TNavRow xN (菜单行, 底部行与收起行共用)
- TTag, TCheckBox, TAlert, TMessageOverlay, TNotificationOverlay, TNavBar, TSideNav,
  TSelect, TPagination, TAvatar, TDropdownMenu, TRate, TEmpty, TResult,
  TCollapse, TTable, TBorderlessWindow 内部均使用 TIcon 渲染字形
- 提供 `@children` 插槽的组件: TBadge (被标记内容), TCard (卡片主体), TDialog
  (自定义主体), TDrawer (面板主体), TNavBar (右侧操作区), TEmpty (操作区),
  TTooltipArea (触发内容), TResult (操作区), TBorderlessWindow (窗口内容)

私有组件 (仅供实现参考, 不经聚合器导出): `TInputFrame` (input/input.slint,
已对库内导出供 TInputNumber 复用), `TModalSurface` (modal-surface.slint, 已对库内
导出供 TDialog/TDrawer/TBottomSheet/TImageViewerOverlay/TCommandPaletteOverlay 继承),
`TViewerButton` / `TViewerChip` (overlay/image-viewer.slint, 遮罩上的圆形图标按钮与控件卡片),
`TNavLabel` (navigation/nav-label.slint),
`TNavRow` (navigation/side-nav.slint),
`TTableCell` (content/table.slint, 表头与数据单元格的文本、对齐与排序图标),
`TTimeColumn` (date-time/time-picker.slint, 时间面板的单列数据; 选中框画在选中行上并随数据滚动),
`TWindowButton` (base/borderless-window.slint), `TPageBtn`
(navigation/pagination.slint), `TPopupSurface` (popup-surface.slint,
TDropdownMenu / TPopconfirm / TSelect 共用的锚定弹层骨架: 零尺寸锚定宿主 + placement
定位 + 弹层皮肤), `TPopupStackLayer` (popup-stack.slint, 覆盖层的入栈/深度重排/ESC
焦点壳, 由 TModalSurface 复用), `TStrokedShape` (base/stroked-shape.slint, 共享 24
网格描边模型: `size`, `stroke-width` 按 size 缩放或 `absolute-stroke-width` 取原始
像素, 以及只读 `computed-stroke-width`; TIcon 与 TLoadingSpinner 都继承它)。

---

## 基础组件 base

```slint
import { TText, TButton, TLink, TIcon, THeading, TParagraph, TCaption, TDivider, TLoading, TKbd, TBorderlessWindow } from "@tela-ui/base.slint";
```

### TButton

按钮。`theme x variant x size` 组合出全部样式; 方形变体可作图标按钮。

继承: `Rectangle`。内部组合 TIcon 与 TLoadingSpinner。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| theme | `TThemeRole` | `TThemeRole.default` | 语义色, default 为中性灰按钮 |
| variant | `TButtonVariant` | `TButtonVariant.base` | `base` 实底 / `outline` 描边 / `text` 纯文字 |
| size | `TSize` | `TSize.medium` | 高度 24/32/40px |
| shape | `TButtonShape` | `TButtonShape.rectangle` | `rectangle` / `square` 图标按钮 (宽=高) / `round` 胶囊 |
| text | `string` | `""` | 按钮文字, 空串不渲染文本 |
| icon | `TIconData` | 空 | 前置图标, loading 时被转圈替代 |
| loading | `bool` | `false` | 显示旋转指示并禁用点击 |
| disabled | `bool` | `false` | 禁用态 |

回调: `clicked()`。

相关枚举: `TButtonVariant { base, outline, text }`, `TButtonShape { rectangle, square, round }`。

### TLink

超链接文字。默认悬停时显示下划线, `underline` 为 true 时常显。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| theme | `TThemeRole` | `TThemeRole.default` | default 使用链接色令牌, 其余用对应色阶 |
| text | `string` | `""` | 链接文字 |
| icon | `TIconData` | 空 | 前置图标 |
| disabled | `bool` | `false` | 禁用态, 显示禁用灰并隐藏下划线 |
| underline | `bool` | `false` | 常显下划线 (默认仅悬停显示) |
| size | `TSize` | `TSize.medium` | 字号随 TSize 缩放 |

回调: `clicked()`。

### TIcon

通用图标渲染器。数据驱动, 与 lucide-slint 相同的几何组织方式。

继承: 私有 `TStrokedShape` (见"继承与组合总览"), 因此 `size` / `stroke-width` 为 `in`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| icon | `TIconData` | 空 | `TIconSet.Xxx` 或自定义字形 |
| icon-color | `brush` | `TTheme.text-color-primary` | 线条与填充颜色 |
| size | `length` (in) | `16px` | 渲染尺寸 |
| stroke-width | `float` (in) | `2.0` | 24 网格单位的线宽, 随 size 缩放 |
| absolute-stroke-width | `bool` | `false` | 为 true 时 stroke-width 按绝对像素 |

只读输出: `computed-stroke-width` (`length`, 实际线宽)。

### TText / THeading / TParagraph / TCaption

排版组件。`TText` 是库内文本的基类, 默认即 TDesign `body-medium`:
主题字体, 字号 14px, 行高 22px, 常规字重, 主文本色; 其余三者继承它, 只写各自不同的部分
(如 THeading 的 `level` 字号/行高, TParagraph 的 `secondary`, TCaption 的 12px/20px)。
均继承 `Text`, 保留 `text`, `color`, `wrap` 等全部内建属性。

| 组件 | 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| TText | line-height | `length` | `TTheme.line-height-m` | TDesign 行盒; 覆盖 `font-size` 时必须一并给出 (如 `TTheme.line-height-title`, 或 `TTheme.size-line-height(size)`) |
| TText | (其余) | — | — | 14px / 22px 正文样式, 库内所有文本的落点 |
| THeading | level | `int` | `3` | 1..5 对应 36/28/24/20/18px, 行高 44/36/32/24/24px, 半粗字重 |
| TParagraph | secondary | `bool` | `false` | true 时用次级文本灰 |
| TCaption | (无新增参数) | — | — | 12px / 20px 次级灰, 常用于辅助说明 |

`THeading`, `TParagraph`, `TCaption` 三者默认 `text` 为空串, 并各自设 `word-wrap` 自动
换行 (`TText` 基类不设 `text`/`wrap`, 单独使用时继承宿主 `Text` 的默认值)。

组件的其它文字一律用 `TText` (只有代码块保留原始 `Text` + 等宽字体): 未设
`font-family` 的 `Text` 会落到宿主 `Window.default-font-family`, 再落到渲染器通用族
(本机为 Arial), 与主题字体不一致。

行高: Slint 只提供"相对字体自然行盒"的 `line-height-factor`, 所以 `TText` 收下 TDesign
的行盒令牌, 再按自身 `font-size` 换算成系数 —— 覆盖字号时只需补 `line-height`:

```slint
TText { font-size: TTheme.font-size-title-m; line-height: TTheme.line-height-title; text: "标题"; }
// 尺寸枚举随 TSize 变时用成对的辅助函数
TText { font-size: TTheme.size-font(size); line-height: TTheme.size-line-height(size); text: root.text; }
```

### TDivider

分割线。水平带文字或纯线, 亦可作 1px 竖线。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| orientation | `Orientation` (Slint 内建) | `Orientation.horizontal` | `vertical` 时渲染为竖线 |
| text | `string` | `""` | 非空时渲染为带文字分割线 |
| align | `TDividerAlign` | `TDividerAlign.center` | 文字位置: `left` / `center` / `right` |
| line-color | `color` | `TTheme.border-level-1-color` | 线色 |
| text-color | `color` | `TTheme.text-color-secondary` | 文字色 |
| content-gap | `length` | `TTheme.margin-l` | 文字与线之间的间距 |
| inset | `length` | `0px` | 距容器边缘的留白 (仅水平方向) |

相关枚举: `TDividerAlign { left, center, right }`。

### TLoading / TLoadingSpinner

加载指示。`TLoading` 是带文字标签的完整组件, `TLoadingSpinner` 是裸转圈
(TButton 内部也使用它)。

继承: `TLoading` 继承 `HorizontalLayout` (本身就是一行布局), `TLoadingSpinner` 继承私有 `TStrokedShape` (见"继承
与组合总览")。TLoading 内部组合 TLoadingSpinner。

| 组件 | 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| TLoading | text | `string` | `""` | 空串时只显示转圈 |
| TLoading | theme | `TThemeRole` | `TThemeRole.primary` | 圈的颜色 |
| TLoading | size | `length` (in-out) | `24px` | 圈直径 |
| TLoadingSpinner | spinner-color | `brush` | `TTheme.brand-color` | 圈的颜色 |
| TLoadingSpinner | size | `length` (in) | `16px` | 圈直径 |
| TLoadingSpinner | stroke-width | `float` (in) | `2.0` | 24 网格单位线宽 |
| TLoadingSpinner | absolute-stroke-width | `bool` | `false` | 按绝对像素解释线宽 |

`TLoadingSpinner` 只读输出: `computed-stroke-width` (`length`, 实际线宽)。

### TBorderlessWindow

无边框应用窗口。自绘标题栏 (拖拽热区, 最小化/最大化/关闭), 圆角与边缘缩放。
用作应用窗口根, 子元素即窗口内容。

继承: `Window` (保留 `title`, `icon`, `minimized`, `maximized` 等内建属性)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| window-background | `brush` | `TTheme.bg-color-page` | 圆角窗口表面色; 不要设置 Window 自身的 `background`, 那会把圆角涂成不透明 |
| window-radius | `length` | `TTheme.radius-large` | 窗口圆角 (9px), 最大化时归零 |
| resize-border | `length` | `8px` | 边缘拖拽缩放热区宽度, 最大化时归零 |
| header-height | `length` | `TTheme.height-l` | 标题栏高度 (40px) |
| header-background | `brush` | `TTheme.bg-color-container` | 标题栏底色 |
| logo | `TIconData` | 空 | 标题前的品牌图标 |
| maximizable | `bool` | `true` | 为 false 时隐藏最大化按钮 (工具对话框等) |
| header-title-visible | `bool` | `true` | 隐藏标题文字 (任务栏 title 不受影响) |

内部组合: 私有 TWindowButton (最小化 / 最大化 / 关闭三个键共用 46px 命中区与悬停皮肤,
最大化键按 `maximized` 在 `TIconSet.Square` 与 `TIconSet.Copy` 之间换字形), TIcon 与
内建 `WindowMoveArea` (拖拽热区)。

---

### TKbd

键位标签 (如 `Ctrl + C` 中的按键帽)。小号文字 + 浅底细边圆角, 宽度随文字
收紧; 与 `Text` 的 "+" 混排组合成快捷键说明。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` | `""` | 单个键帽文字 |
| keys | `[string]` | `[]` | 组合键的各按键, 非空时优先于 `text`, 芯片之间自动插入 "+" |
| size | `TSize` | `TSize.medium` | 键帽尺寸, 全部由令牌驱动: 高 `TTheme.size-height(size)` (24/32/40px), 字号 `TTheme.size-font(size)` (12/14/16px), 行盒 `TTheme.size-line-height(size)`, 左右内边距 `TTheme.size-padding(size)` (8/12/16px) |

## 输入 input

```slint
import { TInput, TTextArea, TInputStatus, TInputNumber, TSlider, TRate, TAutoComplete, TAutoCompleteItem } from "@tela-ui/input.slint";
```

### TInput / TTextArea

输入框。共享私有基类 `TInputFrame`, 边框随状态着色, 点击边框聚焦内部 TextInput。

继承: `TInputFrame` (私有, 继承 `Rectangle`)。

TInput 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| size | `TSize` | `TSize.medium` | 高度与字号 |
| placeholder-text | `string` | `""` | 占位文字 |
| text | `string` (in-out) | `""` | 输入内容, 双向绑定到内部 TextInput |
| read-only | `bool` (in-out) | `false` | 只读 (可聚焦可复制, 不可编辑) |
| prefix-icon | `TIconData` | 空 | 前缀图标 |
| suffix-icon | `TIconData` | 空 | 后缀图标 (显示清空钮时被隐藏) |
| clearable | `bool` | `false` | 悬停且非空时显示清空钮 |
| input-type | `InputType` (Slint 内建) | `InputType.text` | `password` 等输入类型 |

TTextArea 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| placeholder-text | `string` | `""` | 占位文字 |
| text | `string` (in-out) | `""` | 输入内容, 自动换行 |
| read-only | `bool` (in-out) | `false` | 只读 (可聚焦可复制, 不可编辑) |
| rows | `int` | `4` | 期望可见行数, 高度随之计算 |

继承自 TInputFrame 的参数 (两者通用):

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| status | `TInputStatus` | `TInputStatus.default` | 边框色: `default` / `success` / `warning` / `error` |
| disabled | `bool` | `false` | 禁用态 |
| radius | `length` | `TTheme.radius-default` | 圆角 (TTextArea 固定为 `radius-medium`) |

回调: TInput `accepted(string)` (回车), `changed(string)` (每次输入), `cleared()`;
TTextArea `changed(string)`。

公开函数 (TInput): `move-caret-to-end()` — 把光标移到文本末尾并清除选区; 宿主写入
`text` 后调用, 否则下一次键入会落在旧光标位置。

只读输出 (来自 TInputFrame): `frame-hovered` (`bool`, 边框悬停态)。

相关枚举: `TInputStatus { default, success, warning, error }`。

### TInputNumber

数字输入框。左右减/加步进按钮夹住居中的文本框 (TDesign 行内布局); 键入实时
钳制到 [min, max] 并同步 value, 回车或失焦后按两位小数规范化显示; 初始值越界
时自动收敛到区间内; 文本框内可用 Up/Down 方向键步进; 到达边界时对应按钮
禁用。暴露 spinbox 无障碍语义 (值/上下限/步长与增减动作)。

继承: `TInputFrame` (私有)。内部组合 TIcon (Minus/Plus)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| value | `float` (in-out) | `0` | 当前值, 交互时组件内部自写, 请用双向绑定 (`value <=> v`) |
| min | `float` | `0` | 下限, 到达时减号禁用 |
| max | `float` | `100` | 上限, 到达时加号禁用 |
| step | `float` | `1` | 步进量 |
| editable | `bool` | `true` | 文本框是否允许键入 (InputType.decimal, 仅数字与小数点), false 时仅按钮步进 |
| size | `TSize` | `TSize.medium` | 高度与字号 |
| disabled | `bool` | `false` | 禁用态 (来自 TInputFrame) |

回调: `changed(float)` — 键入, 回车或步进后的新值。外部写入 value 时, 文本框
在未聚焦状态下自动同步显示。

### TAutoComplete

输入联想。输入框 + 锚定建议列表: 聚焦输入框或键入文字时展开, 按标签做不区分大小写的
子串过滤 (输入为空时列出全部); 高亮第一条可选匹配, 回车或点击选中, 选中后把
`text` 设为该项 `label` 并关闭; 过滤后没有匹配项时列表显示 `empty-text` (`items` 为空则不展开)。

继承: `Rectangle`。内部组合 TInput, TPopupSurface 与 ScrollView。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TAutoCompleteItem]` | `[]` | 建议项; 过滤在组件内完成, 无需宿主重建模型 |
| text | `string` (in-out) | `""` | 输入内容; 选中建议时被写为该项 `label` |
| placeholder-text | `string` | `""` | 占位文字 |
| disabled | `bool` | `false` | 禁用输入框, 并关闭已展开的列表 |
| clearable | `bool` | `false` | 悬停且非空时显示清空钮 |
| size | `TSize` | `TSize.medium` | 输入框高度与字号 |
| max-rows | `int` | `6` | 列表最多显示行数, 超出后滚动 (至少按 1 行计算) |
| empty-text | `string` | `"无匹配结果"` | 无匹配时的提示文字 |

只读输出: `is-open` (`bool`) — 建议列表是否展开。

回调: `changed(string)` — 键入或清空后的当前文字; `selected(int, string)` — 点击或回车
选中建议, 参数为 (下标, value); `accepted(string)` — 无高亮建议时回车, 参数为输入文字。

相关结构: `TAutoCompleteItem { label: string, value: string, disabled: bool }`;
过滤只匹配 `label`, `disabled` 项可见但不可选、也不参与回车高亮。

已知限制: 不支持上下方向键步进 (1.18 语言内没有循环语句, 组件无法扫描"下一条匹配");
高亮停在第一条可选匹配或指针悬停项。列表随过滤结果伸缩, 最多 `max-rows` 行。

### TSlider

滑块。拖动与键盘 (方向键步进, Home/End 到端点) 均可调值。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| minimum | `float` | `0` | 最小值 |
| maximum | `float` | `100` | 最大值 |
| step | `float` | `1` | 步长 |
| disabled | `bool` | `false` | 禁用态 |
| value | `float` (in-out) | `minimum` | 当前值, 始终落在步长刻度上 |

回调: `changed(float)` (拖动中持续), `released(float)` (松开或按键释放)。

公开函数: `set-value(float)` — 钳制到区间并按 step 取整后写入。

### TRate

星级评分。整数步进; 悬停预览目标分, 点击当前分清零。

继承: `Rectangle`。内部组合 TIcon (StarFill 填充星)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| max | `int` | `5` | 星星总数 |
| value | `float` (in-out) | `0` | 当前分, 交互时组件内部自写, 请用双向绑定 |
| read-only | `bool` | `false` | 只读 (正常配色, 不可交互) |
| disabled | `bool` | `false` | 禁用 (整体灰化) |
| allow-clear | `bool` | `true` | 点击当前分值清零 |
| theme | `TThemeRole` | `TThemeRole.warning` | 选中星颜色 |
| size | `TSize` | `TSize.medium` | 星星 16/20/24px |

回调: `changed(float)` — 点击后的新分值。

## 选择 select

```slint
import { TSelect, TSelectItem, TCheckBox, TRadio, TRadioGroup, TRadioItem, TSwitch, TSegmented } from "@tela-ui/select.slint";
```

### TSelect

下拉选择。弹层锚定在触发器下方, 宽度随触发器 (至少 120px), 最多显示 6 项;
暂无键盘导航。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TSelectItem]` | `[]` | 选项列表 |
| current-index | `int` (in-out) | `-1` | 选中项下标, -1 为未选择 |
| current-value | `string` (in-out) | 由 current-index 派生 | 选中项 value, 与 current-index 永不失同步; 建议宿主只绑 current-index |
| placeholder-text | `string` | `""` | 未选择时的占位文字 |
| disabled | `bool` | `false` | 禁用态 |
| prefix-icon | `TIconData` | 空 | 前缀图标 |
| size | `TSize` | `TSize.medium` | 高度与字号 |

回调: `selected(int, string)` — 参数为 (下标, value)。

相关结构: `TSelectItem { label: string, value: string, disabled: bool }`。

### TCheckBox

复选框, 支持半选。

继承: `Rectangle`。内部组合 TIcon (对勾/横线)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` | `""` | 标签文字 |
| checked | `bool` (in-out) | `false` | 选中态, 点击时组件自行翻转 |
| indeterminate | `bool` | `false` | 半选样式, 显示上优先于 checked |
| disabled | `bool` | `false` | 禁用态 |

回调: `changed(bool)` — 翻转后的选中值。

### TRadio / TRadioGroup

单选。`TRadio` 是单个圆钮, `TRadioGroup` 将一组选项排成一行并持有选中值。

继承: `TRadio` 为 `Rectangle`; `TRadioGroup` 为 `HorizontalLayout`。TRadioGroup 内部
组合 TRadio。

TRadio 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` | `""` | 标签文字 |
| selected | `bool` | `false` | 选中态, 选中逻辑由调用方在 `clicked` 中处理 |
| disabled | `bool` | `false` | 禁用态 |

TRadioGroup 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TRadioItem]` | `[]` | 选项列表 |
| value | `string` (in-out) | `""` | 选中的 value |
| disabled | `bool` | `false` | 整组禁用 |

回调: TRadio `clicked()`; TRadioGroup `changed(string)`。

相关结构: `TRadioItem { text: string, value: string }`。

注意: TRadioGroup 点击时会自赋值 `value`, 这会永久断开外部对 `value` 的绑定。
选中态需要持续受外部数据源控制时 (例如绑定主题偏好), 改用独立 `TRadio` 并自绑
`selected` 与 `clicked`。

### TSwitch

开关。small 36x18 / medium 40x22 / large 48x24。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| checked | `bool` (in-out) | `false` | 开关态, 点击时组件自行翻转 |
| disabled | `bool` | `false` | 禁用态 |
| size | `TSize` | `TSize.medium` | 控件尺寸 |

回调: `changed(bool)` — 翻转后的开关值。

### TSegmented

分段控制器 (iOS 风格)。填满容器宽度, 各段等分, 选中段白底加投影;
段文字单行居中。

继承: `Rectangle` (内部为等分布局)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[string]` | `[]` | 选项文本 |
| current-index | `int` (in-out) | `0` | 选中下标 |
| size | `TSize` | `TSize.medium` | 高度与字号 |
| disabled | `bool` | `false` | 禁用态 |

回调: `changed(int)` — 选中下标变化时触发。

## 日期时间 date-time

```slint
import { TCalendar, TDate, TDatePicker, TTimePicker, TTime } from "@tela-ui/date-time.slint";
```

### TCalendar

日历面板。表头月份导航 + 星期行 + 6x7 日期网格; 相邻月份日期填满网格并灰显,
支持单选与区间。月历运算 (闰年, 大小月, 首日星期) 在 Slint 内完成;
`today` 与初始显示月份由宿主传入 (Slint 不暴露当前日期)。星期从周日开始。

继承: `Rectangle` (内部 42 个日期单元, 无私有组件)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| year | `int` (in-out) | `2026` | 显示年份; 上下月切换时组件自写 |
| month | `int` (in-out) | `1` | 显示月份 1..12; 上下月切换时组件自写 |
| value | `TDate` (in-out) | `{ year: 0, month: 0, day: 0 }` | 单选结果; `year == 0` 表示未选 |
| range | `bool` | `false` | 区间模式 |
| range-start / range-end | `TDate` (in-out) | `{ year: 0, month: 0, day: 0 }` | 区间端点, 两端都有效时区间完整 |
| today | `TDate` | `{ year: 0, month: 0, day: 0 }` | 高亮日期, 由宿主传入 |
| cell-size | `length` | `TTheme.spacing-12` (40px) | 日期格边长 |

回调: `selected(TDate)` — 每次点击有效日期都触发, 即使该日期已选中;
`changed(TDate)` — 选中结果真正变化后触发 (单选重复点击同一天不触发);
`range-changed(TDate, TDate)` — 区间两个端点都确定后触发, 起点晚于终点时自动交换;
`month-changed(int, int)` — 上下月切换完成后触发新 `(year, month)`。

公开函数: `show-date(TDate)` — 把面板切到该日期所在月。

相关结构: `TDate { year: int, month: int, day: int }`, 全 0 表示未选。

注意: 区间完成后再次点击会以该日为新区间起点; 相邻月份的灰色日期同样可选。
`year` / `month` 组件会写入, 宿主需要持续同步时用 `<=>`。暂无键盘导航。

### TDatePicker

日期选择器。只读触发器 + 锚定 `TCalendar`; 点击展开, 单选后关闭, 区间选完关闭。

继承: `Rectangle`。内部组合 TInput, TPopupSurface 与 TCalendar。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| size | `TSize` | `TSize.medium` | 触发器高度与字号 |
| placement | `TPopupPlacement` | `TPopupPlacement.bottom-start` | 弹层方向; 靠近窗口底部时改用 `top-start` |
| placeholder-text | `string` | `"请选择日期"` | 未选时占位文字 |
| disabled | `bool` | `false` | 禁用触发器 |
| range | `bool` | `false` | 区间模式 |
| value | `TDate` (in-out) | `{ year: 0, month: 0, day: 0 }` | 单选结果 |
| range-start / range-end | `TDate` (in-out) | `{ year: 0, month: 0, day: 0 }` | 区间端点 |
| today | `TDate` | `{ year: 0, month: 0, day: 0 }` | 日历高亮日期 |

只读输出: `is-open` (`bool`) — 弹层是否展开。

回调: `changed(TDate)` — 选中结果变化后; `range-changed(TDate, TDate)` — 区间完成时。

公开函数: `show()`, `close()`, `toggle()`。

已知限制: 触发器只读, 不支持手工输入与字符串解析; `today` 由宿主传入;
弹层高度约 338px, 靠近窗口底部时请设置 `placement: TPopupPlacement.top-start`。

### TTimePicker

时间选择器。只读触发器 + 锚定时间面板; 面板为时/分(秒)三列, 选中框画在选中行上,
整列数据带动画滚动并把选中行送到中间。点击行或滚轮逐格改值; 没有确认按钮,
点击面板外、按 ESC 或再点触发器关闭。

继承: `Rectangle`。内部组合 TInput, TPopupSurface 与私有 TTimeColumn。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| size | `TSize` | `TSize.medium` | 触发器高度与字号 |
| placement | `TPopupPlacement` | `TPopupPlacement.bottom-start` | 弹层方向; 靠近窗口底部时改用 `top-start` |
| placeholder-text | `string` | `"请选择时间"` | 未选时占位文字 |
| disabled | `bool` | `false` | 禁用触发器, 并关闭已展开的面板 |
| show-seconds | `bool` | `false` | 增加秒列, 触发器显示 `HH:mm:ss` |
| value | `TTime` (in-out) | `{ hour: -1, minute: 0, second: 0 }` | 选中时间; `hour < 0` 表示未选 |

只读输出: `is-open` (`bool`) — 弹层是否展开。

回调: `changed(TTime)` — 点击行或滚轮改值后, `value` 已更新。

公开函数: `show()`, `close()`, `toggle()`。

相关结构: `TTime { hour: int, minute: int, second: int }`, 24 小时制;
`hour < 0` 表示未选, 其余字段按 0..59 取值。

已知限制: 触发器只读; 暂不支持键盘导航与手工输入; 滚轮直接修改 `value`
(每次一格并触发 `changed`), 没有惯性滚动。弹层高约 234px, 靠近窗口底部时请设置
`placement: TPopupPlacement.top-start`。

## 展示 display

```slint
import { TTag, TTagVariant, TAvatar, TAvatarSize, TAvatarShape, TBadge, TCard, TEmpty, TDescriptions, TDescriptionItem, TTimeline, TTimelineItem, TStatistic } from "@tela-ui/display.slint";
```

### TTag

小标签。三种填充变体, 可带图标与关闭钮。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| theme | `TThemeRole` | `TThemeRole.default` | 语义色 |
| variant | `TTagVariant` | `TTagVariant.dark` | `dark` 实底 / `light` 浅色底 / `outline` 描边 |
| size | `TSize` | `TSize.medium` | 高度 20/24/28px |
| text | `string` | `""` | 标签文字 |
| icon | `TIconData` | 空 | 前置图标 |
| closable | `bool` | `false` | 显示关闭钮; 点击后触发 `closed()` 并自设 `visible = false` |
| disabled | `bool` | `false` | 禁用态 |
| clickable | `bool` | `false` | 启用悬停高亮与 `clicked()` |

回调: `closed()`, `clicked()`。

相关枚举: `TTagVariant { dark, light, outline }`。

### TAvatar

头像。图片, 文字或默认人形图标的圆形/圆角方形占位。

继承: `Rectangle`。内部组合 TIcon (默认人形)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| size | `TAvatarSize` | `TAvatarSize.medium` | `small` 32 / `medium` 40 / `large` 48 / `extra-large` 64px |
| text | `string` | `""` | 无图时显示的文字, 字号按直径自适应 |
| source | `image` | 空 | 头像图片, 存在时优先显示 (cover 裁剪) |
| shape | `TAvatarShape` | `TAvatarShape.circle` | `circle` / `square` |
| theme | `TThemeRole` | `TThemeRole.primary` | 文字/默认图标的颜色与浅色底 |

显示优先级: `source` > `text` > 默认图标。

相关枚举: `TAvatarSize { small, medium, large, extra-large }`,
`TAvatarShape { circle, square }`。

### TBadge

徽标。包裹在默认子元素右上角的计数胶囊或红点。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| count | `int` | `0` | 计数 |
| dot | `bool` | `false` | 红点模式, 不显示数字 |
| max-count | `int` | `99` | 超出显示 `{max-count}+` |
| badge-color | `color` | `TTheme.error-color` | 徽标底色 |
| show-zero | `bool` | `false` | count 为 0 时仍然显示 |

插槽: 默认子元素 (`@children`) 为被标记的内容。

### TCard

卡片容器。可选标题/副标题头部与带分隔线的页脚。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| title | `string` | `""` | 标题 |
| subtitle | `string` | `""` | 副标题 |
| footer | `string` | `""` | 页脚文字, 上方带 1px 分隔线 |
| bordered | `bool` | `true` | 是否描边 |
| shadowed | `bool` | `false` | 是否投影 (shadow-1) |
| card-padding | `length` | `TTheme.margin-l` | 内边距 |

插槽: 默认子元素为卡片主体。

### TEmpty

空状态占位: 大图标 + 描述文字 + 操作区, 整体在容器内居中。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| icon | `TIconData` | `TIconSet.Inbox` | 占位图标 |
| description | `string` | `"暂无数据"` | 为空字符串时隐藏 |
| icon-size | `length` (in-out) | `48px` | 图标边长 |

插槽: 默认子元素为描述下方的操作区, 在一行内居中排布, 通常放按钮。

### TDescriptions

键值描述列表。`columns` 等宽分列; `bordered` 为网格样式 (标签底色 + 细边框),
否则为无框样式。单元格单行省略。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TDescriptionItem]` | `[]` | 条目 |
| bordered | `bool` | `false` | 网格样式 |
| columns | `int` | `2` | 列数, 最小 1 |

相关结构: `TDescriptionItem { label: string, value: string }`。

### TTimeline

垂直时间线。条目按 `TThemeRole` 语义色着色, 末项空心圆点, 连线自动贯穿条目间距;
描述为空时隐藏。

继承: `VerticalLayout` (本身即纵向布局)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TTimelineItem]` | `[]` | 条目 |

相关结构: `TTimelineItem { label: string, content: string, theme: TThemeRole }`
(theme 缺省为 `TThemeRole.default`)。

### TStatistic

数值统计块: 标题 + 大号数值 + 可选前后缀, 数值按 `TThemeRole` 语义色着色;
`loading` 时数值位显示转圈。

继承: `VerticalLayout`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| title | `string` | `""` | 标题, 为空隐藏 |
| value | `string` | `""` | 数值文本 (由调用方格式化) |
| prefix / suffix | `string` | `""` | 数值前后缀, 为空隐藏 |
| theme | `TThemeRole` | `TThemeRole.default` | 数值颜色 (default → 主文本色) |
| loading | `bool` | `false` | 加载态 |

## 内容 content

```slint
import { TCollapse, TCollapseItem, TCarousel, TCarouselSlide, TCodeBlock, TTable, TTableColumn, TTableRow, TTableAlign, TTableSortDirection } from "@tela-ui/content.slint";
```

### TCollapse

折叠面板。`accordion: true` 为手风琴模式 (单开, `current-index` 记录展开项,
-1 全收); 否则各面板独立开合 (状态在面板内部, 不支持外部预设)。内容多行
换行, 展开收起带高度过渡动画。

继承: `VerticalLayout`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TCollapseItem]` | `[]` | 面板 |
| accordion | `bool` | `false` | 手风琴模式 |
| current-index | `int` (in-out) | `-1` | 仅手风琴模式有效 |

回调: `changed(int)` — 仅手风琴模式下 `current-index` 真正变化后触发, 参数是新索引,
`-1` 表示全部收起; 非手风琴模式不触发。

相关结构: `TCollapseItem { label: string, content: string, disabled: bool }`。

### TCarousel

走马灯。滑动切换 (缓动), 底部居中指示点 (当前项加宽), 可选自动轮播; 幻灯片区支持指针拖拽翻页 (越过首/末张有 1/3 阻力), 拖动超过 80px 或半张宽度 (取小) 松手即翻页, 否则回弹; 拖动期间自动轮播暂停。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TCarouselSlide]` | `[]` | 幻灯片 |
| current | `int` (in-out) | `0` | 当前页下标 (写入越界值时视觉按夹紧值渲染) |
| autoplay | `bool` | `false` | 自动轮播 |
| interval | `duration` | `3s` | 轮播间隔 |

| 回调 | 签名 | 触发 |
| --- | --- | --- |
| changed | `int` | 拖拽 / 点指示点 / 自动轮播使 `current` 真正变化之后 (同值写入不触发) |

相关结构: `TCarouselSlide { image: image }`。

### TCodeBlock

代码块。主题化底色的等宽字体面板 (固定 Consolas), 带 1px `component-border` 描边,
不换行, 高度随内容自适应。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| code | `string` | `""` | 代码文本 |

### TTable

数据表格。表头 + ListView 行 (只实例化可见行); 弹性列均分剩余宽度, 固定列按给定宽度;
固定列宽总和超出表宽时表体出现底部横向滚动条, 表头随表体同步水平滚动并保持纵向固定;
可排序列点击循环 升序 → 降序 → 取消; 模型为空时显示空态。
排序只上报, 行重排由宿主在 `sort-changed` 里完成 (Slint 无循环语句, 组件内无法扫描模型)。

继承: `Rectangle`。内部组合 ListView 与 TEmpty。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| columns | `[TTableColumn]` | `[]` | 列定义 |
| rows | `[TTableRow]` | `[]` | 行模型, 单元格文本按列下标对应 |
| sort-column | `int` (in-out) | `-1` | 当前排序列, -1 为无 |
| sort-direction | `TTableSortDirection` (in-out) | `TTableSortDirection.none` | 当前排序方向 |
| row-height | `length` | `TTheme.height-l + TTheme.spacing-4` (48px) | 表头与数据行高 |
| empty-text | `string` | `"暂无数据"` | 空态描述 |

回调: `sort-changed(int, TTableSortDirection)` — 点击可排序表头后触发, 参数为当前排序列与方向;
`row-clicked(int)` — 点击数据行后触发, 参数为行下标。

相关结构: `TTableColumn { title: string, width: length = 0px, min-width: length = 0px,
align: TTableAlign = TTableAlign.left, sortable: bool = false }`, `width: 0px` 为弹性列;
`TTableRow { cells: [string] }`。
相关枚举: `TTableAlign { left, center, right }`, `TTableSortDirection { none, ascending, descending }`。

已知限制: 列宽不可拖拽; 不支持多列排序与单元格自定义内容。表高由宿主给定
(`preferred-height` 默认 5 行)。

---

## 提示 notice

```slint
import { TAlert, TMessage, TMessageOverlay, TMessageTheme, TMessagePlacement, TNotification, TNotificationOverlay, TNotificationTheme, TNotificationPlacement, TNotificationData, TProgress, TProgressVariant, TResult, TResultStatus } from "@tela-ui/notice.slint";
```

### TAlert

警示条。整条浅色底, 图标与标题着语义色, 描述用主文本色。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| theme | `TThemeRole` | `TThemeRole.default` | 语义色, 决定底色/图标/标题 |
| title | `string` | `""` | 标题 (语义色, 半粗) |
| description | `string` | `""` | 描述文字, 空串时单行布局 |
| closable | `bool` | `false` | 显示关闭钮; 点击后自设 `open = false` 并触发 `closed()` |
| open | `bool` (in-out) | `true` | 显示/隐藏 |

回调: `closed()`。

### Message (TMessage + TMessageOverlay)

全局消息。任何代码 (Slint 或 Rust) 调用 `TMessage` 的函数弹消息, 窗口根部放
一个 `TMessageOverlay` 负责渲染; 3 秒后自动隐藏, 无 TouchArea, 不拦截输入。

`TMessage` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` (in-out) | `""` | 消息文字 |
| theme | `TMessageTheme` (in-out) | `TMessageTheme.info` | 消息类型, 决定图标与颜色 |
| visible | `bool` (in-out) | `false` | 显示状态, 由 show 函数与 Overlay 定时器维护 |
| seq | `int` (out) | `0` | 每次 show 自增, Overlay 靠它重置 3 秒计时 |
| placement | `TMessagePlacement` (in-out) | `TMessagePlacement.top-center` | 贴靠的窗口角落 |

`TMessage` 函数: `info(string)`, `success(string)`, `warning(string)`,
`error(string)`, 以及底层 `show(string, TMessageTheme)`。

`TMessageOverlay` 组件无参数, 放置即可。

相关枚举: `TMessageTheme { info, success, warning, error }`,
`TMessagePlacement { top-left, top-center, top-right, bottom-left, bottom-center, bottom-right }`。

### Notification (TNotification + TNotificationOverlay)

全局通知。任何代码 (Slint 或 Rust) 调用 `TNotification` 的函数弹通知,
窗口根部放一个 `TNotificationOverlay` 负责渲染; 右上角堆叠, 最多 4 条, 满员时
挤掉最旧一条; 每条 4.5 秒后自动消失, 也可点击关闭; 无 TouchArea, 不拦截输入。
内容文字按列宽自动换行。

`TNotification` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TNotificationData]` (in-out) | `[]` | 变长队列, 下标 0 最旧; 用 Slint 1.18 的 array push/remove 维护, 最多 4 条 |
| seq | `int` (out) | `0` | 每次 show 自增, 并作为 stamp 写进新条目 |
| placement | `TNotificationPlacement` (in-out) | `TNotificationPlacement.top-right` | 贴靠的窗口角落 |

`TNotification` 函数: `info(string, string)`, `success(…)`, `warning(…)`,
`error(…)` (参数为 标题, 内容), 底层 `show(string, string, TNotificationTheme)`,
以及 `close(int)` (按下标移除) 与 `clear()` (清空队列)。

`TNotificationOverlay` 组件无参数, 放置即可。

相关枚举/结构: `TNotificationTheme { info, success, warning, error }`,
`TNotificationPlacement { top-left, top-right, bottom-left, bottom-right }`,
`TNotificationData { title, content, theme, stamp }` (stamp 为 show() 单调
序号, 供 Overlay 区分数据替换)。

### TProgress

进度条。线性或圆环;默认数值直通,`animated: true` 时按 `duration-base` 缓动。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| variant | `TProgressVariant` | `TProgressVariant.line` | `line` / `circle` |
| value | `float` | `0` | 0..100 |
| theme | `TThemeRole` | `TThemeRole.primary` | 进度色 |
| size | `TSize` | `TSize.medium` | 线粗 4/6/10px, 圆环直径 64/80/96px |
| show-label | `bool` | `true` | 是否显示百分比文字 |
| animated | `bool` | `false` | 数值变化带缓动 (200ms);给逐帧变化的值(拖动、轮询)保持默认即可 |

相关枚举: `TProgressVariant { line, circle }`。

### TResult

结果页块: 大号状态图标 + 标题 + 描述 + 操作区, 整体在容器内居中。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| status | `TResultStatus` | `TResultStatus.info` | 状态, 决定图标与颜色 |
| title | `string` | `""` | 标题, 为空隐藏 |
| description | `string` | `""` | 描述, 为空隐藏, 单行省略 |

插槽: 默认子元素为操作区 (一行居中, 通常放按钮)。

相关枚举: `TResultStatus { info, success, warning, error }`。

## 浮层 overlay

```slint
import { TDialog, TDialogService, TDialogServiceOverlay, TDrawer, TDrawerPlacement, TBottomSheet, TActionSheetService, TActionSheetOverlay, TActionSheetItem, TImageViewerService, TImageViewerOverlay, TCommandPaletteService, TCommandPaletteOverlay, TCommandItem, TPopconfirm, TTooltipArea, TTooltipOverlay, TTooltipPlacement, TLoadingService, TLoadingServiceOverlay } from "@tela-ui/overlay.slint";
```

### TDialog

模态对话框。渲染半透明遮罩与居中卡片, 需放置在窗口级覆盖层中。

继承: `TModalSurface` (私有, 提供 `open`/`close-on-overlay`/`closed`/`close()` 与
遮罩, ESC, z-modal)。内部组合 TButton (取消/确认)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| open | `bool` (in-out) | `false` | 显示/隐藏 |
| title | `string` | `""` | 标题 |
| content | `string` | `""` | 正文文字 |
| confirm-text | `string` | `"确认"` | 确认按钮文字 |
| cancel-text | `string` | `"取消"` | 取消按钮文字 |
| show-cancel | `bool` | `true` | 为 false 时只渲染确认按钮 (alert 式单按钮) |
| close-on-overlay | `bool` | `true` | 点击遮罩是否关闭 |
| dialog-width | `length` | `480px` | 卡片宽度 (不超过父容器 - 48px) |

回调: `accepted()` (确认), `canceled()` (取消), `closed()` (任意途径关闭)。

公开函数: `close()` — 关闭并触发 `closed()`。

插槽: 默认子元素渲染在 content 与按钮行之间, 作自定义主体。

### TDialogService (TDialogService + TDialogServiceOverlay)

全局确认/警示框。任何代码 (Slint 或 Rust) 调 `TDialogService` 的函数弹框, 窗口根部
放一个 `TDialogServiceOverlay` 负责渲染; 结果通过 `result-seq` / `accepted` 上报。

`TDialogService` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| visible | `bool` (in-out) | `false` | 显示状态, 由函数与 Overlay 维护 |
| title | `string` (out) | `""` | 标题 |
| content | `string` (out) | `""` | 正文文字 |
| show-cancel | `bool` (out) | `true` | 为 false 时只渲染确认按钮 (alert 用) |
| result-seq | `int` (out) | `0` | 每次关闭自增一次, 供宿主监听 |
| accepted | `bool` (out) | `false` | 本次结果: 确认按钮为 true, 取消 / ESC / 点遮罩均为 false |

`TDialogService` 函数: `confirm(string, string)` (取消 + 确认), `alert(string, string)`
(仅确认), 以及 Overlay 内部使用的 `finish(bool)`。

`TDialogServiceOverlay` 组件无参数, 放置在窗口根部即可; 它借用 TDialog 的 z-modal
层级渲染, 并注册进 `TPopupStack`, 所以 ESC 也能关闭。

监听结果: 用镜像属性 `private property <int> r: TDialogService.result-seq;`, 在
`changed r` 里读 `TDialogService.accepted`。

### TDrawer

抽屉面板。从指定边缘滑入, 覆盖在半透明遮罩上; 面板常驻, 通过透明度与位移动画
出入场, 关闭动画不会被裁断。

与 `TBottomSheet` 的分工: TDrawer 是桌面口径的四边面板 (直角、尺寸由
`drawer-width`/`drawer-height` 固定、恒有标题行与关闭钮); `placement: bottom` 只是
把它贴到底边, 不代表移动端形态。移动端形态的底部弹层 (顶部圆角、高度随内容、可无标题
行) 用 TBottomSheet, 二者共用 TModalSurface 的遮罩与 ESC 语义。

继承: `TModalSurface` (私有, 同 TDialog)。内部组合 TIcon (关闭钮)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| open | `bool` (in-out) | `false` | 显示/隐藏 |
| title | `string` | `""` | 面板标题 |
| placement | `TDrawerPlacement` | `TDrawerPlacement.right` | `top` / `right` / `bottom` / `left` |
| drawer-width | `length` | `320px` | 面板宽度 (left/right 方向使用) |
| drawer-height | `length` | `280px` | 面板高度 (top/bottom 方向使用) |
| close-on-overlay | `bool` | `true` | 点击遮罩是否关闭 |

回调: `closed()`。公开函数: `close()`。

插槽: 默认子元素为标题下方的面板主体。

### TBottomSheet

底部弹层 (移动端形态)。面板贴窗口底边, 带圆角上沿, 高度默认随内容, 通过位移动画滑入
滑出; 遮罩, z-modal 层级, ESC 与点击遮罩关闭都来自 `TModalSurface`。适合动作列表、
移动端风格的选择面板等。与 `TDrawer` 的分工见上节: 需要固定尺寸的桌面边缘面板用
TDrawer, 需要"内容多高就多高"的底部面板用 TBottomSheet。

继承: `TModalSurface` (私有, 同 TDialog)。内部组合 TIcon (关闭钮)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| open | `bool` (in-out) | `false` | 显示/隐藏 |
| title | `string` | `""` | 面板标题 |
| sheet-height | `length` | `0px` | `0px` 时高度随主体内容, 其余值固定面板高度 |
| closable | `bool` | `true` | 为 false 时隐藏标题右侧关闭钮 (动作面板自带取消行) |
| close-on-overlay | `bool` | `true` | 点击遮罩是否关闭 |

高度上限为 `窗口高 - 48px`, 超出部分被裁掉 (与 TDialog / TDrawer 同策略, 无滚动容器)。

回调: `closed()`。公开函数: `close()`。

插槽: 默认子元素为标题下方的面板主体。

### TActionSheet (TActionSheetService + TActionSheetOverlay)

全局动作面板。任何代码 (Slint 或 Rust) 调 `TActionSheetService` 的函数弹面板, 窗口
根部放一个 `TActionSheetOverlay` 负责渲染; 面板本身就是 `TBottomSheet`, 因此遮罩,
ESC, z-modal 与滑入动画全部复用。结果通过 `result-seq` / `selected-index` 上报。

`TActionSheetService` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| visible | `bool` (in-out) | `false` | 显示状态, 由函数与 Overlay 维护 |
| title | `string` (out) | `""` | 面板标题 |
| description | `string` (out) | `""` | 标题下的说明, 空串不渲染 |
| actions | `[TActionSheetItem]` (out) | `[]` | 动作列表 |
| cancel-text | `string` (in-out) | `"取消"` | 底部取消按钮文字, 宿主可改以适配语言 |
| result-seq | `int` (out) | `0` | 每次关闭自增一次, 供宿主监听 |
| selected-index | `int` (out) | `-1` | 本次结果: 动作下标, 取消 / ESC / 点遮罩均为 `-1` |

`TActionSheetService` 函数: `show(title, description, actions)` 打开面板,
`close()` (等价 `finish(-1)`), `finish(int)` (记录结果并关闭; 各关闭路径唯一的
写入点, 所以不会重复上报)。

动作行高 48px, 文案居中并取 `theme` 语义色; `disabled` 行为禁用色且不响应点击/AT 动作。
`TActionSheetItem { label, theme: TThemeRole, disabled }`。

监听结果: 用镜像属性 `private property <int> r: TActionSheetService.result-seq;`, 在
`changed r` 里读 `TActionSheetService.selected-index`。

### TImageViewer (TImageViewerService + TImageViewerOverlay)

全局全屏图片预览。任何代码 (Slint 或 Rust) 调 `TImageViewerService.open(urls, index)`
打开, 窗口根部放一个 `TImageViewerOverlay` 负责渲染; 1x 时图片按原图比例适配进适配框
(四周留白) 居中, 适配框下方是缩略图条 (多于一张时), 左上角是 `当前 / 总数`、右上角是
缩放控件与关闭钮, 适配框左右各一个翻页箭头 (到端点变暗不可点)。

交互:

| 操作 | 效果 |
| --- | --- |
| 在图片上左右拖拽 | 超过 `min(80px, 适配框宽 / 3)` 切换上一张/下一张, 首尾拖拽有 1/3 阻力 |
| 滚轮 | 以指针为锚点缩放, 向下放大 / 向上缩小, 范围 `1x`(适配) ~ `max-zoom` |
| 双击图片 | 在适配与 2x 之间切换 |
| 缩放后拖拽 | 平移画面, 直到画面边缘到达窗口边缘 (放大态不再触发切换) |
| 点缩略图 | 直接跳到该图 |
| 点图片以外的窗口区域 / 关闭钮 / ESC | 关闭 |

缩放的显示范围是整个窗口: 放大后只被窗口裁剪, 可以铺满全屏。缩放是视图状态, 换图或
关闭时自动回到适配; 缩略图条超过窗口宽度时横向滚动。

计数 / 缩放组 / 关闭 / 左右箭头 / 缩略图条都坐在 `TLoadingServiceOverlay` 同款的不透明
卡片上 (`bg-color-container` + `radius-large` + shadow-3): 放大后图片会从控件底下走过,
白底图不再吞掉图标。

`TImageViewerService` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| visible | `bool` (in-out) | `false` | 显示状态, 由函数与 Overlay 维护 |
| urls | `[image]` (out) | `[]` | 图片表 |
| current-index | `int` (out) | `0` | 当前图片下标, 永远夹紧在 `[0, urls.length - 1]` |
| change-seq | `int` (out) | `0` | 下标真正变化时自增一次, 供宿主监听 |

`TImageViewerService` 函数: `open(urls, index)` (下标越界自动夹紧), `close()`,
`previous()`, `next()`, `go-to(index)` (同值或夹紧到边界不触发 `change-seq`)。

`TImageViewerOverlay` 组件参数: `max-zoom` (`float`, `4`, 缩放上限; 下限恒为适配),
`thumbnails` (`bool`, `true`, 是否显示缩略图条)。未包含: 旋转, 图片说明文字,
方向键翻页 (键盘焦点由模态层的 ESC 作用域占用)。

### TCommandPalette (TCommandPaletteService + TCommandPaletteOverlay)

全局命令面板 (Ctrl+K 形态)。宿主先把命令表写进 `TCommandPaletteService.commands`,
再调 `open()`; 窗口根部放一个 `TCommandPaletteOverlay` 负责渲染。面板顶部是搜索框
(打开时自动取得键盘焦点), 下面是命令列表; 输入即过滤, 回车执行高亮的第一条, 点击列表
项直接执行, ESC 与点遮罩关闭并上报"未执行"。

面板原地淡入淡出 (位移留给列表滚动), 因此关闭态的面板仍居中挂载 —— 面板内每个可交互元素
都按 `open` 闸住, 不可见的输入框不会吃掉焦点或点击。

过滤在组件内完成, 不动模型: 每条命令始终保留自己的行, 不匹配的行高折叠为 0, 因此不需要
重建数组或重排下标。`active-index` 是**第一条匹配项** — 关键字变化时每行各报一次自己,
取最小下标, 与各行触发顺序无关。匹配规则是 `label` 的大小写不敏感子串匹配。

`TCommandPaletteService` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| visible | `bool` (in-out) | `false` | 显示状态, 由函数与 Overlay 维护 |
| commands | `[TCommandItem]` (in-out) | `[]` | 命令表, 宿主可随时替换 |
| keyword | `string` (out) | `""` | 过滤关键字, 只经 `set-keyword()` 写入 |
| active-index | `int` (out) | `-1` | 高亮项下标; 第一条匹配项, 指针悬停会改写它, 无匹配为 `-1` |
| run-seq | `int` (out) | `0` | 每次关闭自增一次 |
| run-index | `int` (out) | `-1` | 本次执行的下标, 未执行任何命令为 `-1` |

`TCommandPaletteService` 函数: `open()` (清空关键字并聚焦输入框), `close()`
(等价 `run(-1)`), `set-keyword(string)`, `run(int)` / `run-active()` (越界或当前
过滤下不匹配的下标不执行), `consider(int)` 与 `highlight(int)` (供 Overlay 内部
上报匹配与悬停), `hit(TCommandItem)` (只读的匹配判定)。

`TCommandItem { label, shortcut }`, `shortcut` 右对齐显示在行尾, 通常放快捷键提示。

`TCommandPaletteOverlay` 组件参数: `palette-width` (`length`, `560px`),
`max-rows` (`int`, `6`, 超出后列表滚动), `placeholder-text`, `empty-text`。

未包含: 上下方向键在列表内步进 (1.18 的语言内没有循环语句, 服务层无法扫描匹配项 —
见 README 已知限制), 分组标题, 快捷键内置绑定 (宿主用 `KeyBinding` 放在窗口级
`FocusScope` 里自行绑定)。

### TPopconfirm

气泡确认。锚定与触发方式同 TDropdownMenu (共用 `TPopupPlacement`); 面板内为
描述文字与右对齐的取消/确定按钮, 点击任一按钮或点击外部后关闭。

继承: `Rectangle`。内部组合 TButton x2。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` | `""` | 确认描述, 超宽换行 |
| confirm-text | `string` | `"确定"` | 确认按钮文字 (主色) |
| cancel-text | `string` | `"取消"` | 取消按钮文字 (描边) |
| placement | `TPopupPlacement` | `TPopupPlacement.top-start` | 弹出锚点 |
| auto-open | `bool` | `true` | 同 TDropdownMenu |

回调: `confirm()`, `cancel()` (仅点击对应按钮时触发)。公开函数: `show()`,
`close()`, `toggle()`。只读输出: `open` (`bool`)。

### Tooltip

悬停提示气泡。渲染在窗口级 (参考 sleek-ui 的实现): 触发物通过 `TTooltip`
全局上报几何, 窗口根部的 `TTooltipOverlay` 统一绘制气泡 — 不会被卡片或
ScrollView 裁剪, 也没有包裹组件的尺寸问题。气泡延迟 250ms 出现 (可配),
移开立即消失, 反色底配菱形箭头, 文本超宽自动换行。

静态触发物用 `TTooltipArea` (自动充满父容器并接管悬停):

```slint
// 窗口根部放置一次:
TTooltipOverlay { }

TTooltipArea {
    width: 120px; height: 32px;
    text: "提示"; placement: TTooltipPlacement.top;
    Text { text: "目标"; }
}
```

交互式触发物 (TButton) 自行上报:

```slint
ta := TouchArea {
    changed has-hover => {
        TTooltip.show("提示", TTooltipPlacement.top,
                         self.has-hover, self.absolute-position,
                         self.width, self.height);
    }
}
```

`TTooltip` 全局函数: `show(text, placement, hovering, coords, width, height)`
— coords 传触发物的 `absolute-position` (窗口坐标), hovering 为 false 时仅
驱动隐藏, 不覆盖状态。

`TTooltipArea` 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` | `""` | 气泡文字 |
| placement | `TTooltipPlacement` | `TTooltipPlacement.top` | 弹出方向 |

插槽: 默认子元素为触发内容 (悬停层在其之上, 静态内容适用)。

`TTooltipOverlay` 参数 (应用级配置):

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| delay | `duration` (in-out) | `250ms` | 显示延迟 |
| wrap-width | `length` (in-out) | `240px` | 换行阈值 |
| show-arrow | `bool` (in-out) | `true` | 菱形箭头 |

相关枚举: `TTooltipPlacement { top, bottom, left, right }`。

### TLoadingService (TLoadingService + TLoadingServiceOverlay)

全局全屏加载遮罩。任何代码 (Slint 或 Rust) 调 `TLoadingService.show(text)` 开启,
`hide()` 关闭; 窗口根部放一个 `TLoadingServiceOverlay` 负责渲染, 遮罩期间吞掉指针输入。

`TLoadingService` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| visible | `bool` (in-out) | `false` | 显示状态, 由 show/hide 维护 |
| text | `string` (out) | `"加载中…"` | 卡片上的加载文字 |

`TLoadingService` 函数: `show(string)`, `hide()`。

`TLoadingServiceOverlay` 组件无参数, 放置在窗口根部即可; 层级在模态之上 (也盖住已打开
的对话框), 淡出后隐藏且隐藏时不吞输入。它刻意不注册进 `TPopupStack` — 加载是状态,
不该被 ESC 关掉。

## 导航 navigation

```slint
import { TTabs, TPagination, TNavBar, TSideNav, TBreadcrumb, TSteps, TDropdownMenu, TBackTop } from "@tela-ui/navigation.slint";
```

### TTabs

标签页头。下划线随选中项平滑移动, 各项等宽; 页面内容由宿主用
`if current-index == i` 切换渲染。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[string]` | `[]` | 标签文字 |
| current-index | `int` (in-out) | `0` | 选中下标 |

回调: `changed(int)` — 新选中的下标。

### TPagination

分页器。上一页/下一页与最多 7 个页码的滑动窗口。

继承: `Rectangle`。内部使用私有 TPageBtn 与 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| total-pages | `int` | `1` | 总页数 |
| current | `int` (in-out) | `1` | 当前页, 从 1 起 |

回调: `changed(int)` — 新的页码。

### TNavBar

顶部导航栏。左侧品牌 (logo + 标题), 中间菜单项 (选中项带下划线), 右侧操作区。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| title | `string` | `""` | 品牌标题 |
| logo | `TIconData` | 空 | 品牌图标 |
| items | `[TNavItem]` | `[]` | 菜单项 |
| current-value | `string` (in-out) | `""` | 选中的 value |
| bordered | `bool` | `true` | 底部 1px 分隔线 |

回调: `changed(string)` — 新选中的 value。

相关结构: `TNavItem { label: string, value: string }`。

插槽: 默认子元素为右侧操作区 (已垂直居中, 无需自行定位)。

### TSideNav

可折叠侧边导航。品牌头部, 垂直菜单与钉在底部的页脚区; 折叠时图标+文字行收缩为
居中图标, 宽度在 232px 与 64px 间动画过渡。

继承: `Rectangle`。内部组合私有 TNavRow 与 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| title | `string` | `""` | 品牌标题 |
| logo | `TIconData` | 空 | 品牌图标 |
| items | `[TSideNavItem]` | `[]` | 菜单项 |
| current-value | `string` (in-out) | `""` | 选中的 value (菜单项与带 value 的页脚项共用) |
| collapsed | `bool` (in-out) | `false` | 折叠态, 收起行与顶部按钮均可翻转 |
| mode | `TSideNavMode` | `TSideNavMode.icon-text` | `icon-text` 可折叠 / `text` 纯文字 (恒 232px) / `icon` 纯图标 (恒 64px) |
| toggle-position | `TSideNavTogglePosition` | `TSideNavTogglePosition.top` | `top` 头部右缘 / `bottom` 底部收起行 |
| bordered | `bool` | `true` | 右侧 1px 分隔线 |
| footer-items | `[TSideNavFooterItem]` | `[]` | 钉在底部的行, 位于收起行之上 |

回调: `changed(string)` (菜单或带 value 的页脚项), `footer-clicked(TSideNavFooterItem)`
(无 value 的页脚项, 纯动作)。

相关结构:

- `TSideNavItem { label: string, value: string, icon: TIconData = { paths: [] }, disabled: bool = false }`
- `TSideNavFooterItem { label: string = "", icon: TIconData = { paths: [] }, value: string = "" }`
  — `value` 非空时行为等同菜单行 (参与选中并触发 `changed`), 为空时是纯按钮并触发
  `footer-clicked`。

相关枚举: `TSideNavMode { icon-text, text, icon }`,
`TSideNavTogglePosition { top, bottom }`。

### TBreadcrumb

面包屑。非当前项渲染为链接色并触发 `changed`, 当前项为主文本色。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TCrumbItem]` | `[]` | 层级项 |
| current-value | `string` (in-out) | 最后一项的 value | 当前层级 value |
| separator | `string` | `"/"` | 分隔符 |
| separator-color | `color` | `TTheme.text-color-disabled` | 分隔符颜色 |

回调: `changed(string)` — 点击的非当前项 value。

相关结构: `TCrumbItem { label: string, value: string }`。

### TSteps

步骤条。等宽分栏, `current` 之前的步骤显示对勾, 之后的显示序号。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[string]` | `[]` | 步骤名 |
| current | `int` | `0` | 当前步骤下标, 之前的步骤计为已完成 |

### TDropdownMenu

下拉动作菜单。锚定触发内容的 PopupWindow 弹层; `auto-open` (默认开) 在触发
内容之上叠加点击层, 适合静态触发物 (图标/文本/头像); TButton 触发请设
`auto-open: false` 并在 `clicked` 中调用 `toggle()`。点击外部或选中后关闭;
暂无键盘导航, 超宽条目省略显示, 超过 max-visible-items 后弹层内滚动。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TDropdownMenuItem]` | `[]` | 菜单项 |
| placement | `TPopupPlacement` | `TPopupPlacement.bottom-start` | 弹出锚点 |
| auto-open | `bool` | `true` | 内置点击层开合; 交互式触发器关掉它 |
| item-height | `length` | `TTheme.height-m` | 单项高度 (分隔线行同高) |
| max-visible-items | `int` | `6` | 弹层内直接可见的行数, 超出后滚动 |

回调: `selected(int, string)` — (下标, value)。公开函数: `show()`, `close()`,
`toggle()`。只读输出: `open` (`bool`, 弹层是否展开)。

相关结构/枚举: `TDropdownMenuItem { label, value, icon, disabled, danger,
divider }` (icon 为 `TIconData` 默认空; divider 为 true 时渲染分隔线并忽略
其余字段); `TPopupPlacement { bottom-start, bottom-end, top-start, top-end }`
与 TPopconfirm 共用。

### TBackTop

回到顶部按钮。`content-y` 与目标 ScrollView 的 `content-y` 双向绑定:
滚动超过 `threshold` 显示, 点击滚动归零。需绝对定位于滚动容器内
(参见 gallery 页)。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| content-y | `length` (in-out) | `0` | 双向绑定目标滚动位置 |
| threshold | `length` | `200px` | 显隐阈值 |

回调: `clicked()` — 归零之外附加动作时使用。

## 实验性组件 experimental

API 不稳定, 可能随版本调整或移除。

```slint
import { TSkeleton } from "@tela-ui/experimental.slint";
```

## 全局对象

### TTheme (主题令牌)

全局单例, 定义在 `@tela-ui/theme.slint`。承载完整 TDesign 令牌集并提供亮/暗双预设。

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| preference | `TThemePreference` (in-out) | `TThemePreference.system` | 用户意图: 跟随系统/强制亮/强制暗 |
| is-dark | `bool` (out) | — | 实际生效模式 (`system` 对 OS 色彩方案解析后的结果); 亮暗开关绑定到它而非 preference |
| theme | `TAppThemeConfig` (in-out) | 亮暗随 is-dark 切换 | 当前生效配置, 直接赋值可完全接管 |
| light / dark | `TAppThemeConfig` (in-out) | 内置预设 | 两个模式预设, 可逐字段覆盖 |

`TThemePreference { system, light, dark }`。

令牌转发 (均为 `out` 只读快捷属性): 四组语义色阶 `primary/success/warning/error`
(完整 `TPrimaryColor` 结构) 与 7 态快捷色 (`brand-color`, `brand-color-hover`,
`brand-color-focus`, `brand-color-active`, `brand-color-disabled`, `brand-color-light`,
`brand-color-light-hover`, success/warning/error 同构); 14 级灰 `gray-1..gray-14`;
背景 `bg-color-*`; 文本 `text-color-*`; 边框与遮罩 `border-level-1/2-color`,
`component-stroke`, `component-border`, `mask-active/disabled`; 阴影 `shadow-1/2/3-*`;
圆角 `radius-*`; 间距 `spacing-1..16`, `padding-lr-*`, `margin-*`; 控件高度
`height-s/m/l`; 字体 `font-family`, `font-weight-*`, `font-size-*`, `line-height-*`;
动效 `duration-fast/base/slow`。

辅助函数 (供组件与应用按语义取值):

| 函数 | 返回类型 | 说明 |
| --- | --- | --- |
| `theme-scheme(t: TThemeRole)` | `TPrimaryColor` | 语义角色对应的色阶, default 解析为品牌色阶 |
| `theme-color(t)` / `-hover` / `-active` / `-light` / `-light-hover` / `-disabled` | `color` | 语义色的各状态; default 保持中性灰语义 |
| `size-height(s: TSize)` | `length` | 控件高度 24/32/40px |
| `size-font(s: TSize)` | `length` | 字号 12/14/16px |
| `size-line-height(s: TSize)` | `length` | `size-font` 配套的行盒 20/22/24px, 交给 `TText.line-height` |
| `size-padding(s: TSize)` | `length` | 水平内边距 8/12/16px |

### TIconSet (图标数据)

全局图标库, 定义在 `@tela-ui/base/icon/data.slint` (经 `@tela-ui/base.slint` 再导出)。
纯数据文件, 每个图标是 24x24 网格上的 SVG 路径集合, 由共享的 `TIcon` 组件渲染。
几何数据来自 Lucide (ISC 许可)。

结构:

- `TIconPath { command: string, has_fill: bool }` — 单条路径
- `TIconData { paths: [TIconPath] }` — 完整图标, 即各组件的图标参数类型

内置 44 个字形, 以 `TIconSet.名称` 取用 (按源文件声明顺序):

X, Check, Minus, Plus, Search, ChevronDown, ChevronUp, ChevronLeft, ChevronRight,
Menu, Info, CircleCheck, CircleX, CircleAlert, TriangleAlert, Eye, Calendar, Clock,
User, LoaderCircle, Sun, Moon, Bell, House, Lock, Mail, Star, StarFill, Heart, Trash2,
Pencil, Settings, Download, Upload, RefreshCw, Image, Ellipsis, LayoutDashboard,
PanelLeftClose, PanelLeftOpen, FlaskConical, Inbox, Square, Copy。

其中 `Square` / `Copy` 是窗口最大化键的两个字形 (最大化显示 `Square`, 已最大化显示
`Copy` 表示"向下还原")。

### TLightColors / TDarkColors (调色板)

定义在 `@tela-ui/colors.slint`。两组 `out` 属性 `brand` / `success` / `warning` /
`error`, 各返回一个完整 `TPrimaryColor`。内置预设消费它们; 自定义主题也应基于
它们派生以保留原色阶。

### TPopupStack (浮层计数与 ESC)

窗口级浮层账本, 定义在 `tela-ui/popup-stack.slint` (顶层文件, 不经聚合器导出)。
记录当前打开了多少层覆盖 (对话框 / 抽屉 / 服务 Overlay) 与哪一层在最上面; ESC 只
投递给最顶层, 由那一层自己关闭。

`TPopupStack` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| open-count | `int` (out) | `0` | 当前打开的层数 |
| close-top-seq | `int` (out) | `0` | 自增一次表示"最顶层该关了" (ESC) |
| closed-seq | `int` (out) | `0` | 任意层关闭时自增 |
| closed-depth | `int` (out) | `-1` | 刚关闭那一层的槽位, 供其上的层重排深度 |

`TPopupStack` 函数:

| 函数 | 说明 |
| --- | --- |
| `layer-opened()` | 层激活时调用; 该层先把当时的 `open-count` 记为自己的深度, 再调它 |
| `layer-closed(int)` | 层关闭时调用, 参数是该层深度 |
| `press-escape()` | 转发一次 ESC; `open-count > 0` 时自增 `close-top-seq` |

`TPopupStackLayer` 组件: 每个覆盖层嵌一个, `active` 绑本层的 open 状态, 收到
`close-requested` 就按自己的方式关闭 (通常调 `close()`)。它负责登记/注销深度、在
激活期间抓住键盘焦点 (否则 ESC 到不了这里)、并在 `depth == open-count - 1` 时响应
`close-top-seq`; 中间层关闭后上层深度自动下移。

| 属性/回调 | 类型 | 说明 |
| --- | --- | --- |
| active | `bool` (in) | 绑到宿主 open 状态, 为 true 时入栈并抢焦点 |
| grab-focus | `bool` (in) | 默认 `true`; 层内自带可聚焦内容 (命令面板输入框) 时置 false, 由宿主自己聚焦并转发 ESC |
| close-requested() | callback | ESC 落到本层时要求关闭 |

`TModalSurface` 把 `grab-focus` 原样透出给自己的层, 所以 `TCommandPaletteOverlay`
只需在子类上写 `grab-focus: false`, 再让输入框所在的 `FocusScope` 处理 ESC。

基于 PopupWindow 的弹层 (dropdown / popconfirm / select) 不注册: Slint 运行时自己
就会在 ESC 时关掉最上面的那个 PopupWindow。

---

## 共享结构体与枚举

### structs/theme.slint

- `TPrimaryColor` — 一个色彩家族: 10 级梯度 `color-1..color-10` (1 最浅 .. 10 最深)
  加 7 个语义快捷字段 `color`, `hover`, `focus`, `active`, `disabled`, `light`,
  `light-hover`, 类型均为 `color`。
- `TAppThemeConfig` — 一个模式的完整令牌集: 动画时长 3 项 (`duration`), 四组色阶,
  背景/文本/边框/遮罩/阴影令牌, 圆角, 间距系统, 控件高度与排版令牌, 字段清单见
  源文件 `tela-ui/structs/theme.slint`。

### structs/enums.slint (经 @tela-ui/theme.slint 再导出)

- `TThemeRole { default, primary, success, warning, error }` — 语义色角色。
- `TSize { small, medium, large }` — 标准控件尺寸。

### 各组件自带的枚举与结构

| 名称 | 归属组件 | 取值/字段 |
| --- | --- | --- |
| TButtonVariant / TButtonShape | TButton | base, outline, text / rectangle, square, round |
| TTagVariant | TTag | dark, light, outline |
| TDividerAlign | TDivider | left, center, right |
| TInputStatus | TInput, TTextArea | default, success, warning, error |
| TSelectItem | TSelect | label, value, disabled |
| TRadioItem | TRadio, TRadioGroup | text, value |
| TProgressVariant | TProgress | line, circle |
| TAvatarSize / TAvatarShape | TAvatar | small, medium, large, extra-large / circle, square |
| TDrawerPlacement | TDrawer | top, right, bottom, left |
| TActionSheetItem | TActionSheetService | label, theme, disabled |
| TCommandItem | TCommandPaletteService | label, shortcut |
| TMessageTheme / TMessagePlacement | Message | info, success, warning, error / 六向贴靠 |
| TNotificationTheme / TNotificationPlacement / TNotificationData | Notification | info, success, warning, error / 四角贴靠 / title, content, theme, stamp |
| TTooltipPlacement | Tooltip | top, bottom, left, right |
| TDropdownMenuItem | TDropdownMenu | label, value, icon, disabled, danger, divider |
| TPopupPlacement | TDropdownMenu, TPopconfirm (经 @tela-ui/theme.slint 再导出) | bottom-start, bottom-end, top-start, top-end |
| TTimelineItem | TTimeline | label, content, theme |
| TDescriptionItem | TDescriptions | label, value |
| TResultStatus | TResult | info, success, warning, error |
| TCollapseItem | TCollapse | label, content, disabled |
| TCarouselSlide | TCarousel | image |
| TDate | TCalendar, TDatePicker | year, month, day (全 0 = 未选) |
| TTime | TTimePicker | hour, minute, second (hour < 0 = 未选) |
| TAutoCompleteItem | TAutoComplete | label, value, disabled |
| TTableColumn / TTableRow / TTableAlign / TTableSortDirection | TTable | title, width, min-width, align, sortable / cells / left, center, right / none, ascending, descending |
| TNavItem | TNavBar | label, value |
| TSideNavMode / TSideNavTogglePosition | TSideNav | icon-text, text, icon / top, bottom |
| TSideNavItem | TSideNav | label, value, icon, disabled |
| TSideNavFooterItem | TSideNav | label, icon, value |
| TCrumbItem | TBreadcrumb | label, value |
| TThemePreference | TTheme | system, light, dark |
| TIconPath / TIconData | TIcon | command, has_fill / paths |
