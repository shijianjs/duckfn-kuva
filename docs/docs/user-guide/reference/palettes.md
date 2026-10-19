---
title: Colour palettes
sidebar_position: 6
description: The named palettes and how to pass your own list of colours.
---

# Colour palettes

`palette` is a top-level field: it chooses the colours the figure hands out to series that do not set
their own `color`. It is a name, or a `{"colors": [...]}` object of colour strings of your own.

| Value | Notes |
| --- | --- |
| `"wong"` | The Wong palette — eight high-contrast, colour-blind-safe colours. |
| `"okabe_ito"` | The Okabe–Ito palette. |
| `"tol_bright"` · `"tol_muted"` · `"tol_light"` | Paul Tol's three qualitative schemes. |
| `"ibm"` | IBM's colour-blind-safe palette. |
| `"deuteranopia"` · `"protanopia"` · `"tritanopia"` | Palettes tuned for each common form of colour blindness. |
| `"category10"` | Ten categorical colours (the default fallback). |
| `"pastel"` · `"bold"` | Softer and stronger variants. |
| `"paloma"` … `"casa_natal"` | Fifteen curated **LTC** qualitative palettes (see below). |

A `{"colors": [...]}` object replaces the named palette:

```json
{ "palette": {"colors": ["#4c72b0", "#dd8452", "#55a868", "#c44e52"]} }
```

## Colours

The exact colours, in the order they are handed out:

| Palette | N | Colours |
| --- | --- | --- |
| `wong` / `okabe_ito` | 8 | `#E69F00` `#56B4E9` `#009E73` `#F0E442` `#0072B2` `#D55E00` `#CC79A7` `#000000` |
| `tol_bright` | 7 | `#4477AA` `#EE6677` `#228833` `#CCBB44` `#66CCEE` `#AA3377` `#BBBBBB` |
| `tol_muted` | 10 | `#CC6677` `#332288` `#DDCC77` `#117733` `#88CCEE` `#882255` `#44AA99` `#999933` `#AA4499` `#DDDDDD` |
| `tol_light` | 9 | `#77AADD` `#EE8866` `#EEDD88` `#FFAABB` `#99DDFF` `#44BB99` `#BBCC33` `#AAAA00` `#DDDDDD` |
| `ibm` | 5 | `#648FFF` `#785EF0` `#DC267F` `#FE6100` `#FFB000` |
| `category10` | 10 | `#1f77b4` `#ff7f0e` `#2ca02c` `#d62728` `#9467bd` `#8c564b` `#e377c2` `#7f7f7f` `#bcbd22` `#17becf` |
| `pastel` | 10 | `#aec7e8` `#ffbb78` `#98df8a` `#ff9896` `#c5b0d5` `#c49c94` `#f7b6d2` `#c7c7c7` `#dbdb8d` `#9edae5` |
| `bold` | 10 | `#e41a1c` `#377eb8` `#4daf4a` `#984ea3` `#ff7f00` `#a65628` `#f781bf` `#999999` `#66c2a5` `#fc8d62` |

### LTC palettes

Fifteen curated qualitative palettes from [loukesio/ltc-color-palettes](https://github.com/loukesio/ltc-color-palettes)
(MIT), added in kuva 0.6. They are smaller than `category10` — 4 to 9 colours — and each has a deliberate
mood, which makes them a good fit for a figure where the palette is part of the design rather than a
neutral default.

| Palette | N | Colours |
| --- | --- | --- |
| `paloma` | 5 | `#83AF9B` `#C8C8A9` `#F8DA8A` `#F7BF95` `#FE8CA1` |
| `maya` | 5 | `#3D5A80` `#98C1D9` `#E0FBFC` `#EE6C4D` `#293241` |
| `dora` | 5 | `#52777A` `#542437` `#C02942` `#D95B43` `#ECD078` |
| `ploen` | 5 | `#3F5671` `#83A1C3` `#CEB5C8` `#FAC898` `#B17776` |
| `olga` | 5 | `#C9E3C2` `#8BC8CB` `#ECCD80` `#F5AB70` `#9C87A1` |
| `mterese` | 5 | `#F7DDAA` `#FAC3AD` `#F897A1` `#9298BA` `#9CBEED` |
| `franscoise` | 5 | `#5980B1` `#B96A8D` `#A55062` `#E05256` `#E9A986` |
| `fernande` | 4 | `#FF7676` `#F9D662` `#7CAB7D` `#75B7D1` |
| `sylvie` | 5 | `#E8B961` `#E88170` `#C6BDE8` `#5DB7C4` `#FD95BC` |
| `expevo` | 6 | `#FC4E07` `#E7B800` `#00AFBB` `#8B4769` `#1D457F` `#808080` |
| `minou` | 6 | `#00798C` `#D1495B` `#EDAE49` `#66A182` `#2E4057` `#8D96A3` |
| `alger` | 5 | `#000000` `#1A5B5B` `#ACC8BE` `#F4AB5C` `#D1422F` |
| `seafarer` | 5 | `#013D5A` `#FCF3E3` `#BDD3CE` `#708C69` `#E4A25B` |
| `luminaries` | 6 | `#FF5B04` `#075056` `#233038` `#FDF6E3` `#F4D47C` `#D3DBDD` |
| `casa_natal` | 9 | `#245E55` `#ED773C` `#808BC5` `#C63F3E` `#EAC119` `#EAA7C7` `#9ED6DF` `#1D1D1B` `#EAE4DA` |

Three of the names are **aliases** rather than palettes of their own: `deuteranopia` and `protanopia` both
return Wong (safe for red-green deficiency — about 7 % of males between them), and `tritanopia` returns
`tol_bright` (safe for the rare blue-yellow case).

Colours are handed out in order and wrap around, so series seven in a five-colour palette gets colour two.
`category10` is what you get when no palette is set.

**Which to pick:** `wong` / `okabe_ito` is the safest general choice, `tol_muted` has the most colours for a
larger set, and `tol_bright` is the one that stays apart under tritanopia. The LTC palettes are for when
you want a specific look and 4–9 categories are enough. If colour cannot be relied on at all, use
[black & white mode](./bw-mode.md) instead of a palette.

:::note[When a palette applies]

A series that sets its own `color` keeps it. If **no** series sets a colour, `category10` is used so an
overlay does not come out monochrome. Passing `palette` explicitly overrides both rules and paints
every colour-less series from your palette.

:::

## Examples

The Wong palette, with a legend, over three grouped series:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': 'wong',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg({'x': x, 'y': y} ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

Your own list of colours, cycled across the bars:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': {'colors': ['#4c72b0', '#dd8452', '#55a868', '#c44e52']},
  'x_axis': {'name': 'GO term', 'tick_rotate': 60},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## See also

- [Colormaps](./colormaps.md) — continuous colour scales for value-encoded charts.
- [Grid, ticks & canvas switches](./grid.md) — `bw_mode`, the grayscale alternative.
