//! 砖墙图 -> `Plot::Brick`。
//!
//! kuva 这边有真实的 panic 风险，所以校验比别的图型重：配色表必须存在，且序列里不能出现
//! 配色表没覆盖的字符（渲染时是 `template.get(&value).expect(...)`）。
//!
//! 0.6 起还有两处**静默降级**要在扩展侧挡住：STRIGAR 的旧格式（`|` / `@`）与新格式里解析不了的
//! 片段、CIGAR 长度与行不匹配 —— kuva 都只报告不画，SQL 调用方拿不到那个报告，所以这里把它变成错误。

use kuva::plot::brick::{BrickAnchor, BrickSort, BrickTemplate, CigarOp};
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
    if let Some(names) = &s.names
        && names.len() != rows
    {
        return Err(format!(
            "brick: `names` has {} entries but there are {rows} rows to label",
            names.len()
        ));
    }
    if let Some(row) = s.consensus_row
        && row >= rows
    {
        return Err(format!(
            "brick: `consensus_row` is {row} but there are only {rows} rows"
        ));
    }
    if let Some(cigars) = &s.cigars
        && cigars.len() != rows
    {
        return Err(format!(
            "brick: `cigars` has {} entries but there are {rows} rows",
            cigars.len()
        ));
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
    if let Some(v) = s.names.clone() {
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
            "brick: `x_offsets` and `start_positions` both set the per-row offset — give one"
                .into(),
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

    // ---- CIGAR 条（0.6）----
    if let Some(cigars) = s.cigars {
        plot = plot.with_cigars(cigars);
    }
    if let Some(v) = s.cigar_dedup {
        plot = plot.with_cigar_dedup(v);
    }
    if let Some(v) = s.cigar_run_bracket {
        plot = plot.with_cigar_run_bracket(v);
    }
    if let Some(colors) = s.cigar_colors {
        plot = plot.with_cigar_colors(colors.into_iter().map(|(op, color)| (cigar_op(op), color)));
    }
    if let Some(sort) = &s.sort {
        plot = plot.with_sort(match sort {
            BrickSortSpec::Named(BrickSortKind::None) => BrickSort::None,
            BrickSortSpec::Named(BrickSortKind::Cigar) => BrickSort::Cigar,
            BrickSortSpec::Custom { custom } => BrickSort::Custom(custom.clone()),
        });
    }

    // ---- 纵向参考线（0.6）----
    for line in &s.marker_lines {
        if matches!(line.orientation, Some(OrientationKind::Horizontal)) {
            return Err(
                "brick: `marker_lines` must be vertical — drop `orientation` or set it to \"vertical\""
                    .into(),
            );
        }
        let mut r = ReferenceLine::vertical(line.value);
        if let Some(v) = &line.color {
            r = r.with_color(v.clone());
        }
        if let Some(v) = line.stroke_width {
            r = r.with_stroke_width(v);
        }
        if let Some(v) = &line.dasharray {
            r = r.with_dasharray(v.clone());
        }
        if let Some(v) = &line.label {
            r = r.with_label(v.clone());
        }
        plot = plot.with_marker_line(r);
    }

    // kuva 对这两类问题只记录、不画；SQL 调用方看不到那个报告，所以在这里升级成错误。
    report_strigar_issues(&plot, s.names.as_deref())?;
    report_cigar_issues(&plot, s.names.as_deref())?;
    Ok(plot.into())
}

fn cigar_op(op: CigarOpKind) -> CigarOp {
    match op {
        CigarOpKind::Match => CigarOp::Match,
        CigarOpKind::Mismatch => CigarOp::Mismatch,
        CigarOpKind::Insertion => CigarOp::Insertion,
        CigarOpKind::Deletion => CigarOp::Deletion,
        CigarOpKind::SoftClip => CigarOp::SoftClip,
    }
}

/// 把 kuva 的 STRIGAR 拒收报告变成错误。
fn report_strigar_issues(plot: &BrickPlot, names: Option<&[String]>) -> Result<(), String> {
    use kuva::plot::brick::StrigarIssue;
    let issues = plot.strigar_issues();
    let Some(first) = issues.first() else {
        return Ok(());
    };
    let row = match first {
        StrigarIssue::LegacyFormat { row, .. } | StrigarIssue::Unparseable { row, .. } => *row,
    };
    let who = names
        .and_then(|n| n.get(row))
        .map(|n| format!("`{n}` (row {row})"))
        .unwrap_or_else(|| format!("row {row}"));
    let detail = match first {
        StrigarIssue::LegacyFormat { marker, .. } => format!(
            "uses the pre-0.6 STRIGAR format (`{marker}` separators / gap markers). \
             kuva 0.6 rejects it: regenerate the STRIGAR with a current bladerunner, or write runs as \
             `<count><letter>` with no separators (e.g. \"10A2B\")"
        ),
        StrigarIssue::Unparseable { fragment, .. } => format!(
            "contains `{fragment}`, which is neither a repeat count nor an uppercase (base-26) letter"
        ),
    };
    Err(format!(
        "brick: strigar {who} {detail} ({} row(s) affected)",
        issues.len()
    ))
}

/// 把 kuva 的 CIGAR 拒收报告变成错误。
fn report_cigar_issues(plot: &BrickPlot, names: Option<&[String]>) -> Result<(), String> {
    use kuva::plot::brick::CigarIssue;
    let issues = plot.cigar_issues();
    let Some(first) = issues.first() else {
        return Ok(());
    };
    let row = match first {
        CigarIssue::Unparseable { row, .. } | CigarIssue::LengthMismatch { row, .. } => *row,
    };
    let who = names
        .and_then(|n| n.get(row))
        .map(|n| format!("`{n}` (row {row})"))
        .unwrap_or_else(|| format!("row {row}"));
    let detail = match first {
        CigarIssue::Unparseable { cigar, .. } => {
            format!("has a CIGAR `{cigar}` that does not parse")
        }
        CigarIssue::LengthMismatch {
            cigar_read_bases,
            row_bases,
            ..
        } => format!(
            "has a CIGAR covering {cigar_read_bases} read bases but the row draws {row_bases} \
             (flanks included)"
        ),
    };
    Err(format!(
        "brick: cigar {who} {detail}; supply the CIGAR from whatever produced the row ({} row(s) affected)",
        issues.len()
    ))
}

/// strigar 是**游程**语法：`<次数><字母>` 首尾相接，**没有分隔符**（0.6 起）。字母是双射 base-26，
/// 所以一个字母可以是多字符（`AA`）。次数不能省 —— kuva 会把没有次数的片段静默丢掉，画出来少一截。
///
/// 旧格式（`|` 分段、`@` 空位）单独报错：kuva 0.6 直接拒收这类行，且无法从剩下的字符里还原分段。
///
/// STRIGAR is a run syntax: `<count><letter>` back to back with **no separators** (since 0.6).
/// Letters are bijective base-26, so one letter may be several characters (`AA`). A count is
/// mandatory — kuva silently drops fragments without one, which renders a row shorter than asked.
/// The pre-0.6 format (`|` segments, `@` gaps) gets its own message: kuva 0.6 rejects those rows
/// and the segment structure cannot be recovered from what remains.
fn check_strigar_runs(name: &str, seq: &str) -> Result<(), String> {
    if let Some(marker) = seq.chars().find(|c| matches!(c, '|' | '@')) {
        return Err(format!(
            "brick: strigar `{name}` uses the pre-0.6 format (`{marker}`); kuva 0.6 rejects it. \
             Regenerate it with a current bladerunner, or write `<count><letter>` runs with no separators (e.g. \"10A2B\")"
        ));
    }
    let mut digits_seen = false;
    let mut in_letter = false;
    for c in seq.chars() {
        if c.is_ascii_digit() {
            digits_seen = true;
            in_letter = false;
        } else if c.is_ascii_uppercase() {
            if !digits_seen {
                return Err(format!(
                    "brick: strigar `{name}` has `{c}` without a repeat count; every letter needs one, e.g. \"1A1C2G1T\" (got `{seq}`)"
                ));
            }
            in_letter = true;
        } else {
            return Err(format!(
                "brick: strigar `{name}` has `{c}`, which is neither a digit nor an uppercase letter (got `{seq}`)"
            ));
        }
    }
    if digits_seen && !in_letter {
        return Err(format!(
            "brick: strigar `{name}` ends with a count and no letter (got `{seq}`)"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{
        assert_renders, render_json, render_svg,
    };

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
    ///
    /// 0.6 的格式没有分隔符：`"10A"`、`"12A1B"`。字母可以是多字符（`AA`），见下一个用例。
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

    /// 双射 base-26 的多字符字母：`AA` 是第 27 个 motif。
    const BRICK_STRIGAR_BASE26: &str = r##"{
      "series": [{
        "type": "brick",
        "names": ["r1"],
        "strigars": [["CAG:A,TGC:AA", "5A3AA"]],
        "row_height": 20
      }]
    }"##;

    #[test]
    fn renders_brick_strigar_base26_letter() {
        assert_renders(&render_svg(BRICK_STRIGAR_BASE26), "BRICK_STRIGAR_BASE26");
    }

    #[test]
    fn brick_character_outside_template_is_reported() {
        // DNA 配色表里没有 `X`；kuva 渲染时找不到就是 panic。
        let err =
            render_json(r##"{"series":[{"type":"brick","sequences":["ACGX"],"template":"dna"}]}"##)
                .unwrap_err();
        assert!(
            err.contains("color template does not cover"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn brick_strigar_without_repeat_count_is_reported() {
        // kuva 会把没有次数的片段静默丢掉，画出来少一截。
        let err = render_json(
            r#"{"series":[{"type":"brick","names":["r1"],"strigars":[["CAG:A","A"]]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("without a repeat count"),
            "unexpected message: {err}"
        );
    }

    /// 旧格式（`|` / `@`）在 kuva 0.6 里被拒收 —— 扩展必须把它变成错误，而不是画一张空图。
    #[test]
    fn brick_legacy_strigar_format_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"brick","names":["r1"],"strigars":[["CAG:A","10A | 2A"]]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("pre-0.6 format"), "unexpected message: {err}");
        assert!(
            err.contains("bladerunner"),
            "the message should say how to fix it: {err}"
        );
    }

    #[test]
    fn brick_strigar_trailing_count_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"brick","names":["r1"],"strigars":[["CAG:A","10A5"]]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("count and no letter"),
            "unexpected message: {err}"
        );
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
          ["A:A,GAA:B,AGA:C", "16A1B9C"],
          ["AGA:A",           "12A"]
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

    /// CIGAR 条：每行的 CIGAR 必须覆盖该行画出的全部碱基 —— `8A` 的 motif 是 3 bp（24 bp），
    /// 第三行是 `6A2B`（A=3 bp、B=1 bp，共 20 bp）。
    const BRICK_CIGAR: &str = r##"{
      "series": [{
        "type": "brick",
        "names": ["read_1", "read_2", "read_3"],
        "strigars": [
          ["CAG:A", "8A"],
          ["CAG:A", "8A"],
          ["CAG:A,C:B", "6A2B"]
        ],
        "cigars": ["20M4I", "20M4I", "18M2S"],
        "cigar_dedup": true,
        "cigar_run_bracket": true,
        "cigar_colors": {"insertion": "#e9c46a", "soft_clip": "#8ecae6"},
        "sort": "cigar",
        "row_height": 20
      }]
    }"##;

    #[test]
    fn renders_brick_cigar() {
        assert_renders(&render_svg(BRICK_CIGAR), "BRICK_CIGAR");
    }

    #[test]
    fn brick_cigar_length_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"brick","names":["r1"],"strigars":[["CAG:A","8A"]],
                 "cigars":["5M"]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("CIGAR covering"), "unexpected message: {err}");
        assert!(
            err.contains("flanks included"),
            "the message should explain the span: {err}"
        );
    }

    #[test]
    fn brick_cigar_count_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"brick","names":["r1"],"strigars":[["CAG:A","8A"]],
                 "cigars":["8M", null]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("`cigars` has 2 entries"),
            "unexpected message: {err}"
        );
    }

    /// 自定义行序：`{"custom": [...]}` 交给 kuva 的 `BrickSort::Custom`。
    #[test]
    fn renders_brick_custom_sort_and_marker_lines() {
        let svg = render_svg(
            r##"{"series":[{"type":"brick","names":["a","b","c"],
                 "sequences":["ACGT","ACGA","ACGC"],"template":"dna",
                 "sort":{"custom":[2,0,1]},
                 "marker_lines":[{"value":2,"label":"primer","color":"crimson","dasharray":"4 2"}]}]}"##,
        );
        assert_renders(&svg, "BRICK_CUSTOM_SORT");
        assert!(svg.contains("primer"), "the marker line label is missing");
    }

    #[test]
    fn brick_horizontal_marker_line_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"brick","sequences":["ACGT"],"template":"dna",
                 "marker_lines":[{"orientation":"horizontal","value":2}]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("must be vertical"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn brick_two_data_modes_are_reported() {
        let err = render_json(
            r#"{"series":[{"type":"brick","sequences":["ACGT"],"strigars":[["CAG:A","4A"]]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("mutually exclusive"),
            "unexpected message: {err}"
        );
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
