---
title: 热力图
sidebar_position: 12
description: 把矩阵里的数值画成一格格彩色单元，带行列标签与色图。
---

# 热力图

热力图把一个数值矩阵画成一格格按连续[色图](../../reference/colormaps.md)上色的格子。表达矩阵、相关性
表、任何已经表列好的网格，都用它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'heatmap',
    'data': matrix,
    'row_labels': genes,
    'col_labels': ['Sample_01','Sample_02','Sample_03','Sample_04','Sample_05','Sample_06',
                   'Sample_07','Sample_08','Sample_09','Sample_10','Sample_11','Sample_12'],
    'color_map': 'viridis',
    'legend': 'z-score'
  }]
})) AS chart
FROM (
  SELECT
    array_agg(list_value(Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                         Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12)
              ORDER BY gene) AS matrix,
    array_agg(gene ORDER BY gene) AS genes
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | number[][] | **必填。** 行优先的矩阵（`data[row][col]`）。每行必须等长。 |
| `row_labels` | string[] | 行标签（y 轴，从下往上）；长度必须等于行数。 |
| `col_labels` | string[] | 列标签（x 轴，从左往右）；长度必须等于列数。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `show_values` | boolean | 在每格里写出数值。 |
| `legend` | string | 色条的标题。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每格一条提示，按行优先顺序（`rows * cols` 条）。 |
| `x_range` / `y_range` | `[number, number]` | 只画矩阵的这一段（默认覆盖全部列 / 行）。 |
| `cell_size` | number | 格子占槽位的比例，内部夹到 `[0.5, 1]`。 |

:::note[没有 `color`]

热力图用色图编码数值，所以它没有 `color` 字段 —— 用 `color_map`。

:::

## 说明

- **每行必须等长。** 长度不齐会作为错误报出来，而不是静默丢列。
- **`row_labels` / `col_labels` 的长度必须与矩阵维度一致**；对不上会报错。
- 构造矩阵的办法：每行用 `list_value(…)` 聚出，再用 `array_agg(…)` 把行聚起来。

## 另见

- [kuva — 热力图](https://psy-fer.github.io/kuva/plots/heatmap.html) —— 绘图库自己的图型参考。
- [色图](../../reference/colormaps.md) —— 这个图型用的连续色标。
- [二维直方图](./histogram2d.md) 与 [六边形分箱图](./hexbin.md) —— 把点云分到格子里。
