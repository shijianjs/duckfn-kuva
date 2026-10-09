---
title: Manhattan plot
sidebar_position: 6
description: GWAS significance across the genome, chromosome by chromosome.
---

# Manhattan plot

A Manhattan plot lays GWAS p-values out across the genome: the x axis spans the chromosomes, the y axis is
**−log₁₀(p)**, and the chromosomes alternate colour so the boundaries stay readable. Peaks standing above
the dashed significance lines are the whole point of the figure.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS results',
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## Sequential mode

Without a `position`, chromosomes are sorted in genomic order (`1–22`, `X`, `Y`, `MT`) and each point gets
a consecutive integer x within its chromosome. That is the honest default when the base-pair coordinates
are not available — the shape of the peaks is preserved, but the axis no longer reflects physical distance.

## Base-pair mode

Give each point a `position` and a `build`, and the x axis becomes a true genomic coordinate: chromosome
lengths come from the reference build, so a wide chromosome takes proportionally more of the axis. Every
chromosome in the build gets a labelled band, even the ones with no data.

| `build` | Assembly |
| --- | --- |
| `"hg19"` | GRCh37 / hg19 |
| `"hg38"` | GRCh38 / hg38 |
| `"t2t"` | T2T-CHM13 v2.0 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'On real genomic coordinates',
  'x_axis': {'name': 'position', 'tick_format': 'sci'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'build': 'hg38',
    'label_top': 8,
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'position': pos, 'pvalue': pvalue,
               'label': gene}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

Chromosome names are accepted with or without a `"chr"` prefix, so `chr11` and `11` are the same
chromosome.

## Labelling specific points

`label_top` labels the *n* most significant points **above the genome-wide threshold** — a peak with no
gene name is still worth a dot. To name a particular point instead, give it a `label`: that label wins over
the automatic selection, and `label_style` decides how it is placed.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Named peaks',
  'x_axis': {'name': 'position', 'tick_format': 'sci'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'build': 'hg38',
    'label_top': 5,
    'label_style': {'offset_x': 10, 'offset_y': 14},
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'position': pos, 'pvalue': pvalue,
               'label': gene}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## Thresholds and colours

| Field | Default | What it sets |
| --- | --- | --- |
| `genome_wide` | `7.301` | The red dashed line, in `−log10(p)` — i.e. `p = 5×10⁻⁸` |
| `suggestive` | `5.0` | The grey dashed line — i.e. `p = 1×10⁻⁵` |
| `color_a` / `color_b` | `steelblue` / `#5aadcb` | The alternating chromosome colours |
| `point_size` | `2.5` | Circle radius in pixels |

Both thresholds are given **on the `−log10` scale** — for `p = 1×10⁻⁶` pass `6`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Custom thresholds and colours',
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'genome_wide': 6,
    'suggestive': 4,
    'color_a': '#4c72b0',
    'color_b': '#dd8452',
    'point_size': 3,
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## Chromosome label overlap

At the default canvas width the small autosomes are narrow enough that their labels collide. The layout's
`x_axis.label_overlap` decides what happens — and `stagger` is the one that keeps every name visible:

| `label_overlap` | Behaviour |
| --- | --- |
| `"allow"` | Every label drawn; small chromosomes may overlap **(default)** |
| `"thin"` | Labels that would overprint a neighbour are dropped |
| `"stagger"` | All labels kept, colliding ones alternate between two rows |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Staggered chromosome labels',
  'x_axis': {'name': 'position', 'label_overlap': 'stagger', 'tick_format': 'sci'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'build': 'hg38',
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'position': pos, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

`stagger` expands the bottom margin for its second row, so it costs a little height and buys a legible
axis.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** Each entry is `{chromosome, pvalue, position?, label?}`. |
| `position` | number | Base-pair position — with a `build`, x becomes a genomic coordinate. |
| `label` | string | A name for this point; overrides the automatic `label_top` selection. |
| `build` | string | `"hg19"` · `"hg38"` · `"t2t"` — supplies chromosome lengths. |
| `genome_wide` / `suggestive` | number | The two dashed thresholds, in `−log10(p)`. |
| `color_a` / `color_b` | string | The alternating chromosome colours. |
| `point_size` | number | Circle radius in pixels (default `2.5`). |
| `label_top` | integer | Label the *n* most significant points above the genome-wide line. |
| `label_style` | string \| object | `"nudge"` (default) · `"exact"` · `{offset_x, offset_y}`. |
| `pvalue_floor` | number | Explicit p-value floor for the `−log10` transform. |
| `legend` | string | Any non-empty value turns the threshold legend on. |

## Notes

- **`points` must not be empty**, and every point needs `chromosome` and `pvalue`.
- `pvalue` is the **raw** p-value. `genome_wide` and `suggestive`, by contrast, are already `−log10` — the
  two are on different scales in the same spec, which is worth double-checking when a threshold line lands
  in the wrong place.
- A `position` with no `build` is used as the x coordinate **directly**, which is the escape hatch for
  pre-computed or non-human coordinates.
- Chromosome ordering follows the build in bp mode and the standard genomic order in sequential mode —
  `chr10` sorts after `chr9`, not alphabetically.
- Colour alternation is by **chromosome order**, not by name, so a build that omits a chromosome shifts
  every later colour.

## See also

- [kuva — Manhattan plot](https://psy-fer.github.io/kuva/plots/manhattan.html) — the plotting library's own reference for this chart.
- [Volcano](./volcano.md) — effect size versus significance.
- [Q-Q plot](../distributions/qq.md) — check for genomic inflation before believing the peaks.
