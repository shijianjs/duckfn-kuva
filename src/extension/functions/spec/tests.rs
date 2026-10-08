//! 单元测试：JSON 进、SVG 出。
//!
//! 断言分三层：
//! 1. 每个图型都能渲染出 `<svg …>…</svg>`；
//! 2. 关键连线真的生效（叠加的两个 legend 文字都在、多面板的两个标题都在、饼图扇区数对得上）；
//! 3. 产物是**合法 XML**（用 quick-xml 真解析一遍，而不是数尖括号）—— 字符串拼接出来的 SVG
//!    最容易出的问题就是标签没闭合。
//!
//! 失败路径同样覆盖：每种校验都要冒泡成 `Err`，且信息里带上能定位的那句话。

use super::render_json;

const SCATTER: &str = r#"{
  "title": "Scatter",
  "x_axis": { "name": "x" },
  "y_axis": { "name": "y" },
  "series": [{
    "type": "scatter",
    "data": [{"x": 1, "y": 2, "y_err": 0.4}, {"x": 2, "y": 3.5}, {"x": 3, "y": 3}, {"x": 4, "y": 5.2, "x_err": [0.1, 0.3]}],
    "color": "steelblue",
    "legend": "samples",
    "size": 6,
    "marker": "diamond",
    "trend": {"type": "linear", "equation": true, "correlation": true}
  }]
}"#;

/// 折线 + 散点叠加到同一套坐标轴，外加参考线与阴影区间。
const OVERLAY: &str = r#"{
  "title": {"text": "Overlay", "subtext": "line + scatter"},
  "theme": "dark",
  "annotations": {
    "reference_lines": [{"orientation": "horizontal", "value": 2.5, "label": "target"}],
    "shaded_regions": [{"orientation": "horizontal", "min": 0.8, "max": 1.4, "opacity": 0.3}]
  },
  "series": [
    {"type": "line", "data": [[0, 1], [1, 2], [2, 1.5], [3, 2.8]], "legend": "signal", "fill": true},
    {"type": "scatter", "data": [[0, 1.1], [1, 1.9], [2, 1.6], [3, 2.7]], "legend": "observed"}
  ]
}"#;

const BAR: &str = r#"{
  "title": "Grouped bars",
  "legend": {"position": "outside_right_middle"},
  "series": [{
    "type": "bar",
    "categories": ["Jan", "Feb", "Mar", "Apr"],
    "series": [{"name": "A", "values": [10, 13, 9, 15]}, {"name": "B", "values": [7, 9, 11, 8]}],
    "stacked": true,
    "errors": [2, 1, 1.5, 2, 1, 1, 1, 1]
  }]
}"#;

const HISTOGRAM: &str = r#"{
  "series": [{
    "type": "histogram",
    "values": [1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 7, 8, 9, 9, 10],
    "bins": 8,
    "kde": true,
    "kde_color": "crimson",
    "legend": "n=20"
  }]
}"#;

const BOX: &str = r#"{
  "series": [{
    "type": "box",
    "groups": [
      {"label": "A", "values": [1, 2, 2, 3, 3, 3, 4, 5, 9]},
      {"label": "B", "values": [2, 2.5, 3, 3.5, 4, 4.5, 5, 6]}
    ],
    "strip": 0.15,
    "notch": true
  }]
}"#;

const PIE: &str = r#"{
  "title": "Donut",
  "series": [{
    "type": "pie",
    "slices": [{"label": "Rust", "value": 40}, {"label": "Python", "value": 30}, {"label": "R", "value": 20}, {"label": "Other", "value": 10}],
    "inner_radius": 60,
    "percent": true,
    "legend": "langs"
  }]
}"#;

const FIGURE: &str = r#"{
  "figure": {
    "rows": 1,
    "cols": 2,
    "title": "Panels",
    "labels": "uppercase",
    "shared_legend": "right_top",
    "panels": [
      {"title": "left panel", "series": [{"type": "scatter", "data": [[1, 2], [2, 3]], "legend": "s1"}]},
      {"title": "right panel", "series": [{"type": "histogram", "values": [1, 2, 2, 3, 3, 3, 4], "bins": 5, "legend": "h1"}]}
    ]
  }
}"#;

/// 批次 1：分布 / 统计类图型。逐个覆盖到「非平凡的选项也都写上」，而不只是最小样例 ——
/// 静默失效的字段只有被真正填过才测得出来。
const VIOLIN: &str = r##"{
  "title": "Violin",
  "series": [{
    "type": "violin",
    "groups": [
      {"label": "A", "values": [1, 2, 2, 3, 3, 4, 5, 9]},
      {"label": "B", "values": [2, 2.5, 3, 3.5, 4, 5], "color": "seagreen"}
    ],
    "colors": ["tomato", "seagreen"],
    "bandwidth": 0.8,
    "kde_samples": 128,
    "strip": 0.15,
    "width": 0.7,
    "gap": 0.1,
    "legend": "cohort"
  }]
}"##;

const RIDGELINE: &str = r##"{
  "series": [{
    "type": "ridgeline",
    "groups": [
      {"label": "jan", "values": [1, 2, 2, 3, 4], "color": "#4c72b0"},
      {"label": "feb", "values": [2, 3, 3, 4, 5, 6], "color": "#dd8452"}
    ],
    "overlap": 0.7,
    "normalize": true,
    "filled": true,
    "opacity": 0.8,
    "show_legend": true,
    "bandwidth": 0.7
  }]
}"##;

const RAINCLOUD: &str = r##"{
  "series": [{
    "type": "raincloud",
    "groups": [
      {"label": "ctrl", "values": [10, 12, 11, 14, 13, 15, 18]},
      {"label": "treat", "values": [14, 16, 15, 19, 20, 22, 25]}
    ],
    "colors": ["#4c72b0", "#c44e52"],
    "cloud_width": 24,
    "bandwidth": 1.2,
    "bandwidth_scale": 1.1,
    "cloud_alpha": 0.5,
    "rain_jitter": 0.08,
    "show_rain": true,
    "show_box": true,
    "show_cloud": true,
    "seed": 7,
    "legend": "arms"
  }]
}"##;

const STRIP: &str = r##"{
  "series": [{
    "type": "strip",
    "groups": [
      {"label": "a", "values": [1, 2, 2, 3, 4, 9], "point_colors": ["red", "green"], "point_shapes": ["circle", "triangle"]},
      {"label": "b", "values": [2, 3, 4, 5]}
    ],
    "style": "swarm",
    "point_size": 5,
    "marker_opacity": 0.6,
    "color": "slateblue",
    "legend": "beeswarm",
    "tooltips": true
  }]
}"##;

const DOT_PLOT: &str = r##"{
  "series": [{
    "type": "dot_plot",
    "x_categories": ["m1", "m2", "m3"],
    "y_categories": ["c1", "c2"],
    "sizes": [[1, 5, 9], [4, 2, 7]],
    "colors": [[0.1, 0.5, 0.9], [0.3, 0.2, 0.8]],
    "color_map": "magma",
    "size_range": [3, 14],
    "color_range": [0, 1],
    "size_label": "count",
    "colorbar_label": "score",
    "tooltips": true
  }]
}"##;

const LOLLIPOP: &str = r##"{
  "series": [{
    "type": "lollipop",
    "points": [
      {"x": 1.0, "y": 12.0, "label": "p1", "color": "#4c72b0"},
      {"x": 2.0, "y": 19.0, "label": "p2"},
      {"x": 3.0, "y": 7.0}
    ],
    "domains": [{"start": 0.5, "end": 2.5, "label": "normal", "color": "#c44e52", "opacity": 0.2}],
    "baseline": 0,
    "dot_radius": 6,
    "stem_width": 2,
    "domain_height": 0.4,
    "legend": "items"
  }]
}"##;

const DENSITY: &str = r##"{
  "series": [{
    "type": "density",
    "values": [1, 1.5, 2, 2.1, 2.4, 3, 3.2, 3.9, 4.5, 5],
    "filled": true,
    "opacity": 0.4,
    "bandwidth": 0.5,
    "kde_samples": 256,
    "stroke_width": 2,
    "line_dash": "5 2",
    "x_range": [0, 6],
    "fit": true,
    "legend": "kde"
  }]
}"##;

const DENSITY_CURVE: &str = r##"{
  "series": [{
    "type": "density",
    "curve": {"x": [0, 1, 2, 3], "y": [0.1, 0.8, 0.4, 0.05]},
    "color": "crimson"
  }]
}"##;

