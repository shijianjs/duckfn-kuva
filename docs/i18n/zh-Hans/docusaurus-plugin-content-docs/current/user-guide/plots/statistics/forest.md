---
title: 森林图
sidebar_position: 4
description: 逐行的点估计与置信区间，标记大小可编码权重。
---

# 森林图

森林图逐行列出点估计与它的置信区间，通常配一条零效应线。标记大小可以编码一项研究的权重。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'effect size'},
  'series': [{
    'type': 'forest',
    'rows': list({'label': study, 'estimate': estimate, 'ci_lower': ci_lower,
                  'ci_upper': ci_upper, 'weight': weight}),
    'null_value': 0,
    'cap_size': 3
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `rows` | row[] | **必填。** 每行一项：`{label, estimate, ci_lower, ci_upper, weight?, color?}`。 |
| `marker_size` | number | 基准权重下的标记大小。 |
| `whisker_width` | number | 置信区间横线的粗细。 |
| `null_value` | number | 零效应参考线的位置。 |
| `show_null_line` | boolean | 画那条参考线。 |
| `cap_size` | number | 区间端帽的宽度（0 = 不画端帽）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **`rows` 不能为空**，且每行的 `ci_lower` 不能大于 `ci_upper`。
- `weight` 按 `sqrt(weight / max_weight)` 缩放标记。

## 另见

- [kuva — 森林图](https://psy-fer.github.io/kuva/plots/forest.html) —— 绘图库自己的图型参考。
- [散点图](../relationships/scatter.md) —— 散点上的误差棒。
