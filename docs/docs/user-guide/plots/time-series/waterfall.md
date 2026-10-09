---
title: Waterfall chart
sidebar_position: 4
description: A running total built from floating bars, with subtotals and anchored comparisons.
---

# Waterfall chart

A waterfall chart shows a running total as a sequence of floating bars. Each bar starts where the previous
one ended — green for a positive increment, red for a negative one — so the reader can follow how a number
was arrived at rather than only what it is.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Enrichment by process',
  'x_axis': {'name': 'process', 'tick_rotate': 45},
  'y_axis': {'name': 'running total (log2 FC)'},
  'series': [{
    'type': 'waterfall',
    'bars': bars
  }]
})) AS chart
FROM (
  SELECT list({'label': process, 'value': log2fc} ORDER BY log2fc DESC) AS bars
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waterfall.tsv')
);
```

The running total accumulates left to right. Which order the bars appear in is the whole analysis, so sort
the list deliberately — by size here, which puts every gain together and then every loss.

## Total bars

`{"kind": "total"}` places a bar spanning from zero to the current running total, in its own colour. Its
`value` is ignored — the height *is* the accumulated total — which makes it the subtotal bar: drop one after
each section of deltas to show the intermediate figure.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With subtotals',
  'x_axis': {'name': 'step', 'tick_rotate': 45},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'waterfall',
    'bars': [
      {'label': 'Revenue',       'value': 850},
      {'label': 'Cost of goods', 'value': -340},
      {'label': 'Gross profit',  'kind': 'total'},
      {'label': 'Personnel',     'value': -180},
      {'label': 'Operations',    'value': -90},
      {'label': 'Marketing',     'value': -70},
      {'label': 'EBITDA',        'kind': 'total'},
      {'label': 'Depreciation',  'value': -40},
      {'label': 'Interest',      'value': -20},
      {'label': 'Tax',           'value': -65},
      {'label': 'Net income',    'kind': 'total'}
    ],
    'show_values': true
  }]
})) AS chart;
```

A total bar **does not reset the accumulator** — it reports it. That is what allows "gross profit" to be
followed by more deductions and an "EBITDA" bar later on.

## Connectors and value labels

`connectors` draws a dashed line from each bar's top (or bottom) to the start of the next, which is what
keeps a wide waterfall traceable. `show_values` prints each bar's number.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Connectors and values',
  'x_axis': {'name': 'quarter', 'tick_rotate': 45},
  'y_axis': {'name': 'sales'},
  'series': [{
    'type': 'waterfall',
    'bars': [
      {'label': 'Q1 sales', 'value': 420},
      {'label': 'Q2 sales', 'value': 380},
      {'label': 'Returns',  'value': -95},
      {'label': 'Discounts','value': -60},
      {'label': 'H1 net',   'kind': 'total'},
      {'label': 'Q3 sales', 'value': 410},
      {'label': 'Q4 sales', 'value': 455},
      {'label': 'Returns',  'value': -105},
      {'label': 'Discounts','value': -70},
      {'label': 'H2 net',   'kind': 'total'}
    ],
    'connectors': true,
    'show_values': true
  }]
})) AS chart;
```

## Difference bars

`{"kind": "difference", "from": a, "to": b}` draws a standalone bar anchored at two explicit levels
instead of the running total: green when `to > from`, red when `to < from`. It **does not touch the
accumulator**.

The clearest use is a comparison between two total bars, where `from` and `to` are exactly those two
heights, so the reader can trace the connection by eye.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Period over period',
  'x_axis': {'name': 'step', 'tick_rotate': 45},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'waterfall',
    'bars': [
      {'label': 'Revenue', 'value': 500},
      {'label': 'Costs',   'value': -180},
      {'label': 'Period A','kind': 'total'},
      {'label': 'Revenue', 'value': 600},
      {'label': 'Costs',   'value': -190},
      {'label': 'Period B','kind': 'total'},
      {'label': 'A to B',  'kind': 'difference', 'from': 320, 'to': 730}
    ],
    'show_values': true,
    'connectors': true
  }]
})) AS chart;
```

Here `320` and `730` are the two period totals, so the anchored bar sits exactly between them and reads as
the improvement. It is an annotation, not a step in the arithmetic — the running total after it is
unchanged.

## Custom colours

The three bar colours are all replaceable, which is worth doing when a project has its own conventions for
gain and loss.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Custom colours',
  'x_axis': {'name': 'step'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'waterfall',
    'bars': [
      {'label': 'Gain', 'value': 100},
      {'label': 'Loss', 'value': -40},
      {'label': 'Net',  'kind': 'total'}
    ],
    'color_positive': 'darkgreen',
    'color_negative': 'crimson',
    'color_total': 'navy',
    'show_values': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `bars` | bar[] | **Required.** Each entry is `{label, value?, kind?, from?, to?}`. |
| `kind` | string | `"delta"` (default) · `"total"` · `"difference"`. |
| `value` | number | The increment for `delta`; ignored for `total`. |
| `from` / `to` | number | The two levels a `difference` bar is anchored at. |
| `bar_width` | number | Bar width as a share of the slot (default `0.6`). |
| `gap` | number | Gap between bars. |
| `color_positive` / `color_negative` / `color_total` | string | The three bar colours. |
| `connectors` | boolean | Draw the dashed connectors. |
| `show_values` | boolean | Print each bar's value. |

## Notes

- **`bars` must not be empty**, and every bar needs a `label`.
- A `delta` with no `value` contributes nothing but still takes a slot; a `total` ignores `value` entirely.
- `difference` defaults into place: a bar with `from`/`to` and no `kind` is treated as a difference, and a
  bar with neither is a delta — being explicit with `kind` is worth it in generated SQL.
- The connector lines join consecutive bars in list order, including across a total bar.
- There is no automatic "final total"; add a closing `total` bar if the end state needs to stand out.

## See also

- [kuva — Waterfall chart](https://psy-fer.github.io/kuva/plots/waterfall.html) — the plotting library's own reference for this chart.
- [Bar chart](../categorical/bar.md) — a plain, non-cumulative comparison.
- [Stacked area](./stacked_area.md) — a continuous running total instead of discrete steps.
