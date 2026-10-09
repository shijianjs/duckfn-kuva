---
title: Sunburst chart
sidebar_position: 2
description: A hierarchy as concentric rings, arc width proportional to value.
---

# Sunburst chart

A sunburst chart lays a hierarchy out as concentric rings: the innermost ring is the top level, each further
ring a deeper level, and an arc's width within its ring is proportional to that node's value. It uses the
same node model as the [treemap](./treemap.md) — only the geometry differs.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Classes',
  'series': [{
    'type': 'sunburst',
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

The rings read outward as the hierarchy deepens, and the angle at any radius is the share of that level.
An inner node's own `value` is summed from its children, so a parent's arc is always as wide as the sum of
its parts.

## Several roots

Multiple roots share the innermost ring, each taking a distinct palette colour. It is the shape to use when
the top level is a small set of independent categories rather than one root.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Two groups',
  'series': [{
    'type': 'sunburst',
    'roots': [
      {'label': 'Frontend', 'children': [
        {'label': 'React',  'value': 50},
        {'label': 'Vue',    'value': 30},
        {'label': 'Svelte', 'value': 20}
      ]},
      {'label': 'Backend', 'children': [
        {'label': 'Rust',   'value': 40},
        {'label': 'Go',     'value': 35},
        {'label': 'Python', 'value': 25}
      ]}
    ],
    'color_mode': 'by_parent'
  }]
})) AS chart;
```

## Donut style

`inner_radius` is a **fraction** of the outer radius, clamped to `[0, 0.95]`. A hole of `0.3`–`0.35` keeps
the centre calm and leaves room for a total or a title, which is what makes a sunburst readable in a
dashboard.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Donut style',
  'series': [{
    'type': 'sunburst',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'inner_radius': 0.35,
    'ring_gap': 2
  }]
})) AS chart;
```

## Colour modes

| `color_mode` | Arcs |
| --- | --- |
| `"by_parent"` | Inherit their root's colour **(default)** |
| `"explicit"` | Use the node's own `color` |
| `{"color_map": "viridis"}` | Coloured by value, with the parent arcs left neutral |

In by-value mode the **inner arcs are drawn in a neutral grey**, because a ring's parent has no single
value to colour by — an arc cannot be both its own colour and the sum of its children's.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Coloured by value',
  'series': [{
    'type': 'sunburst',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': {'color_map': 'viridis'},
    'colorbar': true,
    'colorbar_label': 'value'
  }]
})) AS chart;
```

## Second-dimension colouring

`color_values` is a flat, depth-first list of numbers parallel to the leaves, so size can encode one
quantity and colour another — the GO-enrichment pattern, where the arc is the gene count and the colour is
the p-value.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Size versus significance',
  'series': [{
    'type': 'sunburst',
    'roots': [
      {'label': 'GO:0006955', 'value': 120},
      {'label': 'GO:0007049', 'value': 85},
      {'label': 'GO:0016310', 'value': 60}
    ],
    'color_values': [-10, -6.7, -4.3],
    'color_mode': {'color_map': 'viridis'},
    'colorbar': true,
    'colorbar_label': '-log10(p)'
  }]
})) AS chart;
```

`-log10(p)` rather than `p` itself: the colour scale should run dark-to-bright as significance increases,
and there is no sensible colour for a p-value of `0`.

## Angles, rings and labels

| Field | Default | What it sets |
| --- | --- | --- |
| `start_angle` | `0` | Degrees, `0` = twelve o'clock, running clockwise |
| `ring_gap` | `1` | Gap between rings, in pixels |
| `show_labels` | `true` | Draw the arc labels |
| `min_label_angle` | `15` | Do not label an arc sweeping less than this |
| `rotate_labels` | `true` | Rotate labels along the circumference |
| `max_depth` | — | Depth limit |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Rotated start, wider rings',
  'series': [{
    'type': 'sunburst',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'start_angle': 90,
    'ring_gap': 3,
    'min_label_angle': 8,
    'max_depth': 2
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `roots` | node[] | **Required.** One entry per root; each is `{label, value?, color?, children?}`. |
| `value` | number | A leaf's size; summed from the children on an inner node. |
| `children` | node[] | Nested nodes. |
| `color_mode` | string \| object | `"by_parent"` (default) · `"explicit"` · `{"color_map": …}`. |
| `color_values` | number[] | Flat depth-first values for the colour encoding. |
| `color_range` | `[number, number]` | Clamp the colour scale. |
| `colorbar` / `colorbar_label` | boolean / string | The colour bar and its title. |
| `inner_radius` | number | Inner hole as a fraction of the outer radius, clamped to `[0, 0.95]`. |
| `ring_gap` | number | Gap between rings, in pixels. |
| `start_angle` | number | The first arc's angle, in degrees. |
| `show_labels` / `min_label_angle` | boolean / number | Arc labels and their minimum sweep. |
| `rotate_labels` | boolean | Rotate labels with the circumference. |
| `max_depth` | integer | Depth limit. |
| `tooltips` | boolean | SVG hover tooltips (default on). |

## Notes

- **`roots` must not be empty.** Every node needs a `label`; a leaf also needs a `value`.
- `children` must be an empty **list**, never `null` — hence the `COALESCE(…, CAST([] AS …))` in the SQL.
- `inner_radius` is a **fraction**, unlike the [pie chart's](../categorical/pie.md) `inner_radius`, which is
  in pixels. The two fields share a name and nothing else.
- Arcs below `min_label_angle` are left unlabelled, so a hierarchy with many small leaves ends up relying
  on the tooltips or a legend.
- `color_values` must be in depth-first leaf order and exactly as long as the leaf count.

## See also

- [kuva — Sunburst chart](https://psy-fer.github.io/kuva/plots/sunburst.html) — the plotting library's own reference for this chart.
- [Treemap](./treemap.md) — the same hierarchy in rectangles.
- [Pie chart](../categorical/pie.md) — a single ring with no hierarchy.
