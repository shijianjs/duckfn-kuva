---
title: 调色板
sidebar_position: 6
description: 具名调色板，以及怎么传入自己的一组颜色。
---

# 调色板

`palette` 是顶层字段：它决定整张图发给「没有自己设 `color` 的 series」的那些颜色。取值可以是一个名字，
也可以是你自己给的一组颜色字符串。

| 取值 | 说明 |
| --- | --- |
| `"wong"` | Wong 调色板 —— 八个高对比、色盲友好的颜色。 |
| `"okabe_ito"` | Okabe–Ito 调色板。 |
| `"tol_bright"` · `"tol_muted"` · `"tol_light"` | Paul Tol 的三套定性配色。 |
| `"ibm"` | IBM 的色盲友好调色板。 |
| `"deuteranopia"` · `"protanopia"` · `"tritanopia"` | 分别针对常见色盲类型调过的调色板。 |
| `"category10"` | 十个分类色（默认兜底）。 |
| `"pastel"` · `"bold"` | 更柔和与更浓烈的一对变体。 |

给一组颜色就会替换掉具名调色板：

```json
{ "palette": ["#4c72b0", "#dd8452", "#55a868", "#c44e52"] }
```

:::note[调色板什么时候生效]

自己设了 `color` 的 series 保留自己的颜色。如果**没有任何** series 设颜色，就会用 `category10`，免得叠加图
变成清一色。显式传 `palette` 会同时覆盖上面两条规则，把所有没设色的 series 都按你的调色板上色。

:::

## 示例

Wong 调色板 + 图例，配三组 series：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': 'wong',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

自己的一组颜色，在柱子上循环：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': ['#4c72b0', '#dd8452', '#55a868', '#c44e52'],
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

## 另见

- [色图](./colormaps.md) —— 给「用数值编码颜色」的图型用的连续色标。
- [网格、刻度与画布开关](./grid.md) —— `bw_mode`，灰度替代方案。
