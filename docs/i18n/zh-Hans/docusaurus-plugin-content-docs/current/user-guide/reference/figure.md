---
title: 多面板（figure）
sidebar_position: 12
description: 把若干面板排成网格，并加上共享的标题、标签与图例。
---

# 多面板（figure）

在顶层加一个 `figure` 对象，渲染就从单张图切换成一组面板的网格。每个格子都是一个完整的[面板](#面板) ——
有自己的坐标轴、标题与 series —— 网格再给它们加一层共享的外框。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `rows` / `cols` | integer | 网格尺寸，都必须 ≥ 1。 |
| `panels` | panel[] | 每格一项，按行优先顺序。长度**必须**等于 `rows * cols`。 |
| `title` | string | 整张图的标题。 |
| `title_size` | integer | 标题字号。 |
| `labels` | string \| string[] | 面板标签：`"uppercase"` · `"lowercase"` · `"numeric"` · `"none"`，或你自己的数组。 |
| `shared_x_all` | boolean | 同一列的面板共用一个 x 范围。 |
| `shared_y_all` | boolean | 同一行的面板共用一个 y 范围。 |
| `shared_legend` | string | 整张图共用一个图例，位置取 figure 级的取值（如 `"right_top"`、`"bottom"`）。 |
| `spacing` | number | 面板之间的间距。 |
| `padding` | number | 图边缘的内边距。 |
| `cell_width` / `cell_height` | number | 单个面板的尺寸；**两个都要给**。 |
| `figure_width` / `figure_height` | number | 整张图的尺寸；**两个都要给**。 |

## 面板

`panels` 里每一项与单张图的顶层对象相同（只是不能再有 `figure`）—— `title`、`x_axis`、`y_axis`、
`grid`、`legend`、`annotations`、`series` 等等。面板的 `series` 不能为空。

## 尺寸

`cell_width` / `cell_height` / `figure_width` / `figure_height` 都别写。kuva 的默认格子是 `500 × 380`；
让它自己排版，每个面板的比例才和单张图一致。把整张 figure 钉进一个又宽又扁的框里，会把每个面板都压扁。

## 示例

一张散点旁边放一张直方图，共用图例与面板字母：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'title': 'Two views',
    'labels': 'uppercase',
    'shared_legend': 'right_top',
    'panels': [
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'legend': 'points',
                   'data': (SELECT array_agg([x, y]) FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))}]},
      {'x_axis': {'name': 'length (bp)'}, 'y_axis': {'name': 'reads'},
       'series': [{'type': 'histogram', 'legend': 'length', 'bins': 30,
                   'values': (SELECT list(value) FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv'))}]}
    ]
  }
})) AS chart;
```

## 说明

[第二坐标轴](./secondary-axes.md)属于某一个面板；一个面板也可以像单张图那样叠加多个 series。
