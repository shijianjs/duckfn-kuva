---
title: Canvas, title & axes
sidebar_position: 2
description: The canvas size, the title block, the font sizes, and every option on the x and y axes.
---

# Canvas, title & axes

These top-level fields describe the drawing surface and its two primary axes. They are ignored in
multi-panel mode, where each [panel](./figure.md) carries its own copy.

## Canvas

| Field | Type | What it sets |
| --- | --- | --- |
| `width` | number | Canvas width in pixels. |
| `height` | number | Canvas height in pixels. |

The default canvas is `600 × 450`, plus margins computed from the title, tick labels and legend — which is
why a default chart comes out at about `675 × 511`. Leave both unset so the chart keeps that proportion — a
chart pinned into a wide, short box is squashed. Nothing else is needed in a docs page either: a
`"show":"svg"` block grows to the SVG's own height, so the preview box is never pinned.

## Title

`title` is either a plain string, or an object with a subtitle and sizes:

```json
{ "text": "Fragment length", "subtext": "bimodal distribution", "size": 22, "subtext_size": 14 }
```

| Field | Type | What it sets |
| --- | --- | --- |
| `text` | string | The title. |
| `subtext` | string | A subtitle under it. |
| `size` | integer | Title font size. |
| `subtext_size` | integer | Subtitle font size. |
| `wrap` | integer | Wrap the title after this many characters. |
| `subtext_wrap` | integer | Wrap the subtitle after this many characters. |

## Font

Sizes for every piece of text, in one place. A `theme` can set the family too.

| Field | Type | What it sets |
| --- | --- | --- |
| `family` | string | Font family for all text. |
| `title_size` | integer | Title size (overridden by `title.size`). |
| `label_size` | integer | Axis-label size. |
| `tick_size` | integer | Tick-label size. |
| `body_size` | integer | Body text (legends, annotations). |

## Axes

`x_axis` and `y_axis` share this shape:

| Field | Type | What it sets |
| --- | --- | --- |
| `name` | string | The axis label. |
| `categories` | string[] | Category labels for a categorical axis (bar, box, …). Overrides the labels collected from the data. |
| `min` / `max` | number | Fixed bounds. |
| `log` | boolean | A logarithmic axis. |
| `tick_format` | string \| integer | `"auto"` · `"integer"` · `"sci"` · `"percent"` · `"degree"`, or an integer for that many decimal places. |
| `tick_step` | number | Round ticks to this interval. |
| `wrap` | integer | Wrap the axis label after this many characters. |
| `label_offset` | `[number, number]` | Shift the axis label by `[dx, dy]` pixels. |
| `tick_rotate` | number | Rotate tick labels by this many degrees. **x axis only.** |
| `label_overlap` | string | `"allow"` · `"thin"` · `"stagger"`. **x axis only.** |

:::note[The y axis has fewer knobs]

`tick_rotate` and `label_overlap` are accepted under `y_axis` but have no effect: they write to the x
axis. Everything else above works on both.

:::

## Text wrapping

Long titles and axis labels can be wrapped at a character limit instead of forcing the canvas to grow.
Wrapping is opt-in: nothing wraps until you set a width.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'A title that would otherwise make the top margin enormous',
  'x_axis': {'name': 'a long x-axis label that would push the bottom margin out'},
  'grid': {'wrap': 28},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 20}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

`grid.wrap` sets every text element at once. The per-element fields are applied after it, so they override
it: `title.wrap`, `title.subtext_wrap`, `x_axis.wrap`, `y_axis.wrap`, `legend.wrap`.

| Element | What wrapping does |
| --- | --- |
| Title / subtitle | Centred lines; the top margin grows to fit. |
| x-axis label | Centred lines; the bottom margin grows. |
| y-axis label | Several rotated lines stacked sideways; the left margin grows. |
| Legend labels and titles | Continuation lines with the swatch kept on the first; the legend box gets taller and its width is capped, so the right margin does not run away. |

Wrapping breaks at whitespace, and a single word longer than the limit is hard-broken.

## Colour bar

Charts that draw a colour bar — [heatmap](../plots/distributions/heatmap.md),
[hexbin](../plots/distributions/hexbin.md), [2D histogram](../plots/distributions/histogram2d.md) and
[contour](../plots/relationships/contour.md) — take one more figure-level field:

| Field | Type | What it sets |
| --- | --- | --- |
| `colorbar_tick_format` | string \| integer | Colour-bar label format: `"auto"` (default) · `"sci"` · `"integer"` · `"percent"` · `"degree"`, or a number of decimal places. |

## Examples

Axis labels, a subtitle and a tick format, over one histogram:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': {'text': 'Fragment length', 'subtext': 'two overlapping peaks'},
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'font': {'label_size': 15, 'tick_size': 12},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

A fixed range and a logarithmic y axis, from the same data the scatter page uses:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x', 'min': 0, 'max': 10, 'tick_step': 2},
  'y_axis': {'name': 'y', 'log': true},
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

## See also

- [Grid, ticks & canvas switches](./grid.md) — grid lines, axis lines, tick placement, `bw_mode`.
- [Date & time axes](./datetime.md) — turning a numeric axis into a date axis.
- [Secondary axes](./secondary-axes.md) — a second x or y axis on the same canvas.
