//! 组合图 -> `Scene`：kuva 的 `CoveragePlot` / `BrickPopPlot`。
//!
//! 这两个不是 `Plot` 变体，自己拼 `Scene`，所以这里直接调它们的 `render(width)`，不经过
//! `Layout` / `Figure`。校验照旧在这一层：组合图拿到的数据长度不对时 kuva 只会 `debug_assert`
//! （release 里静默错位），SQL 调用方需要一条明确的错误。
//!
//! ----------------------------------------------------------------------------
//! Composites -> `Scene`: kuva's `CoveragePlot` / `BrickPopPlot`. Neither is a `Plot` variant — each
//! assembles its own `Scene` — so this calls their `render(width)` directly and skips
//! `Layout` / `Figure`. Validation still happens here: a length mismatch only trips a `debug_assert`
//! inside kuva (and silently misaligns in release), while a SQL caller needs an explicit error.

use kuva::prelude::*;
use kuva::render::brick_pop::{BrickPopPlot, MetricColumn};
use kuva::render::coverage::CoveragePlot;
use kuva::render::render::Scene;

use super::charts::build_brick_series;
use super::enums::{color_map, cycle_color, to_theme};
use crate::extension::functions::spec::schema::*;

/// 组合图的缺省画布宽度：比单图（675）宽一些，因为这类图横向要放下整段基因组坐标。
const DEFAULT_COMPOSITE_WIDTH: f64 = 900.0;

pub(super) fn render(c: CompositeSpec) -> Result<Scene, String> {
    match c {
        CompositeSpec::Coverage(s) => render_coverage(*s),
        CompositeSpec::BrickPop(s) => render_brick_pop(*s),
    }
}

fn render_coverage(s: CoverageSpec) -> Result<Scene, String> {
    if s.samples.is_empty() {
        return Err("coverage: `samples` must not be empty".into());
    }
    for (i, sample) in s.samples.iter().enumerate() {
        if sample.depth.is_empty() {
            return Err(format!(
                "coverage: sample `{}` (`samples[{i}]`) has no depth points",
                sample.name
            ));
        }
    }
    if let Some((start, end)) = s.locus
        && start >= end
    {
        return Err(format!(
            "coverage: `locus` must be [start, end] with start < end (got [{start}, {end}])"
        ));
    }
    check_intervals("coverage", "features", &s.features)?;
    check_intervals("coverage", "regions", &s.regions)?;

    let mut plot = CoveragePlot::new();
    if let Some(v) = &s.title {
        plot = plot.with_title(v.clone());
    }
    if let Some(v) = &s.x_label {
        plot = plot.with_x_label(v.clone());
    }
    if let Some((start, end)) = s.locus {
        plot = plot.with_locus(start, end);
    }
    if let Some(t) = &s.theme {
        plot = plot.with_theme(to_theme(t));
    }
    if let Some(v) = &s.tick_font_family {
        plot = plot.with_tick_font_family(v.clone());
    }
    if s.overlay_samples == Some(true) {
        plot = plot.with_overlaid_samples();
    }
    for sample in s.samples {
        plot = plot.with_sample(
            sample.name,
            sample.depth.iter().map(|p| (p[0], p[1])).collect(),
        );
    }
    for (i, v) in s.variants.iter().enumerate() {
        plot = plot.with_variants(
            v.label.clone(),
            v.color.clone().unwrap_or_else(|| cycle_color(i)),
            v.positions.clone(),
        );
    }
    for f in &s.features {
        plot = plot.with_feature(f.start, f.end, f.label.clone().unwrap_or_default());
    }
    for r in &s.regions {
        plot = plot.with_region(r.start, r.end, r.label.clone().unwrap_or_default());
    }
    if let Some(v) = &s.feature_track_name {
        plot = plot.with_feature_track_name(v.clone());
    }
    if let Some(v) = &s.region_track_name {
        plot = plot.with_region_track_name(v.clone());
    }
    for t in &s.coverage_thresholds {
        plot = match &t.label {
            Some(label) => plot.with_coverage_threshold_labeled(t.depth, label.clone()),
            None => plot.with_coverage_threshold(t.depth),
        };
    }

    let width = s.width.unwrap_or(DEFAULT_COMPOSITE_WIDTH).max(1.0);
    Ok(match s.height {
        Some(h) => plot.render_sized(width, h.max(1.0)),
        None => plot.render(width),
    })
}