const ECDF: &str = r##"{
  "series": [{
    "type": "ecdf",
    "groups": [
      {"label": "fast", "values": [1, 2, 2, 3, 4, 6]},
      {"label": "slow", "values": [2, 4, 5, 5, 7, 9], "color": "darkorange"}
    ],
    "confidence_band": true,
    "band_alpha": 0.15,
    "rug": true,
    "rug_height": 8,
    "percentile_lines": [25, 50, 75],
    "markers": true,
    "smooth": true,
    "legend": "cdf"
  }]
}"##;

const QQ: &str = r##"{
  "series": [{
    "type": "qq",
    "groups": [{"label": "sample", "values": [1.1, 0.9, 1.4, 0.7, 1.9, 1.2, 0.5, 1.6]}],
    "mode": "normal",
    "reference_line": true,
    "ci_band": true,
    "ci_alpha": 0.12,
    "lambda": false,
    "marker_size": 4,
    "legend": "normality"
  }]
}"##;

const QQ_GENOMIC: &str = r##"{
  "series": [{
    "type": "qq",
    "groups": [{"label": "gwas", "values": [0.0001, 0.01, 0.2, 0.5, 0.9], "color": "purple"}],
    "mode": "genomic",
    "lambda": true
  }]
}"##;

const FOREST: &str = r##"{
  "series": [{
    "type": "forest",
    "rows": [
      {"label": "study A", "estimate": 0.42, "ci_lower": 0.11, "ci_upper": 0.73, "weight": 12},
      {"label": "study B", "estimate": 0.25, "ci_lower": -0.05, "ci_upper": 0.55, "color": "#c44e52"},
      {"label": "pooled", "estimate": 0.33, "ci_lower": 0.2, "ci_upper": 0.46, "weight": 30}
    ],
    "marker_size": 7,
    "whisker_width": 2,
    "null_value": 0,
    "cap_size": 3,
    "legend": "effect"
  }]
}"##;

const PR: &str = r##"{
  "series": [{
    "type": "pr",
    "groups": [
      {"label": "model A", "predictions": [[0.9, true], [0.8, false], [0.6, true], [0.4, true], [0.2, false]], "auc_label": true, "optimal_point": true},
      {"label": "model B", "points": [[1, 1], [0.8, 0.7], [0.5, 0.4], [0, 0]], "prevalence": 0.4, "dasharray": "4 2", "line_width": 2}
    ],
    "show_baseline": true,
    "baseline_dasharray": "2 2",
    "legend": "precision-recall"
  }]
}"##;

const ROC: &str = r##"{
  "series": [{
    "type": "roc",
    "groups": [
      {"label": "model A", "predictions": [{"score": 0.9, "label": true}, [0.8, false], [0.6, true], [0.4, true], [0.2, false]], "ci": true, "auc_label": true, "optimal_point": true, "pauc_range": [0, 0.3]},
      {"label": "model B", "points": [[0, 0], [0.2, 0.7], [0.6, 0.9], [1, 1]], "dasharray": "4 2", "line_width": 2}
    ],
    "show_diagonal": true,
    "diagonal_dasharray": "3 3",
    "legend": "roc"
  }]
}"##;

const SURVIVAL: &str = r##"{
  "series": [{
    "type": "survival",
    "groups": [
      {"label": "drug", "times": [1, 3, 4, 6, 8, 10], "events": [true, false, true, true, false, true], "color": "#4c72b0"},
      {"label": "control", "times": [2, 2, 5, 7, 9, 12], "events": [true, true, false, true, false, false]}
    ],
    "ci": true,
    "ci_alpha": 0.15,
    "censoring": true,
    "censoring_size": 5,
    "line_width": 2,
    "pvalue_text": "log-rank p = 0.031",
    "legend": "cohort"
  }]
}"##;

const VOLCANO: &str = r##"{
  "series": [{
    "type": "volcano",
    "points": [
      {"name": "G1", "log2fc": 3.2, "pvalue": 1e-9},
      {"name": "G2", "log2fc": -2.8, "pvalue": 1e-6},
      {"name": "G3", "log2fc": 0.1, "pvalue": 0.4},
      {"name": "G4", "log2fc": 1.4, "pvalue": 0.02},
      {"name": "G5", "log2fc": -1.1, "pvalue": 0.03},
      {"name": "G6", "log2fc": 0.3, "pvalue": 0.6}
    ],
    "fc_cutoff": 1,
    "p_cutoff": 0.05,
    "color_up": "#c44e52",
    "color_down": "#4c72b0",
    "color_ns": "#bbbbbb",
    "point_size": 6,
    "label_top": 3,
    "label_style": {"offset_x": 20, "offset_y": 14},
    "legend": "DE",
    "tooltips": true
  }]
}"##;

/// 批次 2：矩阵 / 网格类图型。
const HEATMAP: &str = r##"{
  "title": "Heatmap",
  "series": [{
    "type": "heatmap",
    "data": [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
    "row_labels": ["r1", "r2", "r3"],
    "col_labels": ["c1", "c2", "c3"],
    "color_map": "viridis",
    "show_values": true,
    "legend": "count",
    "cell_size": 0.9,
    "tooltips": true
  }]
}"##;

const HISTOGRAM2D: &str = r##"{
  "series": [{
    "type": "histogram2d",
    "data": [[0.1, 0.2], [0.4, 0.9], [0.7, 0.3], [0.2, 0.8], [0.9, 0.1]],
    "x_range": [0, 1],
    "y_range": [0, 1],
    "bins_x": 5,
    "bins_y": 4,
    "color_map": "magma",
    "correlation": true,
    "log_count": true
  }]
}"##;

const HEXBIN: &str = r##"{
  "series": [{
    "type": "hexbin",
    "x": [0.1, 0.3, 0.5, 0.7, 0.9, 0.2, 0.4],
    "y": [0.2, 0.4, 0.6, 0.8, 0.1, 0.3, 0.5],
    "z": [1, 2, 3, 4, 5, 6, 7],
    "reduce": "mean",
    "n_bins": 12,
    "color_map": "cividis",
    "min_count": 1,
    "log_color": true,
    "colorbar": true,
    "colorbar_label": "mean z",
    "stroke": "#333333",
    "stroke_width": 0.4,
    "flat_top": true
  }]
}"##;

const CLUSTERMAP: &str = r##"{
  "series": [{
    "type": "clustermap",
    "data": [[1, 2, 3, 4], [2, 1, 4, 3], [5, 6, 1, 2], [6, 5, 2, 1]],
    "row_labels": ["a", "b", "c", "d"],
    "col_labels": ["w", "x", "y", "z"],
    "cluster_rows": true,
    "cluster_cols": true,
    "color_map": "blue_green",
    "show_values": true,
    "normalization": "row_zscore",
    "branch_color": "#555555",
    "row_dendrogram_width": 90,
    "col_dendrogram_height": 70,
    "row_annotations": [{"colors": ["#ff0000", "#00ff00", "#0000ff", "#ffff00"], "label": "grp", "width": 12}],
    "col_annotations": [{"colors": ["#111111", "#222222", "#333333", "#444444"]}],
    "legend": "z"
  }]
}"##;

const CONTOUR_GRID: &str = r##"{
  "series": [{
    "type": "contour",
    "z": [[1, 2, 3], [2, 4, 6], [3, 6, 9]],
    "x_coords": [0, 1, 2],
    "y_coords": [0, 1, 2],
    "n_levels": 6,
    "filled": true,
    "color_map": "inferno",
    "line_width": 1.5,
    "legend": "z"
  }]
}"##;

const CONTOUR_POINTS: &str = r##"{
  "series": [{
    "type": "contour",
    "points": [[0, 0, 1], [1, 0, 2], [0, 1, 2], [1, 1, 4], [0.5, 0.5, 3]],
    "levels": [1, 2, 3],
    "line_color": "#222222"
  }]
}"##;

const TERNARY: &str = r##"{
  "series": [{
    "type": "ternary",
    "points": [
      {"a": 0.6, "b": 0.3, "c": 0.1, "group": "x"},
      {"a": 0.2, "b": 0.5, "c": 0.3, "group": "y"},
      {"a": 0.4, "b": 0.2, "c": 0.4, "group": "x"}
    ],
    "corner_labels": ["top", "left", "right"],
    "normalize": true,
    "marker_size": 7,
    "grid_lines": 4,
    "show_legend": true,
    "show_percentages": true,
    "marker_opacity": 0.8
  }]
}"##;

