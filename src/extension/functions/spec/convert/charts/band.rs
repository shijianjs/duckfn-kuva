//! 带状区间 -> `Plot::Band`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_band(s: IntervalSpec) -> Result<Plot, String> {
    if s.x.len() < 2 {
        return Err("band: `x` needs at least 2 entries".into());
    }
    // kuva 用 zip 配对，长度不一致时静默截断到最短。
    for (field, v) in [("y_lower", &s.y_lower), ("y_upper", &s.y_upper)] {
        if v.len() != s.x.len() {
            return Err(format!(
                "band: `{field}` has {} entries but `x` has {}",
                v.len(),
                s.x.len()
            ));
        }
    }
    // 负的不透明度会让 kuva 拼出非法的颜色串，渲染时 panic。
    if let Some(v) = s.opacity {
        if v < 0.0 {
            return Err("band: `opacity` must not be negative".into());
        }
    }

    let mut plot = BandPlot::new(s.x.clone(), s.y_lower.clone(), s.y_upper.clone());
    if let Some(v) = &s.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.opacity {
        plot = plot.with_opacity(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const BAND: &str = r##"{
      "series": [{
        "type": "band",
        "x": [1, 2, 3, 4],
        "y_lower": [0.8, 1.4, 1.9, 2.5],
        "y_upper": [1.2, 1.8, 2.4, 3.1],
        "color": "steelblue",
        "opacity": 0.25,
        "legend": "95% CI"
      }]
    }"##;

    #[test]
    fn renders_band() {
        assert_renders(&render_svg(BAND), "BAND");
    }

    #[test]
    fn band_negative_opacity_is_reported() {
        // 负不透明度会让 kuva 拼出非法的颜色串，渲染时 panic。
        let err = render_json(
            r#"{"series":[{"type":"band","x":[1,2],"y_lower":[0,0],"y_upper":[1,1],"opacity":-0.2}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("must not be negative"), "unexpected message: {err}");
    }
}
