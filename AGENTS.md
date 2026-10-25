# AGENTS.md — tela-ui 协作约定

面向 AI 与人类协作者。改动前读本文件; 细节见文末文档地图。

## 写作约定

一句话一个事实, 结论放句首, 位置用 `file:line`。

README 只做入门: 不放组件统计与已知限制。统计进 `docs/architecture.md`, 限制进 `docs/components.md` 对应组件节。

## 仓库速览

- `tela-ui/` 组件库, `@tela-ui/...`; 分类聚合, `common.slint` 仅兼容。
- `ui/` + `src/` gallery (bin `tela`), Slint 1.18.1 / edition 2024 / `fluent`。

## 改动流程

1. 读 `docs/slint-semantics.md` 与 `docs/decisions.md` 相关条目; 已定取舍不翻案。
2. 改完自证: 每个改动的 `.slint` `--check` 0 诊断; 全量扫描只允许 5 个非组件数据文件报既有 "No component found"; Rust/gallery 改动跑 `cargo build`。
3. 亮/暗各渲染一张并对 `docs/harness.md` 参考值; 读不了图就交 PNG 给用户终判, 不声称"看过了"。
4. 行为/事件改动跑 Rust 探针, 放 `Temp/`; 配方见 `docs/harness.md`。
5. 同步 `common.slint`、gallery 段落、`docs/components.md` (API 与已知限制)、`docs/architecture.md` 统计、`CHANGELOG.md` (只记总体, 不分类); 清空 `Temp/`。

## 组件约定

- 可变索引/值配 `callback changed(...)`, 仅真实变化触发; 点击先自更新再回调 (TBreadcrumb 例外); 索引/页码夹紧。
- 可点击元素写 `accessible-role`/`accessible-label`; 禁用态显式 `accessible-enabled` 且动作回调自带守卫; 浮层/消息文本用 `accessible-live-region`。
- 文本用 `TText` (例外 `code-block.slint`), 不抄默认字体/字号/字重/颜色/行高; 覆盖 `font-size` 补 `line-height` 令牌; 长文本单行 `elide`, 多行 `clip: true` + 显式高度, `max-height` 无效。
- 颜色/阴影/几何走令牌, 不写硬编码色值与就地 blur/offset。
- 浮层 z 序取 `TTheme.z-*`; 模态接入 `TPopupStackLayer` 管 ESC 与层计数。
- 不覆写宿主可能绑定的属性; 非布局子元素显式 `x`/`y`; 内容尺寸容器不用 `self.width` 定位。
- struct `bool` 字段给默认值; 组件属性默认私有。

## 文档地图

一条事实只写一处, 别处只引用:

- 机制→`docs/slint-semantics.md`; 取舍→`docs/decisions.md`; API/已知限制→`docs/components.md`。
- 配方→`docs/harness.md`; 结构/统计→`docs/architecture.md`; 本地待办→`docs/TODO.md` (gitignored); 版本总览→`CHANGELOG.md`。
- Slint 技能→`skills/slint/`。
