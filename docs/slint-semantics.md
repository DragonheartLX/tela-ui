# Slint 1.18 语义实证

> 每条都对着锁定版本源码或本仓库生成产物核对过, 不是猜测。
> 引用时写 `docs/slint-semantics.md` 第 N 条。

1. **Rectangle 不消费指针事件** — `input_event` 返回 `EventIgnored`, 遍历 FrontToBack 继续; 模态卡片自身无 TouchArea 时点正文会落到遮罩, 面板内放空白 `TouchArea {}` 吸收。

2. **`visible` 与 `animate` 互斥** — `visible: false` 不渲染但仍实例化、动画照跑; 退场用 `opacity`, 真销毁用 `if`。

3. **整份模型替换 ≠ 单行写入** — 整数组赋值走 `RepeaterTracker::reset()` 重建实例、私有属性归零; `items[i] = x` 走 `row_changed` 保留实例; 依赖槽位私有状态要自带 stamp 并在 `changed` 时 restart。

4. **`Timer.running` 默认 `true`** — 常驻耗电; `restart()` 只重启此前启动过的, 改成 `running: false` 后必须 `start()`。

5. **无障碍属性只对 Text / TextInput 自动推导** — Rectangle 派生组件禁用态对读屏不可见, 要显式 `accessible-enabled`; `accessible-action-default` 在 disabled/loading 仍触发, 回调自带守卫。

6. **`to-float()` 失败或空串 = 0** — 数字输入清空会夹到 min 并触发 `changed`。

7. **`Text` 默认 `overflow: clip` 与 `wrap: no-wrap`** — 省略显式 `elide`, 换行显式 `word-wrap`。

8. **赋值会移除绑定** — 不对宿主可能绑定的属性做命令式赋值; 复用组件不替宿主决定可见性。

9. **`a <=> b` 初值取右侧** — `in` 属性也可双向绑定; 无修饰 `property` 默认私有。

10. **带 `animate` 的绑定首次求值不跑动画** — 动画只在依赖变化后启动, 挂载/切页不会全量触发。

11. **`animate` 支持 `enabled:` 关断** — 关断当帧直达目标; 要"索引变才动画、几何即时跟随"就把动画放标量镜像属性, 几何属性非动画引用 (TCarousel 的 `slide-pos`)。

12. **Timer 是布局不可见元素** — 可直接放布局, 不占槽位、不触发 spacing; 不需要 0 高 Rectangle 包装。

13. **未设 `font-family` 的 `Text` 继承 `Window.default-font-family`** — 仍为空则落到渲染器通用族 (本机 Arial); 库内文本统一走 `TText`。

14. **`line-height-factor` 是相对自然行盒的系数, 不是 px** — 单行只改首选高度; Microsoft YaHei 自然行盒 = `ceil(字号 × 1.3198)`; `TText` 按 `line-height ÷ ceil(字号 × 1.3198)` 换算。

15. **无障碍声明只落在显式写了 `accessible-role` 的元素上** — role 须常量表达式, 其余 accessible-* 可动态绑定; 生成代码枚举变体带 `r#` 前缀。

16. **布局里的 `Text` 不会纵向收缩** — `max-height` 无效; 封顶只能 `clip: true` 包装盒 + 显式高度。

17. **`pure function` 让编译器去重属性读取** — 可读自身属性; 单元格重复读加 `pure` 即可, 不必摊成参数。

18. **`overflow: elide` 不改 `preferred-width`** — 省略只影响绘制; 宿主按 `preferred-width` 量宽仍按全宽。

19. **`animate` 的参数在同一帧内改不生效** — `enabled` / `duration` 在重新武装时读取; 开关与值同 tick 写不缓动, 隔一帧正常。

20. **`visible: false` 不改变布局占用** — 降级为 visibility Clip; 整行消失要自己塌陷高度 + `spacing: 0px`, 把行距烘进行高。

21. **1.18 没有循环语句, 数组只有 `length`/`push`/`remove`/`insert`** — `index-of` 等在实验开关后, 函数不能递归; 过滤/找下一条只能元素树表达 (命令面板每行上报下标取最小)。

22. **内容尺寸容器用 `self.width` 定位会画在第一轮坐标上** — 首轮 `self.width` 约等于父宽, 容器被放到 x≈-8; 给显式宽/固定 `x`, 或元素直接 `x`/`y` 定位。单帧 `--screenshot` 必现, 布局落定后不现。

