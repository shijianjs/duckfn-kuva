---
title: 图例图
sidebar_position: 4
description: 单独画一份图例，供报告与幻灯片使用。
---

# 图例图

图例图把一份图例单独画出来 —— 共享图例该放在图旁边而不是图里时，或者你想要一张图例图片放进幻灯片时用它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'entries': [
      to_json({'label': 'control', 'color': '#4c72b0'}),
      to_json({'label': 'drug A', 'color': '#c44e52', 'shape': 'line', 'dasharray': '4 2'}),
      to_json({'label': 'drug B', 'color': '#55a868', 'shape': 'circle'}),
      to_json({'label': 'drug C', 'color': '#8172b2', 'shape': {'marker': 'triangle'}}),
      to_json({'label': 'dose', 'color': '#937860', 'shape': {'size': 6}})
    ],
    'cols': 2,
    'title': 'groups',
    'show_box': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `entries` | entry[] | **必填。** 每个图例条目一项：`{label, color, shape?, dasharray?}`。 |
| `cols` | integer | 固定列数。 |
| `max_cols` | integer | 自动布局时的列数上限。 |
| `max_entries` | integer | 最多显示几条（**至少 1**）。 |
| `title` | string | 粗体图例标题。 |
| `show_box` | boolean | 给图例画外框（`false` 去掉）。 |

条目的 `shape` 取 `"rect"`（默认）· `"line"` · `"circle"`，或 `{"marker": "triangle"}`（散点的 marker
形状）、`{"size": 6}`（大小不同的圆）。`dasharray` 用于 `"line"` 形状。

## 说明

- **`entries` 不能为空**，且 **`max_entries` 至少为 1** —— 0 会下溢。
- `type` 也可以写别名 `"legend"`。

## 另见

- [kuva — 图例图](https://psy-fer.github.io/kuva/plots/legend.html) —— 绘图库自己的图型参考。
- [图例](../../reference/legends.md) —— 图内的图例。
