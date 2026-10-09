//! 玫瑰图 -> `Plot::Rose`。

use kuva::plot::rose::{RoseEncoding, RoseMode};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_rose(s: RoseSpec) -> Result<Plot, String> {
    // 三种写法互斥：逐扇区 / 多系列 / 原始方位角。
    let entries = [
        !s.slices.is_empty(),
        !s.series.is_empty(),
        !s.bearings.is_empty(),
    ];
    let chosen = entries.iter().filter(|b| **b).count();
    if chosen == 0 {
        return Err("rose: needs one of `slices`, `series` or `bearings`".into());
    }
    if chosen > 1 {
        return Err(
            "rose: `slices`, `series` and `bearings` are three ways to give the same data — give one"
                .into(),
        );
    }
    let multi = !s.series.is_empty();
    let bearing_mode = !s.bearings.is_empty();
    if bearing_mode && s.bearings_bins.is_none() {
        return Err("rose: `bearings` needs `bearings_bins` (how many sectors to bin into)".into());
    }
    if !bearing_mode && s.bearings_bins.is_some() {
        return Err("rose: `bearings_bins` is only used together with `bearings`".into());
    }
    if s.bearings_bins == Some(0) {
        return Err("rose: `bearings_bins` must be at least 1".into());
    }
    let n = if bearing_mode {
        s.bearings_bins.expect("checked just above")
    } else if multi {
        let n = s.labels.as_ref().map_or(0, Vec::len);
        for spec in &s.series {
            if spec.values.len() != n {
                return Err(format!(
                    "rose: series `{}` has {} values but there are {n} labels",
                    spec.name,
                    spec.values.len()
                ));
            }
        }
        n
    } else {
        s.slices.len()
    };
    if n == 0 {
        return Err("rose: needs at least one sector".into());
    }
    if let Some(labels) = &s.labels {
        if labels.len() != n {
            return Err(format!(
                "rose: `labels` has {} entries but there are {n} sectors",
                labels.len()
            ));
        }
    }

    let mut plot = RosePlot::new();
    if bearing_mode {
        // 分箱与计数交给 kuva：它先按 `n` 建好扇区，再 push 一个名为 "Count" 的系列，
        // 所以颜色只能在它之后补。
        plot = plot.with_bearing_data(s.bearings.iter().copied(), n);
        if let Some(c) = &s.color {
            if let Some(first) = plot.series.first_mut() {
                first.color = Some(c.clone());
            }
        }
    } else if multi {
        if let Some(labels) = s.labels {
            plot = plot.with_x_labels(labels);
        }
        for spec in &s.series {
            plot = plot.with_stack(spec.name.clone(), spec.values.clone());
            if let Some(c) = spec.color.clone().or_else(|| s.color.clone()) {
                if let Some(last) = plot.series.last_mut() {
                    last.color = Some(c);
                }
            }
        }
    } else {
        for slice in &s.slices {
            plot = plot.with_slice(slice.label.clone(), slice.value);
            if let Some(c) = slice.color.clone().or_else(|| s.color.clone()) {
                if let Some(last) = plot.series.last_mut() {
                    last.color = Some(c);
                }
            }
        }
    }

    if let Some(v) = &s.encoding {
        plot = plot.with_encoding(match v {
            RoseEncodingKind::Area => RoseEncoding::Area,
            RoseEncodingKind::Radius => RoseEncoding::Radius,
        });
    }
    if let Some(v) = &s.mode {
        plot = plot.with_mode(match v {
            RoseModeKind::Stacked => RoseMode::Stacked,
            RoseModeKind::Grouped => RoseMode::Grouped,
        });
    }
    if let Some(v) = s.start_angle {
        plot = plot.with_start_angle(v);
    }
    if let Some(v) = s.clockwise {
        plot = plot.with_clockwise(v);
    }
    if let Some(v) = s.inner_radius {
        plot = plot.with_inner_radius(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = s.show_grid {
        plot = plot.with_grid(v);
    }
    if let Some(v) = s.grid_lines {
        plot = plot.with_grid_lines(v);
    }
    if let Some(v) = s.show_spokes {
        plot = plot.with_spokes(v);
    }
    if let Some(v) = s.show_labels {
        plot = plot.with_show_labels(v);
    }
    if let Some(v) = s.show_values {
        plot = plot.with_show_values(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    // 方位名要等扇区数定下来之后换 —— `with_compass_labels` 是用当前扇区数推出来的。
    if s.compass_labels == Some(true) {
        plot = plot.with_compass_labels();
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

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

    #[test]
    fn renders_rose() {
        assert_renders(&render_svg(ROSE), "ROSE");
    }

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

    #[test]
    fn renders_rose_stacked() {
        assert_renders(&render_svg(ROSE_STACKED), "ROSE_STACKED");
    }

    /// 方位角写法：原始 bearing 交给 kuva 分箱计数，`compass_labels` 把扇区名换成方位名。
    const ROSE_BEARINGS: &str = r##"{
      "series": [{
        "type": "rose",
        "bearings": [10, 45, 90, 135, 180, 225, 270, 315, 355],
        "bearings_bins": 8,
        "compass_labels": true,
        "color": "#4c72b0"
      }]
    }"##;

    #[test]
    fn renders_rose_bearings() {
        let svg = render_svg(ROSE_BEARINGS);
        assert_renders(&svg, "ROSE_BEARINGS");
        // 8 个扇区 → N / NE / E …，而不是自动的度数标签。
        assert!(svg.contains(">N<"), "expected the compass label N in the SVG");
        assert!(svg.contains(">NE<"), "expected the compass label NE in the SVG");
    }

    #[test]
    fn rose_bearings_without_bins_is_reported() {
        let err = render_json(r#"{"series":[{"type":"rose","bearings":[10,20]}]}"#).unwrap_err();
        assert!(
            err.contains("`bearings` needs `bearings_bins`"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn rose_bearing_and_slices_are_reported() {
        let err = render_json(
            r#"{"series":[{"type":"rose","bearings":[10],"bearings_bins":4,
                 "slices":[{"label":"a","value":1}]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("three ways to give the same data"),
            "unexpected message: {err}"
        );
    }
}
