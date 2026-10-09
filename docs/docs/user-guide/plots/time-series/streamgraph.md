---
title: Streamgraph
sidebar_position: 2
description: A stacked area chart with a displaced baseline, so the bands flow rather than pile up.
---

# Streamgraph

A streamgraph is a stacked area chart whose baseline is shifted instead of pinned at zero, so the whole
shape undulates around a central axis. That makes it far easier to read when many series overlap: the eye
can follow a band as it widens and narrows instead of losing it in the pile.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Gut microbiome',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species))
  }]
})) AS chart;
```

## Wiggle baseline

The default baseline is Byron & Wattenberg's *wiggle* algorithm, which places the axis so the silhouette
stays as flat as possible — it minimises the sum of squared slopes across all layer boundaries. This is
the canonical streamgraph look, and the one to use when the bands are of comparable size.

## Symmetric baseline

`"baseline": "symmetric"` centres the total stack on y = 0 at every x, so the silhouette mirrors above and
below the axis. It gives the clearest "river" reading, and it is the ThemeRiver convention.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Symmetric baseline',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'baseline': 'symmetric'
  }]
})) AS chart;
```

## Zero baseline

`"baseline": "zero"` stacks from y = 0, which is a plain stacked area chart — the difference is only that
the bands are drawn with smooth curves. It is the honest choice when the absolute total matters, since the
wiggle and symmetric baselines deliberately decouple the silhouette from the sum.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Zero baseline',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'baseline': 'zero'
  }]
})) AS chart;
```

## 100 % normalised

`normalized` rescales every column to sum to 100 %, so the chart shows proportional composition rather
than magnitude. It pairs naturally with a legend, because with a displaced baseline the y axis has no
meaningful units left to label.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Normalised',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'normalized': true,
    'legend': 'species'
  }]
})) AS chart;
```

## Layer ordering

Three orderings decide which band ends up in the middle and which are pushed to the edges:

| `order` | Effect |
| --- | --- |
| `"inside_out"` | Widest streams near the centre, alternating outward **(default)** |
| `"by_total"` | Sorted by total area, largest at the bottom |
| `"original"` | The order you added the series in |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Ordered by total',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'order': 'by_total',
    'legend': 'species'
  }]
})) AS chart;
```

## Inter-stream strokes

`stroke_between` draws a thin line along the upper edge of each band, which rescues legibility when
adjacent streams have similar hues. Combined with `"show_labels": false` and a legend, it gives the
cleanest dense streamgraph.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'With separator strokes',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'stroke_between': true,
    'stroke_width': 1.2,
    'show_labels': false,
    'legend': 'species'
  }]
})) AS chart;
```

## Linear interpolation

Smooth curves (Catmull-Rom) are the default. `"smooth": false` uses straight segments instead, giving the
familiar angular stacked-area look — the right choice when the x values are very closely spaced, or when a
sharp transition must stay sharp rather than be smoothed into a slope.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Linear interpolation',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'smooth': false,
    'stroke_between': true,
    'show_labels': false,
    'legend': 'species'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `x` | number[] | **Required.** The shared x axis. |
| `series` | series[] | **Required.** `{values, label?, color?}` per band; one value per x. |
| `baseline` | string | `"wiggle"` (default) · `"symmetric"` · `"zero"`. |
| `order` | string | `"inside_out"` (default) · `"by_total"` · `"original"`. |
| `smooth` | boolean | Catmull-Rom smoothing (default on). |
| `fill_opacity` | number | Band opacity (default `0.85`). |
| `stroke_between` | boolean | Draw the separator strokes. |
| `stroke_width` | number | Separator width (default `0.8`). |
| `show_labels` | boolean | Write the series labels inline (default on). |
| `min_label_height` | number | Smallest band, in pixels, that still gets a label (default `14`). |
| `normalized` | boolean | Rescale each column to 100 %. |
| `legend` | string | Legend title; any non-empty value turns the legend on. |
| `legend_position` | string | Any [legend position](../../reference/legends.md). |

## Notes

- **`x` and every series' `values` are required**, and a short series is padded with zeros.
- `baseline: "wiggle"` and `"symmetric"` move the axis — the y axis no longer reports the total. Use
  `"zero"` whenever the total is something the reader must be able to read off the chart.
- `min_label_height` is in pixels, so inline labels vanish as the figure shrinks; pair `show_labels: false`
  with a legend for a layout that degrades gracefully.
- The stack order comes from `order`, which overrides the order of `series` — `"original"` is the way to
  get your own order back.

## See also

- [kuva — Streamgraph](https://psy-fer.github.io/kuva/plots/streamgraph.html) — the plotting library's own reference for this chart.
- [Stacked area](./stacked_area.md) — the zero-baselined version.
- [Horizon](./horizon.md) — many series in very little vertical space.
