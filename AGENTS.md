# AGENTS.md — tela-ui 协作约定

面向本仓库的 AI 与人类协作者。目标: 少踩已踩过的坑, 改完能自证, 不推翻已定决策。

## 写作约定

注释、文档、待办一律**一句话讲清**: 一句一个事实, 结论放句首, 不叠分句。位置用 `file:line`。

## 仓库速览

- `tela-ui/` 是组件库本体, `build.rs` 的 `with_library_paths` 把它注册为 import 前缀 `@tela-ui/...`。
- 顶层聚合按类别分(`base.slint` / `data-display.slint` / …), `common.slint` 仅作兼容出口, 新代码与 gallery 页都用分类聚合。
- `ui/` + `src/` 是 gallery 程序(bin `tela`): `ui/MainWindow.slint` 挂 7 个页面, 页面用 `if` 懒挂载。
- Slint 1.18.1 (Cargo.lock 锁定), edition 2024, 样式 `fluent`(`build.rs` 的 `with_style`), 启用 accessibility / backend-winit / renderer-skia / renderer-software。
- 78 个 `.slint` 共 10,337 行: `tela-ui/` 69 个 8,601 行, `ui/` 9 个 1,736 行; 65 个导出组件 = 52 个 UI 组件 + 8 个服务 overlay + 5 个库内基类 (TStrokedShape / TInputFrame / TModalSurface / TPopupSurface / TPopupStackLayer)。
- 设计令牌: `theme.slint` 的 `TTheme` global (151 个顶层转发) → `structs/theme.slint` 的 `TAppThemeConfig` (103 字段); 色板在 `colors.slint` (亮/暗两套 ramp)。
- 浮层体系: `TPopupSurface` 锚定弹层 + `TPopupStack` 层计数与 ESC + z 序令牌(`z-popup` / `z-modal` / `z-message`) + 服务型 global(`TDialogService` / `TLoadingService` / `TMessage` / `TNotification` / `TTooltip` / `TImageViewerService` / `TActionSheetService` / `TCommandPaletteService`)。

## 改动后必须自证

三条硬要求; 配方、命令与参考值在 `docs/harness.md`。

- 改动的每个 `.slint` 文件 `--check` 0 诊断, 全量扫描也 0 (只有 5 个非组件数据文件报既有的 "No component found")。
- 交付前亮/暗各渲染一张, 像素分布对 `docs/harness.md` 的参考值; 读不了图就别声称"看过了", 把 PNG 交用户人眼终判。
- 行为/事件类改动要有 Rust 探针的运行时证据; 探针放 `Temp/`, 收尾删除。

## Slint 1.18 语义实证

每条都对着锁定版本源码或本仓库生成产物核对过, 不是猜测。

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

## 组件 API 约定
- 可变索引或值的组件必须配 `callback changed(...)`, 且只在真正变化时触发; TCollapse 手风琴模式已于 2026-10-18 补上 `changed(int)`(参数为新索引, `-1` 全收), TCarousel 由拖拽/指示点/自动轮播共用同一条触发路径。
- 点击后先自更新再回调(nav / side-nav / tabs / pagination 都是这个顺序), 不做"只回调、等宿主写回"的受控组件(TBreadcrumb 是已知例外)。
- 索引与页码必须夹紧: 越界值不能画到组件外, 也不能让分页出现"一页都不高亮"的坏状态。
- struct 的 `bool` 字段给默认值; 组件属性默认私有, 不必写 `private`。
- 可点击元素要有 `accessible-role` 与 `accessible-label`; 全库已覆盖 (2026-10-16)。禁用态要显式写 `accessible-enabled`, AT 动作回调自带守卫; 浮层/消息类文本用 `accessible-live-region`。
- 颜色一律走令牌(组件目录当前 0 处硬编码色值, 保持这条); 阴影几何也走 `shadow-*` 令牌, 不要就地写 blur/offset 数值。
- 文本一律用 `TText`, 不要裸 `Text` (唯一例外: `code-block.slint` 的等宽代码块); 覆盖 `font-size` 时补 `line-height: <同行高令牌>` (TSize 驱动时用 `TTheme.size-line-height(size)`)。
- 装长文本的容器要么固定单行给 `overflow: elide`(默认 `clip` 会把字形拦腰截断), 要么给上限: 多行正文用 `clip: true` 包装盒 + 显式高度, `max-height` 无效 (语义实证第 16 条)。
- `TText` 已有默认的字体/字号/字重/颜色/行高, 站点不要再抄一遍同值属性(`font-size-m`、`text-color-primary`、`font-weight-regular`)。
- 浮层 z 序取 `TTheme.z-*`; 需要 ESC 与层计数的模态接入 `TPopupStackLayer`。
- 内容尺寸的容器(没写 `width`)不要用自身 `self.width` 定位: 里面的 `Text` 会画在第一轮坐标上, 给容器显式宽度或把文本自己定位 (语义实证第 22 条)。

## 文档同步

