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

取值是下列名字之一。匹配时忽略大小写与分隔符，并接受 ColorBrewer 的常见缩写 ——
`yellow_green_blue`、`yellow-green-blue` 与 `ylgnbu` 指的是同一个。认不出的名字会报错并指出这个名字，
而不是悄悄退回默认值。

| 类型 | 名字 |
| --- | --- |
| 顺序型，感知均匀 | `turbo` · `viridis` · `inferno` · `magma` · `plasma` · `cividis` · `warm` · `cool` · `cubehelix` |
| 顺序型（ColorBrewer） | `blue_green` · `blue_purple` · `green_blue` · `orange_red` · `purple_blue_green` · `purple_blue` · `purple_red` · `red_purple` · `yellow_green_blue` · `yellow_green` · `yellow_orange_brown` · `yellow_orange_red` |
| 顺序型（单色相） | `blues` · `greens` · `grayscale` · `oranges` · `purples` · `reds` |
| 双向型（两端） | `brown_green` · `pink_green` · `purple_green` · `purple_orange` · `red_blue` · `red_grey` · `red_yellow_blue` · `red_yellow_green` · `spectral` |
| 周期型 | `rainbow` · `sinebow` |

一共 38 条渐变，全部由同一份 `ColorMap` 实现画出来。

**怎么选**：`viridis` 是大多数图型的默认值 —— 感知均匀、对色觉障碍友好，所以适合当通用选择。图形要经得起
黑白打印时用 `grayscale`（或者直接打开[黑白模式](./bw-mode.md)，它会强制换成灰阶）。数据有明确中点
（log fold change、相关系数）时用**双向型**，两个方向才会读起来是两件事；其余情况用**顺序型**。
`rainbow` 与 `sinebow` 只属于真正周期的数据（角度、一年中的第几天、相位）：周期型色图会绕回起始色相，
在并不会绕回的量上读起来像是虚假的断点。

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
