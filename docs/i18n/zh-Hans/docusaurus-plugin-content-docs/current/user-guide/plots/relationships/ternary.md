---
title: 三元图
sidebar_position: 9
description: 三个分量之和为 1 的点，画在一个三角形里。
---

# 三元图

三元图（也叫单纯形图、de Finetti 图）画的是成分数据：每个点是三个分量、其和为一个常数，通常是 `1` 或
`100 %`。

画布是一个等边三角形。每个顶点是某个分量的 100 %，其对边是 0 %，所以一个点到每条边的距离**就是**它的分量
占比。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Ternary plot',
  'series': [{
    'type': 'ternary',
    'points': pts,
    'corner_labels': ['A', 'B', 'C'],
    'normalize': true,
    'marker_size': 6,
    'grid_lines': 4,
    'show_grid': true,
    'show_percentages': true,
    'show_legend': true,
    'marker_opacity': 0.8
  }]
})) AS chart
FROM (
  SELECT list({'a': a, 'b': b, 'c': c, 'group': "group"}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/ternary.tsv')
);
```

## 加点

每个点是一个 `{a, b, c}` 对象：`a` 是上顶点，`b` 是左下，`c` 是右下。再加一个 `group`，这个点就有颜色、
也有图例条目了：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Soil texture',
  'series': [{
    'type': 'ternary',
    'corner_labels': ['Clay', 'Silt', 'Sand'],
    'points': [
      {'a': 0.70, 'b': 0.20, 'c': 0.10, 'group': 'Clay loam'},
      {'a': 0.10, 'b': 0.70, 'c': 0.20, 'group': 'Silt loam'},
      {'a': 0.20, 'b': 0.10, 'c': 0.70, 'group': 'Sandy loam'}
    ],
    'grid_lines': 5,
    'show_legend': true
  }]
})) AS chart;
```

不给 `group` 的点用回退色画，也不进图例。

## 归一化

分量和不等于 1 的 —— 加起来是 100 的百分比，或者原始计数 —— 需要 `"normalize": true`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'ternary',
    'corner_labels': ['A', 'B', 'C'],
    'points': [
      {'a': 60, 'b': 25, 'c': 15},
      {'a': 30, 'b': 50, 'c': 20},
      {'a': 20, 'b': 30, 'c': 50}
    ],
    'normalize': true,
    'grid_lines': 5
  }]
})) AS chart;
```

`"normalize": false`（默认）时，三个分量必须已经之和为 1。

## marker 不透明度与描边

这个密度下点会叠成一坨不透明的色块；`marker_opacity` 配上 `marker_stroke_width`，边界区域才看得清、每个
样本也还数得过来。描边的颜色就是填充色。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Semi-transparent markers',
  'series': [{
    'type': 'ternary',
    'points': pts,
    'corner_labels': ['A', 'B', 'C'],
    'normalize': true,
    'marker_size': 5,
    'marker_opacity': 0.3,
    'marker_stroke_width': 0.8,
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'a': a, 'b': b, 'c': c, 'group': "group"}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/ternary.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每个点一项：`{a, b, c, group?}`。 |
| `corner_labels` | string[] | 三个角的标签，顺序是**上、左下、右下**。 |
| `normalize` | boolean | 把每个点归一到分量之和为 1。 |
| `marker_size` | number | 点半径。 |
| `grid_lines` | integer | 网格分几份。 |
| `show_grid` | boolean | 画网格。 |
| `show_percentages` | boolean | 给网格线标百分比。 |
| `show_legend` | boolean | 显示图例（文字取自各点的 `group`）。 |
| `marker_opacity` | number | 点的透明度。 |
| `marker_stroke_width` | number | 点的描边宽度。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每个点一条提示文字。 |

## 说明

- **`points` 不能为空。** `normalize: false` 时三个分量必须已经之和为 1。
- `corner_labels` 必须**正好 3 项**；数目不对是报错。
- 图例由各点的 `group` 决定，并且需要 `show_legend`。

## 另见

- [kuva — 三元图](https://psy-fer.github.io/kuva/plots/ternary.html) —— 绘图库自己的图型参考。
- [极坐标图](./polar.md) · [雷达图](../categorical/radar.md) —— 其它非笛卡尔布局。
