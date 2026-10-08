//! 漏斗图 -> `Plot::Funnel`。

use kuva::plot::funnel::{FunnelColorMode, FunnelOrientation, FunnelStage};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_funnel(s: FunnelSeries) -> Result<Plot, String> {
    if s.stages.is_empty() {
        return Err("funnel: `stages` must not be empty".into());
    }
    // kuva 用 `fold(0.0, f64::max)` 求最大值，全 0 时早退、整张图不画。
    if s.stages.iter().all(|x| x.value <= 0.0) {
        return Err("funnel: every stage is 0, so the chart would come out blank".into());
    }

    let mut plot = FunnelPlot::new();
    for stage in &s.stages {
        plot = match &stage.color {
            Some(c) => plot.with_stage_color(stage.label.clone(), stage.value, c.clone()),
            None => plot.with_stage(stage.label.clone(), stage.value),
        };
    }
    if let Some(mirror) = &s.mirror {
        // 镜像组只进 `mirror` 队列，不能顺手也加进 `stages`（那就是主漏斗了）。
        plot = FunnelPlot {
            mirror: Some(
                mirror
                    .iter()
                    .map(|x| FunnelStage {
                        label: x.label.clone(),
                        value: x.value,
                        color: x.color.clone(),
                    })
                    .collect(),
            ),
            ..plot
        };
    }
    if let (Some(l), Some(r)) = (&s.left_label, &s.right_label) {
        plot = plot.with_mirror_labels(l.clone(), r.clone());
    } else {
        plot.left_label = s.left_label.clone();
        plot.right_label = s.right_label.clone();
    }
    if let Some(v) = &s.orientation {
        plot = plot.with_orientation(match v {
            FunnelOrientationKind::Vertical => FunnelOrientation::Vertical,
            FunnelOrientationKind::Horizontal => FunnelOrientation::Horizontal,
        });
    }
    if let Some(v) = s.show_connectors {
        plot = plot.with_connectors(v);
    }
    if let Some(v) = s.connector_opacity {
        plot = plot.with_connector_opacity(v);
    }
    if let Some(v) = s.show_values {
        plot = plot.with_show_values(v);
    }
    if let Some(v) = s.show_percents {
        plot = plot.with_show_percents(v);
    }
    if let Some(v) = s.show_conversion {
        plot = plot.with_show_conversion(v);
    }
    if let Some(v) = &s.color_mode {
        plot = plot.with_color_mode(match v {
            FunnelColorModeKind::Uniform => FunnelColorMode::Uniform,
            FunnelColorModeKind::ByStage => FunnelColorMode::ByStage,
            FunnelColorModeKind::Gradient => FunnelColorMode::Gradient,
        });
    }
    if let Some(v) = s.stage_gap {
        plot = plot.with_stage_gap(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
