---
title: 火山图
sidebar_position: 5
description: 效应量对显著性，并把上调 / 下调 / 不显著分开上色。
---

# 火山图

火山图把 **log₂ 倍数变化**放在 x 轴、把 **−log₁₀(p 值)** 放在 y 轴 —— 于是真正重要的点是那些既高又远的。
同时通过两个阈值的点按上调（右）或下调（左）上色，其余是灰的，阈值线也替你画好。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Tumour vs normal',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

分类全靠两个阈值：`fc_cutoff`（默认 `1`，也就是两倍变化）与 `p_cutoff`（默认 `0.05`）。虚线画在
`±fc_cutoff` 与 `−log10(p_cutoff)` 处。

## 基因标签

`label_top` 会给**最显著的** *n* 个点加标签 —— 这就是「少数几个值得点名的基因被点名，而不是给两万个点全加标签」的
做法。标签位置有三种：

| `label_style` | 放法 |
| --- | --- |
| `"nudge"` | 按 x 排序后竖直方向挪开以避免叠字（**默认**） |
| `"exact"` | 就放在点上、不做调整 —— 用于稀疏数据或打算后处理 SVG 的情形 |
| `{"offset_x": 14, "offset_y": 16}` | 按像素偏移，并画一条短引线拉回数据点 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Top hits labelled',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'label_top': 12,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

高显著那一角会挤的时候，就该用箭头那种：引线让标签可以待在舒服的距离上。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Arrow labels',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'label_top': 10,
    'label_style': {'offset_x': 14, 'offset_y': 16},
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano_genes.tsv')
);
```

避让时的竖直铺开量是从数据推出来的，所以同样的 `label_top`，在最显著的点聚得紧或散得开时，标签云也会更紧或更松。

## 阈值

更严的阈值把线往 x 轴内侧、y 轴上方挪，把彩色的两个角缩小。同一组对比画出的两张火山图对不上时，第一个该查的就是它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Stricter cutoffs',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'fc_cutoff': 2,
    'p_cutoff': 0.01,
    'color_up': 'darkorange',
    'color_down': 'mediumpurple',
    'label_top': 8,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

## 颜色

| 字段 | 默认 | 用在 |
| --- | --- | --- |
| `color_up` | `firebrick` | `log2fc ≥ fc_cutoff` 且 `p ≤ p_cutoff` |
| `color_down` | `steelblue` | `log2fc ≤ −fc_cutoff` 且 `p ≤ p_cutoff` |
| `color_ns` | `#aaaaaa` | 其余全部 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'House colours',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'color_up': '#d62728',
    'color_down': '#1f77b4',
    'color_ns': '#cccccc',
    'point_size': 3.5,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

## p 值为 0 与极端 p 值

p 值正好为 `0` 时没法取对数。这类点会按**数据里**最小的非零 p 值夹住 —— 也就是说 y 轴顶端落在「最显著的那个基因
恰好是多少」上，两份不同数据集的火山图于是落在不同的刻度上。

`pvalue_floor` 把这个上限显式定下来，几张图就能共用同一根 y 轴：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pinned y ceiling',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'pvalue_floor': 1e-10,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每项是 `{name, log2fc, pvalue}`。 |
| `fc_cutoff` | number | `\|log2FC\|` 阈值（默认 `1`）。 |
| `p_cutoff` | number | p 值阈值（默认 `0.05`）。 |
| `color_up` / `color_down` / `color_ns` | string | 三种分类的颜色。 |
| `point_size` | number | 圆点半径（像素，默认 `3`）。 |
| `label_top` | integer | 给最显著的 *n* 个点加标签（`0` = 不加，默认）。 |
| `label_style` | string \| object | `"nudge"`（默认）· `"exact"` · `{offset_x, offset_y}`。 |
| `pvalue_floor` | number | `−log10` 变换时 p 值的显式下限。 |
| `legend` | string | 任一非空值就打开 上调 / 下调 / 不显著 的图例。 |

## 说明

- **`points` 不能为空**，且每个点要有 `name`、`log2fc`、`pvalue`。
- `pvalue` 是**原始** p 值，不是 `−log10` 之后的结果。
- p 值大于 `1`、或小于等于 `0` 都没有意义；后者由下限处理，前者会被照画成负的 y，而不是被悄悄丢掉。
- `label_top` 是按**显著性**挑标签，不是按倍数变化 —— 标签给的是最高的那些点，这通常是你想要的，但不总是。
- `fc_cutoff` 是与**绝对值**比较的，所以 x 上那两条虚线是对称的。

## 另见

- [kuva — 火山图](https://psy-fer.github.io/kuva/plots/volcano.html) —— 绘图库自己的图型参考。
- [曼哈顿图](./manhattan.md) —— 用基因组位置而不是效应量表达显著性。
- [Q-Q 图](../distributions/qq.md) —— 相信峰之前，先查 p 值分布本身。
