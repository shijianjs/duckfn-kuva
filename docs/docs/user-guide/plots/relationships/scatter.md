---
title: Scatter plot
sidebar_position: 1
description: Individual (x, y) points, with trend lines, confidence bands, error bars, bubble sizes, per-point colours and six marker shapes.
---

# Scatter plot

A scatter plot renders individual `(x, y)` data points as markers. It supports trend lines, error bars,
variable point sizes, per-point colours and six marker shapes.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Scatter plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'data': array_agg([x, y])
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

The dataset is three 80-point groups in one file. Drawn as a single series they share one colour —
*Multiple series* further down splits them apart.

### Layout options

Axis ranges are derived from the data unless you pin them with `x_axis.min` / `x_axis.max`, put an axis
on a log scale with `y_axis.log`, and so on. All of it is in
[Canvas, title & axes](../../reference/layout.md).

## Trend line

`trend` overlays a least-squares fit. The shorthand `"trend": "linear"` draws the line; the object form
adds a colour, a width, and the fit statistics as text inside the data area.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Linear trend line',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'scatter',
    'size': 5,
    'legend': g,
    'data': pts,
    'trend': {'type': 'linear', 'color': 'crimson', 'equation': true, 'correlation': true}
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

> **Tip:** `equation` and `correlation` print the fit statistics as floating text. With a dense point
> cloud that competes with the data — a [stats box](../../reference/stats-box.md) puts the same numbers in
> a bordered inset instead.

## Confidence band

`band` shades an uncertainty region. `lower` and `upper` are two lists of y values **aligned to the
points' x positions** — same length, same order.

```sql {"type":"duckfn","show":"svg"}
WITH t AS (
  SELECT x, x * 1.8 + 0.5 AS y
  FROM (SELECT unnest(range(1, 11))::DOUBLE AS x)
)
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'data': (SELECT array_agg([x, y] ORDER BY x) FROM t),
    'band': {
      'lower': (SELECT array_agg(y - 1.2 ORDER BY x) FROM t),
      'upper': (SELECT array_agg(y + 1.2 ORDER BY x) FROM t)
    }
  }]
})) AS chart;
```

## Error bars

Per-point errors live in the point itself, so a point that carries them is written as an object rather
than an `[x, y]` pair. `x_err` / `y_err` take a **number** for a symmetric bar, or a
**`[negative, positive]` pair** for an asymmetric one.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'data': [
      {'x': 1.0, 'y': 2.0, 'x_err': 0.2,  'y_err': 0.6},
      {'x': 2.0, 'y': 4.5, 'x_err': 0.15, 'y_err': 0.8},
      {'x': 3.0, 'y': 5.8, 'x_err': 0.3,  'y_err': 0.4},
      {'x': 4.0, 'y': 8.2, 'x_err': 0.1,  'y_err': 0.9},
      {'x': 5.0, 'y': 10.1, 'x_err': 0.25, 'y_err': 0.5}
    ]
  }]
})) AS chart;
```

### Asymmetric errors

Give the pair as `[negative, positive]` — the two arms do not have to match.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 6,
    'data': [
      {'x': 1.0, 'y': 5.0, 'y_err': [0.3, 0.8]},
      {'x': 2.0, 'y': 6.0, 'y_err': [0.5, 1.2]},
      {'x': 3.0, 'y': 7.5, 'y_err': [0.2, 1.6]}
    ]
  }]
})) AS chart;
```

## Marker shapes

Six marker shapes are available through `marker`. They are most useful when several series share one
pair of axes and colour alone is not enough to tell them apart.

`"circle"` (default) · `"square"` · `"triangle"` · `"diamond"` · `"cross"` · `"plus"`

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Marker shapes',
  'x_axis': {'name': 'x'},
  'legend': {'position': 'outside_right_top'},
  'series': series
})) AS chart
FROM (
  SELECT list({
    'type': 'scatter',
    'data': [[1.0, y], [2.0, y], [3.0, y]],
    'color': color,
    'size': 7,
    'marker': marker,
    'legend': label
  } ORDER BY y) AS series
  FROM (VALUES
    (1.0, 'steelblue',   'circle',   'Circle'),
    (2.0, 'crimson',     'square',   'Square'),
    (3.0, 'seagreen',    'triangle', 'Triangle'),
    (4.0, 'darkorange',  'diamond',  'Diamond'),
    (5.0, 'purple',      'cross',    'Cross'),
    (6.0, 'saddlebrown', 'plus',     'Plus')
  ) AS t(y, color, marker, label)
);
```

## Bubble plot

Encode a third dimension in the point's area with `sizes` — per-point radii in pixels. `sizes` is
matched to the data by index and overrides `size`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bubble plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'data': [[1.0, 3.0], [2.5, 6.5], [4.0, 4.0], [5.5, 8.0], [7.0, 5.5], [8.5, 9.0]],
    'sizes': [5.0, 14.0, 9.0, 18.0, 11.0, 7.0]
  }]
})) AS chart;
```

