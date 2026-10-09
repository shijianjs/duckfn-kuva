//! 砖墙图 -> `Plot::Brick`。
//!
//! kuva 这边有真实的 panic 风险，所以校验比别的图型重：配色表必须存在，且序列里不能出现
//! 配色表没覆盖的字符（渲染时是 `template.get(&value).expect(...)`）。

use kuva::plot::brick::{BrickAnchor, BrickTemplate};
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::brick_template;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_brick(s: BrickSeries) -> Result<Plot, String> {
    let chosen = [
        !s.sequences.is_empty(),
        !s.strigars.is_empty(),
        !s.flanked_strigars.is_empty(),
    ]
    .iter()
    .filter(|b| **b)
    .count();
    if chosen == 0 {
        return Err("brick: needs one of `sequences`, `strigars` or `flanked_strigars`".into());
    }
    if chosen > 1 {
        return Err(
            "brick: `sequences`, `strigars` and `flanked_strigars` are mutually exclusive — give exactly one"
                .into(),
        );
    }
    let sequence_mode = !s.sequences.is_empty();
    let flanked_mode = !s.flanked_strigars.is_empty();
    // 缺省补上 DNA 配色：没有它 kuva 渲染时会 `expect("... rendered without template")`。
    // 顺带一提：一旦给了 `strigars`，kuva 会用自己生成的 strigar 配色**覆盖**这个 template。
    let template = match &s.template {
        Some(spec) => brick_template(spec),
        None => BrickTemplate::new().dna().template,
    };
    // 真正画出来的行数：strigar / flanked-strigar 一行一条，与 `sequences` 的行数同义。
    let rows = if sequence_mode {
        s.sequences.len()
    } else if flanked_mode {
        s.flanked_strigars.len()
    } else {
        s.strigars.len()
    };
    // STRIGAR（含 FLANKED-STRIGAR 里那一段）的 `(motif, strigar)` 对，用来做游程校验。
    let strigar_sources: Vec<(&str, &str)> = if sequence_mode {
        Vec::new()
    } else if flanked_mode {
        s.flanked_strigars
            .iter()
            .map(|f| (f[1].as_str(), f[2].as_str()))
            .collect()
    } else {
        s.strigars
            .iter()
            .map(|p| (p[0].as_str(), p[1].as_str()))
            .collect()
    };
    if sequence_mode {
        let known: std::collections::HashSet<char> = template.keys().copied().collect();
        for (i, seq) in s.sequences.iter().enumerate() {
            if let Some(bad) = seq.chars().find(|c| !known.contains(c)) {
                return Err(format!(
                    "brick: sequence #{i} contains `{bad}`, which the color template does not cover (known: {})",
                    {
                        let mut ks: Vec<char> = known.iter().copied().collect();
                        ks.sort_unstable();
                        ks.into_iter().collect::<String>()
                    }
                ));
            }
        }
    }
    if let Some(names) = &s.names {
        if names.len() != rows {
            return Err(format!(
                "brick: `names` has {} entries but there are {rows} rows to label",
                names.len()
            ));
        }
    }
    if let Some(row) = s.consensus_row {
        if row >= rows {
            return Err(format!(
                "brick: `consensus_row` is {row} but there are only {rows} rows"
            ));
        }
    }

    for (i, (motif, strigar)) in strigar_sources.iter().enumerate() {
        let who = s
            .names
            .as_ref()
            .and_then(|names| names.get(i))
            .map(String::as_str)
            .unwrap_or(*motif);
        check_strigar_runs(who, strigar)?;
    }

    let mut plot = BrickPlot::new()
        .with_sequences(s.sequences.clone())
        .with_template(template);
    if let Some(v) = s.names {
        plot = plot.with_names(v);
    }
    // kuva 的 `with_strigars` 会**自己生成**配色表并覆盖 `template`，所以自定义的
    // `strigar_palette` 必须在它之前给出；`with_consensus_row` 也必须在它之前 —— 旋转的解析
    // 就发生在 strigar 展开的那一刻（官方文档明说 consensus row 要先设）。
    if let Some(v) = &s.strigar_palette {
        plot = plot.with_strigar_colors(v.clone());
    }
    if let Some(v) = s.consensus_row {
        plot = plot.with_consensus_row(v);
    }
    if flanked_mode {
        plot = plot.with_flanked_strigars(
            s.flanked_strigars
                .iter()
                .map(|f| (f[0].clone(), f[1].clone(), f[2].clone(), f[3].clone())),
        );
    } else if !sequence_mode {
        plot = plot.with_strigars(s.strigars.iter().map(|[m, q]| (m.clone(), q.clone())));
    }
    if s.x_offsets.is_some() && s.start_positions.is_some() {
        return Err(
            "brick: `x_offsets` and `start_positions` both set the per-row offset — give one".into(),
        );
    }
    if let Some(v) = s.x_offset {
        plot = plot.with_x_offset(v);
    }
    if let Some(v) = s.x_offsets {
        plot = plot.with_x_offsets(v);
    }
    if let Some(v) = s.start_positions {
        plot = plot.with_start_positions(v);
    }
    if let Some(v) = s.x_origin {
        plot = plot.with_x_origin(v);
    }
    if s.show_values == Some(true) {
        plot = plot.with_values();
    }
    if let Some(v) = &s.anchor {
        plot = plot.with_anchor(match v {
            BrickAnchorKind::Left => BrickAnchor::Left,
            BrickAnchorKind::Right => BrickAnchor::Right,
        });
    }
    if s.mark_primary == Some(true) {
        plot = plot.with_mark_primary();
    }
    if let Some(v) = s.notations {
        plot = plot.with_notations(v);
    }
    if let Some(v) = s.row_height {
        plot = plot.with_row_height(v);
    }
    Ok(plot.into())
}

