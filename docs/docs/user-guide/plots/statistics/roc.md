---
title: ROC curve
sidebar_position: 1
description: True positive rate against false positive rate, with AUC, confidence bands and pAUC.
---

# ROC curve

A ROC curve plots the true positive rate (sensitivity) against the false positive rate (1 − specificity) as
the classification threshold is swept. The area under it — the AUC — compresses discrimination ability into
one number: `1.0` is perfect, `0.5` is chance.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'ROC curve',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'auc_label': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

Each group takes **raw predictions** — `[score, is_positive]` — and the curve, the AUC and any confidence
band are computed from them. `label = 1` turns the integer column into the boolean the field wants, so the
whole sweep happens inside the chart rather than in the query.

## Confidence intervals

`ci` shades the DeLong 95 % confidence band around the curve. The DeLong estimator is computed from the raw
predictions, so it needs no bootstrap — but it is only available in `predictions` mode, since pre-computed
points carry no per-sample information to estimate from. `ci_alpha` sets the band's opacity.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a 95 % CI',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'ci': true,
      'ci_alpha': 0.2,
      'optimal_point': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

## Optimal threshold (Youden's J)

`optimal_point` marks the threshold maximising Youden's J, `J = TPR − FPR` — the operating point with the
best trade-off between sensitivity and specificity. It is drawn as a filled circle, and its coordinates
read directly: sensitivity is the y value, specificity is `1 − x`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Youden optimum, annotated',
  'x_axis': {'name': '1 − specificity'},
  'y_axis': {'name': 'sensitivity'},
  'stats_box': {'position': 'inside_bottom_right',
                'entries': ['sensitivity = 0.90', 'specificity = 0.75']},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Biomarker',
      'predictions': preds,
      'optimal_point': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

The numbers in that stats box are the ones you read off the marker — kuva does not compute them, so they
belong in the query if they are to stay in step with the data.

## Several classifiers

One group per model; colours come from the palette and each legend entry gains its AUC automatically.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Model comparison',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'roc',
    'groups': [
      {'label': 'all predictions', 'predictions': preds, 'auc_label': true},
      {'label': 'high confidence only',
       'predictions': (SELECT list({'score': score, 'label': (label = 1)}) FROM
                         read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
                       WHERE score > 0.5),
       'auc_label': true,
       'dasharray': '8 4'}
    ]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

`auc_label: false` on any group drops the AUC from its legend entry, which is how you keep a long legend
readable when the numbers are already in the caption.

## Partial AUC

`pauc_range` integrates only over a FPR sub-range — the clinically relevant region, usually the low end. The
partial AUC is normalised to the width of the range, so a perfect classifier still scores `1.0` there.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'pAUC over FPR 0–0.2',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'pauc_range': [0, 0.2]
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

The legend reads `pAUC (0.0–0.2) = …` when a range is set, so the two numbers cannot be confused.

## Pre-computed points

If the curve was computed elsewhere, pass `(fpr, tpr)` points directly. They must be sorted by increasing
FPR; the AUC then comes from the trapezoidal rule, and no confidence band is available.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'From pre-computed points',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'external classifier',
      'points': [[0, 0], [0.05, 0.42], [0.1, 0.61], [0.2, 0.78],
                 [0.35, 0.88], [0.5, 0.93], [0.75, 0.97], [1, 1]]
    }]
  }]
})) AS chart;
```

## Line style

Colour is not available in a greyscale figure, so give each curve a `dasharray` and a `line_width` instead.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Greyscale-safe',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'roc',
    'groups': [
      {'label': 'model A', 'predictions': preds, 'color': 'black', 'line_width': 2.5},
      {'label': 'model B',
       'predictions': (SELECT list({'score': score, 'label': (label = 1)}) FROM
                         read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
                       WHERE score > 0.5),
       'color': 'black', 'line_width': 1.5, 'dasharray': '8 4'}
    ]
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
| `groups` | group[] | **Required.** One entry per classifier. |
| `predictions` | `[number, boolean][]` | Raw `[score, is_positive]` pairs — computes the curve, AUC and CI. |
| `points` | `[number, number][]` | Pre-computed `[fpr, tpr]` points, sorted by increasing FPR. |
| `label` | string | The group's legend entry. |
| `color` | string | Curve and band colour. |
| `ci` / `ci_alpha` | boolean / number | The DeLong 95 % band and its opacity. |
| `pauc_range` | `[number, number]` | Restrict the AUC to this FPR interval. |
| `optimal_point` | boolean | Mark the Youden's J optimum. |
| `auc_label` | boolean | Append `AUC = …` to the legend entry (default on). |
| `line_width` / `dasharray` | number / string | Curve stroke. |
| `show_diagonal` | boolean | Draw the chance diagonal (default on). |
| `diagonal_color` / `diagonal_dasharray` | string | Its appearance. |

## Notes

- **`groups` must not be empty**, and each group needs either `predictions` or `points`.
- `predictions` wins if both are given.
- `ci` is silently unavailable with `points` input — pre-computed points carry no per-sample data.
- `predictions` must be `[score, is_positive]` pairs; `label = 1` in SQL produces the boolean.
- The AUC is computed **within** each group, so a group given a filtered subset of the predictions gets a
  different curve — that is a feature, not a bug, but it is worth labelling the legend accordingly.

## See also

- [kuva — ROC curve](https://psy-fer.github.io/kuva/plots/roc.html) — the plotting library's own reference for this chart.
- [Precision–recall](./pr.md) — the right curve when the classes are imbalanced.
- [Survival](./survival.md) — another step-function statistical curve.
