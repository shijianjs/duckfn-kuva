---
title: 直方图
sidebar_position: 1
description: 把一列数值分箱、把计数画成柱子，可选叠一条 KDE 曲线。
---

# 直方图

直方图把一维数据分成等宽的区间，每个区间画一根柱子。想知道一列数据「长什么样」，第一件事就是画它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

除非你用 `range` 钉住，否则分箱区间由数据推导 —— 不需要自己去算 min/max（底层库是要的）。

## 箱数

`bins` 是等宽箱的个数（默认 `10`）。箱少一点，噪声被抹平；多一点，细节出来、但每个箱的计数就少了。下面两张图
用的是同一份数据。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 8, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 60, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 固定区间

给 `range` 一对明确的 `[min, max]`，分箱边界就与数据无关了。区间之外的值会被**静默丢弃** —— 想聚焦在某一段、
或者把离群值切掉，就是这么写。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fixed range',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 20,
              'range': [20, 60], 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 归一化直方图

`normalize` 把柱高按最高的那根缩到 `1.0`。这是「峰值归一」—— y 轴是相对频数、不是计数 —— 样本量不同的两个分布
靠它才能比形状。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Normalised histogram',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'relative frequency'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40,
              'normalize': true, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 叠加分布

在 `series` 里列两个直方图，它们共用坐标轴。柱子没有单独的透明度字段，所以用 8 位十六进制颜色做半透明：
`#RRGGBBAA`，其中 `ff` 完全不透明、`80` 约 50 %、`40` 约 25 %。

两个 series 要给**同一个 `range`**，否则各自按自己的 min/max 分箱，两根轴就对不上了：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_B')
)
SELECT kuva_render(to_json({
  'title': 'Overlapping distributions',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'count'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'histogram',
    'values': vals,
    'bins': 24,
    'range': [(SELECT min(expression) FROM d), (SELECT max(expression) FROM d)],
    'color': CASE g WHEN 'Control' THEN '#4682b480' ELSE '#dc143c80' END,
    'legend': g
  } ORDER BY g)
})) AS chart
FROM (SELECT g, list(expression) AS vals FROM d GROUP BY g);
```

## 阶梯轮廓

`step` 把直方图画成只有描边的阶梯，而不是实心柱。叠好几个分布时这是最干净的形态：轮廓互不遮挡，
不必再靠半透明填充。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_B')
)
SELECT kuva_render(to_json({
  'title': 'Step histograms',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'count'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'histogram',
    'values': vals,
    'bins': 24,
    'range': [(SELECT min(expression) FROM d), (SELECT max(expression) FROM d)],
    'color': CASE g WHEN 'Control' THEN '#4682b4' ELSE '#dc143c' END,
    'step': true,
    'legend': g
  } ORDER BY g)
})) AS chart
FROM (SELECT g, list(expression) AS vals FROM d GROUP BY g);
```

## 累积模式

`cumulative` 从左往右累加计数：每根柱子都包含它左边的一切，最后一根等于样本总数。配上 `normalize`
就是经验分布函数（ECDF）那种形状。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Cumulative histogram',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'cumulative count'},
  'series': [{
    'type': 'histogram',
    'values': list(value),
    'bins': 40,
    'color': 'steelblue',
    'cumulative': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 分组堆叠

`groups` 追加额外序列，`stacked` 让它们逐箱相加、一层压一层。主序列（`values`）在最下面；不开
`stacked` 时，各组从同一条基线叠加。

每个组自带 `data`、`color` 与可选的 `label`；不给颜色就按默认调色板顺延。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS grp, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_A', 'Drug_B')
),
agg AS (SELECT grp, list(expression) AS vals FROM d GROUP BY grp)
SELECT kuva_render(to_json({
  'title': 'Stacked groups',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'count'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'histogram',
    'values': (SELECT vals FROM agg WHERE grp = 'Control'),
    'bins': 24,
    'range': [(SELECT min(expression) FROM d), (SELECT max(expression) FROM d)],
    'color': '#4e79a7',
    'legend': 'Control',
    'stacked': true,
    'groups': (SELECT list({
      'data': vals,
      'color': CASE grp WHEN 'Drug_A' THEN '#f28e2b' ELSE '#59a14f' END,
      'label': grp
    } ORDER BY grp) FROM agg WHERE grp <> 'Control')
  }]
})) AS chart;
```

## 自动箱数

`bin_method` 按数据自己挑箱数，**覆盖** `bins`。三种规则：

| 规则 | 箱宽 / 箱数 | 适合 |
| --- | --- | --- |
| `"sturges"` | `ceil(log2(n)) + 1` 箱 | 小样本、近似正态 |
| `"scott"` | 箱宽 `3.49·sd·n^(-1/3)` | 较平滑的数据 |
| `"freedman_diaconis"`（别名 `"fd"`） | 箱宽 `2·IQR·n^(-1/3)` | 有离群点（稳健） |

数据退化（方差或 IQR 为零）时，Scott 与 Freedman–Diaconis 会退回 Sturges。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Freedman–Diaconis bins',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{
    'type': 'histogram',
    'values': list(value),
    'bin_method': 'fd',
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 加权样本

`weights` 给每个样本一个权重：它进箱时贡献的是权重而不是 `1`。列表长度必须**与 `values` 完全一致** ——
对不上会报错，而不是静默退回普通计数。权重只作用于主序列，`groups` 里的序列一律不加权。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Weighted histogram',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'weighted count'},
  'series': [{
    'type': 'histogram',
    'values': [1, 2, 2, 3, 3, 3, 4, 4, 5, 6],
    'weights': [1, 1, 2, 1, 1, 3, 1, 1, 1, 2],
    'bins': 5,
    'color': 'steelblue'
  }]
})) AS chart;
```

