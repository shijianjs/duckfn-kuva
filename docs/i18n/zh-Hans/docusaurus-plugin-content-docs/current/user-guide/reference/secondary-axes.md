---
title: 第二坐标轴（双 Y 轴）
sidebar_position: 10
description: 在同一块画布上用第二根 x 或 y 轴再画一组 series。
---

# 第二坐标轴（双 Y 轴）

一张图通常只有一套坐标轴。要把两个量纲差很远的量画在一起 —— 比如价格与成交量 —— 就把第二个量放进
**`secondary_series`**，并用 `y2_axis`（右侧）或 `x2_axis`（上方）描述它的轴。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `secondary_series` | series[] | 画在第二根轴上的 series，形状与 `series` 相同。 |
| `y2_axis` | object | 右侧的 y 轴。 |
| `x2_axis` | object | 上方的 x 轴。 |

`y2_axis` 与 `x2_axis` 接受[坐标轴字段](./layout.md)的一个子集：

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `name` | string | 坐标轴标题。 |
| `min` / `max` | number | 固定范围。 |
| `log` | boolean | 对数轴。 |
| `tick_format` | string \| integer | 与主轴相同。 |
| `wrap` | integer | 标题按字符数折行。 |
| `label_offset` | `[number, number]` | 把标题平移 `[dx, dy]` 像素。 |

:::warning[第二根 x 轴的两条规则]

第二根 **x** 轴的 `min` 与 `max` 必须**一起**给 —— 只给一个会报错。另外，只有 `secondary_series` 里有东西时
第二根轴才会画出来；只写 `y2_axis` 能解析，但得到的是一张普通的单轴图。

:::

**右轴的范围从 `secondary_series` 的数据里推出来**，所以只写了 `name` 的 `y2_axis` 也画得出那根轴。
`min` / `max` 是可选的覆盖：两个都给就是固定范围，只给一个就固定那一端、另一端继续跟着数据走。
下面的示例就没给。

## 示例

左轴收盘价、右轴成交量，用的是同一批行：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'price vs volume',
  'y_axis': {'name': 'close', 'min': 100, 'max': 200},
  'y2_axis': {'name': 'volume', 'tick_format': 'sci'},
  'legend': {'position': 'outside_right_top'},
  'series': [{'type': 'line', 'data': price, 'legend': 'close', 'color': '#4c72b0'}],
  'secondary_series': [{'type': 'line', 'data': vol, 'legend': 'volume', 'color': '#c44e52'}]
})) AS chart
FROM (
  SELECT
    array_agg([epoch(CAST(date AS DATE)), close] ORDER BY date) AS price,
    array_agg([epoch(CAST(date AS DATE)), volume] ORDER BY date) AS vol
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

## 说明

多面板模式下，第二根轴属于某一个[面板](./figure.md) —— 也就是说一张双 Y 轴图是网格里的一个面板。