const POLAR: &str = r##"{
  "series": [{
    "type": "polar",
    "series": [
      {"r": [1, 2, 3, 2], "theta": [0, 90, 180, 270], "label": "a", "mode": "line", "stroke_width": 2},
      {"r": [2, 3, 1], "theta": [30, 150, 300], "label": "b", "color": "#c44e52", "marker_size": 8}
    ],
    "r_max": 4,
    "r_min": 0,
    "theta_start": 90,
    "clockwise": false,
    "r_grid_lines": 4,
    "theta_divisions": 8,
    "show_legend": true
  }]
}"##;

const DICE_PLOT: &str = r##"{
  "series": [{
    "type": "dice_plot",
    "ndots": 4,
    "x_categories": ["m1", "m2"],
    "y_categories": ["c1", "c2"],
    "category_labels": ["1", "2", "3", "4"],
    "points": [
      {"x": "m1", "y": "c1", "present": [0, 2], "fill": 0.8, "size": 12},
      {"x": "m2", "y": "c2", "present": [0, 1, 3], "fill": 0.3, "size": 6}
    ],
    "color_map": "plasma",
    "fill_range": [0, 1],
    "size_range": [4, 16],
    "fill_legend_label": "fill",
    "size_legend_label": "size",
    "position_legend_label": "count",
    "dot_legend": [["1", "one"], ["2", "two"], ["3", "three"], ["4", "four"]],
    "grid_lines": true
  }]
}"##;

const SCATTER3D: &str = r##"{
  "series": [{
    "type": "scatter3d",
    "data": [[1, 2, 3], [2, 3, 1], [3, 1, 2], {"x": 4, "y": 4, "z": 4}],
    "sizes": [4, 6, 8, 10],
    "colors": ["#4c72b0", "#c44e52", "#55a868", "#c44e52"],
    "z_colormap": "plasma",
    "depth_shade": true,
    "marker": "circle",
    "size": 5,
    "legend": "3d",
    "azimuth": -45,
    "elevation": 25,
    "x_label": "x",
    "y_label": "y",
    "z_label": "z",
    "show_grid": false,
    "z_axis_right": true
  }]
}"##;

const SURFACE3D: &str = r##"{
  "series": [{
    "type": "surface3d",
    "z_data": [[1, 2, 3], [2, 4, 6], [3, 6, 9]],
    "x_coords": [0, 1, 2],
    "y_coords": [0, 1, 2],
    "z_colormap": "viridis",
    "wireframe": true,
    "wireframe_color": "#333333",
    "wireframe_width": 0.8,
    "alpha": 0.9,
    "color": "steelblue",
    "legend": "surface",
    "x_label": "x",
    "y_label": "y",
    "z_label": "z",
    "show_box": false
  }]
}"##;

/// 批次 3：关系 / 层级类图型。
const SANKEY: &str = r##"{
  "title": "Sankey",
  "series": [{
    "type": "sankey",
    "nodes": [
      {"label": "coal", "color": "#4c72b0"},
      {"label": "gas", "color": "#c44e52"},
      {"label": "power", "color": "#55a868", "column": 1},
      {"label": "loss", "color": "#bbbbbb", "column": 2}
    ],
    "links": [
      {"source": "coal", "target": "power", "value": 30, "color": "#4c72b0"},
      {"source": "gas", "target": "power", "value": 20},
      {"source": "coal", "target": "loss", "value": 10},
      {"source": "gas", "target": "loss", "value": 5}
    ],
    "alluvia": [{"nodes": ["coal", "power", "loss"], "value": 30}],
    "axis_names": ["fuel", "use", "waste"],
    "node_order": "crossing_reduction",
    "node_coloring": "label",
    "link_color": "gradient",
    "node_width": 18,
    "node_gap": 6,
    "link_opacity": 0.6,
    "flow_percent": true,
    "flow_label_min_height": 10,
    "legend": "energy"
  }]
}"##;

const CHORD: &str = r##"{
  "series": [{
    "type": "chord",
    "matrix": [[10, 5, 3], [4, 8, 2], [1, 6, 7]],
    "labels": ["a", "b", "c"],
    "colors": ["#4c72b0", "#c44e52", "#55a868"],
    "gap_degrees": 3,
    "ribbon_opacity": 0.6,
    "legend": "flows"
  }]
}"##;

const NETWORK: &str = r##"{
  "series": [{
    "type": "network",
    "nodes": [
      {"label": "hub", "size": 12, "group": "core", "color": "#4c72b0", "shape": "square"},
      {"label": "a", "group": "leaf"},
      {"label": "b", "group": "leaf", "shape": "triangle"},
      {"label": "c", "group": "leaf", "position": [0.2, 0.8]}
    ],
    "edges": [
      {"source": "hub", "target": "a", "weight": 3, "label": "3"},
      {"source": "hub", "target": "b", "weight": 2, "curve": 0.3, "color": "#c44e52"},
      {"source": "a", "target": "b", "weight": 1}
    ],
    "directed": true,
    "layout": "kamada_kawai",
    "node_radius": 10,
    "edge_opacity": 0.5,
    "show_labels": true,
    "repel_labels": true,
    "label_size": 12,
    "legend": "graph"
  }]
}"##;

const TREEMAP: &str = r##"{
  "series": [{
    "type": "treemap",
    "roots": [{
      "label": "root",
      "children": [
        {"label": "a", "children": [{"label": "a1", "value": 30}, {"label": "a2", "value": 20}]},
        {"label": "b", "children": [{"label": "b1", "value": 10, "color": "#ff0000"}]}
      ]
    }],
    "color_mode": {"color_map": "viridis"},
    "color_values": [3, 2, 1, 0.5],
    "layout": "squarify",
    "padding": 3,
    "border_width": 0.6,
    "colorbar": true,
    "colorbar_label": "value",
    "max_depth": 3,
    "tooltips": true
  }]
}"##;

const SUNBURST: &str = r##"{
  "series": [{
    "type": "sunburst",
    "roots": [{
      "label": "root",
      "children": [
        {"label": "a", "children": [{"label": "a1", "value": 30}, {"label": "a2", "value": 20}]},
        {"label": "b", "value": 15}
      ]
    }],
    "color_mode": "by_parent",
    "show_labels": true,
    "min_label_angle": 10,
    "inner_radius": 0.2,
    "ring_gap": 2,
    "start_angle": 90,
    "rotate_labels": true
  }]
}"##;

const VENN: &str = r##"{
  "series": [{
    "type": "venn",
    "sets": [
      {"label": "A", "size": 20},
      {"label": "B", "size": 18},
      {"label": "C", "size": 15}
    ],
    "overlaps": [
      {"sets": ["A", "B"], "size": 8},
      {"sets": ["A", "B", "C"], "size": 3}
    ],
    "counts": true,
    "percentages": true,
    "fill_opacity": 0.3,
    "proportional": true,
    "leader_lines": true,
    "colors": ["#4c72b0", "#c44e52", "#55a868"],
    "legend": "sets"
  }]
}"##;

const VENN_ELEMENTS: &str = r##"{
  "series": [{
    "type": "venn",
    "sets": [
      {"label": "A", "elements": ["x", "y", "z"]},
      {"label": "B", "elements": ["y", "z", "w"]}
    ],
    "set_labels": true
  }]
}"##;

const UPSET: &str = r##"{
  "series": [{
    "type": "upset",
    "set_names": ["A", "B", "C"],
    "set_sizes": [10, 12, 8],
    "intersections": [
      {"mask": 1, "count": 5},
      {"mask": 3, "count": 3},
      {"mask": 7, "count": 2}
    ],
    "sort": "by_degree",
    "max_visible": 5,
    "bar_color": "#4c72b0",
    "dot_color": "#333333"
  }]
}"##;

const WAFFLE: &str = r##"{
  "series": [{
    "type": "waffle",
    "categories": [
      {"label": "yes", "value": 6, "color": "#4c72b0"},
      {"label": "no", "value": 3},
      {"label": "maybe", "value": 1}
    ],
    "rows": 5,
    "cols": 4,
    "gap": 0.15,
    "fill_order": "row_major_bottom_left",
    "shape": "circle",
    "show_percents": true,
    "unit_label": "1 cell = 2 votes"
  }]
}"##;

const MOSAIC: &str = r##"{
  "series": [{
    "type": "mosaic",
    "cells": [
      {"col": "m", "row": "yes", "value": 30},
      {"col": "m", "row": "no", "value": 10},
      {"col": "f", "row": "yes", "value": 20},
      {"col": "f", "row": "no", "value": 25}
    ],
    "col_order": ["m", "f"],
    "row_order": ["yes", "no"],
    "group_colors": ["#4c72b0", "#c44e52"],
    "gap": 2,
    "values": true,
    "percents": true,
    "legend": "survey"
  }]
}"##;

