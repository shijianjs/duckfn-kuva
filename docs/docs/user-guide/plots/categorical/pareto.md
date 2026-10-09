---
title: Pareto chart
sidebar_position: 5
description: Sorted bars with a cumulative-percentage line on a fixed 0–100 % axis.
---

# Pareto chart

A Pareto chart is a bar chart of category values, sorted descending, with a cumulative-percentage line on a
secondary axis fixed at 0–100 %. A dashed reference line at 80 % by default shows how many categories
account for the bulk of the total — the "80/20 rule" chart, common in QC, variant analysis and
error-categorisation work.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Support ticket error categories',
  'series': [{
    'type': 'pareto',
    'categories': cats
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

Categories are sorted descending by value before drawing (`"sorted": false` keeps your order instead). A
legend for "Value" / "Cumulative %" is on by default, because the bars and the line are two encodings that
always come together.

The secondary axis is a percentage **by construction** — there is nothing to configure — and its ticks are
formatted `0%, 20%, …`. The category axis defaults to rotated, collision-thinned labels, since Pareto data
usually has more categories than a hand-built bar chart.

## Styling

`color` is the bar fill, `line_color` the cumulative line, `threshold` the reference line (setting it also
turns the line on), and `cumulative_labels` writes the percentage above each line point.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Styled Pareto',
  'series': [{
    'type': 'pareto',
    'categories': cats,
    'color': 'seagreen',
    'line_color': 'darkorange',
    'threshold': 90,
    'cumulative_labels': true,
    'bar_legend_label': 'count',
    'line_legend_label': 'cumulative %'
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

## Collapsing a long tail

Real defect logs have a long tail of one-off categories that clutter the axis. `max_categories` collapses
everything past the top `n - 1` into a single bar — but instead of summing the tail into an opaque total,
that bar is drawn as a **stack** of its constituents, each with its own legend entry, so nothing is hidden.
`other_label` renames it (default `"Other"`).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Top 5 + Other',
  'series': [{
    'type': 'pareto',
    'categories': cats,
    'max_categories': 5,
    'other_label': 'Other',
    'cumulative_labels': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

The bucket contributes exactly one point to the cumulative line, matching its one slot on the axis — so the
curve stays honest even though six categories are hidden inside one bar.

## Horizontal mode

`horizontal` puts the categories on the y axis. The cumulative line then moves to a secondary **x** axis
drawn along the top, because the secondary axis always pairs with whichever axis carries the values.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal Pareto',
  'series': [{
    'type': 'pareto',
    'categories': cats,
    'horizontal': true,
    'cumulative_labels': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `categories` | category[] | **Required.** `{label, value}` per category. Give the raw values, not the cumulative ones. |
| `color` | string | Bar fill (default `steelblue`). |
| `line_color` | string | Cumulative-line colour (default `firebrick`). |
| `width` | number | Bar width as a fraction of the category slot (default `0.8`). |
| `sorted` | boolean | Sort descending by value (default on; `false` keeps your order). |
| `cumulative_labels` | boolean | Write a `%` label at each cumulative-line point. |
| `show_threshold` | boolean | Draw the dashed reference line (default on). |
| `threshold` | number | The reference line's level, in percent (default `80`). |
| `max_categories` | integer | Collapse everything past the top `n - 1` into one stacked bar. |
| `other_label` | string | The bucket's name (default `"Other"`). |
| `bar_legend_label` / `line_legend_label` | string | Legend labels for the bars and the line. |
| `show_legend` | boolean | Draw the legend (default on). |
| `horizontal` | boolean | Categories on the y axis; the cumulative line moves to a top x axis. |

## Notes

- **The cumulative line is computed from the values you give** — pass raw counts, never pre-cumulated ones.
- `sorted` defaults to `true`; a Pareto chart's whole point is the descending order, so `false` is for cases
  where the categories already have a natural order you must keep.
- `max_categories` counts the "Other" bar as one of the `n` slots.
- All-zero values are an error (there is no total to take percentages of).

## See also

- [kuva — Pareto chart](https://psy-fer.github.io/kuva/plots/pareto.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) — plain categorical bars, no cumulative line.
- [Waterfall](../time-series/waterfall.md) — bars carrying a running total.
