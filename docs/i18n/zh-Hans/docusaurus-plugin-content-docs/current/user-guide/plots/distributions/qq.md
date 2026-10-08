---
title: Q-Q 图
sidebar_position: 6
description: 与正态分布或基因组期望作比较的分位数-分位数图。
---

# Q-Q 图

Q-Q 图把你的样本分位数放到某个理论分布的分位数上；点落在一条直线上就说明样本与该分布相符。`mode` 决定
拿什么来比。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'theoretical quantiles'},
  'y_axis': {'name': 'sample quantiles'},
  'series': [{
    'type': 'qq',
    'groups': [{'label': 'Control', 'values': vals}],
    'mode': 'normal',
    'reference_line': true,
    'ci_band': true,
    'ci_alpha': 0.12,
    'marker_size': 4,
    'legend': 'Control'
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每组一列点，每项 `{label, values, color?}`。 |
| `mode` | string | `"normal"`（默认）与正态分布比；`"genomic"` 画 `-log10(p)` 的 Q-Q。 |
| `reference_line` | boolean | 画期望的那条直线（`false` 去掉）。 |
| `ci_band` | boolean | 画置信带。 |
| `ci_alpha` | number | 置信带的不透明度。 |
| `lambda` | boolean | 标出基因组膨胀因子 λ（`false` 去掉）。 |
| `marker_size` | number | 点的半径。 |
| `stroke_width` | number | 点的轮廓线宽。 |
| `fill_opacity` | number | 点的填充不透明度（`null` 表示不填充）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)；组自己的 `color` 会覆盖它。

## 说明

- **`mode: "genomic"` 期望 p 值落在 0–1。** 区间外的值会被静默丢弃。
- `mode: "normal"`（默认）下 `values` 就是普通观测值。

## 另见

- [kuva — Q-Q 图](https://psy-fer.github.io/kuva/plots/qq.html) —— 绘图库自己的图型参考。
- [ECDF 图](./ecdf.md) —— 累积分布本身。
- 火山图是差异表达结果的配套视图。