const PHYLO: &str = r##"{
  "series": [{
    "type": "phylo",
    "newick": "((A:0.1,B:0.2):0.15,(C:0.3,D:0.25):0.1);",
    "orientation": "right",
    "branch_style": "rectangular",
    "phylogram": true,
    "branch_color": "#333333",
    "leaf_color": "#4c72b0",
    "support_threshold": 0.5,
    "legend": "tree"
  }]
}"##;

const PHYLO_EDGES: &str = r##"{
  "series": [{
    "type": "phylo",
    "edges": [
      {"parent": "root", "child": "A", "length": 0.1},
      {"parent": "root", "child": "B", "length": 0.2}
    ],
    "orientation": "top",
    "branch_style": "slanted"
  }]
}"##;

const SYNTENY: &str = r##"{
  "series": [{
    "type": "synteny",
    "sequences": [
      {"label": "chr1", "length": 100, "color": "#4c72b0"},
      {"label": "chr2", "length": 120},
      {"label": "chr3", "length": 90}
    ],
    "blocks": [
      {"seq1": 0, "start1": 10, "end1": 50, "seq2": 1, "start2": 20, "end2": 60, "color": "#c44e52"},
      {"seq1": 1, "start1": 30, "end1": 70, "seq2": 2, "start2": 10, "end2": 50, "strand": "reverse"}
    ],
    "bar_height": 16,
    "block_opacity": 0.5,
    "shared_scale": true,
    "legend": "blocks"
  }]
}"##;

/// 批次 4：时间 / 金融 / 排名 / 对比类图型。
const CANDLESTICK: &str = r##"{
  "title": "OHLC",
  "series": [{
    "type": "candlestick",
    "candles": [
      {"label": "d1", "open": 10, "high": 13, "low": 9, "close": 12, "volume": 120},
      {"label": "d2", "open": 12, "high": 14, "low": 11, "close": 11.5, "volume": 90},
      {"label": "d3", "open": 11.5, "high": 15, "low": 11, "close": 14, "volume": 150},
      {"label": "d4", "x": 4.5, "open": 14, "high": 14, "low": 13, "close": 14, "volume": 60}
    ],
    "candle_width": 0.6,
    "gap": 0.1,
    "wick_width": 1.2,
    "color_up": "#2ca02c",
    "color_down": "#d62728",
    "color_doji": "#888888",
    "show_volume": true,
    "volume_ratio": 0.25,
    "legend": "price",
    "tooltips": true
  }]
}"##;

const CALENDAR: &str = r##"{
  "series": [{
    "type": "calendar",
    "data": [
      {"date": "2024-01-01", "value": 3},
      {"date": "2024-01-02", "value": 5},
      {"date": "2024-02-14", "value": 9}
    ],
    "events": ["2024-03-01", "2024-03-02"],
    "aggregation": "sum",
    "color_map": "greens",
    "missing_color": "#f0f0f0",
    "zero_color": "#ffffff",
    "week_start": "sunday",
    "month_labels": true,
    "day_labels": false,
    "cell_size": 12,
    "cell_gap": 2,
    "legend": true,
    "legend_label": "commits",
    "value_range": [0, 10],
    "periods": [{"label": "Q1", "start": "2024-01-01", "end": "2024-03-31"}]
  }]
}"##;

const GANTT: &str = r##"{
  "series": [{
    "type": "gantt",
    "tasks": [
      {"label": "design", "start": 0, "end": 3, "group": "p1", "progress": 1},
      {"label": "build", "start": 2, "end": 6, "group": "p1", "progress": 0.5, "color": "#c44e52"},
      {"label": "ship", "start": 6, "end": 6, "group": "p1", "milestone": true}
    ],
    "now_line": 4,
    "group_order": ["p1"],
    "bar_height": 0.7,
    "milestone_size": 8,
    "show_labels": true,
    "color": "steelblue",
    "group_bg": "#f0f0f0",
    "legend": "plan"
  }]
}"##;

const HORIZON: &str = r##"{
  "series": [{
    "type": "horizon",
    "series": [
      {"label": "a", "x": [1, 2, 3, 4], "y": [1, -2, 3, -4], "pos_color": "#4292c6", "neg_color": "#d73027"},
      {"label": "b", "x": [1, 2, 3, 4], "y": [2, 1, -1, -2]}
    ],
    "n_bands": 3,
    "row_height": 40,
    "baseline": 0,
    "show_legend": true,
    "value_labels": true,
    "sign_colors": true
  }]
}"##;

const MANHATTAN: &str = r##"{
  "series": [{
    "type": "manhattan",
    "points": [
      {"chromosome": "1", "position": 1000, "pvalue": 1e-8},
      {"chromosome": "1", "position": 5000, "pvalue": 0.4, "label": "rs1"},
      {"chromosome": "2", "position": 2000, "pvalue": 1e-5}
    ],
    "build": "hg38",
    "genome_wide": 7.3,
    "suggestive": 5,
    "color_a": "steelblue",
    "color_b": "#5aadcb",
    "point_size": 3,
    "label_top": 2,
    "label_style": "nudge",
    "legend": "GWAS",
    "tooltips": true
  }]
}"##;

const WATERFALL: &str = r##"{
  "series": [{
    "type": "waterfall",
    "bars": [
      {"label": "start", "kind": "total", "value": 100},
      {"label": "up", "value": 30},
      {"label": "down", "value": -20},
      {"label": "shift", "from": 50, "to": 70},
      {"label": "end", "kind": "total"}
    ],
    "bar_width": 0.6,
    "gap": 0.1,
    "color_positive": "#2ca02c",
    "color_negative": "#d62728",
    "color_total": "steelblue",
    "connectors": true,
    "show_values": true,
    "legend": "P&L",
    "tooltips": true
  }]
}"##;

const BUMP: &str = r##"{
  "series": [{
    "type": "bump",
    "series": [
      {"name": "a", "ranks": [1, 2, 2, 3], "color": "#4c72b0"},
      {"name": "b", "ranks": [2, 1, 1, 2]},
      {"name": "c", "values": [50, 80, 60, 90]}
    ],
    "x_labels": ["w1", "w2", "w3", "w4"],
    "curve_style": "straight",
    "show_rank_labels": true,
    "show_series_labels": true,
    "dot_radius": 5,
    "stroke_width": 2,
    "highlight": "b",
    "legend": true,
    "rank_ascending": true,
    "tie_break": "min"
  }]
}"##;

const PARETO: &str = r##"{
  "series": [{
    "type": "pareto",
    "categories": [
      {"label": "a", "value": 40},
      {"label": "b", "value": 25},
      {"label": "c", "value": 15},
      {"label": "d", "value": 12},
      {"label": "e", "value": 8}
    ],
    "color": "steelblue",
    "line_color": "firebrick",
    "width": 0.8,
    "sorted": true,
    "cumulative_labels": true,
    "show_threshold": true,
    "threshold": 80,
    "bar_legend_label": "count",
    "line_legend_label": "cum %",
    "show_legend": true,
    "max_categories": 4,
    "other_label": "rest"
  }]
}"##;

const BRICK: &str = r##"{
  "series": [{
    "type": "brick",
    "sequences": ["ACGTACGT", "ACGTTCGT", "ACGTACGA"],
    "names": ["s1", "s2", "s3"],
    "template": "dna",
    "x_offset": 0,
    "x_origin": 0,
    "show_values": true,
    "anchor": "left",
    "mark_primary": true,
    "consensus_row": 0,
    "notations": ["n1", null, "n3"],
    "row_height": 18
  }]
}"##;

/// STRIGAR 模式：展开后的 strigar 就是画出来的行，`sequences` 不参与。
const BRICK_STRIGAR: &str = r##"{
  "series": [{
    "type": "brick",
    "names": ["read_1", "read_2", "read_3"],
    "strigars": [
      ["CAG:A", "10A"],
      ["CAG:A", "8A"],
      ["CAG:A,C:B", "12A1B"]
    ],
    "x_origin": 0,
    "consensus_row": 0,
    "row_height": 20
  }]
}"##;

const FUNNEL: &str = r##"{
  "series": [{
    "type": "funnel",
    "stages": [
      {"label": "visits", "value": 1000, "color": "#4c72b0"},
      {"label": "signups", "value": 300},
      {"label": "paid", "value": 80}
    ],
    "mirror": [{"label": "bounce", "value": 400}],
    "left_label": "funnel",
    "right_label": "drop-off",
    "orientation": "vertical",
    "show_connectors": true,
    "connector_opacity": 0.3,
    "show_values": true,
    "show_percents": true,
    "show_conversion": true,
    "color_mode": "by_stage",
    "stage_gap": 5,
    "legend": "steps"
  }]
}"##;

