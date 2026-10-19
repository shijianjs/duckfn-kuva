---
title: 散点图
sidebar_position: 1
description: 逐个 (x, y) 点，支持线性或 LOESS 趋势线、置信带、误差棒、气泡大小、逐点颜色、点标签与十种 marker 形状。
---

# 散点图

散点图把 `(x, y)` 逐个画成 marker。它支持线性与 LOESS 趋势线、误差棒、可变点大小、逐点颜色、点标签与
十种 marker 形状。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Scatter plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'data': array_agg({'x': x, 'y': y})
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

这份数据是同一个文件里的三组、每组 80 个点。当成单个 series 画就是一个颜色 —— 下面「多系列」会把它们拆开。

### 坐标轴选项

除非你用 `x_axis.min` / `x_axis.max` 钉死，否则取值范围由数据推导；`y_axis.log` 可以把某根轴换成对数轴。
这些都在[画布、标题与坐标轴](../../reference/layout.md)里。

## 趋势线

`trend` 叠一条最小二乘拟合线。缩写形式 `"trend": "linear"` 只画线；对象形式还能给颜色、线宽，以及把拟合
统计量写在数据区里。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Linear trend line',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'scatter',
    'size': 5,
    'legend': g,
    'data': pts,
    'trend': {'type': 'linear', 'color': 'crimson', 'equation': true, 'correlation': true}
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg({'x': time, 'y': value} ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

> **提示：** `equation` 与 `correlation` 会把拟合统计量当作浮动文字压在数据区上。点一密，它就和数据抢地方；
> 用[统计框](../../reference/stats-box.md)可以把同一批数字放进带边框的插图里。

### LOESS 平滑

关系不是直线时，直线就是错的模型。`"trend": {"type": "loess"}` 改叠一条 LOESS（局部加权回归）平滑曲线：
每个预测点用距离权重拟合一条局部直线。`span` 是每次局部拟合用到的点比例（`0.05`–`1`，默认 `0.5`）——
越小越贴近局部细节，越大越平滑。

```sql {"type":"duckfn","show":"svg"}
WITH pts AS (
  -- 确定性的带噪正弦：与 kuva 自己的 LOESS 示例同一批 120 个点。
  SELECT i,
         i / 120.0 * 10.0 AS x,
         sin(i / 120.0 * 10.0)
           + 0.6 * ((sin(i * 12.9898) * 43758.5453)
                    - trunc(sin(i * 12.9898) * 43758.5453) - 0.5) AS y
  FROM (SELECT unnest(range(0, 120))::DOUBLE AS i)
)
SELECT kuva_render(to_json({
  'title': 'LOESS smoother (span 0.3)',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 3,
    'data': (SELECT array_agg({'x': x, 'y': y} ORDER BY i) FROM pts),
    'trend': {'type': 'loess', 'span': 0.3, 'color': 'crimson', 'width': 2.5}
  }]
})) AS chart;
```

> **注意：** `equation` 与 `correlation` 描述的是**线性**拟合。LOESS 没有截距与斜率可报，`type` 为
> `"loess"` 时它们会被忽略。

## 置信带

`band` 给一块不确定区间上色。`lower` 与 `upper` 是两列 y 值，**必须与点的 x 位置一一对齐** —— 长度相同、顺序
相同。

```sql {"type":"duckfn","show":"svg"}
WITH t AS (
  SELECT x, x * 1.8 + 0.5 AS y
  FROM (SELECT unnest(range(1, 11))::DOUBLE AS x)
)
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'data': (SELECT array_agg({'x': x, 'y': y} ORDER BY x) FROM t),
    'band': {
      'lower': (SELECT array_agg(y - 1.2 ORDER BY x) FROM t),
      'upper': (SELECT array_agg(y + 1.2 ORDER BY x) FROM t)
    }
  }]
})) AS chart;
```

## 误差棒

逐点误差写在点自己身上。`x_err` / `y_err` 给**一个数**就是对称的两臂，给
**`{"neg": …, "pos": …}` 对象**就是不对称的。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'data': [
      {'x': 1.0, 'y': 2.0, 'x_err': 0.2,  'y_err': 0.6},
      {'x': 2.0, 'y': 4.5, 'x_err': 0.15, 'y_err': 0.8},
      {'x': 3.0, 'y': 5.8, 'x_err': 0.3,  'y_err': 0.4},
      {'x': 4.0, 'y': 8.2, 'x_err': 0.1,  'y_err': 0.9},
      {'x': 5.0, 'y': 10.1, 'x_err': 0.25, 'y_err': 0.5}
    ]
  }]
})) AS chart;
```

### 不对称误差

写成 `{"neg": …, "pos": …}` 对象 —— 两臂长度不必相等。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 6,
    'data': [
      {'x': 1.0, 'y': 5.0, 'y_err': {"neg": 0.3, "pos": 0.8}},
      {'x': 2.0, 'y': 6.0, 'y_err': {"neg": 0.5, "pos": 1.2}},
      {'x': 3.0, 'y': 7.5, 'y_err': {"neg": 0.2, "pos": 1.6}}
    ]
  }]
})) AS chart;
```

