---
title: 函数
sidebar_position: 3
description: duckfn_kuva 注册的 SQL 函数与它接受的 JSON 图表规格，都配了能在浏览器里直接跑的例子。
---

# 函数

加载扩展会注册一个函数。它和 DuckDB 自带的函数一样用：可以放进投影、`WHERE` 条件或 `GROUP BY`，也能和
内置函数随意组合。

| 函数 | 类别 | 签名 | 说明 |
| --- | --- | --- | --- |
| `kuva_render` | 标量 | `VARCHAR -> VARCHAR` | 把一段 JSON 描述的图表渲染成一份 SVG 文档。 |

## kuva_render

```text
kuva_render(spec_json VARCHAR) -> VARCHAR
```

`kuva_render` 包装了 [kuva](https://crates.io/crates/kuva) —— 一个纯 Rust 的统计绘图库。它的入参是一段
描述单张图的 JSON，出参是一份完整的 SVG 文档字符串。绘图发生在扩展内部，所以同一张图在 DuckDB 能跑的
任何地方都长一样 —— CLI、Python / R 会话、JVM 宿主，或浏览器里的 DuckDB-Wasm —— 宿主机上不需要
matplotlib / ggplot2。

最小可用的一次调用 —— 读一对列，然后把它画出来：

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
-- 点一下 Run：结果是一整份 SVG 文档，直接画在这里
SELECT kuva_render(to_json({
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS chart
FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/scatter.tsv');
```

下面每个块都写了 `"show":"svg"`，点 **Run** 就会把图画在结果区里；缩放与平移在全屏里。

为什么是 JSON 而不是 DuckDB 的 `STRUCT`：一张图里的 `series` 是异构的（`scatter` 与 `bar` 的字段各不
相同），而 `STRUCT` 的 LIST 要求元素同型，表达不了 `[StructA, StructB]`。键名一律 snake_case。

### 顶层字段

除 `series` 外都可选：

| 字段 | 设置什么 |
| --- | --- |
| `series` | **必填。** 要画的 series 列表。 |
| `title` | 图的标题。 |
| `x_axis` / `y_axis` | 坐标轴配置（见下）。 |
| `grid` | 网格线、坐标轴线与刻度。 |
| `legend` | 图例的开关、位置、标题与排布。 |
| `theme` | `"light"` / `"dark"` / `"minimal"` / `"solarized"`，或用对象逐个覆盖颜色。 |
| `palette` | 具名调色板，或一组颜色字符串。 |
| `font` | 字体族与字号（`title_size`、`label_size`、`tick_size`、`body_size`）。 |
| `annotations` | 参考线、阴影区域与文字标注。 |
| `width` / `height` | 画布尺寸。 |
| `figure` | 切换成多面板网格（见[组合多种图](#组合多种图)）。 |

### series

`series` 是一串对象，每个用 `type` 区分。目前实现了六种：

| `type` | 主要字段 |
| --- | --- |
| `scatter` | `data`、`size` / `sizes` / `colors`、`marker`（circle/square/triangle/diamond/cross/plus）、`marker_opacity`、`marker_stroke_width`、`trend`、`band`、`group_name`。 |
| `line` | `data`、`stroke_width`、`line_style`（solid/dashed/dotted/dash_dot 或自定义 dasharray 字符串）、`step`、`fill`、`fill_opacity`、`band`。 |
| `bar` | `categories` + `values`（简单模式），或 `series` + `stacked` / `horizontal`（分组 / 堆叠）；`width`、`gap`、`colors`、`errors`、`error_color`、`error_cap_width`。 |
| `histogram` | `values` + `bins` / `range` / `normalize`，或预分箱的 `edges` + `counts`；另有 `kde`、`kde_color`、`kde_bandwidth`、`kde_samples`。 |
| `box` | `groups`（`[{"label":…,"values":[…]}]`）、`colors`、`width`、`gap`、`horizontal`、`strip`、`swarm`、`overlay_color`、`overlay_size`、`notch`、`notch_depth`、`notch_width`。 |
| `pie` | `slices`（`[{"label":…,"value":…}]`）、`inner_radius`（> 0 即环形图）、`label_position`（auto/inside/outside/none）、`percent`、`min_label_fraction`。 |

每个 series 还都接受 `color`、`legend`、`tooltips` 与 `tooltip_labels`。

`scatter` 与 `line` 的 `data` 是一串点，可以写成 `[x, y]`，也可以写成对象
（`{"x":…,"y":…,"x_err":…,"y_err":…}`）；误差是单个数字表示对称，`[下, 上]` 表示不对称。

这六个块跑在绘图库自带的[示例数据集](https://github.com/Psy-Fer/kuva/tree/master/examples/data)上，
数据由本站直接供出 —— 真实查询也正是这样拼 spec 的：先把行聚合成 series 要的
`data` / `categories` / `values` / `slices`，再把整个对象交给 `to_json`。

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/scatter.tsv')
  GROUP BY "group"
);
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': list({'type': 'line', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/measurements.tsv')
  GROUP BY "group"
);
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/bar.tsv');
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 20}]
})) AS chart
FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/histogram.tsv');
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [{'type': 'box', 'groups': list({'label': g, 'values': vals} ORDER BY g)}]
})) AS chart
FROM (
  SELECT "group" AS g, list(expression) AS vals
  FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/samples.tsv')
  GROUP BY "group"
);
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'pie',
    'slices': list({'label': feature, 'value': percentage} ORDER BY percentage DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/pie.tsv');
```

### 坐标轴

`x_axis` 与 `y_axis` 接受：

| 字段 | 设置什么 |
| --- | --- |
| `name` | 轴标题。 |
| `categories` | 类别轴的刻度标签。 |
| `min` / `max` | 固定上下界。 |
| `log` | 对数轴。 |
| `tick_format` | `auto` / `integer` / `sci` / `percent` / `degree`，或一个整数表示定点小数位。 |
| `tick_rotate`、`label_overlap`（allow/thin/stagger）、`wrap` | 刻度标签的排布。 |

### 网格、图例、主题与调色板

`grid` 包含 `show_grid`、`ticks`、`axis_line`（open/box）、`tick_align`、`tick_pos`、
`grid_line_width`、`axis_line_width`、`tick_width`、`tick_length`、`minor_ticks`、`show_minor_grid`、
`clamp_axis`、`clamp_y_axis`、`bw_mode`、`interactive`、`equal_aspect`、`scale` 与 `label_background`。

`legend` 包含 `show`、`position`（例如 `outside_right_top`、`inside_top_left`、`outside_bottom_columns`）、
`title`、`show_box`、`width`、`height`、`col_limit`、`entry_limit`、`wrap`、`at` 与 `at_data`。

`theme` 是四种具名主题之一，或一个对象，覆盖 `background`、`axis_color`、`grid_color`、`tick_color`、
`text_color`、`legend_bg`、`legend_border`、`pie_leader`、`box_median`、`violin_border`、
`colorbar_border`、`font_family` 与 `show_grid`。

`palette` 是具名调色板 —— `wong`、`okabe_ito`、`tol_bright`、`tol_muted`、`tol_light`、`ibm`、
`deuteranopia`、`protanopia`、`tritanopia`、`category10`、`pastel`、`bold` —— 或一组颜色字符串。

### 标注

`annotations` 里有三组列表：

- `reference_lines`：`{"orientation":"horizontal"|"vertical","value":…,"color":…,"stroke_width":…,"dasharray":…,"label":…}`。
- `shaded_regions`：`{"orientation":…,"min":…,"max":…,"color":…,"opacity":…}`。
- `texts`：`{"text":…,"x":…,"y":…,"target_x":…,"target_y":…,"color":…,"font_size":…,"arrow_padding":…}`。

### 组合多种图

支持两种组合方式。

**叠加。** 在一个 `series` 里放多个 series，它们共用一套坐标轴。比如一条折线加它的散点（两者都由同一
批行生成，因此不会各走各的）：

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [
    {'type': 'line', 'data': pts, 'legend': 'trend'},
    {'type': 'scatter', 'data': pts, 'legend': 'points'}
  ]
})) AS chart
FROM (
  SELECT array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

**多面板。** 用顶层的 `figure` 对象代替单张画布。它带 `rows`、`cols`、`title`、`title_size`、
`labels`（`"uppercase"` / `"lowercase"` / `"numeric"` / `"none"` 或自定义数组）、`shared_x_all`、
`shared_y_all`、`shared_legend`（位置字符串，如 `"right_top"`）、`spacing`、`padding`、`cell_width`、
`cell_height`、`figure_width`、`figure_height`，以及 `panels` —— 每个格子一个对象，各自带布局字段与
`series`。`panels` 的个数必须正好等于 `rows * cols`，按行优先排列。

```sql {"type":"duckfn","show":"svg","option":{"height":"360px"}}
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'panels': [
      {'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/scatter.tsv'))}]},
      {'series': [{'type': 'histogram', 'values': (SELECT list(value) FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/histogram.tsv')), 'bins': 20}]}
    ]
  }
})) AS chart;
```

不要设 `figure_width` / `figure_height`，让 kuva 用它默认的单元格尺寸（每个 `500x380`）来排布面板，
这样单个面板的比例与单张图一致。把整张 figure 钉成又宽又扁的框，会把里面每个面板都压扁。

## 错误

任何失败 —— JSON 不合法、字段类型不对、`series` 为空、长度不一致，等等 —— 都会让整条查询失败，而不是
返回 `NULL`。错误信息是英文，且一律以 `kuva_render: ` 开头：

```sql {"type":"duckfn","expect":"error"}
SELECT kuva_render('{"series":[]}');   -- 报错：`series` 不能为空
```

## 几点说明

- **渲染失败会让整条语句失败。** 报错信息里带函数名，查询剩下的部分不会再执行。不会有任何东西被悄悄
  变成 `NULL`。
- **结果是一个字符串，不是文件。** `kuva_render` 返回 SVG 文本；把它落盘或对外提供，由调用方决定
  （例如 `COPY (SELECT kuva_render(…)) TO 'chart.svg'`）。
- **JSON 是回退 API。** 它之所以用 JSON，是因为 `series` 异构；以后可以在同一个渲染器之上再包一层
  SQL 友好的 API（每个图型一个函数、`STRUCT` 参数）。