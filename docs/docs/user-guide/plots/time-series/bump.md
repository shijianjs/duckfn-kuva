---
title: Bump chart
sidebar_position: 8
description: Rank over time drawn as crossing lines, from known ranks or raw values.
---

# Bump chart

A bump chart draws each series' **rank** over time, so crossings show who overtook whom. Give it known
`ranks`, or raw `values` and let it rank them.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'bump',
    'series': ser,
    'x_labels': xl,
    'show_rank_labels': true,
    'show_series_labels': true
  }]
})) AS chart
FROM (
  SELECT
    (SELECT list({'name': s, 'ranks': rk}) FROM (
      SELECT "series" AS s, list(rank ORDER BY time) AS rk
      FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv')
      GROUP BY "series"
    )) AS ser,
    (SELECT list(CAST(t AS VARCHAR) ORDER BY t) FROM (
      SELECT DISTINCT time AS t FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv')
    )) AS xl
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `series` | series[] | **Required.** One entry per line: `{name, ranks?, values?, color?}`. |
| `x_labels` | string[] | One label per time step. |
| `curve_style` | string | `"sigmoid"` (default) or `"straight"`. |
| `show_rank_labels` | boolean | Print each point's rank. |
| `show_series_labels` | boolean | Print the series name at the line's end. |
| `dot_radius` | number | Point radius. |
| `stroke_width` | number | Line width. |
| `highlight` | string | Draw only this one series in an accent colour. |
| `legend` | boolean | Show the legend. |
| `rank_ascending` | boolean | Rank 1 at the top (default). |
| `tie_break` | string | How ties are ranked: `"average"` · `"min"` · `"max"` · `"stable"`. |

Each series gives **either** `ranks` (known ranks; a `null` at a step means it was absent and the line
breaks) **or** `values` (raw values, ranked automatically).

## Notes

- **Bump charts support at most 10 series** — the built-in palette indexes ten colours without wrapping,
  so an 11th is reported as an error.
- Give a series `ranks` or `values`, not neither; mixing the two forms across series is allowed (only the
  ones with `values` are ranked together).
- Rank 1 is at the top unless `rank_ascending` is `false`.

## See also

- [kuva — Bump chart](https://psy-fer.github.io/kuva/plots/bump.html) — the plotting library's own reference for this chart.
- [Slope chart](../categorical/slope.md) — a single before/after step.
- [Line plot](../relationships/line.md) — the values themselves rather than their ranks.
