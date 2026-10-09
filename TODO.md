# TODO

> 这份文件是**给人看的**：待办、已知差异、以及"为什么先不做"的判断。
> 改文档前 AI 该读的约定在 [`docs/AGENTS.md`](./docs/AGENTS.md)（页面形态、可运行块、互链、
> 验证流程都在那里）；仓库级的构建/发版约定在根 [`AGENTS.md`](./AGENTS.md)。

## 记录在案、暂不处理

- [ ] **`line` 的 tooltips 无法实现**：kuva 的 `LinePlot` 结构体本身没有 tooltip 字段，
      扩展侧没有地方可挂。文档里已写明「`tooltips` 接受但未实现」。
      （上游若补了字段，这里跟着接上即可。）
- [ ] `lollipop` 的 tooltips 同上（kuva 没有对应字段）。
- [ ] **`polar` 的自定义 theta 刻度标签做不了**：官方文档那一节用的是
      `TickFormat::Custom(Arc<dyn Fn(f64) -> String>)` —— 一个 Rust 闭包，JSON 表达不了。
      已经开放的具名格式（`x_axis.tick_format`）覆盖了绝大多数场景。
- [ ] **`phylo` 拿不到叶子的渲染顺序**：官方用 `leaf_labels_top_to_bottom()` 把热力图的行对齐到树上，
      那是一个 Rust 方法，值回不到 SQL 里。`phylo.md` 里已写明「改用 clustermap」。
      能补：给某种输出函数（或 `kuva_render` 的伴随函数）返回叶子顺序，再交给 `heatmap` 的
      `y_axis.categories` —— 但配套还要有「按该顺序重排矩阵行」的手段，收益有限。
      另：这一条会打破「函数只画图、不打补丁」的交互逻辑（调用方得先拿顺序、再重排数据、再画第二张图），
      所以先不做。



## 可选增强

- [ ] **`y2_axis` 的范围自动推**：第二根 y 轴按 `min`/`max` 画，只给 `name` 时那根轴不出现（series 照画）。
      现在文档里要求显式给两个；能补的话，在有 `secondary_series` 而 `y2_axis` 没给范围时，用
      `Layout::auto_from_twin_y_plots` 的结果补上。属于「更好用」而非「缺功能」。
