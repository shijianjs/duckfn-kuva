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

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const VENN: &str = r##"{
      "series": [{
        "type": "venn",
        "sets": [
          {"label": "A", "size": 20},
          {"label": "B", "size": 18},
          {"label": "C", "size": 15}
        ],
        "overlaps": [
          {"sets": ["A", "B"], "size": 8},
          {"sets": ["A", "B", "C"], "size": 3}
        ],
        "counts": true,
        "percentages": true,
        "fill_opacity": 0.3,
        "proportional": true,
        "leader_lines": true,
        "colors": ["#4c72b0", "#c44e52", "#55a868"],
        "legend": "sets"
      }]
    }"##;

    #[test]
    fn renders_venn() {
        assert_renders(&render_svg(VENN), "VENN");
    }

    const VENN_ELEMENTS: &str = r##"{
      "series": [{
        "type": "venn",
        "sets": [
          {"label": "A", "elements": ["x", "y", "z"]},
          {"label": "B", "elements": ["y", "z", "w"]}
        ],
        "set_labels": true
      }]
    }"##;

    #[test]
    fn renders_venn_elements() {
        assert_renders(&render_svg(VENN_ELEMENTS), "VENN_ELEMENTS");
    }

    #[test]
    fn venn_with_too_many_sets_is_reported() {
        // kuva 对超过 4 个集合直接不画（一行白图）。
        let err = render_json(
            r#"{"series":[{"type":"venn","sets":[{"label":"a","size":1},{"label":"b","size":1},
                 {"label":"c","size":1},{"label":"d","size":1},{"label":"e","size":1}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("it supports 1 to 4"), "unexpected message: {err}");
    }

    #[test]
    fn venn_mixed_set_forms_are_reported() {
        // 一个 set 带 elements 就整体走原始元素模式，预计算的 size/overlap 会被静默忽略。
        let err = render_json(
            r#"{"series":[{"type":"venn","sets":[{"label":"a","elements":["x"]},{"label":"b","size":3}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("mix the two set forms"), "unexpected message: {err}");
    }
}
