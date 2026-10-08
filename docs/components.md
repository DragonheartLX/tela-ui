# 组件文档

本文档列出 tela-ui 的全部组件, 说明每个组件的继承关系, 参数, 参数类型与回调。
组件按功能分为五类, 每类一个目录与聚合器: `base/` (基础), `data-entry/` (数据录入),
`data-display/` (数据展示), `feedback/` (反馈), `navigation/` (导航), 另有
`experimental/` (实验性); 分别由 `@tela-ui/base.slint`, `@tela-ui/data-entry.slint`,
`@tela-ui/data-display.slint`, `@tela-ui/feedback.slint`, `@tela-ui/navigation.slint`,
`@tela-ui/experimental.slint` 聚合导出, 也可以直接从组件文件导入
(如 `@tela-ui/base/button.slint`)。`@tela-ui/common.slint` 保留为全量兼容出口,
汇总全部类别。主题令牌统一从 `@tela-ui/theme.slint` 导入。

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
| TBorderlessWindow | `Window` | 在 Window 内建属性 (`title`, `no-frame` 等) 上叠加自绘标题栏 |
| THeading, TParagraph, TCaption | `Text` | 保留 Text 全部内建属性 (`text`, `color`, `wrap` 等) |
| TRadioGroup | `HorizontalLayout` | 本身就是一行布局 |
| TTimeline, TStatistic, TCollapse | `VerticalLayout` | 本身即纵向布局, 首选尺寸随内容 |
| TLoading, TSegmented, TCarousel (内部) | 布局参与 sizing | 见各组件说明 (布局根的约束是头等公民) |
| 其余全部组件 | `Rectangle` | 基础容器 |

组合关系 (has-a, 内部使用的组件):

- TButton = TIcon + TLoadingSpinner
- TLoading = TLoadingSpinner
- TDialog = TButton x2 (确认/取消)
- TPopconfirm = TButton x2 (取消/确认)
- TRadioGroup = TRadio xN
- TSideNav = 私有 TNavRow xN (菜单行, 底部行与收起行共用)
- TTag, TCheckBox, TAlert, TMessageOverlay, TNotificationOverlay, TNavBar, TSideNav,
  TSelect, TPagination, TAvatar, TDropdownMenu, TRate, TEmpty, TResult,
  TCollapse, TBorderlessWindow 内部均使用 TIcon 渲染字形
- 提供 `@children` 插槽的组件: TBadge (被标记内容), TCard (卡片主体), TDialog
  (自定义主体), TDrawer (面板主体), TNavBar (右侧操作区), TEmpty (操作区),
  TTooltipArea (触发内容), TResult (操作区), TBorderlessWindow (窗口内容)

私有组件 (仅供实现参考, 不经聚合器导出): `TInputFrame` (data-entry/input.slint,
已对库内导出供 TInputNumber 复用), `TNavRow` (navigation/side-nav.slint),
`TWindowButton` (base/borderless-window.slint), `TPageBtn`
(navigation/pagination.slint)。

---

## 基础组件 base

