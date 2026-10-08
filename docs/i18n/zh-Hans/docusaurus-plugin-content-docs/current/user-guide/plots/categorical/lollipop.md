---
title: 棒棒糖图
sidebar_position: 7
description: 每个值一根杆加一个点，可选背景区间与基线。
---

# 棒棒糖图

棒棒糖图每个条目从基线画一根杆上去、末端一个点 —— 比柱更轻，配上每个点的标签很好读。背景的 `domains` 可以
标出「正常范围」。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'lollipop',
    'points': pts,
    'baseline': 0,
    'dot_radius': 5,
    'stem_width': 1.5,
    'legend': 'genes'
  }]
})) AS chart
FROM (
  SELECT list({'x': rn, 'y': expression, 'label': gene} ORDER BY rn) AS pts
  FROM (
    SELECT gene, expression, row_number() OVER (ORDER BY expression DESC) AS rn
    FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每个条目一项：`{x, y, label?, color?}`。 |
| `domains` | domain[] | 背景区间带：`{start, end, label?, color, opacity?}`。 |
| `baseline` | number | 杆的起点。 |
| `stem_width` | number | 杆的线宽。 |
| `dot_radius` | number | 端点半径。 |
| `dot_stroke` | string | 端点描边颜色。 |
| `dot_stroke_width` | number | 端点描边线宽。 |
| `show_baseline` | boolean | 画基线。 |
| `baseline_color` | string | 基线颜色。 |
| `baseline_width` | number | 基线宽度。 |
| `baseline_dash` | string | 基线虚线样式（如 `"4 2"`）。 |
| `domain_height` | number | 背景区间带的高度。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)；点自己的 `color` 会覆盖它。
`tooltips` 能解析但未实现。

## 说明

- **`points` 不能为空。**
- `x` 是数值；想让每个条带有类目名，就把名字放进点的 `label`、`x` 用下标（如示例）。

## 另见

- [kuva — 棒棒糖图](https://psy-fer.github.io/kuva/plots/lollipop.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) —— 更重的柱形。
- [点图](./dotplot.md) —— 两个类目轴的网格，而不是一根轴。
