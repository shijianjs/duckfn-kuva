---
title: series 与通用字段
sidebar_position: 1
description: 图表的 series 列表长什么样、每个 series 都能用的字段，以及跨图型共用的值类型 —— 点、误差棒、置信带、趋势线与分组值。
---

# series 与通用字段

每张图都是一个 JSON 对象，它的 `series` 是一个**列表**。列表里每一项是一个 series，用 `type` 标明类型：

```json
{
  "series": [
    { "type": "scatter", "data": [[1, 2], [2, 3]] },
    { "type": "line", "data": [[1, 2], [2, 3]], "legend": "trend" }
  ]
}
```

用列表（而不是单个对象）是为了让一张图能把几种图型**叠加**到同一套坐标轴上；这也是规格用 JSON 而不是
DuckDB `STRUCT` 的原因：`scatter` 与 `bar` 的字段不同，`STRUCT` 的 LIST 装不下 `[StructA, StructB]`。

## 图型一览

kuva 的 64 种图型全部已实现。侧边栏按 [kuva 官方文档](https://github.com/Psy-Fer/kuva/tree/master/docs/src)
的分类方式分组：

| 分组 | `type` 取值 |
| --- | --- |
| 分布 | `histogram` · `histogram2d` · `density` · `ridgeline` · `ecdf` · `qq` · `box` · `violin` · `strip` · `raincloud` · `hexbin` · `heatmap` |
| 关系与相关 | `scatter` · `line` · `series` · `band` · `jointplot` · `contour` · `parallel` · `polar` · `ternary` · `quiver` |
| 分类与对比 | `bar` · `pie` · `waffle` · `funnel` · `pareto` · `pyramid` · `lollipop` · `slope` · `dot_plot` · `dice_plot` · `mosaic` · `venn` · `upset` · `radar` · `rose` |
| 时间序列 | `stacked_area` · `streamgraph` · `candlestick` · `waterfall` · `horizon` · `calendar` · `gantt` · `bump` |
| 统计与模型评估 | `roc` · `pr` · `survival` · `forest` · `volcano` · `manhattan` |
| 层级与网络 | `treemap` · `sunburst` · `network` · `sankey` · `chord` · `phylo` · `clustermap` |
| 3D | `scatter3d` · `surface3d` |
| 组合与工具 | `brick` · `synteny` · `text` · `legend_plot` |

少数图型还接受一个更短的别名：`dotplot`、`diceplot`、`histogram_2d`、`scatter_3d`、`surface_3d`、
`up_set`、`phylo_tree`、`interval`（对应 `band`）、`joint`（对应 `jointplot`）、`legend`（对应
`legend_plot`）。

## 每个 series 都能用的字段

下面四个字段（几乎）每种图型都接受；各图型页面不再重复它们：

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `color` | string | series 的颜色，CSS 颜色（`"steelblue"`、`"#4c72b0"`、`"rgb(…)"`）。默认黑色；除非整张图里没有任何 series 指定颜色 —— 那时按调色板轮换。 |
| `legend` | string | 这个 series 的图例文字。**只有设了它，series 才会出现在图例里。** |
| `tooltips` | boolean | 往 SVG 里注入悬停提示。`line` 未实现。 |
| `tooltip_labels` | string[] | 每个数据点一条提示文字，按数据顺序。 |

:::note[用色图编码的图型]

用连续[色图](./colormaps.md)编码数值的图型 —— `heatmap`、`histogram2d`、`hexbin`、`clustermap`、
`contour`、`dice_plot`、`calendar` 等 —— 没有 `color` 字段，改用 `color_map`。以各图型页面的字段表为准。

:::

## 点

`scatter`、`line`、`series` 与 `quiver` 接受点列表。一个点可以是一对数值，也可以是一个对象（可带逐点
误差棒）：

```json
[[1.0, 2.0], [2.0, 3.5]]
```

```json
{ "x": 1.0, "y": 2.0, "x_err": 0.2, "y_err": [0.3, 0.8] }
```

`x_err` / `y_err` 要么是一个数（**对称**误差，即 ± 该值），要么是一个 `[下, 上]` 对（**不对称**误差，
给出两条臂的长度）。

## 置信带

阴影区域是两列与数据 x 位置对齐的上下界：

```json
{ "lower": [0.8, 1.7, 2.4], "upper": [1.3, 2.4, 3.1] }
```

在 `scatter` 或 `line` series 上以 `band` 传入。两列长度都必须与数据一致。

## 趋势线

在 `scatter` series 上设 `trend`。短写法拟合一条最小二乘直线；对象写法还能设置样式并印出拟合统计量：

```json
{ "type": "linear", "color": "crimson", "width": 2, "equation": true, "correlation": true }
```

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `type` | `"linear"` | 拟合方式，`linear` 是唯一取值。 |
| `color` | string | 线颜色（默认 `"black"`）。 |
| `width` | number | 线宽。 |
| `equation` | boolean | 在图里印出回归方程 `y = mx + b`。 |
| `correlation` | boolean | 在图里印出 Pearson R²。 |

点很密时想更干净，就别开这两个，改成把数字放进[统计框](./stats-box.md)。

## 分组值

分布类图型 —— `violin`、`ridgeline`、`raincloud`、`strip`、`ecdf` 与 `qq` —— 的数据是一列 `groups`，
而不是一列平铺的数值。每组是一个标签加一列观测值，还可以给每组一个颜色：

```json
{ "label": "Control", "values": [1.2, 0.9, 1.5], "color": "steelblue" }
```

每组至少要有一个值。

## 示例

把折线与它的点从同一批行里叠出来，两者就不可能对不上：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [
    {'type': 'line', 'data': pts, 'legend': 'trend'},
    {'type': 'scatter', 'data': pts, 'legend': 'points'}
  ]
})) AS chart
FROM (
  SELECT array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

误差棒来自逐点对象；趋势线就拟合这批点：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'scatter',
    'data': pts,
    'color': 'steelblue',
    'trend': {'type': 'linear', 'equation': true, 'correlation': true}
  }]
})) AS chart
FROM (
  SELECT array_agg({'x': time, 'y': value, 'y_err': 1.5} ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_B'
);
```

## 说明

- **只有 `legend` 才会让 series 进图例。** 不给它值，series 就画成不带标签的。
- **`tooltips` 需要有能跑 JavaScript 的宿主。** 提示是内联的 SVG + JS，在浏览器预览和任何 HTML 页面里
  能用，导出成静态图就不行。
- **空的 `series` 列表是错误**，某个 series 的必填数据字段为空也是。见[函数的错误一节](../functions.md#错误)。
