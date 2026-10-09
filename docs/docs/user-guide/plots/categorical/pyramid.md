---
title: Population pyramid
sidebar_position: 6
description: Back-to-back horizontal bars, one row per age group, two sides to compare.
---

# Population pyramid

A population pyramid is a back-to-back horizontal bar chart: one row per age group, one side for each
demographic. The symmetric layout makes the two age distributions comparable at a glance.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Population pyramid',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [{'label': 'census', 'groups': groups}]
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

`left_label` and `right_label` print above the two sides. Age groups run bottom to top in the order given,
which for a pyramid is usually youngest first.

## Normalised mode

`normalize` expresses every bar as a percentage of the total, which makes the left-right comparison
scale-invariant — the only way to compare two populations of different sizes fairly.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Normalised (%)',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [{'label': 'census', 'groups': groups}],
    'normalize': true,
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

## Comparing two series

Give more than one entry in `series` and each gets its own sub-band inside every age group — the natural
way to put two census years, or two regions, side by side.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Two series',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [
      {'label': 'group A', 'groups': groups, 'color': '#4c72b0'},
      {'label': 'group B', 'groups': groups2, 'color': '#dd8452'}
    ],
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups,
         list({'age': age, 'left': female * 1.15, 'right': male * 1.15}) AS groups2
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

The age groups come from the **first** series, so the later ones must list the same labels in the same
order — they are drawn against that shared row structure.

## Overlap mode

`mode: "overlap"` draws each series as translucent bars **on top of** each other instead of side by side.
It is for the two-series case where the question is how one profile sits *within* another.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Overlap mode',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [
      {'label': 'group A', 'groups': groups,  'opacity': 0.6},
      {'label': 'group B', 'groups': groups2, 'opacity': 0.6}
    ],
    'mode': 'overlap',
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups,
         list({'age': age, 'left': female * 1.15, 'right': male * 1.15}) AS groups2
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `series` | series[] | **Required.** One entry per series: `{label, groups, color?, opacity?}`. |
| `left_label` / `right_label` | string | Labels above the two sides (default `"Left"` / `"Right"`). |
| `left_color` / `right_color` | string | Bar colours for a single series (default `#4C72B0` / `#DD8452`). |
| `normalize` | boolean | Express values as a percentage of the total. |
| `show_values` | boolean | Write the value at each bar's tip. |
| `group_gap` | number | Blank space between rows, as a fraction of the row height. |
| `bar_gap` | number | Gap between sub-bands in grouped mode. |
| `mode` | string | `"grouped"` (default) or `"overlap"`. |
| `show_legend` | boolean | Show one legend entry per series. |

Each entry of a series' `groups` is `{age, left, right}`.

## Notes

- **`series` must not be empty**, and every series needs at least one group.
- Age groups are taken from the **first** series; a later series with different labels does not add rows.
- `normalize` measures each **side** against its own side's total, so the two halves each add up to 100 %.
- `opacity` only matters in overlap mode; in grouped mode the series are separated by `bar_gap` anyway.

## See also

- [kuva — Population pyramid](https://psy-fer.github.io/kuva/plots/pyramid.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) — a one-directional categorical comparison.
- [Waffle](./waffle.md) — another proportion-focused layout.
