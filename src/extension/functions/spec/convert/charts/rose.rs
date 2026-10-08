//! 玫瑰图 -> `Plot::Rose`。

use kuva::plot::rose::{RoseEncoding, RoseMode};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_rose(s: RoseSpec) -> Result<Plot, String> {
    if s.slices.is_empty() && s.series.is_empty() {
        return Err("rose: needs `slices` or `series`".into());
    }
    let multi = !s.series.is_empty();
    if !s.slices.is_empty() && multi {
        return Err("rose: `slices` and `series` are two ways to give the same data — give one".into());
    }
    let n = if multi {
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
    if multi {
        if let Some(labels) = s.labels {
            plot = plot.with_x_labels(labels);
        }
        for spec in &s.series {
            plot = plot.with_stack(spec.name.clone(), spec.values.clone());
            if let Some(c) = &spec.color {
                if let Some(last) = plot.series.last_mut() {
                    last.color = Some(c.clone());
                }
            }
        }
    } else {
        for slice in &s.slices {
            plot = plot.with_slice(slice.label.clone(), slice.value);
            if let Some(c) = &slice.color {
                if let Some(last) = plot.series.last_mut() {
                    last.color = Some(c.clone());
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
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

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
}
