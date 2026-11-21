# Changelog

本文件只记录版本级总体变更; 逐组件细节、机制与取舍见 `docs/components.md`、`docs/slint-semantics.md` 与 `docs/decisions.md`。

## [0.1.3] - 2026-11-21

### Changed
- 升级 Slint 1.18.1 → 1.19.0 (gallery 的 slint / slint-build 依赖): 保留官方要求的兼容 feature `compat-1-18`; gallery 构建启用 Slint 语言 edition 1.19 (1.19 的行为修复为 opt-in; 经渲染逐像素对比, tela-ui 组件行为与 1.18 一致, 见 `docs/slint-semantics.md` 头注)。
- gallery 新增 `slint-project.json` (Slint 项目文件: 语言 edition 1.19 / main `ui/MainWindow.slint` / library-paths `tela-ui`), 供工具链 (LSP / live-preview / slint-viewer) 识别 edition 与库路径; `build.rs` 的 `CompilerConfiguration` 仍优先, 二者一致无冲突。
- tela-ui 库新增 `slint-project.json` (Slint 库项目文件: 语言 edition 1.19, 无 `main`——库由消费者目录式挂载; 项目文件供工具链/LSP 编辑库源码时识别 edition; 经构建与渲染验证 gallery 编译与渲染不变)。

### Fixed
- 无障碍: 输入类组件 (TInput / TTextArea / TMention / TSelectInput) 内部 TextInput/TInput 设 `accessible-role: none` 并在组件根补 `accessible-value`, 消除 Slint 1.19 新增的"嵌套 text-input 重复暴露"诊断, 读屏不再重复朗读输入框 (对齐官方 LineEdit 模式)。
- 适配 Slint 1.19 新增诊断: 清理全部"属性设为默认值"冗余绑定 (编译器保证行为不变), 并移除 TImageViewer 布局内冗余 y。
- 修复 gallery 代码样本门禁脚本 (`scripts/check-code-samples.py`): 路径改为 workspace 结构 (`gallery/ui/pages`), `slint-viewer` 调用加 `--edition 1.19`, 且仅以 error/warning 判失败 (Slint 1.19 新增 info 级"绑定到默认值"建议不再误判样本)。

## [0.1.2] - 2026-11-15

### Changed
- 仓库改为 Cargo workspace: 根包 `tela-ui` 为纯库 (去掉 slint 依赖, 仅 `import_paths()` 出口), 展示二进制独立为 `gallery/` 成员 (bin `tela`, slint / slint-build 依赖移到此处); 发布包改为只含 Rust 出口与 `tela-ui/` Slint 源。

## [0.1.1] - 2026-11-06

### Added
- 新增展示组件 `THeatmap` (瓦片网格热力图, 行主序五档着色, 悬停标签气泡与图例), 随 `@tela-ui/display` 聚合导出; 细节见 `docs/components.md`。

### Fixed
- 修复 gallery Windows 构建窗口圆角外黑底: slint 依赖弃 `renderer-skia` 改 `renderer-femtovg` (skia 的 DX12 wgpu 表面建不出 premultiplied alpha, 上游 slint#13572 / #4120), 全平台一致; 上游修复后可换回 skia。

## [0.1.0] - 2026-11-05

首个版本: 面向 Slint 1.18.1 的 TDesign 风格组件库。

- 116 个导出: 100 个 UI 组件 (基础 12 / 输入 14 / 选择 11 / 日期时间 5 / 展示 12 / 内容 20 / 提示 4 / 浮层 6 / 导航 15 / 实验 1) + 9 个服务浮层 + 5 个库内基类 + 2 个库内部件; 按交互类型聚合导出 (`@tela-ui/base` 等), `@tela-ui/common.slint` 为全量兼容出口。
- 服务型浮层: 对话框 / 加载 / 消息 / 通知 / 图片预览 / 命令面板 / 动作面板 / 提示 / 漫游引导; `TPopupSurface` + `TPopupStack` 统一管层计数、ESC 与 z 序。
- 主题: `TTheme` 全局 151 个顶层令牌转发至 `TAppThemeConfig` 103 字段; 亮/暗色板见 `colors.slint`; 组件模式偏好走标准库 `Palette.color-scheme`; `TIconSet` 内置 46 字形 (Lucide 几何数据)。
- 滚动: `TScrollBar` 滚动容器 + 共享滑块 (视口同步 / `scroll-to` 缓动), 替代 std ScrollView; `TList` / `TTable` 用内部 `TScrollThumb` 叠加条。
- 可访问性: 组件 accessible role / label / enabled 与默认动作, 浮层消息用 accessible-live-region。
- gallery (bin `tela`): 11 页组件陈列, 亮/暗双主题, `TELA_THEME` 可强制主题; 随包附带亮/暗截图基线与变改探针配方 (`docs/harness.md`)。
