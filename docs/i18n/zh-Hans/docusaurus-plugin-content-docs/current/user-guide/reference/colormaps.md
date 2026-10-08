---
title: 色图
sidebar_position: 7
description: 给「用数值编码颜色」的图型用的连续色标，以及全部名字。
---

# 色图

把数值编码成颜色的图型 —— `heatmap`、`histogram2d`、`hexbin`、`clustermap`、`contour`、`dice_plot`、
`calendar` 等 —— 接受 `color_map`。它把数值**连续**地映射成颜色，不同于[调色板](./palettes.md)（后者
把离散的颜色发给各 series）。

`color_map` 是**每个 series 各自**的字段，不是顶层字段：

```json
{ "type": "heatmap", "data": [[1, 2], [3, 4]], "color_map": "viridis" }
```

## 名字

取值是下列名字的 snake_case 写法：

| 类型 | 名字 |
| --- | --- |
| 顺序型，感知均匀 | `turbo` · `viridis` · `inferno` · `magma` · `plasma` · `cividis` · `warm` · `cool` · `cubehelix` |
| 顺序型（ColorBrewer） | `blue_green` · `blue_purple` · `green_blue` · `orange_red` · `purple_blue_green` · `purple_blue` · `purple_red` · `red_purple` · `yellow_green_blue` · `yellow_green` · `yellow_orange_brown` · `yellow_orange_red` |
| 顺序型（单色相） | `blues` · `greens` · `grayscale` · `oranges` · `purples` · `reds` |
| 双向型（两端） | `brown_green` · `pink_green` · `purple_green` · `purple_orange` · `red_blue` · `red_grey` · `red_yellow_blue` · `red_yellow_green` · `spectral` |
| 周期型 | `rainbow` · `sinebow` |

数据有明确中点（fold change、相关性）时用**双向型**，否则用**顺序型**。**周期型**适合角度、相位与一天中
的时刻。

## 示例

一张 z-score 表达矩阵，用 `viridis` 画。矩阵用 `list_value` 逐行拼出来，基因名与样本名成为坐标轴标签：

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

## 另见

- [调色板](./palettes.md) —— 给 series 的离散颜色。
