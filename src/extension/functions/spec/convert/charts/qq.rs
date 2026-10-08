//! QQ 图 -> `Plot::QQ`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_qq(s: QqSeries) -> Result<Plot, String> {
    super::require_groups("qq", &s.groups)?;

    let genomic = matches!(s.mode, Some(QqModeKind::Genomic));
    let mut plot = QQPlot::new();
    for g in &s.groups {
        // genomic 模式喂的是 p 值，kuva 用另一条 builder（它会把 mode 一并设成 Genomic）。
        plot = match (&g.color, genomic) {
            (Some(c), false) => plot.with_data_colored(g.label.clone(), g.values.clone(), c.clone()),
            (None, false) => plot.with_data(g.label.clone(), g.values.clone()),
            (Some(c), true) => {
                plot.with_pvalues_colored(g.label.clone(), g.values.clone(), c.clone())
            }
            (None, true) => plot.with_pvalues(g.label.clone(), g.values.clone()),
        };
    }
    // 显式写了 `mode: "normal"` 时，即便没有 group 走 pvalues 分支也要把模式掰回来。
    if !genomic {
        plot = plot.with_normal();
    }

    match s.reference_line {
        Some(true) => plot = plot.with_reference_line(),
        Some(false) => plot = plot.without_reference_line(),
        None => {}
    }
    if s.ci_band == Some(true) {
        plot = plot.with_ci_band();
    }
    if let Some(v) = s.ci_alpha {
        plot = plot.with_ci_alpha(v);
    }
    match s.lambda {
        Some(true) => plot = plot.with_lambda(),
        Some(false) => plot = plot.without_lambda(),
        None => {}
    }
    if let Some(v) = s.marker_size {
        plot = plot.with_marker_size(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.fill_opacity {
        plot = plot.with_fill_opacity(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
