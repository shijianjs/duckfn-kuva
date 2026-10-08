---
title: 马赛克图
sidebar_position: 11
description: 类目 × 类目的表画成方块，宽高正比于数值。
---

# 马赛克图

马赛克图把一张列 × 行的表画成方块，宽与高正比于计数。它同时展示每列的大小与列内的构成 —— 是堆叠柱在分类
数据上的「面积诚实」版本。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'mosaic',
    'cells': list({'col': region, 'row': outcome, 'value': count}),
    'values': true,
    'percents': true,
    'legend': 'outcome'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `cells` | cell[] | **必填。** 每个 `列 × 行` 一项：`{col, row, value}`。缺失的组合按 0，重复的会求和。 |
| `col_order` | string[] | 列的顺序（默认按 `cells` 里首次出现的顺序）。 |
| `row_order` | string[] | 行的顺序。 |
| `group_colors` | string[] | 逐行配色（按 `row_order` 的位置）。 |
| `gap` | number | 方块之间的留白。 |
| `percents` | boolean | 在每个方块里写百分比。 |
| `values` | boolean | 在每个方块里写数值。 |
| `min_label_height` | number | 低于这个高度就不画行标签。 |
| `min_label_width` | number | 低于这个宽度就不画列标签。 |
| `normalize` | boolean | 每列归一到满高（关掉则各列按自己的总量）。 |
| `legend` | string | 图例标题。 |

## 说明

- **`cells` 不能为空。** 给原始计数即可；版面比例由渲染器算。
- 重复的 `列 × 行` 会求和，所以长表可以直接喂进来。

## 另见

- [kuva — 马赛克图](https://psy-fer.github.io/kuva/plots/mosaic.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) —— 堆叠柱。
- [韦恩图](./venn.md) · [UpSet 图](./upset.md) —— 同一思路的集合交叠视图。
