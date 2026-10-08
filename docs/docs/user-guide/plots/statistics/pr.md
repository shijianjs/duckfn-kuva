---
title: Precision-recall curve
sidebar_position: 2
description: Precision against recall as the threshold sweeps, with a prevalence baseline.
---

# Precision-recall curve

A precision-recall curve plots precision against recall as the threshold sweeps. It is the more
informative curve when the positive class is rare.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'model',
      'predictions': preds,
      'prevalence': 0.5,
      'auc_label': true, 'optimal_point': true
    }],
    'show_baseline': true,
    'legend': 'precision-recall'
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One curve per model (see below). |
| `show_baseline` | boolean | Draw the random baseline (a horizontal line at `prevalence`). |
| `baseline_color` | string | Baseline colour. |
| `baseline_dasharray` | string | Baseline dash pattern. |

Each group carries `label`, plus `predictions` or `points`:

| Field | Type | What it sets |
| --- | --- | --- |
| `predictions` | prediction[] | Raw predictions as `[score, is_positive]` or `{score, label}`; thresholds swept. |
| `points` | `[recall, precision][]` | A pre-computed curve. `predictions` wins if both are given. |
| `prevalence` | number | The class prior, used for the baseline in `points` mode (default 0.5). |
| `color` | string | Curve colour. |
| `optimal_point` | boolean | Mark the F1-optimal threshold point. |
| `auc_label` | boolean | Print the AUC. |
| `line_width` | number | Curve width. |
| `dasharray` | string | Dash pattern (e.g. `"4 2"`). |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **`groups` must not be empty**, and every group needs `predictions` or `points`.
- A group's `points` are `(recall, precision)` pairs.

## See also

- [kuva — Precision-recall curve](https://psy-fer.github.io/kuva/plots/pr.html) — the plotting library's own reference for this chart.
- [ROC curve](./roc.md) — the threshold sweep on the ROC axes.
