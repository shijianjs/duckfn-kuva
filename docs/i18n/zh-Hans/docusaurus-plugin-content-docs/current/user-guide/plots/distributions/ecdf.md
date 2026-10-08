---
title: ECDF 图
sidebar_position: 5
description: 经验累积分布曲线，每个分组一条，可选置信带、rug 与分位线。
---

# ECDF 图

ECDF 图画的是 `F(x) = P(X ≤ x)` —— 小于等于某个值的观测占比 —— 一条右连续的阶梯曲线。它是单分布诊断里信息量
最大的图之一：不用分箱、不用选带宽，整个分布一眼看全。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'ECDF',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

## 多组对比

`groups` 里一项一条曲线；分组自己的 `color` 会覆盖统一色。几条 ECDF 放在同一对坐标轴上，是回答「这几个分布到底
一样不一样」最老实的办法 —— 分组有序时曲线不会相交，相交的位置正好是一组反超另一组的地方。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Treatment vs control',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'ecdf',
    'groups': groups
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 互补累积分布

`complementary` 把曲线翻成 `1 − F(x)` —— 生存函数，或者叫超出概率。看读长（有多少比例的读段不短于 N bp？）、
看覆盖度（有多少比例的位置深度不低于 N×？），或者任何「主体部分没意思、就看尾巴」的重尾数据，标准画法都是它。
配上对数 x 轴更自然。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Read length — CCDF',
  'x_axis': {'name': 'read length (bp)', 'log': true, 'tick_format': 'integer'},
  'y_axis': {'name': 'fraction ≥ length'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'reads', 'values': lens}],
    'complementary': true,
    'rug': true,
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list("end" - "start") AS lens
  FROM read_csv_auto('{{DFK_BASE_URL}}data/reads.tsv')
);
```

## 置信带

`confidence_band` 给每条曲线加一条 DKW 95 % 置信带，半宽 `ε = √(ln 40 / 2n)` —— 样本小就宽，样本大就紧。它
把「这两条曲线看着不一样」变成「它们差得比抽样噪声还多」，填充透明度由 `band_alpha` 控制。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'DKW confidence bands',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'ecdf',
    'groups': groups,
    'confidence_band': true,
    'band_alpha': 0.15
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    WHERE "group" IN ('Control', 'Drug_A')
    GROUP BY "group"
  )
);
```

## rug

`rug` 在绘图区底部给每个观测在自己那个 x 位置上画一小段竖线。它把原始样本的真实位置摆出来 —— 聚集、空隙、离群
点，这些光看阶梯曲线会被抹掉的东西。几个分组时，各组的竖线会稍微错开，不至于互相盖住。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a rug',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue',
    'rug': true,
    'rug_height': 8
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

## 分位参考线

`percentile_lines` 在给定的 F 高度上画虚线并在右边缘标注。给的是 **0–1 的 F 值**，所以 `[0.25, 0.5, 0.75]`
标的是四分位数与中位数 —— 读每根线与某条曲线的交点，就是那一组的四分位值。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Quartile reference lines',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'ecdf',
    'groups': groups,
    'percentile_lines': [0.25, 0.5, 0.75]
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 阶梯上的点

`markers` 在每个阶梯的端点画一个圆点，把 ECDF 的离散本质显出来。样本小（三十几个点以内）时值得开；再多，点就
糊进线里了。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'value', 'tick_format': 'integer'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'n=8', 'values': [1.2, 2.4, 2.9, 3.5, 4.1, 5.0, 5.8, 7.2]}],
    'color': 'steelblue',
    'markers': true,
    'marker_size': 4
  }]
})) AS chart;
```

## 平滑 CDF

`smooth` 把阶梯换成由核密度积分出来的平滑 CDF（带宽按 Silverman 规则），`stroke_width` 是线宽，`line_dash`
把它打成虚线。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Smooth CDF',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'ecdf',
    'groups': groups,
    'smooth': true,
    'stroke_width': 2
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每个分组一条曲线，每项是 `{label, values, color?}`。 |
| `complementary` | boolean | 画互补累积分布（`1 − F`，从 1 往下走）。 |
| `confidence_band` | boolean | 每条曲线加一条 DKW 95 % 置信带。 |
| `band_alpha` | number | 置信带的透明度（默认 `0.15`）。 |
| `rug` | boolean | 在坐标轴上给每个观测画一小段竖线。 |
| `rug_height` | number | 竖线的高度（像素，默认 `6`）。 |
| `percentile_lines` | number[] | 这些 **F 高度（0–1）** 上的水平参考线，如 `[0.25, 0.5, 0.75]`。 |
| `markers` | boolean | 在每个阶梯端点画圆点。 |
| `marker_size` | number | 点半径（默认 `3`）。 |
| `smooth` | boolean | 把阶梯换成核密度积分的平滑 CDF。 |
| `smooth_samples` | integer | 平滑 CDF 的采样点数（默认 `200`）。 |
| `stroke_width` | number | 曲线线宽（默认 `1.5`）。 |
| `line_dash` | string | 虚线样式（如 `"4 2"`）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)；分组自己的 `color` 会覆盖它。

## 说明

- **每个分组至少要有一个值**；`groups` 为空是报错。
- `percentile_lines` 收的是**比例，不是百分数** —— 写 `[0.25, 0.5, 0.75]`，不是 `[25, 50, 75]`。
- 样本密的时候，`rug` 配 `percentile_lines` 通常比 `markers` 清楚。
- 问尾部的问题时，`complementary` 配 `x_axis.log`。

## 另见

- [kuva — ECDF 图](https://psy-fer.github.io/kuva/plots/ecdf.html) —— 绘图库自己的图型参考。
- [Q-Q 图](./qq.md) —— 同样的数据与理论分布作比较。
- [密度曲线图](./density.md) —— 用平滑曲线代替精确的阶梯。
