---
title: 日期与时间轴
sidebar_position: 9
description: 把数值轴变成日期轴 —— 刻度的单位、步长与格式。
---

# 日期与时间轴

kuva 的坐标轴都是数值轴：时间轴也无非是一些数字。`x_datetime` 与 `y_datetime` 让渲染器把某一根轴的取值当作
**Unix 秒级时间戳**，并把刻度按日历排布。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `unit` | string | 刻度间隔的日历单位：`"auto"` · `"year"` · `"month"` · `"week"` · `"day"` · `"hour"` · `"minute"` · `"second"`。 |
| `step` | integer | 每几个单位一个刻度（默认 `1`）。 |
| `format` | string | `strftime` 风格的标签格式，如 `"%Y-%m-%d"`。`unit` 不是 `"auto"` 时必填。 |

取值本身仍来自数据，所以要给 series 一列 epoch 秒 —— DuckDB 里就是 `epoch(date_column)`。

## 怎么造时间戳

不管源列长什么样，DuckDB 都能把秒算出来：

| 源列 | 当作坐标传什么 |
| --- | --- |
| `DATE` | `epoch(CAST(d AS DATE))` |
| `TIMESTAMP` | `epoch(ts)` |
| 字符串 | `epoch(strptime(s, '%Y-%m-%d %H:%M:%S'))` |
| 已经是毫秒 epoch | 先除以 1000 再画 |

## 单位与格式

| `unit` | 刻度间隔 | 配得上的格式 |
| --- | --- | --- |
| `"year"` | 一年 | `"%Y"` |
| `"month"` | 一个月 | `"%b %Y"` |
| `"week"` | 一周（周一） | `"%b %d"` |
| `"day"` | 一天 | `"%Y-%m-%d"` |
| `"hour"` | 一小时 | `"%H:%M"` |
| `"minute"` | 一分钟 | `"%H:%M"` |
| `"second"` | 一秒 | `"%H:%M:%S"` |

月度轴上写 `step: 2` 就是每两个月一个刻度，以此类推。

## 示例

一段时间里的收盘价，一个月一个刻度：

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

## 交给 kuva 自己挑 —— `"unit": "auto"`

事前不知道跨度时，把轴的范围交出去，让 kuva 自己挑单位与格式：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_datetime': {'unit': 'auto'},
  'x_axis': {'name': 'date', 'min': 1704067200, 'max': 1735689600},
  'y_axis': {'name': 'close'},
  'series': [{'type': 'line', 'data': pts, 'color': '#4c72b0'}]
})) AS chart
FROM (
  SELECT array_agg([epoch(CAST(date AS DATE)), close] ORDER BY date) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

`auto` 需要知道轴的范围：要么像上面那样把轴的 `min` 与 `max` 都给全，要么让 series 自己报一个范围 —— 有数据可依的
图型都会报。这个模式下 `format` 与 `step` 会被忽略；其余单位则必须给 `format`，缺了会报错，而不是替你猜一个。

## 用在 y 轴上

时间沿竖直方向走时（甘特图、样本时间线），`y_datetime` 与 `x_datetime` 完全一样：

```json
{ "y_datetime": {"unit": "day", "format": "%Y-%m-%d"} }
```

## 说明

- **取值是秒，不是毫秒、也不是日期。** 毫秒级的 epoch 会被画成五万年后的某一天，表现出来是空图或范围离谱的轴，
  而不是一条错误。
- **具名单位必须给 `format`** —— kuva 用它（`chrono` 的 `strftime` 语法）写刻度标签，所以没有「合理的默认」可退回。
- **粗一点的单位更好读。** 长时间跨度上用日内单位，刻度标签会互相压住；用 `auto`，或者干脆粗一档。
- 日期轴与轴的**标题**互不影响：仍然由 `x_axis.name` 命名，[标签里的数学公式](./math.md)在那里也有效。

## 另见

- [画布、标题与坐标轴](./layout.md) —— 其余的轴字段。
- [K 线图](../plots/time-series/candlestick.md) —— 带价格带的时间轴。
- [甘特图](../plots/time-series/gantt.md) —— 日期轴在 y 侧。
