---
title: 热力图
sidebar_position: 12
description: 行 × 列的数值矩阵，用连续色图编码。
---

# 热力图

热力图把一张二维网格画出来，每格的颜色编码一个数值。数值会归一到数据自身的范围、再过一遍色图，右侧画一条
色条。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Expression heatmap',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene) FROM d),
    'row_labels': (SELECT list(gene ORDER BY gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'color_map': 'viridis'
  }]
})) AS chart;
```

`data` 是**行优先**的：`data[row][col]`，外层列表从上往下、内层从左往右。一张宽表就是「一行一个列表」——
对数值列做一次 `list([…])`，行序由你排。

## 轴标签

`row_labels` 与 `col_labels` 给格子命名。`row_labels` 是**从下往上**的（y 轴的惯例），所以第一个标签落在图的
底部；`col_labels` 从左往右。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene DESC
)
SELECT kuva_render(to_json({
  'title': 'Labelled rows',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene DESC) FROM d),
    'row_labels': (SELECT list(gene ORDER BY gene DESC) FROM d),
    'col_labels': cols,
    'color_map': 'inferno'
  }]
})) AS chart
FROM (
  SELECT ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
          'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'] AS cols
);
```

标签列表要和矩阵**同一个顺序** —— 两者对不上，你会得到一张画得完美、标得全错的图。

## 写上数值

`show_values` 把每格的原始数值写在格子里。小网格值得开，大网格根本读不了。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'gene'},
  'series': [{
    'type': 'heatmap',
    'data': [
      [10.0, 20.0, 30.0, 15.0],
      [45.0, 55.0, 25.0, 60.0],
      [70.0, 35.0, 80.0, 40.0],
      [50.0, 90.0, 65.0, 20.0]
    ],
    'row_labels': ['GeneA', 'GeneB', 'GeneC', 'GeneD'],
    'col_labels': ['Ctrl', 'T1', 'T2', 'T3'],
    'show_values': true,
    'color_map': 'grayscale'
  }]
})) AS chart;
```

## 色图

| `color_map` | 色阶 | 说明 |
| --- | --- | --- |
| `"viridis"` | 蓝 → 绿 → 黄 | 感知均匀、对色盲友好。**（默认）** |
| `"inferno"` | 黑 → 紫 → 黄 | 对比强；灰度打印也还读得出来 |
| `"grayscale"` | 黑 → 白 | 干净的出版风格 |

[色图](../../reference/colormaps.md)里的名字都能用；`legend` 给色条加标题。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'With a colour bar title',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene) FROM d),
    'row_labels': (SELECT list(gene ORDER BY gene) FROM d),
    'color_map': 'grayscale',
    'legend': 'z-score'
  }]
})) AS chart;
```

## 自定义坐标范围

默认列映射到 `[0.5, 列数 + 0.5]`、行映射到 `[0.5, 行数 + 0.5]`，这样整数刻度正好落在格心。当这张网格代表一个
物理区域时，用 `x_range` / `y_range` 把真实坐标放到轴上。

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT i AS row, j AS col,
         ((j + 0.5) * 20.0 / 40.0) - 10.0 AS x,
         4.0 - ((i + 0.5) * 8.0 / 16.0) AS y
  FROM (SELECT unnest(range(0, 16)) AS i), (SELECT unnest(range(0, 40)) AS j)
),
rows AS (
  SELECT row, list(exp(-(x * x / 16.0 + y * y / 4.0) / 2) ORDER BY col) AS row_vals
  FROM g
  GROUP BY row
)
SELECT kuva_render(to_json({
  'title': 'Scalar field',
  'x_axis': {'name': 'x (m)'},
  'y_axis': {'name': 'y (m)'},
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY row) FROM rows),
    'color_map': 'inferno',
    'x_range': [-10, 10],
    'y_range': [-4, 4]
  }]
})) AS chart;
```

两侧也可以只给一边 —— 只钉 x 而让 y 留在整数刻度上，或者反过来。

## 格子大小

`cell_size` 是每格矩形占自己槽位的比例。默认 `0.99` 留一道发丝缝，格子的边界看得见；`1.0` 是严丝合缝、没有网格线
—— 大矩阵上就该用它，否则那些缝会变成一片干扰性的花纹。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Flush cells',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene) FROM d),
    'color_map': 'magma',
    'cell_size': 1.0
  }]
})) AS chart;
```

## 行序

热力图好不好用，全看行序。这里没有隐藏的排序：让矩阵与标签按同一个顺序排，画出来就是这个顺序。按行的均值排序是
最经典的一招 —— 它把矩阵变成一道一眼就能读的渐变。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene,
         [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
          Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals,
         (Sample_01 + Sample_02 + Sample_03 + Sample_04 + Sample_05 + Sample_06
        + Sample_07 + Sample_08 + Sample_09 + Sample_10 + Sample_11 + Sample_12) / 12 AS row_mean
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Rows sorted by mean',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY row_mean) FROM d),
    'row_labels': (SELECT list(gene ORDER BY row_mean) FROM d),
    'color_map': 'viridis',
    'legend': 'z-score'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | number[][] | **必填。** 矩阵，行优先：`data[row][col]`。每行必须等长。 |
| `row_labels` | string[] | 行标签（y 轴，**从下往上**）。 |
| `col_labels` | string[] | 列标签（x 轴，从左往右）。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `show_values` | boolean | 把每格的数值写在格子里。 |
| `x_range` / `y_range` | `[number, number]` | 给坐标轴一套自定义坐标（默认 `[0.5, n + 0.5]`）。 |
| `cell_size` | number | 格子填充比例，内部夹到 `[0.5, 1]`（默认 `0.99`）。 |
| `legend` | string | 色条的标题。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每格一条提示文字，按行优先顺序。 |

热力图**没有 `color` 字段** —— 颜色本身就是编码，所以选颜色只能靠 `color_map`。

## 说明

- **`data` 的每一行必须等长**，且矩阵不能为空。
- 给了 `row_labels` / `col_labels` 就要与行数 / 列数一致 —— 而且顺序必须与 `data` 对齐，因为不会替你重排。
- `cell_size` 会被夹到 `[0.5, 1]`。
- 数值是按矩阵自己的最小最大值归一的，所以两张取值范围不同的热力图并不可比，除非你用 `x_range` / `y_range`
  把坐标钉住 —— 色标跟的是数据，不是坐标范围。

## 另见

- [kuva — 热力图](https://psy-fer.github.io/kuva/plots/heatmap.html) —— 绘图库自己的图型参考。
- 聚类热图是同一张矩阵做层次聚类的结果。
- [二维直方图](./histogram2d.md) · [六边形分箱图](./hexbin.md) —— 矩阵来自点云、而不是表格的时候。
- [色图](../../reference/colormaps.md) —— 所有色图名字。
