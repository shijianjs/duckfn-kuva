---
title: 甘特图
sidebar_position: 7
description: 时间轴上的任务条，带分组表头、进度填充与里程碑。
---

# 甘特图

甘特图把任务铺成时间轴上的一条条横条，按分组排在表头下，可选进度填充与菱形里程碑。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'series': [{
    'type': 'gantt',
    'tasks': list({'label': task, 'start': start, 'end': "end", 'group': phase,
                   'progress': progress, 'milestone': (milestone = 'yes')}),
    'bar_height': 0.7,
    'show_labels': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `tasks` | task[] | **必填。** 每个任务一项：`{label, start, end, group?, progress?, color?, milestone?}`。 |
| `now_line` | number | 在这个时间值上画一条「今天」线。 |
| `group_order` | string[] | 分组表头的顺序。 |
| `bar_height` | number | 条的厚度占行高的比例，内部夹到 `[0.1, 1]`。 |
| `milestone_size` | number | 里程碑标记的大小。 |
| `show_labels` | boolean | 标出任务名。 |
| `color` | string | 任务条的默认颜色。 |
| `group_bg` | string | 分组表头的底色。 |
| `legend` | string | 图例标题。 |

## 说明

- **`tasks` 不能为空**，非里程碑的任务不能结束早于开始。
- 里程碑给 `milestone: true`，并把 `end` 设成与 `start` 相同。
- `start` / `end` 就是普通数值 —— kuva 不解释它们，自己保证单位一致即可。

## 另见

- [kuva — 甘特图](https://psy-fer.github.io/kuva/plots/gantt.html) —— 绘图库自己的图型参考。
- [斜率图](../categorical/slope.md) —— 没有时间轴的前后对比。
