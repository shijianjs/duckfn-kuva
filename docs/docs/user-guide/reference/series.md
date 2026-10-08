---
title: Series & shared fields
sidebar_position: 1
description: The shape of a chart's series list, the fields every series accepts, and the value types shared across chart types — points, error bars, bands, trend lines and grouped values.
---

# Series & shared fields

Every chart is a JSON object whose `series` is a **list**. Each entry is one series, tagged by `type`:

```json
{
  "series": [
    { "type": "scatter", "data": [[1, 2], [2, 3]] },
    { "type": "line", "data": [[1, 2], [2, 3]], "legend": "trend" }
  ]
}
```

A list (rather than a single object) is what lets one figure **overlay** several chart types on the same
axes, and it is why the spec is JSON instead of a DuckDB `STRUCT`: `scatter` and `bar` carry different
fields, and a `STRUCT` list cannot hold `[StructA, StructB]`.

## Chart types

All 64 of kuva's chart types are implemented. They are grouped in the sidebar the way
[kuva's own documentation](https://github.com/Psy-Fer/kuva/tree/master/docs/src) groups them:

| Group | `type` values |
| --- | --- |
| Distributions | `histogram` · `histogram2d` · `density` · `ridgeline` · `ecdf` · `qq` · `box` · `violin` · `strip` · `raincloud` · `hexbin` · `heatmap` |
| Relationships & correlation | `scatter` · `line` · `series` · `band` · `jointplot` · `contour` · `parallel` · `polar` · `ternary` · `quiver` |
| Categorical & comparison | `bar` · `pie` · `waffle` · `funnel` · `pareto` · `pyramid` · `lollipop` · `slope` · `dot_plot` · `dice_plot` · `mosaic` · `venn` · `upset` · `radar` · `rose` |
| Time series | `stacked_area` · `streamgraph` · `candlestick` · `waterfall` · `horizon` · `calendar` · `gantt` · `bump` |
| Statistical & model evaluation | `roc` · `pr` · `survival` · `forest` · `volcano` · `manhattan` |
| Hierarchical & network | `treemap` · `sunburst` · `network` · `sankey` · `chord` · `phylo` · `clustermap` |
| 3D | `scatter3d` · `surface3d` |
| Composite & utility | `brick` · `synteny` · `text` · `legend_plot` |

A few types also accept a shorter alias: `dotplot`, `diceplot`, `histogram_2d`, `scatter_3d`,
`surface_3d`, `up_set`, `phylo_tree`, `interval` (for `band`), `joint` (for `jointplot`) and
`legend` (for `legend_plot`).

## Fields every series accepts

These four are common to (almost) every series; on the chart pages they are not repeated:

| Field | Type | What it sets |
| --- | --- | --- |
| `color` | string | The series colour, as a CSS colour (`"steelblue"`, `"#4c72b0"`, `"rgb(…)"`). Defaults to black, unless no series in the figure sets a colour — then the palette cycles. |
| `legend` | string | The legend entry for this series. **A series only appears in the legend when this is set.** |
| `tooltips` | boolean | Injects hover tooltips into the SVG. Not implemented for `line`. |
| `tooltip_labels` | string[] | One tooltip string per data point, in data order. |

:::note[Colour-mapped charts]

Charts that encode a value through a continuous [colormap](./colormaps.md) — `heatmap`, `histogram2d`,
`hexbin`, `clustermap`, `contour`, `dice_plot`, `calendar` and a few others — have no `color` field;
they take `color_map` instead. Each chart page's field table is authoritative.

:::

## Points

`scatter`, `line`, `series` and `quiver` take a list of points. A point is either a pair, or an object
that may carry per-point error bars:

```json
[[1.0, 2.0], [2.0, 3.5]]
```

```json
{ "x": 1.0, "y": 2.0, "x_err": 0.2, "y_err": [0.3, 0.8] }
```

`x_err` / `y_err` are either a single number (a **symmetric** error bar, ± that much) or a
`[lower, upper]` pair (an **asymmetric** one, giving the two arm lengths).

## Bands

A shaded uncertainty region is a pair of columns aligned with the x positions of the data:

```json
{ "lower": [0.8, 1.7, 2.4], "upper": [1.3, 2.4, 3.1] }
```

Pass it as `band` on a `scatter` or `line` series. Both lists must be the same length as the data.

## Trend lines

Set `trend` on a `scatter` series. The short form fits an ordinary-least-squares line; the object form
also styles it and prints the fit statistics:

```json
{ "type": "linear", "color": "crimson", "width": 2, "equation": true, "correlation": true }
```

| Field | Type | What it sets |
| --- | --- | --- |
| `type` | `"linear"` | The fit. `linear` is the only kind. |
| `color` | string | Line colour (default `"black"`). |
| `width` | number | Line stroke width. |
| `equation` | boolean | Print the regression equation `y = mx + b` on the plot. |
| `correlation` | boolean | Print the Pearson R² on the plot. |

For a cleaner result on a dense cloud, leave these off and put the numbers in a
[stats box](./stats-box.md) instead.

## Grouped values

The distribution charts — `violin`, `ridgeline`, `raincloud`, `strip`, `ecdf` and `qq` — take their data
as a list of `groups` rather than one flat column. Each group is a label plus a column of observations,
with an optional per-group colour:

```json
{ "label": "Control", "values": [1.2, 0.9, 1.5], "color": "steelblue" }
```

Every group must carry at least one value.

## Examples

Overlay a line and its points from the same rows, so the two series cannot drift apart:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [
    {'type': 'line', 'data': pts, 'legend': 'trend'},
    {'type': 'scatter', 'data': pts, 'legend': 'points'}
  ]
})) AS chart
FROM (
  SELECT array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

Error bars come from per-point objects; a trend line fits through the same points:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'scatter',
    'data': pts,
    'color': 'steelblue',
    'trend': {'type': 'linear', 'equation': true, 'correlation': true}
  }]
})) AS chart
FROM (
  SELECT array_agg({'x': time, 'y': value, 'y_err': 1.5} ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_B'
);
```

## Notes

- **Only `legend` puts a series in the legend.** Give it a value, or the series draws unlabelled.
- **`tooltips` needs a host that runs JavaScript.** The tooltips are inline SVG + JS, so they work in
  the browser preview and in any HTML page, but not in a static image export.
- **An empty `series` list is an error**, as is a series whose required data field is empty. See
  [the function's error section](../functions.md#errors).