/// strigar 的序列是**游程**语法：**每个**字母前面都要带重复次数（`"1A1C2G1T"` = A、C、G×2、T）。
/// kuva 在展开时对没有次数的字母做 `parse::<usize>().expect(...)`，直接 panic，所以在这里挡住。
///
/// `|` 用来分段（kuva 先按 `|` 切、再对每段 trim），所以段与段之间的空格是合法的；但**段内**
/// 的空格不合法 —— kuva 只 trim 两端，段中间那个空格会被当成字母，于是又走到同一条 panic 上。
fn check_strigar_runs(name: &str, seq: &str) -> Result<(), String> {
    for seg in seq.split('|') {
        let seg = seg.trim();
        if seg.is_empty() {
            continue;
        }
        let mut digits_seen = false;
        for c in seg.chars() {
            if c.is_ascii_digit() {
                digits_seen = true;
                continue;
            }
            if c.is_whitespace() {
                return Err(format!(
                    "brick: strigar `{name}` has whitespace inside the segment `{seg}`; kuva only trims the ends, so write `10A | 2B` rather than `10 A`"
                ));
            }
            if !digits_seen {
                return Err(format!(
                    "brick: strigar `{name}` has `{c}` without a repeat count; every letter needs one, e.g. \"1A1C2G1T\" (got `{seg}`)"
                ));
            }
            digits_seen = false;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const BRICK: &str = r##"{
      "series": [{
        "type": "brick",
        "sequences": ["ACGTACGT", "ACGTTCGT", "ACGTACGA"],
        "names": ["s1", "s2", "s3"],
        "template": "dna",
        "x_offset": 0,
        "x_origin": 0,
        "show_values": true,
        "anchor": "left",
        "mark_primary": true,
        "consensus_row": 0,
        "notations": ["n1", null, "n3"],
        "row_height": 18
      }]
    }"##;

    #[test]
    fn renders_brick() {
        assert_renders(&render_svg(BRICK), "BRICK");
    }

    /// STRIGAR 模式：展开后的 strigar 就是画出来的行，`sequences` 不参与。
    const BRICK_STRIGAR: &str = r##"{
      "series": [{
        "type": "brick",
        "names": ["read_1", "read_2", "read_3"],
        "strigars": [
          ["CAG:A", "10A"],
          ["CAG:A", "8A"],
          ["CAG:A,C:B", "12A1B"]
        ],
        "x_origin": 0,
        "consensus_row": 0,
        "row_height": 20
      }]
    }"##;

    #[test]
    fn renders_brick_strigar() {
        assert_renders(&render_svg(BRICK_STRIGAR), "BRICK_STRIGAR");
    }

    #[test]
    fn brick_character_outside_template_is_reported() {
        // DNA 配色表里没有 `X`；kuva 渲染时找不到就是 panic。
        let err = render_json(
            r##"{"series":[{"type":"brick","sequences":["ACGX"],"template":"dna"}]}"##,
        )
        .unwrap_err();
        assert!(err.contains("color template does not cover"), "unexpected message: {err}");
    }

    #[test]
    fn brick_strigar_without_repeat_count_is_reported() {
        // kuva 展开 strigar 时对没有次数的字母做 parse().expect(..)，直接 panic。
        let err = render_json(
            r#"{"series":[{"type":"brick","names":["r1"],"strigars":[["CAG:A","A"]]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("without a repeat count"), "unexpected message: {err}");
    }

    /// FLANKED-STRIGAR：两侧的原始 DNA 直接给字符串，中间那段照旧是 `(motif, strigar)`。
    const BRICK_FLANKED: &str = r##"{
      "series": [{
        "type": "brick",
        "names": ["consensus", "read_1"],
        "flanked_strigars": [
          ["ACGTACGT", "CAG:A,CAA:B", "6A1B8A", "TGCATGCA"],
          ["ACGTACGT", "CAG:A",       "16A",    "TGCATGCA"]
        ],
        "consensus_row": 0,
        "mark_primary": true,
        "row_height": 20
      }]
    }"##;

    #[test]
    fn renders_brick_flanked() {
        assert_renders(&render_svg(BRICK_FLANKED), "BRICK_FLANKED");
    }

    /// 逐行的参考起点（基因组坐标）：kuva 内部是 `x_offsets` 取负，与 `x_origin` 配合把某个
    /// 位置钉到 x = 0。
    const BRICK_START_POSITIONS: &str = r##"{
      "series": [{
        "type": "brick",
        "names": ["read_1", "read_2"],
        "strigars": [
          ["A:A | @:GAA | AGA:B", "16A | 1@ | 9B"],
          ["AGA:A",               "12A"]
        ],
        "start_positions": [0, 19],
        "x_origin": 19,
        "row_height": 20
      }]
    }"##;

    #[test]
    fn renders_brick_start_positions() {
        assert_renders(&render_svg(BRICK_START_POSITIONS), "BRICK_START_POSITIONS");
    }

    #[test]
    fn brick_two_data_modes_are_reported() {
        let err = render_json(
            r#"{"series":[{"type":"brick","sequences":["ACGT"],"strigars":[["CAG:A","4A"]]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("mutually exclusive"), "unexpected message: {err}");
    }

    #[test]
    fn brick_offsets_and_start_positions_are_reported() {
        let err = render_json(
            r#"{"series":[{"type":"brick","sequences":["ACGT"],"x_offsets":[1.0],
                 "start_positions":[2.0]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("both set the per-row offset"),
            "unexpected message: {err}"
        );
    }
}
