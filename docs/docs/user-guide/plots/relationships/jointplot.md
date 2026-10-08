---
title: Joint plot
sidebar_position: 5
description: A scatter with marginal distributions along the top and right edges.
---

# Joint plot

A joint plot combines a central scatter with marginal distribution panels on the top and right edges.
Each panel shows the univariate distribution of the corresponding axis — as histogram bars or a kernel
density estimate. That puts the bivariate relationship and both marginal shapes in one figure.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Joint plot',
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

Both panels default to histograms with 20 bins. Axis labels for a joint plot come from its own
`x_label` / `y_label` — the figure-level `x_axis` / `y_axis` do not reach the scatter axis here.

## Marginal type

`marginal_type` switches between histogram bars and a filled density curve.

| `marginal_type` | Panel |
| --- | --- |
| `"histogram"` | Histogram bars **(default)** |
| `"density"` | Filled kernel density estimate |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Density marginals',
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'marginal_type': 'density',
    'bandwidth': 0.4,
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

The KDE bandwidth defaults to Silverman's rule of thumb; `bandwidth` overrides it with a fixed value.

## Showing and hiding marginal panels

Each panel toggles independently. Turning both off leaves a plain scatter — the same cells render either
way, which makes the joint plot a drop-in replacement while you tune it.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'show_right': false,
    'bins': 25,
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## Multiple groups

Give each group a `label` and a `color`. As soon as two groups carry labels, the legend appears to the
right of the marginal panel.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Multiple groups',
  'series': [{
    'type': 'jointplot',
    'groups': list({'x': xs, 'y': ys, 'label': g} ORDER BY g),
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT "group" AS g,
         list(x ORDER BY x, y) AS xs,
         list(y ORDER BY x, y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

Each group's marginal bars or density fill use the group colour at reduced alpha, which
`marginal_alpha` controls.

## Trend lines

A group takes the scatter trend fields directly: `trend` for the least-squares line, plus `equation`
and `correlation` for the fit statistics.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Trend lines',
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys, 'color': '#e15759',
                'trend': true, 'correlation': true}],
    'x_label': 'measurement',
    'y_label': 'response'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## Error bars

A group takes per-point `x_err` / `y_err` — a **number** for a symmetric bar, a **`[negative, positive]`
pair** for an asymmetric one — just like a [scatter](./scatter.md) series. Each list must be the same
length as the group's `x` / `y`.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    (1.0,  2.1, 0.15, 0.30),
    (2.0,  3.9, 0.15, 0.40),
    (3.0,  6.2, 0.20, 0.35),
    (4.0,  7.8, 0.20, 0.40),
    (5.0, 10.1, 0.25, 0.50),
    (6.0, 12.3, 0.25, 0.45),
    (7.0, 13.9, 0.30, 0.50),
    (8.0, 16.2, 0.30, 0.55)
  ) AS t(x, y, x_err, y_err)
)
SELECT kuva_render(to_json({
  'title': 'Error bars',
  'series': [{
    'type': 'jointplot',
    'groups': [{
      'x': (SELECT list(x ORDER BY x) FROM d),
      'y': (SELECT list(y ORDER BY x) FROM d),
      'color': '#76b7b2',
      'x_err': (SELECT list(x_err ORDER BY x) FROM d),
      'y_err': (SELECT list(y_err ORDER BY x) FROM d)
    }],
    'x_label': 'measurement',
    'y_label': 'response'
  }]
})) AS chart;
```

## Marker shape and size

`marker` is per group. `marker_size`, `marker_opacity` and `marker_stroke_width` can be set per group, and
the first two also have a top-level default that applies to every group that does not override it.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys, 'color': '#59a14f', 'marker': 'square'}],
    'marker_size': 5,
    'marker_opacity': 0.7,
    'marginal_type': 'density',
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## Per-point colours

`colors` on a group colours each marker individually. The marginal panels still use the group's uniform
`color`, so give one as well if you want them to match.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys, 'colors': cs}],
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys,
         list(CASE WHEN y > 5.5 THEN '#4e79a7'
                   WHEN y > 4.5 THEN '#59a14f'
                   ELSE '#e15759' END ORDER BY x, y) AS cs
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## Tooltips

`tooltips` turns on hover `<title>` elements in the SVG output; `tooltip_labels` supplies the text per
point (one string per point, in the same order as the group's x / y).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'tooltips': true,
    'tooltip_labels': tips,
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys,
         list('(' || round(x, 2) || ', ' || round(y, 2) || ')' ORDER BY x, y) AS tips
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## Panel sizing

| Field | Effect |
| --- | --- |
| `marginal_size` | Panel thickness in pixels (default `80`) |
| `marginal_gap` | Gap between a panel and the scatter (default `4`) |
| `bins` | Histogram bin count (default `20`) |
| `marginal_alpha` | Bar / fill opacity (default `0.6`) |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'marginal_size': 120,
    'marginal_gap': 8,
    'bins': 30,
    'marginal_alpha': 0.5,
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One scatter layer per group (see below). |
| `marginal_type` | string | `"histogram"` (default) or `"density"`. |
| `show_top` | boolean | Draw the top marginal (default on). |
| `show_right` | boolean | Draw the right marginal (default on). |
| `marginal_size` | number | Thickness of the marginal panels, in pixels. |
| `marginal_gap` | number | Gap between the marginals and the main plot, in pixels. |
| `bins` | integer | Histogram bin count (at least 1). |
| `bandwidth` | number | Density bandwidth (when `marginal_type` is `"density"`). |
| `marginal_alpha` | number | Marginal fill opacity. |
| `x_label` / `y_label` | string | Axis labels for the main plot. |
| `marker_size` | number | Shared point radius. |
| `marker_opacity` | number | Shared point opacity. |
| `tooltips` | boolean | Hover tooltips. |
| `tooltip_labels` | string[] | One tooltip per point. |

Each entry of `groups` carries `x`, `y` (both required and the same length), plus:

| Group field | Type | What it sets |
| --- | --- | --- |
| `label` | string | The legend entry. |
| `color` | string | Marker colour. |
| `marker` | string | Marker shape. |
| `sizes` | number[] | Per-point radii (a bubble plot). |
| `colors` | string[] | Per-point colours. |
| `x_err` / `y_err` | (number \| `[number, number]`)[] | Per-point error bars; symmetric as a number, asymmetric as `[neg, pos]`. |
| `marker_size` / `marker_opacity` / `marker_stroke_width` | number | This group's marker style; overrides the top-level default. |
| `trend` | boolean | Overlay a least-squares line. |
| `equation` / `correlation` | boolean | Annotate the fit statistics. |

## Notes

- **Each group's `x` and `y` must be the same length**, and a group cannot be empty; `bins` must be at
  least 1 (0 would divide by zero when normalising).
- `sizes` / `colors` on a group must match the group's point count.
- `x_err` / `y_err`, when given, must match the group's point count; a mismatch is an error rather than a
  silent truncation.
- A group's `marker_size` / `marker_opacity` override the top-level ones; `marker_stroke_width` has no
  top-level counterpart.
- The figure-level `x_axis` / `y_axis` do not label a joint plot; use `x_label` / `y_label`.

## See also

- [kuva — Joint plot](https://psy-fer.github.io/kuva/plots/jointplot.html) — the plotting library's own reference for this chart.
- [Scatter plot](./scatter.md) — the scatter on its own.
- [2D histogram](../distributions/histogram2d.md) — a binned view of the same cloud.
