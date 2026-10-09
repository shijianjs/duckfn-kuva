//! 棒棒糖图 -> `Plot::Lollipop`。

use kuva::plot::lollipop::LollipopPoint;
use kuva::prelude::*;

use super::apply_common;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_lollipop(s: LollipopSeries) -> Result<Plot, String> {
    if s.points.is_empty() {
        return Err("lollipop: `points` must not be empty".into());
    }

    let mut plot = LollipopPlot::new();
    for (i, p) in s.points.iter().enumerate() {
        // 分类 x：取该点在 `points` 里的次序当坐标，字符串本身成为点的标签（kuva 的 CLI
        // 对字符串列就是这么做的）。显式给了 `label` 就以 `label` 为准。
        let (x, category) = match &p.x {
            LollipopXSpec::Number(v) => (*v, None),
            LollipopXSpec::Category(name) => (i as f64, Some(name.clone())),
        };
        plot.points.push(LollipopPoint {
            x,
            y: p.y,
            label: p.label.clone().or(category),
            color: p.color.clone(),
        });
    }
    for d in &s.domains {
        plot = plot.with_domain_opacity(
            d.start,
            d.end,
            d.label.as_deref(),
            d.color.clone(),
            d.opacity.unwrap_or(0.35),
        );
    }

    if let Some(v) = s.baseline {
        plot = plot.with_baseline(v);
    }
    if let Some(v) = s.stem_width {
        plot = plot.with_stem_width(v);
    }
    if let Some(v) = s.dot_radius {
        plot = plot.with_dot_radius(v);
    }
    if let Some(v) = &s.dot_stroke {
        plot = plot.with_dot_stroke(v.clone());
    }
    if let Some(v) = s.dot_stroke_width {
        plot = plot.with_dot_stroke_width(v);
    }
    if let Some(v) = s.show_baseline {
        plot = plot.with_show_baseline(v);
    }
    if let Some(v) = &s.baseline_color {
        plot = plot.with_baseline_color(v.clone());
    }
    if let Some(v) = s.baseline_width {
        plot = plot.with_baseline_width(v);
    }
    if let Some(v) = &s.baseline_dash {
        plot = plot.with_baseline_dash(v.clone());
    }
    if let Some(v) = s.domain_height {
        plot = plot.with_domain_height(v);
    }
    apply_common(
        &mut plot.color,
        &mut plot.legend_label,
        &mut false,
        &mut None,
        s.common,
    );
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

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

    #[test]
    fn renders_lollipop() {
        assert_renders(&render_svg(LOLLIPOP), "LOLLIPOP");
    }

    /// 分类 x：字符串 x 取该点在数组里的次序，字符串本身成为点的标签
    /// （与 kuva 自己的 CLI 对字符串列的处理一致）。
    const LOLLIPOP_CATEGORICAL: &str = r##"{
      "series": [{
        "type": "lollipop",
        "points": [
          {"x": "alpha", "y": 12.0},
          {"x": "beta",  "y": 19.0},
          {"x": "gamma", "y": 7.0, "label": "third"}
        ],
        "baseline": 0
      }]
    }"##;

    #[test]
    fn renders_lollipop_categorical() {
        let svg = render_svg(LOLLIPOP_CATEGORICAL);
        assert_renders(&svg, "LOLLIPOP_CATEGORICAL");
        let texts: Vec<&str> = svg
            .split("<text")
            .skip(1)
            .take(12)
            .map(|s| &s[..s.len().min(60)])
            .collect();
        assert!(
            svg.contains("alpha"),
            "the category name should label its point; texts: {texts:?}"
        );
        assert!(
            svg.contains("beta"),
            "the category name should label its point; texts: {texts:?}"
        );
        // 显式给的 `label` 优先于分类名：第三个点的分类名 `gamma` 不再出现。
        assert!(
            svg.contains("third"),
            "an explicit label wins over the category name; texts: {texts:?}"
        );
        assert!(
            !svg.contains("gamma"),
            "the explicit label replaces the category name; texts: {texts:?}"
        );
    }
}
