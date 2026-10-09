# AGENTS.md — tela-ui 协作约定

面向在本仓库工作的 AI 与人类协作者。目标: 少踩已经踩过的坑, 改完能自证, 不推翻已定决策。

## 仓库速览

- `tela-ui/` 是组件库本体, 通过 `build.rs` 的 `with_library_paths` 注册为 import 前缀 `@tela-ui/...`; 顶层聚合文件按类别分 (`base.slint` / `data-display.slint` / …), `common.slint` 只是兼容出口, **新代码与演示页都用分类聚合**。
- `ui/` + `src/` 是演示程序 (bin `tela`): `ui/MainWindow.slint` 挂 7 个页面 (`ui/pages/*.slint`), 页面用 `if` 懒挂载。
- 技术栈: **Slint 1.18.1** (Cargo.lock 锁定), edition 2024, 样式 `fluent` (`build.rs` 的 `with_style`), 启用 `accessibility` / `backend-winit` / `renderer-skia` / `renderer-software`。
- 规模: `tela-ui/` 62 个 `.slint` / 6,766 行, `ui/` 8 个 (7 个页面 + MainWindow) / 1,779 行, 合计 70 文件 / 8,545 行。
- 设计令牌: `tela-ui/theme.slint` 的 `TTheme` global (151 个顶层转发) → `tela-ui/structs/theme.slint` 的 `TAppThemeConfig` (103 字段); 色板在 `tela-ui/colors.slint` (亮/暗两套 ramp)。
- 浮层体系: `TPopupSurface` (锚定弹层) + `TPopupStack` (层计数/ESC) + z 序令牌 (`z-popup`/`z-modal`/`z-message`) + 服务型 global (`TDialogService`/`TLoadingService`/`TMessage`/`TNotification`/`TTooltip`)。

## 改动后必须自证

```powershell
# 1) 编译检查: 0 诊断才算通过 (对每个改动过的文件都跑)
slint-viewer -L "tela-ui=<repo>\tela-ui" --check <file.slint> --style fluent

# 2) 全量检查
Get-ChildItem tela-ui,ui -Recurse -Filter *.slint |
  ForEach-Object { slint-viewer -L "tela-ui=$PWD\tela-ui" --check $_.FullName --style fluent }

# 3) 渲染核对 (亮/暗各一张, 交付前必做)
slint-viewer -L "tela-ui=$PWD\tela-ui" --screenshot out-light.png --size 1200x800 ui\MainWindow.slint
#    暗色: --load-data dark.json, 内容 {"TTheme": {"preference": "dark"}}
#    注意 JSON 必须无 BOM (Set-Content 默认带 BOM, 会报 "expected value at line 1 column 1")
```

- 本机工具 `slint-viewer` / `slint-lsp` / `cargo`。
- **视觉终判需要人眼**: 若当前模型读不了图, 不要声称"我看过了" —— 改为统计像素色值分布 (亮色基线约 `#ffffff 53.4% / #f3f3f3 21.2% / #eeeeee 15.7% / #0052d9 1.5%`, 暗色为 `#242424 / #2c2c2c / #181818 / #4582e6`), 并把 PNG 交给用户确认。
- 想看"声明是否真的生效", 直接查生成代码: `target\debug\build\tela-ui-*\out\MainWindow.rs` (约 13.6 万行, 每次构建后更新), 用 `Select-String` 统计特征字符串比读文档更可靠。例: 统计 `AccessibleStringProperty\s*::\s*r#([A-Za-z]+)` 可知无障碍属性是否真的进了产物。

## Slint 1.18 语义实证 (本仓库踩过的坑)

以下每条都在本轮审查中对着锁定版本的源码或本仓库生成产物核对过, 不是猜测:

