//! 联合分布 -> `Plot::Joint`。

use kuva::plot::jointplot::JointGroup;
use kuva::plot::scatter::TrendLine;
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::marker_shape;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_joint(s: JointSpec) -> Result<Plot, String> {
    if s.groups.is_empty() {
        return Err("jointplot: `groups` must not be empty".into());
    }
    // JointGroup 内部用 zip 配对 x/y，长度不一致时短的那个说了算 —— 直接报出来。
    for (i, g) in s.groups.iter().enumerate() {
        if g.x.len() != g.y.len() {
            return Err(format!(
                "jointplot: group {i} has {} x values but {} y values",
                g.x.len(),
                g.y.len()
            ));
        }
        if g.x.is_empty() {
            return Err(format!("jointplot: group {i} has no points"));
        }
        super::check_optional_len("jointplot", "sizes", g.sizes.as_ref().map(|v| v.len()), g.x.len())?;
        super::check_optional_len("jointplot", "colors", g.colors.as_ref().map(|v| v.len()), g.x.len())?;
    }
    if s.bins == Some(0) {
        return Err("jointplot: `bins` must be at least 1 (0 divides by zero when normalizing)".into());
    }

    let mut plot = JointPlot::new();
    for g in &s.groups {
        let mut group = JointGroup::new(g.x.clone(), g.y.clone());
        if let Some(v) = &g.label {
            group = group.with_label(v.clone());
        }
        if let Some(v) = &g.color {
            group = group.with_color(v.clone());
        }
        if let Some(v) = &g.marker {
            group = group.with_marker(marker_shape(v));
        }
        if let Some(v) = &g.sizes {
            group = group.with_sizes(v.clone());
        }
        if let Some(v) = &g.colors {
            group = group.with_colors(v.clone());
        }
        if g.trend == Some(true) {
            group = group.with_trend(TrendLine::Linear);
        }
        if g.equation == Some(true) {
            group = group.with_equation();
        }
        if g.correlation == Some(true) {
            group = group.with_correlation();
        }
        plot = plot.with_joint_group(group);
    }
    if let Some(v) = &s.marginal_type {
        plot = plot.with_marginal_type(match v {
            MarginalTypeKind::Histogram => MarginalType::Histogram,
            MarginalTypeKind::Density => MarginalType::Density,
        });
    }
    if let Some(v) = s.show_top {
        plot = plot.with_top_marginal(v);
    }
    if let Some(v) = s.show_right {
        plot = plot.with_right_marginal(v);
    }
    if let Some(v) = s.marginal_size {
        plot = plot.with_marginal_size(v);
    }
    if let Some(v) = s.marginal_gap {
        plot = plot.with_marginal_gap(v);
    }
    if let Some(v) = s.bins {
        plot = plot.with_bins(v);
    }
    if let Some(v) = s.bandwidth {
        plot = plot.with_bandwidth(v);
    }
    if let Some(v) = s.marginal_alpha {
        plot = plot.with_marginal_alpha(v);
    }
    if let Some(v) = &s.x_label {
        plot = plot.with_x_label(v.clone());
    }
    if let Some(v) = &s.y_label {
        plot = plot.with_y_label(v.clone());
    }
    if let Some(v) = s.marker_size {
        plot = plot.with_marker_size(v);
    }
    if let Some(v) = s.marker_opacity {
        plot = plot.with_marker_opacity(v);
    }
    if s.tooltips == Some(true) {
        plot.groups.iter_mut().for_each(|g| {
            g.scatter.show_tooltips = true;
        });
    }
    if let Some(v) = s.tooltip_labels {
        plot.groups.iter_mut().for_each(|g| {
            g.scatter.tooltip_labels = Some(v.clone());
        });
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

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

    #[test]
    fn renders_joint() {
        assert_renders(&render_svg(JOINT), "JOINT");
    }

    #[test]
    fn jointplot_xy_length_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"jointplot","groups":[{"x":[1,2,3],"y":[1,2]}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("3 x values but 2 y values"), "unexpected message: {err}");
    }
}