## marker 形状

`marker` 有十种形状。几个 series 共用一对坐标轴、光靠颜色分不开时，它最有用。

`"circle"`（默认）· `"square"` · `"triangle"` · `"triangle_down"` · `"diamond"` · `"cross"` ·
`"plus"` · `"star"` · `"pentagon"` · `"hexagon"`

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Marker shapes',
  'x_axis': {'name': 'x'},
  'legend': {'position': 'outside_right_top'},
  'series': series
})) AS chart
FROM (
  SELECT list({
    'type': 'scatter',
    'data': [{"x": 1.0, "y": y}, {"x": 2.0, "y": y}, {"x": 3.0, "y": y}],
    'color': color,
    'size': 7,
    'marker': marker,
    'legend': label
  } ORDER BY y) AS series
  FROM (VALUES
    (1.0,  'steelblue',   'circle',        'Circle'),
    (2.0,  'crimson',     'square',        'Square'),
    (3.0,  'seagreen',    'triangle',      'Triangle'),
    (4.0,  'darkorange',  'triangle_down', 'Triangle down'),
    (5.0,  'purple',      'diamond',       'Diamond'),
    (6.0,  'saddlebrown', 'cross',         'Cross'),
    (7.0,  'teal',        'plus',          'Plus'),
    (8.0,  'goldenrod',   'star',          'Star'),
    (9.0,  'slategray',   'pentagon',      'Pentagon'),
    (10.0, 'firebrick',   'hexagon',       'Hexagon')
  ) AS t(y, color, marker, label)
);
```

## 气泡图

用 `sizes` 把第三个维度编码进点的面积 —— 给的是每个点的半径（像素）。`sizes` 按顺序与 `data` 对应，并覆盖
`size`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bubble plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'data': [{"x": 1.0, "y": 3.0}, {"x": 2.5, "y": 6.5}, {"x": 4.0, "y": 4.0}, {"x": 5.5, "y": 8.0}, {"x": 7.0, "y": 5.5}, {"x": 8.5, "y": 9.0}],
    'sizes': [5.0, 14.0, 9.0, 18.0, 11.0, 7.0]
  }]
})) AS chart;
```

## 逐点颜色

`colors` 按顺序给每个点一个颜色，超出列表长度就回退到 `color`。当数据本身已经带分组标签、你不想为每个分组拆
成一个 series 时，就用它。