1. **`Rectangle` 不消费指针事件** → 点击会继续传给下层兄弟。机制: `Rectangle::input_event` 返回 `EventIgnored`; 指针遍历用 `TraversalOrder::FrontToBack` (从最后声明的兄弟开始), 且 `EventAccepted => abort` / `EventIgnored => 继续`。**后果**: 模态卡片/面板若自身不含 TouchArea, 点正文会落到背景遮罩上 —— 面板内必须放一个空白 `TouchArea {}` 当吸收层。
2. **`visible:` 与 `animate` 互斥**: `visible: false` 只是不渲染 (元素仍实例化, 动画照跑但看不见), 所以退场动画永远看不到。退场动画只能靠 `opacity`; 需要真正销毁用 `if`。
3. **整份模型替换 ≠ 单行写入**: 赋值整个数组走 `RepeaterTracker::reset()` → `instances.clear()` (实例重建, for 体里的私有属性归零); `items[i] = x` 走 `row_changed` → 就地 `comp.update()` (实例与私有属性保留)。**后果**: 依赖"槽位私有状态"的组件 (如通知的 per-slot Timer) 必须自己带 stamp 并在 `changed` 时 restart。
4. **`Timer.running` 默认 `true`** (官方明确警告常驻耗电); `restart()` 只重启"此前已启动过"的计时器, 所以改成 `running: false` 后必须用 `start()`, 否则自动隐藏会失效。
5. **无障碍属性只对两个内置类型自动推导** (`Text` → role/label, `TextInput` → role/value/enabled/read-only/动作): `Rectangle` 派生组件**不会**自动获得 `accessible-enabled`; 而后端仅在"取到 Enabled 且 ≠ `"true"`"时才标记禁用。**后果**: 不显式写 `accessible-enabled` 的组件, 禁用态对读屏不可见, 且 `accessible-action-default` 会在 disabled/loading 时照样触发 —— 动作回调里必须自带守卫。本仓库当前生成产物里 `Enabled` 分支仅 2 处, 都来自 TextInput 的自动绑定。
6. **`to-float()` 解析失败/空字符串 = 0** (`unwrap_or_default()`; C++ 侧同样显式初始化 0) → 数字输入框清空会被夹到 min 并触发 `changed`。
7. **`Text.overflow` 默认 `clip`**、`Text.wrap` 默认 `no-wrap`: 要省略必须显式 `overflow: elide`, 要换行必须显式 `wrap: word-wrap`。
8. **赋值会移除绑定**: 不要对宿主可能绑定的属性做命令式赋值 (典型: 组件内部写 `root.visible = false`), 否则宿主的绑定被永久销毁; 复用组件不要替宿主决定可见性。
9. 双向绑定 `a <=> b` 的初值取右侧; `in` 属性也可以双向绑定; 组件内不写 `in/out/private` 的 `property` 默认就是私有。

## 组件 API 约定

- 状态类组件 (可变的索引/值) 必须配 `callback changed(...)`, 并且**只在真正变化时触发** (全库唯一两个例外是 TCollapse/TCarousel, 见待办)。
- 点击后先自更新再回调 (nav / side-nav / tabs / pagination 都是这个顺序), 不要做"只回调、等宿主写回"的受控组件 (TBreadcrumb 是例外, 已知不一致)。
- 索引/页码必须夹紧: 越界值不能画到组件外, 也不能让分页出现"一页都不高亮"的坏状态。
- struct 里的 `bool` 字段给默认值; 组件属性默认私有, 不必写 `private`。
- 可点击元素要有 `accessible-role` + `accessible-label`; 表单类已覆盖, 导航/展示类还有 14 个文件待补。
- 颜色**一律走令牌** (当前组件目录 0 处硬编码色值, 保持这条); 阴影几何也走 `shadow-*` 令牌, 不要就地写 blur/offset 数值。
- 浮层: z 序取 `TTheme.z-*`; 需要 ESC/层计数的模态接入 `TPopupStackLayer`。

## 文档同步

- 新增/改名/改签名的组件: 同步 `docs/components.md` (当前 92 个导出名里缺 4 个服务导出) 与 `README.md` (含"已知限制"一节)。
- `docs/TODO.md` 是 gitignored 的本地待办, 已完成项保留 `[x]`; 新增待办请写清 `文件:行号` 与一句话结论。
- `docs/components-plan.md` 同样是 gitignored 的规划稿。

## 已定决策 (勿再翻案)

- **主题令牌表结构冻结**: 103 个字段在两套预设里重复、以及 151 个顶层转发, 是刻意对齐 TDesign 令牌清单的结果, 不做嵌套/合并重构; 最多加一个"两套预设模式无关字段必须全等"的漂移守卫。
- `line-height-headline-s/m/l` 是预留令牌 (仅缺顶层转发), 加注释即可。
- 各浮层皮肤里直接引用 `shadow-*` 三行组属有意保留 (单面组件各自独立), 不属于重复收敛目标。
- "TCollapse 换数据后错误面板仍展开" 已被否决 (原因见上文第 3 条)。
- 演示页的状态失同步、Timer 常驻等是**演示代码**问题, 修的时候不要顺手改组件库语义。

## 参考

- `skills/slint/` — 本仓库内置的 Slint 技能与参考文档 (语言/布局/事件与浮层/坑/主题/互操作/调试)。
- `docs/components.md` — 组件文档 (需同步); `docs/TODO.md` — 待办; `docs/screenshot-*.png` — 已提交的官方截图。