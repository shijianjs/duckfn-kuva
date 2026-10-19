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

## 起始坐标

`start_positions` 说的是同一件事的另一面：直接给每行的**参考起始坐标**，kuva 会把这一行平移，让那个坐标落在共享
的轴上。它实际上就是取负值的 `x_offsets`，但写起来更像数据本身 —— 还能与 `x_origin` 配合，把某个有生物学意义的
位置（重复区的起点）钉到 x = 0。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Aligned by reference start',
  'x_axis': {'name': 'reference position', 'tick_format': 'integer'},
  'series': [{
    'type': 'brick',
    'names': ['read_1', 'read_2', 'read_3'],
    'strigars': [
      ['CAG:A', '8A'],
      ['CAG:A', '12A'],
      ['CAG:A', '10A']
    ],
    'start_positions': [0, 19, 40],
    'row_height': 20
  }]
})) AS chart;
```

## 自定义配色表

`template` 也可以收一个 `{"colors": {…}}` 表（字符 → 颜色），于是任何单字符字母表都能用：二级结构、重复单元类别、染色质状态。
`show_values` 把字符印在每块砖里 —— 只有砖够宽、读得出来时才值得开。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Protein secondary structure',
  'series': [{
    'type': 'brick',
    'sequences': ['CCCHHHHHHHHHHCCCCEEEEEECCC', 'CHHHHHHHHCCEEEEECCCHHHHCC'],
    'names': ['prot_1', 'prot_2'],
    'template': {'colors': {'H': 'steelblue', 'E': 'firebrick', 'C': '#aaaaaa', 'T': 'seagreen'}},
    'show_values': true
  }]
})) AS chart;
```

## STRIGAR 模式

处理串联重复时，`strigars` 收 `[motif 串, strigar 串]` 对。motif 串把局部字母映射到 k-mer
（`"CAT:A,C:B"`），strigar 则是这些字母的游程编码（`"10A1B4A"`）。kuva 会把 k-mer 旋转到规范形式、按出现频率分配
全局字母，并画出宽度正比于各 motif 长度的砖块。

游程**首尾相接、没有分隔符**：`<次数><字母><次数><字母>…`。字母是**双射 base-26**（`A`…`Z`，然后是
`AA`、`AB`…），所以一行可以引用超过 26 种 motif，一个字母也可能不止一个字符。

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
    'template': {'colors': {'A': '#4c72b0', 'B': '#dd8452', 'C': '#55a868',
                            'G': '#e6a532', 'T': '#c44e52'}},
    'consensus_row': 0,
    'mark_primary': true
  }]
})) AS chart;
```

`consensus_row` 把规范旋转锁定到某一行 —— 第 0 行是参考序列时正是你想要的：图例随后显示的是**参考**的重复单元
拼法，而不是恰好最常见的那个旋转。

> **kuva 0.6 的破坏性变更：** 旧格式用 `|` 分段、用 `@` 表示空位（`"10A | 2@ | 2A"`）。这两个字符现在会被
> **直接拒绝** —— kuva 宁可丢掉这一行，也不画出一张看起来合理、实则错位的图；本扩展会把它变成一条点名到行
> 的错误。旧串请用当前的 bladerunner 重新生成，或改写成没有分隔符的 `<次数><字母>`。

## 带侧翼的 STRIGAR

真实的 read 在重复区两侧都带着侧翼 DNA。`flanked_strigars` 直接收
`[左侧翼, motif, strigar, 右侧翼]`：两侧给原始 DNA 字符串（一个字符一块砖，用标准 A/C/G/T 配色），中间那一对
`(motif, strigar)` 与上面完全一样。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Flanked STR locus',
  'series': [{
    'type': 'brick',
    'names': ['consensus', 'read_1', 'read_2'],
    'flanked_strigars': [
      ['ACGTACGT', 'CAG:A,CAA:B', '6A1B8A',  'TGCATGCA'],
      ['ACGTACGT', 'CAG:A',       '16A',     'TGCATGCA'],
      ['ACGTACGT', 'CAG:A',       '20A',     'TGCA']
    ],
    'consensus_row': 0,
    'mark_primary': true,
    'row_height': 20
  }]
})) AS chart;
```

