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
