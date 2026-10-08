---
title: Population pyramid
sidebar_position: 6
description: Two back-to-back columns of values per age group, grouped or overlapped.
---

# Population pyramid

A population pyramid draws two sets of bars back-to-back per group — classically male and female per age
band. Several series can be drawn side by side (e.g. two years) or overlaid.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'pyramid',
    'series': [{'label': '2026', 'groups': groups}],
    'left_label': 'male',
    'right_label': 'female',
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `series` | series[] | **Required.** One entry per series (e.g. per year), each `{label, groups, color?, opacity?}`. |
| `left_label` / `right_label` | string | Labels for the two sides. |
| `left_color` / `right_color` | string | Colours for the two sides. |
| `normalize` | boolean | Draw each side as a percentage of the total. |
| `show_values` | boolean | Print the values on the bars. |
| `group_gap` | number | Gap between age groups. |
| `bar_gap` | number | Gap between the two bars within a group. |
| `mode` | string | `"grouped"` (default) or `"overlap"`. |
| `show_legend` | boolean | Draw the legend. |

Each entry of `series` carries `groups`: a list of `{age, left, right}`.

## Notes

- **The age axis comes from the first series**, so every series' `groups` must line up — a different
  length is an error rather than a chart drawn past its axis.
- In `mode: "overlap"` the per-series `opacity` controls the blend.

## See also

- [kuva — Population pyramid](https://psy-fer.github.io/kuva/plots/pyramid.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) — one-sided columns.