## 叠一条 KDE

`kde` 在柱子上再画一条核密度曲线 —— 同一个分布的平滑版，同一套刻度。`kde_bandwidth` 默认按 Silverman 经验法则，
`kde_samples` 决定曲线采样多细。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a KDE curve',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40,
              'color': 'steelblue',
              'kde': true, 'kde_color': 'crimson'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 预分箱输入

如果分箱已经在别处做完了，就把 `edges` 与 `counts` 交给它，而不是原始 `values` —— `edges` 必须正好比 `counts`
多一项。什么都不会重算，柱子就按你给的画。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{
    'type': 'histogram',
    'edges': [100, 150, 200, 250, 300, 350],
    'counts': [42, 118, 205, 163, 57],
    'color': 'steelblue'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `values` | number[] | 要分箱的原始观测值。除非给 `edges` + `counts`，否则必填。 |
| `bins` | integer | 箱数（默认 `10`）。 |
| `range` | `[number, number]` | 按这个区间分箱，而不是按数据自己的 min/max。 |
| `normalize` | boolean | 把计数缩放成最高的柱子为 `1.0`。 |
| `edges` | number[] | 预先算好的箱边界。必须比 `counts` **多一项**。 |
| `counts` | number[] | 预先算好的每箱计数。 |
| `kde` | boolean | 叠一条核密度曲线。 |
| `kde_color` | string | KDE 曲线的颜色。 |
| `kde_bandwidth` | number | KDE 带宽；默认按 Silverman 经验法则。 |
| `kde_samples` | integer | KDE 曲线采样多少个点。 |
| `step` | boolean | 画成只有描边的阶梯，而不是实心柱。 |
| `cumulative` | boolean | 从左往右累加计数。 |
| `bin_method` | string | `"sturges"` · `"scott"` · `"freedman_diaconis"`（别名 `"fd"`）；覆盖 `bins`。 |
| `weights` | number[] | 每个值一个权重；只作用于主序列。 |
| `groups` | object[] | 额外序列，每项 `{data, color?, label?}`，叠加（或堆叠）在主序列上。 |
| `stacked` | boolean | 把 `groups` 逐箱相加，而不是从同一基线叠加。 |

`color`、`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **`values` 或者 `edges` + `counts`。** 都不给是报错；`edges` / `counts` 只给一个会退回 `values` 那条路。
- `edges` 必须递增，且比 `counts` 长一项；对不上是报错。
- 这里的 `range` 是可选的 —— 不给时扩展会用 `values` 自己算。想让两个直方图共用分箱边界时才需要钉住它。
- 一张图里叠多个直方图，就在 `series` 里多列几个 —— 它们共用坐标轴。
- `weights` 与 `bin_method` 只在「拿原始 `values` 分箱」那条路上有意义：和 `edges` + `counts` 一起给会
  报错，因为箱已经定死了。
- `weights` 必须与 `values` 等长。kuva 自己会静默退回普通计数，那样一个错位的 join 就看不出来了 ——
  扩展选择直接报错。
- `bins` 与 `bin_method` 同时给时，`bin_method` 生效。

## 另见

- [kuva — 直方图](https://psy-fer.github.io/kuva/plots/histogram.html) —— 绘图库自己的图型参考。
- [密度曲线图](./density.md) —— 去掉柱子的平滑版本。
- [二维直方图](./histogram2d.md) —— 对*两列*分箱。
- [山脊图](./ridgeline.md) —— 多个分布叠起来比形状。
