//! 甘特图 -> `Plot::Gantt`。

use kuva::plot::gantt::GanttTask;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_gantt(s: GanttSeries) -> Result<Plot, String> {
    if s.tasks.is_empty() {
        return Err("gantt: `tasks` must not be empty".into());
    }
    for t in &s.tasks {
        if !t.milestone.unwrap_or(false) && t.end < t.start {
            return Err(format!(
                "gantt: task `{}` ends at {} before it starts at {}",
                t.label, t.end, t.start
            ));
        }
    }

    let mut plot = GanttPlot::new();
    for t in &s.tasks {
        plot.tasks.push(GanttTask {
            label: t.label.clone(),
            start: t.start,
            end: t.end,
            group: t.group.clone(),
            progress: t.progress,
            color: t.color.clone(),
            is_milestone: t.milestone.unwrap_or(false),
        });
    }
    if let Some(v) = s.now_line {
        plot = plot.with_now_line(v);
    }
    if let Some(v) = s.group_order {
        plot = plot.with_group_order(v);
    }
    if let Some(v) = s.bar_height {
        plot = plot.with_bar_height(v);
    }
    if let Some(v) = s.milestone_size {
        plot = plot.with_milestone_size(v);
    }
    if let Some(v) = s.show_labels {
        plot = plot.with_show_labels(v);
    }
    if let Some(v) = &s.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = &s.group_bg {
        plot = plot.with_group_bg(v.clone());
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
