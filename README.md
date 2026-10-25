![tela-ui](https://socialify.dev/DragonheartLX/tela-ui/image?description=1&font=JetBrains+Mono&logo=https%3A%2F%2Fraw.githubusercontent.com%2FDragonheartLX%2Ftela-ui%2Frefs%2Fheads%2Fmain%2Fui%2Fassets%2Flogo.svg&name=1&pattern=Plus&stargazers=1&theme=Auto)

# Tela UI

TDesign 风格的 [Slint](https://slint.dev) 第三方组件库

![screenshot](docs/screenshot.png)

## 快速开始

在 `build.rs` 中注册组件库路径, 之后即可用 `@tela-ui/` 前缀导入组件:

```rust
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let library_paths = HashMap::from([(
        "tela-ui".to_string(),
        PathBuf::from("path/to/tela-ui"), // 本项目的 tela-ui/ 文件夹
    )]);
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(library_paths);
    slint_build::compile_with_config("ui/MainWindow.slint", config).unwrap();
}
```

```slint
import { TTheme, TThemeRole } from "@tela-ui/theme.slint";
// 分类聚合导出 — 也可以直接从 "@tela-ui/base/button.slint" 等文件导入
import { TButton, TButtonVariant, TIcon } from "@tela-ui/base.slint";
import { TInput } from "@tela-ui/input.slint";
import { TSelect } from "@tela-ui/select.slint";
import { TCard, TTag } from "@tela-ui/display.slint";
import { TSkeleton } from "@tela-ui/experimental.slint";

export component MainWindow inherits Window {
    background: TTheme.bg-color-page;

    TButton {
        theme: TThemeRole.primary;
        variant: TButtonVariant.outline;
        text: "主要按钮";
    }
}
```

运行 gallery (组件陈列应用):

```sh
cargo run                     # 跟随系统深浅色
TELA_THEME=dark cargo run     # POSIX: 强制暗色 (只认 light / dark, 其它值启动即报错退出)
$env:TELA_THEME="dark"; cargo run   # PowerShell 写法
```

## 主题

所有令牌都在 `TTheme` 全局 (`theme.slint`) 上。当前生效的配置是一个 `TAppThemeConfig` 结构体; `light` / `dark` 是两个 `in-out` 预设。

```slint
// 读取令牌
Rectangle { background: TTheme.bg-color-container; }

// 覆盖某个预设的单个令牌
TTheme.dark.bg-page = #101010;

// 整体替换预设 (省略的字段必须补齐 — 请基于 TLightColors / TDarkColors 派生, 以保留原色阶)
import { TDarkColors } from "@tela-ui/colors.slint";
TTheme.dark = {
    animation-duration: 200ms,
    duration-fast: 150ms,
    duration-slow: 300ms,
    primary: TDarkColors.brand,
    // …完整字段清单见 structs/theme.slint
};
```

```rust
// 从 Rust 侧操作 (TTheme/TThemePreference 由 ui/MainWindow.slint 再导出)
use tela_lib::{TTheme, TThemePreference};
TTheme::get(&ui).set_preference(TThemePreference::Dark);
```

`preference` 默认为 `system`, 实时跟随 `Palette.color-scheme`; 组件通过各自的 `states` 自动换肤。亮暗开关应绑定 `TTheme.is-dark` (而非 `preference`), 以反映解析后的真实模式。

```slint
TSwitch {
    checked: TTheme.is-dark;      // OS 为暗色且 preference 为 system 时为 true
    changed(c) => { TTheme.preference = c ? TThemePreference.dark : TThemePreference.light; }
}
```

93 个组件 (基础 12 / 输入 14 / 选择 11 / 日期时间 5 / 展示 12 / 内容 16 / 提示 4 / 浮层 6 / 导航 12, 另有实验性 1; 另有库内部件 TNavLabel / TScrollThumb 不在聚合出口) 按类别聚合导出; 另有 8 个服务型浮层 (对话框 / 加载 / 消息 / 通知 / 图片预览 / 命令面板 / 动作面板 / 提示) 与主题、图标、浮层栈三个全局对象。`@tela-ui/common.slint` 保留为全量兼容出口, 完整参数说明见 [docs/components.md](docs/components.md)。

## 致谢与说明

- 图标几何数据来自 [Lucide](https://lucide.dev) (ISC 许可), 按 [lucide-slint](https://github.com/cnlancehu/lucide-slint) (MIT OR Apache-2.0)的数据化结构打包。
- 设计令牌遵循 [TDesign 规范](https://tdesign.tencent.com)。
- 已知限制:
    - 按钮无虚线变体 (Slint 边框仅支持实线)。
    - TSelect 与 TDropdownMenu 暂无键盘导航; TCommandPalette 的列表不支持上下方向键步进 (1.18 语言内没有循环语句, 组件层无法扫描匹配项 —— 高亮停在第一条匹配项, 回车执行它或点击列表项)。
    - TSelectInput / TTreeSelect / TCascader 暂无键盘导航 (对齐 TSelect); TTreeSelect 的选中值经 `TTreeLogic` 全局共享 (同 TTree); TCascader 面板为单列逐级展开, 触发器只回显叶子 label, 完整路径回显暂不支持 (声明式嵌套下叶子拿不到祖先 label)。
    - TTextArea 暂无 maxLength 字数统计。
    - 对话框 / 通知 / 底部弹层正文超过窗口高度时按上限裁剪 (无滚动容器)。
    - TImageViewer 无旋转与图片说明文字。
    - TTable 列宽不可拖拽, 不支持多列排序; 排序只回调, 行重排由宿主完成 (1.18 语言内没有循环语句)。
    - TDatePicker 触发器只读, 不支持手工输入或字符串解析; 日历的 `today` 需宿主传入 (Slint 不暴露当前日期)。
    - TCalendar / TDatePicker 暂无键盘导航 (方向键切换日期、回车确认)。
    - TTimePicker 触发器只读, 滚轮逐格改值并直接触发 `changed`, 无确认按钮 (点击外部或 ESC 关闭), 暂无键盘导航。
    - TAutoComplete 暂无上下方向键步进 (高亮停在第一条可选匹配或悬停项); 过滤只匹配 `label` 子串, 不区分大小写。
    - TTagInput 单行不换行, 重复校验由宿主完成; 删除只上报 (下标 + 标签), 模型增删由宿主执行。
    - TOtpInput 无 `max-length` (Slint 1.18 未提供), 段内溢出清理与整段粘贴分发由宿主改写 `values` 完成; 已填段吞掉后续可打印键, 换字符需先退格清除。
    - TMention 的 `@` 触发解析与选中文本拼接由宿主完成 (1.18 无字符串切片); 建议面板锚输入框而非光标。
    - TPicker 无触发器 (多列联合显示文本无法在语言内拼接) 与联动列, 列宽固定 56px; `value` 请初始化成与列数等长 (短模型只有恰好补到末位的拾取能追加, 中间留空的列写不进)。
    - TDateTimePicker 触发器只读, 年列范围由宿主给定 (整列渲染), 无"此刻"高亮。
    - TList 行结构固定 (图标/标题/说明), 不支持宿主自定义行渲染 (ListView 行是模型驱动)。
    - TTree 用嵌套声明 + `@children`, 选中态存在 `TTreeLogic` 全局里 (Slint 无实例级 global), 同屏多个树共享一份选中值; 折叠态是分支自己的 `opened`, 不写回模型。
    - TContextMenu 弹层在窗口边缘会被裁剪; 默认触发区会吞掉左键, 交互式触发内容需设 `auto-area: false` 并自行调用 `open-at()`。
    - 贴窗口边缘的 TScrollBar 会被原生拉伸热区抢占命中 (TBorderlessWindow `resize-border`, 默认 8px); 设 `thumb-inset` 为 `resize-border` 宽度即可避开, 或调小 `resize-border` (gallery 用 4px + 4px 内缩)。
    - TSplitter 第二栏无最小尺寸约束, 可被拖到 0; 主栏折叠无过渡动画。
    - TPopover 主体是 `title` + `text` 两段文本, 完全自定义主体需用 `TPopupSurface` 自建。
    - TForm 是轻量脚手架: 无校验引擎, help/error 只做状态呈现, 校验逻辑在 Rust 侧。
    - TComment 纯展示: 回复列表与图片由宿主自行嵌套; 点赞态组件自翻转后回调。
    - TNoticeBar 单行 elide 无跑马灯; 多条公告为整条垂直轮播; 关闭走 visible, 隐藏期间布局占位仍在。
    - TCell 分隔线由每行自绘, 组内末行需设 divider: false; 行结构固定 (图标/标题/说明/note/箭头)。
    - TCountDown 由剩余时长 (duration) 驱动, 无目标时刻解析 (Slint 不暴露当前日期); 时位不按 24 回绕。
    - TTabBar 贴底定位与安全区由宿主容器负责。
    - TIndexes 只上报 changed(index), 分组列表滚动由宿主执行; 气泡溢出组件左缘。
    - TSortable 排序只上报 reorder(from, to), 行序由宿主重建模型 (1.18 语言内没有循环语句); 拖拽中列表不滚动。
    - TFab 展开面板不是弹出层, 层叠按声明序 (宿主后续兄弟元素会盖住它), 子操作容量由宿主控制。

## 许可证

版权所有 2026 DragonheartLX, 采用 [Apache-2.0](LICENSE) 许可证。

注意: 本许可证仅覆盖 tela-ui 源码。基于 Slint 构建的应用还额外受 Slint 自身许可 (GPL-3.0 或免版税的 Slint 许可) 约束。
