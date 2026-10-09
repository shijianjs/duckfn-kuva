---
title: 系统发育树
sidebar_position: 6
description: 从 Newick、边表、距离矩阵或 linkage 输出画出的树。
---

# 系统发育树

系统发育树（树状图）展示层级或演化关系。它支持四种输入写法 —— Newick 字符串、边表、两两距离矩阵、scipy/R 的
linkage 输出 —— 外加三种树枝画法、四种朝向、子树着色与支撑值显示。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Tree',
  'series': [{
    'type': 'phylo',
    'newick': '((A:1.0,B:2.0)95:1.0,(C:0.5,D:0.5)88:1.5,E:3.0);',
    'support_threshold': 80
  }]
})) AS chart;
```

枝长可以省略，内部节点上的支撑值会直接从 Newick 字符串里读出来。默认版式把根放在左侧、各叶子对齐 —— 也就是一张
*cladogram*（支序图），深度轴本身不携带信息。

## 支长图模式

`phylogram: true` 会按「从根累积的枝长」给每个节点定位，深度轴于是变成演化距离。这与 cladogram 表达的是**不同的
主张**，两者绝不能混用。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Phylogram',
  'series': [{
    'type': 'phylo',
    'newick': '((A:1.0,B:3.0)90:1.0,(C:2.0,(D:0.5,E:1.5)85:1.0):2.0);',
    'orientation': 'top',
    'phylogram': true,
    'support_threshold': 80
  }]
})) AS chart;
```

## 环形布局

`branch_style: "circular"` 把树径向投开、根在圆心。一棵大树不想变成又高又窄的一条时，就用它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Circular',
  'series': [{
    'type': 'phylo',
    'newick': '((((Sp_A:0.05,Sp_B:0.08):0.12,(Sp_C:0.07,Sp_D:0.06):0.10):0.15,(((Sp_I:0.08,Sp_J:0.12):0.10,(Sp_K:0.05,Sp_L:0.09):0.11):0.15,(Sp_M:0.07,Sp_N:0.08):0.12):0.18):0.10,(Sp_Q:0.15,Sp_R:0.12):0.25);',
    'branch_style': 'circular',
    'phylogram': true
  }]
})) AS chart;
```

## 从边表来

`edges` 收 `{parent, child, length}` 三元组；根就是那个从没当过子的节点。节点下标按**首次出现**的顺序分配 ——
`clade_colors` 用的就是这套下标。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'From edges',
  'series': [{
    'type': 'phylo',
    'edges': (SELECT list({'parent': parent, 'child': child, 'length': length})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/phylo.tsv')),
    'phylogram': true
  }]
})) AS chart;
```

## 子树着色

`clade_colors` 收 `[节点下标, 颜色]` 对，把以该节点为根的整棵子树着色。下标是**首次出现顺序**、不是标签 ——
`[1, …]` 的意思是「边表里提到的第二个不同节点」。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured clades',
  'series': [{
    'type': 'phylo',
    'edges': [
      {'parent': 'root',     'child': 'Bacteria',    'length': 1.5},
      {'parent': 'root',     'child': 'Eukarya',     'length': 2.0},
      {'parent': 'Bacteria', 'child': 'E. coli',     'length': 0.5},
      {'parent': 'Bacteria', 'child': 'B. subtilis', 'length': 0.7},
      {'parent': 'Eukarya',  'child': 'Yeast',       'length': 1.0},
      {'parent': 'Eukarya',  'child': 'Human',       'length': 0.8}
    ],
    'clade_colors': [{'node': 1, 'color': '#e41a1c'}, {'node': 2, 'color': '#377eb8'}],
    'legend': 'domains'
  }]
})) AS chart;
```

## 从距离矩阵做 UPGMA

`distance_matrix` 用 UPGMA 聚类并返回一棵有根的等距树。矩阵必须是方阵且对称，对角线会被忽略。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'UPGMA',
  'series': [{
    'type': 'phylo',
    'distance_matrix': {
      'labels': ['Wolf', 'Cat', 'Whale', 'Human'],
      'dist': [
        [0.0, 0.5, 0.9, 0.8],
        [0.5, 0.0, 0.9, 0.8],
        [0.9, 0.9, 0.0, 0.7],
        [0.8, 0.8, 0.7, 0.0]
      ]
    },
    'phylogram': true
  }]
})) AS chart;
```