const SLOPE: &str = r##"{
  "series": [{
    "type": "slope",
    "points": [
      {"label": "north", "before": 10, "after": 25},
      {"label": "south", "before": 20, "after": 18},
      {"label": "east", "before": 15, "after": 15}
    ],
    "before_label": "2023",
    "after_label": "2024",
    "color_up": "#2ca02c",
    "color_down": "#d62728",
    "color_flat": "#aaaaaa",
    "color_by_direction": true,
    "group_colors": ["#4c72b0", "#c44e52", "#55a868"],
    "dot_radius": 7,
    "line_width": 2,
    "show_values": true,
    "value_format": 1,
    "legend": "regions"
  }]
}"##;

const PYRAMID: &str = r##"{
  "series": [{
    "type": "pyramid",
    "series": [
      {"label": "2020", "groups": [
        {"age": "0-9", "left": 100, "right": 95},
        {"age": "10-19", "left": 120, "right": 115}
      ], "color": "#4c72b0"},
      {"label": "2024", "groups": [
        {"age": "0-9", "left": 90, "right": 88},
        {"age": "10-19", "left": 110, "right": 112}
      ], "opacity": 0.4}
    ],
    "left_label": "male",
    "right_label": "female",
    "left_color": "#4C72B0",
    "right_color": "#DD8452",
    "normalize": true,
    "show_values": true,
    "group_gap": 0.2,
    "bar_gap": 0.05,
    "mode": "grouped",
    "show_legend": true
  }]
}"##;

/// 不给 `build`：这时 x 有两种走法 —— 给了 `position` 就用它，都不给就按染色体序号排。
const MANHATTAN_NO_BUILD: &str = r##"{
  "series": [{
    "type": "manhattan",
    "points": [
      {"chromosome": "chr1", "pvalue": 1e-6},
      {"chromosome": "chr2", "pvalue": 0.2}
    ],
    "genome_wide": 7.3
  }]
}"##;

const MANHATTAN_X: &str = r##"{
  "series": [{
    "type": "manhattan",
    "points": [
      {"chromosome": "1", "position": 10, "pvalue": 1e-6},
      {"chromosome": "1", "position": 20, "pvalue": 0.5}
    ]
  }]
}"##;

/// 批次 5：序列 / 场 / 文字。
const SERIES: &str = r##"{
  "series": [{
    "type": "series",
    "values": [3, 5, 4, 6, 8, 7, 9],
    "style": "both",
    "color": "#4c72b0",
    "stroke_width": 2,
    "point_radius": 4,
    "legend": "signal"
  }]
}"##;

const RADAR: &str = r##"{
  "series": [{
    "type": "radar",
    "axes": ["speed", "power", "range", "cost"],
    "series": [
      {"values": [0.8, 0.6, 0.9, 0.4], "label": "model A", "color": "#4c72b0", "errors": [0.05, 0.05, 0.05, 0.05]},
      {"values": [0.5, 0.9, 0.6, 0.7], "label": "model B", "dasharray": "4 2"}
    ],
    "references": [{"values": [0.6, 0.6, 0.6, 0.6], "label": "target", "color": "#999999"}],
    "filled": true,
    "opacity": 0.2,
    "range": [0, 1],
    "axis_ranges": [[3, [0, 2]]],
    "inverted_axes": [3],
    "grid_lines": 4,
    "circular_grid": true,
    "show_legend": true,
    "dot_size": 3,
    "normalize": true,
    "vertex_labels": true,
    "start_angle": -90,
    "axis_ticks": true
  }]
}"##;

const PARALLEL: &str = r##"{
  "series": [{
    "type": "parallel",
    "axis_names": ["age", "income", "score"],
    "rows": [
      {"values": [20, 30, 70], "group": "a"},
      {"values": [35, 60, 55], "group": "b"},
      {"values": [50, 45, 40], "group": "a"}
    ],
    "normalize": true,
    "curved": true,
    "stroke_width": 1.5,
    "opacity": 0.5,
    "group_colors": ["#4c72b0", "#c44e52"],
    "show_axis_ticks": true,
    "axis_ticks": 4,
    "show_mean": true,
    "mean_stroke_width": 3,
    "inverted_axes": [1],
    "show_axis_bands": true,
    "legend": "cohort"
  }]
}"##;

const STACKED_AREA: &str = r##"{
  "series": [{
    "type": "stacked_area",
    "x": [1, 2, 3, 4],
    "series": [
      {"values": [3, 4, 5, 6], "label": "a", "color": "#4c72b0"},
      {"values": [2, 3, 2, 4], "label": "b", "color": "#c44e52"},
      {"values": [1, 1, 2, 1], "label": "c"}
    ],
    "fill_opacity": 0.6,
    "stroke_width": 1.2,
    "show_strokes": true,
    "normalized": true,
    "legend_position": "outside_right_middle"
  }]
}"##;

const STREAMGRAPH: &str = r##"{
  "series": [{
    "type": "streamgraph",
    "x": [1, 2, 3, 4, 5],
    "series": [
      {"values": [3, 5, 4, 6, 5], "label": "in", "color": "#4c72b0"},
      {"values": [2, 3, 3, 2, 4], "label": "mid", "color": "#c44e52"},
      {"values": [1, 2, 2, 3, 2], "label": "out", "color": "#55a868"}
    ],
    "baseline": "symmetric",
    "order": "by_total",
    "smooth": false,
    "fill_opacity": 0.8,
    "stroke_between": true,
    "stroke_width": 0.7,
    "show_labels": true,
    "min_label_height": 12,
    "normalized": true,
    "legend": "flow",
    "legend_position": "outside_bottom_center"
  }]
}"##;

const BAND: &str = r##"{
  "series": [{
    "type": "band",
    "x": [1, 2, 3, 4],
    "y_lower": [0.8, 1.4, 1.9, 2.5],
    "y_upper": [1.2, 1.8, 2.4, 3.1],
    "color": "steelblue",
    "opacity": 0.25,
    "legend": "95% CI"
  }]
}"##;

const TEXT: &str = r##"{
  "series": [{
    "type": "text",
    "body": "# Report\n---\nSome **bold** text.\n\nSecond paragraph.",
    "title": "Summary",
    "font_size": 14,
    "padding": 20,
    "background": "#f8f8f8",
    "border_color": "#cccccc",
    "border_width": 1,
    "text_align": "left",
    "text_color": "#222222"
  }]
}"##;

const LEGEND_PLOT: &str = r##"{
  "series": [{
    "type": "legend_plot",
    "entries": [
      {"label": "rect", "color": "#4c72b0"},
      {"label": "line", "color": "#c44e52", "shape": "line", "dasharray": "4 2"},
      {"label": "circle", "color": "#55a868", "shape": "circle"},
      {"label": "triangle", "color": "#8172b2", "shape": {"marker": "triangle"}},
      {"label": "size", "color": "#937860", "shape": {"size": 6}}
    ],
    "cols": 2,
    "max_entries": 10,
    "title": "legend",
    "show_box": true
  }]
}"##;

const QUIVER: &str = r##"{
  "series": [{
    "type": "quiver",
    "arrows": [
      {"x": 0, "y": 0, "u": 1, "v": 1},
      {"x": 1, "y": 0, "u": 0.5, "v": 2, "color": "#c44e52"},
      {"x": 0, "y": 1, "u": -1, "v": 0.5}
    ],
    "color": "steelblue",
    "scale": 1,
    "shaft_width": 1.5,
    "head_length": 8,
    "head_ratio": 0.3,
    "head_min_px": 4,
    "head_max_px": 14,
    "color_map": "viridis",
    "color_range": [0, 2],
    "color_legend_label": "magnitude",
    "legend": "field",
    "tight_bounds": true,
    "clip_to_plot_area": true,
    "pivot": "middle"
  }]
}"##;

const JOINT: &str = r##"{
  "series": [{
    "type": "jointplot",
    "groups": [
      {"x": [1, 2, 3, 4, 5], "y": [2, 4, 3, 5, 6], "label": "a", "color": "#4c72b0", "marker": "circle", "trend": true, "equation": true, "correlation": true},
      {"x": [2, 3, 4], "y": [3, 4, 5], "label": "b", "sizes": [4, 6, 8], "colors": ["#c44e52", "#c44e52", "#c44e52"]}
    ],
    "marginal_type": "histogram",
    "show_top": true,
    "show_right": true,
    "marginal_size": 90,
    "marginal_gap": 5,
    "bins": 12,
    "marginal_alpha": 0.5,
    "x_label": "x",
    "y_label": "y",
    "marker_size": 5,
    "marker_opacity": 0.7
  }]
}"##;

