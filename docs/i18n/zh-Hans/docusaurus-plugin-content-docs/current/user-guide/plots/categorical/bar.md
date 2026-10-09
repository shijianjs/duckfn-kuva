---
title: 柱状图
sidebar_position: 1
description: 分类柱：简单、逐柱颜色、分组与堆叠。
---

# 柱状图

柱状图把分类数据画成柱子。三种模式 —— 简单、分组、堆叠 —— 用的是同一个结构，区别只在数值怎么摆。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bar chart',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category),
    'values': list(count),
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

`categories` 与 `values` 是两条平行的列表 —— 每根柱子一个标签、一个数值，顺序一致。类别列表比数值列表长是报错，
而不是截断。

## 逐柱颜色

`colors` 给每根柱子单独上色，与 `categories` 按位置对应。柱子本身就是**类别**（突变类型、碱基变异）而不是同一个
测量的重复时，就该用它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Hits by GO term',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': cats,
    'values': vals,
    'colors': cols
  }]
})) AS chart
FROM (
  SELECT
    list(category ORDER BY count DESC) AS cats,
    list(count ORDER BY count DESC) AS vals,
    list(CASE WHEN count >= 2 * avg_count THEN '#c44e52'
              WHEN count >= avg_count     THEN '#dd8452'
              ELSE '#4c72b0' END ORDER BY count DESC) AS cols
  FROM (SELECT category, count, avg(count) OVER () AS avg_count
        FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv'))
);
```

`avg(count) OVER ()` 把均值算到每一行上，于是 CASE 可以在一趟里按「高于还是低于均值」给每根柱子上色 —— 用窗口
函数按均值分组上色，通常是最干净的写法。

## 分组柱状图

给了 `series` 就切到多系列模式：每个系列一项，各有 `name` 与一条 `values`，**每个类别一个值**，顺序与
`categories` 一致。柱子并排画。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv'))
SELECT kuva_render(to_json({
  'title': 'Grouped bars',
  'x_axis': {'name': 'pathway', 'tick_rotate': 45},
  'y_axis': {'name': 'mean expression'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'bar',
    'categories': (SELECT list(pathway ORDER BY pathway) FROM (SELECT DISTINCT pathway FROM d)),
    'series': (SELECT list({'name': ct, 'values': vals} ORDER BY ct)
               FROM (SELECT cell_type AS ct, list(mean_expr ORDER BY pathway) AS vals
                     FROM d GROUP BY cell_type))
  }]
})) AS chart;
```

这份数据是长格式 —— 每个 (pathway, cell type) 一行 —— 所以类别列表要 `DISTINCT`，而每个系列用
`list(… ORDER BY pathway)` 把自己的行收成一条。图例文字取自系列的 `name`，所以 `ORDER BY ct` 同时定下了颜色
顺序与图例顺序。

## 堆叠柱状图

`stacked` 用同一套结构，把各段堆起来而不是并排。每个类别的**总量**和它的拆分同样重要时，就该用它。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv'))
SELECT kuva_render(to_json({
  'title': 'Stacked bars',
  'x_axis': {'name': 'pathway', 'tick_rotate': 45},
  'y_axis': {'name': 'mean expression'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'bar',
    'categories': (SELECT list(pathway ORDER BY pathway) FROM (SELECT DISTINCT pathway FROM d)),
    'series': (SELECT list({'name': ct, 'values': vals} ORDER BY ct)
               FROM (SELECT cell_type AS ct, list(mean_expr ORDER BY pathway) AS vals
                     FROM d GROUP BY cell_type)),
    'stacked': true
  }]
})) AS chart;
```

## 横向

`horizontal` 把图转过来：分类在 y 轴、数值在 x 轴。三种模式都支持；分类名很长时正是该用它 —— 从左往右读，
不用旋转，也不会被截断。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal bar chart',
  'x_axis': {'name': 'hits'},
  'y_axis': {'name': 'GO term'},
  'series': [{
    'type': 'bar',
    'categories': list(category),
    'values': list(count),
    'color': 'steelblue',
    'horizontal': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## 柱宽与间距

`width` 是柱子占分类槽位的比例（默认 `0.8`），`1.0` 就是紧挨在一起。`gap` 是同一个旋钮的另一面，等价于
`1 - width`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Narrower bars',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category),
    'values': list(count),
    'color': 'steelblue',
    'width': 0.5
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## 误差棒

`errors` 给每根柱子挂一条误差 —— 给**一个数**是对称的，给 **`[负臂, 正臂]` 对**是不对称的；`error_color` 与
`error_cap_width` 控制它的外观。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With errors',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY category),
    'values': list(count ORDER BY category),
    'errors': list(count * 0.15 ORDER BY category),
    'error_color': '#333333',
    'error_cap_width': 6,
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

### 怎么选模式

| 想要 | 字段 |
| --- | --- |
| 一个颜色、每类一根 | `categories` + `values` + `color` |
| 每根柱子一个颜色 | `categories` + `values` + `colors` |
| 多系列并排 | `categories` + `series` |
| 多系列堆叠 | `categories` + `series` + `"stacked": true` |

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `categories` | string[] | **必填。** 每根柱子 / 每个类别一个标签。 |
| `values` | number[] | 简单模式下的柱高；每个类别一个。 |
| `colors` | string[] | 简单模式下的逐柱颜色，按位置对应。 |
| `series` | series[] | 分组 / 堆叠模式：每项是 `{name, values, color?}`。 |
| `errors` | (number \| `[number, number]`)[] | 每根柱子一条误差；一个数是对称的，`[负, 正]` 是不对称的。 |
| `error_color` | string | 误差棒的颜色。 |
| `error_cap_width` | number | 误差棒端帽的宽度（像素）。 |
| `width` | number | 柱宽占槽位的比例（默认 `0.8`）。 |
| `gap` | number | 柱子之间的空隙（等价于 `1 - width`）。 |
| `stacked` | boolean | 堆叠各系列，而不是并排。 |
| `horizontal` | boolean | 横向画（数值在 x 轴）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **`categories` 必填**；简单模式下 `values` 必须与它等长。
- 分组 / 堆叠模式下，**每个系列的 `values` 必须每个类别一个值**，且顺序一致。
- 某一根柱子缺逐柱颜色时，回退到统一的 `color`。
- `width: 1` 会完全去掉缝隙；`gap` 与 `width` 是同一件事的两种写法。

## 另见

- [kuva — 柱状图](https://psy-fer.github.io/kuva/plots/bar.html) —— 绘图库自己的图型参考。
- [帕累托图](./pareto.md) —— 柱子加一条累计百分比线。
- [棒棒糖图](./lollipop.md) —— 柱子的轻量替代。
- [瀑布图](../time-series/waterfall.md) —— 带累计值的柱子。
