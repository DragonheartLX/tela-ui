# Slint 1.18 语义实证

> 每条都对着锁定版本源码或本仓库生成产物核对过, 不是猜测。
> 引用时写 `docs/slint-semantics.md` 第 N 条。

1. **Rectangle 不消费指针事件** — `input_event` 返回 `EventIgnored`, 遍历是 FrontToBack 且 `EventIgnored => 继续`, 所以模态卡片自身不含 TouchArea 时点正文会落到背景遮罩, 面板内要放一个空白 `TouchArea {}` 当吸收层。

2. **`visible` 与 `animate` 互斥** — `visible: false` 只是不渲染但元素仍实例化、动画照跑, 所以退场动画永远看不到; 退场只能靠 `opacity`, 需要真正销毁用 `if`。

3. **整份模型替换 ≠ 单行写入** — 赋值整个数组走 `RepeaterTracker::reset()` 重建实例使私有属性归零, `items[i] = x` 走 `row_changed` 就地更新保留实例与私有属性; 依赖槽位私有状态的组件必须自己带 stamp 并在 `changed` 时 restart。

4. **`Timer.running` 默认 `true`** — 官方警告常驻耗电; `restart()` 只重启此前已启动过的计时器, 所以改成 `running: false` 后必须用 `start()`, 否则自动隐藏会失效。

5. **无障碍属性只对 Text 与 TextInput 自动推导** — `Rectangle` 派生组件拿不到 `accessible-enabled`, 而后端仅在取到 Enabled 且 ≠ `"true"` 时才标记禁用; 不显式写的组件禁用态对读屏不可见, 且 `accessible-action-default` 会在 disabled/loading 时照样触发, 动作回调里必须自带守卫。

6. **`to-float()` 解析失败或空串 = 0** — 数字输入框清空会被夹到 min 并触发 `changed`。

7. **`Text` 默认 `overflow: clip` 与 `wrap: no-wrap`** — 要省略必须显式 `elide`, 要换行必须显式 `word-wrap`。

8. **赋值会移除绑定** — 不要对宿主可能绑定的属性做命令式赋值(典型是组件内写 `root.visible = false`), 否则宿主的绑定被永久销毁; 复用组件不要替宿主决定可见性。

9. **`a <=> b` 的初值取右侧** — `in` 属性也可以双向绑定; 组件内不写 `in/out/private` 的 `property` 默认就是私有。

10. **带 `animate` 的绑定首次求值不跑动画** — `AnimatedBindingCallable` 初始态 NotAnimating 时直接赋值(`properties_animations.rs:380`), 动画只在依赖变化后启动; "组件挂载/切页会全量触发动画"的推断不成立(TCarousel 探针实证: 挂载后两帧 diff = 0)。

11. **`animate` 支持 `enabled:` 条件关断** — `animate x { enabled: <bool表达式>; }`(`PropertyAnimation.enabled`, `items.rs:1238`), 关断时当帧直达目标值; 要"索引变化才动画、几何变化即时跟随"就把动画放在标量镜像属性上、让几何属性非动画地引用它(TCarousel 的 `slide-pos` 即此法, 动画写在 `x` 上会让 resize 重放整段滑动)。

12. **Timer 是布局不可见元素** — 可直接作为布局子元素, 不占槽位、不触发 spacing(探针实测: spacing 28 下 红40+Timer+绿40 总高 108 = 单份 spacing); 旧说法"Timer 不能直接放布局、要用 0 高 Rectangle 包装"不成立, 包装反而多出一份 spacing 与 28px 内容高度。

13. **未设 `font-family` 的 `Text` 继承宿主窗口默认字体** — `resolved_font_request` 在局部字体为空时沿父链找 `Window.default-font-family`(`items.rs:1443`), 仍为空则落到渲染器通用族 (`sharedfontique.rs:188` 的 SansSerif/SystemUi, 本机解析为 Arial); 实测同一页面里裸 `Text` 与 `font-family: TTheme.font-family` 的像素差 4302/26880, 不是同一字体。

