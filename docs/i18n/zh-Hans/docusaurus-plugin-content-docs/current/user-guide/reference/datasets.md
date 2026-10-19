---
title: 数据集（配置与数据分离）
sidebar_position: 19
description: "渲染函数的可选第二个参数 —— 一份具名数据集的 JSON 注册表 —— 以及系列用来把数据集的列接到图表上的 `dataset` / `encode` 字段。"
---

# 数据集（配置与数据分离）

`kuva_render`、`kuva_render_terminal`、`kuva_render_typst`、`kuva_render_file` 都有一个**可选的第二个参数**：一个 JSON
对象，把**数据集名**映射到行数组。系列用 `dataset` 挑数据集、用 `encode` 把数据集的列接到自己的数据
字段上 —— 于是图表的**配置**与**数据**各走各的。

当配置由宿主 SDK（比如 ECharts 那一类）构造、而数据来自 DuckDB 时，这个分离正是你要的：SDK 写配置，
SQL 用 `to_json` / `array_agg` 产出数据，两边都不必知道对方的形状。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(
  '{"series":[{"type":"scatter","dataset":"sales","encode":{"x":"revenue","y":"profit"}}]}',
  to_json({'sales': array_agg({'revenue': revenue, 'profit': profit} ORDER BY month)})
) AS chart
FROM (VALUES (1, 10, 3), (2, 12, 5), (3, 9, 4), (4, 15, 8)) AS t(month, revenue, profit);
```

单参数写法完全没变：带内联 `data` 的规格照旧渲染。

## 数据参数

第二个参数是一个 JSON 对象。它的键是数据集名，每个值是一个**行对象数组**：

```json
{ "sales": [ {"x": 1, "y": 2}, {"x": 3, "y": 4} ],
  "temps": [ {"t": 0, "c": 20.5} ] }
```

一行就是一个「字段名 → 值」的对象。字段值可以是标量，也可以是数组 —— 分组分布图型的每组观测值就是
数组。各数据集互相独立：名字不同、字段不同、行数不同。每个数据集对所有系列都可见，包括
[`figure`](./figure.md) 的各个面板。

只保留行式是刻意的：SQL 侧 `array_agg({'x': x, 'y': y})` 一句到位，而且 DuckDB 的 `STRUCT` 字段可以
异型、`LIST` 不行 —— 列式的 `[[x, group], …]` 根本拼不出来。

## 关联：`dataset` 与 `encode`

系列按名字挑数据集，并把数据集的列接到自己的数据字段上：

| 字段 | 类型 | 作用 |
| --- | --- | --- |
| `dataset` | string | 这个系列用哪个数据集 —— 数据参数里的一个键。 |
| `encode` | object | 角色 → 字段名。数据集的哪一列喂给哪个角色。 |

两条规则让它保持好读：

- **一律具名关联**，任何地方都没有 `0` / `1` / `2` 这样的下标。
- **按名自动映射**：数据集字段名本身就是角色名时无需 `encode` —— 字段叫 `x`、`y` 的数据集不用写任何
  映射就是一张散点图。显式 `encode` 覆盖自动映射，且没有按位置的回退。

`encode` **只选列**。过滤、连接、分组、聚合都留在 SQL —— 扩展一概不做，所以一个数据集就是「你决定发
过来的那些行」。

## 角色

一个系列接受哪些角色取决于它的 `type`。图表的数据是「记录列表」的（`mosaic` 的格子、`sankey` 的连边、
`candlestick` 的蜡烛……），角色就是那条记录自己的字段名；数据是「一列」或「点列表」的，角色就是坐标 /
数值的名字。

| 数据族 | 图型 | 角色 |
| --- | --- | --- |
| 点 | `scatter` `line` | `x` `y`，以及可选的 `x_err` `y_err` `size` `color` `label`（置信带用 `lower` `upper`） |
| 一列 | `series` `density` `histogram` | `value`（已分箱的直方图另加 `edge` / `count`） |
| 分类 | `bar` `pie` `waffle` `funnel` `pareto` `rose` | `name`（或 `label`）与 `value` |
| 分组值 | `box` `violin` `ridgeline` `raincloud` `strip` `ecdf` `qq` | `label` `values` `color` |
| 二维 | `hexbin` | `x` `y`（以及 `z`） |
| 矩阵，数据集一行对应矩阵一行 | `heatmap` `clustermap` | `row`，以及 `row_label` / `col_label` |
| 连边 | `network` `sankey` | `source` `target` 与 `weight` / `value` |
| K 线 | `candlestick` | `label` `open` `high` `low` `close` `volume` |
| 三维点 | `scatter3d` | `x` `y` `z` |

上面这些是常见情形。`encode` 里出现未知角色时，报错会连该类型接受的角色一起列出来 —— 猜错也能马上
看到对的。

## 示例

字段已经叫 `x` / `y` 的数据集不需要 `encode`：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(
  '{"series":[{"type":"line","dataset":"d"}]}',
  to_json({'d': array_agg({'x': x, 'y': y} ORDER BY x)})
) AS chart
FROM (VALUES (0, 1), (1, 3), (2, 2), (3, 5)) AS t(x, y);
```

分组分布：一行一组，每组的观测值是一个数组 —— 分组是 SQL 里的 `GROUP BY`：

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

多个数据集，字段与行数各不相同：

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

## 报错

与单参数写法一样，出问题让整条语句失败，而不是返回 `NULL`。数据集相关的报错会说清哪里不对：

| 情形 | 报错 |
| --- | --- |
| 配置里点了一个不存在的数据集 | `unknown dataset 'nope' (the data argument defines: d)` |
| 该图型没有这个角色 | `encode.z is not a role of this chart type (roles: …)` |
| `encode` 指向数据集里没有的字段 | `encode.x refers to 'nope', which dataset 'd' does not have (fields: …)` |
| `dataset` 与同一字段的内联数据同时给 | `dataset fills 'data', so 'data' cannot also be given inline` |
| 这个 `type` 不接受数据集 | `text: this chart type takes no dataset` |
| 数据集不是对象数组 | `dataset 'd' must be an array of row objects` |
| 坐标格不是数 | `dataset 'd' row 1 field 'a' must be a number` |

## 说明

- **`text` 与 `synteny` 不接受数据集。** `text` 画一整段文字，没有逐行字段；`synteny` 有两份字段名互相
  重叠的记录（序列与区块），一份数据集表达不了两者。
- **扩展不做聚合。** 两行落在同一个类别与分组上，说明你还欠一个 `GROUP BY` —— 在 SQL 里做完再发结果。
- **数据集与内联数据可以在同一张图里共存**，只是不能落在同一个系列的同一个字段上：数据集画的散点图
  可以叠上一条 `data` 内联的折线。
