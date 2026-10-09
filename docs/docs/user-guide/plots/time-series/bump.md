---
title: Bump chart
sidebar_position: 8
description: How each series' rank moves across time points.
---

# Bump chart

A bump chart follows the **rank** of each series across discrete time points or conditions. Lines connect
consecutive ranks and the best rank sits at the top, so crossings are the story: a line crossing another is
one series overtaking another.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv'))
SELECT kuva_render(to_json({
  'title': 'Rank over time',
  'series': [{
    'type': 'bump',
    'series': (SELECT list({'name': series, 'ranks': rs} ORDER BY series)
               FROM (SELECT series, list(rank ORDER BY time) AS rs FROM d GROUP BY series)),
    'x_labels': (SELECT list(CAST(time AS VARCHAR) ORDER BY time)
                 FROM (SELECT DISTINCT time FROM d))
  }]
})) AS chart;
```

Each series carries a `ranks` list with one entry per time point, and `x_labels` names the points. The two
must be the same length — they are paired by position.

## Auto-ranking from raw values

Instead of pre-computed ranks, a series can carry `values` and let the chart rank them at each time point.
That is the honest mode when the input really is a score rather than a standing.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv'))
SELECT kuva_render(to_json({
  'title': 'Ranked from values',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'values': [95, 80, 88, 92]},
      {'name': 'Beta',  'values': [80, 95, 72, 86]},
      {'name': 'Gamma', 'values': [70, 85, 95, 78]}
    ],
    'x_labels': ['Q1', 'Q2', 'Q3', 'Q4'],
    'tie_break': 'average'
  }]
})) AS chart;
```

By default a **higher value ranks first**. `rank_ascending: true` flips it, for the measures where less is
better — race times, error counts, latency.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Lower is better',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'values': [12.4, 11.8, 11.9, 11.2]},
      {'name': 'Beta',  'values': [13.1, 12.6, 11.5, 10.9]},
      {'name': 'Gamma', 'values': [11.9, 12.2, 12.0, 11.6]}
    ],
    'x_labels': ['Q1', 'Q2', 'Q3', 'Q4'],
    'rank_ascending': true
  }]
})) AS chart;
```

## Highlight mode

`highlight` names one series to emphasise: it gets a heavier stroke and bolder endpoint labels while
everything else drops to 20 % opacity. It is the standard way to make a bump chart about one subject
without deleting the context.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv'))
SELECT kuva_render(to_json({
  'title': 'Highlighting one series',
  'series': [{
    'type': 'bump',
    'series': (SELECT list({'name': series, 'ranks': rs} ORDER BY series)
               FROM (SELECT series, list(rank ORDER BY time) AS rs FROM d GROUP BY series)),
    'x_labels': (SELECT list(CAST(time AS VARCHAR) ORDER BY time)
                 FROM (SELECT DISTINCT time FROM d)),
    'highlight': 'Beta'
  }]
})) AS chart;
```

## Missing time points

A `null` in `ranks` means the series was absent at that point, and the line breaks rather than interpolating
through it — which is the difference between "not measured" and "ranked last".

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With gaps',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'ranks': [1, NULL, 2, 1]},
      {'name': 'Beta',  'ranks': [2, 1, 1, 3]},
      {'name': 'Gamma', 'ranks': [3, 2, NULL, 2]}
    ],
    'x_labels': ['A', 'B', 'C', 'D']
  }]
})) AS chart;
```

## Tie-breaking

When auto-ranking finds equal values, `tie_break` decides the ranks:

| `tie_break` | Ranks awarded |
| --- | --- |
| `"average"` | The average of the positions they occupy (e.g. `2.5`, `2.5`) **(default)** |
| `"min"` | All of them get the best rank |
| `"max"` | All of them get the worst rank |
| `"stable"` | They keep their input order |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Ties, broken by minimum',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'values': [90, 90, 80]},
      {'name': 'Beta',  'values': [90, 70, 95]},
      {'name': 'Gamma', 'values': [60, 90, 85]}
    ],
    'x_labels': ['Q1', 'Q2', 'Q3'],
    'tie_break': 'min'
  }]
})) AS chart;
```

Average ranks are fractional, which is why `ranks` accepts decimals — `2.5` is a legitimate pre-ranked
value meaning "tied for second".

## Curve style and labels

| Field | Default | What it sets |
| --- | --- | --- |
| `curve_style` | `"sigmoid"` | `"sigmoid"` curves through the point; `"straight"` joins them with a line |
| `show_rank_labels` | `false` | Write the rank number inside each dot |
| `show_series_labels` | `true` | Series names at the left and right edges |
| `dot_radius` | `6` | Dot radius in pixels |
| `stroke_width` | `2.5` | Line width in pixels |
| `legend` | `true` | Show the legend |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Straight lines, rank labels',
  'series': [{
    'type': 'bump',
    'series': [
      {'name': 'Alpha', 'ranks': [1, 3, 2, 1]},
      {'name': 'Beta',  'ranks': [2, 1, 1, 3]},
      {'name': 'Gamma', 'ranks': [3, 2, 3, 2]}
    ],
    'x_labels': ['2021', '2022', '2023', '2024'],
    'curve_style': 'straight',
    'show_rank_labels': true,
    'dot_radius': 8,
    'stroke_width': 2
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `series` | series[] | **Required.** Each entry is `{name, ranks?, values?, color?}`. |
| `ranks` | (number \| null)[] | Pre-computed ranks; `null` breaks the line. |
| `values` | (number \| null)[] | Raw values, ranked automatically; `null` breaks the line. |
| `x_labels` | string[] | One label per time point. |
| `curve_style` | string | `"sigmoid"` (default) or `"straight"`. |
| `rank_ascending` | boolean | Lower values rank first (default off). |
| `tie_break` | string | `"average"` (default) · `"min"` · `"max"` · `"stable"`. |
| `highlight` | string | Emphasise one series by name; the rest are muted. |
| `show_rank_labels` / `show_series_labels` | boolean | Rank numbers in the dots, series names at the edges. |
| `dot_radius` / `stroke_width` | number | Dot and line sizes. |
| `legend` | boolean | Show the legend (default on). |

## Notes

- **`series` must not be empty**, and every entry needs a `name` plus either `ranks` or `values`.
- Do not mix the two within one chart: `values` are ranked **within the group that gave values**, so a
  series with `ranks` and a series with `values` are on different scales.
- Use `null` for an absent time point, not `0` — `0` is a rank, and it would sort above rank 1.
- `x_labels` must cover every time point; a shorter list leaves the extra points unlabelled.
- The best rank is at the **top**, and `rank_ascending` only changes which raw values get the low numbers —
  it does not flip the axis.

## See also

- [kuva — Bump chart](https://psy-fer.github.io/kuva/plots/bump.html) — the plotting library's own reference for this chart.
- [Slope](../categorical/slope.md) — the two-time-point version.
- [Parallel coordinates](../relationships/parallel.md) — ranks across more than a time axis.