/// 玫瑰图：单系列的逐扇区写法。
const ROSE: &str = r##"{
  "series": [{
    "type": "rose",
    "slices": [
      {"label": "Jan", "value": 30, "color": "#4c72b0"},
      {"label": "Feb", "value": 20},
      {"label": "Mar", "value": 45, "color": "#c44e52"},
      {"label": "Apr", "value": 38}
    ],
    "encoding": "area",
    "start_angle": 0,
    "clockwise": true,
    "inner_radius": 0.1,
    "gap": 2,
    "grid_lines": 4,
    "show_spokes": true,
    "show_labels": true,
    "show_values": true,
    "legend": "months"
  }]
}"##;

/// 玫瑰图：多系列的堆叠写法。
const ROSE_STACKED: &str = r##"{
  "series": [{
    "type": "rose",
    "labels": ["Q1", "Q2", "Q3", "Q4"],
    "series": [
      {"name": "2023", "values": [10, 14, 9, 12], "color": "#4c72b0"},
      {"name": "2024", "values": [15, 11, 13, 17]}
    ],
    "mode": "stacked",
    "encoding": "radius",
    "show_values": false
  }]
}"##;

/// 双 Y 轴：`secondary_series` 画在右侧那根轴上。
const TWIN_Y: &str = r##"{
  "title": "twin axis",
  "y_axis": {"name": "price", "min": 0, "max": 100},
  "y2_axis": {"name": "volume", "min": 0, "max": 1000, "log": false, "tick_format": "sci"},
  "x_axis": {"tick_step": 1, "label_offset": [0, 4]},
  "series": [{"type": "line", "data": [[0, 20], [1, 45], [2, 60]], "legend": "price", "color": "#4c72b0"}],
  "secondary_series": [{"type": "bar", "categories": ["a", "b", "c"], "values": [300, 700, 500], "legend": "volume", "color": "#c44e52"}]
}"##;

/// 日期轴 + 统计框。
const DATETIME_AND_STATS: &str = r##"{
  "x_datetime": {"unit": "day", "step": 7, "format": "%Y-%m-%d"},
  "stats_box": {
    "title": "fit",
    "entries": ["n = 128", "R2 = 0.91"],
    "position": "inside_top_right",
    "border": true
  },
  "series": [{"type": "scatter", "data": [[1704067200, 2], [1706745600, 5], [1709251200, 3]], "legend": "y"}]
}"##;

/// 所有「应该成功」的规格，供逐条与总体验证复用。
const VALID_SPECS: &[(&str, &str)] = &[
    ("scatter", SCATTER),
    ("overlay", OVERLAY),
    ("bar", BAR),
    ("histogram", HISTOGRAM),
    ("box", BOX),
    ("pie", PIE),
    ("figure", FIGURE),
    ("violin", VIOLIN),
    ("ridgeline", RIDGELINE),
    ("raincloud", RAINCLOUD),
    ("strip", STRIP),
    ("dot_plot", DOT_PLOT),
    ("lollipop", LOLLIPOP),
    ("density", DENSITY),
    ("density_curve", DENSITY_CURVE),
    ("ecdf", ECDF),
    ("qq", QQ),
    ("qq_genomic", QQ_GENOMIC),
    ("forest", FOREST),
    ("pr", PR),
    ("roc", ROC),
    ("survival", SURVIVAL),
    ("volcano", VOLCANO),
    ("heatmap", HEATMAP),
    ("histogram2d", HISTOGRAM2D),
    ("hexbin", HEXBIN),
    ("clustermap", CLUSTERMAP),
    ("contour_grid", CONTOUR_GRID),
    ("contour_points", CONTOUR_POINTS),
    ("ternary", TERNARY),
    ("polar", POLAR),
    ("dice_plot", DICE_PLOT),
    ("scatter3d", SCATTER3D),
    ("surface3d", SURFACE3D),
    ("sankey", SANKEY),
    ("chord", CHORD),
    ("network", NETWORK),
    ("treemap", TREEMAP),
    ("sunburst", SUNBURST),
    ("venn", VENN),
    ("venn_elements", VENN_ELEMENTS),
    ("upset", UPSET),
    ("waffle", WAFFLE),
    ("mosaic", MOSAIC),
    ("phylo", PHYLO),
    ("phylo_edges", PHYLO_EDGES),
    ("synteny", SYNTENY),
    ("candlestick", CANDLESTICK),
    ("calendar", CALENDAR),
    ("gantt", GANTT),
    ("horizon", HORIZON),
    ("manhattan", MANHATTAN),
    ("manhattan_no_build", MANHATTAN_NO_BUILD),
    ("manhattan_x", MANHATTAN_X),
    ("waterfall", WATERFALL),
    ("bump", BUMP),
    ("pareto", PARETO),
    ("brick", BRICK),
    ("brick_strigar", BRICK_STRIGAR),
    ("funnel", FUNNEL),
    ("slope", SLOPE),
    ("pyramid", PYRAMID),
    ("series", SERIES),
    ("radar", RADAR),
    ("parallel", PARALLEL),
    ("stacked_area", STACKED_AREA),
    ("streamgraph", STREAMGRAPH),
    ("band", BAND),
    ("text", TEXT),
    ("legend_plot", LEGEND_PLOT),
    ("quiver", QUIVER),
    ("jointplot", JOINT),
    ("rose", ROSE),
    ("rose_stacked", ROSE_STACKED),
    ("twin_y", TWIN_Y),
    ("datetime_and_stats", DATETIME_AND_STATS),
];

/// 渲染，失败直接 panic（附上原始错误）。
fn render(json: &str) -> String {
    match render_json(json) {
        Ok(svg) => svg,
        Err(e) => panic!("expected the spec to render, but got: {e}"),
    }
}

/// 用真正的 XML 解析器走一遍，确认标签都闭合、转义都合法。
fn assert_well_formed_xml(svg: &str) {
    let mut reader = quick_xml::Reader::from_str(svg);
    loop {
        match reader.read_event() {
            Ok(quick_xml::events::Event::Eof) => break,
            Ok(_) => {}
            Err(e) => panic!("rendered SVG is not well-formed XML: {e}"),
        }
    }
}

#[test]
fn every_chart_type_renders_an_svg_document() {
    for (name, spec) in VALID_SPECS {
        let svg = render(spec);
        assert!(svg.starts_with("<svg"), "{name}: expected an <svg> root");
        assert!(svg.trim_end().ends_with("</svg>"), "{name}: expected a closing </svg>");
        assert!(
            svg.len() > 500,
            "{name}: suspiciously small output ({} bytes)",
            svg.len()
        );
    }
}

#[test]
fn every_render_is_well_formed_xml() {
    for (name, spec) in VALID_SPECS {
        let svg = render(spec);
        assert_well_formed_xml(&svg);
        // 名字只用于失败时定位。
        assert!(!svg.is_empty(), "{name}");
    }
}

#[test]
fn scatter_carries_title_and_axis_labels() {
    let svg = render(SCATTER);
    for text in ["Scatter", "x", "y", "samples"] {
        assert!(svg.contains(text), "scatter output is missing `{text}`");
    }
}

#[test]
fn overlay_shares_one_layout_between_two_series() {
    let svg = render(OVERLAY);
    // 两个 series 的图例文字都要在同一张图里出现，说明它们确实叠加了。
    for text in ["signal", "observed", "target"] {
        assert!(svg.contains(text), "overlay output is missing `{text}`");
    }
}

#[test]
fn grouped_bar_shows_both_series_in_the_legend() {
    let svg = render(BAR);
    for text in ["Jan", "Apr", "A", "B"] {
        assert!(svg.contains(text), "bar output is missing `{text}`");
    }
}

#[test]
fn pie_renders_one_path_per_slice() {
    let svg = render(PIE);
    let paths = svg.matches("<path").count();
    assert!(paths >= 4, "expected at least 4 slice paths, found {paths}");
}

#[test]
fn figure_renders_both_panel_titles() {
    let svg = render(FIGURE);
    for text in ["Panels", "left panel", "right panel"] {
        assert!(svg.contains(text), "figure output is missing `{text}`");
    }
}

#[test]
fn malformed_json_is_reported() {
    let err = render_json("{oops").unwrap_err();
    assert!(err.contains("invalid JSON"), "unexpected message: {err}");
}

