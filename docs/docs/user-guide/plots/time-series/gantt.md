---
title: Gantt chart
sidebar_position: 7
description: Task bars over time, with grouped headers, progress fills and milestones.
---

# Gantt chart

A Gantt chart lays tasks out as bars along a time axis, grouped under headers, with an optional progress
fill and diamond milestones.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'series': [{
    'type': 'gantt',
    'tasks': list({'label': task, 'start': start, 'end': "end", 'group': phase,
                   'progress': progress, 'milestone': (milestone = 'yes')}),
    'bar_height': 0.7,
    'show_labels': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `tasks` | task[] | **Required.** One entry per task: `{label, start, end, group?, progress?, color?, milestone?}`. |
| `now_line` | number | Draw a "today" line at this time value. |
| `group_order` | string[] | The order of the group headers. |
| `bar_height` | number | Bar thickness as a fraction of the row height, clamped to `[0.1, 1]`. |
| `milestone_size` | number | Size of the milestone markers. |
| `show_labels` | boolean | Label the tasks. |
| `color` | string | Default bar colour. |
| `group_bg` | string | Background colour of the group headers. |
| `legend` | string | The legend title. |

## Notes

- **`tasks` must not be empty**, and a non-milestone task must not end before it starts.
- For a milestone, give `milestone: true` and set `end` equal to `start`.
- `start` / `end` are plain numbers — kuva does not interpret them, so pick a consistent unit.

## See also

- [kuva — Gantt chart](https://psy-fer.github.io/kuva/plots/gantt.html) — the plotting library's own reference for this chart.
- [Slope chart](../categorical/slope.md) — before/after comparisons without a time axis.
