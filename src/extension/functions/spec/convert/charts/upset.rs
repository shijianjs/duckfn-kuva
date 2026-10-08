//! UpSet 图 -> `Plot::UpSet`。

use kuva::plot::upset::UpSetSort;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_upset(s: UpSetSeries) -> Result<Plot, String> {
    if s.set_names.is_empty() {
        return Err("upset: `set_names` must not be empty".into());
    }
    if s.set_names.len() > 64 {
        return Err(format!(
            "upset: {} sets were given, but a bitmask only holds 64",
            s.set_names.len()
        ));
    }
    // kuva 分别遍历 set_names 与 set_sizes，长度不一致时不会 panic，只会画出长短不一的图。
    if s.set_sizes.len() != s.set_names.len() {
        return Err(format!(
            "upset: `set_sizes` has {} entries but there are {} set names",
            s.set_sizes.len(),
            s.set_names.len()
        ));
    }
    for i in &s.intersections {
        if i.mask == 0 {
            return Err("upset: an intersection has `mask` 0, which means the empty set".into());
        }
        // 掩码里超出 set_names 范围的位在点矩阵里找不到行，那一列会静默不画。
        if (i.mask >> s.set_names.len()) != 0 {
            return Err(format!(
                "upset: the intersection mask {} sets bits beyond the {} declared sets",
                i.mask,
                s.set_names.len()
            ));
        }
    }

    let intersections: Vec<(u64, usize)> = s
        .intersections
        .iter()
        .map(|i| (i.mask, i.count))
        .collect();
    let mut plot = UpSetPlot::new().with_data(s.set_names, s.set_sizes, intersections);
    if let Some(v) = &s.sort {
        plot = plot.with_sort(match v {
            UpSetSortKind::ByFrequency => UpSetSort::ByFrequency,
            UpSetSortKind::ByDegree => UpSetSort::ByDegree,
            UpSetSortKind::Natural => UpSetSort::Natural,
        });
    }
    if let Some(v) = s.max_visible {
        plot = plot.with_max_visible(v);
    }
    if let Some(v) = s.counts {
        plot.show_counts = v;
    }
    if s.show_set_sizes == Some(false) {
        plot = plot.without_set_sizes();
    }
    if let Some(v) = &s.bar_color {
        plot = plot.with_bar_color(v.clone());
    }
    if let Some(v) = &s.dot_color {
        plot = plot.with_dot_color(v.clone());
    }
    if let Some(v) = &s.dot_empty_color {
        plot.dot_empty_color = v.clone();
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const UPSET: &str = r##"{
      "series": [{
        "type": "upset",
        "set_names": ["A", "B", "C"],
        "set_sizes": [10, 12, 8],
        "intersections": [
          {"mask": 1, "count": 5},
          {"mask": 3, "count": 3},
          {"mask": 7, "count": 2}
        ],
        "sort": "by_degree",
        "max_visible": 5,
        "bar_color": "#4c72b0",
        "dot_color": "#333333"
      }]
    }"##;

    #[test]
    fn renders_upset() {
        assert_renders(&render_svg(UPSET), "UPSET");
    }

    #[test]
    fn upset_mask_beyond_declared_sets_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"upset","set_names":["a"],"set_sizes":[3],
                 "intersections":[{"mask":3,"count":1}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("sets bits beyond the 1"), "unexpected message: {err}");
    }

    #[test]
    fn upset_set_sizes_length_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"upset","set_names":["a","b"],"set_sizes":[3],
                 "intersections":[{"mask":1,"count":1}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("`set_sizes` has 1 entries"), "unexpected message: {err}");
    }
}
