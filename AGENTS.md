# AGENTS.md — tela-ui 协作约定

面向本仓库的 AI 与人类协作者。目标: 少踩已踩过的坑, 改完能自证, 不推翻已定决策。

## 写作约定

注释、文档、待办一律**一句话讲清**: 一句一个事实, 结论放句首, 不叠分句。位置用 `file:line`。

## 仓库速览

- `tela-ui/` 是组件库本体, `build.rs` 的 `with_library_paths` 把它注册为 import 前缀 `@tela-ui/...`。
- 顶层聚合按类别分(`base.slint` / `data-display.slint` / …), `common.slint` 仅作兼容出口, 新代码与演示页都用分类聚合。
- `ui/` + `src/` 是演示程序(bin `tela`): `ui/MainWindow.slint` 挂 7 个页面, 页面用 `if` 懒挂载。
- Slint 1.18.1 (Cargo.lock 锁定), edition 2024, 样式 `fluent`(`build.rs` 的 `with_style`), 启用 accessibility / backend-winit / renderer-skia / renderer-software。
- 74 个 `.slint` 共 8,936 行: `tela-ui/` 65 个 7,317 行, `ui/` 9 个 1,619 行(P4 前为 70 个 8,610 行)。
- 设计令牌: `theme.slint` 的 `TTheme` global (151 个顶层转发) → `structs/theme.slint` 的 `TAppThemeConfig` (103 字段); 色板在 `colors.slint` (亮/暗两套 ramp)。
- 浮层体系: `TPopupSurface` 锚定弹层 + `TPopupStack` 层计数与 ESC + z 序令牌(`z-popup` / `z-modal` / `z-message`) + 服务型 global(`TDialogService` / `TLoadingService` / `TMessage` / `TNotification` / `TTooltip`)。

## 改动后必须自证

```powershell
# 1) 每个改动文件 0 诊断才算通过
slint-viewer -L "tela-ui=<repo>\tela-ui" --check <file.slint> --style fluent

# 2) 全量检查
Get-ChildItem tela-ui,ui -Recurse -Filter *.slint |
  ForEach-Object { slint-viewer -L "tela-ui=$PWD\tela-ui" --check $_.FullName --style fluent }

# 3) 亮/暗各一张, 交付前必做
slint-viewer -L "tela-ui=$PWD\tela-ui" --screenshot out-light.png --size 1200x800 ui\MainWindow.slint
#    暗色: --load-data dark.json, 内容 {"TTheme": {"preference": "dark"}}
#    JSON 必须无 BOM, Set-Content 默认带 BOM 会报 "expected value at line 1 column 1"
```

- 本机工具: `slint-viewer` / `slint-lsp` / `cargo`。
- 视觉终判要人眼: 读不了图就别声称"我看过了", 改统计像素色值分布(2026-10-18 基准, 1200x800 MainWindow: 亮 `#ffffff 56.1%` / `#f3f3f3 20.5%` / `#eeeeee 14.0%` / `#0052d9 1.5%`, 暗 `#242424 56.1%` / `#2c2c2c 20.5%` / `#181818 14.0%` / `#4582e6 1.5%`), 再把 PNG 交用户确认。
- 想看声明是否真生效就查生成代码 `target\debug\build\tela-ui-*\out\MainWindow.rs`, 用 `Select-String` 统计特征串比读文档可靠。
- 行为/事件类改动要有运行时证据: 一次性 Rust 探针(testing backend 注入指针事件、`take_snapshot` 按帧量像素), 配方见 `docs/TODO.md` 末尾"Rust harness 踩坑"; 探针放 `Temp/` 且收尾时删掉。

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

## 文档同步

- 新增、改名、改签名的组件要同步 `docs/components.md`, 以及 `README.md` 的"已知限制"一节。
- 一条事实只写一处: 语言/框架机制进 `AGENTS.md`"语义实证", 不可翻案的取舍进"已定决策", 组件 API 进 `docs/components.md`, 验证数字与踩坑进 `docs/TODO.md`; 别处只用 `(见…第 N 条)` 指过去, 不复制正文。
- `docs/TODO.md` 是 gitignored 的本地文件; 新增待办写清 `文件:行号` 与一句话结论。
- `docs/TODO.md` 只留未完成项, 已完成直接删; 历史记 daily log, 决策记本节"已定决策"。

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
- **演示页的状态失同步与 Timer 常驻属演示代码问题** — 修的时候不要顺手改组件库语义。

## 参考

- `skills/slint/` — 本仓库内置的 Slint 技能与参考文档。
- `docs/components.md` — 组件文档(需同步); `docs/TODO.md` — 待办; `docs/screenshot-*.png` — 官方截图。
