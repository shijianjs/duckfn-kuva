---
title: Band plot
sidebar_position: 4
description: A shaded interval between an upper and a lower boundary — a confidence band or a range.
---

# Band plot

A band plot fills the area between two boundaries. Use it for a confidence interval, an error range, or
any "normal range" band; overlay a [line](./line.md) on top for the central estimate.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'band',
    'x': xs, 'y_lower': los, 'y_upper': ups,
    'color': 'steelblue', 'opacity': 0.3, 'legend': '±0.5'
  }]
})) AS chart
FROM (
  SELECT list(time ORDER BY time) AS xs,
         list(value - 0.5 ORDER BY time) AS los,
         list(value + 0.5 ORDER BY time) AS ups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

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

## See also

- [kuva — Band plot](https://psy-fer.github.io/kuva/plots/band.html) — the plotting library's own reference for this chart.
- [Line plot](./line.md) — the centre line for the band.
- [Scatter plot](./scatter.md) — per-point `band` for a local interval.
