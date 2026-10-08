//! 和弦图 -> `Plot::Chord`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_chord(s: ChordSeries) -> Result<Plot, String> {
    super::check_matrix("chord", &s.matrix)?;
    if s.matrix.is_empty() {
        return Err("chord: `matrix` must not be empty".into());
    }
    let n = s.matrix.len();
    // 缺的那几格 kuva 会当成 0.0 静默补上（不 panic），但那几乎肯定是用错了形状 —— 比如把
    // 一个不对称矩阵当方阵传进来。
    if let Some(bad) = s.matrix.iter().position(|row| row.len() != n) {
        return Err(format!(
            "chord: the matrix must be square, but row {bad} has {} entries (expected {n})",
            s.matrix[bad].len()
        ));
    }
    if let Some(labels) = &s.labels {
        if labels.len() != n {
            return Err(format!(
                "chord: `labels` has {} entries but the matrix is {n}x{n}",
                labels.len()
            ));
        }
    }

    let mut plot = ChordPlot::new().with_matrix(s.matrix);
    if let Some(v) = s.labels {
        plot = plot.with_labels(v);
    }
    if let Some(v) = s.colors {
        plot = plot.with_colors(v);
    }
    if let Some(v) = s.gap_degrees {
        plot = plot.with_gap(v);
    }
    if let Some(v) = s.pad_fraction {
        plot.pad_fraction = v;
    }
    if let Some(v) = s.ribbon_opacity {
        plot = plot.with_opacity(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

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

    #[test]
    fn renders_chord() {
        assert_renders(&render_svg(CHORD), "CHORD");
    }
}