14. **`line-height-factor` 是"相对自然行盒"的系数, 不是 px** — 单行文本的系数只改首选高度, 固定高容器里的字形位置不受影响(探针: 20px 文本在 20/22/24px 盒里, 系数 1 与 1.16 的墨迹行列完全一致); Microsoft YaHei 自然行盒 = `ceil(字号 × 1.3198)`(10/12/13/14/15/16/18/20/21/24/28/30/36px 十三个字号实测全中)。换算由 `TText` 自己按 `line-height ÷ ceil(self.font-size × 1.3198)` 完成, 站点只给 `line-height` 令牌。

15. **无障碍声明只落在显式写了 `accessible-role` 的元素上** — role 必须是常量表达式(`lower_accessibility.rs:32`), 其余 accessible-* 可动态绑定; `accessible-value` 绑 float 自动转 string; 生成代码里枚举变体带 `r#` 前缀, 特征串要写 `AccessibleStringProperty :: r#Label` 形态, 普通匹配会漏统计。

16. **布局里的 `Text` 不会纵向收缩** — 官方只承诺 `wrap: word-wrap` / `overflow: elide` 把 `min-width` 降到 0, `preferred-height` 与 `min-height` 仍是整段文本的高度(`builtin_elements.rs:1170-1175`); 探针实测在 `VerticalLayout` 里给 `max-height` 也无效(隐式尺寸元素自己重算尺寸), 溢出照旧画到父盒外(TCollapse 换行正文、TDialog 长正文都靠"`clip: true` 包装盒 + 显式高度"封顶, 见 `collapse.slint` 的 `content-wrap`)。

17. **`pure function` 让编译器去重属性读取** — `passes/deduplicate_property_read.rs` 用 `purity_check` 判定, 标 `pure` 的函数体可被合并; `pure` 允许读自身属性(实测 `pure function col-of(i: int) -> int { return Math.mod(i, self.cols); }` 通过 `--check`), 所以"每个单元格的 x/y 绑定各调一次"的重复读加 `pure` 即可, 不必把依赖摊成参数。

18. **`overflow: elide` 不改 `preferred-width`** — 宿主拿 `preferred-width` 量行宽与指示条时(如 TNavBar / TCard 的 `label-width`), 加省略只影响绘制: 探针里 200px 盒的 clip 版最后一个字形顶到 x=199, elide 版收在 x=195 且尾部是三个稀疏点。

19. **`animate` 的参数在同一"帧"内改不生效** — 动画的 `enabled` / `duration` 由动画在重新武装时读取(`properties_animations.rs:171` 的 `!enabled` 快路径直接返回目标值), 所以"先写开关、再写值、中间没有任何一帧被求值"时那次变化不缓动; 隔一帧(哪怕只是先渲染一次)就正常。探针实测: 开关开+值同 tick 写 → 当帧满格; 隔一帧 → 0.06→0.24→1.00; 点击开关(天然隔帧)→ 正常缓动。`enabled:` 与 `duration:` 两种写法表现一致, 换写法绕不开。

20. **`visible: false` 不改变布局占用** (第 2 条"不渲染但仍实例化"的补充) — 编译器把它降级成一个 `is-visibility-clip` 的 `Clip`(`passes/visible.rs:128`), 重复元素甚至被注入成实例根; 探针实测 `VerticalLayout` spacing 12 里 红40 + 隐藏绿40 + 蓝40 的蓝条从 y=104 起, 隐藏行与它的 spacing 全在。要"整行消失"只能自己塌陷高度(`height: hit ? h : 0px` + `clip: true`), 并让容器 `spacing: 0px`、把行距烘进行高 —— 布局 spacing 对 0 高子元素照样生效。

21. **1.18 语言内没有循环语句, 数组只有 `length`/`push`/`remove`/`insert`** — `index-of`/`any`/`all`/`find-index` 在 `lookup.rs:1336-1341` 存在但挂在 `experimental_lookup_enabled()` 后面, 稳定模式拿不到; 函数也不能递归(`passes` 直接报 binding loop: root.fn -> root.fn)。所以"按条件过滤模型 / 找下一个匹配项"无法在 .slint 里扫描, 只能用元素树表达(命令面板即: 每行自己塌陷 + 每行在关键字变化时上报自己的下标, 取最小者 = 第一条匹配项, 与触发顺序无关)。

