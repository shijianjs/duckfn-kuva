---
title: Band plot
sidebar_position: 4
description: A shaded interval between an upper and a lower boundary — a confidence band or a range.
---

# Band plot

A band plot fills the region between two y-curves over a shared x axis. Use it for a **confidence
interval**, a prediction band, an IQR envelope, or any shaded range around a central estimate.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': [
    {'type': 'band', 'x': xs, 'y_lower': los, 'y_upper': ups,
     'color': 'steelblue', 'opacity': 0.25, 'legend': '±0.5'},
    {'type': 'line', 'color': 'steelblue', 'stroke_width': 2, 'data': pts, 'legend': 'Condition_A'}
  ]
})) AS chart
FROM (
  SELECT list(time ORDER BY time) AS xs,
         list(value - 0.5 ORDER BY time) AS los,
         list(value + 0.5 ORDER BY time) AS ups,
         array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

A band is a series like any other, so it is drawn **in the order it appears** in `series`. Put it before
the line that goes on top of it.

## Attached to a line

Instead of a separate `band` series, a [`line`](./line.md) can carry a `band` field. The band is then
aligned to the line's own x positions and inherits its colour — one call instead of two series.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Band attached to a line',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'line',
    'color': 'firebrick',
    'stroke_width': 2,
    'data': array_agg([time, value] ORDER BY time),
    'band': {
      'lower': array_agg(value - 0.5 ORDER BY time),
      'upper': array_agg(value + 0.5 ORDER BY time)
    }
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

## Attached to a scatter plot

The same field works on a [`scatter`](./scatter.md): the band renders behind the points.

```sql {"type":"duckfn","show":"svg"}
WITH t AS (
  SELECT (i * 0.5)::DOUBLE AS x, (i * 0.5)::DOUBLE * 2 + 1 AS y
  FROM (SELECT unnest(range(0, 21)) AS i)
)
SELECT kuva_render(to_json({
  'title': 'Band attached to a scatter plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'seagreen',
    'size': 5,
    'data': (SELECT array_agg([x, y] ORDER BY x) FROM t),
    'band': {
      'lower': (SELECT array_agg(y - 1.5 ORDER BY x) FROM t),
      'upper': (SELECT array_agg(y + 1.5 ORDER BY x) FROM t)
    }
  }]
})) AS chart;
```

## Multiple series with bands

Each series carries its own independent band; the bands stack up under their own line.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Multiple series with bands',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'line',
    'legend': g,
    'color': CASE g WHEN 'Condition_A' THEN 'steelblue' ELSE 'darkorange' END,
    'stroke_width': 2,
    'data': pts,
    'band': {'lower': los, 'upper': ups}
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g,
         array_agg([time, value] ORDER BY time) AS pts,
         array_agg(value - 0.25 ORDER BY time) AS los,
         array_agg(value + 0.25 ORDER BY time) AS ups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" IN ('Condition_A', 'Condition_B')
  GROUP BY "group"
);
```

## Opacity

`opacity` sets the fill transparency. The default `0.2` is deliberately light so that overlapping bands
and the line underneath stay readable.

| `opacity` | Effect |
| --- | --- |
| `0.1`–`0.2` | Light; line and overlapping bands visible **(default `0.2`)** |
| `0.3`–`0.5` | Moderate; the band is prominent |
| `1.0` | Fully opaque; hides anything behind it |

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `x` | number[] | **Required.** The shared x positions (at least 2). |
| `y_lower` | number[] | **Required.** The lower boundary; same length as `x`. |
| `y_upper` | number[] | **Required.** The upper boundary; same length as `x`. |
| `color` | string | Fill colour. |
| `opacity` | number | Fill opacity (must not be negative). |
| `legend` | string | The legend entry. |

The `type` also accepts the alias `"interval"`.

## Notes

- **`x` needs at least two entries**, and `y_lower` / `y_upper` must match its length — a mismatch is an
  error rather than a silent truncation.
- **`opacity` must not be negative** (a negative value would build an invalid colour string).
- The standalone band has no bound: three parallel lists define the shape. That is what makes it usable
  for an arbitrary envelope, not just a symmetric one.

## See also

- [kuva — Band plot](https://psy-fer.github.io/kuva/plots/band.html) — the plotting library's own reference for this chart.
- [Line plot](./line.md) — the centre line for the band.
- [Scatter plot](./scatter.md) — per-point `band` for a local interval.