```slint
import { TButton, TLink, TIcon, THeading, TParagraph, TCaption, TDivider, TLoading, TKbd, TBorderlessWindow } from "@tela-ui/base.slint";
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
| block | `bool` | `false` | 撑满所在布局的剩余宽度 |
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

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| icon | `TIconData` | 空 | `TIconSet.Xxx` 或自定义字形 |
| icon-color | `brush` | `TTheme.text-color-primary` | 线条与填充颜色 |
| size | `length` (in-out) | `16px` | 渲染尺寸 |
| stroke-width | `float` (in-out) | `2.0` | 24 网格单位的线宽, 随 size 缩放 |
| absolute-stroke-width | `bool` | `false` | 为 true 时 stroke-width 按绝对像素 |

只读输出: `computed-stroke-width` (`length`, 实际线宽)。

### THeading / TParagraph / TCaption

排版组件, 均继承 `Text`, 保留 `text`, `color`, `wrap` 等全部内建属性。

| 组件 | 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| THeading | level | `int` | `3` | 1..5 对应 36/28/24/20/18px, 半粗字重 |
| TParagraph | secondary | `bool` | `false` | true 时用次级文本灰 |
| TCaption | (无新增参数) | — | — | 12px 次级灰, 常用于辅助说明 |

三者的默认颜色为 `text-color-primary` (THeading/TParagraph) 与
`text-color-secondary` (TCaption), TParagraph 与 TCaption 默认 `wrap: word-wrap`。

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

继承: 均为 `Rectangle`。TLoading 内部组合 TLoadingSpinner。

| 组件 | 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| TLoading | text | `string` | `""` | 空串时只显示转圈 |
| TLoading | theme | `TThemeRole` | `TThemeRole.primary` | 圈的颜色 |
| TLoading | size | `length` (in-out) | `24px` | 圈直径 |
| TLoadingSpinner | spinner-color | `brush` | `TTheme.brand-color` | 圈的颜色 |
| TLoadingSpinner | size | `length` (in-out) | `16px` | 圈直径 |
| TLoadingSpinner | stroke-width | `float` (in-out) | `2.0` | 24 网格单位线宽 |
| TLoadingSpinner | absolute-stroke-width | `bool` | `false` | 按绝对像素解释线宽 |

### TBorderlessWindow

无边框应用窗口。自绘标题栏 (拖拽热区, 最小化/最大化/关闭), 圆角与边缘缩放。
用作应用窗口根, 子元素即窗口内容。

继承: `Window` (保留 `title`, `icon`, `minimized`, `maximized` 等内建属性)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| window-background | `brush` | `TTheme.bg-color-page` | 圆角窗口表面色; 不要设置 Window 自身的 `background`, 那会把圆角涂成不透明 |
| window-radius | `length` | `8px` | 窗口圆角, 最大化时归零 |
| resize-border | `length` | `8px` | 边缘拖拽缩放热区宽度, 最大化时归零 |
| header-height | `length` | `40px` | 标题栏高度 |
| header-background | `brush` | `TTheme.bg-color-container` | 标题栏底色 |
| logo | `TIconData` | 空 | 标题前的品牌图标 |
| maximizable | `bool` | `true` | 为 false 时隐藏最大化按钮 (工具对话框等) |
| header-title-visible | `bool` | `true` | 隐藏标题文字 (任务栏 title 不受影响) |

内部组合: 私有 TWindowButton, TIcon 与内建 `WindowMoveArea` (拖拽热区)。

---

### TKbd

键位标签 (如 `Ctrl + C` 中的按键帽)。小号文字 + 浅底细边圆角, 宽度随文字
收紧; 与 `Text` 的 "+" 混排组合成快捷键说明。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` | `""` | 键帽文字 |

## 数据录入 data-entry

```slint
import { TInput, TTextArea, TInputNumber, TSelect, TCheckBox, TRadioGroup, TSwitch, TSlider, TRate, TSegmented } from "@tela-ui/data-entry.slint";
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
| read-only | `bool` (in-out) | `false` | 只读 |
| prefix-icon | `TIconData` | 空 | 前缀图标 |
| suffix-icon | `TIconData` | 空 | 后缀图标 (显示清空钮时被隐藏) |
| clearable | `bool` | `false` | 悬停且非空时显示清空钮 |
| input-type | `InputType` (Slint 内建) | `InputType.text` | `password` 等输入类型 |

TTextArea 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| placeholder-text | `string` | `""` | 占位文字 |
| text | `string` (in-out) | `""` | 输入内容, 自动换行 |
| read-only | `bool` (in-out) | `false` | 只读 |
| rows | `int` | `4` | 期望可见行数, 高度随之计算 |

继承自 TInputFrame 的参数 (两者通用):

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| status | `TInputStatus` | `TInputStatus.default` | 边框色: `default` / `success` / `warning` / `error` |
| disabled | `bool` | `false` | 禁用态 |
| radius | `length` | `TTheme.radius-default` | 圆角 (TTextArea 固定为 `radius-medium`) |

回调: TInput `accepted(string)` (回车), `changed(string)` (每次输入), `cleared()`;
TTextArea `changed(string)`。

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

### TSelect

下拉选择。弹层锚定在触发器下方, 宽度随触发器 (至少 120px), 最多显示 6 项;
暂无键盘导航。

继承: `Rectangle`。内部组合 TIcon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TSelectItem]` | `[]` | 选项列表 |
| current-index | `int` (in-out) | `-1` | 选中项下标, -1 为未选择 |
| current-value | `string` (in-out) | `""` | 选中项 value, 选择时组件内部一并写入 |
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


