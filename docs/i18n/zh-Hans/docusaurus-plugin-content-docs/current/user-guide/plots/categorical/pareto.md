---
title: 帕累托图
sidebar_position: 5
description: 降序排列的柱子，配一条固定在 0–100 % 轴上的累计百分比线。
---

# 帕累托图

帕累托图是「按值降序的柱状图」加一条副轴上的累计百分比折线，副轴固定为 0–100 %。默认在 80 % 处画一条虚线，
一眼就能看出「几个类别占掉了大部分」—— 就是那张「二八法则」图，在质量控制、变异分析、错误归类的场景里很常见。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Support ticket error categories',
  'series': [{
    'type': 'pareto',
    'categories': cats
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

画之前会按值降序排序（`"sorted": false` 就保留你给的顺序）。「Value」/「Cumulative %」的图例默认开着 ——
柱子与折线本来就总是同时出现。

副轴在**构造上**就是百分比，没有可配的东西，刻度按 `0%, 20%, …` 排版。类别轴默认旋转并做防重叠抽稀，因为帕累托
数据的类别数通常比手搭的柱状图多。

## 样式

`color` 是柱子的填充，`line_color` 是累计线，`threshold` 是参考线的位置（给了它就同时打开参考线），
`cumulative_labels` 把百分比写在每个折线点上方。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Styled Pareto',
  'series': [{
    'type': 'pareto',
    'categories': cats,
    'color': 'seagreen',
    'line_color': 'darkorange',
    'threshold': 90,
    'cumulative_labels': true,
    'bar_legend_label': 'count',
    'line_legend_label': 'cumulative %'
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

## 把长尾并成「其他」

真实的缺陷日志总有一堆只出现一次、把坐标轴挤爆的小类别。`max_categories` 把排在 `n - 1` 名之后的全并成一根柱子
—— 但它不是把尾巴加成一个大数糊过去，而是画成**一摞堆叠**，每个成分各有自己的图例条目，什么都没藏。
`other_label` 给它改名（默认 `"Other"`）。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Top 5 + Other',
  'series': [{
    'type': 'pareto',
    'categories': cats,
    'max_categories': 5,
    'other_label': 'Other',
    'cumulative_labels': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

这一摞在累计线上只贡献**一个**点，与它在轴上占的一格对应 —— 所以哪怕六个类别被塞进了一根柱子，曲线也还是诚实的。

## 横向

`horizontal` 把类别放到 y 轴。这时累计线会挪到画在顶部的一条副 **x** 轴上，因为副轴永远跟着「装数值」的那根轴走。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal Pareto',
  'series': [{
    'type': 'pareto',
    'categories': cats,
    'horizontal': true,
    'cumulative_labels': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `categories` | category[] | **必填。** 每个类别 `{label, value}`。给**原始值**，不要给累计值。 |
| `color` | string | 柱子的填充色（默认 `steelblue`）。 |
| `line_color` | string | 累计线的颜色（默认 `firebrick`）。 |
| `width` | number | 柱宽占类别槽位的比例（默认 `0.8`）。 |
| `sorted` | boolean | 按值降序排（默认开；`false` 保留你给的顺序）。 |
| `cumulative_labels` | boolean | 在每个累计折线点旁写百分比。 |
| `show_threshold` | boolean | 画虚线参考线（默认开）。 |
| `threshold` | number | 参考线的水平（百分比，默认 `80`）。 |
| `max_categories` | integer | 把 `n - 1` 名之后的全并成一根堆叠柱。 |
| `other_label` | string | 那根柱子的名字（默认 `"Other"`）。 |
| `bar_legend_label` / `line_legend_label` | string | 柱子与折线的图例文字。 |
| `show_legend` | boolean | 画图例（默认开）。 |
| `horizontal` | boolean | 类别在 y 轴；累计线挪到顶部的副 x 轴。 |

## 说明

- **累计线是用你给的数值算的** —— 传原始计数，千万别传已经累计过的。
- `sorted` 默认 `true`；帕累托图的意义就在降序，`false` 是留给「类别本身有必须保留的自然顺序」的场合。
- `max_categories` 把「其他」那根柱子算在 `n` 个槽位之内。
- 全为 0 的数值是报错（没有总量可以取百分比）。

## 另见

- [kuva — 帕累托图](https://psy-fer.github.io/kuva/plots/pareto.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) —— 普通的分类柱，没有累计线。
- [瀑布图](../time-series/waterfall.md) —— 带累计值的柱子。
