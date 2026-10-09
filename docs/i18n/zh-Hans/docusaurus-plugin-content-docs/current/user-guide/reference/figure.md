---
title: 多面板图（Figure）
sidebar_position: 12
description: 把若干面板排成网格 —— 合并单元格、共享坐标轴、一个共用图例、逐面板的尺寸。
---

# 多面板图（Figure）

在顶层加一个 `figure` 对象，渲染就从「一块面板」切到「一片网格」。每一格都是一个完整的面板 —— 自己的坐标轴、
标题与系列 —— 网格在它们外面加一圈共享的框。面板按**行优先**填充：从左到右、从上到下。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `rows` / `cols` | integer | 网格大小，都至少为 1。 |
| `panels` | panel[] | 逐个面板，按行优先顺序。长度必须等于 `rows * cols`；给了 `structure` 时等于它的长度。 |
| `structure` | integer[][] | 合并单元格：每个内层数组列出组成同一个面板的格子下标。格子 `0` 在左上角，编号按行优先。 |
| `title` / `title_size` | string / integer | 跨整张图的标题。 |
| `labels` | string \| string[] \| object | 面板标签：`"uppercase"` · `"lowercase"` · `"numeric"` · `"none"`，也可以给自定义数组，或 `{"names": [...], "size": …, "bold": …}`。 |
| `shared_x_all` / `shared_y_all` | boolean | 所有面板共用一套范围。 |
| `shared_x_cols` / `shared_y_rows` | integer[] | 在这些**列**内共用 x 范围 / 在这些**行**内共用 y 范围。 |
| `shared_x_slices` / `shared_y_slices` | object[] | 只在某列/某行的一段内共用：`{"index": …, "start": …, "end": …}`（两端都含）。 |
| `shared_legend` | string | 整张图一个图例，位置是图级别的（如 `"right_top"`、`"bottom"`）。 |
| `shared_legend_entries` | entry[] | 那个共享图例用手写条目，而不是从各面板收集。 |
| `keep_panel_legends` | boolean | 是否同时保留各面板自己的图例（默认有共享图例就不画）。 |
| `spacing` / `padding` | number | 面板之间的间隙 / 图边缘的内边距。 |
| `cell_width` / `cell_height` | number | 单个面板的尺寸；**两个都要给**。 |
| `figure_width` / `figure_height` | number | 整张图的尺寸；**两个都要给**，优先于单元格尺寸。 |
| `row_heights` / `col_widths` | object | 逐行 / 逐列的尺寸覆盖，键是 0 起的下标：`{"2": 80}`。 |

## 基本网格

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'title': 'Two views',
    'panels': [
      {'title': 'scatter', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter',
                   'data': (SELECT array_agg([x, y]) FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))}]},
      {'title': 'y distribution', 'x_axis': {'name': 'y'}, 'y_axis': {'name': 'count'},
       'series': [{'type': 'histogram', 'bins': 20,
                   'values': (SELECT list(y) FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))}]}
    ]
  }
})) AS chart;
```

每个面板各自按自己的数据定范围，所以只有在你需要几个面板对齐时，才去给轴范围（见「共享坐标轴」）。某个面板的
`series` 为空是报错，而不是画一个空格子。

## 合并单元格

`structure` 用来跨格。每个内层数组列出组成一个面板的格子下标，此时 `panels` 是**一组一项**，而不是一格一项：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 2, 'cols': 3,
    'title': 'Three views over one series',
    'structure': [[0], [1], [2], [3, 4, 5]],
    'panels': [
      {'title': 'scatter', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d)}]},
      {'title': 'y distribution', 'x_axis': {'name': 'y'}, 'y_axis': {'name': 'count'},
       'series': [{'type': 'histogram', 'values': (SELECT list(y) FROM d), 'bins': 20}]},
      {'title': 'x distribution', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'count'},
       'series': [{'type': 'histogram', 'values': (SELECT list(x) FROM d), 'bins': 20}]},
      {'title': 'y over x, joined', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'line', 'data': (SELECT array_agg([x, y] ORDER BY x) FROM d)}]}
    ]
  }
})) AS chart;
```

2×2 里「左列一整条高面板」是同一套写法：`structure` 写成 `[[0, 2], [1], [3]]` —— `0` 与 `2` 正是左列的
上下两格。

## 共享坐标轴

