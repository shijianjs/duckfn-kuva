---
title: UpSet plot
sidebar_position: 15
description: Intersection bars over a dot matrix — set overlaps that scale past four sets.
---

# UpSet plot

An UpSet plot is the scalable successor to the [Venn diagram](./venn.md) once there are more than three or
four sets. It has three parts:

| Part | Shows |
| --- | --- |
| **Intersection bars** (top) | How many elements are in each exact combination of sets |
| **Dot matrix** (middle) | Which sets take part in each intersection — filled dots joined by a line |
| **Set size bars** (left) | The total size of each set |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')),
m AS (
  SELECT (GWAS_hit + eQTL * 2 + Splicing_QTL * 4
        + Methylation_QTL * 8 + Conservation * 16 + ClinVar * 32) AS mask
  FROM d
)
SELECT kuva_render(to_json({
  'title': 'Genomic annotation overlap',
  'series': [{
    'type': 'upset',
    'set_names': ['GWAS_hit', 'eQTL', 'Splicing_QTL', 'Methylation_QTL', 'Conservation', 'ClinVar'],
    'set_sizes': [
      (SELECT sum(GWAS_hit) FROM d),
      (SELECT sum(eQTL) FROM d),
      (SELECT sum(Splicing_QTL) FROM d),
      (SELECT sum(Methylation_QTL) FROM d),
      (SELECT sum(Conservation) FROM d),
      (SELECT sum(ClinVar) FROM d)
    ],
    'intersections': (SELECT list({'mask': mask, 'count': n})
                      FROM (SELECT mask, count(*) AS n FROM m WHERE mask > 0 GROUP BY mask)),
    'counts': true,
    'max_visible': 20
  }]
})) AS chart;
```

## The mask

Everything hinges on one number. The `mask` is a bitmask where **bit `i` means "set `i` takes part"**, so
for the six sets above:

| Combination | Mask |
| --- | --- |
| `GWAS_hit` only | `1` |
| `eQTL` only | `2` |
| `GWAS_hit` ∩ `eQTL` | `3` |
| all six | `63` |

The example builds it by multiplying each 0/1 column by its bit weight and summing — which is exactly what
a bitmask is, written out.

An intersection is **exact**: mask `3` counts elements in `GWAS_hit` and `eQTL` and in nothing else. That
is the opposite convention from a Venn's inclusive overlaps, and it is what makes the columns add up.

## Sort order

| `sort` | Order |
| --- | --- |
| `"by_frequency"` | Largest intersection first **(default)** |
| `"by_degree"` | Most sets involved first, ties broken by count |
| `"natural"` | The order you gave |

`"by_degree"` is the one to use when the interesting columns are the complex ones — the rare
six-way overlaps — rather than the biggest ones.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')),
m AS (
  SELECT (GWAS_hit + eQTL * 2 + Splicing_QTL * 4
        + Methylation_QTL * 8 + Conservation * 16 + ClinVar * 32) AS mask
  FROM d
)
SELECT kuva_render(to_json({
  'title': 'Sorted by degree',
  'series': [{
    'type': 'upset',
    'set_names': ['GWAS_hit', 'eQTL', 'Splicing_QTL', 'Methylation_QTL', 'Conservation', 'ClinVar'],
    'set_sizes': [
      (SELECT sum(GWAS_hit) FROM d),
      (SELECT sum(eQTL) FROM d),
      (SELECT sum(Splicing_QTL) FROM d),
      (SELECT sum(Methylation_QTL) FROM d),
      (SELECT sum(Conservation) FROM d),
      (SELECT sum(ClinVar) FROM d)
    ],
    'intersections': (SELECT list({'mask': mask, 'count': n})
                      FROM (SELECT mask, count(*) AS n FROM m WHERE mask > 0 GROUP BY mask)),
    'sort': 'by_degree',
    'counts': true,
    'bar_color': '#1d4ed8',
    'dot_color': '#1e3a8a',
    'max_visible': 15
  }]
})) AS chart;
```

## Limiting and trimming

`max_visible` keeps only the first `n` intersections **after** sorting, so the ones dropped are always the
smallest (or lowest-degree). With six sets there are up to 63 non-empty combinations; nobody reads 63
columns.

`show_set_sizes: false` hides the left panel when the totals are already known from context, which buys a
noticeably more compact layout.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')),
m AS (
  SELECT (GWAS_hit + eQTL * 2 + Splicing_QTL * 4
        + Methylation_QTL * 8 + Conservation * 16 + ClinVar * 32) AS mask
  FROM d
)
SELECT kuva_render(to_json({
  'title': 'Top 8, no set-size panel',
  'series': [{
    'type': 'upset',
    'set_names': ['GWAS_hit', 'eQTL', 'Splicing_QTL', 'Methylation_QTL', 'Conservation', 'ClinVar'],
    'set_sizes': [
      (SELECT sum(GWAS_hit) FROM d),
      (SELECT sum(eQTL) FROM d),
      (SELECT sum(Splicing_QTL) FROM d),
      (SELECT sum(Methylation_QTL) FROM d),
      (SELECT sum(Conservation) FROM d),
      (SELECT sum(ClinVar) FROM d)
    ],
    'intersections': (SELECT list({'mask': mask, 'count': n})
                      FROM (SELECT mask, count(*) AS n FROM m WHERE mask > 0 GROUP BY mask)),
    'max_visible': 8,
    'show_set_sizes': false,
    'counts': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `set_names` | string[] | **Required.** One name per set; bit `i` refers to `set_names[i]`. |
| `set_sizes` | integer[] | Each set's total, for the left bars; must match `set_names` in length. |
| `intersections` | intersection[] | **Required.** `{mask, count}` per non-empty combination. |
| `sort` | string | `"by_frequency"` (default) · `"by_degree"` · `"natural"`. |
| `max_visible` | integer | Keep only the first `n` intersections after sorting. |
| `counts` | boolean | Write the count on each bar. |
| `show_set_sizes` | boolean | Draw the left set-size panel (default on). |
| `bar_color` | string | Intersection and set-size bar colour (default `#333333`). |
| `dot_color` | string | Filled dot colour (default `#333333`). |
| `dot_empty_color` | string | Colour of the dots for sets *not* in an intersection. |

## Notes

- **Masks are exact combinations**, not inclusive counts — mask `3` is "in sets 0 and 1 and nothing else".
  A mask of `0` (in no set at all) is not an intersection and is not drawn.
- `set_sizes` must line up with `set_names`; it is the only place the set totals come from, so it does not
  have to equal the sum of the masks.
- `max_visible` is applied after sorting, so raising it never reorders the chart.
- Dots for sets that are *not* in an intersection are still drawn, in a light grey — they are what makes
  each column's combination readable.

## See also

- [kuva — UpSet plot](https://psy-fer.github.io/kuva/plots/upset.html) — the plotting library's own reference for this chart.
- [Venn](./venn.md) — for two to four sets.
- [Mosaic](./mosaic.md) — two-way categorical proportions instead of set overlaps.
