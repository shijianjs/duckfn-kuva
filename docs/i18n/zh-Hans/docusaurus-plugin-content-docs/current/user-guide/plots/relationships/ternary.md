---
title: 三元图
sidebar_position: 9
description: 三个分量之和为 1 的点，画在三角形里。
---

# 三元图

三元图用三个和为 1 的比例来定位点 —— 组成类数据的经典形式（土壤成分、合金配比、时间分配）。每个角是一个
分量。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'ternary',
    'points': pts,
    'corner_labels': ['A', 'B', 'C'],
    'normalize': true,
    'marker_size': 6,
    'grid_lines': 4,
    'show_legend': true,
    'show_percentages': true,
    'marker_opacity': 0.8
  }]
})) AS chart
FROM (
  SELECT list({'a': a, 'b': b, 'c': c, 'group': "group"}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/ternary.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每个点一项：`{a, b, c, group?}`。 |
| `corner_labels` | string[] | 三个角的标签，顺序是**上、左下、右下**。 |
| `normalize` | boolean | 把每个点归一到三者和为 1。 |
| `marker_size` | number | 点半径。 |
| `grid_lines` | integer | 网格分几份。 |
| `show_grid` | boolean | 画网格。 |
| `show_percentages` | boolean | 在网格线上标百分比。 |
| `show_legend` | boolean | 显示图例（文字取自各点的 `group`）。 |
| `marker_opacity` | number | 点的不透明度。 |
| `marker_stroke_width` | number | 点的轮廓线宽。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每个点一条提示。 |

## 说明

- **`points` 不能为空。** `normalize` 为 `false` 时，三个分量必须已经和为 1。
- `corner_labels` 必须**正好 3 个**；数量不对会报错。

## 另见

- [kuva — 三元图](https://psy-fer.github.io/kuva/plots/ternary.html) —— 绘图库自己的图型参考。
- [极坐标图](./polar.md) · [雷达图](../categorical/radar.md) —— 其它非笛卡尔布局。
