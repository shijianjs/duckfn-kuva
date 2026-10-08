---
title: Manhattan plot
sidebar_position: 6
description: Genome-wide association results laid out along the chromosomes.
---

# Manhattan plot

A Manhattan plot lays association results out along the genome, chromosome by chromosome, with −log10(p)
on y. Peaks above the significance line are the hits.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'chromosome'},
  'y_axis': {'name': '-log10(p)'},
  'series': [{
    'type': 'manhattan',
    'points': list({'chromosome': chr, 'position': pos, 'pvalue': pvalue}),
    'build': 'hg38',
    'genome_wide': 7.3,
    'suggestive': 5,
    'point_size': 3
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** One entry per variant: `{chromosome, position?, pvalue, label?}`. |
| `genome_wide` | number | The genome-wide significance line, in `-log10(p)`. |
| `suggestive` | number | The suggestive-significance line. |
| `color_a` / `color_b` | string | Colours for odd and even chromosomes (default: `color_a`'s lighter shade). |
| `build` | string | `"hg19"` · `"hg38"` · `"t2t"` — uses that build's chromosome lengths for cumulative coordinates. |
| `point_size` | number | Point radius. |
| `label_top` | integer | Label the N most significant points (`0` labels none). |
| `label_style` | string \| object | `"nudge"` (default) · `"exact"` · `{"offset_x":…, "offset_y":…}`. |
| `pvalue_floor` | number | The p-value floor (avoids `log10(0)`). |

## Notes

- **`points` must not be empty**, p values must not be negative, and **`build` requires every point to
  carry a `position`** (in bp) — otherwise it is an error.
- The x coordinate depends on what each point carries: chromosome index (chromosome + pvalue only),
  cumulative base pairs (with `build`), or the raw `position` (position without `build`).
- A point's explicit `label` always wins; `label_top` fills in the unlabelled top points.

## See also

- [kuva — Manhattan plot](https://psy-fer.github.io/kuva/plots/manhattan.html) — the plotting library's own reference for this chart.
- [Volcano plot](./volcano.md) — per-gene fold change instead of genome position.
