---
title: ROC curve
sidebar_position: 1
description: True-positive rate against false-positive rate, from raw scores or a pre-computed curve.
---

# ROC curve

An ROC curve plots the true-positive rate against the false-positive rate as the classification threshold
sweeps. Give it raw prediction scores and it sweeps the thresholds, or give it a pre-computed curve.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'model',
      'predictions': preds,
      'ci': true, 'auc_label': true, 'optimal_point': true
    }],
    'show_diagonal': true,
    'legend': 'roc'
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One curve per model (see below). |
| `show_diagonal` | boolean | Draw the random-guess diagonal. |
| `diagonal_color` | string | The diagonal's colour. |
| `diagonal_dasharray` | string | The diagonal's dash pattern. |

Each group carries `label`, plus `predictions` **or** `points`:

| Field | Type | What it sets |
| --- | --- | --- |
| `predictions` | prediction[] | Raw predictions as `[score, is_positive]` or `{score, label}`; the thresholds are swept. |
| `points` | `[fpr, tpr][]` | A pre-computed curve. `predictions` wins if both are given. |
| `color` | string | Curve colour. |
| `ci` / `ci_alpha` | boolean / number | Draw a confidence band, and its opacity. |
| `pauc_range` | `[number, number]` | Integrate only over this false-positive-rate range (partial AUC). |
| `optimal_point` | boolean | Mark the Youden-optimal point. |
| `auc_label` | boolean | Print the AUC. |
| `line_width` | number | Curve width. |
| `dasharray` | string | Dash pattern (e.g. `"4 2"`). |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **`groups` must not be empty**, and every group needs `predictions` or `points`.
- A group's `points` are `(fpr, tpr)` pairs in 0–1.

## See also

- [kuva — ROC curve](https://psy-fer.github.io/kuva/plots/roc.html) — the plotting library's own reference for this chart.
- [Precision-recall curve](./pr.md) — the alternative for imbalanced classes.
