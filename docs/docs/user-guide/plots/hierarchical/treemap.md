---
title: Treemap
sidebar_position: 1
description: A hierarchy as nested rectangles whose areas are proportional to the values.
---

# Treemap

A treemap lays a hierarchy out as nested rectangles, each sized by its value. It shows a part-to-whole
breakdown across two levels at once, without the wasted space of a pie.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'treemap',
    'roots': roots,
    'color_mode': 'by_parent',
    'show_labels': true,
    'padding': 3,
    'legend': 'value'
  }]
})) AS chart
FROM (
  SELECT list({'label': parent, 'children': kids} ORDER BY parent) AS roots
  FROM (
    SELECT parent, list({'label': label, 'value': value} ORDER BY value DESC) AS kids
    FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')
    WHERE coalesce(parent, '') <> ''
    GROUP BY parent
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `roots` | node[] | **Required.** The forest's roots; each is a tree (see below). |
| `color_values` | number[] | Colour values parallel to the leaves, in depth-first order. |
| `color_mode` | string \| object | `"by_parent"` · `"explicit"` · `{"color_map": "viridis"}` (colour by leaf value). |
| `layout` | string | `"squarify"` (default) · `"slice_dice"` · `"binary"`. |
| `show_labels` | boolean | Label the leaves. |
| `show_parent_labels` | boolean | Label the internal nodes. |
| `min_label_area` | number | Hide a label below this area, in px². |
| `padding` | number | Padding between rectangles. |
| `border_width` | number | Rectangle border width. |
| `root_border_width` | number | Width of the root rectangle's border. |
| `color_range` | `[number, number]` | The colour-value range. |
| `colorbar` | boolean | Draw a colour bar. |
| `colorbar_label` | string | The colour bar's title. |
| `max_depth` | integer | Draw only down to this depth. |
| `tooltips` | boolean | Hover tooltips (on by default). |

A node is `{label, value?, color?, children?}`. A node **with** `children` is internal (`value` defaults
to the sum of its children); a node **without** is a leaf and needs a `value`.

## Notes

- **`roots` must not be empty**, and **every leaf needs a `value`** — a `value` of 0 or less makes a root
  render blank and is reported.
- `color_values` must be parallel to the leaves in depth-first order; a mismatch is not checked.

## See also

- [kuva — Treemap](https://psy-fer.github.io/kuva/plots/treemap.html) — the plotting library's own reference for this chart.
- [Sunburst](./sunburst.md) — the same hierarchy on rings.
- [Bar chart](../categorical/bar.md) — a flat part-to-whole view.