fn check_intervals(kind: &str, field: &str, items: &[CoverageIntervalSpec]) -> Result<(), String> {
    for (i, item) in items.iter().enumerate() {
        if item.start >= item.end {
            return Err(format!(
                "{kind}: `{field}[{i}]` must have start < end (got [{}, {}])",
                item.start, item.end
            ));
        }
    }
    Ok(())
}

fn render_brick_pop(s: BrickPopSpec) -> Result<Scene, String> {
    let rows = brick_row_count(&s.brick)?;
    if let Some(freqs) = &s.frequencies
        && freqs.len() != rows
    {
        return Err(format!(
            "brick_pop: `frequencies` has {} entries but the brick has {rows} rows",
            freqs.len()
        ));
    }
    for (i, m) in s.metrics.iter().enumerate() {
        if m.values.len() != rows {
            return Err(format!(
                "brick_pop: `metrics[{i}]` (`{}`) has {} values but the brick has {rows} rows",
                m.label,
                m.values.len()
            ));
        }
    }

    let brick = match build_brick_series(s.brick)? {
        Plot::Brick(bp) => bp,
        // `build_brick_series` 只会返回 `Plot::Brick`；走到这里说明上游改了返回值。
        _ => return Err("brick_pop: the brick series did not build a brick plot".into()),
    };
    let mut plot = BrickPopPlot::new(brick);
    if let Some(v) = &s.title {
        plot = plot.with_title(v.clone());
    }
    if let Some(t) = &s.theme {
        plot = plot.with_theme(to_theme(t));
    }
    if let Some(v) = &s.frequencies {
        plot = plot.with_frequencies(v.clone());
    }
    if let Some((lo, hi)) = s.frequency_range {
        plot = plot.with_frequency_range(lo, hi);
    }
    if let Some(v) = &s.frequency_label {
        plot = plot.with_frequency_label(v.clone());
    }
    if let Some(v) = &s.freq_bar_color {
        plot = plot.with_freq_bar_color(v.clone());
    }
    if let Some(v) = &s.na_color {
        plot = plot.with_na_color(v.clone());
    }
    for m in s.metrics {
        let mut column = MetricColumn::new(
            m.label,
            m.values,
            m.color_map.as_ref().map_or(ColorMap::Viridis, color_map),
        );
        if let (Some(lo), Some(hi)) = (m.min, m.max) {
            column = column.with_range(lo, hi);
        } else if let Some(lo) = m.min {
            column.vmin = Some(lo);
        } else if let Some(hi) = m.max {
            column.vmax = Some(hi);
        }
        plot = plot.with_metric_column(column);
    }
    if let Some(v) = s.metric_values {
        plot = plot.with_metric_values(v);
    }
    if s.sorted_by_frequency == Some(true) {
        plot = plot.sorted_by_frequency();
    }
    if let Some(v) = s.row_height {
        plot = plot.with_row_height(v);
    }

    let width = s.width.unwrap_or(DEFAULT_COMPOSITE_WIDTH).max(1.0);
    Ok(match s.height {
        Some(h) => plot.render_sized(width, h.max(1.0)),
        None => plot.render(width),
    })
}

