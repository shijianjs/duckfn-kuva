---
title: Treemap
sidebar_position: 1
description: Nested rectangles proportional to value, for hierarchical data.
---

# Treemap

A treemap tiles a rectangle with nested rectangles proportional to node values. The squarified layout used
by default keeps each rectangle's aspect ratio as close to square as it can, which is what makes the areas
comparable by eye.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'By region',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': 'by_parent'
  }]
})) AS chart;
```

The tree is built in SQL: one `list(…)` per parent collects its children, and a left join hangs them off
each root. `COALESCE(…, CAST([] AS …))` is there because a root with no children must still carry an
**empty list** — a `children: null` is a type error, not "no children".

## Flat data

When every node is a leaf there is nothing to nest, and the treemap becomes a single level of rectangles.
That is the right shape for a plain part-to-whole breakdown, and it needs no join at all.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Leaves only',
  'series': [{
    'type': 'treemap',
    'roots': roots
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'value': value} ORDER BY value DESC) AS roots
  FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')
  WHERE parent IS NOT NULL AND parent <> ''
);
```

## Colour modes

| `color_mode` | Leaves |
| --- | --- |
| `"by_parent"` | Inherit their root's colour **(default)** |
| `"explicit"` | Use the node's own `color` |
| `{"color_map": "viridis"}` | Coloured by a parallel value, with a colour bar |

`color_values` is a **flat, depth-first** list of numbers parallel to the leaves, which is how a treemap
carries a second variable: size encodes one quantity and colour another.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Coloured by value',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': {'color_map': 'viridis'},
    'colorbar_label': 'value'
  }]
})) AS chart;
```

## Layout algorithms

| `layout` | Tiling |
| --- | --- |
| `"squarify"` | Bruls 2000 — minimises the worst aspect ratio per strip **(default)** |
| `"slice_dice"` | Alternating horizontal / vertical cuts per depth — simple and predictable |
| `"binary"` | Balanced binary splits, also alternating direction |

`slice_dice` is faster and produces the classic "striped" look; it also produces thin slivers on unbalanced
data, which is exactly what squarify exists to avoid.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Slice and dice',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'layout': 'slice_dice',
    'padding': 6
  }]
})) AS chart;
```

## Padding, borders and labels

| Field | Default | What it sets |
| --- | --- | --- |
| `padding` | `4` | Gap between a parent's border and its children, in pixels — halved at each depth |
| `border_width` | `0.5` | Leaf and inner border width |
| `root_border_width` | `2` | Root border width |
| `show_labels` / `show_parent_labels` | `true` | Leaf labels and group labels |
| `min_label_area` | `1200` | Do not draw a label in a cell smaller than this (px²) |
| `max_depth` | — | Render at most this many levels deep (root = depth 0) |
| `tooltips` | `true` | Emit SVG hover tooltips |

`min_label_area` is the one that matters on a dense treemap: without it, small cells print unreadable
fragments of text.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Large labels only',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'min_label_area': 4000,
    'padding': 8,
    'root_border_width': 3,
    'tooltips': false
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `roots` | node[] | **Required.** One entry per root; each is `{label, value?, color?, children?}`. |
| `value` | number | A leaf's size; on an inner node it is summed from the children when omitted. |
| `children` | node[] | Nested nodes. A node with children is an inner node. |
| `color` | string | Per-node colour, used by `"explicit"` mode. |
| `color_mode` | string \| object | `"by_parent"` (default) · `"explicit"` · `{"color_map": …}`. |
| `color_values` | number[] | Flat depth-first values for the colour encoding. |
| `color_range` | `[number, number]` | Clamp the colour scale. |
| `colorbar` / `colorbar_label` | boolean / string | The colour bar and its title. |
| `layout` | string | `"squarify"` (default) · `"slice_dice"` · `"binary"`. |
| `padding` | number | Padding between a parent and its children. |
| `border_width` / `root_border_width` | number | Border widths. |
| `show_labels` / `show_parent_labels` | boolean | Leaf and group labels. |
| `min_label_area` | number | Smallest cell that still gets a label, in px². |
| `max_depth` | integer | Depth limit. |
| `tooltips` | boolean | SVG hover tooltips (default on). |

## Notes

- **`roots` must not be empty.** Every node needs a `label`; a leaf also needs a `value`.
- An inner node with **no** `value` sums its children; give one and it wins, which is how you make a parent
  deliberately larger than the sum of the parts.
- `children` must be an empty **list**, never `null` — that is what the `COALESCE(…, CAST([] AS …))` in the
  SQL examples is for.
- `color_values` is a flat list in **depth-first** order, so its length has to equal the leaf count and its
  order has to match the tree — the easiest thing in this page to get subtly wrong.
- `padding` halves at each depth, so a deep tree's inner levels end up with very small gaps.

## See also

- [kuva — Treemap](https://psy-fer.github.io/kuva/plots/treemap.html) — the plotting library's own reference for this chart.
- [Sunburst](./sunburst.md) — the same hierarchy, laid out radially.
- [Heatmap](../distributions/heatmap.md) — a flat matrix instead of a hierarchy.