- 新增、改名、改签名的组件要同步 `docs/components.md`, 以及 `README.md` 的"已知限制"一节。
- 一条事实只写一处: 语言/框架机制进 `AGENTS.md`"语义实证", 不可翻案的取舍进"已定决策", 组件 API 进 `docs/components.md`, 探针配方与踩坑进 `docs/harness.md`, 待办与组件规划进 `docs/components-plan.md`; 别处只用 `(见…第 N 条)` 指过去, 不复制正文。
- `docs/components-plan.md` 是 gitignored 的本地清单, 只列未做项, 完成一个删一条; 里程碑与验证数字随当次提交记在提交说明与 `docs/harness.md`。

## 已定决策 (勿再翻案)

- **几何一律落到令牌刻度上 (2026-10-18)** — 间距/圆角/控件高只写令牌; 两类例外: 图标盒内在尺寸(16/20/24px 等)保持字面量, 无对应刻度的 OS 约定值(标题栏键宽 46px、`resize-border` 8px)保持字面量并在行内注释说明。TKbd 交由 TSize 驱动(24/32/40px 高、12/14/16px 字号、8/12/16px 内边距), 窗口圆角取 `radius-large` (9px) 而非 Windows 的 8px。
- **模态正文靠裁剪封顶, 不引滚动容器 (2026-10-18)** — TDialog 正文按 `窗口高 - 48px - 卡片 chrome` 封顶, TNotification 单卡上限 `(高 - 24px) / 4`, 超出直接裁掉; 布局中的 Text 不会纵向收缩(语义实证第 16 条), 封顶只能靠 `clip: true` 包装盒, 缺滚动是已知限制。
- **TProgress 的值动画是开关, 默认关 (2026-10-18)** — `animated: true` 时才给 `p` 挂 `animate`(用 `enabled:` 而非挂载/卸载, 关掉时当帧直达目标值); 默认关是因为逐帧变化的值套动画只会让进度条与百分比滞后于数据。不要改成"target + 命令式镜像属性"的写法: 那种写法挂载时会把非零初值从 0 扫上来(探针实测首帧 0.0, 应为 0.39)。开关与值同帧写不缓动属 Slint 行为, 见语义实证第 19 条。
- **TTabs 的无障碍模型是 tab-list + 每列一个 tab (2026-10-18)** — 根是容器角色, 各列承担 `tab` 角色/标签/禁用态与默认动作, 与 TNavBar/TBreadcrumb 的行级 role 写法一致。
- **主题令牌表结构冻结** — 103 个字段在两套预设里重复、以及 151 个顶层转发, 是刻意对齐 TDesign 令牌清单的结果, 不做嵌套/合并重构; 最多加一个"两套预设模式无关字段必须全等"的漂移守卫。
- **字体走令牌 + 统一 `TText`, 不用 `Window.default-font-family`** — 未设 `font-family` 的 `Text` 只落到宿主窗口默认字体, 再落到渲染器通用族(实测 Arial), 主题字体失效; 库内文本一律经 `TText`(TDesign `body-medium`: 14px/22px), 覆盖字号时给 `line-height` 令牌, 系数由 `TText` 按自身字号换算。
- **`line-height-headline-s/m/l` 已顶层转发 (2026-10-18)** — THeading 各级行盒直接读它们, 不再是预留令牌。
- **`font-family-medium` 是预留令牌** — 两套预设定值相同, 暂无消费方; 与 line-height 不同, 它缺的是消费方而非转发。
- **各浮层皮肤直接引用 `shadow-*` 三行组属有意保留** — 单面组件各自独立, 不属于重复收敛目标。
- **"TCollapse 换数据后错误面板仍展开"已被否决** — 整份模型替换走 repeater reset() 会清空实例, 真实表现是全部收起(见语义实证第 3 条)。
- **TIconSet 图标不抽公共路径属性 (2026-10-15)** — 每个图标独立, 哪怕 Star/StarFill 逐字节同路径也不去重, 图标数据保持可整段复制替换。
- **gallery 页的状态失同步与 Timer 常驻属 gallery 代码问题** — 修的时候不要顺手改组件库语义。
- **cdylib 保留 (2026-10-18)** — `Cargo.toml` 的 `crate-type = ["lib", "cdylib"]` 是移动平台预留, 不因"当前无 C ABI 消费者"删除。
- **TELA_THEME 精确匹配 (2026-10-18)** — 只认字面量 `light` / `dark`, 不做 trim 也不做小写化; 设了别的值一律报错退出(`src/lib.rs` 的 `theme_from_env`), 不静默跟随系统。
- **启动失败只在没有控制台的构建里弹窗 (2026-10-18)** — `src/fatal.rs` 的 `MessageBoxW` 只在 Windows 且 `not(debug_assertions)` 时弹原生错误框; dev 只写 stderr, 否则自动化运行会被模态框卡住。panic 走同一通道, 因为 release 是 `panic = "abort"` 加 `windows_subsystem = "windows"`, 什么都不留。
- **不做令牌漂移守卫 (2026-10-18)** — 两套预设里那 70 个字段重复是刻意对齐 TDesign 令牌清单的结果, 不加运行时断言也不建测试工程; 改 `theme.slint` 时人工比对两套预设。

## 参考

- `skills/slint/` — 本仓库内置的 Slint 技能与参考文档。
- `docs/components.md` — 组件文档(需同步); `docs/harness.md` — 探针配方; `docs/screenshot-*.png` — 官方截图。
