---
title: UpSet plot
sidebar_position: 13
description: Set intersections as a bar chart over a membership matrix — the scalable alternative to a Venn diagram.
---

# UpSet plot

An UpSet plot shows set intersections as a bar chart above a dot matrix. Where a Venn diagram runs out of
room at four sets, an UpSet plot handles dozens.

```sql {"type":"duckfn","show":"svg"}
WITH t AS (
  SELECT GWAS_hit, eQTL, Splicing_QTL, Methylation_QTL, Conservation, ClinVar
  FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')
)
SELECT kuva_render(to_json({
  'series': [{
    'type': 'upset',
    'set_names': ['GWAS_hit', 'eQTL', 'Splicing_QTL', 'Methylation_QTL', 'Conservation', 'ClinVar'],
    'set_sizes': [
      (SELECT sum(GWAS_hit) FROM t), (SELECT sum(eQTL) FROM t),
      (SELECT sum(Splicing_QTL) FROM t), (SELECT sum(Methylation_QTL) FROM t),
      (SELECT sum(Conservation) FROM t), (SELECT sum(ClinVar) FROM t)
    ],
    'intersections': [
      {'mask': 1, 'count': (SELECT count(*) FROM t WHERE GWAS_hit = 1)},
      {'mask': 2, 'count': (SELECT count(*) FROM t WHERE eQTL = 1)},
      {'mask': 3, 'count': (SELECT count(*) FROM t WHERE GWAS_hit = 1 AND eQTL = 1)}
    ],
    'sort': 'by_frequency',
    'counts': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `set_names` | string[] | **Required.** The set names (the matrix rows). |
| `set_sizes` | integer[] | Each set's total size; same length as `set_names`. |
| `intersections` | intersection[] | The non-empty intersections: `{mask, count}`. |
| `sort` | string | `"by_frequency"` (default) · `"by_degree"` · `"natural"`. |
| `max_visible` | integer | Show at most this many intersections (the rest collapse). |
| `counts` | boolean | Print the count on each bar. |
| `show_set_sizes` | boolean | Draw the left-hand set-size bars (`false` hides them). |
| `bar_color` | string | Intersection bar colour. |
| `dot_color` | string | "Present" dot colour. |
| `dot_empty_color` | string | "Absent" dot colour. |

In an intersection, `mask` is a bitmask: bit `i` set means `set_names[i]` is in that intersection.

## Notes

- **`set_sizes` must match `set_names` in length**, and there may be at most **64 sets** (a bitmask holds
  64 bits).
- A `mask` of 0 means the empty set and is rejected; a mask that sets bits beyond the declared sets is
  also rejected.

## See also

- [kuva — UpSet plot](https://psy-fer.github.io/kuva/plots/upset.html) — the plotting library's own reference for this chart.
- [Venn diagram](./venn.md) · [Dice plot](./diceplot.md) — other set-overlap views.
