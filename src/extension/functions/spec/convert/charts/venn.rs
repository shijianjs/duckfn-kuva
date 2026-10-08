//! 韦恩图 -> `Plot::Venn`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_venn(s: VennSeries) -> Result<Plot, String> {
    // kuva 对超过 4 个集合直接不画（一行白图），所以这里先挡下来。
    if s.sets.is_empty() || s.sets.len() > 4 {
        return Err(format!(
            "venn: {} sets were given; it supports 1 to 4",
            s.sets.len()
        ));
    }
    let raw = s.sets.iter().any(|x| x.elements.is_some());
    let sized = s.sets.iter().any(|x| x.size.is_some());
    if raw && sized {
        return Err("venn: mix the two set forms — either every set has `elements`, or none does (and then `overlaps` carries the intersections)".into());
    }

    let mut plot = VennPlot::new();
    for set in &s.sets {
        plot = match (&set.elements, set.size) {
            (Some(elements), _) => plot.with_set(set.label.clone(), elements.clone()),
            (None, Some(n)) => plot.with_set_size(set.label.clone(), n),
            // 两套写法都没给时 kuva 会当成大小 0 的集合；要求显式写出来更省心。
            (None, None) => {
                return Err(format!(
                    "venn: set `{}` needs either `elements` or `size`",
                    set.label
                ));
            }
        };
    }
    if !raw {
        let known: std::collections::HashSet<&str> =
            s.sets.iter().map(|x| x.label.as_str()).collect();
        for o in &s.overlaps {
            if o.sets.is_empty() {
                return Err("venn: an `overlaps` entry has no `sets`".into());
            }
            for name in &o.sets {
                if !known.contains(name.as_str()) {
                    return Err(format!(
                        "venn: an `overlaps` entry refers to an unknown set `{name}`"
                    ));
                }
            }
            // 同一个集合在一行里出现两次没有意义，kuva 会照单全收成另一个 mask。
            let mut unique = o.sets.clone();
            unique.sort();
            unique.dedup();
            if unique.len() != o.sets.len() {
                return Err(format!(
                    "venn: the overlap of [{}] lists a set more than once",
                    o.sets.join(", ")
                ));
            }
            plot = plot.with_overlap(o.sets.clone(), o.size);
        }
    }

    if let Some(v) = s.counts {
        plot = plot.with_counts(v);
    }
    if let Some(v) = s.percentages {
        plot = plot.with_percentages(v);
    }
    if let Some(v) = s.set_labels {
        plot = plot.with_set_labels(v);
    }
    if let Some(v) = s.fill_opacity {
        plot = plot.with_fill_opacity(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.proportional {
        plot = plot.with_proportional(v);
    }
    if let Some(v) = s.loss {
        plot = plot.with_loss(v);
    }
    if let Some(v) = s.colors {
        plot = plot.with_colors(v);
    }
    if let Some(v) = s.leader_lines {
        plot = plot.with_leader_lines(v);
    }
    if let Some(v) = s.set_indicators {
        plot = plot.with_set_indicators(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
