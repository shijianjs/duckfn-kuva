---
title: Kaplan-Meier 生存曲线
sidebar_position: 3
description: 每组一条生存曲线，可选置信带、删失标记与 p 值注记。
---

# Kaplan-Meier 生存曲线

生存曲线为每组估计「随时间仍无事件」的比例（Kaplan-Meier）。给每组每个受试者一个随访时间与一个事件
标记。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'survival'},
  'series': [{
    'type': 'survival',
    'groups': grps,
    'ci': true,
    'censoring': true,
    'legend': 'cohort'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'times': ts, 'events': ev} ORDER BY g) AS grps
  FROM (
    SELECT "group" AS g,
           list(time ORDER BY time) AS ts,
           list(event = 1 ORDER BY time) AS ev
    FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv')
    GROUP BY "group"
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每组一条曲线：`{label, times, events, color?}`。 |
| `colors` | string[] | 逐组颜色，按位置与 `groups` 对应。 |
| `line_width` | number | 曲线线宽。 |
| `ci` | boolean | 画置信带。 |
| `ci_alpha` | number | 置信带的不透明度。 |
| `censoring` | boolean | 标出删失观测。 |
| `censoring_size` | number | 删失标记的尺寸。 |
| `pvalue_text` | string | 图上的一行文字（典型是 log-rank 的 p 值 —— 由你自己算好传进来）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **`groups` 不能为空**，且同一组里 `times` 与 `events` 必须等长、非空 —— 对不上会报错，而不是静默 zip。
- **`pvalue_text` 不会替你算**；自己跑 log-rank 检验，把字符串传进来。

## 另见

- [kuva — 生存曲线](https://psy-fer.github.io/kuva/plots/survival.html) —— 绘图库自己的图型参考。
- [统计框](../../reference/stats-box.md) —— 放算好的统计量的另一个位置。
