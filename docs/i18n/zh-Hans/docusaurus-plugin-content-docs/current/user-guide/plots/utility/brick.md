---
title: 砖墙图
sidebar_position: 1
description: 把一段序列画成一排彩色砖块 —— motif、比对与 STRIGAR 游程。
---

# 砖墙图

砖墙图把一段序列画成一排彩色砖块，一个字符一格 —— 展示 motif、比对后的 reads，或游程编码的 STRIGAR 时
都用它。颜色来自一张配色模板。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'brick',
    'sequences': ['ACGTACGTACGT', 'ACGTACGTTCGT', 'ACGTTCGTACGT', 'ACGTACGTACGA'],
    'names': ['read_1', 'read_2', 'read_3', 'read_4'],
    'template': 'dna',
    'consensus_row': 0,
    'row_height': 18
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `sequences` | string[] | 逐行一条序列，一个字符一格。给了 `strigars` 就不参与渲染。 |
| `names` | string[] | 行的名字；长度必须与真正画出来的行数一致。 |
| `strigars` | `[string, string][]` | `[motif, strigar]` 对；展开后的游程**代替** `sequences`。 |
| `template` | string \| object | `"dna"` · `"rna"`，或 `{字符: 颜色}` 映射。默认 `dna`。 |
| `x_offset` | number | 所有行共用的 x 起点。 |
| `x_offsets` | number[] | 逐行的 x 起点（每项可为 null）。 |
| `x_origin` | number | 坐标原点。 |
| `show_values` | boolean | 标出每个字符。 |
| `strigar_palette` | string[] | STRIGAR 结构域的配色。 |
| `anchor` | string | `"left"` 或 `"right"`。 |
| `mark_primary` | boolean | 给「主」那一条加个标记。 |
| `consensus_row` | integer | 共识序列是哪一行（0 起）。 |
| `notations` | (string \| null)[] | 逐行的注记文字。 |
| `row_height` | number | 行高（像素）。 |

## 说明

- **给 `sequences` 或 `strigars`，不能都不给。** 走 `strigars` 时 `sequences` 完全不画。
- **每个字符都必须被配色模板覆盖** —— 没覆盖的字符会报错（否则会 panic）。
- STRIGAR 游程里**每个**字母都要带重复次数，如 `"10A|30@|2A"`；光秃秃的字母会报错。

## 另见

- [kuva — 砖墙图](https://psy-fer.github.io/kuva/plots/brick.html) —— 绘图库自己的图型参考。
- [共线性图](./synteny.md) —— 整条序列之间的区块。
