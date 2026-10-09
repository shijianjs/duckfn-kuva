---
title: Survival curve
sidebar_position: 3
description: Kaplan–Meier curves for time-to-event data, with censoring, bands and a p-value.
---

# Survival curve

A Kaplan–Meier plot shows the probability of remaining event-free over time. Each subject contributes one
observation: either the time the event happened, or the time of last follow-up for a censored subject who
did not have the event. It is the standard tool for time-to-event outcomes in trials and epidemiology.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group"))
  }]
})) AS chart;
```

A group takes parallel `times` and `events` lists; `event = 1` becomes the `true` the field expects, so the
long-format table is pivoted and typed in one pass. Tick marks on the curves are the censored
observations — the subjects still event-free at their last follow-up.

## Several arms, with a p-value

One group per arm. The log-rank p-value is **not** computed here: kuva renders the string you give it, so
the test has to be run in SQL (or elsewhere) and the result passed in. That is deliberate — a chart that
invents a p-value is worse than one without.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group")),
    'pvalue_text': 'log-rank p = 0.031',
    'legend': 'arm'
  }]
})) AS chart;
```

## Confidence bands

`ci` overlays the Greenwood 95 % band around each curve and `ci_alpha` sets its opacity. Bands that overlap
heavily are the visual version of "this difference is not significant", which is worth looking at before
quoting any p-value.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group")),
    'ci': true,
    'ci_alpha': 0.15,
    'pvalue_text': 'p < 0.001',
    'legend': 'arm'
  }]
})) AS chart;
```

## Colours

`colors` assigns one colour per group, by position. Give the arms colours that carry meaning — treatment
versus control, high versus low — rather than letting the palette decide, because the palette order depends
on how the groups were sorted.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group")),
    'colors': ['#2ca02c', '#d62728'],
    'ci': true,
    'legend': 'arm'
  }]
})) AS chart;
```

## Styling

| Field | Default | What it sets |
| --- | --- | --- |
| `line_width` | `2` | Curve stroke width |
| `censoring` | `true` | Draw the censoring tick marks |
| `censoring_size` | `4` | Half-height of those ticks, in pixels |
| `ci` / `ci_alpha` | `false` / `0.2` | The Greenwood band and its opacity |
| `pvalue_text` | — | A string drawn in the upper-right corner |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group")),
    'censoring': false,
    'line_width': 3,
    'legend': 'arm'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One entry per arm: `{label, times, events, color?}`. |
| `times` | number[] | Each subject's follow-up time. |
| `events` | boolean[] | `true` = the event occurred, `false` = censored. |
| `colors` | string[] | Per-group colours, matched by position. |
| `line_width` | number | Curve stroke width (default `2`). |
| `ci` / `ci_alpha` | boolean / number | The Greenwood 95 % band and its opacity. |
| `censoring` / `censoring_size` | boolean / number | The censoring ticks. |
| `pvalue_text` | string | Free text drawn in the corner — kuva does not compute it. |
| `legend` | string | Legend title; one entry per group. |

## Notes

- **`groups` must not be empty**, and within each group `times` and `events` must be the same length.
- Event times are numeric — months, days, cycles — whatever one x unit means.
- `pvalue_text` is a **string**: the log-rank test is not run for you. Compute it in SQL and pass the
  formatted result, so the number on the chart is traceable.
- A censored subject contributes a tick, not a drop; if the ticks are hidden, say so in the caption.
- `colors` is matched by position, so it has to line up with the order the groups come out of the query.

## See also

- [kuva — Survival curve](https://psy-fer.github.io/kuva/plots/survival.html) — the plotting library's own reference for this chart.
- [Forest](./forest.md) — pooled effect estimates instead of time-to-event curves.
- [ROC](./roc.md) — another step-function statistical curve.