## 逐段注记

`notations` 每行给一项：任意字符串就打开该行砖块上方的 `(kmer)count` 标签，`null` 表示不开。字符串内容会被忽略 ——
标签是根据展开后的 strigar 的游程结构生成的。

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
    'template': {'colors': {'A': '#4c72b0', 'B': '#dd8452', 'C': '#55a868'}},
    'consensus_row': 0,
    'notations': ['', null, null]
  }]
})) AS chart;
```

相邻段的标签会重叠时，它们会错开到最多四层，画布也会自动补出需要的那点上边距。

## CIGAR 条

`cigars` 在每行上方画一条细带，按该 read 的 CIGAR 操作上色 —— 一眼就能看出哪些碱基有参考支持、哪些是插入
或软剪切。它画在**读段坐标**里，与砖块同一个坐标系，所以每个操作总与它下面的碱基对齐。

CIGAR 必须覆盖**这一行画出的全部碱基（含两侧翼）** —— 也就是 `row_base_len` 返回的长度。读段长度对不上的
CIGAR 会被**拒收而不是照画**：短几个碱基的条会让后面每个操作都错位，看起来却仍然像一条合法注记。CIGAR 应该
由产生这一行的工具给出；从基因组坐标反推会丢掉 span 的边界。

相邻且 CIGAR 相同的行会合并成一条（`cigar_dedup`，默认开），左侧沟槽里用 `xN` 标出这一段的计数
（`cigar_run_bracket`，默认开）。`sort: "cigar"` 把相同 CIGAR 归到一起，去重后条数最少。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'CIGAR bars',
  'series': [{
    'type': 'brick',
    'names': ['consensus', 'read_1', 'read_2', 'read_3'],
    'flanked_strigars': [
      ['ACGTACGT', 'CAG:A,CAA:B', '6A1B8A', 'TGCATGCA'],
      ['ACGTACGT', 'CAG:A,CAA:B', '6A1B8A', 'TGCATGCA'],
      ['ACGTACGT', 'CAG:A',       '16A',    'TGCATGCA'],
      ['ACGTACGT', 'CAG:A',       '20A',    'TGCA']
    ],
    'cigars': ['55M6S', '55M6S', '58M6I', '66M6S'],
    'cigar_dedup': true,
    'cigar_run_bracket': true,
    'cigar_colors': {'insertion': '#e9c46a', 'soft_clip': '#8ecae6'},
    'sort': 'cigar',
    'row_height': 22
  }]
})) AS chart;
```

`cigar_colors` 按操作名覆盖配色：`"match"` · `"mismatch"` · `"insertion"` · `"deletion"` ·
`"soft_clip"`。删除（`D` / `N`）不消耗读段碱基，所以画成一个零宽的尖角，而不是在行里开一个缺口。

## 参考线

