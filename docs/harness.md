# 自证配方与探针踩坑

> 规则见 `AGENTS.md` 改动流程; 这里只放配方、参考值与踩坑。一次性探针放 `Temp/`, 收尾删除。

## 自证配方

```powershell
# 1) 每个改动文件 0 诊断
slint-viewer -L "tela-ui=<repo>\tela-ui" --check <file.slint> --style fluent

# 2) 全量 (仅 5 个数据文件报 "No component found": colors / theme /
#    base/icon/data / structs/enums / structs/theme)
Get-ChildItem tela-ui,ui -Recurse -Filter *.slint |
  ForEach-Object { slint-viewer -L "tela-ui=$PWD\tela-ui" --check $_.FullName --style fluent }

# 3) 亮/暗各一张; 暗色 --load-data dark.json = {"Palette":{"color-scheme":"dark"}},
#    亮色必须显式 light.json (单帧渲染里 color-scheme 仍是 unknown); JSON 无 BOM
slint-viewer -L "tela-ui=$PWD\tela-ui" --screenshot out-light.png --size 1200x800 ui\MainWindow.slint

# 4) gallery 代码样本门禁 (当前 90 个)
python scripts\check-code-samples.py
```

- 工具: `slint-viewer` / `slint-lsp` / `cargo`。
- 像素基准 (1200x800 MainWindow): 亮 `#ffffff 56.1%` / `#f3f3f3 20.5%` / `#eeeeee 14.0%` / `#0052d9 1.5%`; 暗 `#242424 56.1%` / `#2c2c2c 20.5%` / `#181818 14.0%` / `#4582e6 1.5%`。验证无副作用: 剥掉新内容渲染, 与 HEAD 逐字节一致。
- 声明是否生效查生成代码 `target\debug\build\tela-ui-*\out\MainWindow.rs`; 枚举变体带 `r#`, 特征串写 `AccessibleStringProperty :: r#Label`。

## 探针踩坑

1. 时间: `mock_elapsed_time(Duration)`; 动画类先等 `duration-base` (200ms) 再点。
2. 类型: 指针 `slint::LogicalPosition { x, y }`, 按钮 `slint::platform::PointerEventButton`, `WindowEvent` 也在 `platform::`; Slint 枚举变体 PascalCase; `SharedString` 先 `.to_string()`。
3. 启动: 用 `i_slint_backend_testing::init_no_event_loop()`; 截图/量像素要 software 渲染器 + `set_platform(Box::new(TestingBackend::new(TestingBackendOptions { mock_time: true, threading: false, renderer_name: Some("software".into()) })))`, 每进程仅一次, 场景间复用平台、各自 `Probe::new()`。
4. 组件 `.window()`, global `ui.global::<T>()` + `invoke_xxx()`; 前提是主 .slint 用 `export { T }` 提升为公共 API。
5. 探针 crate 在仓库树里要加空 `[workspace]`; `@tela-ui/...` 需 build.rs 注册同名 library path, 否则退化到 crate 根。
6. 语法: `private function` 无效 (用 `function`); callback 体内不能用 `var`; `TextInput.edited` 是 void, 不能 return accept/reject。
7. 点元素前定死窗口尺寸并让布局恰好装下内容; 建好后派发一次 `WindowEvent::Resized`, 否则截图约一半像素全透明。命中区差 1px 就漏。
8. 不知道元素位置: 写扫描循环逐点点击打印命中。
9. 合成事件限制: testing 后端的指针/滚轮只命中 Window 直接子元素, 嵌套 TouchArea/Flickable 收不到; 驱动嵌套交互用 public function/直接写属性, 滚轮路由只能源码或真机验证。
10. 拖拽也算一次点击: 500ms/10px 内连续 press 会串成双击链, 每次释放后 `mock_elapsed_time(600ms)`。
11. 量平移: 别用包围盒; 取一行亮度剖面做归一化互相关 (±1px)。量列表位置: 别用逐行墨迹计数 (行距会混叠), 用量墨迹范围/最深可见行号。
12. 量缩放: 非遮罩包围盒, 排除工具条/芯片行带; 悬停图标的品牌色光晕会干扰蓝色谓词。
13. 场景隔离: 每个 scenario 各自 `Probe::new()`, 场景间关净 overlay 并断言栈空。
14. 嵌套 TouchArea 慢拖只能 OS 鼠标: `EnumWindows` 按 pid 取面积最大有标题的窗口, `ClientToScreen` 取客户区, 1px/步 `mouse_event` 保持按下, 每步 `CopyFromScreen`; 只看跨帧相对变化 (临时脚本放 `Temp/`)。
