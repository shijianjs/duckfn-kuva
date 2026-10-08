---
title: Sunburst chart
sidebar_position: 2
description: A hierarchy as concentric rings, each wedge sized by its value.
---

# Sunburst chart

A sunburst chart draws a hierarchy as concentric rings: the root at the centre, each level a ring, each
node a wedge sized by its value. It shares its data model with the [treemap](./treemap.md).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'sunburst',
    'roots': roots,
    'color_mode': 'by_parent',
    'show_labels': true,
    'inner_radius': 0.2,
    'ring_gap': 2,
    'max_depth': 3
  }]
})) AS chart
FROM (
  SELECT list({'label': parent, 'children': kids} ORDER BY parent) AS roots
  FROM (
    SELECT parent, list({'label': label, 'value': value} ORDER BY value DESC) AS kids
    FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')
    WHERE coalesce(parent, '') <> ''
    GROUP BY parent
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `roots` | node[] | **Required.** The forest's roots; several roots share the innermost ring. |
| `color_values` | number[] | Colour values parallel to the leaves, in depth-first order. |
| `color_mode` | string \| object | `"by_parent"` · `"explicit"` · `{"color_map": "viridis"}`. |
| `show_labels` | boolean | Label the wedges. |
| `min_label_angle` | number | Skip a label below this wedge angle, in degrees. |
| `inner_radius` | number | Inner radius as a fraction of the outer radius, clamped to `[0, 0.95]`. |
| `ring_gap` | number | Gap between rings, in pixels. |
| `start_angle` | number | Start angle in degrees: `0` is 12 o'clock, clockwise. |
| `rotate_labels` | boolean | Rotate labels along the circumference. |
| `max_depth` | integer | Draw only down to this depth. |
| `colorbar` | boolean | Draw a colour bar. |
| `color_range` | `[number, number]` | The colour-value range. |
| `tooltips` | boolean | Hover tooltips (on by default). |

Nodes are `{label, value?, color?, children?}` exactly as for the treemap.

## Notes

- **`roots` must not be empty**, and **every leaf needs a `value`**.
- `min_label_angle` keeps thin wedges from accumulating unreadable labels.

## See also

- [kuva — Sunburst chart](https://psy-fer.github.io/kuva/plots/sunburst.html) — the plotting library's own reference for this chart.
- [Treemap](./treemap.md) — the same hierarchy as rectangles.
