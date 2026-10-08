---
title: Kaplan-Meier survival curve
sidebar_position: 3
description: Survival curves per group, with confidence bands, censoring marks and a p-value annotation.
---

# Kaplan-Meier survival curve

A survival curve estimates, for each group, the fraction still event-free over time (Kaplan-Meier). Give
each group a follow-up time and an event flag per subject.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'survival'},
  'series': [{
    'type': 'survival',
    'groups': grps,
    'ci': true,
    'censoring': true,
    'legend': 'cohort'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'times': ts, 'events': ev} ORDER BY g) AS grps
  FROM (
    SELECT "group" AS g,
           list(time ORDER BY time) AS ts,
           list(event = 1 ORDER BY time) AS ev
    FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv')
    GROUP BY "group"
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One curve per group: `{label, times, events, color?}`. |
| `colors` | string[] | Per-group colours, matched to `groups` by position. |
| `line_width` | number | Curve width. |
| `ci` | boolean | Draw a confidence band. |
| `ci_alpha` | number | Band opacity. |
| `censoring` | boolean | Mark censored observations. |
| `censoring_size` | number | Censoring mark size. |
| `pvalue_text` | string | A text line on the plot (typically a log-rank p value — computed by you, not by the chart). |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **`groups` must not be empty**, and within a group `times` and `events` must be the same length and
  non-empty — a mismatch is an error rather than a silent zip.
- **`pvalue_text` is not computed**; run your own log-rank test and pass the string in.

## See also

- [kuva — Survival curve](https://psy-fer.github.io/kuva/plots/survival.html) — the plotting library's own reference for this chart.
- [Stats box](../../reference/stats-box.md) — another place to put a computed statistic.
