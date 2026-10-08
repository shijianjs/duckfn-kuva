---
title: 散点图
sidebar_position: 1
description: 一个个 (x, y) 点，支持趋势线、误差棒、气泡大小与逐点颜色。
---

# 散点图

散点图画出一个个 `(x, y)` 点。它支持线性趋势线、误差棒、可变点半径（气泡图）、逐点颜色，以及六种 marker
形状。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | **必填。** 点，`[x, y]` 对或 `{x, y, x_err?, y_err?}` 对象。 |
| `size` | number | 统一的点半径（默认 3）。 |
| `sizes` | number[] | 逐点半径（气泡图）；会覆盖 `size`。 |
| `colors` | string[] | 逐点颜色；超出部分回退到 `color`。 |
| `marker` | string | `"circle"`（默认）· `"square"` · `"triangle"` · `"diamond"` · `"cross"` · `"plus"`。 |
| `marker_opacity` | number | 填充透明度：`0` 空心，`1` 实心。 |
| `marker_stroke_width` | number | 轮廓线宽，用填充色画。 |
| `trend` | `"linear"` \| object | 叠加一条最小二乘直线；对象形式还能给 `color`、`width`、`equation`、`correlation`。 |
| `band` | `{lower, upper}` | 与点的 x 位置对齐的阴影带。 |
| `group_name` | string | 交互式 SVG 的分组名（不进图例）。 |

`color`、`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)；
`x_err` / `y_err` 见[点的写法](../../reference/series.md#点)。

## 说明

- **`data` 不能为空。** `sizes` 或 `colors` 比数据长没关系；短了会回退到 `size` / `color`。
- 逐点颜色**不会**更新图例 —— 想要带标签的图例，就按组拆成多个 series。
- `x_err` / `y_err` 给一个数是（对称）误差棒，给 `[下, 上]` 对是不对称的。

## 另见

- [kuva — 散点图](https://psy-fer.github.io/kuva/plots/scatter.html) —— 绘图库自己的图型参考。
- [折线图](./line.md) —— 有序、需要连线时用。
- [二维直方图](../distributions/histogram2d.md) 与 [六边形分箱图](../distributions/hexbin.md) —— 点非常多时用。
