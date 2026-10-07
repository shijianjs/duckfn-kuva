//! 柱状图 -> `Plot::Bar`。简单模式（每类一根）与分组 / 堆叠模式（每组每类一根）都走这里。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::cycle_color;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_bar(s: BarSeries) -> Result<Plot, String> {
    let categories = s
        .categories
        .clone()
        .ok_or("bar: `categories` is required (the category-axis labels)")?;
    if categories.is_empty() {
        return Err("bar: `categories` must not be empty".into());
    }

    let mut plot = BarPlot::new();
    if let Some(groups) = &s.series {
        if groups.is_empty() {
            return Err("bar: `series` must not be empty".into());
        }
        for g in groups {
            if g.values.len() != categories.len() {
                return Err(format!(
                    "bar: series `{}` has {} values but there are {} categories",
                    g.name,
                    g.values.len(),
                    categories.len()
                ));
            }
        }
        for (i, cat) in categories.iter().enumerate() {
            let bars: Vec<(f64, String)> = groups
                .iter()
                .enumerate()
                .map(|(j, g)| (g.values[i], g.color.clone().unwrap_or_else(|| cycle_color(j))))
                .collect();
            plot = plot.with_group(cat.clone(), bars);
        }
        let legend: Vec<&str> = groups.iter().map(|g| g.name.as_str()).collect();
        plot = plot.with_legend(legend);
    } else {
        let values = s.values.ok_or("bar: the simple form needs `values`")?;
        if values.len() != categories.len() {
            return Err(format!(
                "bar: `values` has {} entries but there are {} categories",
                values.len(),
                categories.len()
            ));
        }
        match &s.colors {
            Some(colors) => {
                if colors.len() < values.len() {
                    return Err(format!(
                        "bar: `colors` has {} entries, fewer than the {} values",
                        colors.len(),
                        values.len()
                    ));
                }
                let bars: Vec<(String, f64, String)> = categories
                    .iter()
                    .cloned()
                    .zip(values)
                    .zip(colors.iter().cloned())
                    .map(|((label, value), color)| (label, value, color))
                    .collect();
                plot = plot.with_colored_bars(bars);
            }
            None if s.common.color.is_some() => {
                let bars: Vec<(String, f64)> = categories.iter().cloned().zip(values).collect();
                plot = plot.with_bars(bars);
                plot = plot.with_color(s.common.color.clone().unwrap());
            }
            None => {
                let bars: Vec<(String, f64, String)> = categories
                    .iter()
                    .cloned()
                    .zip(values)
                    .enumerate()
                    .map(|(i, (label, value))| (label, value, cycle_color(i)))
                    .collect();
                plot = plot.with_colored_bars(bars);
            }
        }
        if let Some(v) = &s.common.legend {
            plot = plot.with_legend(vec![v.as_str()]);
        }
    }

    if let Some(v) = s.width {
        plot = plot.with_width(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if s.stacked == Some(true) {
        plot = plot.with_stacked();
    }
    if let Some(v) = s.horizontal {
        plot = plot.with_horizontal(v);
    }
    if let Some(errors) = &s.errors {
        let arms: Vec<(f64, f64)> = errors.iter().map(|e| e.arms()).collect();
        plot = plot.with_asymmetric_error(arms);
    }
    if let Some(v) = &s.error_color {
        plot = plot.with_error_color(v.clone());
    }
    if let Some(v) = s.error_cap_width {
        plot = plot.with_error_cap_width(v);
    }
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}