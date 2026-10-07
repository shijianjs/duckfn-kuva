//! 把 [`schema`](super::schema) 里反序列化出来的结构体翻译成 kuva 的 `Vec<Plot>` + `Layout`
//! （或多面板的 `Figure`）。
//!
//! 分工：`schema` 只做「JSON -> 结构体」，这一层才做校验（数组非空、长度一致、枚举字符串合法）
//! 与默认值填充，因为只有这里知道 kuva 的约束。所有错误都以 `Err(String)` 冒泡（**信息一律英文**，
//! 它会原样出现在 DuckDB 的错误里），最终由 `kuva_render` 变成一条让查询失败的错误。
//!
//! 子模块：`charts`（各图型的 build）、`layout`（画布外观）、`enums`（字符串/枚举/值的翻译）。

mod charts;
mod enums;
mod layout;

use kuva::prelude::*;

use super::schema::*;
use layout::build_layout;

/// 顶层入口：有 `figure` 就走多面板，否则单图。
pub(crate) fn render(spec: RenderSpec) -> Result<String, String> {
    let RenderSpec { panel, figure } = spec;
    match figure {
        Some(fig) => render_figure(fig),
        None => render_single(panel),
    }
}

/// 单图：一组 series 叠加到同一套坐标轴上。
fn render_single(panel: PanelSpec) -> Result<String, String> {
    let mut panel = panel;
    let series = std::mem::take(&mut panel.series);
    if series.is_empty() {
        return Err("`series` must not be empty: a single-figure chart needs at least one series".into());
    }
    let has_explicit_color = series.iter().any(SeriesSpec::has_explicit_color);
    let plots = build_series(series)?;
    let layout = build_layout(&panel, &plots, has_explicit_color)?;
    Ok(SvgBackend.render_scene(&render_multiple(plots, layout)))
}

/// 多面板：每个 panel 各自构建 plots + layout，再交给 `Figure` 排版。
fn render_figure(fig: FigureSpec) -> Result<String, String> {
    if fig.rows == 0 || fig.cols == 0 {
        return Err("figure: `rows` and `cols` must both be greater than 0".into());
    }
    let expected = fig.rows * fig.cols;
    if fig.panels.len() != expected {
        return Err(format!(
            "figure: {} panels were given but rows * cols = {}",
            fig.panels.len(),
            expected
        ));
    }

    let mut all_plots: Vec<Vec<Plot>> = Vec::with_capacity(expected);
    let mut all_layouts: Vec<Layout> = Vec::with_capacity(expected);
    for mut panel in fig.panels {
        let series = std::mem::take(&mut panel.series);
        if series.is_empty() {
            return Err("figure: every panel needs at least one series".into());
        }
        let has_explicit_color = series.iter().any(SeriesSpec::has_explicit_color);
        let plots = build_series(series)?;
        let layout = build_layout(&panel, &plots, has_explicit_color)?;
        all_plots.push(plots);
        all_layouts.push(layout);
    }

    let mut figure = Figure::new(fig.rows, fig.cols)
        .with_plots(all_plots)
        .with_layouts(all_layouts);

    if let Some(title) = &fig.title {
        figure = figure.with_title(title.clone());
    }
    if let Some(size) = fig.title_size {
        figure = figure.with_title_size(size);
    }
    if let Some(labels) = &fig.labels {
        figure = match labels {
            LabelsSpec::Named(LabelsKind::None) => figure,
            LabelsSpec::Named(LabelsKind::Uppercase) => figure.with_labels(),
            LabelsSpec::Named(LabelsKind::Lowercase) => figure.with_labels_lowercase(),
            LabelsSpec::Named(LabelsKind::Numeric) => figure.with_labels_numeric(),
            LabelsSpec::Custom(names) => {
                let refs: Vec<&str> = names.iter().map(String::as_str).collect();
                figure.with_labels_custom(refs, LabelConfig::default())
            }
        };
    }
    if fig.shared_x_all == Some(true) {
        figure = figure.with_shared_x_all();
    }
    if fig.shared_y_all == Some(true) {
        figure = figure.with_shared_y_all();
    }
    if let Some(pos) = &fig.shared_legend {
        figure = figure.with_shared_legend_position(enums::figure_legend_position(pos)?);
    }
    if let Some(v) = fig.spacing {
        figure = figure.with_spacing(v);
    }
    if let Some(v) = fig.padding {
        figure = figure.with_padding(v);
    }
    if let (Some(w), Some(h)) = (fig.cell_width, fig.cell_height) {
        figure = figure.with_cell_size(w, h);
    }
    if let (Some(w), Some(h)) = (fig.figure_width, fig.figure_height) {
        figure = figure.with_figure_size(w, h);
    }

    Ok(SvgBackend.render_scene(&figure.render()))
}

/// 逐个把 series 描述翻成 kuva 的 `Plot`。
fn build_series(specs: Vec<SeriesSpec>) -> Result<Vec<Plot>, String> {
    specs.into_iter().map(SeriesSpec::build).collect()
}