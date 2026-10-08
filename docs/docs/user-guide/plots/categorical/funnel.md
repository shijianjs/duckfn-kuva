---
title: Funnel chart
sidebar_position: 4
description: Stage-by-stage conversion, with connectors, percentages and a mirrored drop-off side.
---

# Funnel chart

A funnel chart shows how a quantity shrinks through ordered stages — the classic conversion funnel. It
can also draw a back-to-back "drop-off" side with `mirror`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'funnel',
    'stages': list({'label': stage, 'value': n_screened}),
    'show_values': true,
    'show_percents': true,
    'show_conversion': true,
    'color_mode': 'gradient'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `stages` | stage[] | **Required.** One entry per stage, in order: `{label, value, color?}`. |
| `mirror` | stage[] | A second set of stages drawn back-to-back (a diverging funnel). |
| `left_label` / `right_label` | string | Labels for the two sides in mirror mode. |
| `orientation` | string | `"vertical"` (default) or `"horizontal"`. |
| `show_connectors` | boolean | Draw the bands between stages. |
| `connector_opacity` | number | Connector opacity. |
| `show_values` | boolean | Print each stage's value. |
| `show_percents` | boolean | Print each stage as a percentage of the first. |
| `show_conversion` | boolean | Print the conversion rate between stages. |
| `color_mode` | string | `"uniform"` (default) · `"by_stage"` · `"gradient"`. |
| `stage_gap` | number | Gap between stages. |
| `legend` | string | The legend title. |

## Notes

- **`stages` must not be empty**, and **not every stage may be 0** — an all-zero funnel would render
  blank and is reported as an error.
- `mirror` stages live only on the mirrored side; they are not part of the main funnel.

## See also

- [kuva — Funnel chart](https://psy-fer.github.io/kuva/plots/funnel.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) · [Waterfall](../time-series/waterfall.md) — other stage-by-stage views.
