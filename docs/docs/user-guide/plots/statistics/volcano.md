---
title: Volcano plot
sidebar_position: 5
description: Effect size against significance, with up / down / not-significant classification.
---

# Volcano plot

A volcano plot puts **log₂ fold change** on the x axis against **−log₁₀(p-value)** on the y axis — so the
points that matter are the ones high up and far out. Points passing both cutoffs are coloured up (right) or
down (left), everything else is grey, and the threshold lines are drawn for you.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Tumour vs normal',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

Two thresholds do all the classification: `fc_cutoff` (default `1`, i.e. a two-fold change) and `p_cutoff`
(default `0.05`). Dashed lines appear at `±fc_cutoff` and at `−log10(p_cutoff)`.

## Gene labels

`label_top` labels the *n* most significant points, which is how the handful of genes worth naming get
named without labelling 20 000 of them. Three placement styles are available:

| `label_style` | Placement |
| --- | --- |
| `"nudge"` | Labels sorted by x and nudged vertically to reduce stacking **(default)** |
| `"exact"` | At the point, with no adjustment — for sparse data or post-processing |
| `{"offset_x": 14, "offset_y": 16}` | Offset by pixels, with a short leader line back to the point |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Top hits labelled',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'label_top': 12,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

The arrow style is the one to use when the labels would crowd the high-significance corner, since the
leader line lets a label sit at a comfortable distance from its point:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Arrow labels',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'label_top': 10,
    'label_style': {'offset_x': 14, 'offset_y': 16},
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano_genes.tsv')
);
```

Adding the `nudges' vertical spread is derived from the data, so the same `label_top` produces a tighter
or looser cloud depending on how clustered the top hits are.

## Thresholds

Stricter cutoffs move the lines inward on the x axis and upward on the y axis, shrinking the coloured
corners. They are the first thing to check when two volcano plots of the same comparison disagree.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Stricter cutoffs',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'fc_cutoff': 2,
    'p_cutoff': 0.01,
    'color_up': 'darkorange',
    'color_down': 'mediumpurple',
    'label_top': 8,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

## Colours

| Field | Default | Used for |
| --- | --- | --- |
| `color_up` | `firebrick` | `log2fc ≥ fc_cutoff` and `p ≤ p_cutoff` |
| `color_down` | `steelblue` | `log2fc ≤ −fc_cutoff` and `p ≤ p_cutoff` |
| `color_ns` | `#aaaaaa` | Everything else |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'House colours',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'color_up': '#d62728',
    'color_down': '#1f77b4',
    'color_ns': '#cccccc',
    'point_size': 3.5,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

## Zero and extreme p-values

A p-value of exactly `0` cannot be log-transformed. Such points are clamped to the smallest non-zero
p-value **in the data** — which means the y axis tops out wherever the most significant gene happens to be,
and two volcano plots of different datasets end up on different scales.

`pvalue_floor` sets that ceiling explicitly, so several plots can share a y axis:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pinned y ceiling',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'pvalue_floor': 1e-10,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** Each entry is `{name, log2fc, pvalue}`. |
| `fc_cutoff` | number | The `\|log2FC\|` threshold (default `1`). |
| `p_cutoff` | number | The p-value threshold (default `0.05`). |
| `color_up` / `color_down` / `color_ns` | string | The three classifications' colours. |
| `point_size` | number | Circle radius in pixels (default `3`). |
| `label_top` | integer | Label the *n* most significant points (`0` = none, the default). |
| `label_style` | string \| object | `"nudge"` (default) · `"exact"` · `{offset_x, offset_y}`. |
| `pvalue_floor` | number | Explicit p-value floor for the `−log10` transform. |
| `legend` | string | Any non-empty value turns the up / down / NS legend on. |

## Notes

- **`points` must not be empty**, and every point needs `name`, `log2fc` and `pvalue`.
- `pvalue` is the **raw** p-value, not `−log10` of it.
- p-values above `1` or at or below `0` are nonsensical; the floor handles the second case, the first is
  left to draw at a negative y rather than being silently dropped.
- `label_top` labels by significance, not by fold change — the labels go to the tallest points, which is
  usually but not always what you want to show.
- `fc_cutoff` is compared against the **absolute** value, so the two dashed x lines are symmetric.

## See also

- [kuva — Volcano plot](https://psy-fer.github.io/kuva/plots/volcano.html) — the plotting library's own reference for this chart.
- [Manhattan](./manhattan.md) — significance by genomic position instead of effect size.
- [Q-Q plot](../distributions/qq.md) — check the p-value distribution itself before trusting the peaks.
