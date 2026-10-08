//! 曼哈顿图 -> `Plot::Manhattan`。

use kuva::plot::manhattan::GenomeBuild;
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::volcano_label_style;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_manhattan(s: ManhattanSeries) -> Result<Plot, String> {
    if s.points.is_empty() {
        return Err("manhattan: `points` must not be empty".into());
    }
    for (i, p) in s.points.iter().enumerate() {
        if p.pvalue < 0.0 {
            // kuva 只把 p > 0 的点算进 floor，负 p 值会一路走到 -log10(负) 变成 NaN。
            return Err(format!(
                "manhattan: point {i} has a negative pvalue ({})",
                p.pvalue
            ));
        }
    }

    let mut plot = match s.build {
        Some(build) => {
            let b = match build {
                GenomeBuildKind::Hg19 => GenomeBuild::Hg19,
                GenomeBuildKind::Hg38 => GenomeBuild::Hg38,
                GenomeBuildKind::T2T => GenomeBuild::T2T,
            };
            let data: Vec<(String, f64, f64)> = s
                .points
                .iter()
                .map(|p| {
                    (
                        p.chromosome.clone(),
                        p.position.unwrap_or(f64::NAN),
                        p.pvalue,
                    )
                })
                .collect();
            if s.points.iter().any(|p| p.position.is_none()) {
                return Err(
                    "manhattan: `build` is given, so every point needs a `position` (in bp)".into(),
                );
            }
            ManhattanPlot::new().with_data_bp(data, b)
        }
        None => {
            let all_have_position = s.points.iter().all(|p| p.position.is_some());
            if all_have_position {
                let data: Vec<(String, f64, f64)> = s
                    .points
                    .iter()
                    .map(|p| (p.chromosome.clone(), p.position.unwrap(), p.pvalue))
                    .collect();
                ManhattanPlot::new().with_data_x(data)
            } else {
                let data: Vec<(String, f64)> =
                    s.points.iter().map(|p| (p.chromosome.clone(), p.pvalue)).collect();
                ManhattanPlot::new().with_data(data)
            }
        }
    };

    // 显式标签优先；没给的靠 `label_top` 自动挑，所以这里只补有 label 的那些。
    let labeled: Vec<(String, f64, String)> = s
        .points
        .iter()
        .filter_map(|p| p.label.as_ref().map(|l| (p.chromosome.clone(), p.pvalue, l.clone())))
        .collect();
    if !labeled.is_empty() {
        plot = plot.with_point_labels(labeled);
    }
    if let Some(v) = s.genome_wide {
        plot = plot.with_genome_wide(v);
    }
    if let Some(v) = s.suggestive {
        plot = plot.with_suggestive(v);
    }
    if let Some(v) = &s.color_a {
        plot = plot.with_color_a(v.clone());
    }
    if let Some(v) = &s.color_b {
        plot = plot.with_color_b(v.clone());
    }
    if let Some(v) = s.point_size {
        plot = plot.with_point_size(v);
    }
    if let Some(v) = s.label_top {
        plot = plot.with_label_top(v);
    }
    if let Some(style) = &s.label_style {
        plot = plot.with_label_style(volcano_label_style(style));
    }
    if let Some(v) = s.pvalue_floor {
        plot = plot.with_pvalue_floor(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    // 统一 `color` 在这个图型里没有对应字段（奇偶位点各有一个颜色），刻意忽略。
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

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

    #[test]
    fn renders_manhattan() {
        assert_renders(&render_svg(MANHATTAN), "MANHATTAN");
    }

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

    #[test]
    fn renders_manhattan_no_build() {
        assert_renders(&render_svg(MANHATTAN_NO_BUILD), "MANHATTAN_NO_BUILD");
    }

    const MANHATTAN_X: &str = r##"{
      "series": [{
        "type": "manhattan",
        "points": [
          {"chromosome": "1", "position": 10, "pvalue": 1e-6},
          {"chromosome": "1", "position": 20, "pvalue": 0.5}
        ]
      }]
    }"##;

    #[test]
    fn renders_manhattan_x() {
        assert_renders(&render_svg(MANHATTAN_X), "MANHATTAN_X");
    }

    #[test]
    fn manhattan_build_without_position_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"manhattan","points":[{"chromosome":"1","pvalue":0.1}],"build":"hg38"}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("needs a `position`"), "unexpected message: {err}");
    }
}
