# 自证配方与探针踩坑

> 改动与自证的**规则**在 `AGENTS.md` "改动流程"; 这里只放怎么做的配方、
> 参考值与踩坑。一次性 Rust 探针写在 `Temp/` 下, 收尾删除。

## 自证配方

```powershell
# 1) 每个改动文件 0 诊断才算通过
slint-viewer -L "tela-ui=<repo>\tela-ui" --check <file.slint> --style fluent

# 2) 全量检查 (仅 5 个非组件数据文件报既有 "No component found": colors / theme /
#    base/icon/data / structs/enums / structs/theme)
Get-ChildItem tela-ui,ui -Recurse -Filter *.slint |
  ForEach-Object { slint-viewer -L "tela-ui=$PWD\tela-ui" --check $_.FullName --style fluent }

# 3) 亮/暗各一张, 交付前必做
slint-viewer -L "tela-ui=$PWD\tela-ui" --screenshot out-light.png --size 1200x800 ui\MainWindow.slint
#    暗色: --load-data dark.json, 内容 {"TTheme": {"preference": "dark"}}
#    JSON 必须无 BOM, Set-Content 默认带 BOM 会报 "expected value at line 1 column 1"
```

- 本机工具: `slint-viewer` / `slint-lsp` / `cargo`。
- 像素参考值 (2026-10-18 基准, 1200x800 MainWindow): 亮 `#ffffff 56.1%` / `#f3f3f3 20.5%` / `#eeeeee 14.0%` / `#0052d9 1.5%`, 暗 `#242424 56.1%` / `#2c2c2c 20.5%` / `#181818 14.0%` / `#4582e6 1.5%`。要证明新增内容没有副作用, 最强口径是把新内容剥掉的副本渲染, 与 HEAD worktree 逐字节一致。
- 想看声明是否真生效就查生成代码 `target\debug\build\tela-ui-*\out\MainWindow.rs`, 用 `Select-String` 统计特征串比读文档可靠; 生成代码里枚举变体带 `r#` 前缀, 特征串要写 `AccessibleStringProperty :: r#Label` 这种形态。

## 探针踩坑

1. `mock_elapsed_time` 要 `std::time::Duration`, 不是裸整数。
2. 指针用 `slint::LogicalPosition { x: f32, y: f32 }`(`platform::` 下没有), 按钮用 `slint::platform::PointerEventButton`。
3. 用 `i_slint_backend_testing::init_no_event_loop()`, 不是 `init_with_backend` 加 `TestingBackend::default()`。
4. 组件用 `.window()`(1.18 无 `as_window()`); global 用 `ui.global::<T>()` 加 `invoke_xxx()`, 前提是主 .slint 里 `export { T }` 把它提升为公共 API。
5. Rust 里的 Slint 枚举变体是 PascalCase, 小写只在 .slint 里用。
6. `SharedString` 比较先 `.to_string()`, 别用 `"x".into()`(ambiguous)。
7. 打开模态后先 `mock_elapsed_time` 等动画跑完(本库 `duration-base` = 200ms)再点, 否则点的是动画中途的位置。
8. 不知道元素在哪就写扫描循环逐点点击打印命中, 一次跑完拿到真实几何。
9. `Probe::get_singleton()` 只在被主 .slint 的 `export { ... }` 声明为 global 的组件存在; 探针工程里直接 `Probe::new().unwrap()` 拿实例, 用 `p.window()` 派发事件。
10. `slint::LogicalPosition` 与 `LogicalSize` 在 1.18 是具体类型不是泛型, 直接 `LogicalPosition::new(x, y)` 或 `LogicalPosition { x, y }`。
11. 探针工程里 `@tela-ui/...` 只有在 build.rs 用 `CompilerConfiguration::with_library_paths(HashMap::from([("tela-ui".into(), repo_root.join("tela-ui"))]))` 注册了同名库路径才生效; 裸 `tela-ui/...` 或 `@tela-ui/...` 未注册时会退化为 crate 根相对路径。
12. **Slint 1.18 语法限制**: `private function` 无效(用 `function`), callback 体内不能用 `var`, `TextInput.edited` 是 void 类型不能 return accept/reject。
13. `WindowEvent` 在 `slint::platform::` 下, 不在 crate 根(`use slint::platform::{PointerEventButton, WindowEvent};`)。
14. 探针 crate 放在仓库树里时, 自己的 `Cargo.toml` 要加一个空的 `[workspace]`, 否则会被根 package 的隐式 workspace 认领而报错。
15. `i_slint_backend_testing::init_no_event_loop()` 的默认渲染器不支持 `Window::take_snapshot`(报 "WindowAdapter::take_snapshot is not implemented by the platform"); 要截图或按帧量像素得自己装: `i-slint-backend-testing` 开 `renderer-software` feature, 再 `set_platform(Box::new(TestingBackend::new(TestingBackendOptions { mock_time: true, threading: false, renderer_name: Some("software".into()) })))`。`set_platform` 一个进程只能调一次(重复报 `AlreadySet`), 所以先装平台、再按场景反复 `Probe::new()`。
16. 探针里点元素前先把窗口尺寸定死, 再让布局恰好装下内容: 有多余高度时会被分给可伸缩子元素, 每根条的高度与 y 都变了。命中区差 1px 就漏 —— 实测开关行 120..142 里 130 命中、131 落在缝隙。
17. 组件建好后先派发一次 `WindowEvent::Resized { size: slint::LogicalSize::new(width, height) }`: testing backend 只上报尺寸不会自动回流, 否则每帧截图约一半像素是全透明(alpha 0), 量出来的包围盒全不可信。
18. 点击判定把"500ms 内、10px 内"的连续 press 串成一次连击, 且奇数次释放就触发 `double-clicked` —— **拖拽也算一次点击**。拖拽与双击/点击混测时, 每次释放后 `mock_elapsed_time(600ms)` 打断连击链, 否则上一次拖拽会让下一次断言踩进 `double-clicked`。
19. 量"平移"别用包围盒: 放大态画面必然盖满视口, 包围盒恒等于视口; 也不要用"去趋势亮度峰"的 argmax(带内容相关的偏差)。可用做法是取一行亮度剖面做归一化互相关, 位移精度 ±1px, 递增拖拽序列顺便就是夹紧证明。
20. 量"列表滚到哪"别用逐行墨迹*计数*做互相关 —— 在 44px 这类行距上会混叠出假位移(实测报出过假的 36px); 改量每行墨迹的*范围*(最右墨迹 x / 最深可见行号)。
21. 量"缩放"用画面非遮罩包围盒即可, 但要把工具条/芯片所在的行带排除, 并且留意悬停中的图标会变品牌色, 其抗锯齿光晕会被"蓝色系"谓词误判 —— 谓词里加红通道上限或按行带排除。
22. 一次 `set_platform` 只能建一个平台; 全部场景共用它, 每个 scenario 各自 `Probe::new()`, 场景之间把所有 overlay 关干净(探针里加"栈已空"断言), 否则上一场景的残留状态会污染下一场景。
