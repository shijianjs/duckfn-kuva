---
title: 联合分布图
sidebar_position: 5
description: 散点加上顶部与右侧的边缘分布。
---

# 联合分布图

联合分布图就是[散点图](./scatter.md)配上两个轴的边缘分布 —— 顶部一条 `x` 的直方图或密度、右侧一条 `y` 的。
一张图里同时看关系与两个一维形状。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'jointplot',
    'groups': grps,
    'marginal_type': 'histogram',
    'bins': 15,
    'show_top': true, 'show_right': true,
    'marginal_size': 90,
    'marker_size': 4,
    'marker_opacity': 0.6
  }]
})) AS chart
FROM (
  SELECT list({'x': xs, 'y': ys, 'label': g, 'trend': true} ORDER BY g) AS grps
  FROM (
    SELECT "group" AS g, list(x ORDER BY x) AS xs, list(y ORDER BY x) AS ys
    FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
    GROUP BY "group"
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每组一层散点（见下）。 |
| `marginal_type` | string | `"histogram"`（默认）或 `"density"`。 |
| `show_top` | boolean | 画顶部边缘面板。 |
| `show_right` | boolean | 画右侧边缘面板。 |
| `marginal_size` | number | 边缘面板的厚度（像素）。 |
| `marginal_gap` | number | 边缘面板与主图之间的缝（像素）。 |
| `bins` | integer | 直方图的箱数（至少 1）。 |
| `bandwidth` | number | 核密度带宽（`marginal_type` 为 `"density"` 时）。 |
| `marginal_alpha` | number | 边缘面板的填充不透明度。 |
| `x_label` / `y_label` | string | 主图的坐标轴标题。 |
| `marker_size` | number | 各组共用的点半径。 |
| `marker_opacity` | number | 各组共用的点不透明度。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每个点一条提示。 |

`groups` 的每一项带 `x`、`y`（都必填且等长），以及 `label`、`color`、`marker`、`sizes`、`colors`、
`trend`、`equation`、`correlation`。

## 说明

- **每组的 `x` 与 `y` 必须等长**，组不能为空；`bins` 至少为 1（0 会在归一化时除零）。
- 组上的 `sizes` / `colors` 必须与该组的点数一致。

## 另见

- [kuva — 联合分布图](https://psy-fer.github.io/kuva/plots/jointplot.html) —— 绘图库自己的图型参考。
- [散点图](./scatter.md) —— 单独的散点。
- [二维直方图](../distributions/histogram2d.md) —— 同一团点云的分箱视图。