23. **显式 `width` 压过 `horizontal-stretch`** — 写了 `width` 就不参与剩余空间分配; 整行按钮由外层布局撑满 (`block` 参数已删)。

24. **TouchArea 的点击判定把拖拽也算一次点击** — 快速两次滑动可凑成 `double-clicked`; 双击分支用私有 flag 排除拖拽; 探针每次释放后推进 600ms。

25. **std `ScrollView` 的内层 Flickable 不认祖先输入闸** — `enabled` 只管滚动条, 关闭态面板仍吞滚轮; 阻断只能用 `visible: false` (等淡出), 裸 Flickable 可 `interactive: false`。

26. **布局外的子元素默认在父级居中** — 撑满父级的轴除外; 整屏覆盖层、整行条带必须显式给 `x`/`y`。

27. **ListView 只自动算 `content-height`** — 横向 `content-width` 要自给, 行宽同值, 表头绑 `content-x` 跟随。

28. **ListView 的 `content-y` 归内部 Flickable** — 外部绑定/赋值留不住; 要数据整体滚动就整列渲染 + 自维护偏移并 `animate`。

29. **`for it[i]` 里写 `it.field = v` 会写回模型, 即使模型是 `in`** — 循环变量是引用; 但子行显隐仍需嵌套声明 (TTree)。

30. **实例化处的绑定覆写组件根的绑定** — 反向不成立; 根绑定拿不到 `parent`, 横/竖静态差异只能拆子类。

31. **布局分配规则** — (a) 显式 `width` 超容不压缩, 改 `preferred-width` + `min-width: 0` + `stretch: 0`; (b) 总 stretch ≤ 0 时全员按 1 均分, 占位元素要显式 `stretch: 1`; (c) 内层布局写 `width: parent.width` 会弄脏组件首选尺寸。

32. **全定宽子元素的嵌套行会把拉伸列塌缩到首选宽** — 行尾放空 `Rectangle { }` 吸收位恢复无界 max; 裸 `Text` 不触发。

33. **整页实例化 10 轮预算** — 超预算后"模型长度/自身首选尺寸"类几何首帧饿死 (TSortable 高度、TTabBar 标签布局); 修法: 固定视口高、for 体内固定几何、gallery 每页一个 TTable; 无显式高度的包装盒同样饿死, 给显式高度。HEAD 真实挂载本就 3 警告, 该警告不是回归; 回归是超得更多后连行都不渲染。

34. **HorizontalLayout 里固定高 TText 的显式 `y` 被忽略** — 补 `width: self.preferred-width` 后生效; 不要用 Rectangle 包文本读 preferred 居中 (成环不渲染)。

35. **`Palette.color-scheme` 是模式偏好的唯一存储, 写入即接管** — 写入替换"跟随 OS"绑定, 回跟随只能写 `unknown`; `unknown` 下标准库按 OS 解析, `AboutSlint` logo 读原始值恒为亮色; 单帧 `--screenshot` 首帧是 `unknown`。

36. **Flickable 在任何左键按下时移除 `content-x`/`content-y` 绑定** — `animate content-y` 活不过第一次按压; 缓动必须组件内 Timer 逐帧赋值 (Timer 写入是普通 set)。

37. **Flickable 无 overscroll 信号, 四条规则决定下拉形态** — (a) 无 overscroll 回调; (b) 越界自动夹回, 拉动量放独立属性平移包装组; (c) `mouse-drag-pan-enabled: false` 让手势层接管鼠标, Wheel 恒被 Flickable 拦, 带 `scroll-event` 的上层 TouchArea 先收 (accept 停, reject 下穿); (d) 未显式设置时 `content-height` 取直接布局子的 min-height, repeater 不参与 — 显式绑定是滚轮受理前提。`delta-y > 0` = 内容向上滚。

38. **struct 字段默认值经 "repeater 变量 → 子组件属性" 丢失** — 省略字段以零值变体到达 (枚举取第一项); 消费端映射回期望值或要求传全字段。

39. **条件重挂载窗口内 in_bound 夹持会吃掉恢复的滚动偏移** — 塌缩期写 `content-y` 被夹回; 对策: 100ms 常驻节拍保存、适应式重申 (装得下才断言、高度变化重置计数)、连续 8 拍收手。滚轮/时序结论要真机验证 (winit 可真驱动滚轮, 但测不出 UIA 拖慢)。

40. **TouchArea 的 `mouse-x`/`mouse-y` 相对元素自身** — 元素平移后位移反馈自激; 用 `mouse-x + (offset - press-offset)` 换成窗口空间位移。
