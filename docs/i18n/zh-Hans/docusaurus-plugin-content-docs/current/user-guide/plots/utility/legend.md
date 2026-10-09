---
title: 图例图
sidebar_position: 4
description: 把图例本身画成一格 —— 共用图例与独立图例。
---

# 图例图

图例图是一个只画图例网格、别的什么都不画的面板：没有坐标轴，没有数据。它为两种场合存在 —— 给一张多面板图**一个
共用图例**，而不是每格重复同一个键；以及产出一份**独立图例**，好拼到别的图上或幻灯片里。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Groups',
    'cols': 3,
    'entries': [
      {'label': 'Treatment', 'color': '#4477AA', 'shape': 'rect'},
      {'label': 'Control',   'color': '#EE6677', 'shape': 'rect'},
      {'label': 'Baseline',  'color': '#CCBB44', 'shape': 'line', 'dasharray': '4 2'}
    ]
  }]
})) AS chart;
```

## 条目的形状

| `shape` | 色块 |
| --- | --- |
| `"rect"` | 一个填充方块，分类填充色的默认形状 |
| `"line"` | 一条线，可以用 `dasharray` 设成虚线 |
| `"circle"` | 一个填充圆 |
| `{"marker": "triangle"}` | 散点的任意 marker 形状 |
| `{"size": 6}` | 一个指定半径的圆 —— 尺寸图例的写法 |

`{"size": n}` 这种写法是尺寸图例需要的：气泡图的键要显示三个半径不同的圆，手写一条就是这么写。`dasharray`
只对 `"line"` 形状的色块有效。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Marker key',
    'cols': 2,
    'entries': [
      {'label': 'circle',   'color': '#4c72b0', 'shape': {'marker': 'circle'}},
      {'label': 'triangle', 'color': '#dd8452', 'shape': {'marker': 'triangle'}},
      {'label': 'square',   'color': '#55a868', 'shape': {'marker': 'square'}},
      {'label': 'diamond',  'color': '#c44e52', 'shape': {'marker': 'diamond'}}
    ]
  }]
})) AS chart;
```

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Expressed in',
    'cols': 2,
    'entries': [
      {'label': '80 %', 'color': '#c44e52', 'shape': {'size': 7}},
      {'label': '40 %', 'color': '#c44e52', 'shape': {'size': 5}},
      {'label': '10 %', 'color': '#c44e52', 'shape': {'size': 3}}
    ]
  }]
})) AS chart;
```

::::note[一个列表里的形状要一致]

`"line"` 色块是裸字符串 `"line"`，marker 色块是对象 `{"marker": …}`，尺寸色块是对象 `{"size": …}`。DuckDB 必须给
一个列表单一的元素类型，而它无法把字符串与结构体统一成一种 —— 所以**同一个 `entries` 列表里只能放一种形状**。
要混用，就拆成两个图例图，或者从一张形状本来统一的表里构造这个数组。

::::

## 一张多面板图里的共用图例

用它的理由：在一格一格的图里，每格重复同一个图例会白占图例需要的位置，而且读起来像是四个不同的键。整张图一个
图例面板，读起来就是一个。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 2,
    'cols': 1,
    'cell_width': 460,
    'cell_height': 300,
    'panels': [
      {
        'x_axis': {'name': 'x'},
        'y_axis': {'name': 'y'},
        'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d),
                    'color': 'steelblue', 'legend': 'measurements'}]
      },
      {
        'series': [{
          'type': 'legend_plot',
          'cols': 2,
          'entries': [
            {'label': 'measurements', 'color': 'steelblue', 'shape': 'circle'},
            {'label': 'model fit',    'color': 'firebrick', 'shape': 'line'}
          ]
        }]
      }
    ]
  }
})) AS chart;
```

图例面板的位置由 `rows` / `cols` 决定；给那一行比数据格更矮的高度，图例就读起来像页脚、而不像第二张图。

## 排布、列数与裁剪

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `cols` | 自动 | 固定列数。 |
| `max_cols` | — | 自动排布时的列数上限。 |
| `max_entries` | — | 最多显示几条 |
| `title` | — | 条目上方的粗体标题行 |
| `show_box` | `true` | 画背景与外框 |

`cols: 1` 就是做侧边图例的办法：单列条目正好放进一条窄面板，图的网格再给它自己的一条带。
`max_entries` 是图例长过面板能装下的量时的退路。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Sequential scale',
    'cols': 1,
    'show_box': false,
    'entries': [
      {'label': 'H1', 'color': '#c6dbef', 'shape': 'rect'},
      {'label': 'H2', 'color': '#9ecae1', 'shape': 'rect'},
      {'label': 'H3', 'color': '#6baed6', 'shape': 'rect'},
      {'label': 'H4', 'color': '#4292c6', 'shape': 'rect'},
      {'label': 'H5', 'color': '#2171b5', 'shape': 'rect'}
    ]
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `entries` | entry[] | **必填。** 每条是 `{label, color, shape?, dasharray?}`。 |
| `cols` | integer | 固定列数；不给就按面板宽度自动。 |
| `max_cols` | integer | 自动排布时的列数上限。 |
| `max_entries` | integer | 最多显示几条 —— **至少 1**。 |
| `title` | string | 条目上方的粗体标题。 |
| `show_box` | boolean | 背景与外框（默认开）。 |

## 说明

- **`entries` 不能为空**，且每条都要有 `label` 与 `color`。
- `max_entries` 给 `0` 会被拒绝：布局算术里要减一，0 会下溢。
- 图例图用自己的网格把面板填满、并且不画坐标轴，所以它布局上的标题是白给的 —— 标题要写在图例自己身上。
- `show_box: false` 让条目没有背景 —— 图例压在有色卡片上而不是白底上时，就要这样。
- 这里的条目是**手写**的；没有办法从别的面板的系列里把图例收过来，所以一个共用图例与它描述的各格，得靠手工保持
  一致。

## 另见

- [kuva — 图例图](https://psy-fer.github.io/kuva/plots/legend.html) —— 绘图库自己的图型参考。
- [文字块图](./text.md) —— 另一种注记面板。
- [参考：图例](../../reference/legends.md) —— 自动图例体系，以及普通图里手写键的 `legend.entries`。
