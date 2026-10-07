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

/// 所有「应该成功」的规格，供逐条与总体验证复用。
const VALID_SPECS: &[(&str, &str)] = &[
    ("scatter", SCATTER),
    ("overlay", OVERLAY),
    ("bar", BAR),
    ("histogram", HISTOGRAM),
    ("box", BOX),
    ("pie", PIE),
    ("figure", FIGURE),
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