## 数据展示 data-display

```slint
import { TTag, TAvatar, TBadge, TCard, TEmpty, TDescriptions, TTimeline, TStatistic, TCollapse, TTooltipArea, TCarousel, TCodeBlock } from "@tela-ui/data-display.slint";
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

相关结构: `TCollapseItem { label: string, content: string, disabled: bool }`。

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

### TCarousel

走马灯。滑动切换 (缓动), 底部居中指示点 (当前项加宽), 可选自动轮播。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[TCarouselSlide]` | `[]` | 幻灯片 |
| current | `int` (in-out) | `0` | 当前页下标 |
| autoplay | `bool` | `false` | 自动轮播 |
| interval | `duration` | `3s` | 轮播间隔 |

相关结构: `TCarouselSlide { image: image }`。

### TCodeBlock

代码块。主题化底色的等宽字体面板 (固定 Consolas), 不换行, 高度随内容自适应。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| code | `string` | `""` | 代码文本 |

---

## 反馈 feedback

```slint
import { TAlert, TDialog, TDrawer, TPopconfirm, TMessage, TNotification, TProgress, TResult } from "@tela-ui/feedback.slint";
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

### TDialog

模态对话框。渲染半透明遮罩与居中卡片, 需放置在窗口级覆盖层中。

继承: `Rectangle`。内部组合 TButton (取消/确认)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| open | `bool` (in-out) | `false` | 显示/隐藏 |
| title | `string` | `""` | 标题 |
| content | `string` | `""` | 正文文字 |
| confirm-text | `string` | `"确认"` | 确认按钮文字 |
| cancel-text | `string` | `"取消"` | 取消按钮文字 |
| close-on-overlay | `bool` | `true` | 点击遮罩是否关闭 |
| dialog-width | `length` | `480px` | 卡片宽度 (不超过父容器 - 48px) |

回调: `accepted()` (确认), `canceled()` (取消), `closed()` (任意途径关闭)。

公开函数: `close()` — 关闭并触发 `closed()`。

插槽: 默认子元素渲染在 content 与按钮行之间, 作自定义主体。

### TDrawer

抽屉面板。从指定边缘滑入, 覆盖在半透明遮罩上; 面板常驻, 通过透明度与位移动画
出入场, 关闭动画不会被裁断。

继承: `Rectangle`。内部组合 TIcon (关闭钮)。

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

### Message (TMessage + TMessageOverlay)

全局消息。任何代码 (Slint 或 Rust) 调用 `TMessage` 的函数弹消息, 窗口根部放
一个 `TMessageOverlay` 负责渲染; 3 秒后自动隐藏, 无 TouchArea, 不拦截输入。

`TMessage` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` (in-out) | `""` | 消息文字 |
| theme | `TMessageTheme` (in-out) | `TMessageTheme.info` | 消息类型, 决定图标与颜色 |
| visible | `bool` (in-out) | `false` | 显示状态, 由 show 函数与 Overlay 定时器维护 |
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
| items | `[TNotificationData]` (in-out) | 4 个隐藏槽位 | 固定容量, 下标 0 最旧; Slint 模型无 push/remove, 以槽位原地赋值实现堆叠 |
| placement | `TNotificationPlacement` (in-out) | `TNotificationPlacement.top-right` | 贴靠的窗口角落 |

