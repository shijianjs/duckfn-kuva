---
title: 砖墙图
sidebar_position: 1
description: 序列画成一行行彩色砖块 —— 一个字符一块砖。
---

# 砖墙图

砖墙图把一条序列画成一行彩色矩形，一个字符一块砖，并用一张配色表把字符映射到颜色。它是为 DNA/RNA 序列视图与
串联重复结构而生的 —— 那类数据里，**图案本身就是数据**。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'DNA repeat region',
  'series': [{
    'type': 'brick',
    'sequences': [
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCAT',
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCATCAT'
    ],
    'names': ['read_1', 'read_2'],
    'template': 'dna',
    'x_offset': 18
  }]
})) AS chart;
```

`x_offset` 把一段共同的侧翼前缀藏起来，让关心的区域从零开始 —— 这里丢掉 18 个碱基的前缀，`CAT` 重复就在两条 read
之间对齐了。不给它的话，有意思的部分会跑到右边，两行看起来像是互相矛盾。

## 逐行偏移

read 很少从同一个位置开始。`x_offsets` 让每一行独立平移，`null` 表示回退到全局的 `x_offset`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-row offsets',
  'series': [{
    'type': 'brick',
    'sequences': [
      'ACGTACGTACGTACGTACGTACGTAAACCCTTTGGGAAA',
      'ACGTACGTACGTACGTACGTAAACCCTTTGGGAAA',
      'ACGTACGTACGTACGTACGTACGTACGTAAACCCTTTGGGAAA'
    ],
    'names': ['read_1', 'read_2', 'read_3'],
    'template': 'dna',
    'x_offset': 12,
    'x_offsets': [20, 16, null]
  }]
})) AS chart;
```

## 自定义配色表

`template` 也可以收一张「字符 → 颜色」的表，于是任何单字符字母表都能用：二级结构、重复单元类别、染色质状态。
`show_values` 把字符印在每块砖里 —— 只有砖够宽、读得出来时才值得开。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Protein secondary structure',
  'series': [{
    'type': 'brick',
    'sequences': ['CCCHHHHHHHHHHCCCCEEEEEECCC', 'CHHHHHHHHCCEEEEECCCHHHHCC'],
    'names': ['prot_1', 'prot_2'],
    'template': {'H': 'steelblue', 'E': 'firebrick', 'C': '#aaaaaa', 'T': 'seagreen'},
    'show_values': true
  }]
})) AS chart;
```

## STRIGAR 模式

处理串联重复时，`strigars` 收 `[motif 串, strigar 串]` 对。motif 串把局部字母映射到 k-mer
（`"CAT:A,C:B"`），strigar 则是这些字母的游程编码（`"10A1B4A"`）。kuva 会把 k-mer 旋转到规范形式、按出现频率分配
全局字母，并画出宽度正比于各 motif 长度的砖块。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Tandem repeats',
  'series': [{
    'type': 'brick',
    'names': ['read_1', 'read_2', 'read_3'],
    'strigars': [
      ['CAT:A,C:B,T:C',   '10A1B4A1C1A'],
      ['CAT:A,T:B',       '14A1B1A'],
      ['CAT:A,C:B,GGT:C', '10A1B8A1C5A']
    ],
    'template': {'A': '#4c72b0', 'B': '#dd8452', 'C': '#55a868',
                 'G': '#e6a532', 'T': '#c44e52'},
    'consensus_row': 0,
    'mark_primary': true
  }]
})) AS chart;
```

`consensus_row` 把规范旋转锁定到某一行 —— 第 0 行是参考序列时正是你想要的：图例随后显示的是**参考**的重复单元
拼法，而不是恰好最常见的那个旋转。

## 逐段注记

