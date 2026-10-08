---
title: 聚类热图
sidebar_position: 7
description: 带行列层次聚类树的heatmap，外加注释色条。
---

# 聚类热图

聚类热图是按层次聚类重排行列的[热力图](../distributions/heatmap.md)，并把聚类树画在旁边。注释色条可以标注
行或列的分组。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'clustermap',
    'data': matrix,
    'row_labels': genes,
    'col_labels': ['Sample_01','Sample_02','Sample_03','Sample_04','Sample_05','Sample_06',
                   'Sample_07','Sample_08','Sample_09','Sample_10','Sample_11','Sample_12'],
    'cluster_rows': true,
    'cluster_cols': true,
    'color_map': 'blue_green',
    'normalization': 'row_zscore',
    'branch_color': '#555555',
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
| `data` | number[][] | **必填。** 行优先矩阵；每行等长。 |
| `row_labels` / `col_labels` | string[] | 标签；长度必须与行数 / 列数一致。 |
| `cluster_rows` / `cluster_cols` | boolean | 在该方向做聚类（默认都开）。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `show_values` | boolean | 在每格里写数值。 |
| `normalization` | string | `"none"`（默认）· `"row_zscore"` · `"col_zscore"`。 |
| `branch_color` | string | 聚类树的颜色。 |
| `row_dendrogram_width` | number | 行聚类树的宽度。 |
| `col_dendrogram_height` | number | 列聚类树的高度。 |
| `row_annotations` / `col_annotations` | track[] | 紧贴矩阵的注释色条，每项 `{colors, label?, width?}`。 |
| `legend` | string | 色条的标题。 |
| `tooltips` | boolean | 悬停提示。 |

## 说明

- **矩阵必须矩形且非空**；`row_labels` / `col_labels` 要与其形状一致，每条注释色条的 `colors` 也必须与它
  并排的行 / 列数一致 —— 对不上会报错。
- 各行量纲不同时，优先用 `normalization: "row_zscore"`。

## 另见

- [kuva — 聚类热图](https://psy-fer.github.io/kuva/plots/clustermap.html) —— 绘图库自己的图型参考。
- [热力图](../distributions/heatmap.md) —— 不做聚类。
- [色图](../../reference/colormaps.md) —— 连续色标。