`colors` **不会**更新图例；要带标签的图例就用一个分组一个 series（见下面「多系列」）。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'size': 6,
    'data': pts,
    'colors': colors
  }]
})) AS chart
FROM (
  SELECT
    array_agg({'x': x, 'y': y} ORDER BY x) AS pts,
    array_agg(CASE "group"
      WHEN 'Group_A' THEN '#4c72b0'
      WHEN 'Group_B' THEN '#c44e52'
      ELSE '#55a868'
    END ORDER BY x) AS colors
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## marker 不透明度与描边

`marker_opacity` 控制填充的 alpha，`marker_stroke_width` 用填充色描一圈边。两者合起来给出三种模式，数据密时
差别最大：

| 模式 | 设置 | 适用 |
| --- | --- | --- |
| **实心**（默认） | 两个字段都不给 | 点少、簇之间分得开 |
| **半透明** | `marker_opacity` 小于 `1`，加描边 | 密处颜色叠加变深，同时每个点还看得清 |
| **空心** | `marker_opacity: 0`，加描边 | 点非常多；轮廓互相叠加显出密度，而不会糊成一团 |

### 半透明 marker

contour 数据集里的 600 个点挤在同一块区域。这个密度下实心 marker 会并成一块不透明的色块；把不透明度降到
`0.25`，密处的重叠就能透出来，再配一圈细描边，每个点仍然可辨。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Semi-transparent markers',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'marker_opacity': 0.25,
    'marker_stroke_width': 0.7,
    'data': array_agg({'x': x, 'y': y})
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/contour.tsv');
```

### 空心圆

`marker_opacity: 0` 只画轮廓。点相互重叠时，轮廓的累积正好显出密度落在哪，而不是糊成一坨黑。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Hollow markers',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 4,
    'marker_opacity': 0,
    'marker_stroke_width': 1,
    'data': array_agg({'x': x, 'y': y})
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/contour.tsv');
```

## 点标签

`labels` 给每个点挂一段文字，按顺序与 `data` 对应。**空字符串表示这个点不标** —— 这就是「只标关键点、
又不把 series 拆开」的写法。

`label_style` 决定摆放方式，与[火山图](../statistics/volcano.md)、[曼哈顿图](../statistics/manhattan.md)共用：

| 取值 | 摆放方式 |
| --- | --- |
| `"exact"` | 就压在点上，不做避让（可能重叠） |
| `"nudge"`（默认） | 按 x 排序后纵向错开 |
| `{"offset_x": …, "offset_y": …}` | 固定像素偏移，并画一条引线 |
| `"repel"` | 力导向（ggrepel / adjustText 风格）：标签互相推开、也推离数据点，并画细引线 |

标签密集、挤在一起时用 `"repel"`：下面这张图只标最显著的 6 个基因，它们夹在几千个点里仍然读得出来。

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT gene, log2fc, -log10(pvalue) AS neglog10p,
         row_number() OVER (ORDER BY pvalue) AS rk
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Point labels (repel)',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p)'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 3,
    'marker_opacity': 0.5,
    'data': (SELECT array_agg({'x': log2fc, 'y': neglog10p} ORDER BY gene) FROM g),
    'labels': (SELECT array_agg(CASE WHEN rk <= 6 THEN gene ELSE '' END ORDER BY gene) FROM g),
    'label_style': 'repel'
  }]
})) AS chart;
```

## 多系列

`series` 里一个 series 一个对象，各自带 `color` 与 `legend`，就会画在同一对坐标轴上。只要有任意一个 series
带了 `legend`，图例就会出现。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'scatter', 'size': 5, 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg({'x': x, 'y': y} ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | **必填。** 点，对象 `{x, y, x_err?, y_err?}`。 |
| `size` | number | 统一的点半径（默认 3）。 |
| `sizes` | number[] | 逐点半径（气泡图）；会覆盖 `size`。 |
| `colors` | string[] | 逐点颜色；超出列表长度就回退到 `color`。 |
| `marker` | string | `"circle"`（默认）· `"square"` · `"triangle"` · `"triangle_down"` · `"diamond"` · `"cross"` · `"plus"` · `"star"` · `"pentagon"` · `"hexagon"`。 |
| `marker_opacity` | number | 填充 alpha：`0` 空心，`1` 实心。 |
| `marker_stroke_width` | number | 描边线宽，颜色取填充色。 |
| `labels` | string[] | 逐点文字标签，与 `data` 对齐；空字符串表示该点不标。 |
| `label_style` | string \| object | `"nudge"`（默认）· `"exact"` · `"repel"` · `{"offset_x": …, "offset_y": …}`。 |
| `trend` | `"linear"` \| `"loess"` \| object | 叠一条最小二乘线或 LOESS 平滑曲线；对象形式可加 `type`、`span`（LOESS）、`color`、`width`、`equation`、`correlation`。 |
| `band` | `{lower, upper}` | 与点的 x 位置对齐的阴影带。 |
| `group_name` | string | 交互式 SVG 输出里的分组名（不进图例）。 |

`color`、`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)；
`x_err` / `y_err` 见[点类型](../../reference/series.md#点)。

## 说明

- **`data` 不能为空。** `sizes` / `colors` 比数据长没关系；比数据短就回退到 `size` / `color`。
- 逐点颜色**不会**更新图例 —— 要带标签的图例就一个分组一个 series。
- `x_err` / `y_err` 给一个数是对称的，给 `{"neg": …, "pos": …}` 对象是不对称的。
- `band.lower` 与 `band.upper` 必须各自与 `data` 等长、且顺序一致。
- `labels` 可以比 `data` **短**（后面的点不标），但不能更长 —— 多出来的标签会报错，而不是被静默丢掉。
- `equation` / `correlation` 只对线性拟合有效；`"type": "loess"` 时会被忽略。

## 另见

- [kuva — 散点图](https://psy-fer.github.io/kuva/plots/scatter.html) —— 绘图库自己的图型参考。
- [折线图](./line.md) —— 有序、要连起来的数据。
- [二维直方图](../distributions/histogram2d.md) 与 [六边形分箱图](../distributions/hexbin.md) —— 点很多的时候。
