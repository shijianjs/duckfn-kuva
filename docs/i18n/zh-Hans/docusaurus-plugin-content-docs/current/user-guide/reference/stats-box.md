---
title: 统计框
sidebar_position: 11
description: 图角上的一小块文字，放样本量、拟合统计量或模型名。
---

# 统计框

统计框是画在图区内的一块**预先排版好的**文字 —— 样本量、R²、p 值、AUC、模型名。它解决的是一个很具体的呈现问题：
直接浮在数据上的文字会压住点、跟图本身分不开，而且数据一变就得手工挪位置。

它是**版面**特性而不是某个图型的功能，所以任何有普通坐标轴的图都能用。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `entries` | string[] | 逐行文字，一个字符串一行。 |
| `title` | string | 条目上方的粗体标题。 |
| `position` | string | 框放在哪 —— 与 [`legend.position`](./legends.md)同一套取值，含 `inside_*` / `outside_*`。默认 `inside_top_left`。 |
| `border` | boolean | 是否画背景与外框（默认开）。 |

## 数字怎么格式化

这个框自己不做任何格式化：字符串是你拼的，所以小数位、单位、括号都由你的查询决定。在 SQL 里通常就是把聚合值
直接拼进条目：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT expression FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
)
SELECT kuva_render(to_json({
  'x_axis': {'name': 'Control'},
  'y_axis': {'name': 'expression'},
  'stats_box': {
    'title': 'Control group',
    'entries': ['n = ' || (SELECT count(*) FROM d),
                'mean = ' || (SELECT round(avg(expression), 2) FROM d),
                'sd = ' || (SELECT round(stddev_samp(expression), 2) FROM d)]
  },
  'series': [{'type': 'strip', 'groups': [{'label': 'Control', 'values': (SELECT list(expression) FROM d)}]}]
})) AS chart;
```

## 位置

`position` 与图例同名，所以 `inside_top_right`、`inside_bottom_left`、`outside_right_top` 等等都能用，
比较时忽略大小写与分隔符。

| 放哪 | 什么时候选它 |
| --- | --- |
| `inside_top_left`（默认） | 数据区的左上角是空的。 |
| `inside_top_right` / `inside_bottom_right` | 数据从那个角往外走开。 |
| `outside_right_top` | 一点都不许挡数据 —— 改成画布变宽。 |
| `outside_bottom_center` | 读起来像是图下面的图注。 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'stats_box': {
    'entries': ['no box', 'no border'],
    'position': 'inside_bottom_right',
    'border': false
  },
  'series': [{'type': 'scatter', 'data': array_agg([x, y]), 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

`border: false` 留下文字、去掉它后面的矩形 —— 浅色背景上、数据又分得开时，这是更对的选择。

## 与图例共用一个角

把框和图例放在同一个位置，它们会自己叠：框画在图例条目**下面**，而不是压上去，所以不用手工算坐标。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'inside_top_right'},
  'stats_box': {'entries': ['n = 300'], 'position': 'inside_top_right'},
  'series': [{'type': 'scatter', 'legend': 'measured', 'data': (SELECT array_agg([x, y]) FROM d)}]
})) AS chart;
```

## 用它代替浮动的拟合统计量

散点图的趋势线可以把方程与相关系数当成浮动文字印出来（`trend.equation` / `trend.correlation`）。点稀疏时没问题；
点一密，那些文字就压在点上。同一批数字放到统计框里更合适：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'stats_box': {
    'title': 'linear fit',
    'entries': ['n = 300', 'R² = 0.38'],
    'position': 'inside_top_right'
  },
  'series': [{'type': 'scatter', 'color': 'steelblue', 'data': (SELECT array_agg([x, y]) FROM d),
              'trend': 'linear'}]
})) AS chart;
```

## 说明

- **`entries` 可以为空** —— 那就只剩一个带标题的框。
- 框本身不算任何东西，所以它可以显示根本不在图里的数字（来自另一张表的 AUC、某个阈值、一段版本号）。
- 标签里的数学公式在这里也能用：`'R$^2$ = 0.38'` 会渲染成上标。见[标签里的数学公式](./math.md)。
- `outside_*` 的位置会让画布变大，`inside_*` 永远不会 —— 这也是默认值选在里面的原因。

## 另见

- [图例](./legends.md) —— 完整的位置取值。
- [series 与通用字段](./series.md) —— `trend`，含 `equation` / `correlation`。
- [标签里的数学公式](./math.md) —— 条目里的上下标与希腊字母。