/// 砖墙的行数：`sequences` / `strigars` / `flanked_strigars` 里生效的那一份。
///
/// 真正的校验（三选一、名字数量…）在 `charts::brick` 里；这里只为「频率 / 指标列的长度」提前
/// 拿到行数，免得先建完再报错。
fn brick_row_count(s: &BrickSeries) -> Result<usize, String> {
    if !s.sequences.is_empty() {
        Ok(s.sequences.len())
    } else if !s.flanked_strigars.is_empty() {
        Ok(s.flanked_strigars.len())
    } else if !s.strigars.is_empty() {
        Ok(s.strigars.len())
    } else {
        Err(
            "brick_pop: the brick needs one of `sequences`, `strigars` or `flanked_strigars`"
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{
        assert_renders, render_json, render_svg,
    };

    const COVERAGE: &str = r##"{
      "composite": {
        "type": "coverage",
        "title": "Amplicon panel",
        "x_label": "position (bp)",
        "locus": [1000000, 1000600],
        "samples": [
          {"name": "tumour", "depth": [[1000000, 12], [1000200, 48], [1000400, 30], [1000600, 9]]},
          {"name": "normal", "depth": [[1000000, 8], [1000200, 31], [1000400, 22], [1000600, 11]]}
        ],
        "variants": [
          {"label": "SNV", "color": "#d1495b", "positions": [1000150, 1000430]},
          {"label": "InDel", "color": "#e9c46a", "positions": [1000310]}
        ],
        "features": [
          {"start": 1000050, "end": 1000250, "label": "amplicon_1"},
          {"start": 1000300, "end": 1000550, "label": "amplicon_2"}
        ],
        "regions": [{"start": 1000100, "end": 1000500, "label": "GENE1"}],
        "coverage_thresholds": [{"depth": 20, "label": "min depth"}]
      }
    }"##;

    #[test]
    fn renders_coverage_composite() {
        let svg = render_svg(COVERAGE);
        assert_renders(&svg, "COVERAGE");
        for text in [
            "Amplicon panel",
            "tumour",
            "normal",
            "amplicon_1",
            "GENE1",
            "min depth",
        ] {
            assert!(svg.contains(text), "coverage output is missing `{text}`");
        }
    }

    /// 叠加模式：所有样本挤在一条轨道里（只有一个 `depth` 轴标题）。
    #[test]
    fn renders_overlaid_coverage() {
        let svg = render_svg(
            r##"{"composite":{"type":"coverage","overlay_samples":true,
                 "samples":[{"name":"a","depth":[[0,1],[10,5],[20,2]]},
                            {"name":"b","depth":[[0,2],[10,4],[20,3]]}]}}"##,
        );
        assert_renders(&svg, "COVERAGE_OVERLAY");
    }

    #[test]
    fn coverage_without_samples_is_reported() {
        let err = render_json(r#"{"composite":{"type":"coverage","samples":[]}}"#).unwrap_err();
        assert!(
            err.contains("`samples` must not be empty"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn coverage_bad_locus_is_reported() {
        let err = render_json(
            r#"{"composite":{"type":"coverage","locus":[10,5],
                 "samples":[{"name":"a","depth":[[0,1]]}]}}"#,
        )
        .unwrap_err();
        assert!(err.contains("start < end"), "unexpected message: {err}");
    }

    /// 组合图与 `series` 互斥。
    #[test]
    fn composite_and_series_are_mutually_exclusive() {
        let err = render_json(
            r#"{"composite":{"type":"coverage","samples":[{"name":"a","depth":[[0,1]]}]},
                 "series":[{"type":"scatter","data":[{"x":1,"y":2}]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("mutually exclusive"),
            "unexpected message: {err}"
        );
    }

    const BRICK_POP: &str = r##"{
      "composite": {
        "type": "brick_pop",
        "title": "Allele frequencies",
        "names": ["hap_1", "hap_2", "hap_3"],
        "strigars": [
          ["CAG:A", "8A"],
          ["CAG:A,CAA:B", "6A2B"],
          ["CAG:A", "7A1B"]
        ],
        "frequencies": [0.52, 0.31, 0.17],
        "frequency_label": "Frequency",
        "freq_bar_color": "#4c72b0",
        "metrics": [
          {"label": "depth", "values": [120, 80, null], "color_map": "viridis"},
          {"label": "qual", "values": [30, 28, 35], "color_map": "magma", "min": 0, "max": 40}
        ],
        "metric_values": true,
        "sorted_by_frequency": true,
        "row_height": 26
      }
    }"##;

    #[test]
    fn renders_brick_pop_composite() {
        let svg = render_svg(BRICK_POP);
        assert_renders(&svg, "BRICK_POP");
        for text in ["Allele frequencies", "Frequency", "depth", "qual"] {
            assert!(svg.contains(text), "brick_pop output is missing `{text}`");
        }
    }

    #[test]
    fn brick_pop_frequency_count_mismatch_is_reported() {
        let err = render_json(
            r#"{"composite":{"type":"brick_pop","names":["a"],"sequences":["ACGT"],
                 "template":"dna","frequencies":[0.5,0.5]}}"#,
        )
        .unwrap_err();
        assert!(err.contains("2 entries"), "unexpected message: {err}");
    }

    #[test]
    fn brick_pop_metric_count_mismatch_is_reported() {
        let err = render_json(
            r#"{"composite":{"type":"brick_pop","names":["a"],"sequences":["ACGT"],
                 "template":"dna","metrics":[{"label":"m","values":[1,2]}]}}"#,
        )
        .unwrap_err();
        assert!(err.contains("2 values"), "unexpected message: {err}");
    }
}
