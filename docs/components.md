# 组件文档

本文档列出 tela-ui 的全部组件, 说明每个组件的继承关系, 参数, 参数类型与回调。
组件按 `base/` (基础), `common/` (常用), `experimental/` (实验性) 三组组织, 分别由
`@tela-ui/base.slint`, `@tela-ui/common.slint`, `@tela-ui/experimental.slint` 聚合导出,
也可以直接从组件文件导入 (如 `@tela-ui/base/button.slint`)。主题令牌统一从
`@tela-ui/theme.slint` 导入。

文中类型均按 Slint 类型书写。未标注读写的参数为 `in` (只读输入), `in-out` 表示
可从外部读写, `out` 表示组件只写, 调用方只能读取。默认值一栏给出 Slint 默认绑定。

## 通用约定

- **语义色 `Theme`**: `default, primary, success, warning, error`。`default` 渲染中性灰,
  其余映射到对应色阶。定义在 `structs/enums.slint`, 经 `@tela-ui/theme.slint` 再导出。
- **尺寸 `Size`**: `small, medium, large`。控件高度, 字号与内边距随之一档缩放。
- **图标参数**: 所有图标入口统一为 `TIconData` 结构, 传 `TIconSet.Xxx` 的内置图标
  或自行构造的字形数据, 见 [TIconSet](#ticonset-图标数据)。
- **回调命名**: 状态变化用 `changed`, 主动点击用 `clicked`, 结果性动作按语义命名
  (`accepted`, `canceled`, `closed`, `selected` 等)。
- **枚举导入**: 共享枚举 `Theme`, `Size` 从 `@tela-ui/theme.slint` 导入; 组件专属枚举
  (如 `ButtonVariant`, `DrawerPlacement`) 从所属聚合器导入。同名类型不要经由两条
  聚合路径重复导入, 以免命名冲突。

## 继承与组合总览

Slint 组件为单继承 (`inherits`), 子类可直接使用基类的全部属性。库内的继承链:

| 组件 | 继承自 | 说明 |
| --- | --- | --- |
| Input, TextArea | `InputFrame` (私有) | 共享状态边框, 悬停/聚焦/禁用态与点击聚焦行为 |
| BorderlessWindow | `Window` | 在 Window 内建属性 (`title`, `no-frame` 等) 上叠加自绘标题栏 |
| Heading, Paragraph, Caption | `Text` | 保留 Text 全部内建属性 (`text`, `color`, `wrap` 等) |
| RadioGroup | `HorizontalLayout` | 本身就是一行布局 |
| 其余全部组件 | `Rectangle` | 基础容器 |

组合关系 (has-a, 内部使用的组件):

- Button = Icon + LoadingSpinner
- Loading = LoadingSpinner
- Dialog = Button x2 (确认/取消)
- RadioGroup = Radio xN
- SideNav = 私有 NavRow xN (菜单行, 底部行与收起行共用)
- Tag, CheckBox, Alert, MessageOverlay, NavBar, SideNav, Select, Pagination,
  Avatar, BorderlessWindow 内部均使用 Icon 渲染字形
- 提供 `@children` 插槽的组件: Badge (被标记内容), Card (卡片主体), Dialog
  (自定义主体), Drawer (面板主体), NavBar (右侧操作区), BorderlessWindow (窗口内容)

私有组件 (未导出, 仅供实现参考): `InputFrame` (input.slint), `NavRow`
(side-nav.slint), `WindowButton` (borderless-window.slint), `PageBtn` (pagination.slint)。

---

## 基础组件 base

```slint
import { Button, Link, Icon, Heading, Paragraph, Caption, Divider, Tag, Loading, BorderlessWindow } from "@tela-ui/base.slint";
```

### Button

按钮。`theme x variant x size` 组合出全部样式; 方形变体可作图标按钮。

继承: `Rectangle`。内部组合 Icon 与 LoadingSpinner。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| theme | `Theme` | `Theme.default` | 语义色, default 为中性灰按钮 |
| variant | `ButtonVariant` | `ButtonVariant.base` | `base` 实底 / `outline` 描边 / `text` 纯文字 |
| size | `Size` | `Size.medium` | 高度 24/32/40px |
| shape | `ButtonShape` | `ButtonShape.rectangle` | `rectangle` / `square` 图标按钮 (宽=高) / `round` 胶囊 |
| block | `bool` | `false` | 撑满所在布局的剩余宽度 |
| text | `string` | `""` | 按钮文字, 空串不渲染文本 |
| icon | `TIconData` | 空 | 前置图标, loading 时被转圈替代 |
| loading | `bool` | `false` | 显示旋转指示并禁用点击 |
| disabled | `bool` | `false` | 禁用态 |

回调: `clicked()`。

相关枚举: `ButtonVariant { base, outline, text }`, `ButtonShape { rectangle, square, round }`。

### Link

超链接文字。默认悬停时显示下划线, `underline` 为 true 时常显。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| theme | `Theme` | `Theme.default` | default 使用链接色令牌, 其余用对应色阶 |
| text | `string` | `""` | 链接文字 |
| icon | `TIconData` | 空 | 前置图标 |
| disabled | `bool` | `false` | 禁用态, 显示禁用灰并隐藏下划线 |
| underline | `bool` | `false` | 常显下划线 (默认仅悬停显示) |
| size | `Size` | `Size.medium` | 字号随 Size 缩放 |

回调: `clicked()`。

### Icon

通用图标渲染器。数据驱动, 与 lucide-slint 相同的几何组织方式。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| icon | `TIconData` | 空 | `TIconSet.Xxx` 或自定义字形 |
| icon-color | `brush` | `TelaTheme.text-color-primary` | 线条与填充颜色 |
| size | `length` (in-out) | `16px` | 渲染尺寸 |
| stroke-width | `float` (in-out) | `2.0` | 24 网格单位的线宽, 随 size 缩放 |
| absolute-stroke-width | `bool` | `false` | 为 true 时 stroke-width 按绝对像素 |

只读输出: `computed-stroke-width` (`length`, 实际线宽)。

### Heading / Paragraph / Caption

排版组件, 均继承 `Text`, 保留 `text`, `color`, `wrap` 等全部内建属性。

| 组件 | 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| Heading | level | `int` | `3` | 1..5 对应 36/28/24/20/18px, 半粗字重 |
| Paragraph | secondary | `bool` | `false` | true 时用次级文本灰 |
| Caption | (无新增参数) | — | — | 12px 次级灰, 常用于辅助说明 |

三者的默认颜色为 `text-color-primary` (Heading/Paragraph) 与
`text-color-secondary` (Caption), Paragraph 与 Caption 默认 `wrap: word-wrap`。

### Divider

分割线。水平带文字或纯线, 亦可作 1px 竖线。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| orientation | `Orientation` (Slint 内建) | `Orientation.horizontal` | `vertical` 时渲染为竖线 |
| text | `string` | `""` | 非空时渲染为带文字分割线 |
| align | `DividerAlign` | `DividerAlign.center` | 文字位置: `left` / `center` / `right` |
| line-color | `color` | `TelaTheme.border-level-1-color` | 线色 |
| text-color | `color` | `TelaTheme.text-color-secondary` | 文字色 |
| content-gap | `length` | `TelaTheme.margin-l` | 文字与线之间的间距 |
| inset | `length` | `0px` | 距容器边缘的留白 (仅水平方向) |

相关枚举: `DividerAlign { left, center, right }`。

### Tag

小标签。三种填充变体, 可带图标与关闭钮。

继承: `Rectangle`。内部组合 Icon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| theme | `Theme` | `Theme.default` | 语义色 |
| variant | `TagVariant` | `TagVariant.dark` | `dark` 实底 / `light` 浅色底 / `outline` 描边 |
| size | `Size` | `Size.medium` | 高度 20/24/28px |
| text | `string` | `""` | 标签文字 |
| icon | `TIconData` | 空 | 前置图标 |
| closable | `bool` | `false` | 显示关闭钮; 点击后触发 `closed()` 并自设 `visible = false` |
| disabled | `bool` | `false` | 禁用态 |
| clickable | `bool` | `false` | 启用悬停高亮与 `clicked()` |

回调: `closed()`, `clicked()`。

相关枚举: `TagVariant { dark, light, outline }`。

### Loading / LoadingSpinner

加载指示。`Loading` 是带文字标签的完整组件, `LoadingSpinner` 是裸转圈
(Button 内部也使用它)。

继承: 均为 `Rectangle`。Loading 内部组合 LoadingSpinner。

| 组件 | 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- | --- |
| Loading | text | `string` | `""` | 空串时只显示转圈 |
| Loading | theme | `Theme` | `Theme.primary` | 圈的颜色 |
| Loading | size | `length` (in-out) | `24px` | 圈直径 |
| LoadingSpinner | spinner-color | `brush` | `TelaTheme.brand-color` | 圈的颜色 |
| LoadingSpinner | size | `length` (in-out) | `16px` | 圈直径 |
| LoadingSpinner | stroke-width | `float` (in-out) | `2.0` | 24 网格单位线宽 |
| LoadingSpinner | absolute-stroke-width | `bool` | `false` | 按绝对像素解释线宽 |

### BorderlessWindow

无边框应用窗口。自绘标题栏 (拖拽热区, 最小化/最大化/关闭), 圆角与边缘缩放。
用作应用窗口根, 子元素即窗口内容。

继承: `Window` (保留 `title`, `icon`, `minimized`, `maximized` 等内建属性)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| window-background | `brush` | `TelaTheme.bg-color-page` | 圆角窗口表面色; 不要设置 Window 自身的 `background`, 那会把圆角涂成不透明 |
| window-radius | `length` | `8px` | 窗口圆角, 最大化时归零 |
| resize-border | `length` | `8px` | 边缘拖拽缩放热区宽度, 最大化时归零 |
| header-height | `length` | `40px` | 标题栏高度 |
| header-background | `brush` | `TelaTheme.bg-color-container` | 标题栏底色 |
| logo | `TIconData` | 空 | 标题前的品牌图标 |
| maximizable | `bool` | `true` | 为 false 时隐藏最大化按钮 (工具对话框等) |
| header-title-visible | `bool` | `true` | 隐藏标题文字 (任务栏 title 不受影响) |

内部组合: 私有 WindowButton, Icon 与内建 `WindowMoveArea` (拖拽热区)。

---

## 常用组件 common

```slint
import { Input, TextArea, Select, CheckBox, Radio, RadioGroup, Switch, Slider, Progress, Avatar, Badge, Card, Alert, Dialog, Drawer, MessageOverlay, TelaMessage, CodeBlock, Tabs, Pagination, NavBar, SideNav, Breadcrumb, Steps } from "@tela-ui/common.slint";
```

### Input / TextArea

输入框。共享私有基类 `InputFrame`, 边框随状态着色, 点击边框聚焦内部 TextInput。

继承: `InputFrame` (私有, 继承 `Rectangle`)。

Input 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| size | `Size` | `Size.medium` | 高度与字号 |
| placeholder-text | `string` | `""` | 占位文字 |
| text | `string` (in-out) | `""` | 输入内容, 双向绑定到内部 TextInput |
| read-only | `bool` (in-out) | `false` | 只读 |
| prefix-icon | `TIconData` | 空 | 前缀图标 |
| suffix-icon | `TIconData` | 空 | 后缀图标 (显示清空钮时被隐藏) |
| clearable | `bool` | `false` | 悬停且非空时显示清空钮 |
| input-type | `InputType` (Slint 内建) | `InputType.text` | `password` 等输入类型 |

TextArea 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| placeholder-text | `string` | `""` | 占位文字 |
| text | `string` (in-out) | `""` | 输入内容, 自动换行 |
| read-only | `bool` (in-out) | `false` | 只读 |
| rows | `int` | `4` | 期望可见行数, 高度随之计算 |

继承自 InputFrame 的参数 (两者通用):

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| status | `InputStatus` | `InputStatus.default` | 边框色: `default` / `success` / `warning` / `error` |
| disabled | `bool` | `false` | 禁用态 |
| radius | `length` | `TelaTheme.radius-default` | 圆角 (TextArea 固定为 `radius-medium`) |

回调: Input `accepted(string)` (回车), `changed(string)` (每次输入), `cleared()`;
TextArea `changed(string)`。

只读输出 (来自 InputFrame): `frame-hovered` (`bool`, 边框悬停态)。

相关枚举: `InputStatus { default, success, warning, error }`。

### Select

下拉选择。弹层锚定在触发器下方, 宽度随触发器 (至少 120px), 最多显示 6 项;
暂无键盘导航。

继承: `Rectangle`。内部组合 Icon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[SelectItem]` | `[]` | 选项列表 |
| current-index | `int` (in-out) | `-1` | 选中项下标, -1 为未选择 |
| current-value | `string` (in-out) | `""` | 选中项 value, 选择时组件内部一并写入 |
| placeholder-text | `string` | `""` | 未选择时的占位文字 |
| disabled | `bool` | `false` | 禁用态 |
| prefix-icon | `TIconData` | 空 | 前缀图标 |
| size | `Size` | `Size.medium` | 高度与字号 |

回调: `selected(int, string)` — 参数为 (下标, value)。

相关结构: `SelectItem { label: string, value: string, disabled: bool }`。

### CheckBox

复选框, 支持半选。

继承: `Rectangle`。内部组合 Icon (对勾/横线)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` | `""` | 标签文字 |
| checked | `bool` (in-out) | `false` | 选中态, 点击时组件自行翻转 |
| indeterminate | `bool` | `false` | 半选样式, 显示上优先于 checked |
| disabled | `bool` | `false` | 禁用态 |

回调: `changed(bool)` — 翻转后的选中值。

### Radio / RadioGroup

单选。`Radio` 是单个圆钮, `RadioGroup` 将一组选项排成一行并持有选中值。

继承: `Radio` 为 `Rectangle`; `RadioGroup` 为 `HorizontalLayout`。RadioGroup 内部
组合 Radio。

Radio 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` | `""` | 标签文字 |
| selected | `bool` | `false` | 选中态, 选中逻辑由调用方在 `clicked` 中处理 |
| disabled | `bool` | `false` | 禁用态 |

RadioGroup 参数:

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[RadioItem]` | `[]` | 选项列表 |
| value | `string` (in-out) | `""` | 选中的 value |
| disabled | `bool` | `false` | 整组禁用 |

回调: Radio `clicked()`; RadioGroup `changed(string)`。

相关结构: `RadioItem { text: string, value: string }`。

注意: RadioGroup 点击时会自赋值 `value`, 这会永久断开外部对 `value` 的绑定。
选中态需要持续受外部数据源控制时 (例如绑定主题偏好), 改用独立 `Radio` 并自绑
`selected` 与 `clicked`。

### Switch

开关。small 36x18 / medium 40x22 / large 48x24。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| checked | `bool` (in-out) | `false` | 开关态, 点击时组件自行翻转 |
| disabled | `bool` | `false` | 禁用态 |
| size | `Size` | `Size.medium` | 控件尺寸 |

回调: `changed(bool)` — 翻转后的开关值。

### Slider

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

### Progress

进度条。线性或圆环, value 变化带缓动动画。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| variant | `ProgressVariant` | `ProgressVariant.line` | `line` / `circle` |
| value | `float` | `0` | 0..100 |
| theme | `Theme` | `Theme.primary` | 进度色 |
| size | `Size` | `Size.medium` | 线粗 4/6/10px, 圆环直径 64/80/96px |
| show-label | `bool` | `true` | 是否显示百分比文字 |

相关枚举: `ProgressVariant { line, circle }`。

### Avatar

头像。图片, 文字或默认人形图标的圆形/圆角方形占位。

继承: `Rectangle`。内部组合 Icon (默认人形)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| size | `AvatarSize` | `AvatarSize.medium` | `small` 32 / `medium` 40 / `large` 48 / `extra-large` 64px |
| text | `string` | `""` | 无图时显示的文字, 字号按直径自适应 |
| source | `image` | 空 | 头像图片, 存在时优先显示 (cover 裁剪) |
| shape | `AvatarShape` | `AvatarShape.circle` | `circle` / `square` |
| theme | `Theme` | `Theme.primary` | 文字/默认图标的颜色与浅色底 |

显示优先级: `source` > `text` > 默认图标。

相关枚举: `AvatarSize { small, medium, large, extra-large }`,
`AvatarShape { circle, square }`。

### Badge

徽标。包裹在默认子元素右上角的计数胶囊或红点。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| count | `int` | `0` | 计数 |
| dot | `bool` | `false` | 红点模式, 不显示数字 |
| max-count | `int` | `99` | 超出显示 `{max-count}+` |
| badge-color | `color` | `TelaTheme.error-color` | 徽标底色 |
| show-zero | `bool` | `false` | count 为 0 时仍然显示 |

插槽: 默认子元素 (`@children`) 为被标记的内容。

### Card

卡片容器。可选标题/副标题头部与带分隔线的页脚。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| title | `string` | `""` | 标题 |
| subtitle | `string` | `""` | 副标题 |
| footer | `string` | `""` | 页脚文字, 上方带 1px 分隔线 |
| bordered | `bool` | `true` | 是否描边 |
| shadowed | `bool` | `false` | 是否投影 (shadow-1) |
| card-padding | `length` | `TelaTheme.margin-l` | 内边距 |

插槽: 默认子元素为卡片主体。

### Alert

警示条。整条浅色底, 图标与标题着语义色, 描述用主文本色。

继承: `Rectangle`。内部组合 Icon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| theme | `Theme` | `Theme.default` | 语义色, 决定底色/图标/标题 |
| title | `string` | `""` | 标题 (语义色, 半粗) |
| description | `string` | `""` | 描述文字, 空串时单行布局 |
| closable | `bool` | `false` | 显示关闭钮; 点击后自设 `open = false` 并触发 `closed()` |
| open | `bool` (in-out) | `true` | 显示/隐藏 |

回调: `closed()`。

### Dialog

模态对话框。渲染半透明遮罩与居中卡片, 需放置在窗口级覆盖层中。

继承: `Rectangle`。内部组合 Button (取消/确认)。

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

### Drawer

抽屉面板。从指定边缘滑入, 覆盖在半透明遮罩上; 面板常驻, 通过透明度与位移动画
出入场, 关闭动画不会被裁断。

继承: `Rectangle`。内部组合 Icon (关闭钮)。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| open | `bool` (in-out) | `false` | 显示/隐藏 |
| title | `string` | `""` | 面板标题 |
| placement | `DrawerPlacement` | `DrawerPlacement.right` | `top` / `right` / `bottom` / `left` |
| drawer-width | `length` | `320px` | 面板宽度 (left/right 方向使用) |
| drawer-height | `length` | `280px` | 面板高度 (top/bottom 方向使用) |
| close-on-overlay | `bool` | `true` | 点击遮罩是否关闭 |

回调: `closed()`。公开函数: `close()`。

插槽: 默认子元素为标题下方的面板主体。

### Message (TelaMessage + MessageOverlay)

全局消息。任何代码 (Slint 或 Rust) 调用 `TelaMessage` 的函数弹消息, 窗口根部放
一个 `MessageOverlay` 负责渲染; 3 秒后自动隐藏, 无 TouchArea, 不拦截输入。

`TelaMessage` 全局属性:

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| text | `string` (in-out) | `""` | 消息文字 |
| theme | `MessageTheme` (in-out) | `MessageTheme.info` | 消息类型, 决定图标与颜色 |
| visible | `bool` (in-out) | `false` | 显示状态, 由 show 函数与 Overlay 定时器维护 |
| placement | `MessagePlacement` (in-out) | `MessagePlacement.top-center` | 贴靠的窗口角落 |

`TelaMessage` 函数: `info(string)`, `success(string)`, `warning(string)`,
`error(string)`, 以及底层 `show(string, MessageTheme)`。

`MessageOverlay` 组件无参数, 放置即可。

相关枚举: `MessageTheme { info, success, warning, error }`,
`MessagePlacement { top-left, top-center, top-right, bottom-left, bottom-center, bottom-right }`。

### Tabs

标签页头。下划线随选中项平滑移动, 各项等宽; 页面内容由宿主用
`if current-index == i` 切换渲染。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[string]` | `[]` | 标签文字 |
| current-index | `int` (in-out) | `0` | 选中下标 |

回调: `changed(int)` — 新选中的下标。

### Pagination

分页器。上一页/下一页与最多 7 个页码的滑动窗口。

继承: `Rectangle`。内部使用私有 PageBtn 与 Icon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| total-pages | `int` | `1` | 总页数 |
| current | `int` (in-out) | `1` | 当前页, 从 1 起 |

回调: `changed(int)` — 新的页码。

### NavBar

顶部导航栏。左侧品牌 (logo + 标题), 中间菜单项 (选中项带下划线), 右侧操作区。

继承: `Rectangle`。内部组合 Icon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| title | `string` | `""` | 品牌标题 |
| logo | `TIconData` | 空 | 品牌图标 |
| items | `[NavItem]` | `[]` | 菜单项 |
| current-value | `string` (in-out) | `""` | 选中的 value |
| bordered | `bool` | `true` | 底部 1px 分隔线 |

回调: `changed(string)` — 新选中的 value。

相关结构: `NavItem { label: string, value: string }`。

插槽: 默认子元素为右侧操作区 (已垂直居中, 无需自行定位)。

### SideNav

可折叠侧边导航。品牌头部, 垂直菜单与钉在底部的页脚区; 折叠时图标+文字行收缩为
居中图标, 宽度在 232px 与 64px 间动画过渡。

继承: `Rectangle`。内部组合私有 NavRow 与 Icon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| title | `string` | `""` | 品牌标题 |
| logo | `TIconData` | 空 | 品牌图标 |
| items | `[SideNavItem]` | `[]` | 菜单项 |
| current-value | `string` (in-out) | `""` | 选中的 value (菜单项与带 value 的页脚项共用) |
| collapsed | `bool` (in-out) | `false` | 折叠态, 收起行与顶部按钮均可翻转 |
| mode | `SideNavMode` | `SideNavMode.icon-text` | `icon-text` 可折叠 / `text` 纯文字 (恒 232px) / `icon` 纯图标 (恒 64px) |
| toggle-position | `SideNavTogglePosition` | `SideNavTogglePosition.top` | `top` 头部右缘 / `bottom` 底部收起行 |
| bordered | `bool` | `true` | 右侧 1px 分隔线 |
| footer-items | `[SideNavFooterItem]` | `[]` | 钉在底部的行, 位于收起行之上 |

回调: `changed(string)` (菜单或带 value 的页脚项), `footer-clicked(SideNavFooterItem)`
(无 value 的页脚项, 纯动作)。

相关结构:

- `SideNavItem { label: string, value: string, icon: TIconData = { paths: [] }, disabled: bool = false }`
- `SideNavFooterItem { label: string = "", icon: TIconData = { paths: [] }, value: string = "" }`
  — `value` 非空时行为等同菜单行 (参与选中并触发 `changed`), 为空时是纯按钮并触发
  `footer-clicked`。

相关枚举: `SideNavMode { icon-text, text, icon }`,
`SideNavTogglePosition { top, bottom }`。

### Breadcrumb

面包屑。非当前项渲染为链接色并触发 `changed`, 当前项为主文本色。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[CrumbItem]` | `[]` | 层级项 |
| current-value | `string` (in-out) | 最后一项的 value | 当前层级 value |
| separator | `string` | `"/"` | 分隔符 |
| separator-color | `color` | `TelaTheme.text-color-disabled` | 分隔符颜色 |

回调: `changed(string)` — 点击的非当前项 value。

相关结构: `CrumbItem { label: string, value: string }`。

### Steps

步骤条。等宽分栏, `current` 之前的步骤显示对勾, 之后的显示序号。

继承: `Rectangle`。内部组合 Icon。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| items | `[string]` | `[]` | 步骤名 |
| current | `int` | `0` | 当前步骤下标, 之前的步骤计为已完成 |

### CodeBlock

代码块。主题化底色的等宽字体面板 (固定 Consolas), 不换行, 高度随内容自适应。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| code | `string` | `""` | 代码文本 |

---

## 实验性组件 experimental

API 不稳定, 可能随版本调整或移除。

```slint
import { Skeleton } from "@tela-ui/experimental.slint";
```

### Skeleton

加载占位块。透明度呼吸动画; 通过 `width`/`height` 自定形状。

继承: `Rectangle`。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| animated | `bool` | `true` | 呼吸动画开关 |
| circle | `bool` | `false` | 渲染为圆形 (如头像占位) |

---

## 全局对象

### TelaTheme (主题令牌)

全局单例, 定义在 `@tela-ui/theme.slint`。承载完整 TDesign 令牌集并提供亮/暗双预设。

| 属性 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| preference | `TelaThemePreference` (in-out) | `TelaThemePreference.system` | 用户意图: 跟随系统/强制亮/强制暗 |
| is-dark | `bool` (out) | — | 实际生效模式 (`system` 对 OS 色彩方案解析后的结果); 亮暗开关绑定到它而非 preference |
| theme | `TAppThemeConfig` (in-out) | 亮暗随 is-dark 切换 | 当前生效配置, 直接赋值可完全接管 |
| light / dark | `TAppThemeConfig` (in-out) | 内置预设 | 两个模式预设, 可逐字段覆盖 |

`TelaThemePreference { system, light, dark }`。

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
| `theme-scheme(t: Theme)` | `TPrimaryColor` | 语义角色对应的色阶, default 解析为品牌色阶 |
| `theme-color(t)` / `-hover` / `-active` / `-light` / `-light-hover` / `-disabled` | `color` | 语义色的各状态; default 保持中性灰语义 |
| `size-height(s: Size)` | `length` | 控件高度 24/32/40px |
| `size-font(s: Size)` | `length` | 字号 12/14/16px |
| `size-padding(s: Size)` | `length` | 水平内边距 8/12/16px |

### TIconSet (图标数据)

全局图标库, 定义在 `@tela-ui/base/icon/data.slint` (经 `@tela-ui/base.slint` 再导出)。
纯数据文件, 每个图标是 24x24 网格上的 SVG 路径集合, 由共享的 `Icon` 组件渲染。
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

- `Theme { default, primary, success, warning, error }` — 语义色角色。
- `Size { small, medium, large }` — 标准控件尺寸。

### 各组件自带的枚举与结构

| 名称 | 归属组件 | 取值/字段 |
| --- | --- | --- |
| ButtonVariant / ButtonShape | Button | base, outline, text / rectangle, square, round |
| TagVariant | Tag | dark, light, outline |
| DividerAlign | Divider | left, center, right |
| InputStatus | Input, TextArea | default, success, warning, error |
| SelectItem | Select | label, value, disabled |
| RadioItem | Radio, RadioGroup | text, value |
| ProgressVariant | Progress | line, circle |
| AvatarSize / AvatarShape | Avatar | small, medium, large, extra-large / circle, square |
| DrawerPlacement | Drawer | top, right, bottom, left |
| MessageTheme / MessagePlacement | Message | info, success, warning, error / 六向贴靠 |
| NavItem | NavBar | label, value |
| SideNavMode / SideNavTogglePosition | SideNav | icon-text, text, icon / top, bottom |
| SideNavItem | SideNav | label, value, icon, disabled |
| SideNavFooterItem | SideNav | label, icon, value |
| CrumbItem | Breadcrumb | label, value |
| TelaThemePreference | TelaTheme | system, light, dark |
| TIconPath / TIconData | Icon | command, has_fill / paths |
