---
title: 森林图
sidebar_position: 4
description: 每个研究一行的效应量与置信区间，外加一条无效参考线。
---

# 森林图

森林图把多项研究的效应量与置信区间画在一张图里：y 轴上是研究标签，一条横向的置信区间须线，以及在点估计处的一个
方块。一条竖直虚线标出「无效」的位置，于是「这个区间有没有跨过它」就是读者最先看的东西。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Meta-analysis: treatment effect',
  'x_axis': {'name': 'effect size (95 % CI)'},
  'series': [{
    'type': 'forest',
    'rows': rows,
    'null_value': 0
  }]
})) AS chart
FROM (
  SELECT list({'label': study, 'estimate': estimate,
               'ci_lower': ci_lower, 'ci_upper': ci_upper}) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv')
);
```

各行按列表顺序自上而下画 —— 而这个顺序本身就是论证的一部分：把合并估计放在最后（或最前），让它与各研究在视觉上
分开，因为它并不是又一项研究。

## 按权重缩放 marker

`weight` 会让这一行的 marker 按 `sqrt(weight / max_weight)` 缩放 —— 这是表达「这项研究在合并估计里占多少分量」的
标准做法。不按权重画，每项研究看起来就一样有影响力。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Weighted by study size',
  'x_axis': {'name': 'effect size (95 % CI)'},
  'series': [{
    'type': 'forest',
    'rows': rows,
    'marker_size': 6,
    'null_value': 0
  }]
})) AS chart
FROM (
  SELECT list({'label': study, 'estimate': estimate,
               'ci_lower': ci_lower, 'ci_upper': ci_upper,
               'weight': weight}) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv')
);
```

## 尺寸与无效线

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `marker_size` | `6` | marker 的基础半宽（像素） |
| `whisker_width` | `1.5` | 置信区间横线的粗细 |
| `null_value` | `0` | 虚线无效线的位置 |
| `show_null_line` | `true` | 是否画它 |
| `cap_size` | `0` | 区间端帽的半高 —— `0` 就是不画端帽 |

研究数量不多时值得打开 `cap_size`：端帽让区间的**范围**看得见，而不是靠线在哪里断掉去猜。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With end caps and a ratio null',
  'x_axis': {'name': 'risk ratio (95 % CI)'},
  'series': [{
    'type': 'forest',
    'rows': [
      {'label': 'Smith 2019',    'estimate': 0.72, 'ci_lower': 0.55, 'ci_upper': 0.94},
      {'label': 'Johnson 2020',  'estimate': 0.94, 'ci_lower': 0.78, 'ci_upper': 1.14},
      {'label': 'Williams 2020', 'estimate': 0.81, 'ci_lower': 0.63, 'ci_upper': 1.04},
      {'label': 'Overall',       'estimate': 0.83, 'ci_lower': 0.72, 'ci_upper': 0.96, 'color': '#333333'}
    ],
    'null_value': 1,
    'cap_size': 4,
    'whisker_width': 2
  }]
})) AS chart;
```

比值型的效应量，无效值是 `1`、不是 `0` —— 这是森林图最常见的那个错误。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `rows` | row[] | **必填。** 每项是 `{label, estimate, ci_lower, ci_upper, weight?, color?}`。 |
| `weight` | number | 缩放 marker：`sqrt(weight / max_weight)`。 |
| `color` | string | 这一行的颜色；盖过 series 的颜色。 |
| `marker_size` | number | marker 的基础半宽（默认 `6`）。 |
| `whisker_width` | number | 置信区间线宽（默认 `1.5`）。 |
| `null_value` | number | 无效参考值（默认 `0`）。 |
| `show_null_line` | boolean | 画那条虚线（默认开）。 |
| `cap_size` | number | 区间端帽的半高（`0` = 不画）。 |

## 说明

- **`rows` 不能为空**，且每行都要给全 `label`、`estimate`、`ci_lower`、`ci_upper`。
- **`ci_lower` 不能大于 `ci_upper`** —— 区间反了是报错，而不是悄悄换过来。
- `null_value` 取决于效应量本身：均值差或取过对数的比值是 `0`，比值本身是 `1`。
- 行序就是列表顺序，所以「合并行放在哪」是你在查询里做的一个决定。
- 权重只改 marker；置信区间是照给的画，所以加权与不加权的区间画出来是同一条线。

## 另见

- [kuva — 森林图](https://psy-fer.github.io/kuva/plots/forest.html) —— 绘图库自己的图型参考。
- [生存曲线](./survival.md) —— 时间-事件曲线，而不是合并估计。
- [坡度图](../categorical/slope.md) —— 更简单的两值对比。
