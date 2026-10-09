---
title: Phylogenetic tree
sidebar_position: 6
description: A dendrogram from Newick, an edge list, a distance matrix or linkage output.
---

# Phylogenetic tree

A phylogenetic tree (dendrogram) shows hierarchical or evolutionary relationships. Four input forms are
supported — a Newick string, an edge list, a pairwise distance matrix, or scipy/R linkage output — along
with three branch styles, four orientations, clade colouring and support values.

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

Branch lengths are optional, and support values on internal nodes are read straight off the Newick string.
The default layout puts the root on the left with the leaves aligned — a *cladogram*, where the depth axis
carries no information.

## Phylogram mode

`phylogram: true` positions each node by the accumulated branch length from the root, so the depth axis
becomes evolutionary distance. That is a different claim from a cladogram's, and the two should never be
used interchangeably.

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

## Circular layout

`branch_style: "circular"` projects the tree radially with the root at the centre. It is the layout that
keeps a large tree from becoming a very tall, very thin picture.

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

## From an edge list

`edges` takes `{parent, child, length}` triples; the root is whichever node never appears as a child. Node
ids are assigned in order of first appearance, which is what `clade_colors` indexes.

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

## Clade colouring

`clade_colors` takes `[node_index, colour]` pairs and colours the whole subtree rooted at that node. The
indices are **first-appearance order** across the edges, not labels — so `[1, …]` means "the second distinct
node the edge list mentions".

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

## UPGMA from a distance matrix

`distance_matrix` clusters the taxa by UPGMA and returns a rooted, ultrametric tree. The matrix must be
square and symmetric; its diagonal is ignored.

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

## Linkage input

`linkage` accepts the output of scipy's `linkage` or R's `hclust`: each row is
`[left, right, distance, n_leaves]`, where leaves are `0..n-1` in label order and merges are numbered from
`n` onward. It is the way to bring in a clustering computed elsewhere without re-deriving it.

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

## Orientations and branch styles

| `orientation` | Root |
| --- | --- |
| `"left"` | Left edge, leaves fanning right **(default)** |
| `"right"` | Right edge, leaves fanning left |
| `"top"` | Top edge, leaves hanging down |
| `"bottom"` | Bottom edge, leaves growing up |

| `branch_style` | Shape |
| --- | --- |
| `"rectangular"` | Right-angle elbow at the parent's depth **(default)** |
| `"slanted"` | A single diagonal from parent to child |
| `"circular"` | Polar / radial projection |

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

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `newick` | string | A Newick string — one of the four input forms. |
| `edges` | edge[] | `{parent, child, length}` triples. |
| `distance_matrix` | `{labels, dist}` | UPGMA clustering of a square distance matrix. |
| `linkage` | `{labels, linkage}` | scipy / R `hclust` output. |
| `orientation` | string | `"left"` (default) · `"right"` · `"top"` · `"bottom"`. |
| `branch_style` | string | `"rectangular"` (default) · `"slanted"` · `"circular"`. |
| `phylogram` | boolean | Use branch lengths on the depth axis (default: cladogram). |
| `branch_color` / `leaf_color` | string | Branch and label colours. |
| `support_threshold` | number | Show support values at or above this level. |
| `clade_colors` | `{node, color}[]` | Colour the subtree rooted at a node index. |
| `legend` | string | Legend title; one entry per coloured clade. |

## Notes

- **Give exactly one of the four inputs** — `newick`, `edges`, `distance_matrix` or `linkage`.
- `clade_colors` indexes nodes by **first-appearance order**, not by label, which is the easiest thing here
  to get wrong; the official docs' own example is a small edge list precisely so the indices are countable.
- `support_threshold` filters for display only — the values are still parsed, just not drawn.
- A `distance_matrix` must be square and symmetric; UPGMA has no defined behaviour on an asymmetric one.
- There is **no way to read the leaves' render order** back out of a chart, so the "align a heatmap to the
  tree" recipe from the library docs cannot be reproduced here — use a [clustermap](./clustermap.md)
  instead, which computes both and guarantees the alignment.

## See also

- [kuva — Phylogenetic tree](https://psy-fer.github.io/kuva/plots/phylo.html) — the plotting library's own reference for this chart.
- [Clustermap](./clustermap.md) — clustering by similarity, with the heatmap built in.
- [Synteny](../utility/synteny.md) — genome structure across the same taxa.
