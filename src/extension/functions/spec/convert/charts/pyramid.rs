//! 人口金字塔 -> `Plot::Pyramid`。

use kuva::plot::pyramid::{PopulationPyramid, PyramidMode, PyramidSeries};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_pyramid(s: PyramidSpec) -> Result<Plot, String> {
    if s.series.is_empty() {
        return Err("pyramid: `series` must not be empty".into());
    }
    // 年龄组只由第一个 series 决定；别人比它长就会画到轴范围之外（不 panic，但图是错的）。
    let expected = s.series[0].groups.len();
    for spec in s.series.iter().skip(1) {
        if spec.groups.len() != expected {
            return Err(format!(
                "pyramid: series `{}` has {} age groups but the first series has {expected}; the age axis comes from the first series, so they must line up",
                spec.label,
                spec.groups.len()
            ));
        }
    }

    let mut plot = PopulationPyramid::new();
    for spec in &s.series {
        plot.series.push(PyramidSeries {
            label: spec.label.clone(),
            groups: spec
                .groups
                .iter()
                .map(|g| (g.age.clone(), g.left, g.right))
                .collect(),
            color: spec.color.clone(),
            opacity: spec.opacity.unwrap_or(0.6),
        });
    }
    if let Some(v) = &s.left_label {
        plot = plot.with_left_label(v.clone());
    }
    if let Some(v) = &s.right_label {
        plot = plot.with_right_label(v.clone());
    }
    if let Some(v) = &s.left_color {
        plot = plot.with_left_color(v.clone());
    }
    if let Some(v) = &s.right_color {
        plot = plot.with_right_color(v.clone());
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = s.show_values {
        plot = plot.with_show_values(v);
    }
    if let Some(v) = s.group_gap {
        plot = plot.with_group_gap(v);
    }
    if let Some(v) = s.bar_gap {
        plot = plot.with_bar_gap(v);
    }
    if let Some(v) = &s.mode {
        plot = plot.with_mode(match v {
            PyramidModeKind::Grouped => PyramidMode::Grouped,
            PyramidModeKind::Overlap => PyramidMode::Overlap,
        });
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_legend(v);
    }
    Ok(plot.into())
}
