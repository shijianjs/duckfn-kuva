---
title: 骰子图
sidebar_position: 10
description: 类目 × 类目的网格，每格最多画六个点，用点数编码计数。
---

# 骰子图

骰子图在两个轴上都放类目，每格最多画 `ndots` 个点，像骰子的面。点的个数编码计数、填充色编码第三个值、点
大小编码第四个值 —— 在网格上同时展示集合成员关系或小计数时非常紧凑。

```sql {"type":"duckfn","show":"svg"}
WITH pats AS (
  SELECT (GWAS_hit::VARCHAR || eQTL::VARCHAR || Splicing_QTL::VARCHAR
          || Methylation_QTL::VARCHAR || Conservation::VARCHAR || ClinVar::VARCHAR) AS pat
  FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')
),
agg AS (
  SELECT pat, count(*) AS n FROM pats GROUP BY pat HAVING pat <> '000000' ORDER BY n DESC LIMIT 6
)
SELECT kuva_render(to_json({
  'series': [{
    'type': 'dice_plot',
    'ndots': 6,
    'x_categories': ['pattern'],
    'y_categories': list(pat ORDER BY n DESC),
    'points': list({
      'x': 'pattern', 'y': pat, 'fill': n,
      'present': list_filter(range(6), p -> substr(pat, p::INT + 1, 1) = '1')
    } ORDER BY n DESC),
    'fill_legend_label': 'variants'
  }]
})) AS chart
FROM agg;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每格一项：`{x, y, present, fill?, size?}`。 |
| `ndots` | integer | 每格几个点，1–6（默认 4）。 |
| `x_categories` | string[] | **必填。** 列类目（x 轴）。 |
| `y_categories` | string[] | **必填。** 行类目（y 轴）。 |
| `category_labels` | string[] | 每个点一个标签；长度必须等于 `ndots`。 |
| `color_map` | string | 填充值用的[色图](../../reference/colormaps.md)。 |
| `fill_range` | `[number, number]` | 色图跨越的填充值区间。 |
| `size_range` | `[number, number]` | 大小编码值的区间。 |
| `fill_legend_label` | string | 填充图例的标题。 |
| `size_legend_label` | string | 尺寸图例的标题。 |
| `position_legend_label` | string | 位置（点数）图例的标题。 |
| `dot_legend` | `[string, string][]` | 每个点的图例条目，长度 `ndots`。 |
| `grid_lines` | boolean | 画格子的分隔线。 |
| `dot_radius` | number | 点半径（`0` = 按格自动）。 |
| `cell_width` / `cell_height` | number | 格子占槽位的比例。 |
| `pad` | number | 格子之间的留白。 |

每格的 `present` 列出显示哪几个点，是**从 0 开始**的下标，且必须小于 `ndots`。

## 说明

- **`points` 不能为空**，`ndots` 必须在 1–6，且每个 `present` 下标都要小于 `ndots`。
- `category_labels` 与 `dot_legend` 给了的话，都必须正好 `ndots` 项。

## 另见

- [kuva — 骰子图](https://psy-fer.github.io/kuva/plots/diceplot.html) —— 绘图库自己的图型参考。
- [点图](./dotplot.md) —— 每格一个点，配连续编码。
- [UpSet 图](./upset.md) —— 集合交集的矩阵视图。
