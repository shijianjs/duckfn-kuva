---
title: 组合图（覆盖度与 brick-pop）
sidebar_position: 19
description: 两种不是 series 的图 —— 基因组浏览器式的覆盖度图与群体等位基因频率图，都由顶层的 composite 对象驱动。
---

# 组合图（覆盖度与 brick-pop）

kuva 里有两张图不是 `Plot` 变体，而是**组合图**：它们自己拼画布、自己决定轨道高度与共享轴，不走通用的
`Layout` / `Figure` 排版。

| 组合图 | 画什么 |
| --- | --- |
| `"coverage"` | 基因组浏览器式的图：每个样本一条深度轨道，共用一条基因组 x 轴；可选的变异位点轨道，以及轴上方的 feature / 基因带。 |
| `"brick_pop"` | 砖墙图 + 每行旁边的等位基因频率条 + 可选的指标热格列。 |

它们用顶层的 `composite` 对象、由 `type` 分派：

```json
{ "composite": { "type": "coverage", … } }
```

`composite` 与 `series`、`figure` **互斥** —— 组合图就是整张图，所以和 series 列表一起给会报错，而不是
静默丢掉其中一边。它自带 `width` / `height`（宽度默认 `900`，高度按轨道数 / 行数自动算）。

## 覆盖度图

覆盖度图把每个样本画成一条填充的深度轨道，全部共用一条**基因组** x 轴，于是覆盖度、变异、扩增子与基因
逐列对齐。feature 与基因在轴的**上方**，x 轴标题放在左侧沟槽里。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'composite': {
    'type': 'coverage',
    'title': 'Amplicon panel',
    'x_label': 'position (bp)',
    'locus': [1000000, 1000600],
    'samples': [
      {'name': 'tumour', 'depth': [[1000000, 12], [1000150, 44], [1000300, 51],
                                   [1000450, 38], [1000600, 9]]},
      {'name': 'normal', 'depth': [[1000000, 8], [1000150, 29], [1000300, 33],
                                   [1000450, 27], [1000600, 11]]}
    ],
    'variants': [
      {'label': 'SNV', 'color': '#d1495b', 'positions': [1000150, 1000430]},
      {'label': 'InDel', 'color': '#e9c46a', 'positions': [1000310]}
    ],
    'features': [
      {'start': 1000050, 'end': 1000250, 'label': 'amplicon_1'},
      {'start': 1000300, 'end': 1000550, 'label': 'amplicon_2'}
    ],
    'regions': [{'start': 1000100, 'end': 1000500, 'label': 'GENE1'}],
    'coverage_thresholds': [{'depth': 20, 'label': 'min depth'}]
  }
})) AS chart;
```

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `samples` | `{name, depth}`[] | **必填。** 每个样本一条深度轨道；`depth` 是 `[[位置, 深度], …]`。 |
| `locus` | `[start, end]` | 钉住基因组 x 范围；默认取所有数据的并集。 |
| `variants` | `{label, color?, positions}`[] | 分组的变异位点，每组一个图例条目。 |
| `features` | `{start, end, label?}`[] | 轴上方的 feature 带（扩增子 / 引物）。 |
| `regions` | `{start, end, label?}`[] | 紧贴轴上方那一条轨道里的基因组区间（基因 / ORF）。 |
| `coverage_thresholds` | `{depth, label?}`[] | 每条深度轨道上的水平虚线阈值。 |
| `overlay_samples` | boolean | 把所有样本叠进一条共用轨道，而不是每样本一条。 |
| `x_label` | string | x 轴标题（默认 `"position"`）。 |
| `feature_track_name` / `region_track_name` | string | 轨道名（默认 `"features"` / `"genes"`）。 |
| `title` | string | 轨道上方的标题带。 |
| `theme` | string \| object | 与普通图一样的主题。 |
| `tick_font_family` | string | 所有刻度标签（基因组坐标与每条深度轴）的字体。 |
| `width` / `height` | number | 画布尺寸（像素）；高度默认按轨道数算。 |

## brick-pop

`brick_pop` 把一张[砖墙图](../plots/utility/brick.md)包起来，每行配一条频率条，并可加若干列按色图上色的
指标热格。砖墙那一部分的字段名与 `type: "brick"` **完全相同**，直接平铺在 `composite` 对象里 —— 同一份
砖墙配置两种画法都能用。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'composite': {
    'type': 'brick_pop',
    'title': 'Allele frequencies',
    'names': ['hap_1', 'hap_2', 'hap_3'],
    'strigars': [
      ['CAG:A', '8A'],
      ['CAG:A,CAA:B', '6A2B'],
      ['CAG:A', '7A1B']
    ],
    'frequencies': [0.52, 0.31, 0.17],
    'frequency_label': 'Frequency',
    'freq_bar_color': '#4c72b0',
    'metrics': [
      {'label': 'depth', 'values': [120, 80, null], 'color_map': 'viridis'},
      {'label': 'qual', 'values': [30, 28, 35], 'color_map': 'magma', 'min': 0, 'max': 40}
    ],
    'metric_values': true,
    'sorted_by_frequency': true,
    'row_height': 26
  }
})) AS chart;
```

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| *（砖墙字段）* | — | `sequences` / `strigars` / `flanked_strigars`、`names`、`template`、`x_offset`、`row_height`…… 与[砖墙页](../plots/utility/brick.md)完全一致。 |
| `frequencies` | number[] | 每行一个值，顺序与行一致。 |
| `frequency_range` | `[min, max]` | 钉住频率条的刻度；默认在值都像比例时用 `[0, 1]`，否则 `[0, max]`。 |
| `frequency_label` | string | 频率面板的表头（默认 `"Frequency"`）。 |
| `freq_bar_color` | string | 频率条颜色。 |
| `metrics` | `{label, values, color_map?, min?, max?}`[] | 指标热格列，从左到右排在频率条与砖墙之间。`values` 里的 `null` 表示缺失 —— 与 0 不同。 |
| `metric_values` | boolean | 在每个指标格里写出数值（热色改画在格子边框上）。 |
| `na_color` | string | 缺失指标格的填充色。 |
| `sorted_by_frequency` | boolean | 按频率从高到低重排行，砖墙、频率与指标列一起置换。 |
| `title` / `theme` / `width` / `height` | — | 与 `coverage` 相同。 |

## 说明

- **`composite` 取代 `series`。** 两者同时给是错误；`composite` 配 `figure` 也是。
- **不支持数据集。** `dataset` / `encode` 是把列绑到某个 *series* 上的；组合图没有 series，所以数据要内联
  （在 SQL 里用 `list(...)` / `array_agg(...)` 聚合好再传）。
- **长度在这里查。** `frequencies` 与每个 `metrics[].values` 都必须与砖墙行数完全一致，`locus` /
  feature / region 的区间必须 `start < end`。kuva 自己只会 `debug_assert`（release 里静默错位），所以由
  扩展报错。
- **画布尺寸归组合图自己。** 顶层的 `width` / `height` 会被忽略；要设就设在 `composite` 对象里。

## 另见

- [kuva — 覆盖度图](https://psy-fer.github.io/kuva/plots/coverage.html) —— 绘图库自己的图型参考。
- [kuva — Brick pop 图](https://psy-fer.github.io/kuva/plots/brick_pop.html) —— 绘图库自己的图型参考。
- [砖墙图](../plots/utility/brick.md) —— 同一批行的朴素版本。
- [多面板（figure）](./figure.md) —— 那是 *series* 的网格排版，属于另一种组合方式。