22. **内容尺寸的容器用自身尺寸定位时, 内容会画在第一轮的绝对坐标上** — 容器(布局或组件)没写 `width`、靠内容撑开, `x` 又读 `self.width`(右对齐 `parent.width - self.width - 8px`、居中 `(parent.width - self.width) / 2`)时, 第一轮 `self.width` 还没算出来(暂等于父宽), 容器的定位置于 x≈-8 并被画在那个位置, 之后只有部分子元素被修正, 表现为"文本跑到窗口左缘、同级矩形还在正确位置"。探针实测三种: 居中布局里的红字画在 x 2..35(应 ~590); 右对齐布局里文本画在 x 4..22(应 ~1112)而同一布局的 40px 矩形落在正确的 1152..1191; 矩形在前文本在后时两者都停在 x≈-8。skia 与软件渲染器一致, 不是渲染器问题。**只在布局落定前就开始绘制时出现**: 用 `slint-viewer --screenshot`(单帧)必现, 而先 `mock_elapsed_time` + 派发 `Resized` 再截图的探针没重现(那一帧里布局已定, 位置是对的)。修法: 给容器显式宽度或固定 `x`, 或者把元素用 `x`/`y` 自己定位(实测 `TText { x: parent.width - self.width - 8px; }` 位置正确)。TImageViewerOverlay 的右上角计数即因此从"内容尺寸布局"改为直接定位。

23. **显式 `width` 绑定压过 `horizontal-stretch`** — 子元素自己写了 `width`(哪怕只是绑定到常量)时, 布局分配剩余空间不会给它, 它就拿绑定值; 交叉轴上 `horizontal-stretch` 本来也不生效(VerticalLayout 的横轴是交叉轴, 那里只有没写 `width` 的子元素才铺满)。探针实测(1000px 宽的 HorizontalLayout 里): 无 `width` 的 Rectangle 加 stretch → 铺满 16..1005; `width: 100px` 加 stretch → 仍是 16..115; `TButton { block: true }`(内部是 `width: max(h, 内容宽 + 内边距)` 加 `horizontal-stretch: block ? 1 : 0`)→ 只有 52px, 后面的 10px 标记元素紧跟着落在 x=68。所以 `block` 从未真正生效, 参数已删 (要整行按钮就在外层布局里撑满), 动作面板的取消行也是自绘整行。

24. **TouchArea 的点击判定把拖拽也算一次点击** — 双击检测只看"两次 press 间隔 < 500ms 且位移 < 10px"的计数, 中间发生过多少次 `moved` 不参与判定, 所以**两次快速滑动会凑成一次 `double-clicked`**。探针实测: 连续两次 800px 拖拽后 `double-clicked` 触发, 视口缩放从 2x 回到 1x; 探针改用每次释放后推进 600ms 的拖拽才稳定。给"拖拽"和"双击"同时挂处理器时, 双击分支必须自己用私有 flag 排除拖拽(图片预览即 `if (!root.dragged)`)。

25. **std `ScrollView` 的内层 `Flickable` 不认祖先的输入闸** — `ScrollView.enabled` 只管滚动条, 滚轮由内核 `Flickable` 自己吃(`flickable.rs` 的 `MouseEvent::Wheel` 分支), 而 `ScrollView` 没有暴露 `interactive`; 于是"整个覆盖层 `opacity: 0`"的关闭态里, 面板上的那块 `ScrollView` 仍是一个滚轮黑洞(探针: 关着命令面板时窗口中心 100 格里有 16 格滚轮被吞, 卸载该 overlay 后 0 格)。阻断办法只有 `visible: false`(它降级的 visibility Clip 会拦输入, 见第 2/20 条), 且要等淡出动画走完再置 false, 否则列表会先于卡片消失; 裸 `Flickable` 则可以自己写 `interactive: false`(内核文档: "all events are forwarded to elements underneath")。

26. **布局外的子元素默认在父级居中** — 没写 `x`/`y` 的子元素由 `passes/default_geometry.rs` 的 `maybe_center_in_parent` 按 `(父尺寸 - 自身尺寸) / 2` 定位, 不是左上角; 只有撑满父级 (默认 100%) 的那一轴不居中。实测 200x400 里的 40x40 红块落在 80..120 / 180..220; TTable 的表头 48px 矩形因此画在 y=176 并令表头点击全部落空, 显式 `y: 0` 后恢复。整屏覆盖层、整行条带这类非布局子元素必须显式给 `x`/`y`。

