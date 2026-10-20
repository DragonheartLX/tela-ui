# Changelog

本文件记录 tela-ui 的可见变更。格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### Added
- `TTable`: 列定义 + 行模型, 表头排序回调 (`sort-changed`), 行点击回调, ListView 虚拟滚动与空态; 列宽超出时水平滚动且表头同步; 排序只上报, 行重排由宿主完成。
- `common.slint`: 补齐 `TTable` 与 `TSkeleton` 导出, 与分类聚合器保持一致。