`marker_lines` 在所有行上画竖直参考线 —— 引物边界、变异位点、关注的位置。它们用砖墙自己的参考坐标，
所以 `x_offset` / `x_origin` 会一并生效。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Primer boundary',
  'series': [{
    'type': 'brick',
    'names': ['read_1', 'read_2', 'read_3'],
    'sequences': [
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCAT',
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCATCAT',
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCATCATCAT'
    ],
    'template': 'dna',
    'x_offset': 18,
    'marker_lines': [
      {'value': 6, 'label': 'primer', 'color': 'crimson', 'dasharray': '4 2'}
    ]
  }]
})) AS chart;
```

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
| `flanked_strigars` | `[string, string, string, string][]` | `[左侧翼, motif, strigar, 右侧翼]` —— 取代 `strigars`。 |
| `template` | string \| object | `"dna"` · `"rna"` · 一个 `{"colors": {字符: CSS 颜色}}` 对象。 |
| `x_offset` | number | 所有行共用的 x 平移量。 |
| `x_offsets` | (number \| null)[] | 逐行平移；`null` 回退到 `x_offset`。 |
| `start_positions` | number[] | 逐行的参考起始坐标；不能与 `x_offsets` 同时给。 |
| `x_origin` | number | 映射到 x = 0 的那个坐标，叠加在各行偏移之上。 |
| `show_values` | boolean | 把每个字符印在它的砖里。 |
| `strigar_palette` | string[] | strigar 字母的配色，按顺序。 |
| `anchor` | string | `"left"`（默认）或 `"right"`。 |
| `mark_primary` | boolean | 给主导 motif 的图例文字后面加一个 `*`。 |
| `consensus_row` | integer | 把 k-mer 的规范旋转锁定到这一行。 |
| `notations` | (string \| null)[] | 该行砖块上方的 `(kmer)count` 标签。 |
| `row_height` | number | 行高（像素）。 |
| `cigars` | (string \| null)[] | 逐行 CIGAR；`null` 表示该行不画。必须覆盖该行画出的全部碱基（含两侧翼）。 |
| `cigar_dedup` | boolean | 相邻且 CIGAR 相同的行合并成一条（默认 `true`）。 |
| `cigar_run_bracket` | boolean | 给合并后的每一段标 `xN` 计数（默认 `true`）。 |
| `cigar_colors` | object | 操作 → 颜色，键是 `match` · `mismatch` · `insertion` · `deletion` · `soft_clip`。 |
| `sort` | string \| object | `"none"`（默认）· `"cigar"` · `{"custom": [行序]}`。 |
| `marker_lines` | object[] | 竖直参考线 `{value, label?, color?, stroke_width?, dasharray?}`；只接受竖直方向。 |

## 说明

- **`sequences`、`strigars` 与 `flanked_strigars` 是同一份数据的三种写法** —— 只能给一种。
- `template` 没有一个「合理」的默认值：请给 `"dna"`、`"rna"`，或者一个 `{"colors": …}` 表，**表里没有的字符没有颜色**。
  （strigar / flanked-strigar 模式下 strigar 那部分的配色由 motif 自动生成，`template` 只对 `sequences` 生效。）
- `names` 必须与生效的那一份数据行数一致 —— 按 `strigars` / `flanked_strigars` 或 `sequences` 算。
- **第 0 行在最上面**，共识序列因此天然读起来像一行表头。
- strigar 模式下每一段都必须带次数：写 `"10A"`，不能写 `"A"`。kuva 会把没有次数的片段静默丢掉，画出来比要求的
  短一截 —— 所以扩展侧直接报错。段与段之间没有分隔符：`"10A2B"`。0.6 之前的 `|` / `@` 写法会被**拒绝**
  （见「STRIGAR 模式」）。
- `cigars` 必须一行一项，且每条 CIGAR 的读段长度（`M` / `I` / `S` / `=` / `X`）必须等于该行画出的碱基
  （左侧翼 + STRIGAR + 右侧翼）。这两项检查在这里都是错误，而不是「静默画错」或「静默丢掉」。
- `marker_lines` 只支持竖直方向：不写 `orientation`，或写 `"vertical"`。水平线会报错 —— 砖墙的 y 轴是行号。
- `consensus_row` 锁的是这一行的规范旋转，所以它必须在 strigar 被解析**之前**设好 —— 扩展侧已经替你处理好顺序；
  不给时以「所有 read 里最常见的那个旋转」为准。
- `x_offsets` 与 `start_positions` 都在设逐行偏移，所以不能同时给（后者是前者取负值之后的说法，以参考坐标表达）。

## 另见

- [kuva — 砖墙图](https://psy-fer.github.io/kuva/plots/brick.html) —— 绘图库自己的图型参考。
- [共线性图](./synteny.md) —— 基因组之间的结构比较。
