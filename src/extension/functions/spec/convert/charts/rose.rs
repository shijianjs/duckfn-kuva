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
