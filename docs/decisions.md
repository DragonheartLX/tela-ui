# 已定决策 (勿再翻案)

> 这些取舍已随实现验证; 改代码前读, 不要重新翻案。

- **几何一律落到令牌刻度上 (2026-10-18)** — 间距/圆角/控件高只写令牌; 两类例外: 图标盒内在尺寸(16/20/24px 等)保持字面量, 无对应刻度的 OS 约定值(标题栏键宽 46px、`resize-border` 8px)保持字面量并在行内注释说明。TKbd 交由 TSize 驱动(24/32/40px 高、12/14/16px 字号、8/12/16px 内边距), 窗口圆角取 `radius-large` (9px) 而非 Windows 的 8px。
- **模态正文靠裁剪封顶, 不引滚动容器 (2026-10-18)** — TDialog 正文按 `窗口高 - 48px - 卡片 chrome` 封顶, TNotification 单卡上限 `(高 - 24px) / 4`, 超出直接裁掉; 布局中的 Text 不会纵向收缩(`slint-semantics.md` 第 16 条), 封顶只能靠 `clip: true` 包装盒, 缺滚动是已知限制。
- **TProgress 的值动画是开关, 默认关 (2026-10-18)** — `animated: true` 时才给 `p` 挂 `animate`(用 `enabled:` 而非挂载/卸载, 关掉时当帧直达目标值); 默认关是因为逐帧变化的值套动画只会让进度条与百分比滞后于数据。不要改成"target + 命令式镜像属性"的写法: 那种写法挂载时会把非零初值从 0 扫上来(探针实测首帧 0.0, 应为 0.39)。开关与值同帧写不缓动属 Slint 行为, 见 `slint-semantics.md` 第 19 条。
- **TTabs 的无障碍模型是 tab-list + 每列一个 tab (2026-10-18)** — 根是容器角色, 各列承担 `tab` 角色/标签/禁用态与默认动作, 与 TNavBar/TBreadcrumb 的行级 role 写法一致。
- **TTable 横向滚动用 ListView 自带轴 (2026-10-20)** — `content-width` 设为列固有宽度, 行宽同值, 表头绑 `content-x` 跟随; 不套第二层横向 ScrollView, 避免横竖两个滚动容器抢滚轮与拖拽 (机制见 `slint-semantics.md` 第 27 条)。
- **TTable 排序只上报 (2026-10-20)** — 1.18 语言内没有循环语句, 组件无法重排模型; 保持 `sort-changed`, 行重排由宿主完成 (见 `slint-semantics.md` 第 21 条)。
- **TDatePicker 触发器只读, today 由宿主传入 (2026-10-20)** — Slint 1.18 不向普通组件暴露当前日期与字符串切片, 所以不做手工输入/字符串解析; `TCalendar.today` 与初始显示月份由宿主提供, 组件只负责月历运算与选择。
- **主题令牌表结构冻结** — 103 个字段在两套预设里重复、以及 151 个顶层转发, 是刻意对齐 TDesign 令牌清单的结果, 不做嵌套/合并重构。
- **字体走令牌 + 统一 `TText`, 不用 `Window.default-font-family`** — 未设 `font-family` 的 `Text` 只落到宿主窗口默认字体, 再落到渲染器通用族(实测 Arial), 主题字体失效; 库内文本一律经 `TText`(TDesign `body-medium`: 14px/22px), 覆盖字号时给 `line-height` 令牌, 系数由 `TText` 按自身字号换算。
- **`line-height-headline-s/m/l` 已顶层转发 (2026-10-18)** — THeading 各级行盒直接读它们, 不再是预留令牌。
- **`font-family-medium` 是预留令牌** — 两套预设定值相同, 暂无消费方; 与 line-height 不同, 它缺的是消费方而非转发。
- **各浮层皮肤直接引用 `shadow-*` 三行组属有意保留** — 单面组件各自独立, 不属于重复收敛目标。
- **"TCollapse 换数据后错误面板仍展开"已被否决** — 整份模型替换走 repeater reset() 会清空实例, 真实表现是全部收起(见 `slint-semantics.md` 第 3 条)。
- **TIconSet 图标不抽公共路径属性 (2026-10-15)** — 每个图标独立, 哪怕 Star/StarFill 逐字节同路径也不去重, 图标数据保持可整段复制替换。
- **gallery 页的状态失同步与 Timer 常驻属 gallery 代码问题** — 修的时候不要顺手改组件库语义。
- **cdylib 保留 (2026-10-18)** — `Cargo.toml` 的 `crate-type = ["lib", "cdylib"]` 是移动平台预留, 不因"当前无 C ABI 消费者"删除。
- **TELA_THEME 精确匹配 (2026-10-18)** — 只认字面量 `light` / `dark`, 不做 trim 也不做小写化; 设了别的值一律报错退出(`src/lib.rs` 的 `theme_from_env`), 不静默跟随系统。
- **启动失败只在没有控制台的构建里弹窗 (2026-10-18)** — `src/fatal.rs` 的 `MessageBoxW` 只在 Windows 且 `not(debug_assertions)` 时弹原生错误框; dev 只写 stderr, 否则自动化运行会被模态框卡住。panic 走同一通道, 因为 release 是 `panic = "abort"` 加 `windows_subsystem = "windows"`, 什么都不留。
- **不做令牌漂移守卫 (2026-10-18)** — 两套预设里那 70 个字段重复是刻意对齐 TDesign 令牌清单的结果, 不加运行时断言也不建测试工程; 改 `theme.slint` 时人工比对两套预设。
- **分类聚合器按交互类型拆分 (2026-10-22)** — 组件数到 57 后, `data-entry` / `data-display` / `feedback` 各自塞了 13–16 个组件, 单类过宽; 拆成 `input` / `select` / `date-time` / `display` / `content` / `notice` / `overlay` 等目录与聚合器, 旧聚合器删除 (破坏性变更), `common.slint` 继续汇总全部导出; gallery 侧栏同步拆成 10 个页面 (另加设置页), 与聚合器一一对应。
- **TTimePicker 用整列数据 + 行内选中框, 不套 ListView (2026-10-21)** — `ListView` 的 `content-y` 由内部 Flickable 接管, 按选中值定位会被下一次布局清回 0(见 `slint-semantics.md` 第 28 条), 所以时间列直接渲染整列(时 24 / 分秒 60 行), 自己维护偏移量并 `animate` 它: 选中框画在选中行上, 数据带动画滚动时它也一起移动, 点击或滚轮把新值送到中间。无确认按钮, 靠点击外部 / ESC 关闭。代价是没有虚拟化与惯性滚动; 换来真正的数据滚动与跟随数据的选中框。
