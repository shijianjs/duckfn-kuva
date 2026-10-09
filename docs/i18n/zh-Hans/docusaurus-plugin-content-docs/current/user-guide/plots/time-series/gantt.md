---
title: 甘特图
sidebar_position: 7
description: 时间轴上的任务条，可分组、可显示进度、里程碑与「今天」线。
---

# 甘特图

甘特图把任务画成一段横条，横跨一个时间区间，于是排期、时长、先后与重叠都能一眼看出来。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Project plan',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end"} ORDER BY start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

条内放得下就把标签画在条里，放不下就画到条的右侧；右留白会自动撑开，所以画在外面的标签不会被裁掉。

## 分组与阶段

给任务一个 `group`，它就会落在一条带底色的表头行下面，颜色取自调色板。组内的任务保持列表顺序，所以阶段顺序与
阶段内部的顺序都由查询决定。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Phased plan',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks,
    'group_order': ['Design', 'Build', 'Launch']
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end",
               'group': phase,
               'progress': progress,
               'milestone': CASE WHEN milestone IN ('yes', 'true', '1') THEN TRUE ELSE FALSE END}
              ORDER BY phase, start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

`group_order` 显式定下阶段顺序；没列进去的组排在后面、按首次出现的顺序。不给它的话，阶段就按任务到达的顺序出现
—— 所以这个例子先按 `phase` 排序。

## 进度条与「今天」线

`progress`（内部夹到 `0`–`1`）加一段更深的内部填充，表示任务完成了多少 —— 它是把「计划」变成「状态报告」的那一笔。
`now_line` 在指定的 x 上画一条竖直虚线（当前时点），读者于是能看出**到现在为止**本该完成什么。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Progress at week 6',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks,
    'group_order': ['Design', 'Build', 'Launch'],
    'now_line': 6
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end",
               'group': phase,
               'progress': progress,
               'milestone': CASE WHEN milestone IN ('yes', 'true', '1') THEN TRUE ELSE FALSE END}
              ORDER BY phase, start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

## 里程碑

`milestone: true` 把任务变成一个单点时点上的菱形，而不是一条横条 —— 用于签字、封版、发布。把 `start` 与 `end`
设成同一时刻，它的标签会用粗体画在右侧。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With milestones',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks,
    'group_order': ['Design', 'Build', 'Launch'],
    'now_line': 6,
    'milestone_size': 9
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end",
               'group': phase,
               'progress': progress,
               'milestone': CASE WHEN milestone IN ('yes', 'true', '1') THEN TRUE ELSE FALSE END}
              ORDER BY phase, start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

## 尺寸与颜色

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `bar_height` | `0.6` | 条的厚度占行高的比例，内部夹到 `[0.1, 1]` |
| `milestone_size` | `7` | 菱形半径（像素） |
| `show_labels` | `true` | 画任务与里程碑的标签 |
| `color` | `steelblue` | 没有分组时的条色 |
| `group_bg` | `#ebebeb` | 分组表头行的底色 |

任务自己的 `color` 会盖过组色与默认色 —— 想把一个掉队的任务单独标出来、又不想为它单开一个阶段，就用它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Taller bars, tighter rows',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks,
    'group_order': ['Design', 'Build', 'Launch'],
    'bar_height': 0.85,
    'group_bg': '#f0f0f5'
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end",
               'group': phase,
               'progress': progress,
               'milestone': CASE WHEN milestone IN ('yes', 'true', '1') THEN TRUE ELSE FALSE END}
              ORDER BY phase, start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `tasks` | task[] | **必填。** 每项是 `{label, start, end, group?, progress?, color?, milestone?}`。 |
| `now_line` | number | 在这个 x 上画「今天」线。 |
| `group_order` | string[] | 显式的阶段顺序；没列出的组排在后面。 |
| `bar_height` | number | 条的厚度占行高比例（默认 `0.6`）。 |
| `milestone_size` | number | 菱形半径（像素，默认 `7`）。 |
| `show_labels` | boolean | 画标签（默认开）。 |
| `color` | string | 没有分组时的默认条色。 |
| `group_bg` | string | 分组表头底色（默认 `#ebebeb`）。 |
| `legend` | string | 这个系列的图例文字。 |

## 说明

- **`tasks` 不能为空**，且每个任务都要有 `label`、`start`、`end`。
- 时间是纯数字 —— 天、周、月都行，只要一个单位等于一个 x 单位。
- `milestone: true` 的任务应当 `start == end`；跨度不为零时只在起点画一个菱形，终点被忽略。
- `group_order` 不做过滤：任务里出现、但列表里没写的组照样会有表头，只是排在列出的那些之后。
- 组内按列表顺序排，所以查询里的 `ORDER BY` 就是排期本身。

## 另见

- [kuva — 甘特图](https://psy-fer.github.io/kuva/plots/gantt.html) —— 绘图库自己的图型参考。
- [日历热力图](./calendar.md) —— 每日活动，而不是排期任务。
- [K 线图](./candlestick.md) —— 另一种以周期为轴的图。
