---
title: Text plot
sidebar_position: 3
description: Formatted, word-wrapped prose as a panel of a figure.
---

# Text plot

A text plot renders formatted, word-wrapped prose as a chart in its own right, which is how a methods note, a
statistical summary or a caption gets to sit **inside** the same figure as the data it describes — rather
than in a slide's speaker notes where it will be lost.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'title': 'Methods',
    'body': 'Samples were collected from three sites between April and June. All measurements are reported as mean ± SD (n = 48).'
  }]
})) AS chart;
```

Long lines are word-wrapped to the cell width, so the paragraph shape is a property of the panel, not of the
string.

## Markup

The body understands a small set of line-level markup:

| Syntax | Renders as |
| --- | --- |
| `# Heading` | A large bold heading |
| `## Subheading` | A medium bold heading |
| `**bold line**` | A bold paragraph |
| `---` | A horizontal rule |
| A blank line | Paragraph spacing |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'body': '# Results' || chr(10) || chr(10)
         || 'The treatment group showed a significant improvement.' || chr(10) || chr(10)
         || '## Primary endpoint' || chr(10) || chr(10)
         || '**p < 0.001 (log-rank test)**' || chr(10) || chr(10)
         || '---' || chr(10) || chr(10)
         || 'Secondary endpoints are reported in the supplementary material.'
  }]
})) AS chart;
```

A newline is written as `chr(10)` in SQL: DuckDB's plain string literals do not interpret `\n`, so a real
line break has to be concatenated in.

## Appearance

`background` and `border_color`/`border_width` give the panel a card to sit on, `padding` gives it air, and
`text_align` centres or right-aligns it. The defaults are transparent and left-aligned, which is right for a
panel inside a figure and wrong for a standalone slide.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'title': 'Note',
    'body': 'Significant outliers were removed prior to analysis (n = 3, z > 3.5).',
    'background': '#f8f4e8',
    'border_color': '#ccaa66',
    'border_width': 1.5,
    'font_size': 13,
    'padding': 20,
    'text_align': 'center',
    'text_color': '#333333'
  }]
})) AS chart;
```

## Inside a figure

The intended use: one panel of a multi-panel figure is the description of the others. The `figure` block
lays out the grid and each panel carries its own series.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1,
    'cols': 2,
    'cell_width': 380,
    'cell_height': 320,
    'panels': [
      {
        'title': 'Northern transect',
        'x_axis': {'name': 'x'},
        'y_axis': {'name': 'y'},
        'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d), 'color': 'steelblue'}]
      },
      {
        'series': [{
          'type': 'text',
          'title': 'About this data',
          'body': 'Measurements taken from the Northern transect.' || chr(10) || chr(10)
               || '**n = 48**, collected April–June 2025.' || chr(10) || chr(10)
               || '---' || chr(10) || chr(10)
               || 'Outliers excluded per pre-registered protocol.'
        }]
      }
    ]
  }
})) AS chart;
```

The text panel has no axes, so its `x_axis` / `y_axis` are simply left out; the two panels share only the
figure's grid.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `body` | string | **Required.** The prose; supports the markup above. |
| `title` | string | A bold title above the body. |
| `font_size` | integer | Font size in pixels. |
| `padding` | number | Inner padding on all sides (default `16`). |
| `background` | string | Background fill; transparent by default. |
| `border_color` / `border_width` | string / number | Border colour and width (`0` draws none). |
| `text_align` | string | `"left"` (default) · `"center"` · `"right"`. |
| `text_color` | string | Text colour. |

## Notes

- **`body` is required**, though it may be empty — an empty text plot renders an empty panel.
- A newline must be a real newline: in SQL that means `chr(10)`, since `'\n'` stays two characters.
- `font_size` must not be `0` — the renderer divides by it to compute character widths.
- Markup is **line-level**, so `**bold**` only works when the asterisks wrap a whole line; inline emphasis
  mid-sentence is not parsed.
- `border_width: 0` draws no border whatever the colour, which is the way to keep a background fill without
  a frame.

## See also

- [kuva — Text plot](https://psy-fer.github.io/kuva/plots/text.html) — the plotting library's own reference for this chart.
- [Legend plot](./legend.md) — the other pixel-space annotation panel.
- [Figures](../../reference/figure.md) — the multi-panel grid these panels live in.
