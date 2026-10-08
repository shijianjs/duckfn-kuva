//! 日历热力图 -> `Plot::Calendar`。

use kuva::plot::calendar::{CalendarAgg, CalendarPlot, WeekStart};
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::color_map;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_calendar(s: CalendarSeries) -> Result<Plot, String> {
    if s.data.is_empty() && s.events.is_empty() {
        return Err("calendar: needs `data` or `events`".into());
    }
    for (i, d) in s
        .data
        .iter()
        .map(|p| p.date.as_str())
        .chain(s.events.iter().map(String::as_str))
        .enumerate()
    {
        // kuva 的日期解析是按字节切 `s[0..4]` / `s[5..7]` / `s[8..10]`，非 ASCII 字符串会 panic。
        if !d.is_ascii() {
            return Err(format!(
                "calendar: date #{i} (`{d}`) contains non-ASCII characters; dates must be ASCII `YYYY-MM-DD`"
            ));
        }
    }

    let mut plot = CalendarPlot::new()
        .with_data(s.data.iter().map(|p| (p.date.clone(), p.value)));
    if !s.events.is_empty() {
        plot = plot.with_events(s.events.clone());
    }
    if let Some(v) = &s.aggregation {
        plot = plot.with_aggregation(match v {
            CalendarAggKind::Count => CalendarAgg::Count,
            CalendarAggKind::Sum => CalendarAgg::Sum,
            CalendarAggKind::Mean => CalendarAgg::Mean,
            CalendarAggKind::Max => CalendarAgg::Max,
        });
    }
    if let Some(v) = &s.color_map {
        plot = plot.with_color_map(color_map(v));
    }
    if let Some(v) = &s.missing_color {
        plot = plot.with_missing_color(v.clone());
    }
    if let Some(v) = &s.zero_color {
        plot = plot.with_zero_color(v.clone());
    }
    if let Some(v) = &s.week_start {
        plot = plot.with_week_start(match v {
            WeekStartKind::Monday => WeekStart::Monday,
            WeekStartKind::Sunday => WeekStart::Sunday,
        });
    }
    if let Some(v) = s.month_labels {
        plot = plot.with_month_labels(v);
    }
    if let Some(v) = s.day_labels {
        plot = plot.with_day_labels(v);
    }
    if let Some(v) = s.cell_size {
        plot = plot.with_cell_size(v);
    }
    if let Some(v) = s.cell_gap {
        plot = plot.with_cell_gap(v);
    }
    if let Some(v) = s.legend {
        plot = plot.with_legend(v);
    }
    if let Some(v) = &s.legend_label {
        plot = plot.with_legend_label(v.clone());
    }
    if let Some((lo, hi)) = s.value_range {
        plot = plot.with_value_range(lo, hi);
    }
    if let Some(v) = s.year {
        plot = plot.with_year(v);
    }
    if let Some(v) = s.years {
        plot = plot.with_years(v);
    }
    for p in &s.periods {
        plot = plot.with_period(p.label.clone(), p.start.clone(), p.end.clone());
    }
    if let Some(r) = &s.date_range {
        plot = plot.with_date_range(r.start.clone(), r.end.clone());
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const CALENDAR: &str = r##"{
      "series": [{
        "type": "calendar",
        "data": [
          {"date": "2024-01-01", "value": 3},
          {"date": "2024-01-02", "value": 5},
          {"date": "2024-02-14", "value": 9}
        ],
        "events": ["2024-03-01", "2024-03-02"],
        "aggregation": "sum",
        "color_map": "greens",
        "missing_color": "#f0f0f0",
        "zero_color": "#ffffff",
        "week_start": "sunday",
        "month_labels": true,
        "day_labels": false,
        "cell_size": 12,
        "cell_gap": 2,
        "legend": true,
        "legend_label": "commits",
        "value_range": [0, 10],
        "periods": [{"label": "Q1", "start": "2024-01-01", "end": "2024-03-31"}]
      }]
    }"##;

    #[test]
    fn renders_calendar() {
        assert_renders(&render_svg(CALENDAR), "CALENDAR");
    }
}
