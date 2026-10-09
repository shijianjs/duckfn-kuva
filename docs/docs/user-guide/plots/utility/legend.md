---
title: Legend plot
sidebar_position: 4
description: A legend as its own panel — shared keys and standalone keys.
---

# Legend plot

A legend plot is a panel that renders a legend grid and nothing else: no axes, no data. It exists for two
jobs — giving a multi-panel figure **one shared legend** instead of repeating the same key in every panel,
and producing a **standalone key** to composite onto an image or a slide.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Groups',
    'cols': 3,
    'entries': [
      {'label': 'Treatment', 'color': '#4477AA', 'shape': 'rect'},
      {'label': 'Control',   'color': '#EE6677', 'shape': 'rect'},
      {'label': 'Baseline',  'color': '#CCBB44', 'shape': 'line', 'dasharray': '4 2'}
    ]
  }]
})) AS chart;
```

## Entry shapes

| `shape` | Swatch |
| --- | --- |
| `"rect"` | A filled square, the default for categorical fills |
| `"line"` | A line, optionally dashed via `dasharray` |
| `"circle"` | A filled circle |
| `{"marker": "triangle"}` | Any of the scatter marker shapes |
| `{"size": 6}` | A circle of an explicit radius — the size-legend form |

The `{"size": n}` form is what a size legend needs: a dot plot's key shows three circles of different radii,
and this is how one is written by hand. `dasharray` only has an effect on a `"line"` swatch.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Expressed in',
    'cols': 2,
    'entries': [
      {'label': '80 %', 'color': '#c44e52', 'shape': {'size': 7}},
      {'label': '40 %', 'color': '#c44e52', 'shape': {'size': 5}},
      {'label': '10 %', 'color': '#c44e52', 'shape': {'size': 3}}
    ]
  }]
})) AS chart;
```

::::note[Keep the shapes in a list identical]

A `"line"` swatch is the bare string `"line"`, a marker swatch is an object `{"marker": …}` and a size swatch
is an object `{"size": …}`. DuckDB has to give one list a single element type, and it cannot unify a string
with a struct — so a single `entries` list has to hold **one kind of shape**. Mixing them means splitting the
key into two legend plots, or building the array from a table where the shapes are already uniform.

::::

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Marker key',
    'cols': 2,
    'entries': [
      {'label': 'circle',   'color': '#4c72b0', 'shape': {'marker': 'circle'}},
      {'label': 'triangle', 'color': '#dd8452', 'shape': {'marker': 'triangle'}},
      {'label': 'square',   'color': '#55a868', 'shape': {'marker': 'square'}},
      {'label': 'diamond',  'color': '#c44e52', 'shape': {'marker': 'diamond'}}
    ]
  }]
})) AS chart;
```

## A shared legend in a figure

The reason to reach for this chart: in a grid of panels, repeating the same legend in each one wastes the
space a legend needs and still reads as four different keys. One legend panel for the whole figure reads as
one.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 2,
    'cols': 1,
    'cell_width': 460,
    'cell_height': 300,
    'panels': [
      {
        'x_axis': {'name': 'x'},
        'y_axis': {'name': 'y'},
        'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d),
                    'color': 'steelblue', 'legend': 'measurements'}]
      },
      {
        'series': [{
          'type': 'legend_plot',
          'cols': 2,
          'entries': [
            {'label': 'measurements', 'color': 'steelblue', 'shape': 'circle'},
            {'label': 'model fit',    'color': 'firebrick', 'shape': 'line'}
          ]
        }]
      }
    ]
  }
})) AS chart;
```

The legend panel's `rows`/`cols` place it; give the row less height than the data panel so the key reads as
a footer rather than as a second chart.

## Placement, columns and trimming

| Field | Default | What it sets |
| --- | --- | --- |
| `cols` | auto | Fix the number of columns. |
| `max_cols` | — | Cap the columns when the layout is automatic. |
| `max_entries` | — | Show at most this many entries |
| `title` | — | A bold title row above the entries |
| `show_box` | `true` | Draw the background and border |

`cols: 1` is how you build a side legend: a single column of entries fits a narrow panel, and the figure's
own grid gives it its own band. `max_entries` is the escape hatch for a key that has grown past what the
panel can show.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Sequential scale',
    'cols': 1,
    'show_box': false,
    'entries': [
      {'label': 'H1', 'color': '#c6dbef', 'shape': 'rect'},
      {'label': 'H2', 'color': '#9ecae1', 'shape': 'rect'},
      {'label': 'H3', 'color': '#6baed6', 'shape': 'rect'},
      {'label': 'H4', 'color': '#4292c6', 'shape': 'rect'},
      {'label': 'H5', 'color': '#2171b5', 'shape': 'rect'}
    ]
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `entries` | entry[] | **Required.** Each entry is `{label, color, shape?, dasharray?}`. |
| `cols` | integer | Fixed column count; otherwise auto from the panel width. |
| `max_cols` | integer | Column cap in automatic layout. |
| `max_entries` | integer | Show at most this many entries — **at least 1**. |
| `title` | string | A bold title above the entries. |
| `show_box` | boolean | Background and border (default on). |

## Notes

- **`entries` must not be empty**, and every entry needs a `label` and a `color`.
- `max_entries` of `0` is rejected: the layout arithmetic subtracts one and would underflow.
- A legend plot fills its panel with a grid and does not draw axes, so a `title` on its layout is wasted —
  put the title on the legend itself.
- `show_box: false` gives the entries no background, which is what you want when the legend sits on a
  coloured card rather than on white.
- The entries are **hand-written** here; there is no way to harvest a legend from another panel's series, so
  a shared key and the panels it describes have to be kept in step by hand.

## See also

- [kuva — Legend plot](https://psy-fer.github.io/kuva/plots/legend.html) — the plotting library's own reference for this chart.
- [Text plot](./text.md) — the other annotation panel.
- [Reference: legends](../../reference/legends.md) — the automatic legend system, and `legend.entries` for a
  hand-written key inside a normal chart.
