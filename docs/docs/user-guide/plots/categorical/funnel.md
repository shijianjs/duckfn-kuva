---
title: Funnel chart
sidebar_position: 4
description: Ordered stages shrinking stage by stage, with optional diverging arms.
---

# Funnel chart

A funnel chart shows how a value **attrits through ordered stages**: one bar per stage, widths proportional
to the stage values, with trapezoidal connectors between them making the drop-off explicit.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'CONSORT flow',
  'series': [{
    'type': 'funnel',
    'stages': stages
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

Stages are drawn top to bottom in list order, so the order you aggregate is the order of the funnel. The
first stage is the reference: every percentage and conversion rate on the chart is measured against it.

## Colour modes

| `color_mode` | Bars |
| --- | --- |
| `"uniform"` | All the same colour **(default)** |
| `"by_stage"` | A distinct palette colour per stage |
| `"gradient"` | Progressively darker from top to bottom |

A stage's own `color` overrides the mode for that bar, which is how you highlight one specific step.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Gradient',
  'series': [{
    'type': 'funnel',
    'stages': stages,
    'color_mode': 'gradient',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

## Horizontal orientation

`orientation: "horizontal"` turns the funnel on its side, which suits short stage labels and wide figures.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal funnel',
  'series': [{
    'type': 'funnel',
    'stages': stages,
    'orientation': 'horizontal',
    'color_mode': 'by_stage',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

## Diverging mode

`mirror` adds a second set of stages back to back with the first — the classic treatment-vs-control
comparison. `left_label` and `right_label` name the two sides.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv'))
SELECT kuva_render(to_json({
  'title': 'Treatment vs control',
  'series': [{
    'type': 'funnel',
    'stages': (SELECT list({'label': stage, 'value': n_screened}) FROM d),
    'mirror': (SELECT list({'label': stage, 'value': n_placebo}) FROM d),
    'left_label': 'Treatment',
    'right_label': 'Control',
    'color_mode': 'by_stage'
  }]
})) AS chart;
```

Both sides share the same stage list, so the two arms line up row for row. Give them different numbers of
stages and the shorter one simply runs out.

## Labels and connectors

| Field | Default | What it sets |
| --- | --- | --- |
| `show_values` | `true` | The absolute value on each bar |
| `show_percents` | `false` | The share of the first stage, next to the value |
| `show_conversion` | `true` | The step-to-step rate in the connector areas |
| `show_connectors` | `true` | Draw the trapezoidal connectors at all |
| `connector_opacity` | `0.4` | Connector fill opacity |
| `stage_gap` | `4` | Gap between adjacent bars, in pixels |

Turning the values off and leaving the conversion rates on gives a minimal funnel that reads as pure
percentages:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Conversion rates only',
  'series': [{
    'type': 'funnel',
    'stages': stages,
    'show_values': false,
    'show_percents': false,
    'show_conversion': true,
    'color_mode': 'gradient',
    'stage_gap': 8
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `stages` | stage[] | **Required.** One entry per stage, in order: `{label, value, color?}`. |
| `mirror` | stage[] | A second set of stages drawn back to back — diverging mode. |
| `left_label` / `right_label` | string | Side labels in diverging mode. |
| `orientation` | string | `"vertical"` (default) or `"horizontal"`. |
| `color_mode` | string | `"uniform"` (default) · `"by_stage"` · `"gradient"`. |
| `show_connectors` | boolean | Draw the trapezoidal connectors (default on). |
| `connector_opacity` | number | Connector fill opacity (default `0.4`). |
| `show_values` | boolean | Value label on each bar (default on). |
| `show_percents` | boolean | Percentage-of-first-stage label (default off). |
| `show_conversion` | boolean | Step-to-step conversion rate (default on). |
| `stage_gap` | number | Gap between adjacent bars, in pixels (default `4`). |
| `legend` | string | Any non-empty value turns the legend on. |

## Notes

- **`stages` must not be empty**, and the values must be ordered widest first — a later stage larger than
  its predecessor draws a funnel that widens, which is usually a data error rather than a chart style.
- All-zero stages are an error (there is no reference width to scale against).
- `show_percents` measures against the **first** stage, while `show_conversion` measures against the
  previous one — they answer different questions.
- A stage's `color` wins over `color_mode`.

## See also

- [kuva — Funnel chart](https://psy-fer.github.io/kuva/plots/funnel.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) — a simpler categorical comparison.
- [Pie chart](./pie.md) — a static share rather than a sequence.
