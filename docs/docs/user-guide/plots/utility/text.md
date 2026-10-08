---
title: Text plot
sidebar_position: 3
description: A block of lightly marked-up prose laid out as a figure.
---

# Text plot

A text plot lays a block of lightly marked-up text out as a figure, so a caption or a methods note can
be rendered — and exported — alongside the charts it belongs to.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'body': '# Summary\n---\nA **single** render across every host.\n\nNo matplotlib, no ggplot2.',
    'title': 'Notes',
    'font_size': 14,
    'padding': 20,
    'background': '#f8f8f8',
    'border_color': '#cccccc',
    'border_width': 1,
    'text_align': 'left'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `body` | string | **Required.** The text. Line markers: `#` / `##` headings, `**bold**`, `---` rules, blank lines as paragraph breaks. |
| `title` | string | A title above the body. |
| `font_size` | integer | Body font size (must be at least 1). |
| `padding` | number | Inner padding, in pixels. |
| `background` | string | Background colour. |
| `border_color` | string | Border colour (unset: light grey). |
| `border_width` | number | Border width; `0` draws none. |
| `text_align` | string | `"left"` · `"center"` · `"right"`. |
| `text_color` | string | Text colour. |

## Notes

- **`body` must not be empty**, and **`font_size` must be at least 1** — 0 would divide by zero.
- The markup is deliberately minimal: headings, bold, rules and paragraph breaks only.

## See also

- [kuva — Text plot](https://psy-fer.github.io/kuva/plots/text.html) — the plotting library's own reference for this chart.
- [Legend plot](./legend.md) — the other figure-as-a-figure utility.
