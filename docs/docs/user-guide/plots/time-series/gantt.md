---
title: Gantt chart
sidebar_position: 7
description: Task bars on a time axis, with phases, progress fills, milestones and a now line.
---

# Gantt chart

A Gantt chart draws tasks as horizontal bars spanning a time range, which makes schedule, duration, sequence
and overlap readable at a glance.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Project plan',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end"} ORDER BY start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

Labels are drawn inside a bar when there is room and to its right when there is not; the right margin grows
to fit them, so an outside label is never clipped.

## Groups and phases

Give a task a `group` and it lands under a shaded header row, coloured from the palette. Tasks within a
group keep their list order, so the query controls both the phase order and the order inside a phase.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Phased plan',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks,
    'group_order': ['Design', 'Build', 'Launch']
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end",
               'group': phase,
               'progress': progress,
               'milestone': CASE WHEN milestone IN ('yes', 'true', '1') THEN TRUE ELSE FALSE END}
              ORDER BY phase, start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

`group_order` fixes the phase order explicitly; groups not listed follow afterwards in first-seen order.
Without it, phases appear in the order the tasks arrive — which is why the example sorts by `phase` first.

## Progress fills and the now line

`progress` (clamped to `0`–`1`) adds a darker inner fill showing how much of a task is done, which is what
turns a plan into a status report. `now_line` draws a dashed vertical line at a given x — the current
period — so the reader can see what *should* be finished by now.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Progress at week 6',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks,
    'group_order': ['Design', 'Build', 'Launch'],
    'now_line': 6
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end",
               'group': phase,
               'progress': progress,
               'milestone': CASE WHEN milestone IN ('yes', 'true', '1') THEN TRUE ELSE FALSE END}
              ORDER BY phase, start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

## Milestones

`milestone: true` turns a task into a diamond at a single point in time rather than a bar — for a sign-off,
a code freeze, a launch. Give it `start == end` at the moment it happens, and its label is drawn in bold to
the right.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With milestones',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks,
    'group_order': ['Design', 'Build', 'Launch'],
    'now_line': 6,
    'milestone_size': 9
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end",
               'group': phase,
               'progress': progress,
               'milestone': CASE WHEN milestone IN ('yes', 'true', '1') THEN TRUE ELSE FALSE END}
              ORDER BY phase, start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

## Sizing and colours

| Field | Default | What it sets |
| --- | --- | --- |
| `bar_height` | `0.6` | Bar thickness as a share of the row height, clamped to `[0.1, 1]` |
| `milestone_size` | `7` | Diamond half-size, in pixels |
| `show_labels` | `true` | Draw the task and milestone labels |
| `color` | `steelblue` | The bar colour when there are no groups |
| `group_bg` | `#ebebeb` | The group header row's background |

A per-task `color` overrides both the group colour and the default, which is how one late task is singled
out without giving it a phase of its own.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Taller bars, tighter rows',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks,
    'group_order': ['Design', 'Build', 'Launch'],
    'bar_height': 0.85,
    'group_bg': '#f0f0f5'
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end",
               'group': phase,
               'progress': progress,
               'milestone': CASE WHEN milestone IN ('yes', 'true', '1') THEN TRUE ELSE FALSE END}
              ORDER BY phase, start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `tasks` | task[] | **Required.** Each entry is `{label, start, end, group?, progress?, color?, milestone?}`. |
| `now_line` | number | Draw the "now" line at this x. |
| `group_order` | string[] | Explicit phase order; unlisted groups follow. |
| `bar_height` | number | Bar thickness as a share of the row (default `0.6`). |
| `milestone_size` | number | Diamond half-size in pixels (default `7`). |
| `show_labels` | boolean | Draw the labels (default on). |
| `color` | string | Default bar colour when there are no groups. |
| `group_bg` | string | Group header background (default `#ebebeb`). |
| `legend` | string | Legend label for the series. |

## Notes

- **`tasks` must not be empty**, and every task needs `label`, `start` and `end`.
- Times are plain numbers — days, weeks, months — anything you like, as long as one unit is one x unit.
- A task with `milestone: true` should have `start == end`; a non-zero span draws a diamond at the start
  and ignores the end.
- `group_order` does not filter: a group present in the tasks but missing from the list still gets a
  header, after the listed ones.
- Rows appear in list order within a group, so the query's `ORDER BY` is the schedule.

## See also

- [kuva — Gantt chart](https://psy-fer.github.io/kuva/plots/gantt.html) — the plotting library's own reference for this chart.
- [Calendar](./calendar.md) — daily activity rather than scheduled tasks.
- [Candlestick](./candlestick.md) — another chart on a period axis.
