---
title: 共线性图
sidebar_position: 2
description: 序列之间保守区块，画成正向或交叉的带子。
---

# 共线性图

共线性图比较两条或多条序列之间的保守区域：每条序列一根横条，每个共线区块是一条连接对应区间的带子。正向区块用两侧
平行的带子，倒位区块用交叉的「蝴蝶结」带子 —— 于是一次重排变成一个形状，而不是一个数字。

```sql {"type":"duckfn","show":"svg"}
WITH s AS (SELECT name, length FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_seqs.tsv')),
idx AS (SELECT name, (row_number() OVER (ORDER BY name)) - 1 AS i FROM s),
b AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_blocks.tsv'))
SELECT kuva_render(to_json({
  'title': 'Sequence alignment blocks',
  'series': [{
    'type': 'synteny',
    'sequences': (SELECT list({'label': name, 'length': length} ORDER BY name) FROM s),
    'blocks': (SELECT list({'seq1': i1, 'start1': start1, 'end1': end1,
                            'seq2': i2, 'start2': start2, 'end2': end2,
                            'strand': CASE WHEN strand = '-' THEN 'reverse' ELSE 'forward' END}
                           ORDER BY start1, seq2, start2)
               FROM (SELECT b.*, a.i AS i1, c.i AS i2
                     FROM b
                     JOIN idx a ON a.name = b.seq1
                     JOIN idx c ON c.name = b.seq2)),
    'shared_scale': true
  }]
})) AS chart;
```

`seq1` / `seq2` 是**下标**、不是名字，所以一份以名字为键的区块表需要一个 join 把名字换成位置 —— 而两个列表必须以
同一种顺序排列，这就是为什么序列列表与下标 CTE 都按 `name` 排序。

## 倒位

`strand: "reverse"` 把一个区块标成反向互补：带子把源区间右缘接到目标区间左缘，画出来就是交叉的形状。共线区块与一次
重排之间，差别只有这一个字段 —— 而这正是它值得画出来的原因。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Forward and inverted blocks',
  'series': [{
    'type': 'synteny',
    'sequences': [
      {'label': 'Seq A', 'length': 1000000},
      {'label': 'Seq B', 'length': 1000000}
    ],
    'blocks': [
      {'seq1': 0, 'start1': 0,       'end1': 200000, 'seq2': 1, 'start2': 0,       'end2': 200000},
      {'seq1': 0, 'start1': 250000,  'end1': 500000, 'seq2': 1, 'start2': 250000,  'end2': 500000,
       'strand': 'reverse'},
      {'seq1': 0, 'start1': 600000,  'end1': 900000, 'seq2': 1, 'start2': 600000,  'end2': 900000}
    ]
  }]
})) AS chart;
```

## 多条序列

超过两条序列就得到一摞两两比较。区块可以连接**任意**两个下标，所以连接第 0 与第 2 条序列的区块会横跨整幅图的高度 ——
三方共同保守的区域就该这样画。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Three-way comparison',
  'series': [{
    'type': 'synteny',
    'sequences': [
      {'label': 'Genome A', 'length': 500000},
      {'label': 'Genome B', 'length': 480000},
      {'label': 'Genome C', 'length': 450000}
    ],
    'blocks': [
      {'seq1': 0, 'start1': 0,       'end1': 100000, 'seq2': 1, 'start2': 0,       'end2': 95000},
      {'seq1': 0, 'start1': 150000,  'end1': 300000, 'seq2': 1, 'start2': 140000,  'end2': 280000},
      {'seq1': 1, 'start1': 0,       'end1': 100000, 'seq2': 2, 'start2': 5000,    'end2': 105000},
      {'seq1': 1, 'start1': 200000,  'end1': 350000, 'seq2': 2, 'start2': 190000,  'end2': 340000,
       'strand': 'reverse'}
    ],
    'shared_scale': true
  }]
})) AS chart;
```

## 颜色与图例