共享之后，相关联的几个面板用同一套范围，内侧边上重复的刻度标签也不再画 —— 这正是让一排面板能一眼比较的原因。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'shared_y_all': true,
    'panels': [
      {'title': 'all points', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d)}]},
      {'title': 'upper half', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d WHERE y > 4)}]}
    ]
  }
})) AS chart;
```

| 字段 | 共享范围 |
| --- | --- |
| `shared_x_all` / `shared_y_all` | 所有面板 |
| `shared_x_cols` / `shared_y_rows` | 某一列 / 某一行（给 0 起的下标） |
| `shared_x_slices` / `shared_y_slices` | 某列/某行的一段：`{"index": 0, "start": 0, "end": 1}` |

## 面板标签

`labels` 是四种内置样式的简写，也可以给一个自定义字符串数组；再想调字号与粗细就给对象：

```json
{ "labels": {"names": ["i", "ii", "iii"], "size": 14, "bold": false} }
```

`"uppercase"` 是 A、B、C…；对象写法不给 `size` / `bold` 时用 16 px 加粗，也就是 `"uppercase"` 的效果。

## 共享图例

整张图一个图例，而不是每个面板各来一个 —— 画布会变宽（或者下边距变大）来容纳它，各面板自己的图例则不再画：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'shared_legend': 'right_top',
    'panels': [
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'legend': 'measured', 'data': (SELECT array_agg([x, y]) FROM d)}]},
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'line', 'legend': 'smoothed',
                   'data': (SELECT array_agg([x, y] ORDER BY x) FROM d)}]}
    ]
  }
})) AS chart;
```

`shared_legend` 收 `right` · `right_top` · `right_middle` · `right_bottom` · `left_top` · `left_middle` ·
`left_bottom` · `top_left` · `top_center` · `top_right` · `bottom` · `bottom_left` · `bottom_center` ·
`bottom_right`（大小写与分隔符不敏感）。想自己写条目、并且保留各面板的图例，就用 `shared_legend_entries`
与 `keep_panel_legends`：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'shared_legend': 'bottom',
    'keep_panel_legends': true,
    'shared_legend_entries': [
      {'label': 'measured', 'color': 'steelblue', 'shape': 'circle'},
      {'label': 'trend', 'color': 'crimson', 'shape': 'line', 'dasharray': '6 4'}
    ],
    'panels': [
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'legend': 'measured', 'data': (SELECT array_agg([x, y]) FROM d)}]},
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'line', 'legend': 'trend', 'data': (SELECT array_agg([x, y] ORDER BY x) FROM d)}]}
    ]
  }
})) AS chart;
```

## 尺寸与间距

`cell_width` / `cell_height` / `figure_width` / `figure_height` 都别给。kuva 默认单元格是 `500 × 380`，
让它自己排，每个面板的比例才和单张图一致；把整张图钉进一个又宽又扁的盒子，会把里面每个面板都压扁。

| 字段 | 作用 |
| --- | --- |
| `cell_width` / `cell_height` | 单个面板的像素尺寸（默认 `500 × 380`）；两个都要给。 |
| `figure_width` / `figure_height` | 整张图的尺寸，单元格按剩余空间算；优先于单元格尺寸。 |
| `row_heights` / `col_widths` | 某一行的高度 / 某一列的宽度，例如 `{"2": 80}` 做一条细注记带。 |
| `spacing` / `padding` | 面板之间的间隙（默认 `15`）与网格外的边距（默认 `10`）。 |

产出的 SVG 尺寸由单元格尺寸、间距、边距、标题以及共享图例一起算出来，这些都不用你手工配平。

## 面板

`panels` 的每一项与单张图的配置是同一个对象，只是没有 `figure` —— `title`、`x_axis`、`y_axis`、`grid`、
`legend`、`annotations`、`stats_box`、`series` 等等。某个面板的 `series` 不能为空。

## 说明

- **`panels` 要与网格对得上**：一格一项；用了合并单元格时，则是一组一项。
- `structure` 的每一组都必须能拼成一个**实心矩形** —— L 形会被拒，而不是按包围盒画成别的样子。同一个格子
  也不能出现两次，更不能越界。
- 第二根坐标轴属于单个面板；一个面板也可以像单张图那样叠加多个系列。
- 面板标签是逐个面板画的；图级别的 `title` 在所有网格之上。

## 另见

- [图例](./legends.md) —— 条目形状、位置与手工条目。
- [画布、标题与坐标轴](./layout.md) —— 面板继承的全部字段。
- [网格、刻度与画布开关](./grid.md) —— 网格线、轴线与 `bw_mode`。
