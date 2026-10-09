---
title: 棒棒糖图
sidebar_position: 7
description: 每个值一根杆加一个点，可选在杆后画区间带。
---

# 棒棒糖图

棒棒糖图把每个值画成一根杆、顶端一个点。它和[柱状图](./bar.md)承载的信息一样，但视觉更轻 —— 杆与杆之间的空白
让相邻高度更好比较。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, expression,
         row_number() OVER (ORDER BY expression DESC) AS pos
  FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Expression by gene',
  'x_axis': {'name': 'rank', 'tick_format': 'integer'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'lollipop',
    'points': (SELECT list({'x': pos, 'y': expression, 'label': gene} ORDER BY pos) FROM d),
    'color': 'steelblue'
  }]
})) AS chart;
```

这里的 `x` 是数值，所以用 `row_number()` 把行排成等间距的杆，基因名则作为每个点的标签跟着走。

## 标签与逐点颜色

一个点可以带 `label`，也可以有自己的 `color` —— 想单独点出几个值得命名的条目时就靠它。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, expression,
         row_number() OVER (ORDER BY expression DESC) AS pos
  FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Top-ranked genes',
  'x_axis': {'name': 'rank', 'tick_format': 'integer'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'lollipop',
    'points': (SELECT list(CASE WHEN pos <= 5
                                THEN {'x': pos, 'y': expression, 'label': gene, 'color': '#d62728'}
                                ELSE {'x': pos, 'y': expression} END ORDER BY pos) FROM d),
    'dot_radius': 5.5
  }]
})) AS chart;
```

::::note[这个 CASE 的两支要同型]

`CASE` 的两支都产出 struct，DuckDB 会把它们的类型统一 —— 所以带标签的那一支必须把相同的键写全。形状要是别扭
起来，就分两步拼这个列表，或者退回一张小的内联 `VALUES` 表。

::::

## 区间带

`domains` 在杆后面画彩色带子，锚在基线下方。蛋白突变图谱的标准画法就是这样：功能域沿序列标出来。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ( 12.0,  3.0, 'R12H'),
    ( 35.0,  8.0, 'G35V'),
    ( 67.0,  2.0, 'K67R'),
    ( 94.0,  5.0, 'P94L'),
    (118.0, 11.0, 'R118*'),
    (145.0,  4.0, 'T145A'),
    (173.0,  7.0, 'D173N'),
    (201.0,  3.0, 'E201K')
  ) AS t(pos, cnt, mut)
)
SELECT kuva_render(to_json({
  'title': 'Mutation landscape',
  'x_axis': {'name': 'amino acid position', 'tick_format': 'integer'},
  'y_axis': {'name': 'count'},
  'series': [{
    'type': 'lollipop',
    'points': (SELECT list({'x': pos, 'y': cnt, 'label': mut} ORDER BY pos) FROM d),
    'domains': [
      {'start': 1,   'end': 55,  'label': 'N-term', 'color': '#4e79a7'},
      {'start': 56,  'end': 130, 'label': 'Kinase', 'color': '#f28e2b'},
      {'start': 131, 'end': 195, 'label': 'SH2',    'color': '#59a14f'},
      {'start': 196, 'end': 240, 'label': 'C-term', 'color': '#b07aa1'}
    ],
    'domain_height': 0.8,
    'stem_width': 1.5,
    'dot_radius': 5
  }]
})) AS chart;
```

`domain_height` 的单位是**数据单位**，从基线往下量 —— 所以它随 y 轴缩放，图一高，带子也跟着变厚。

## 基线与负值

杆从 `baseline`（默认 `0`）出发。比它小的值朝下长，标签也放到点下面 —— log2 fold change 这类图要的正是这个：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Log2 fold change',
  'x_axis': {'name': 'gene index', 'tick_format': 'integer'},
  'y_axis': {'name': 'log2 FC'},
  'series': [{
    'type': 'lollipop',
    'points': [
      {'x': 1, 'y': 2.3}, {'x': 2, 'y': -1.8}, {'x': 3, 'y': 0.5},
      {'x': 4, 'y': -3.1}, {'x': 5, 'y': 1.9}, {'x': 6, 'y': -0.7},
      {'x': 7, 'y': 4.2}
    ],
    'color': 'steelblue',
    'baseline': 0,
    'baseline_dash': '4 3'
  }]
})) AS chart;
```

`show_baseline`、`baseline_color`、`baseline_width` 控制基线本身；`dot_stroke` 与 `dot_stroke_width`
给端点描边。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每根杆一项：`{x, y, label?, color?}`。 |
| `domains` | domain[] | 背景带：`{start, end, label?, color, opacity?}`。 |
| `baseline` | number | 杆的起点（默认 `0`）。 |
| `stem_width` | number | 杆的线宽（默认 `1.5`）。 |
| `dot_radius` | number | 端点的半径（默认 `5`）。 |
| `dot_stroke` / `dot_stroke_width` | string / number | 端点的描边颜色与宽度。 |
| `show_baseline` | boolean | 画那条横向基线（默认开）。 |
| `baseline_color` / `baseline_width` | string / number | 它的颜色与宽度。 |
| `baseline_dash` | string | 它的虚线样式（如 `"4 3"`）。 |
| `domain_height` | number | 区间带的高度（数据单位，基线以下，默认 `0.5`）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。点自己的 `color` 盖过 series 的颜色；
`tooltips` 这个字段接受，但 `lollipop` 没有实现。

## 说明

- **`points` 不能为空。**
- `x` 是数值 —— 想要分类轴，就把行排个名次、用名次当 x，分类名作为点的 `label` 带上。
- `domain_height` 的单位是数据单位、不是像素，所以区间带会随 y 轴一起缩放。
- 低于基线的点，杆朝下画、标签放到点下方。

## 另见

- [kuva — 棒棒糖图](https://psy-fer.github.io/kuva/plots/lollipop.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) —— 经典的实心版本。
- [坡度图](./slope.md) —— 前后对比。
- [帕累托图](./pareto.md) —— 排序柱加累计线。