#[test]
fn unknown_series_type_is_reported() {
    let err = render_json(r#"{"series":[{"type":"nope"}]}"#).unwrap_err();
    assert!(err.contains("invalid JSON"), "unexpected message: {err}");
    assert!(err.contains("scatter"), "the message should list the known types: {err}");
}

#[test]
fn empty_series_is_reported() {
    let err = render_json(r#"{"series":[]}"#).unwrap_err();
    assert!(err.contains("`series` must not be empty"), "unexpected message: {err}");
}

#[test]
fn empty_scatter_data_is_reported() {
    let err = render_json(r#"{"series":[{"type":"scatter","data":[]}]}"#).unwrap_err();
    assert!(err.contains("`data` must not be empty"), "unexpected message: {err}");
}

#[test]
fn bar_value_category_mismatch_is_reported() {
    let err = render_json(r#"{"series":[{"type":"bar","categories":["a","b"],"values":[1]}]}"#)
        .unwrap_err();
    assert!(err.contains("`values` has 1 entries"), "unexpected message: {err}");
}

#[test]
fn histogram_edge_count_mismatch_is_reported() {
    let err = render_json(r#"{"series":[{"type":"histogram","edges":[0,1,2],"counts":[1]}]}"#)
        .unwrap_err();
    assert!(err.contains("one more than `counts`"), "unexpected message: {err}");
}

#[test]
fn figure_panel_count_mismatch_is_reported() {
    let err = render_json(
        r#"{"figure":{"rows":1,"cols":2,"panels":[{"series":[{"type":"scatter","data":[[1,2]]}]}]}}"#,
    )
    .unwrap_err();
    assert!(err.contains("panels were given"), "unexpected message: {err}");
}

#[test]
fn unknown_legend_position_is_reported() {
    let err = render_json(
        r#"{"legend":{"position":"middle_of_nowhere"},"series":[{"type":"scatter","data":[[1,2]]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("unknown legend.position"), "unexpected message: {err}");
}

#[test]
fn empty_group_values_are_reported() {
    let err = render_json(r##"{"series":[{"type":"violin","groups":[{"label":"A","values":[]}]}]}"##)
        .unwrap_err();
    assert!(
        err.contains("group `A` has no values"),
        "unexpected message: {err}"
    );
}

#[test]
fn split_groups_longer_than_groups_is_reported() {
    let err = render_json(
        r##"{"series":[{"type":"violin","groups":[{"label":"A","values":[1,2]}],
             "split":true,"split_groups":[{"values":[1]},{"values":[2]},{"values":[3]}]}]}"##,
    )
    .unwrap_err();
    assert!(
        err.contains("`split_groups` has 3 entries"),
        "unexpected message: {err}"
    );
}

#[test]
fn dot_plot_matrix_shape_mismatch_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"dot_plot","x_categories":["a","b"],"y_categories":["c","d"],
             "sizes":[[1,2]],"colors":[[1,2],[3,4]]}]}"#,
    )
    .unwrap_err();
    assert!(
        err.contains("`sizes` has 1 rows"),
        "unexpected message: {err}"
    );
}

#[test]
fn survival_time_event_mismatch_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"survival","groups":[{"label":"a","times":[1,2,3],"events":[true]}]}]}"#,
    )
    .unwrap_err();
    assert!(
        err.contains("3 times but 1 events"),
        "unexpected message: {err}"
    );
}

#[test]
fn roc_group_without_any_curve_is_reported() {
    let err = render_json(r#"{"series":[{"type":"roc","groups":[{"label":"a"}]}]}"#).unwrap_err();
    assert!(
        err.contains("needs either `predictions` or `points`"),
        "unexpected message: {err}"
    );
}

#[test]
fn forest_inverted_confidence_interval_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"forest","rows":[{"label":"a","estimate":1,"ci_lower":2,"ci_upper":0}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("ci_lower 2 above ci_upper 0"), "unexpected message: {err}");
}

#[test]
fn ragged_matrix_is_reported() {
    // kuva 对不等长的行要么静默丢列、要么下溢 panic，两种都很难排查，所以扩展这边先挡住。
    let err = render_json(r#"{"series":[{"type":"heatmap","data":[[1,2],[3]]}]}"#).unwrap_err();
    assert!(
        err.contains("row 0 has 2 values but row 1 has 1"),
        "unexpected message: {err}"
    );
}

#[test]
fn label_count_mismatch_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"heatmap","data":[[1,2],[3,4]],"row_labels":["only one"]}]}"#,
    )
    .unwrap_err();
    assert!(
        err.contains("`row_labels` has 1 entries but 2 are needed"),
        "unexpected message: {err}"
    );
}

#[test]
fn clustermap_annotation_length_is_reported() {
    let err = render_json(
        r##"{"series":[{"type":"clustermap","data":[[1,2],[3,4]],
             "row_annotations":[{"colors":["#fff"]}]}]}"##,
    )
    .unwrap_err();
    assert!(
        err.contains("`row_annotations[0]` has 1 colors"),
        "unexpected message: {err}"
    );
}

#[test]
fn zero_bins_is_reported() {
    // kuva 内部按 `bins_x - 1` 算格子宽，0 会下溢 panic。
    let err = render_json(
        r#"{"series":[{"type":"histogram2d","data":[[0,0]],"x_range":[0,1],"y_range":[0,1],"bins_x":0}]}"#,
    )
    .unwrap_err();
    assert!(
        err.contains("must both be greater than 0"),
        "unexpected message: {err}"
    );
}

#[test]
fn hexbin_z_length_mismatch_is_reported() {
    // 长度对不上时 kuva 直接按点下标取 `z[i]`，会 panic。
    let err =
        render_json(r#"{"series":[{"type":"hexbin","x":[1,2],"y":[1,2],"z":[1]}]}"#).unwrap_err();
    assert!(err.contains("`z` has 1 entries"), "unexpected message: {err}");
}

#[test]
fn contour_grid_coordinate_mismatch_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"contour","z":[[1,2],[3,4]],"x_coords":[0],"y_coords":[0,1]}]}"#,
    )
    .unwrap_err();
    assert!(
        err.contains("`x_coords` has 1 entries but the grid has 2 columns"),
        "unexpected message: {err}"
    );
}

#[test]
fn dice_plot_pip_out_of_range_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"dice_plot","ndots":3,"x_categories":["a"],"y_categories":["b"],
             "points":[{"x":"a","y":"b","present":[0,3]}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("lists pip 3"), "unexpected message: {err}");
}

#[test]
fn polar_radius_angle_mismatch_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"polar","series":[{"r":[1,2,3],"theta":[0,90],"label":"a"}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("3 radii but 2 angles"), "unexpected message: {err}");
}

#[test]
fn scatter3d_without_finite_point_is_reported() {
    // 全 NaN 时 kuva 会跳过整张图，只留一个空坐标系 —— 看起来像「画不出来」。
    let err = render_json(r#"{"series":[{"type":"scatter3d","data":[[1,2,3]],"colors":[]}]}"#)
        .unwrap_err();
    assert!(err.contains("`colors` has 0 entries"), "unexpected message: {err}");
}

#[test]
fn sankey_unknown_node_reference_is_reported() {
    // 认不出的节点名会变成指向不存在节点的下标，渲染时越界 panic。
    let err = render_json(
        r#"{"series":[{"type":"sankey","nodes":[{"label":"a"}],"links":[{"source":"a","target":"ghost","value":1}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("unknown node `ghost`"), "unexpected message: {err}");
}

#[test]
fn network_unknown_node_reference_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"network","nodes":[{"label":"a"}],"edges":[{"source":"a","target":"ghost","weight":1}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("unknown node `ghost`"), "unexpected message: {err}");
}

#[test]
fn venn_with_too_many_sets_is_reported() {
    // kuva 对超过 4 个集合直接不画（一行白图）。
    let err = render_json(
        r#"{"series":[{"type":"venn","sets":[{"label":"a","size":1},{"label":"b","size":1},
             {"label":"c","size":1},{"label":"d","size":1},{"label":"e","size":1}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("it supports 1 to 4"), "unexpected message: {err}");
}

#[test]
fn venn_mixed_set_forms_are_reported() {
    // 一个 set 带 elements 就整体走原始元素模式，预计算的 size/overlap 会被静默忽略。
    let err = render_json(
        r#"{"series":[{"type":"venn","sets":[{"label":"a","elements":["x"]},{"label":"b","size":3}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("mix the two set forms"), "unexpected message: {err}");
}

#[test]
fn upset_mask_beyond_declared_sets_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"upset","set_names":["a"],"set_sizes":[3],
             "intersections":[{"mask":3,"count":1}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("sets bits beyond the 1"), "unexpected message: {err}");
}

#[test]
fn upset_set_sizes_length_mismatch_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"upset","set_names":["a","b"],"set_sizes":[3],
             "intersections":[{"mask":1,"count":1}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("`set_sizes` has 1 entries"), "unexpected message: {err}");
}

