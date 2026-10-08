---
title: Functions
sidebar_position: 3
description: The SQL function duckfn_kuva registers and the JSON chart spec it accepts, with examples you can run in the browser.
---

# Functions

Loading the extension registers one function. It behaves like DuckDB's own: use it in a projection, a
`WHERE` clause or a `GROUP BY`, and it combines with built-in functions freely.

| Function | Kind | Signature | Summary |
| --- | --- | --- | --- |
| `kuva_render` | scalar | `VARCHAR -> VARCHAR` | Renders a chart described by a JSON string and returns it as an SVG document. |

## kuva_render

```text
kuva_render(spec_json VARCHAR) -> VARCHAR
```

`kuva_render` wraps [kuva](https://crates.io/crates/kuva), a pure-Rust statistical plotting library. Its
argument is a JSON document describing one figure; its result is a complete SVG document as a string.
Because the drawing happens inside the extension, a chart renders identically wherever DuckDB runs — the
CLI, a Python or R session, a JVM host, or DuckDB-Wasm in the browser — with no matplotlib or ggplot2 on
the host.

The smallest useful call — read a column pair and render it:

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
-- press Run: the result is a complete SVG document, and it is drawn right here
SELECT kuva_render(to_json({
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

Every block below asks for `"show":"svg"`, so pressing **Run** draws the chart in the result area —
fullscreen is where zoom and pan live.

JSON rather than a DuckDB `STRUCT`: one figure's `series` are heterogeneous (a `scatter` and a `bar` carry
different fields), and a `STRUCT` list is homogeneous, so it cannot express `[StructA, StructB]`. Key names
are snake_case throughout.

### Top-level fields

All of these are optional except `series`:

| Field | What it sets |
| --- | --- |
| `series` | **Required.** The list of series to draw. |
| `title` | The figure title. |
| `x_axis` / `y_axis` | Axis configuration (see below). |
| `grid` | Grid lines, axis lines and ticks. |
| `legend` | Legend visibility, position, title and layout. |
| `theme` | `"light"` / `"dark"` / `"minimal"` / `"solarized"`, or an object overriding individual colours. |
| `palette` | A named palette or a list of colour strings. |
| `font` | Family and sizes (`title_size`, `label_size`, `tick_size`, `body_size`). |
| `annotations` | Reference lines, shaded regions and text callouts. |
| `width` / `height` | The canvas size. |
| `figure` | Switches to a multi-panel grid (see [Combining series](#combining-series)). |

### Series

`series` is a list of objects, each tagged by `type`. Six types are implemented:

| `type` | Key fields |
| --- | --- |
| `scatter` | `data`, `size` / `sizes` / `colors`, `marker` (circle/square/triangle/diamond/cross/plus), `marker_opacity`, `marker_stroke_width`, `trend`, `band`, `group_name`. |
| `line` | `data`, `stroke_width`, `line_style` (solid/dashed/dotted/dash_dot or a dash-array string), `step`, `fill`, `fill_opacity`, `band`. |
| `bar` | `categories` + `values` (simple) or `series` + `stacked` / `horizontal` (grouped or stacked); `width`, `gap`, `colors`, `errors`, `error_color`, `error_cap_width`. |
| `histogram` | `values` + `bins` / `range` / `normalize`, or pre-binned `edges` + `counts`; plus `kde`, `kde_color`, `kde_bandwidth`, `kde_samples`. |
| `box` | `groups` (`[{"label":…,"values":[…]}]`), `colors`, `width`, `gap`, `horizontal`, `strip`, `swarm`, `overlay_color`, `overlay_size`, `notch`, `notch_depth`, `notch_width`. |
| `pie` | `slices` (`[{"label":…,"value":…}]`), `inner_radius` (a donut when > 0), `label_position` (auto/inside/outside/none), `percent`, `min_label_fraction`. |

Every series also accepts `color`, `legend`, `tooltips` and `tooltip_labels`.

For `scatter` and `line`, `data` is a list of points, either as `[x, y]` pairs or as objects
(`{"x":…,"y":…,"x_err":…,"y_err":…}`); an error bar is a single number for a symmetric one or a
`[lower, upper]` pair for an asymmetric one.

The six blocks run against the plot library's own [example datasets](https://github.com/Psy-Fer/kuva/tree/master/examples/data),
served from this site — which is also how a real query builds the spec: aggregate the rows into the
`data` / `categories` / `values` / `slices` a series wants, then `to_json` the whole object.

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': list({'type': 'line', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 20}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [{'type': 'box', 'groups': list({'label': g, 'values': vals} ORDER BY g)}]
})) AS chart
FROM (
  SELECT "group" AS g, list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  GROUP BY "group"
);
```

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'pie',
    'slices': list({'label': feature, 'value': percentage} ORDER BY percentage DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv');
```

### Axes

`x_axis` and `y_axis` accept:

| Field | What it sets |
| --- | --- |
| `name` | The axis label. |
| `categories` | Category labels for a categorical axis. |
| `min` / `max` | Fixed bounds. |
| `log` | A logarithmic axis. |
| `tick_format` | `auto` / `integer` / `sci` / `percent` / `degree`, or an integer for a fixed number of decimals. |
| `tick_rotate`, `label_overlap` (`allow`/`thin`/`stagger`), `wrap` | Tick label layout. |

### Grid, legend, theme and palette

`grid` carries `show_grid`, `ticks`, `axis_line` (open/box), `tick_align`, `tick_pos`,
`grid_line_width`, `axis_line_width`, `tick_width`, `tick_length`, `minor_ticks`, `show_minor_grid`,
`clamp_axis`, `clamp_y_axis`, `bw_mode`, `interactive`, `equal_aspect`, `scale` and `label_background`.

`legend` carries `show`, `position` (for example `outside_right_top`, `inside_top_left` or
`outside_bottom_columns`), `title`, `show_box`, `width`, `height`, `col_limit`, `entry_limit`, `wrap`,
`at` and `at_data`.

`theme` is one of the four named themes or an object overriding `background`, `axis_color`, `grid_color`,
`tick_color`, `text_color`, `legend_bg`, `legend_border`, `pie_leader`, `box_median`, `violin_border`,
`colorbar_border`, `font_family` and `show_grid`.

`palette` is a named palette — `wong`, `okabe_ito`, `tol_bright`, `tol_muted`, `tol_light`, `ibm`,
`deuteranopia`, `protanopia`, `tritanopia`, `category10`, `pastel` or `bold` — or a list of colour
strings.

### Annotations

`annotations` holds three lists:

- `reference_lines`: `{"orientation":"horizontal"|"vertical","value":…,"color":…,"stroke_width":…,"dasharray":…,"label":…}`.
- `shaded_regions`: `{"orientation":…,"min":…,"max":…,"color":…,"opacity":…}`.
- `texts`: `{"text":…,"x":…,"y":…,"target_x":…,"target_y":…,"color":…,"font_size":…,"arrow_padding":…}`.

### Combining series

Two kinds of composition are supported.

**Overlay.** Put several series in one `series` list; they share one set of axes. A line plus its points,
for example (both built from the same rows, so the two series cannot drift apart):

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
SELECT kuva_render(to_json({
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

**Multiple panels.** Add a top-level `figure` object instead of drawing a single panel. It carries `rows`,
`cols`, `title`, `title_size`, `labels` (`"uppercase"` / `"lowercase"` / `"numeric"` / `"none"` or a custom
array), `shared_x_all`, `shared_y_all`, `shared_legend` (a position string such as `"right_top"`),
`spacing`, `padding`, `cell_width`, `cell_height`, `figure_width` and `figure_height`, plus `panels` — one
object per cell, each with its own layout fields and `series`. `panels` must have exactly `rows * cols`
entries, in row-major order.

```sql {"type":"duckfn","show":"svg","option":{"height":"360px"}}
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'panels': [
      {'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))}]},
      {'series': [{'type': 'histogram', 'values': (SELECT list(value) FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv')), 'bins': 20}]}
    ]
  }
})) AS chart;
```

Leave `figure_width` / `figure_height` unset and kuva lays the panels out at its own default cell size
(`500x380` each), so a panel keeps the same proportions as a single figure. Pinning the figure to a
wide, short box would squash every panel inside it.

## Errors

Any failure — malformed JSON, a wrong field type, an empty `series` list, a length mismatch, and so on —
fails the whole query rather than returning `NULL`. The message is in English and always starts with
`kuva_render: `:

```sql {"type":"duckfn","expect":"error"}
SELECT kuva_render('{"series":[]}');   -- error: `series` must not be empty
```

## Notes

- **A failed render fails the statement.** The error names the function, and the rest of the query is not
  evaluated. Nothing is silently turned into `NULL`.
- **The result is a string, not a file.** `kuva_render` returns the SVG text; writing it to a file or
  serving it is up to the caller (for example `COPY (SELECT kuva_render(…)) TO 'chart.svg'`).
- **JSON is the fallback API.** It exists because `series` is heterogeneous; a future SQL-friendly layer
  (one function per chart type, `STRUCT` arguments) can sit on top of the same renderer.