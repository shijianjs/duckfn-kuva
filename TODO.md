# TODO

> 这份文件是**给人看的**：待办、已知差异、以及"为什么先不做"的判断。
> 改文档前 AI 该读的约定在 [`docs/AGENTS.md`](./docs/AGENTS.md)（页面形态、可运行块、互链、
> 验证流程都在那里）；仓库级的构建/发版约定在根 [`AGENTS.md`](./AGENTS.md)。

## 记录在案、暂不处理

- [ ] **`line` 的 tooltips 无法实现**：kuva 的 `LinePlot` 结构体本身没有 tooltip 字段，
      扩展侧没有地方可挂。文档里已写明「`tooltips` 接受但未实现」。
      （上游若补了字段，这里跟着接上即可。）
- [ ] **`polar` 的自定义 theta 刻度标签做不了**：官方文档那一节用的是
      `TickFormat::Custom(Arc<dyn Fn(f64) -> String>)` —— 一个 Rust 闭包，JSON 表达不了。
      已经开放的具名格式（`x_axis.tick_format`）覆盖了绝大多数场景。
- [ ] `lollipop` 的 tooltips 同上（kuva 没有对应字段）。
- [ ] **`rose` 的方位角模式没开放**：kuva 有 `with_bearing_data(bearings, n)` 与
      `with_compass_labels()`（把原始 0–360° 方位角自动分箱、并换成 `N / NE / E …` 标签）。
      我们的 schema 只有 `slices` / `series`，要自己先分好箱、自己写方位标签。
      能补：给 `RoseSpec` 加 `bearings` + `bearings_bins` + `compass_labels`。
- [ ] **`lollipop` 的分类 x 轴没开放**：kuva 的 CLI 支持字符串 x（变成分类轴），但我们的
      `LollipopPointSpec.x` 是 `f64`。现在文档里教的是「用 `row_number()` 当 x、分类名放 `label`」。
      能补：把 `x` 改成枚举（数字或字符串）。
- [ ] **`brick` 的侧翼序列（`flanked_strigars`）没开放**：官方有
      `(left_flank, motifs, strigar, right_flank)` 的便利构造器。这里只能用 `@` 段把侧翼写进 motif 串，
      而渲染器不会给它们上色（报 `value not found in template colormap`），所以 `brick.md` 里已写明
      「拆成两张图或不画侧翼」。能补：给 `BrickSeries` 加 `flanked_strigars`（四元组列表）。
      `start_positions` 同理（官方是 `x_offsets` 取负），文档里已给了取负的写法。
- [ ] **`clustermap` 不能给预建的树**：官方有 `with_row_tree` / `with_col_tree`（把已知拓扑或
      scipy/R 的 linkage 结果直接套到某一侧），我们的 `ClustermapSeries` 没有这两个字段，
      两侧永远由 UPGMA 聚类。`clustermap.md` 里已写明。能补：加 `row_tree` / `col_tree`，
      复用 `PhyloSeries` 的输入形式（newick / edges / distance_matrix / linkage）。
- [ ] **`phylo` 拿不到叶子的渲染顺序**：官方用 `leaf_labels_top_to_bottom()` 把热力图的行对齐到树上，
      那是一个 Rust 方法，值回不到 SQL 里。`phylo.md` 里已写明「改用 clustermap」。
      能补：给某种输出函数（或 `kuva_render` 的伴随函数）返回叶子顺序，再交给 `heatmap` 的
      `y_axis.categories` —— 但配套还要有「按该顺序重排矩阵行」的手段，收益有限。

## 可选增强

- [ ] `legend.entries`（手工图例条目）目前只在 `reference/legends.md` 里讲了。
      等各图型页稳定后，可以在「颜色编码在数据里」的图表页（strip 逐点颜色、heatmap 色条…）
      的「另见」里互链一下。
- [ ] `colorbar_tick_format` 同理（`reference/layout.md` + `histogram2d` 已写）。