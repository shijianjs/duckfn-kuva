---
title: 带状区间图
sidebar_position: 4
description: 在上下两条边界之间填色 —— 置信带或取值范围。
---

# 带状区间图

带状区间图在两条边界之间填充。置信区间、误差范围、任何「正常区间」都用它；在上面叠一条[折线](./line.md)
就是中心估计。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'band',
    'x': xs, 'y_lower': los, 'y_upper': ups,
    'color': 'steelblue', 'opacity': 0.3, 'legend': '±0.5'
  }]
})) AS chart
FROM (
  SELECT list(time ORDER BY time) AS xs,
         list(value - 0.5 ORDER BY time) AS los,
         list(value + 0.5 ORDER BY time) AS ups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `x` | number[] | **必填。** 共享的 x 位置（至少 2 个）。 |
| `y_lower` | number[] | **必填。** 下边界，与 `x` 等长。 |
| `y_upper` | number[] | **必填。** 上边界，与 `x` 等长。 |
| `color` | string | 填充色。 |
| `opacity` | number | 填充不透明度（不能为负）。 |
| `legend` | string | 图例文字。 |

`type` 还接受别名 `"interval"`。

## 说明

- **`x` 至少要有两个值**，且 `y_lower` / `y_upper` 必须与它等长 —— 长度对不上会报错，而不是静默截断。
- **`opacity` 不能为负**（负值会拼出非法的颜色串）。

## 另见

- [kuva — 带状区间图](https://psy-fer.github.io/kuva/plots/band.html) —— 绘图库自己的图型参考。
- [折线图](./line.md) —— 带子的中心线。
- [散点图](./scatter.md) —— 逐点 `band`，画局部区间。
