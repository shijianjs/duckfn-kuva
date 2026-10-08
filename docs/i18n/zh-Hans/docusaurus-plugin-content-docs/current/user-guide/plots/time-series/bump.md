---
title: 排名变化图
sidebar_position: 8
description: 名次随时间变化画成交叉的折线，可给已知名次或原始数值。
---

# 排名变化图

排名变化图把每个系列的**名次**随时间画出来，交叉处就是谁反超了谁。可以给它已知的 `ranks`，也可以给原始
`values` 让它自己排名。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'bump',
    'series': ser,
    'x_labels': xl,
    'show_rank_labels': true,
    'show_series_labels': true
  }]
})) AS chart
FROM (
  SELECT
    (SELECT list({'name': s, 'ranks': rk}) FROM (
      SELECT "series" AS s, list(rank ORDER BY time) AS rk
      FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv')
      GROUP BY "series"
    )) AS ser,
    (SELECT list(CAST(t AS VARCHAR) ORDER BY t) FROM (
      SELECT DISTINCT time AS t FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv')
    )) AS xl
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `series` | series[] | **必填。** 每条线一项：`{name, ranks?, values?, color?}`。 |
| `x_labels` | string[] | 每个时间点一个标签。 |
| `curve_style` | string | `"sigmoid"`（默认）或 `"straight"`。 |
| `show_rank_labels` | boolean | 标出每个点的名次。 |
| `show_series_labels` | boolean | 在折线末端标出系列名。 |
| `dot_radius` | number | 点的半径。 |
| `stroke_width` | number | 线宽。 |
| `highlight` | string | 只把这一条画成强调色。 |
| `legend` | boolean | 显示图例。 |
| `rank_ascending` | boolean | 名次 1 在上（默认）。 |
| `tie_break` | string | 并列名次怎么定：`"average"` · `"min"` · `"max"` · `"stable"`。 |

每个系列给 **`ranks`**（已知名次；某一步给 `null` 表示缺席，折线会断开）**或** `values`（原始数值，自动
排名）。

## 说明

- **排名变化图最多 10 个系列** —— 内置调色板直接索引十种颜色、不取模，第 11 条会报错。
- 每个系列要给 `ranks` 或 `values`，都不能不给；不同系列混用两种写法是允许的（只有给 `values` 的那些一起
  参与排名）。
- 除非 `rank_ascending` 为 `false`，名次 1 在最上面。

## 另见

- [kuva — 排名变化图](https://psy-fer.github.io/kuva/plots/bump.html) —— 绘图库自己的图型参考。
- [斜率图](../categorical/slope.md) —— 只有前后一步。
- [折线图](../relationships/line.md) —— 画数值本身，而不是名次。
