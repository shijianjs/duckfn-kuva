---
title: 极坐标图
sidebar_position: 8
description: 用角度与半径定位的点与曲线。
---

# 极坐标图

极坐标图把数据放在 `(r, θ)` 空间里 —— 半径与角度 —— 画在一张带可调网格的圆形画布上。方向性数据、
周期信号、以及任何绕着圆周测量出来的东西，用它最自然。

默认是**罗盘约定**：`θ = 0` 指向正北（上方），角度顺时针增大。想换成数学约定（`θ = 0` 指东、逆时针），
把 `"theta_start": 90` 与 `"clockwise": false` 一起给上。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Polar plot',
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'r_grid_lines': 4,
    'theta_divisions': 8,
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g, 'mode': 'line'} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
```

角度单位是**度**，不是弧度；一个 series 的 `r` 与 `theta` 是按位置配对的 —— 在聚合里让它们保持同样的排序。

## 散点与折线两种模式

`mode` 决定一个 series 怎么画：

| `mode` | 画成 |
| --- | --- |
| `"scatter"` | 每个 `(r, θ)` 一个 marker（**默认**） |
| `"line"` | 按顺序连接各点的路径 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Directional scatter',
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'theta_divisions': 24,
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
```

## 约定

一条心形线 `r = 1 + cos θ`，用数学约定画：`theta_start` 把 `0°` 放到正东，`clockwise: false` 让角度逆时针
增大。

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT (i * 5.0)::DOUBLE AS th FROM (SELECT unnest(range(0, 72)) AS i))
SELECT kuva_render(to_json({
  'title': 'Cardioid',
  'series': [{
    'type': 'polar',
    'series': [{
      'r': (SELECT list(1.0 + cos(radians(th)) ORDER BY th) FROM t),
      'theta': (SELECT list(th ORDER BY th) FROM t),
      'label': 'Cardioid',
      'mode': 'line'
    }],
    'theta_start': 90,
    'clockwise': false,
    'r_max': 2.1,
    'r_grid_lines': 4,
    'theta_divisions': 12,
    'show_legend': true
  }]
})) AS chart;
```

## marker 不透明度与描边

`marker_opacity` 与 `marker_stroke_width` 只对 `"scatter"` 的 series 有效 —— 折线 series 会忽略它们。
方向性数据一密，降低不透明度就能让密处比边缘更深，细描边则让每个观测还数得清。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Semi-transparent markers',
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'theta_divisions': 24
  }]
})) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g,
               'marker_opacity': 0.3, 'marker_stroke_width': 0.7} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
```

## 半径起点

`r_min` 决定哪个值落在圆心；默认是 `0`。给了非零的 `r_min` 之后，半径为 `r` 的点会画在离圆心
`max(r − r_min, 0) / (r_max − r_min)` 的位置，低于 `r_min` 的一律夹到圆心。这时圆心标签会显示 `r_min`，
刻度不会被误读。

dB 量纲的东西就是靠它画的 —— 天线增益、声压级。下面这个主瓣从 `−20 dBi`（零点）一直到 `0 dBi`：

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT i::DOUBLE AS th FROM (SELECT unnest(range(0, 361)) AS i))
SELECT kuva_render(to_json({
  'title': 'Antenna pattern (dBi)',
  'series': [{
    'type': 'polar',
    'series': [{
      'r': (SELECT list(greatest(-20.0, least(0.0, pow(cos(radians(th) / 2.0), 4) * 20.0 - 20.0))
                      ORDER BY th) FROM t),
      'theta': (SELECT list(th ORDER BY th) FROM t),
      'mode': 'line',
      'color': 'steelblue'
    }],
    'r_min': -20,
    'r_max': 0,
    'r_grid_lines': 4,
    'theta_divisions': 12
  }]
})) AS chart;
```

## 网格控制

| 字段 | 默认 | 作用 |
| --- | --- | --- |
| `r_grid_lines` | `4` | 同心圆网格线的根数 |
| `theta_divisions` | `12` | 圆周方向的辐条数 |
| `show_grid` | `true` | 是否画网格 |
| `show_r_labels` | `true` | 是否给每个圆环标出半径值 |

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `series` | series[] | **必填。** 一条或多条曲线 / 点集（见下）。 |
| `r_max` / `r_min` | number | 半径上下限（默认按数据定）。 |
| `theta_start` | number | 0° 指向哪里（度）。 |
| `clockwise` | boolean | 角度增大方向为顺时针（默认开）。 |
| `r_grid_lines` | integer | 半径方向的网格线根数。 |
| `theta_divisions` | integer | 圆周分成几格。 |
| `show_grid` | boolean | 画网格。 |
| `show_r_labels` | boolean | 标出半径刻度。 |
| `show_legend` | boolean | 显示图例。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每个点一条提示文字。 |

`series` 的每一项带 `r` 与 `theta`（都必填、等长；**theta 单位是度**），外加 `label`、`color`、`mode`
（`"scatter"` 或 `"line"`）、`marker_size`、`stroke_width`、`line_dash`、`marker_opacity`、
`marker_stroke_width`。

## 说明

- **`r` 与 `theta` 必须等长**，series 不能为空。
- 角度单位是度，不是弧度。
- `marker_opacity` / `marker_stroke_width` 对 `"line"` 的 series 无效。
- 图例需要 `show_legend` **且** 至少一个 series 带了 `label`。

## 另见

- [kuva — 极坐标图](https://psy-fer.github.io/kuva/plots/polar.html) —— 绘图库自己的图型参考。
- [雷达图](../categorical/radar.md) · [玫瑰图](../categorical/rose.md) —— 其它环形布局。
- [三元图](./ternary.md) —— 另一种非笛卡尔坐标系。