`TNotification` 函数: `info(string, string)`, `success(…)`, `warning(…)`,
`error(…)` (参数为 标题, 内容), 底层 `show(string, string, TNotificationTheme)`,
以及 `close(int)` (按下标关闭) 与 `clear()`。

`TNotificationOverlay` 组件无参数, 放置即可。

相关枚举/结构: `TNotificationTheme { info, success, warning, error }`,
`TNotificationPlacement { top-left, top-right, bottom-left, bottom-right }`,
`TNotificationData { title, content, theme, visible }` (visible 为槽位占用
标记, 仅 Overlay 内部使用)。

### TProgress

进度条。线性或圆环, value 变化带缓动动画。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| variant | `TProgressVariant` | `TProgressVariant.line` | `line` / `circle` |
| value | `float` | `0` | 0..100 |
| theme | `TThemeRole` | `TThemeRole.primary` | 进度色 |
| size | `TSize` | `TSize.medium` | 线粗 4/6/10px, 圆环直径 64/80/96px |
| show-label | `bool` | `true` | 是否显示百分比文字 |

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
| item-height | `length` | `32px` | 单项高度 (分隔线行 9px) |
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
(参见演示页)。

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
| `size-padding(s: TSize)` | `length` | 水平内边距 8/12/16px |

### TIconSet (图标数据)

全局图标库, 定义在 `@tela-ui/base/icon/data.slint` (经 `@tela-ui/base.slint` 再导出)。
纯数据文件, 每个图标是 24x24 网格上的 SVG 路径集合, 由共享的 `TIcon` 组件渲染。
几何数据来自 Lucide (ISC 许可)。

结构:

- `TIconPath { command: string, has_fill: bool }` — 单条路径
- `TIconData { paths: [TIconPath] }` — 完整图标, 即各组件的图标参数类型

内置 39 个字形, 以 `TIconSet.名称` 取用:

X, Check, Minus, Plus, Search, ChevronDown, ChevronUp, ChevronLeft, ChevronRight,
Menu, Info, CircleCheck, CircleX, CircleAlert, TriangleAlert, Eye, Calendar, User,
LoaderCircle, Sun, Moon, Bell, House, Lock, Mail, Star, Heart, Trash2, Pencil,
Settings, Download, Upload, RefreshCw, Image, Ellipsis, LayoutDashboard,
PanelLeftClose, PanelLeftOpen, FlaskConical。

### TLightColors / TDarkColors (调色板)

定义在 `@tela-ui/colors.slint`。两组 `out` 属性 `brand` / `success` / `warning` /
`error`, 各返回一个完整 `TPrimaryColor`。内置预设消费它们; 自定义主题也应基于
它们派生以保留原色阶。

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
| TMessageTheme / TMessagePlacement | Message | info, success, warning, error / 六向贴靠 |
| TNotificationTheme / TNotificationPlacement / TNotificationData | Notification | info, success, warning, error / 四角贴靠 / title, content, theme, visible |
| TTooltipPlacement | Tooltip | top, bottom, left, right |
| TDropdownMenuItem | TDropdownMenu | label, value, icon, disabled, danger, divider |
| TPopupPlacement | TDropdownMenu, TPopconfirm (经 @tela-ui/theme.slint 再导出) | bottom-start, bottom-end, top-start, top-end |
| TTimelineItem | TTimeline | label, content, theme |
| TDescriptionItem | TDescriptions | label, value |
| TResultStatus | TResult | info, success, warning, error |
| TCollapseItem | TCollapse | label, content, disabled |
| TCarouselSlide | TCarousel | image |
| TNavItem | TNavBar | label, value |
| TSideNavMode / TSideNavTogglePosition | TSideNav | icon-text, text, icon / top, bottom |
| TSideNavItem | TSideNav | label, value, icon, disabled |
| TSideNavFooterItem | TSideNav | label, icon, value |
| TCrumbItem | TBreadcrumb | label, value |
| TThemePreference | TTheme | system, light, dark |
| TIconPath / TIconData | TIcon | command, has_fill / paths |
