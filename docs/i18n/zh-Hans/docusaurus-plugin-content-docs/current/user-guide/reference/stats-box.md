---
title: 统计框
sidebar_position: 11
description: 画在图中一角的小块文字，放样本量、拟合统计量或模型名。
---

# 统计框

统计框是画在图中一角的一块文字 —— 样本量、R²、p 值、模型名。点很密时，用它替代飘在点云上的拟合方程与
R²，那些字会淹没在点里。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `entries` | string[] | 正文，每个字符串一行。 |
| `title` | string | 条目上方的粗体标题。 |
| `position` | string | 框的位置 —— 与 [`legend.position`](./legends.md#位置)同一套取值。 |
| `border` | boolean | 给框画外框。 |

## 示例

把回归的统计量放进右上角：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'stats_box': {
    'title': 'linear fit',
    'entries': ['n = 300', 'R² = 0.38'],
    'position': 'inside_top_right',
    'border': true
  },
  'series': [{'type': 'scatter', 'data': array_agg([x, y]), 'color': 'steelblue', 'trend': 'linear'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

## 说明

点很密时，别用[趋势线](./series.md#趋势线)自带的 `equation` / `correlation`，改成把同样的数字放这里。
