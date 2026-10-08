---
title: 曼哈顿图
sidebar_position: 6
description: 按染色体排布的全基因组关联结果。
---

# 曼哈顿图

曼哈顿图把关联结果沿基因组逐染色体铺开，y 轴是 −log10(p)。越过显著性线的峰就是命中。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'chromosome'},
  'y_axis': {'name': '-log10(p)'},
  'series': [{
    'type': 'manhattan',
    'points': list({'chromosome': chr, 'position': pos, 'pvalue': pvalue}),
    'build': 'hg38',
    'genome_wide': 7.3,
    'suggestive': 5,
    'point_size': 3
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每个位点一项：`{chromosome, position?, pvalue, label?}`。 |
| `genome_wide` | number | 全基因组显著性线（`-log10(p)`）。 |
| `suggestive` | number | 提示性显著线。 |
| `color_a` / `color_b` | string | 奇偶染色体的颜色（缺省用 `color_a` 的浅一档）。 |
| `build` | string | `"hg19"` · `"hg38"` · `"t2t"` —— 用该版本的染色体长度算累积坐标。 |
| `point_size` | number | 点半径。 |
| `label_top` | integer | 标出最显著的 N 个点（`0` = 全不标）。 |
| `label_style` | string \| object | `"nudge"`（默认）· `"exact"` · `{"offset_x":…, "offset_y":…}`。 |
| `pvalue_floor` | number | p 值的下限（避免 `log10(0)`）。 |

## 说明

- **`points` 不能为空**，p 值不能为负，且 **`build` 要求每个点都带 `position`**（bp）—— 否则会报错。
- x 坐标取决于每个点带了什么：只给染色体 + p 值就用染色体序号；带 `build` 就用累积碱基坐标；不带 `build`
  但有 `position` 就直接用 `position`。
- 点自己的 `label` 优先；`label_top` 补上那些没标签的最显著点。

## 另见

- [kuva — 曼哈顿图](https://psy-fer.github.io/kuva/plots/manhattan.html) —— 绘图库自己的图型参考。
- [火山图](./volcano.md) —— 逐基因的倍数变化，而不是基因组位置。