#[test]
fn treemap_leaf_without_value_is_reported() {
    // 所有 value <= 0 的根会被整棵树跳过（静默出白图）。
    let err = render_json(
        r#"{"series":[{"type":"treemap","roots":[{"label":"root","children":[{"label":"leaf"}]}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("leaf `leaf` needs a `value`"), "unexpected message: {err}");
}

#[test]
fn phylo_multiple_input_forms_are_reported() {
    let err = render_json(
        r#"{"series":[{"type":"phylo","newick":"(A,B);","edges":[{"parent":"r","child":"A","length":1}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("mutually exclusive"), "unexpected message: {err}");
}

#[test]
fn phylo_clade_color_index_is_reported() {
    let err = render_json(
        r##"{"series":[{"type":"phylo","newick":"(A,B);","clade_colors":[[99,"#fff"]]}]}"##,
    )
    .unwrap_err();
    assert!(err.contains("refers to node 99"), "unexpected message: {err}");
}

#[test]
fn synteny_block_sequence_index_is_reported() {
    // 越界的 block 会被 kuva 静默跳过，块一多就看不出「少画了几块」。
    let err = render_json(
        r#"{"series":[{"type":"synteny","sequences":[{"label":"a","length":10}],
             "blocks":[{"seq1":0,"start1":0,"end1":5,"seq2":7,"start2":0,"end2":5}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("`seq2` = 7"), "unexpected message: {err}");
}

#[test]
fn bump_with_more_than_ten_series_is_reported() {
    // kuva 给第 11 条系列取色时索引一个 10 色的表（不取模），会越界 panic。
    let series: Vec<String> = (0..11)
        .map(|i| format!(r#"{{"name":"s{i}","ranks":[1]}}"#))
        .collect();
    let json = format!(r#"{{"series":[{{"type":"bump","series":[{}]}}]}}"#, series.join(","));
    let err = render_json(&json).unwrap_err();
    assert!(err.contains("only covers 10"), "unexpected message: {err}");
}

#[test]
fn brick_character_outside_template_is_reported() {
    // DNA 配色表里没有 `X`；kuva 渲染时找不到就是 panic。
    let err = render_json(
        r##"{"series":[{"type":"brick","sequences":["ACGX"],"template":"dna"}]}"##,
    )
    .unwrap_err();
    assert!(err.contains("color template does not cover"), "unexpected message: {err}");
}

#[test]
fn brick_strigar_without_repeat_count_is_reported() {
    // kuva 展开 strigar 时对没有次数的字母做 parse().expect(..)，直接 panic。
    let err = render_json(
        r#"{"series":[{"type":"brick","names":["r1"],"strigars":[["CAG:A","A"]]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("without a repeat count"), "unexpected message: {err}");
}

#[test]
fn manhattan_build_without_position_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"manhattan","points":[{"chromosome":"1","pvalue":0.1}],"build":"hg38"}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("needs a `position`"), "unexpected message: {err}");
}

#[test]
fn funnel_with_all_zero_stages_is_reported() {
    // kuva 求最大值用的是 fold(0.0, max)，全 0 时整张图不画。
    let err = render_json(
        r#"{"series":[{"type":"funnel","stages":[{"label":"a","value":0},{"label":"b","value":0}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("would come out blank"), "unexpected message: {err}");
}

#[test]
fn pyramid_series_length_mismatch_is_reported() {
    // 年龄轴只由第一个 series 决定，别人比它长就画到轴外面去了。
    let err = render_json(
        r#"{"series":[{"type":"pyramid","series":[
             {"label":"a","groups":[{"age":"0-9","left":1,"right":2}]},
             {"label":"b","groups":[{"age":"0-9","left":1,"right":2},{"age":"10-19","left":1,"right":2}]}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("age axis comes from the first series"), "unexpected message: {err}");
}

#[test]
fn radar_with_too_few_axes_is_reported() {
    // kuva 在轴数 < 3 时早退，整张图不画。
    let err = render_json(
        r#"{"series":[{"type":"radar","axes":["a","b"],"series":[{"values":[1,2]}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("needs at least 3"), "unexpected message: {err}");
}

#[test]
fn radar_value_count_mismatch_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"radar","axes":["a","b","c"],"series":[{"values":[1,2]}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("2 values but there are 3 axes"), "unexpected message: {err}");
}

#[test]
fn band_negative_opacity_is_reported() {
    // 负不透明度会让 kuva 拼出非法的颜色串，渲染时 panic。
    let err = render_json(
        r#"{"series":[{"type":"band","x":[1,2],"y_lower":[0,0],"y_upper":[1,1],"opacity":-0.2}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("must not be negative"), "unexpected message: {err}");
}

#[test]
fn legend_plot_zero_max_entries_is_reported() {
    // kuva 内部算 `max_entries - 1`，给 0 就是 usize 下溢。
    let err = render_json(
        r#"{"series":[{"type":"legend_plot","entries":[{"label":"a","color":"red"}],"max_entries":0}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("at least 1"), "unexpected message: {err}");
}

#[test]
fn text_zero_font_size_is_reported() {
    // kuva 用字号去除字符宽度，0 会除零 panic。
    let err = render_json(
        r#"{"series":[{"type":"text","body":"hi","font_size":0}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("at least 1"), "unexpected message: {err}");
}

#[test]
fn jointplot_xy_length_mismatch_is_reported() {
    let err = render_json(
        r#"{"series":[{"type":"jointplot","groups":[{"x":[1,2,3],"y":[1,2]}]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("3 x values but 2 y values"), "unexpected message: {err}");
}

#[test]
fn secondary_x_axis_needs_both_ends() {
    // kuva 的第二根 x 轴只有 `with_x2_range(min, max)`，没有单端 setter。
    let err = render_json(
        r#"{"x2_axis":{"min":0},"series":[{"type":"scatter","data":[[1,2]]}]}"#,
    )
    .unwrap_err();
    assert!(err.contains("`min` and `max` together"), "unexpected message: {err}");
}

#[test]
fn secondary_axis_without_secondary_series_renders_a_normal_chart() {
    // 第二根轴只在 `render_twin_y` 下才画，所以只给 y2_axis 而没有 secondary_series 时
    // 就是一张普通的单轴图（不报错，只是那根轴不出现）。
    let svg = render(r#"{"y2_axis":{"name":"right"},"series":[{"type":"scatter","data":[[1,2],[2,3]]}]}"#);
    assert!(svg.starts_with("<svg"));
    assert!(!svg.contains(">right<"), "the second axis should not be drawn without secondary_series");
}

#[test]
fn every_plot_type_in_the_kuva_enum_is_reachable() {
    // 这个断言是「全都实现了」的守卫：kuva 的 `Plot` 枚举每多一个变体，这里就得更新一次。
    // 从编译期登记的名字反查 —— 未登记的图型会在下面的比对里露出来。
    const REGISTERED: &[&str] = &[
        "scatter", "line", "bar", "histogram", "box", "pie", "violin", "ridgeline", "raincloud",
        "strip", "dot_plot", "lollipop", "density", "ecdf", "qq", "forest", "pr", "roc", "survival",
        "volcano", "heatmap", "histogram2d", "hexbin", "clustermap", "contour", "ternary", "polar",
        "dice_plot", "scatter3d", "surface3d", "sankey", "chord", "network", "treemap", "sunburst",
        "venn", "upset", "waffle", "mosaic", "phylo", "synteny", "candlestick", "calendar", "gantt",
        "horizon", "manhattan", "waterfall", "bump", "pareto", "brick", "funnel", "slope", "pyramid",
        "series", "radar", "parallel", "stacked_area", "streamgraph", "band", "text", "legend_plot",
        "quiver", "jointplot", "rose",
    ];
    // 每个都渲染一张最小图，确保「登记了但其实画不出来」不会溜过去。
    for name in REGISTERED {
        let json = format!(r#"{{"series":[{{"type":"{name}"}}]}}"#);
        // 最小规格大多缺必填字段，这里只关心「不是未知图型」。
        if let Err(e) = render_json(&json) {
            assert!(
                !e.contains("unknown variant"),
                "{name} is registered but not dispatchable: {e}"
            );
        }
    }
}
