---
title: 联合分布图
sidebar_position: 5
description: 散点，加上顶部与右侧的边缘分布。
---

# 联合分布图

联合分布图把一张散点和它在顶部、右侧的边缘分布面板拼在一起。两个面板各画对应轴的一维分布 —— 直方柱或者核
密度曲线。这样，两个变量的关系与各自的分布，一张图里就都有了。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Joint plot',
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

两个面板默认都是直方图、20 个箱。联合分布图的轴标题来自它自己的 `x_label` / `y_label` —— 图级的
`x_axis` / `y_axis` 到不了这张图的坐标轴上。

## 边缘面板的形态

`marginal_type` 在直方柱与填色密度曲线之间切换。

| `marginal_type` | 面板 |
| --- | --- |
| `"histogram"` | 直方柱（**默认**） |
| `"density"` | 填色的核密度曲线 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Density marginals',
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'marginal_type': 'density',
    'bandwidth': 0.4,
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

核密度的带宽默认用 Silverman 经验法则；`bandwidth` 用一个固定值覆盖它。

## 显示 / 隐藏边缘面板

两个面板各自独立开关。都关掉就剩一张纯散点 —— 同一批数据两种画法都能出，调参时可以直接换。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'show_right': false,
    'bins': 25,
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## 多分组

给每个分组一个 `label` 和一个 `color`。只要有两个及以上分组带了标签，图例就出现在边缘面板右侧。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Multiple groups',
  'series': [{
    'type': 'jointplot',
    'groups': list({'x': xs, 'y': ys, 'label': g} ORDER BY g),
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT "group" AS g,
         list(x ORDER BY x, y) AS xs,
         list(y ORDER BY x, y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

每个分组的边缘直方柱 / 密度填充用该组的颜色、降一点透明度画，透明度由 `marginal_alpha` 控制。

## 趋势线

分组直接带散点的趋势字段：`trend` 画最小二乘线，`equation` 与 `correlation` 标出拟合统计量。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Trend lines',
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys, 'color': '#e15759',
                'trend': true, 'correlation': true}],
    'x_label': 'measurement',
    'y_label': 'response'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## 误差棒

分组可以带逐点的 `x_err` / `y_err` —— 给**一个数**是对称的，给 **`[负臂, 正臂]` 对**是不对称的，写法和
[散点图](./scatter.md)的 series 完全一样。每条列表都必须与这一组的 `x` / `y` 等长。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    (1.0,  2.1, 0.15, 0.30),
    (2.0,  3.9, 0.15, 0.40),
    (3.0,  6.2, 0.20, 0.35),
    (4.0,  7.8, 0.20, 0.40),
    (5.0, 10.1, 0.25, 0.50),
    (6.0, 12.3, 0.25, 0.45),
    (7.0, 13.9, 0.30, 0.50),
    (8.0, 16.2, 0.30, 0.55)
  ) AS t(x, y, x_err, y_err)
)
SELECT kuva_render(to_json({
  'title': 'Error bars',
  'series': [{
    'type': 'jointplot',
    'groups': [{
      'x': (SELECT list(x ORDER BY x) FROM d),
      'y': (SELECT list(y ORDER BY x) FROM d),
      'color': '#76b7b2',
      'x_err': (SELECT list(x_err ORDER BY x) FROM d),
      'y_err': (SELECT list(y_err ORDER BY x) FROM d)
    }],
    'x_label': 'measurement',
    'y_label': 'response'
  }]
})) AS chart;
```

## marker 形状与大小

`marker` 是逐分组的。`marker_size`、`marker_opacity`、`marker_stroke_width` 都可以逐组给；前两个还有顶层
默认值，会套用到没有自己覆盖的分组上。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys, 'color': '#59a14f', 'marker': 'square'}],
    'marker_size': 5,
    'marker_opacity': 0.7,
    'marginal_type': 'density',
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## 逐点颜色

分组上的 `colors` 逐个给点上色。边缘面板仍然用分组的统一 `color`，想让它跟着变就顺手也给一个。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys, 'colors': cs}],
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys,
         list(CASE WHEN y > 5.5 THEN '#4e79a7'
                   WHEN y > 4.5 THEN '#59a14f'
                   ELSE '#e15759' END ORDER BY x, y) AS cs
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## 悬停提示

`tooltips` 会在 SVG 输出里注入悬停用的 `<title>`；`tooltip_labels` 给每个点一段文字（每个点一条字符串，
顺序与该组的 x / y 一致）。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'tooltips': true,
    'tooltip_labels': tips,
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys,
         list('(' || round(x, 2) || ', ' || round(y, 2) || ')' ORDER BY x, y) AS tips
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## 面板尺寸

| 字段 | 作用 |
| --- | --- |
| `marginal_size` | 面板厚度（像素，默认 `80`） |
| `marginal_gap` | 面板与主图之间的缝（默认 `4`） |
| `bins` | 直方图的箱数（默认 `20`） |
| `marginal_alpha` | 直方柱 / 填充的透明度（默认 `0.6`） |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'marginal_size': 120,
    'marginal_gap': 8,
    'bins': 30,
    'marginal_alpha': 0.5,
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每个分组一层散点（见下）。 |
| `marginal_type` | string | `"histogram"`（默认）或 `"density"`。 |
| `show_top` | boolean | 画顶部边缘面板（默认开）。 |
| `show_right` | boolean | 画右侧边缘面板（默认开）。 |
| `marginal_size` | number | 边缘面板的厚度（像素）。 |
| `marginal_gap` | number | 边缘面板与主图之间的缝（像素）。 |
| `bins` | integer | 直方图的箱数（至少 1）。 |
| `bandwidth` | number | 核密度带宽（`marginal_type` 为 `"density"` 时）。 |
| `marginal_alpha` | number | 边缘面板的填充不透明度。 |
| `x_label` / `y_label` | string | 主图的轴标题。 |
| `marker_size` | number | 共用的点半径。 |
| `marker_opacity` | number | 共用的点不透明度。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每个点一条提示文字。 |

`groups` 的每一项带 `x`、`y`（都必填且等长），外加：

| 分组字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `label` | string | 图例文字。 |
| `color` | string | marker 颜色。 |
| `marker` | string | marker 形状。 |
| `sizes` | number[] | 逐点半径（气泡图）。 |
| `colors` | string[] | 逐点颜色。 |
| `x_err` / `y_err` | (number \| `[number, number]`)[] | 逐点误差棒；一个数是对称的，`[负, 正]` 是不对称的。 |
| `marker_size` / `marker_opacity` / `marker_stroke_width` | number | 这一组的 marker 样式；覆盖顶层默认值。 |
| `trend` | boolean | 叠一条最小二乘线。 |
| `equation` / `correlation` | boolean | 标出拟合统计量。 |

## 说明

- **每个分组的 `x` 与 `y` 必须等长**，分组不能为空；`bins` 至少为 1（给 0 会在归一化时除零）。
- 分组上的 `sizes` / `colors` 必须与这个分组的点数一致。
- 给了 `x_err` / `y_err` 就要与这一组的点数一致；对不上是报错，而不是悄悄截断。
- 分组自己的 `marker_size` / `marker_opacity` 盖过顶层的同名字段；`marker_stroke_width` 没有顶层对应项。
- 图级的 `x_axis` / `y_axis` 标不了联合分布图，要用 `x_label` / `y_label`。

## 另见

- [kuva — 联合分布图](https://psy-fer.github.io/kuva/plots/jointplot.html) —— 绘图库自己的图型参考。
- [散点图](./scatter.md) —— 单独的一张散点。
- [二维直方图](../distributions/histogram2d.md) —— 同一团点云的分箱视角。
