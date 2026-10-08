//! 小提琴图 -> `Plot::Violin`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_violin(s: ViolinSeries) -> Result<Plot, String> {
    super::require_groups("violin", &s.groups)?;

    let mut plot = ViolinPlot::new();
    // 先把左半边的组都加进去：`with_split_group` 按下标从 `groups` 里取标签，所以顺序有依赖。
    for g in &s.groups {
        plot = plot.with_group(g.label.clone(), g.values.clone());
    }
    if let Some(split) = s.split {
        if split {
            let halves = s.split_groups.as_deref().unwrap_or(&[]);
            if halves.len() > s.groups.len() {
                return Err(format!(
                    "violin: `split_groups` has {} entries but there are only {} groups to pair them with",
                    halves.len(),
                    s.groups.len()
                ));
            }
            for h in halves {
                plot = plot.with_split_group(h.values.clone());
            }
        }
        plot = plot.with_split(split);
    }
    if let Some(v) = &s.split_color {
        plot = plot.with_split_color(v.clone());
    }
    if let Some(v) = &s.split_group_colors {
        plot = plot.with_split_group_colors(v.clone());
    }
    if let Some(v) = &s.split_legend {
        plot = plot.with_split_legend(v.clone());
    }

    if let Some(v) = &s.colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.width {
        plot = plot.with_width(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = s.bandwidth {
        plot = plot.with_bandwidth(v);
    }
    if let Some(v) = s.kde_samples {
        plot = plot.with_kde_samples(v);
    }
    if let Some(v) = s.strip {
        plot = plot.with_strip(v);
    }
    if s.swarm == Some(true) {
        plot = plot.with_swarm_overlay();
    }
    if let Some(v) = &s.overlay_color {
        plot = plot.with_overlay_color(v.clone());
    }
    if let Some(v) = s.overlay_size {
        plot = plot.with_overlay_size(v);
    }
    if let Some(v) = s.horizontal {
        plot = plot.with_horizontal(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
