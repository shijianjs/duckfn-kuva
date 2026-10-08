---
title: 韦恩图
sidebar_position: 12
description: 2~4 个集合的交叠圆。
---

# 韦恩图

韦恩图用 2~4 个交叠的圆表示集合的交集。给它原始元素让它算交叠，或者给它预算好的集合大小与交集计数。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'venn',
    'sets': sets,
    'counts': true,
    'set_labels': true,
    'fill_opacity': 0.35
  }]
})) AS chart
FROM (
  SELECT list({'label': s, 'elements': els} ORDER BY s) AS sets
  FROM (
    SELECT "set" AS s, list(element) AS els
    FROM read_csv_auto('{{DFK_BASE_URL}}data/venn.tsv')
    GROUP BY "set"
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `sets` | set[] | **必填。** 1~4 个集合。每项是 `{label, elements}` 或 `{label, size}` —— 两种写法不要混。 |
| `overlaps` | overlap[] | 预算好的交集：`{sets: […], size}`。某项的 `size` **含**它的所有子交集。只在 `size` 写法下生效。 |
| `counts` | boolean | 在每个区域里写计数。 |
| `percentages` | boolean | 在每个区域里写百分比。 |
| `set_labels` | boolean | 标出每个集合。 |
| `fill_opacity` | number | 圆的填充不透明度。 |
| `stroke_width` | number | 圆的轮廓线宽。 |
| `proportional` | boolean | 圆的面积正比于集合大小。 |
| `loss` | boolean | 显示 vennEuler 求解器的布局应力。 |
| `colors` | string[] | 逐集合颜色。 |
| `leader_lines` | boolean | 从集合名到圆画引线。 |
| `set_indicators` | boolean | 在圆里画集合名的首字母。 |
| `legend` | string | 图例标题。 |

## 说明

- **只支持 1~4 个集合** —— 超过 4 会画成白图，所以直接报错。
- **两种写法不要混：** 要么每个集合都有 `elements`（此时 `overlaps` 被忽略），要么都没有
  （此时交集由 `overlaps` 提供）。混用会报错。
- `overlaps` 里的集合名必须存在，且同一项里不能重复。

## 另见

- [kuva — 韦恩图](https://psy-fer.github.io/kuva/plots/venn.html) —— 绘图库自己的图型参考。
- [UpSet 图](./upset.md) —— 集合多了也能画。