## Per-point colours

`colors` assigns a colour per point, matched by index and falling back to `color` past the end. It is
the way to colour by a label your data already carries without splitting it into one series per group.

The legend is **not** updated by `colors`; use one series per group when you need a labelled legend
(see *Multiple series* below).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'size': 6,
    'data': pts,
    'colors': colors
  }]
})) AS chart
FROM (
  SELECT
    array_agg([x, y] ORDER BY x) AS pts,
    array_agg(CASE "group"
      WHEN 'Group_A' THEN '#4c72b0'
      WHEN 'Group_B' THEN '#c44e52'
      ELSE '#55a868'
    END ORDER BY x) AS colors
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## Marker opacity and stroke

`marker_opacity` sets the fill alpha and `marker_stroke_width` draws an outline in the fill colour.
Together they give three modes, which matter most for dense data:

| Mode | Setting | Use case |
| --- | --- | --- |
| **Solid** (default) | neither field | Small N, well-separated clusters |
| **Semi-transparent** | `marker_opacity` below `1`, plus a stroke | Dense regions pool colour; individual points stay legible |
| **Hollow** | `marker_opacity: 0`, plus a stroke | Very large N; overlapping outlines reveal density without blobs |

### Semi-transparent markers

600 points from the contour dataset share one region. Solid markers at this density merge into a single
opaque mass; dropping the opacity to `0.25` lets the darker overlap show where the cloud is dense, while
a thin stroke keeps each marker readable.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Semi-transparent markers',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'marker_opacity': 0.25,
    'marker_stroke_width': 0.7,
    'data': array_agg([x, y])
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/contour.tsv');
```

### Hollow open circles

`marker_opacity: 0` draws only the outline. With overlapping points the accumulation of outlines shows
exactly where the density is, instead of one dark blob.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Hollow markers',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 4,
    'marker_opacity': 0,
    'marker_stroke_width': 1,
    'data': array_agg([x, y])
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/contour.tsv');
```

## Multiple series

One object per series in the `series` list, each with its own `color` and `legend`, draws them on the
same axes. The legend appears as soon as any series carries a `legend`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'scatter', 'size': 5, 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | point[] | **Required.** The points, as `[x, y]` pairs or `{x, y, x_err?, y_err?}` objects. |
| `size` | number | Uniform point radius (default 3). |
| `sizes` | number[] | Per-point radii (a bubble plot); overrides `size`. |
| `colors` | string[] | Per-point colours; falls back to `color` past the end. |
| `marker` | string | `"circle"` (default) · `"square"` · `"triangle"` · `"diamond"` · `"cross"` · `"plus"`. |
| `marker_opacity` | number | Fill alpha: `0` is hollow, `1` solid. |
| `marker_stroke_width` | number | Outline width, drawn in the fill colour. |
| `trend` | `"linear"` \| object | Overlay a least-squares line; the object adds `type`, `color`, `width`, `equation`, `correlation`. |
| `band` | `{lower, upper}` | A shaded band aligned to the points' x positions. |
| `group_name` | string | A group name for interactive SVG output (does not enter the legend). |

`color`, `legend`, `tooltips` and `tooltip_labels` come from
[series & shared fields](../../reference/series.md); `x_err` / `y_err` from the
[point type](../../reference/series.md#points).

## Notes

- **`data` must not be empty.** A `sizes` or `colors` list that is longer than the data is fine; a
  shorter one falls back to `size` / `color`.
- Per-point colours do **not** update the legend — use one series per group when you want a labelled
  legend.
- `x_err` / `y_err` take a number (symmetric) or a `[lower, upper]` pair (asymmetric).
- `band.lower` and `band.upper` must each be the same length as `data`, in the same order.

## See also

- [kuva — Scatter plot](https://psy-fer.github.io/kuva/plots/scatter.html) — the plotting library's own reference for this chart.
- [Line plot](./line.md) — for ordered, connected data.
- [2D histogram](../distributions/histogram2d.md) and [Hexbin](../distributions/hexbin.md) — for large point clouds.
