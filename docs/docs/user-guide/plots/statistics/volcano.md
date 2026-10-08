---
title: Volcano plot
sidebar_position: 5
description: Fold change against significance, with up/down/neutral colours and top-point labels.
---

# Volcano plot

A volcano plot puts log2 fold change on x and −log10(p) on y, so features that are both strongly changed
and highly significant sit in the upper corners. Points are coloured by whether they pass both cutoffs.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p)'},
  'series': [{
    'type': 'volcano',
    'points': list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}),
    'fc_cutoff': 1,
    'p_cutoff': 0.05,
    'label_top': 10
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** One entry per feature: `{name, log2fc, pvalue}`. |
| `fc_cutoff` | number | The fold-change cutoff, taken as an absolute value. |
| `p_cutoff` | number | The p-value cutoff. |
| `color_up` / `color_down` / `color_ns` | string | Colours for up, down and non-significant points. |
| `point_size` | number | Point radius. |
| `label_top` | integer | Label the N most significant points (`0` labels none). |
| `label_style` | string \| object | `"nudge"` (default) · `"exact"` · `{"offset_x":…, "offset_y":…}`. |
| `pvalue_floor` | number | The p-value floor (avoids `log10(0)`); default: the smallest non-zero p in the data. |

`legend`, `tooltips` and `tooltip_labels` come from [series & shared fields](../../reference/series.md).
There is no uniform `color` — the three classes have their own colours.

## Notes

- **`points` must not be empty.** Give the **raw** `pvalue`, not `-log10(p)`.
- `fc_cutoff` is applied to the absolute fold change, so it colours both up and down extremes.

## See also

- [kuva — Volcano plot](https://psy-fer.github.io/kuva/plots/volcano.html) — the plotting library's own reference for this chart.
- [Manhattan plot](./manhattan.md) — the genome-wide counterpart.
- [Q-Q plot](../distributions/qq.md) — checking the p-value distribution itself.