## linkage 输入

`linkage` 接受 scipy 的 `linkage` 或 R 的 `hclust` 输出：每行 `[左, 右, 距离, 叶子数]`，其中叶子按标签顺序编号为
`0..n-1`，合并节点从 `n` 起编号。在别处算好的聚类想直接用，就靠它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'From linkage',
  'series': [{
    'type': 'phylo',
    'linkage': {
      'labels': ['A', 'B', 'C', 'D'],
      'linkage': [[0, 1, 0.5, 2], [2, 3, 0.7, 2], [4, 5, 1.2, 4]]
    },
    'phylogram': true
  }]
})) AS chart;
```

## 朝向与树枝画法

| `orientation` | 根的位置 |
| --- | --- |
| `"left"` | 左边缘，叶子往右展开（**默认**） |
| `"right"` | 右边缘，叶子往左展开 |
| `"top"` | 上边缘，叶子向下垂 |
| `"bottom"` | 下边缘，叶子向上长 |

| `branch_style` | 形状 |
| --- | --- |
| `"rectangular"` | 在父节点深度处折直角（**默认**） |
| `"slanted"` | 从父到子一根斜线 |
| `"circular"` | 极坐标 / 径向投影 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Slanted, bottom-up',
  'series': [{
    'type': 'phylo',
    'newick': '((A:1,B:2):1,C:3);',
    'branch_style': 'slanted',
    'orientation': 'bottom',
    'branch_color': '#4c72b0',
    'leaf_color': '#333333'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `newick` | string | Newick 字符串 —— 四种输入写法之一。 |
| `edges` | edge[] | `{parent, child, length}` 三元组。 |
| `distance_matrix` | `{labels, dist}` | 方阵距离矩阵的 UPGMA 聚类。 |
| `linkage` | `{labels, linkage}` | scipy / R `hclust` 输出。 |
| `orientation` | string | `"left"`（默认）· `"right"` · `"top"` · `"bottom"`。 |
| `branch_style` | string | `"rectangular"`（默认）· `"slanted"` · `"circular"`。 |
| `phylogram` | boolean | 深度轴用枝长（默认是 cladogram）。 |
| `branch_color` / `leaf_color` | string | 枝与标签的颜色。 |
| `support_threshold` | number | 达到或高于该值的支撑值才显示。 |
| `clade_colors` | `{node, color}[]` | 给某个节点为根的子树着色。 |
| `legend` | string | 图例标题；每个着色的子树一条。 |

## 说明

- **四种输入写法只能给一种** —— `newick`、`edges`、`distance_matrix` 或 `linkage`。
- `clade_colors` 用的是节点的**首次出现顺序**、不是标签，这是这一页最容易搞错的地方；官方文档那个例子特意用了很短
  的边表，就是为了让下标数得清。
- `support_threshold` 只影响**显示** —— 值仍然会被解析，只是不画。
- `distance_matrix` 必须是方阵且对称；非对称矩阵上 UPGMA 没有定义好的行为。
- 图里**读不回**叶子的渲染顺序，所以官方文档那套「把热力图对齐到树上」的做法在这里无法复现 —— 请改用
  [聚类热图](./clustermap.md)，它自己把两者一起算、对齐天然成立。

## 另见

- [kuva — 系统发育树](https://psy-fer.github.io/kuva/plots/phylo.html) —— 绘图库自己的图型参考。
- [聚类热图](./clustermap.md) —— 按相似度聚类，热力图是内建的。
- [共线性图](../utility/synteny.md) —— 同一批物种之间的基因组结构。