27. **ListView 只自动算 `content-height`, 横向内容宽要自己给** — `Flickable` 对 `for` 循环生成的内容不会自动求 `content-width`(源码注明 issue #407), ListView 内部只维护纵向的 `content-height`; 要横向滚动就把 `content-width` 设成内容的固有宽度, 行元素显式给同一宽度, 表头绑 `content-x` 跟随。实测 TTable 1200px 固定列在 600px 视口里可横向滚动且表头同步, 纵向滚动与表头纵向固定不受影响。

28. **ListView 的 `content-y` 归内部 Flickable 所有** — 外部给 `content-y` 写绑定或命令式赋值都留不住: 探针在 TTimeColumn 的 `init` 写入 192px/864px(视口高已就绪), 首帧后两条列都回到 0, 绑定也被那次内部写入解除; 换成 `Timer` 在打开后写同样被下一次布局复位。要做"数据整体滚动 + 选中值居中"的时间列, 不要在外面拨 ListView 的 `content-y`: 直接渲染整列、自己维护偏移量并 `animate` 它(TTimePicker 即整列数据 + 行内选中框), 或只渲染可见行而不滚动。

29. **`for it[i] in model` 里写 `it.field = v` 会写回模型, 即使模型属性是 `in`** — 探针用一个 `in property <[MutRow]> rows` 加 `out property <bool> f0: rows[0].flag`, 点击行内 `TouchArea` 执行 `it.flag = !it.flag`: 点击前 `f0=false`, 点击后 `f0=true`, 说明循环变量对数组项是引用而非拷贝, 写入穿透到模型; 编译期也不把这种写入当"改 `in` 属性"拦下。所以扁平模型组件在回调里改行字段是可行的 —— TTree 最初"点了收不起来"不是写入被丢弃, 而是组件没有任何把子行藏起来的机制 (子行是否出现取决于宿主重建 `rows`), 最终改用嵌套声明 (见 `decisions.md`)。反过来, 依赖"改字段就能重绘"的说法未单独取像素证据, 需要时按本仓 `docs/harness.md` 的截图配方另测。

30. **实例化处的绑定覆写组件内部的根绑定** — 组件根元素自己写的 `border-width: 1px` / `background` / `radius` 等绑定, 在实例化处重写即被替换 (`TInput { border-width: 0px; background: transparent; radius: 0px; }` 实测边框消失, `slint-viewer --screenshot` 逐像素确认); 这也是 `TInputAdornment` 统一状态边框方案的前提。注意反向不成立: 组件根的绑定压过外层布局的 stretch 分配 (见第 23 条), 且根元素绑定里拿不到 `parent` (组件定义时没有实例上下文), "横轴绑 width、竖轴绑 height"这类静态差异只能拆成两个子类 (TSplitter 即此法, `vertical` 旗标由子类钉死)。

31. **布局的富余/缺额分配规则 (TSplitter / TInputAdornment 探针实证, 源码 `i-slint-core/layout.rs` 的 `adjust_items`)** — 三条一起用: (a) **显式 `width` 超容不压缩**: 子元素绑死 width 1000px 放 800px 布局里直接溢出 (`b3.x=1000`); 改绑 `preferred-width` + `min-width: 0` + `stretch: 0` 则富余时精确取 preferred、超容时被布局向 min 压回容器内 — TSplitter 主栏防溢出即此法 (`max-width: 100%` 不存在, 百分比只接受 width/height/preferred-*)。 (b) **总 stretch ≤ 0 时全员按 1 均分**: `actual_stretch` 的回退分支会把每个可增长子元素当 stretch 1 均分剩余空间 — 旁边的"占位"子元素内容自带 stretch 0 (如裸 Text) 时, 主元素的 `stretch: 0` 形同虚设, 必须给占位元素显式 `stretch: 1` 保住总和 (TSplitterPane 即此)。 (c) **给组件内层布局写 `width: parent.width` 会弄脏组件的首选尺寸**: 外层布局测量组件 preferred 要穿过根到子布局, 子布局宽度又绑回根宽, 成环后盒子 preferred 失效 — TInputAdornment 附着段曾因此被压到 0; 布局做根的直接子元素默认就撑满, 不要手写这组绑定。

32. **全定宽子元素的嵌套行会把拉伸列塌缩到首选宽 (TComment 探针实证, 六种变体全复现)** — VerticalLayout 拉伸列 (`horizontal-stretch: 1`, 根是 HorizontalLayout) 里放一个 HorizontalLayout 行, 只要该行的子元素 max 全部有界 (定宽 Rectangle、TLink 这类根上绑 width 的组件、定宽 TIcon), 整列宽度塌到行首选宽, 文本每四五个字换行。裸 `Text` 子元素 (含 `wrap: word-wrap` / `overflow: elide`, 其 layout-info max 无界) 不触发; 行首行尾无所谓, 换 Rectangle 根、给内层行补 `horizontal-stretch: 1`、用 Rectangle 包装行均**无效**。修法 = 行尾放一个空 `Rectangle { }` 吸收位 (gallery 各页横排的写法即此来源), 让行的 max 恢复无界。TComment 的操作行、TTabBar 的标签列 (TBadge 定宽 + TText 无宽) 均按此审查。

33. **整页实例化收尾有 10 轮预算, 超预算后"模型长度绑定/自身首选尺寸"类几何在首帧全部饿死 (第三批 gallery 实证)** — `window.rs` 的 `ensure_tree_instantiated` 最多跑 10 轮"物化条件/重复器 + 跑变更处理器", 之后只打一条 "long callback/instantiation chain" 并把剩余物化推迟。轻量窗口里一切正常的组件, 放进重页 (多个 ListView / 嵌套条件) 后: TSortable 的 `preferred-height: items.length × row-height` 首轮按空模型求值 (48px), 之后该重排不再发生; TTabBar 标签里 `y/height` 读自身 `preferred-height` 的内层 VerticalLayout 整个不渲染 (图标消失)。行与文本本身已物化 (init 计数=5), 纯粹是几何链断供。修法三条: 演示钉死固定视口高度 (TList/TTable 自第一批起的既定写法)、for 体内改固定几何 (TTabBar: 图标/文字按固定 y 定位, 不再内嵌读 preferred 的布局)、gallery 内容页只保留一个 TTable 实例 (第二个 ListView 恰好把页推过预算线, 删掉即 0 警告)。HEAD 在真实挂载下本就 3 警告, 该警告本身不是回归; 回归是"超得更多后连行都不渲染"。第四实例 (2026-10-25): 无显式高度的包装盒同样饿死 — TTabBar gallery 演示的 `Rectangle { width: 520px; }` 靠固定高子元素撑 preferred, 重页里塌成 8px (56px 标签栏被 clip 成 3px 蓝缝贴在卡片标题下), 浅层微探针正常、全页探针必现且不自愈, 与窗口高度无关 (用户在 720px 高窗口报障); 修法 = 包装盒显式 `height: bar.height + 2px` (2px 是描边厚度, bar 内缩 1px 避免盖住描边), 高度不经 preferred 链。

34. **HorizontalLayout 里固定高 TText 的显式 y 被忽略; `width: self.preferred-width` 钉宽后 y 生效 (TCell 探针实证)** — 布局行内给 TText 同时写 `height` + `y: (parent.height - self.height)/2` 时, 实测文本顶对齐 (y 未被应用); 给它补一个 `width: self.preferred-width` (内容宽自读, 无父级环) 后同一条 y 立即生效、精确居中。TIcon (TStrokedShape, 无默认宽高绑定) 的显式 y 则一直有效 (TList 行内图标的既定写法)。另注意: 试图用 `Rectangle { width: 内部文本.preferred-width }` 包装来居中且包装盒靠纵向拉伸时, 与第 22/31c 条同类成环, 元素会整个不渲染 — 不要这么包装。

35. **std-widgets 的 `Palette.color-scheme` 是模式偏好的唯一存储, 写入即接管 (2026-10-25 实证)** — fluent 风格里它 `in-out` 且双向别名到 `FluentPalette.color-scheme`, 后者的默认绑定链是 `ColorSchemeSelector → SlintInternal.color-scheme` (即 `SlintContext` 的颜色方案属性, 平台主题观察者实时推送)。外部写入 `Palette.color-scheme` 按赋值语义替换这条"跟随 OS"绑定: 之后 OS 切换不再传播进来, 要回"跟随系统"只能显式写 `unknown`。`unknown` 的解析是双面的: 标准库组件内部走 `dark-color-scheme` 的回退分支 (unknown → 再读 `SlintInternal`, 即按 OS 实时解析), 但 `AboutSlint` 的 logo 读**原始** `Palette.color-scheme`, unknown 下恒为亮色变体。另实测 `slint-viewer --screenshot` 单帧渲染里 OS 检测尚未落地, `Palette.color-scheme` 首帧是 `unknown` (要亮色参考图必须显式 `{"Palette": {"color-scheme": "light"}}`, 见 harness.md); 实机窗口则由 winit 解析出具体值。`SlintInternal` 命名空间仅在样式内部可见 (`expose_internal_types`), 用户 .slint 读不到 OS 真值; Rust 侧枚举路径是 `slint::private_unstable_api::re_exports::ColorScheme`。

36. **Flickable 在任何左键按下时无条件移除 content-x/y 上的绑定 (2026-10-25 实证, flickable.rs 的 "Stop animation by removing the binding")** — `handle_mouse_filter` 对可滚动的 Flickable 收到 Pressed 即 `content_y.remove_binding()` (items/flickable.rs:713-723); 触摸/滚动手势路径在 `phase != Ended` 时同样移除 (513-514)。推论: **挂在滚动偏移上的 `animate` 活不过视口内的第一次按压** — 点击锚点的那一下按压自己就先把 `animate content-y` 杀掉, 随后的写入全部瞬跳; 且外部经 `a <=> b` 别名链的写入与内部写入落在同一存储上, 哪边挂 animate 都一样死。缓动滚动因此必须由**组件自己的 Timer 逐帧普通赋值**实现 (TScrollBar / TAnchor / TBackTop 的 glide, 16ms × ease-out ≈ 200ms); Timer 写入是普通 set, 不受 remove_binding 影响。附带实证: .slint 的 Timer 元素生成 `TimerMode::Repeated` (无 `repeating` 属性, 单发要自己在 triggered 里停); `running` 绑定翻转经延迟求值的 changed 回调才启动定时器, mock 时间下需按帧步进泵时间, 否则错过武装窗口 (探针 settle 按帧泵)。

37. **Flickable 1.18 无 overscroll 信号, 且四条输入/尺寸规则决定下拉刷新的形态 (2026-10-25 源码 + 探针实证, items/flickable.rs)** — (a) 唯一回调是 `flicked`, 没有 overscroll/drag-beyond-edge 信号; (b) content-x/y 越界时 `in_bound_change_handler` 自动夹回 (且无绑定时直接 set), 所以拉动量不能借道 content-y>0, 必须放独立属性平移一个包装组 (TPullDownRefresh 的 pan); (c) `mouse-drag-pan-enabled: false` 时鼠标 Pressed 不被认领 (`accepts_pan_event`: Pressed 需 `interactive && (touch || mouse_drag_pan)`), 事件继续传给上层 TouchArea — 全幅手势层可行; 而 **Wheel 恒被 Flickable 在 filter 阶段拦截自处理**, 且 TouchArea 的 `scroll-event` 无处理器时默认 reject (input_items.rs), 所以手势层盖在 Flickable 上不挡滚轮。**带处理器时事件先到最上层 TouchArea**: accept 即停在 TouchArea (Flickable 看不见), reject 继续下穿到 Flickable — 顶部滚轮向上由手势层 accept 喂给拉动量, 其余 reject 穿透即靠此 (TPullDownRefresh)。**滚轮 delta 符号 (winit LineDelta × 60px/格, 真机注入实证): delta-y > 0 = 内容向上滚 (content-y 增大, 向顶), < 0 = 向下滚**; 顶部 delta-y > 0 会被 Flickable 的 in_bound 夹持原地吞掉, 这正是需要手势层 accept 的场景; (d) **未显式设置的 `content-height` 编译器默认 = max(直接布局子元素的 min-height, 自身高)** (compiler/passes/flickable.rs), repeater 不参与折叠 (issue #407) — repeater-only 的 body 馈不出内容高, 视口判为不可滚 (wheel 的受理条件 `content_height > height` 不成立), 显式绑定或钉死 content-height 是硬前提 (TPullDownRefresh 绑 root 属性, gallery 演示钉 12×48px)。

38. **struct 字段默认值经 "repeater 变量 → 子组件属性" 这条路径丢失 (2026-10-25 三跳隔离实证)** — 同组件内 `property <TS> a: { 省略字段 }` (单值与数组) 应用声明默认; 实例化处把数组字面量绑给子组件的 `in property <[TS]>` 再**直读下标**也正常; for 循环内**直读**循环变量字段仍正常; 唯独 `for it[i] in model: Child { field: it.field }` 把字段送进子组件属性时, 省略的字段以**零值变体**到达 (枚举 = 第一个变体; TSwipeAction.theme 省略 → `TThemeRole.default` 而非 primary, 探针像素实证)。bool 默认 false 的 struct 因此从未暴露 (零值恰好等于默认)。对策: 消费端把零值变体映射回期望值 (TSwipeActionButton 的 tint), 或要求宿主显式传全字段。

39. **条件重挂载窗口内 in_bound 夹持会吃掉恢复写入的滚动偏移; 塌缩轨迹还会污染"切换时保存"的值; 物化期内容高非单调 (2026-10-25 探针 + 真机 UIA 复盘)** — `if` 条件翻转页面的那几轮布局里, 视口内容高塌缩 (旧页已卸载, 新页 repeater 未物化), Flickable 的 `in_bound_change_handler` (第 37 条) 把恢复写入的负偏移判为越界并当帧夹回 0; 连裸 Flickable 直接写 `content-y = -800` 也会被夹回 0 (其自动 content-height 只取直接布局子的 min-height, for 循环体馈不出)。三条真机新实证 (mock 探针全部漏测, 因其塌缩轨迹/物化节奏与真机不同): **(i) 保存污染** — 塌缩夹持可以跑在 `changed group-value` 延迟回调之前, 回调读到的 content-y 已是中间高度的夹持值 (展示页 -1200 被中间高度 1999 夹到 -1047 后才保存); 对策 = 常驻节拍保存 (100ms), 存值永远早于点击, 切换落定窗口挂起; **(ii) 物化超窗** — 重页面 + UIA 客户端在场时物化慢于任何固定重申窗口 (内容页曾整页回到 0); 对策 = 适应式重申 (视口装得下目标值才断言); **(iii) 高度非单调** — 物化期 content-height 先涨后缩, 缩的一刻晚到的夹持把已恢复的偏移上提, 固定"两拍稳定"误判错误值为终态; 对策 = content-height 任何变化重置稳定计数。另实证: testing 后端的合成 `PointerScrolled` 事件与指针事件同族限制, 不会进入嵌套元素 (探针里裸可滚 Flickable 也收不到); **winit 真后端下 `Window::dispatch_event(PointerScrolled)` 走完整输入管线 (探针可驱动滚轮)**, 但仍测不出 UIA 客户端在场的物化拖慢 — 滚轮/夹持时序类结论要真机 (computer-use 驱动 + 临时诊断属性轮询日志) 才算数。

40. **TouchArea 的 `mouse-x`/`mouse-y` 相对元素自身, 元素自己移动时位移反馈会自激 (2026-10-25 真机 OS 鼠标探针实证)** — 每次 pointer 事件都按元素**当前**绝对位置换算 `mouse-x`; 手势层坐在会平移的内容里时, 指针不动而内容右移 offset, 下一次事件读到的 `mouse-x` 就小 offset。直接用 `mouse-x - pressed-x` 作拖拽位移, 偏移量递推成 `o_k = k - o_{k-1}`: 1px/步慢拖的真机帧序列实测 38→33→40→34→41→35…, 内容来回抽搐 (4px 轴锁开闸那一步从 0 跳到 5, 下一步被拽回 1)。`pressed-x` 是按下瞬间的相对值, 平移后不再对应指针, 补不回来。正解是在 `moved` 里加回 `root.offset - root.press-offset` 换成窗口空间位移 (补偿读的是本次赋值前的 offset, 所以与夹紧无关): 同一序列变成 38→39→41→…→62 单调 1px/步。补偿放在内部 `moved` 而不是改 public `drag-to(dx, dy)` 的语义, 探针仍可直接用窗口空间位移驱动。
