---
title: 马赛克图
sidebar_position: 12
description: 一次画两个分类变量 —— 列宽一个、段高一个。
---

# 马赛克图

马赛克图（Marimekko 图）同时编码两个分类变量。列的**宽度**正比于该列的总量，列内每一段的**高度**是该行分类的占比
—— 于是每个格子的*面积*正比于它的联合频数。它就是一张用面积编码的列联表。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'Outcomes by region',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'proportion'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'legend': 'outcome'
  }]
})) AS chart;
```

每个组合一项。不点名的话，列序与行序都按**首次出现**排 —— 所以这个例子把两者都写了出来，否则版面就取决于你的
查询恰好按什么顺序出产行。

## 颜色与顺序

`group_colors` 按行分类逐个给色，**按 `row_order` 的位置对应**；`gap` 是格子之间的像素缝，列与列之间、段与段之间
都算。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'With custom colours',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'proportion'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'group_colors': ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728'],
    'gap': 3,
    'legend': 'outcome'
  }]
})) AS chart;
```

## 格子里的标签

`percents`（默认开）写每个格子在本列中的占比；`values` 再加上原始数值。只关心计数时就把 `percents` 关掉；
`min_label_height` / `min_label_width` 用来阻止文字被硬塞进放不下的小格子里。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'Raw values',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'count'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'percents': false,
    'values': true,
    'legend': 'outcome'
  }]
})) AS chart;
```

## 列不归一

默认每列会拉开到整幅图的高度，于是只有列**内部**的拆分可比。`"normalize": false` 改成让列高正比于它在总量中的
占比 —— 这时版面也把「各组大小差多少」显示出来了：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'Column heights as shares',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'share of all'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'normalize': false,
    'legend': 'outcome'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `cells` | cell[] | **必填。** 每个组合一项：`{col, row, value}`。 |
| `col_order` | string[] | 列顺序（默认按首次出现）。 |
| `row_order` | string[] | 行 / 段顺序（默认按首次出现）。 |
| `group_colors` | string[] | 逐行颜色，按 `row_order` 的位置对应。 |
| `gap` | number | 格子之间的像素缝（默认 `2`）。 |
| `percents` | boolean | 格子里写百分比（默认开）。 |
| `values` | boolean | 格子里写原始数值（默认关）。 |
| `min_label_height` / `min_label_width` | number | 小于这个尺寸的格子就不写标签。 |
| `normalize` | boolean | 每列拉开到满高（默认开）。 |
| `legend` | string | 图例标题；每个行分类一条。 |

## 说明

- **`cells` 不能为空。** 缺的 `col` × `row` 组合按 `0` 处理，重复的组合会求和。
- `col_order` / `row_order` 不做过滤：在 `cells` 里出现、但不在顺序列表里的分类照样会画。
- 列的**宽度**永远编码列总量 —— 这不是可切换的，那正是它成为马赛克图的原因。
- 各组大小悬殊时，`normalize: false` 才是诚实的画法。

## 另见

- [kuva — 马赛克图](https://psy-fer.github.io/kuva/plots/mosaic.html) —— 绘图库自己的图型参考。
- [韦恩图](./venn.md) · [UpSet 图](./upset.md) —— 换成集合交叠的视角，而不是列联表。
- [骰子图](./dice_plot.md) —— 逐格的多变量网格。
