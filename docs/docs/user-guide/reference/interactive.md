---
title: SVG interactivity
sidebar_position: 15
description: Hover tooltips, click-to-pin, search and legend toggles, embedded straight into the SVG.
---

# SVG interactivity

kuva can embed browser interactivity directly into the SVG it produces — no server, no external
dependencies, no JavaScript from a CDN. Everything travels inside the `.svg` file, so it works from a
`file://` path, an email attachment, or a docs page.

```sql {"type":"duckfn","show":"iframe","option":{"height":"610px"}}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'title': 'Interactive scatter',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'grid': {'interactive': true},
  'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d), 'tooltips': true}]
})) AS chart;
```

Click inside the chart above and try the search box in its top-left corner, or click a legend entry to
toggle that series.

::::note[Why this block is an iframe]

The injected `<script>` runs in whatever document it lands in. Inlined into the page, the plot's JavaScript
would share the docs' own document: its keyboard shortcuts would fight the page's, and the controls it places
against the plot's box would be laid out against the page instead — the search box ends up in the wrong place
and stops tracking the plot. The `iframe` renderer puts the SVG in a sandboxed document of its own
(`allow-scripts`, and deliberately **no** `allow-same-origin`), so coordinates and shortcuts stay inside the
frame.

The `height` is pinned on purpose and is the one exception to "never pin the preview box" on this site: an
iframe has no intrinsic height, so it cannot grow to its content the way the inline SVG blocks do. It is
hand-tuned to the chart's own height (a default chart is about `675 × 511` plus the page's chrome).

::::


## Enabling it

`"grid": {"interactive": true}` is the only field — the plot itself is unchanged, and a non-interactive SVG
is byte-identical to one without the field.

## What it adds

| Feature | How it works |
| --- | --- |
| **Hover tooltip** | Move the cursor over a data element to see its label and value. |
| **Click to pin** | Click an element to keep it highlighted; click it again or press **Escape** to clear. |
| **Search** | Type in the box at the top-left of the plot area to dim everything that does not match; **Escape** clears it. |
| **Coordinate readout** | While the cursor is inside the plot area, the current x and y in data space follow it. |
| **Legend toggle** | Click a legend entry to hide that series; click again to show it. |
| **Save SVG** | The button at the top-right captures the current DOM state. *Download itself is not yet wired up upstream.* |

## Hover tooltips without JavaScript

`tooltips` is a separate, older mechanism: it wraps each element in an SVG `<title>`, which the browser shows
as its own hover tooltip. No script is injected, so it survives hand-off to tools that strip scripts (and it
is what you get in a static image pipeline).

| Field | Where it goes | What it does |
| --- | --- | --- |
| `tooltips: true` | on a series | Native `<title>` tooltip per data element. |
| `tooltip_labels` | on a series | Your own strings, one per element, instead of the auto-generated text. |
| `grid.interactive` | top level | The search box, pinning, legend toggles and the coordinate readout — this page. |

The auto-generated text depends on the chart: a scatter shows `(x, y)`, a bar `label: value`, a volcano plot
`gene (log2fc, −log10p)`. `tooltips` is accepted by scatter, bar, histogram, pie, heatmap, strip, waterfall,
volcano, manhattan, dot plot, candlestick, polar and ternary charts; the rest either have no element to hang
it on (`line`) or draw pixels rather than points.

## Plot support

Everything on this page is wired for the axis-space plots that draw one element per observation — `scatter`,
`line`, `bar`, `strip` and `volcano` respond to hover, search and legend toggles. Other chart types still
accept `interactive` and get the coordinate readout and the search UI, but their individual elements do not
respond yet — that is an upstream limitation, not a switch you are missing.

## Where it does not apply

The SVG output is the only one that carries the script: PNG/PDF rendering and terminal output ignore it.
Inkscape and Illustrator strip `<script>` when they open an SVG, so the saved file loses the interactivity —
open it in a browser to use it.

## Notes

- Interactivity is **additive**: with the field unset, nothing is injected.
- The tooltip popup belongs to the browser, not the SVG — it appears after the usual delay and its styling
  cannot be changed from here.
- `tooltips` and `grid.interactive` are independent; turning both on gives you native tooltips plus the
  search and pinning UI.

## See also

- [Series & shared fields](./series.md) — where `tooltips` and `tooltip_labels` live.
- [Grid, ticks & canvas switches](./grid.md) — the rest of the `grid` object.
- [Figure (multi-panel)](./figure.md) — interactivity works per panel too.
