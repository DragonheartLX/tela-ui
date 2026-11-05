# 已定决策 (勿再翻案)

> 已随实现验证的取舍; 改代码前读。机制细节见 `docs/slint-semantics.md`。

- **几何一律落到令牌刻度上 (2026-10-18)** — 间距/圆角/控件高只写令牌; 例外: 图标内在尺寸与 OS 约定值 (标题栏键宽 46px、`resize-border` 8px) 用字面量并注释; TKbd 由 TSize 驱动; 窗口圆角取 `radius-large` (9px)。
- **模态正文裁剪封顶, 不引滚动容器 (2026-10-18)** — TDialog 按 `窗口高 - 48px - chrome`、TNotification 单卡按 `(高 - 24px) / 4` 封顶; 布局 Text 不会纵向收缩 (语义 16), 只能 `clip: true`。
- **TProgress 值动画是开关, 默认关 (2026-10-18)** — `animated: true` 才挂 `animate` (用 `enabled:` 关断), 默认关避免值滞后; 不要改成命令式镜像属性 (挂载会把初值从 0 扫上来); 开关与值同帧写不缓动 (语义 19)。
- **TTabs 无障碍 = tab-list + 每列一个 tab (2026-10-18)** — 根为容器角色, 列承担 tab role/label/disabled/默认动作, 与 TNavBar/TBreadcrumb 一致。
- **TTable 横向滚动用 ListView 自带轴 (2026-10-20)** — `content-width` 设列固有宽, 行宽同值, 表头绑 `content-x`; 不套第二层横向 ScrollView (语义 27)。
- **TTable 排序只上报 (2026-10-20)** — 1.18 无循环语句, 组件无法重排模型; 行重排由宿主完成 (语义 21)。
- **TDatePicker 触发器只读, today 由宿主传入 (2026-10-20)** — 不暴露当前日期与字符串切片, 不做手工输入/解析; 月历运算与选择在组件内。
- **主题令牌表结构冻结** — 103 字段双预设重复 + 151 顶层转发, 刻意对齐 TDesign 清单, 不做嵌套/合并。
- **字体走令牌 + 统一 `TText`** — 裸 Text 继承不到主题字体 (语义 13); 库内文本一律 `TText` (14px/22px), 覆盖字号补 `line-height`。
- **`line-height-headline-s/m/l` 已顶层转发 (2026-10-18)** — THeading 直接消费, 不再是预留令牌。
- **`font-family-medium` 是预留令牌** — 两套预设相同、暂无消费方; 缺的是消费方而非转发。
- **浮层皮肤直接引用 `shadow-*` 三行组属有意保留** — 单面组件各自独立, 不属重复收敛目标。
- **"TCollapse 换数据后错误面板仍展开"已否决** — 整份模型替换走 reset() 清空实例, 实际表现是全部收起 (语义 3)。
- **TIconSet 图标不抽公共路径 (2026-10-15)** — 每个图标独立, 同路径也不去重, 保持可整段复制替换。
- **gallery 状态失同步与 Timer 常驻属 gallery 代码问题** — 修时不要顺手改组件库语义。
- **cdylib 保留 (2026-10-18)** — 移动平台预留, 不因当前无 C ABI 消费者删除。
- **TELA_THEME 精确匹配 (2026-10-18)** — 只认 `light` / `dark`, 不 trim 不小写化; 其他值报错退出 (`src/lib.rs` 的 `theme_from_env`)。
- **启动失败只在无控制台构建里弹窗 (2026-10-18)** — `MessageBoxW` 仅 Windows + `not(debug_assertions)`; dev 写 stderr; panic 同通道 (release `panic = "abort"`)。
- **不做令牌漂移守卫 (2026-10-18)** — 70 字段重复是刻意对齐 TDesign; 不加断言/测试工程, 改 `theme.slint` 时人工比对。
- **聚合器按交互类型拆分 (2026-10-22)** — 旧 data-entry/data-display/feedback 过宽, 拆成 input/select/date-time/display/content/notice/overlay 等 (破坏性变更); gallery 侧栏一一对应; `common.slint` 仍汇总全部导出。
- **TTimePicker 整列数据 + 行内选中框, 不套 ListView (2026-10-21)** — ListView 的 `content-y` 归内部 Flickable (语义 28); 整列渲染 + 自维护偏移并 animate, 选中框随数据滚动; 无确认按钮/虚拟化/惯性滚动。
- **TTree 用嵌套声明, 不用扁平行模型 (2026-10-23)** — `TTree`/`TTreeItem`/`TTreeBranch` + `@children`; 折叠把子节点放进 clip 盒动画, 实例与焦点保留; 代价是选中态只能放全局 `TTreeLogic`, 同窗口多树共享。
- **模式偏好直接用 `Palette.color-scheme` (2026-10-25)** — 删除 `TThemePreference` / `TTheme.preference`; 读写标准库属性让 tela 与标准库组件同步, `TTheme.is-dark` 只做派生。取舍: `unknown` 下 tela 默认暗色而标准库按 OS 解析, 可能不一致; 写入替换"跟随 OS"绑定 (语义 35)。gallery 开关/设置页/`TELA_THEME` 都走它; `MainWindow.slint` 再导出 `Palette`。
- **TStickyTool = TDesign 固定侧边工具栏 (2026-10-25)** — placement 六向 + offset, shape 改两端, type 56/40px, popup 文本气泡按 trigger; 不是吸底操作条 (吸底用 TAffix `side: bottom`)。
- **滚动缓动一律组件内 Timer 逐帧写入 (2026-10-25)** — `animate content-y` 活不过按压 (语义 36); TScrollBar/TAnchor/TBackTop 用 16ms×ease-out≈200ms glide, 缓动期内覆盖外部写入, 仅滑块拖拽立即接管; 直接写 content-y = 瞬时。
- **TSwipeCell 手势层在宿主子元素之下 (2026-10-25)** — 覆盖层会吞内容点击; 放之下保住子元素事件, 代价是从可点击子元素起手不滑。状态机/吸附动画 public; 操作按钮恒语义色 (default → primary, 语义 38); 分隔线固定在行上, 连续列表末格设 false; 操作列无描边; `mouse-x` 需补 offset 换窗口空间 (语义 40)。
- **TPullDownRefresh 自持拖拽, 拉动量不借道 content-y (2026-10-25)** — 无 overscroll 且越界自动夹回 (语义 37), 用独立 `pull` 平移包装组; 滚轮经手势层分流: 顶部向上 accept 喂拉动, 向下先收回再转交, 其余穿透; header clip; content-height 显式绑定; 状态机 public, `refreshing` 双向 + 超时兜底; 无惯性滚动。
- **TFooter 是纯展示件 (2026-10-25)** — logo/链接列/版权三段居中; 链接用 TLink, 只报 `link-clicked(i)` / `logo-clicked()`; 居中靠交叉轴撑满 + `horizontal-alignment: center`, 不写 `width: parent.width` (binding-loop, 语义第 31 条 (c))。
- **范围外组件不实现 (2026-10-25)** — grid/layout/space (Slint 布局); chart/video/signature (富媒体成本); qrcode/watermark (Rust 生成); Upload (文件对话框 interop, 可先做 DropZone 视觉件); Image 容器 (无加载状态); ColorPicker (低性价比); TTitleBar (TBorderlessWindow); TOverlay (TModalSurface); popup/popover/toast/stepper/menu (已有 TTooltipArea/TMessage/TInputNumber/TSideNav+TNavBar)。
- **页面滚动记忆: 节拍保存 + 适应式重申 (2026-10-25)** — 保存不能现读 content-y (塌缩夹持早于回调, 会存污染值), 改 100ms 常驻节拍 + 切换期挂起; 恢复用 32ms 适应式重申 (装得下才断言, 高度变化重置计数, 连续 8 拍收手, >8 次或 ~1.3s 封顶); 真机验证 -7200 往返一致。
