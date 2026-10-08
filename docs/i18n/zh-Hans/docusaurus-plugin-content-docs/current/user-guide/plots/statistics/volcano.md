---
title: 火山图
sidebar_position: 5
description: 倍数变化对显著性，上调 / 下调 / 不显著分色，并可标注最显著的点。
---

# 火山图

火山图把 log2 倍数变化放 x、−log10(p) 放 y，所以「变化大且显著」的项落在上方两个角里。点按是否同时越过
两个阈值来上色。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p)'},
  'series': [{
    'type': 'volcano',
    'points': list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}),
    'fc_cutoff': 1,
    'p_cutoff': 0.05,
    'label_top': 10
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每一项：`{name, log2fc, pvalue}`。 |
| `fc_cutoff` | number | 倍数变化阈值（取绝对值）。 |
| `p_cutoff` | number | p 值阈值。 |
| `color_up` / `color_down` / `color_ns` | string | 上调、下调、不显著三类点的颜色。 |
| `point_size` | number | 点半径。 |
| `label_top` | integer | 标出最显著的 N 个点（`0` = 全不标）。 |
| `label_style` | string \| object | `"nudge"`（默认）· `"exact"` · `{"offset_x":…, "offset_y":…}`。 |
| `pvalue_floor` | number | p 值的下限（避免 `log10(0)`）；缺省取数据里最小的非零 p 值。 |

`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)。没有统一的
`color` —— 三类点各有颜色。

## 说明

- **`points` 不能为空。** 给**原始** `pvalue`，不是 `-log10(p)`。
- `fc_cutoff` 作用在倍数变化的绝对值上，所以上调与下调的极端都会被它上色。

## 另见

- [kuva — 火山图](https://psy-fer.github.io/kuva/plots/volcano.html) —— 绘图库自己的图型参考。
- [曼哈顿图](./manhattan.md) —— 全基因组层面的对应图。
- [Q-Q 图](../distributions/qq.md) —— 检查 p 值分布本身。
