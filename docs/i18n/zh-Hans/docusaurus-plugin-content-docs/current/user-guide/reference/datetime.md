---
title: 日期与时间轴
sidebar_position: 9
description: 把数值轴变成日期轴，以及刻度的单位、步长与格式。
---

# 日期与时间轴

kuva 的轴是数值的：时间轴不过是一串数字。`x_datetime` 与 `y_datetime` 告诉渲染器把某根轴的取值当作
**Unix 时间戳（秒）**，把它的刻度按日历日期排出来。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `unit` | string | 刻度间隔的日历单位：`"year"` · `"month"` · `"week"` · `"day"` · `"hour"` · `"minute"` · `"second"`。 |
| `step` | integer | 每几个单位一个刻度（默认 `1`）。 |
| `format` | string | `strftime` 风格的标签格式，例如 `"%Y-%m-%d"`。 |

`unit` 与 `format` 必填，`step` 可选。数值本身仍然来自数据，所以要给 series 一列 epoch 秒 —— 在 DuckDB 里
就是 `epoch(日期列)`。

## 示例

一段收盘价走势，每月一个刻度：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_datetime': {'unit': 'month', 'step': 1, 'format': '%Y-%m'},
  'x_axis': {'name': 'date'},
  'y_axis': {'name': 'close'},
  'series': [{'type': 'line', 'data': pts, 'color': '#4c72b0', 'fill': true, 'fill_opacity': 0.1}]
})) AS chart
FROM (
  SELECT array_agg([epoch(CAST(date AS DATE)), close] ORDER BY date) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

## 说明

- **取值的单位是秒，不是微秒，也不是日期本身。** 用 `epoch()` 转换；已经是时间戳的列要写 `epoch(ts)`，
  而不是 `ts`。
- **跨度长时，日期轴要到周或更粗的单位才读得清。** 长跨度上用天以下的单位会挤成一团。
- 日期轴与[坐标轴标题](./layout.md)互不影响：`x_axis.name` 仍然是它的标题。
