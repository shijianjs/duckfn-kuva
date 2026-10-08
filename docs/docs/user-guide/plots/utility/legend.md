---
title: Legend plot
sidebar_position: 4
description: A standalone legend, drawn on its own for reports and slides.
---

# Legend plot

A legend plot draws a legend on its own — useful when a shared legend belongs beside a figure rather
than inside it, or when you want it as an image for a slide.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'entries': [
      to_json({'label': 'control', 'color': '#4c72b0'}),
      to_json({'label': 'drug A', 'color': '#c44e52', 'shape': 'line', 'dasharray': '4 2'}),
      to_json({'label': 'drug B', 'color': '#55a868', 'shape': 'circle'}),
      to_json({'label': 'drug C', 'color': '#8172b2', 'shape': {'marker': 'triangle'}}),
      to_json({'label': 'dose', 'color': '#937860', 'shape': {'size': 6}})
    ],
    'cols': 2,
    'title': 'groups',
    'show_box': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `entries` | entry[] | **Required.** One entry per legend item: `{label, color, shape?, dasharray?}`. |
| `cols` | integer | A fixed column count. |
| `max_cols` | integer | The column cap when laying out automatically. |
| `max_entries` | integer | Show at most this many (at least 1). |
| `title` | string | A bold legend title. |
| `show_box` | boolean | Draw a box around the legend (`false` removes it). |

An entry's `shape` is `"rect"` (default) · `"line"` · `"circle"`, or `{"marker": "triangle"}` (a scatter
marker) or `{"size": 6}` (a sized circle). `dasharray` is used by the `"line"` shape.

## Notes

- **`entries` must not be empty**, and **`max_entries` must be at least 1** — 0 would underflow.
- The alias `"legend"` also works for this `type`.

## See also

- [kuva — Legend plot](https://psy-fer.github.io/kuva/plots/legend.html) — the plotting library's own reference for this chart.
- [Legends](../../reference/legends.md) — the in-figure legend.
