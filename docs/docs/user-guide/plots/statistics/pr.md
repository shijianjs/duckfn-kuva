---
title: Precision–recall curve
sidebar_position: 2
description: Precision against recall, with the prevalence baseline — the right curve for rare positives.
---

# Precision–recall curve

A precision–recall curve plots precision (positive predictive value) against recall (sensitivity) as the
threshold is swept. Unlike a [ROC curve](./roc.md) it is insensitive to the class-imbalance ratio and
concentrates entirely on the positive class, which is why it is the correct curve for rare-event work —
fraud, rare disease, information retrieval.

The area under it (AUC-PR) summarises performance: `1.0` is perfect, and the no-skill baseline is a
horizontal line at the prevalence.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Precision–recall curve',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'auc_label': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

Groups take **raw predictions** — `[score, is_positive]` — and the curve, AUC-PR and prevalence baseline are
all derived from them.

## Optimal F1 threshold

`optimal_point` marks the threshold that maximises the F1 score, the harmonic mean of precision and recall.
It is the natural operating point when the two matter equally; when one matters more, read the threshold
off the curve yourself rather than letting F1 decide.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'F1 optimum',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'optimal_point': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

## Several models

One group per model, colours from the palette, and the AUC-PR appended to each legend entry.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Model comparison',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'pr',
    'groups': [
      {'label': 'all predictions', 'predictions': preds, 'auc_label': true},
      {'label': 'high confidence only',
       'predictions': (SELECT list({'score': score, 'label': (label = 1)}) FROM
                         read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
                       WHERE score > 0.5),
       'auc_label': true,
       'optimal_point': true}
    ]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

## Pre-computed points

Pass `(recall, precision)` points when the curve came from elsewhere, sorted by increasing recall. In this
mode the prevalence is not observable from the points, so `prevalence` has to be supplied to place the
no-skill baseline correctly — without it the baseline sits at `0.5`, which is almost never true.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'From pre-computed points',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'external model',
      'points': [[0, 1], [0.1, 0.91], [0.25, 0.84], [0.4, 0.76],
                 [0.55, 0.67], [0.7, 0.55], [0.85, 0.42], [1, 0.1]],
      'prevalence': 0.1
    }]
  }]
})) AS chart;
```

## Line style

Colour survives neither greyscale printing nor every form of colour blindness, so a publication figure
should distinguish models by stroke as well.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Greyscale-safe',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'pr',
    'groups': [
      {'label': 'model A', 'predictions': preds, 'color': 'black', 'line_width': 2},
      {'label': 'model B',
       'predictions': (SELECT list({'score': score, 'label': (label = 1)}) FROM
                         read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
                       WHERE score > 0.5),
       'color': 'black', 'line_width': 1.5, 'dasharray': '6 3'}
    ]
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
| `groups` | group[] | **Required.** One entry per classifier. |
| `predictions` | `[number, boolean][]` | Raw `[score, is_positive]` pairs. |
| `points` | `[number, number][]` | Pre-computed `[recall, precision]` points. |
| `label` | string | The group's legend entry. |
| `prevalence` | number | Prevalence for the baseline in `points` mode (default `0.5`). |
| `color` | string | Curve colour. |
| `optimal_point` | boolean | Mark the F1-optimal threshold. |
| `auc_label` | boolean | Append `AUC = …` to the legend entry (default on). |
| `line_width` / `dasharray` | number / string | Curve stroke. |
| `show_baseline` | boolean | Draw the prevalence baseline (default on). |
| `baseline_color` / `baseline_dasharray` | string | Its appearance. |

## Notes

- **`groups` must not be empty**, and each group needs either `predictions` or `points`.
- `predictions` wins if both are given.
- In `points` mode the baseline is a guess unless you set `prevalence` — the default `0.5` is a different
  statement from the real prevalence, and it is the most common way a PR curve misleads.
- The curve is not defined at recall `0` for a realistic classifier: precision starts at the prevalence,
  not at `1.0`.
- A PR curve's y axis does **not** start at zero by default; check the axis before reading a "small" drop
  as a large one.

## See also

- [kuva — Precision–recall curve](https://psy-fer.github.io/kuva/plots/pr.html) — the plotting library's own reference for this chart.
- [ROC](./roc.md) — the balanced-class counterpart.
- [Volcano](./volcano.md) — the other plot a differential-expression analysis ends with.
