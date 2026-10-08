//! 雨云图 -> `Plot::Raincloud`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_raincloud(s: RaincloudSeries) -> Result<Plot, String> {
    super::require_groups("raincloud", &s.groups)?;

    let mut plot = RaincloudPlot::new();
    for g in &s.groups {
        plot = plot.with_group(g.label.clone(), g.values.clone());
    }
    if let Some(v) = &s.colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.cloud_width {
        plot = plot.with_cloud_width(v);
    }
    if let Some(v) = s.bandwidth {
        plot = plot.with_bandwidth(v);
    }
    if let Some(v) = s.bandwidth_scale {
        plot = plot.with_bandwidth_scale(v);
    }
    if let Some(v) = s.kde_samples {
        plot = plot.with_kde_samples(v);
    }
    if let Some(v) = s.cloud_alpha {
        plot = plot.with_cloud_alpha(v);
    }
    if let Some(v) = s.show_cloud {
        plot = plot.with_cloud(v);
    }
    if let Some(v) = s.box_width {
        plot = plot.with_box_width(v);
    }
    if let Some(v) = s.show_box {
        plot = plot.with_box(v);
    }
    if let Some(v) = s.rain_size {
        plot = plot.with_rain_size(v);
    }
    if let Some(v) = s.rain_jitter {
        plot = plot.with_rain_jitter(v);
    }
    if let Some(v) = s.rain_alpha {
        plot = plot.with_rain_alpha(v);
    }
    if let Some(v) = s.show_rain {
        plot = plot.with_rain(v);
    }
    if let Some(v) = s.flip {
        plot = plot.with_flip(v);
    }
    if let Some(v) = s.horizontal {
        plot = plot.with_horizontal(v);
    }
    if let Some(v) = s.rain_offset {
        plot = plot.with_rain_offset(v);
    }
    if let Some(v) = s.cloud_offset {
        plot = plot.with_cloud_offset(v);
    }
    if let Some(v) = s.seed {
        plot = plot.with_seed(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const RAINCLOUD: &str = r##"{
      "series": [{
        "type": "raincloud",
        "groups": [
          {"label": "ctrl", "values": [10, 12, 11, 14, 13, 15, 18]},
          {"label": "treat", "values": [14, 16, 15, 19, 20, 22, 25]}
        ],
        "colors": ["#4c72b0", "#c44e52"],
        "cloud_width": 24,
        "bandwidth": 1.2,
        "bandwidth_scale": 1.1,
        "cloud_alpha": 0.5,
        "rain_jitter": 0.08,
        "show_rain": true,
        "show_box": true,
        "show_cloud": true,
        "seed": 7,
        "legend": "arms"
      }]
    }"##;

    #[test]
    fn renders_raincloud() {
        assert_renders(&render_svg(RAINCLOUD), "RAINCLOUD");
    }
}
