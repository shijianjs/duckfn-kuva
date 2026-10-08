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

kuva's natural size is about `675 × 511`. Leave both unset so the chart keeps that proportion — a chart
pinned into a wide, short box is squashed. To show a chart larger, grow the *preview box* (the `height`
in the runnable block's options), not the canvas.

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