`notations` 每行给一项：任意字符串就打开该行砖块上方的 `(kmer)count` 标签，`null` 表示不开。字符串内容会被忽略 ——
标签是根据展开后的 strigar 的游程结构生成的，空位砖块会被跳过。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Annotated runs',
  'series': [{
    'type': 'brick',
    'names': ['consensus', 'read_1', 'read_2'],
    'strigars': [
      ['CAG:A,CAA:B,CCG:C', '6A1B2A1C10A'],
      ['CAG:A,CCG:B',       '8A1B10A'],
      ['CAG:A',             '20A']
    ],
    'template': {'A': '#4c72b0', 'B': '#dd8452', 'C': '#55a868'},
    'consensus_row': 0,
    'notations': ['', null, null]
  }]
})) AS chart;
```

相邻段的标签会重叠时，它们会错开到最多四层，画布也会自动补出需要的那点上边距。

## 锚点

| `anchor` | 各行对齐在 |
| --- | --- |
| `"left"` | 前沿（**默认**） |
| `"right"` | 后沿 —— 用于末尾落在同一个参考位置上的 read |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Right-anchored',
  'series': [{
    'type': 'brick',
    'sequences': [
      'ACGTACGTACGTAAACCCTTTGGGAAA',
      'ACGTACGTAAACCCTTTGGGAAA',
      'ACGTACGTACGTACGTAAACCCTTTGGGAAA'
    ],
    'names': ['read_1', 'read_2', 'read_3'],
    'template': 'dna',
    'anchor': 'right'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `sequences` | string[] | 每行一个字符串；一个字符一块砖。 |
| `names` | string[] | 行名；**第 0 行画在最上面**。 |
| `strigars` | `[string, string][]` | `[motif, strigar]` 对 —— 给了它就取代 `sequences`。 |
| `template` | string \| object | `"dna"` · `"rna"` · 一张 `{字符: CSS 颜色}` 表。 |
| `x_offset` | number | 所有行共用的 x 平移量。 |
| `x_offsets` | (number \| null)[] | 逐行平移；`null` 回退到 `x_offset`。 |
| `x_origin` | number | 映射到 x = 0 的那个坐标，叠加在各行偏移之上。 |
| `show_values` | boolean | 把每个字符印在它的砖里。 |
| `strigar_palette` | string[] | strigar 字母的配色，按顺序。 |
| `anchor` | string | `"left"`（默认）或 `"right"`。 |
| `mark_primary` | boolean | 给主导 motif 的图例文字后面加一个 `*`。 |
| `consensus_row` | integer | 把 k-mer 的规范旋转锁定到这一行。 |
| `notations` | (string \| null)[] | 该行砖块上方的 `(kmer)count` 标签。 |
| `row_height` | number | 行高（像素）。 |

## 说明

- **`sequences` 与 `strigars` 二选一** —— 两者互斥，同时给时以 `strigars` 为准。
- `template` 没有一个「合理」的默认值：请给 `"dna"`、`"rna"`，或者一张表。**表里没有的字符没有颜色**，而且
  strigar 模式下的全局字母（A、B、C…）也是从这张表里取色的，所以要么给足字符，要么给 `strigar_palette`。
- `names` 必须与生效的那一份数据行数一致 —— strigar 模式下按 `strigars` 算，否则按 `sequences` 算。
- **第 0 行在最上面**，共识序列因此天然读起来像一行表头。
- strigar 模式下每一段都必须带次数：写 `"10A"`，不能写 `"A"`。库会直接解析那个数字，缺了就报错，而不是默认成 1。
- `consensus_row` 只在 strigar 模式有意义；不给时以「所有 read 里最常见的那个旋转」为准。
- 没有 `start_positions` 字段：它等价于取负值的 `x_offsets`，直接把起始坐标取负传进来即可。
- **重复序列两侧的侧翼 DNA 没有开放。** 库里有 `flanked_strigars` 那个构造器用于
  `(左侧翼, motifs, strigar, 右侧翼)` 行；在这里侧翼只能用额外的 `@` 段写进 motif 串，而渲染器不会给它们上色，
  所以一个带侧翼的位点只能拆成两张图，或者干脆不画侧翼。

## 另见

- [kuva — 砖墙图](https://psy-fer.github.io/kuva/plots/brick.html) —— 绘图库自己的图型参考。
- [共线性图](./synteny.md) —— 基因组之间的结构比较。
