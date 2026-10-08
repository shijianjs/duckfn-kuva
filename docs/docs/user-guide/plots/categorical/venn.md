---
title: Venn diagram
sidebar_position: 12
description: Overlapping circles for the intersections of two to four sets.
---

# Venn diagram

A Venn diagram draws two to four overlapping circles for the intersections of sets. Give it the raw
elements and it computes the overlaps, or the pre-computed sizes and intersection counts.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'venn',
    'sets': sets,
    'counts': true,
    'set_labels': true,
    'fill_opacity': 0.35
  }]
})) AS chart
FROM (
  SELECT list({'label': s, 'elements': els} ORDER BY s) AS sets
  FROM (
    SELECT "set" AS s, list(element) AS els
    FROM read_csv_auto('{{DFK_BASE_URL}}data/venn.tsv')
    GROUP BY "set"
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `sets` | set[] | **Required.** 1–4 sets. Each is `{label, elements}` or `{label, size}` — do not mix the two forms. |
| `overlaps` | overlap[] | Pre-computed intersections: `{sets: […], size}`. An entry's `size` **includes** all its sub-intersections. Only used with the `size` form. |
| `counts` | boolean | Print the count in each region. |
| `percentages` | boolean | Print the percentage in each region. |
| `set_labels` | boolean | Label each set. |
| `fill_opacity` | number | Circle fill opacity. |
| `stroke_width` | number | Circle outline width. |
| `proportional` | boolean | Make circle area proportional to set size. |
| `loss` | boolean | Show the layout stress from the vennEuler solver. |
| `colors` | string[] | Per-set colours. |
| `leader_lines` | boolean | Draw leader lines from the names to the circles. |
| `set_indicators` | boolean | Print each set's initial inside its circle. |
| `legend` | string | The legend title. |

## Notes

- **Only 1 to 4 sets** are supported — more than 4 renders blank and is reported as an error.
- **Do not mix the two set forms:** either every set has `elements` (and `overlaps` is ignored), or none
  does (and `overlaps` carries the intersections). Mixing is an error.
- In `overlaps`, each set name must exist and must not repeat within one entry.

## See also

- [kuva — Venn diagram](https://psy-fer.github.io/kuva/plots/venn.html) — the plotting library's own reference for this chart.
- [UpSet plot](./upset.md) — scales past four sets.
