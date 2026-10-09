---
title: 名次变化图
sidebar_position: 8
description: 各系列的名次随时间点怎么走。
---

# 名次变化图

名次变化图（bump chart）跟踪每个系列在若干离散时间点或条件下的**名次**。相邻名次之间连线，最好的名次在最上面
—— 于是「交叉」就是故事本身：一条线跨过另一条，就是一个系列超过了另一个。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv'))
SELECT kuva_render(to_json({
  'title': 'Rank over time',
  'series': [{
    'type': 'bump',
    'series': (SELECT list({'name': series, 'ranks': rs} ORDER BY series)
               FROM (SELECT series, list(rank ORDER BY time) AS rs FROM d GROUP BY series)),
    'x_labels': (SELECT list(CAST(time AS VARCHAR) ORDER BY time)
                 FROM (SELECT DISTINCT time FROM d))
  }]
})) AS chart;
```

每个系列带一条 `ranks`，每个时间点一个值；`x_labels` 给这些时间点命名。两者长度必须一致 —— 它们是按位置配对的。

## 从原始数值自动排名

系列也可以带 `values`，由图表在每个时间点上现排名次。输入本来就是一个分数、而不是一个排名时，这才是诚实的做法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Ranked from values',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'values': [95, 80, 88, 92]},
      {'name': 'Beta',  'values': [80, 95, 72, 86]},
      {'name': 'Gamma', 'values': [70, 85, 95, 78]}
    ],
    'x_labels': ['Q1', 'Q2', 'Q3', 'Q4'],
    'tie_break': 'average'
  }]
})) AS chart;
```

默认**数值越大名次越靠前**。`rank_ascending: true` 翻过来，用于越小越好的量 —— 赛跑用时、错误数、延迟。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Lower is better',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'values': [12.4, 11.8, 11.9, 11.2]},
      {'name': 'Beta',  'values': [13.1, 12.6, 11.5, 10.9]},
      {'name': 'Gamma', 'values': [11.9, 12.2, 12.0, 11.6]}
    ],
    'x_labels': ['Q1', 'Q2', 'Q3', 'Q4'],
    'rank_ascending': true
  }]
})) AS chart;
```

## 强调某一个系列

`highlight` 指定一个系列来强调：它拿到更粗的线和更醒目的端点标签，其余全部降到 20 % 不透明度。想让名次变化图
围绕某一个对象讲，又不想把上下文删掉，这是标准做法。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv'))
SELECT kuva_render(to_json({
  'title': 'Highlighting one series',
  'series': [{
    'type': 'bump',
    'series': (SELECT list({'name': series, 'ranks': rs} ORDER BY series)
               FROM (SELECT series, list(rank ORDER BY time) AS rs FROM d GROUP BY series)),
    'x_labels': (SELECT list(CAST(time AS VARCHAR) ORDER BY time)
                 FROM (SELECT DISTINCT time FROM d)),
    'highlight': 'Beta'
  }]
})) AS chart;
```

## 缺席的时间点

`ranks` 里的 `null` 表示该系列在那个时间点缺席，线会**断开**而不是插值穿过去 —— 这就是「没测」与「排最后」
的区别。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With gaps',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'ranks': [1, NULL, 2, 1]},
      {'name': 'Beta',  'ranks': [2, 1, 1, 3]},
      {'name': 'Gamma', 'ranks': [3, 2, NULL, 2]}
    ],
    'x_labels': ['A', 'B', 'C', 'D']
  }]
})) AS chart;
```

## 并列怎么处理

自动排名遇到相同数值时，`tie_break` 决定名次：

| `tie_break` | 给的名次 |
| --- | --- |
| `"average"` | 它们所占位置的平均值（如 `2.5`、`2.5`）（**默认**） |
| `"min"` | 全部拿最好的名次 |
| `"max"` | 全部拿最差的名次 |
| `"stable"` | 保持输入顺序 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Ties, broken by minimum',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'values': [90, 90, 80]},
      {'name': 'Beta',  'values': [90, 70, 95]},
      {'name': 'Gamma', 'values': [60, 90, 85]}
    ],
    'x_labels': ['Q1', 'Q2', 'Q3'],
    'tie_break': 'min'
  }]
})) AS chart;
```

平均名次是小数 —— 这也正是 `ranks` 接受小数的原因：`2.5` 是合法的预排名次，意思是「并列第二」。

## 曲线形状与标签

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `curve_style` | `"sigmoid"` | `"sigmoid"` 用曲线穿过数据点；`"straight"` 用直线相连 |
| `show_rank_labels` | `false` | 把名次数字写在点里 |
| `show_series_labels` | `true` | 左右两端标出系列名 |
| `dot_radius` | `6` | 点半径（像素） |
| `stroke_width` | `2.5` | 线宽（像素） |
| `legend` | `true` | 显示图例 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Straight lines, rank labels',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'ranks': [1, 3, 2, 1]},
      {'name': 'Beta',  'ranks': [2, 1, 1, 3]},
      {'name': 'Gamma', 'ranks': [3, 2, 3, 2]}
    ],
    'x_labels': ['2021', '2022', '2023', '2024'],
    'curve_style': 'straight',
    'show_rank_labels': true,
    'dot_radius': 8,
    'stroke_width': 2
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `series` | series[] | **必填。** 每项是 `{name, ranks?, values?, color?}`。 |
| `ranks` | (number \| null)[] | 已知的名次；`null` 会断线。 |
| `values` | (number \| null)[] | 原始数值，自动排名；`null` 会断线。 |
| `x_labels` | string[] | 每个时间点一个标签。 |
| `curve_style` | string | `"sigmoid"`（默认）或 `"straight"`。 |
| `rank_ascending` | boolean | 数值小的排前面（默认关）。 |
| `tie_break` | string | `"average"`（默认）· `"min"` · `"max"` · `"stable"`。 |
| `highlight` | string | 按名字强调一个系列；其余变淡。 |
| `show_rank_labels` / `show_series_labels` | boolean | 点里的名次数字、两端的系列名。 |
| `dot_radius` / `stroke_width` | number | 点与线的尺寸。 |
| `legend` | boolean | 显示图例（默认开）。 |

## 说明

- **`series` 不能为空**，且每项要有 `name`，以及 `ranks` 或 `values` 其中之一。
- 一张图里别混用两种：`values` 是在**给了 values 的那组内部**排名的，所以带 `ranks` 和带 `values` 的系列不在
  同一个尺度上。
- 缺席的时间点用 `null`，不要用 `0` —— `0` 是一个名次，而且它会排在第 1 名之前。
- `x_labels` 要覆盖到每一个时间点；短了的话多余的点就没有标签。
- 最好的名次在**最上面**，而 `rank_ascending` 只改变「哪种原始数值拿到小号」，不会翻轴。

## 另见

- [kuva — 名次变化图](https://psy-fer.github.io/kuva/plots/bump.html) —— 绘图库自己的图型参考。
- [坡度图](../categorical/slope.md) —— 只有两个时间点的版本。
- [平行坐标](../relationships/parallel.md) —— 维度不止一条时间轴的排名。
