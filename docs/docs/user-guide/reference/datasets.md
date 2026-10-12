---
title: Datasets (config and data apart)
sidebar_position: 18
description: "The optional second argument of the render functions — a JSON registry of named datasets — and the `dataset` / `encode` fields a series uses to map its columns onto the chart."
---

# Datasets (config and data apart)

`kuva_render`, `kuva_render_terminal` and `kuva_render_file` each take an **optional second argument**: a
JSON object that maps **dataset names** to arrays of rows. A series then names a dataset with `dataset`
and maps its columns onto its own data fields with `encode` — so the chart *config* and the *data* travel
separately.

That split is what you want when the config is built by a host SDK (an ECharts-style one, say) while the
data comes from DuckDB: the SDK writes the config, SQL produces the data with `to_json` / `array_agg`,
and neither side has to know the other's shape.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(
  '{"series":[{"type":"scatter","dataset":"sales","encode":{"x":"revenue","y":"profit"}}]}',
  to_json({'sales': array_agg({'revenue': revenue, 'profit': profit} ORDER BY month)})
) AS chart
FROM (VALUES (1, 10, 3), (2, 12, 5), (3, 9, 4), (4, 15, 8)) AS t(month, revenue, profit);
```

The one-argument form is unchanged: a spec with inline `data` renders exactly as before.

## The data argument

The second argument is a single JSON object. Its keys are dataset names, and each value is an **array of
row objects**:

```json
{ "sales": [ {"x": 1, "y": 2}, {"x": 3, "y": 4} ],
  "temps": [ {"t": 0, "c": 20.5} ] }
```

A row is an object of *field name → value*. A field value may be a scalar or an array — a grouped
distribution's per-group observations are an array, for instance. The datasets are independent:
different names, different fields, different row counts. Every dataset is visible to every series,
including the panels of a [`figure`](./figure.md).

The row form is the only form, on purpose: on the SQL side `array_agg({'x': x, 'y': y})` produces it in
one go, and DuckDB's `STRUCT` fields may have different types while a `LIST`'s may not — so a columnar
`[[x, group], …]` could not even be built.

## Association: `dataset` and `encode`

A series picks a dataset by name and maps that dataset's columns onto its data fields:

| Field | Type | What it does |
| --- | --- | --- |
| `dataset` | string | The dataset this series draws from — a key of the data argument. |
| `encode` | object | Role → field name. Which dataset column feeds which role. |

Two rules keep it readable:

- **Named association only.** There are no `0` / `1` / `2` indices anywhere.
- **Auto-map by name.** A dataset field already named after the role needs no `encode` — a dataset with
  `x` and `y` fields is a scatter with no mapping written at all. An explicit `encode` overrides, and
  there is no positional fallback.

`encode` only **selects columns**. Filtering, joining, grouping and aggregating stay in SQL — the
extension never does them, so a dataset is just "the rows you decided to send".

## Roles

Which roles a series accepts depends on its `type`. Where a chart's data is a list of records (`mosaic`
cells, `sankey` links, `candlestick` candles …), the roles are that record's own field names; where it
is a column or a point list, they are the coordinate/value names.

| Chart family | Types | Roles |
| --- | --- | --- |
| Points | `scatter` `line` | `x` `y`, plus optional `x_err` `y_err` `size` `color` `label` (and `lower` `upper` for a band) |
| One column | `series` `density` `histogram` | `value` (plus `edge` / `count` for a pre-binned histogram) |
| Categories | `bar` `pie` `waffle` `funnel` `pareto` `rose` | `name` (or `label`) and `value` |
| Grouped values | `box` `violin` `ridgeline` `raincloud` `strip` `ecdf` `qq` | `label` `values` `color` |
| Two dimensions | `hexbin` | `x` `y` (and `z`) |
| A matrix, one row per dataset row | `heatmap` `clustermap` | `row`, plus `row_label` / `col_label` |
| Edges | `network` `sankey` | `source` `target` and `weight` / `value` |
| OHLC | `candlestick` | `label` `open` `high` `low` `close` `volume` |
| 3-D points | `scatter3d` | `x` `y` `z` |

The roles above are the common cases. An unknown role in `encode` is reported together with the roles
the type does accept, so a wrong guess names the right ones.

## Examples

A dataset already named `x` / `y` needs no `encode`:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(
  '{"series":[{"type":"line","dataset":"d"}]}',
  to_json({'d': array_agg({'x': x, 'y': y} ORDER BY x)})
) AS chart
FROM (VALUES (0, 1), (1, 3), (2, 2), (3, 5)) AS t(x, y);
```

A grouped distribution: one row per group, with the group's observations as an array — the grouping is a
`GROUP BY` in SQL:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(
  '{"series":[{"type":"violin","dataset":"g","encode":{"label":"k","values":"v"}}]}',
  to_json({'g': array_agg({'k': k, 'v': v} ORDER BY k)})
) AS chart
FROM (
  SELECT "group" AS k, array_agg(value ORDER BY value) AS v
  FROM (VALUES ('a', 1), ('a', 2), ('a', 3), ('b', 4), ('b', 5)) AS t("group", value)
  GROUP BY "group"
);
```

Several datasets, each with its own fields and its own row count:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(
  '{"series":[
     {"type":"line","dataset":"actual","encode":{"x":"t","y":"v"},"legend":"actual"},
     {"type":"line","dataset":"plan","encode":{"x":"t","y":"v"},"legend":"plan"}]}',
  to_json({
    'actual': (SELECT array_agg({'t': t, 'v': v} ORDER BY t) FROM (VALUES (0, 1), (1, 4), (2, 3)) AS a(t, v)),
    'plan':   (SELECT array_agg({'t': t, 'v': v} ORDER BY t) FROM (VALUES (0, 2), (1, 3)) AS p(t, v))
  })
) AS chart;
```

## Errors

As with the one-argument form, a problem fails the statement instead of returning `NULL`. The dataset
errors say what is wrong:

| Situation | Message |
| --- | --- |
| A dataset the config never defined | `unknown dataset 'nope' (the data argument defines: d)` |
| A role this chart type does not have | `encode.z is not a role of this chart type (roles: …)` |
| An `encode` field the dataset does not have | `encode.x refers to 'nope', which dataset 'd' does not have (fields: …)` |
| `dataset` plus inline data for the same field | `dataset fills 'data', so 'data' cannot also be given inline` |
| A `type` that takes no dataset | `text: this chart type takes no dataset` |
| A dataset that is not an array of objects | `dataset 'd' must be an array of row objects` |
| A coordinate cell that is not a number | `dataset 'd' row 1 field 'a' must be a number` |

## Notes

- **`text` and `synteny` take no dataset.** `text` draws one block of prose, which has no per-row
  fields; `synteny` has two record lists (sequences and blocks) whose field names overlap, so one
  dataset cannot express both.
- **The extension does not aggregate.** Two rows landing on the same category and group is a `GROUP BY`
  you have not written yet — do it in SQL and send the result.
- **A dataset and inline data can live on one figure**, just not on the same field of the same series:
  a scatter drawn from a dataset can be overlaid with a line whose `data` is inline.
