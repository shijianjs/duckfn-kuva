---
title: Composites (coverage & brick-pop)
sidebar_position: 20
description: The two charts that are not series — a genome-browser coverage figure and a population allele-frequency figure, both driven by a top-level composite object.
---

# Composites (coverage & brick-pop)

Two of kuva's charts are **composites** rather than `Plot` variants: they assemble their own canvas,
track heights and shared axis instead of being laid out by the generic `Layout` / `Figure` machinery.

| Composite | What it draws |
| --- | --- |
| `"coverage"` | A genome-browser style figure: one depth track per sample on a shared genomic x-axis, an optional variant lane, and feature / gene bands above the axis. |
| `"brick_pop"` | A brick plot with an allele-frequency bar beside each row and optional metric heat columns. |

They take a top-level `composite` object, tagged by `type`:

```json
{ "composite": { "type": "coverage", … } }
```

`composite` is **mutually exclusive with `series` and `figure`** — a composite is the whole figure, so
mixing it with a series list is reported as an error rather than silently dropping one of them. It also
carries its own `width` / `height` (default width `900`; height is computed from the track / row count).

## Coverage

A coverage figure stacks one filled-area depth track per sample, all sharing a single **genomic** x-axis,
so coverage, variants, amplicons and genes line up column-for-column. Features and genes sit **above** the
axis, with the x-axis label in the left gutter.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'composite': {
    'type': 'coverage',
    'title': 'Amplicon panel',
    'x_label': 'position (bp)',
    'locus': [1000000, 1000600],
    'samples': [
      {'name': 'tumour', 'depth': [[1000000, 12], [1000150, 44], [1000300, 51],
                                   [1000450, 38], [1000600, 9]]},
      {'name': 'normal', 'depth': [[1000000, 8], [1000150, 29], [1000300, 33],
                                   [1000450, 27], [1000600, 11]]}
    ],
    'variants': [
      {'label': 'SNV', 'color': '#d1495b', 'positions': [1000150, 1000430]},
      {'label': 'InDel', 'color': '#e9c46a', 'positions': [1000310]}
    ],
    'features': [
      {'start': 1000050, 'end': 1000250, 'label': 'amplicon_1'},
      {'start': 1000300, 'end': 1000550, 'label': 'amplicon_2'}
    ],
    'regions': [{'start': 1000100, 'end': 1000500, 'label': 'GENE1'}],
    'coverage_thresholds': [{'depth': 20, 'label': 'min depth'}]
  }
})) AS chart;
```

| Field | Type | What it sets |
| --- | --- | --- |
| `samples` | `{name, depth}`[] | **Required.** One depth track per sample; `depth` is `[[position, depth], …]`. |
| `locus` | `[start, end]` | Pin the genomic x-range; defaults to the union of all data. |
| `variants` | `{label, color?, positions}`[] | Typed variant positions, one legend entry per group. |
| `features` | `{start, end, label?}`[] | Feature bands (amplicons / primers) above the axis. |
| `regions` | `{start, end, label?}`[] | Genome regions (genes / ORFs) in the lane just above the axis. |
| `coverage_thresholds` | `{depth, label?}`[] | Dashed horizontal threshold lines on every depth track. |
| `overlay_samples` | boolean | Draw all samples in one shared track instead of one track each. |
| `x_label` | string | x-axis label (default `"position"`). |
| `feature_track_name` / `region_track_name` | string | Lane names (default `"features"` / `"genes"`). |
| `title` | string | Title band above the tracks. |
| `theme` | string \| object | The same themes as a normal figure. |
| `tick_font_family` | string | Font for every tick label (genomic coordinates and each depth axis). |
| `width` / `height` | number | Canvas size in pixels; height defaults to the track count. |

## Brick-pop

`brick_pop` wraps a [brick plot](../plots/utility/brick.md) with a frequency bar per row and optional
metric columns rendered as colormapped cells. The brick fields are **the same names as `type: "brick"`**,
flattened into the `composite` object — so one brick config can be drawn either way.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'composite': {
    'type': 'brick_pop',
    'title': 'Allele frequencies',
    'names': ['hap_1', 'hap_2', 'hap_3'],
    'strigars': [
      ['CAG:A', '8A'],
      ['CAG:A,CAA:B', '6A2B'],
      ['CAG:A', '7A1B']
    ],
    'frequencies': [0.52, 0.31, 0.17],
    'frequency_label': 'Frequency',
    'freq_bar_color': '#4c72b0',
    'metrics': [
      {'label': 'depth', 'values': [120, 80, null], 'color_map': 'viridis'},
      {'label': 'qual', 'values': [30, 28, 35], 'color_map': 'magma', 'min': 0, 'max': 40}
    ],
    'metric_values': true,
    'sorted_by_frequency': true,
    'row_height': 26
  }
})) AS chart;
```

| Field | Type | What it sets |
| --- | --- | --- |
| *(brick fields)* | — | `sequences` / `strigars` / `flanked_strigars`, `names`, `template`, `x_offset`, `row_height`, … exactly as on the [brick page](../plots/utility/brick.md). |
| `frequencies` | number[] | One value per row, same order as the rows. |
| `frequency_range` | `[min, max]` | Pin the bar scale; defaults to `[0, 1]` for proportion-like values, else `[0, max]`. |
| `frequency_label` | string | Header for the frequency panel (default `"Frequency"`). |
| `freq_bar_color` | string | Bar colour. |
| `metrics` | `{label, values, color_map?, min?, max?}`[] | Metric heat columns, drawn left to right between the bars and the bricks. `values` takes `null` for missing — distinct from zero. |
| `metric_values` | boolean | Print each metric cell's number (the heat colour moves to the cell border). |
| `na_color` | string | Fill for missing metric cells. |
| `sorted_by_frequency` | boolean | Reorder rows by descending frequency, permuting the bricks, frequencies and metric columns together. |
| `title` / `theme` / `width` / `height` | — | As for `coverage`. |

## Notes

- **`composite` replaces `series`.** Both at once is an error; so is `composite` with `figure`.
- **No datasets.** The `dataset` / `encode` mechanism binds columns onto a *series*; a composite has no
  series, so it takes its data inline (aggregate it in SQL with `list(...)` / `array_agg(...)`).
- **Lengths are checked here.** `frequencies` and every `metrics[].values` must have exactly one entry per
  brick row, and `locus` / feature / region ranges must have `start < end`. kuva would only `debug_assert`
  these (and misalign silently in release), so the extension reports them.
- **Canvas size is the composite's own.** A top-level `width` / `height` is ignored; set it inside the
  `composite` object.

## See also

- [kuva — Coverage plot](https://psy-fer.github.io/kuva/plots/coverage.html) — the plotting library's own reference.
- [kuva — Brick pop plot](https://psy-fer.github.io/kuva/plots/brick_pop.html) — the plotting library's own reference.
- [Brick plot](../plots/utility/brick.md) — the plain version of the same rows.
- [Figure (multi-panel)](./figure.md) — grid layout for *series*, which is a different composition.
