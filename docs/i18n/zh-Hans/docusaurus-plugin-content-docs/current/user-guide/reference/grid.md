---
title: 网格、刻度与画布开关
sidebar_position: 3
description: 网格线、坐标轴线、刻度位置与尺寸，以及整块画布上的开关 —— clamp、等比例、缩放、灰度模式与交互。
---

# 网格、刻度与画布开关

`grid` 对象控制所有**不是数据**的东西：网格线、坐标轴外框、刻度，以及一组改变整块画布画法的开关。

## 网格、坐标轴线与刻度

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `show_grid` | boolean | 画网格线。 |
| `axis_line` | string | `"open"`（默认，只画下边与左边）或 `"box"`（整框）。 |
| `ticks` | integer | 每根轴大致画多少刻度。 |
| `tick_align` | string | `"outside"`（默认）· `"inside"` · `"center"`。 |
| `tick_pos` | string | `"primary"`（默认）· `"both"`（四条边都镜像刻度）。 |
| `grid_line_width` | number | 网格线宽。 |
| `axis_line_width` | number | 坐标轴线宽。 |
| `tick_width` | number | 刻度线宽。 |
| `tick_length` | number | 刻度长度（像素）。 |
| `minor_ticks` | integer | 主刻度之间再分几个小格。 |
| `show_minor_grid` | boolean | 副刻度处也画网格线。 |

## 整块画布的开关

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `clamp_axis` | boolean | 轴范围贴紧数据，不再向外取整。 |
| `clamp_y_axis` | boolean | 同上，但只作用在 y 轴。 |
| `equal_aspect` | boolean | 一个数据单位在两轴上占同样的像素（圆保持是圆）。 |
| `scale` | number | 缩放所有文字与刻度，但**不**改变画布尺寸。 |
| `label_background` | boolean | 给数值标签垫一层背景（柱、饼等），让它在填充色上依然看得清。 |
| `bw_mode` | boolean | 灰度 / 色盲友好模式：调色板换成灰阶，线型与 marker 轮流变化，让 series 仍能区分。见[黑白模式](./bw-mode.md)。 |
| `interactive` | boolean | 往 SVG 里注入悬停 / 点击的 JavaScript。见 [SVG 交互](./interactive.md)。 |
| `wrap` | integer | 一次给所有文字（标题、轴标题、图例）设折行宽度；逐元素设置会覆盖它。见[文字折行](./layout.md)。 |

:::note[`bw_mode` 与 `interactive` 属于画布，不属于 series]

它们写在 `grid` 下，不要写在 series 上。`bw_mode` 是[`palette`](./palettes.md)的可访问性版本；
`interactive` 让 SVG 响应指针，需要有能跑 JavaScript 的宿主（浏览器，而不是栅格导出）。

:::

## 示例

整框 + 副刻度 + 四边镜像刻度，画一张简单柱状图：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'GO term', 'tick_rotate': 60},
  'y_axis': {'name': 'hits'},
  'grid': {'axis_line': 'box', 'minor_ticks': 4, 'show_minor_grid': true, 'tick_pos': 'both'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

灰度模式：同一张多 series 散点，画给打印、也给色盲读者看：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'grid': {'bw_mode': true},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

## 另见

- [调色板](./palettes.md) —— `bw_mode` 的颜色一侧。
- [参考线与标注](./annotations.md) —— 画在网格之上的参考线与阴影区域。
