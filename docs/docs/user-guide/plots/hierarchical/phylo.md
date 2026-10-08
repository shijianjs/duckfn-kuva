---
title: Phylogenetic tree
sidebar_position: 6
description: A rooted tree from a Newick string, an edge list, a distance matrix or a linkage matrix.
---

# Phylogenetic tree

A phylogenetic tree draws a rooted tree with branch lengths. Give it a Newick string, an edge list, a
distance matrix or a linkage matrix — exactly one of the four.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'phylo',
    'edges': list({'parent': parent, 'child': child, 'length': length}),
    'orientation': 'right',
    'branch_style': 'slanted',
    'phylogram': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/phylo.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `newick` | string | A Newick string, e.g. `"((A:0.1,B:0.2):0.3,C:0.4);"`. |
| `edges` | edge[] | An edge list: `{parent, child, length}`; the root is the node that is never a child. |
| `distance_matrix` | object | `{labels, dist}` — a square distance matrix (UPGMA clustering). |
| `linkage` | object | `{labels, linkage}` — linkage rows `[left, right, distance, leaf_count]`. |
| `orientation` | string | `"left"` · `"right"` · `"top"` · `"bottom"`. |
| `branch_style` | string | `"rectangular"` · `"slanted"` · `"circular"`. |
| `phylogram` | boolean | Scale branches by accumulated length (otherwise leaves are equally spaced). |
| `branch_color` / `leaf_color` | string | Colours for branches and leaves. |
| `support_threshold` | number | Treat support values below this as noise and skip them. |
| `clade_colors` | `[integer, string][]` | Colour a node (and its subtree): `[node index, color]`. |
| `legend` | string | The legend title. |

## Notes

- **Give exactly one of the four input forms.** None is an error, and more than one is an error
  (they are mutually exclusive).
- For `distance_matrix`, `dist` must be square and `labels` must match its side; `clade_colors` indices
  are checked against the tree's node count.

## See also

- [kuva — Phylogenetic tree](https://psy-fer.github.io/kuva/plots/phylo.html) — the plotting library's own reference for this chart.
- [Treemap](./treemap.md) · [Sunburst](./sunburst.md) — other views of a hierarchy.