`sequence_colors` 给条上色；区块自己的 `color` 会盖过它本来会从源序列继承的颜色。给 `legend` 一个标题之后，每条序列
成为一条图例。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured blocks',
  'series': [{
    'type': 'synteny',
    'sequences': [
      {'label': 'Seq 1', 'length': 500000},
      {'label': 'Seq 2', 'length': 500000}
    ],
    'sequence_colors': ['#4393c3', '#d6604d'],
    'blocks': [
      {'seq1': 0, 'start1': 0,      'end1': 150000, 'seq2': 1, 'start2': 0,      'end2': 150000,
       'color': '#2ca02c'},
      {'seq1': 0, 'start1': 200000, 'end1': 350000, 'seq2': 1, 'start2': 200000, 'end2': 340000,
       'color': '#9467bd'},
      {'seq1': 0, 'start1': 380000, 'end1': 480000, 'seq2': 1, 'start2': 370000, 'end2': 470000,
       'color': '#ff7f0e', 'strand': 'reverse'}
    ],
    'legend': 'blocks'
  }]
})) AS chart;
```

## 共享标尺

默认每根条各自占满整幅宽度，这能把每条序列的细节放到最大，同时**把它们之间的长度差藏起来**。`shared_scale` 改用
一把标尺：每根条的宽度等于 `长度 / 最大长度`，于是 400 kb 的序列挨着 1 Mb 的，看起来真的只有 40 % 长。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Unequal lengths, one ruler',
  'series': [{
    'type': 'synteny',
    'sequences': [
      {'label': 'Long',  'length': 1000000},
      {'label': 'Short', 'length': 400000}
    ],
    'shared_scale': true,
    'blocks': [
      {'seq1': 0, 'start1': 0,      'end1': 300000, 'seq2': 1, 'start2': 0,      'end2': 300000},
      {'seq1': 0, 'start1': 350000, 'end1': 700000, 'seq2': 1, 'start2': 50000,  'end2': 380000}
    ]
  }]
})) AS chart;
```

## 样式

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `bar_height` | `18` | 序列条的高度（像素） |
| `block_opacity` | `0.65` | 带子的不透明度 |
| `shared_scale` | `false` | 所有序列共用一把标尺 |
| `legend` | — | 图例标题；每条序列一条 |

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `sequences` | sequence[] | **必填。** 每条序列 `{label, length, color?}`。 |
| `sequence_colors` | string[] | 条的颜色，与 `sequences` 平行。 |
| `blocks` | block[] | `{seq1, start1, end1, seq2, start2, end2, strand?, color?}`。 |
| `strand` | string | `"forward"`（默认）或 `"reverse"`（交叉带子）。 |
| `bar_height` | number | 条的高度（像素，默认 `18`）。 |
| `block_opacity` | number | 带子的不透明度（默认 `0.65`）。 |
| `shared_scale` | boolean | 用一把共享标尺（默认关）。 |

## 说明

- **`sequences` 不能为空**，且每个区块的 `seq1` / `seq2` 都必须是它的合法下标。
- 下标是 **0 起、按位置** 的，所以 `sequences` 的顺序就是区块引用的对象 —— 在 SQL 里用一个共同的 `ORDER BY`
  把两边对齐。
- `end` 必须大于 `start`；`strand: "reverse"` 时交叉带子仍然从较小的坐标开始，那个**交叉**就是方向本身。
- 带子画在条之前，所以区块即使与自己的序列条重叠，看起来也是干净的。
- 不给 `shared_scale` 时，长度相差很大的两条序列宽度看起来差不多 —— 这是默认值在**悄悄**替你做的一个比较，
  要紧的时候要标出来。

## 另见

- [kuva — 共线性图](https://psy-fer.github.io/kuva/plots/synteny.html) —— 绘图库自己的图型参考。
- [系统发育树](../hierarchical/phylo.md) —— 同一批序列之间的亲缘关系。
- [砖墙图](./brick.md) —— 逐碱基的细节，而不是区块级。
