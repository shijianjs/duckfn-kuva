//! 砖墙图 -> `Plot::Brick`。
//!
//! kuva 这边有真实的 panic 风险，所以校验比别的图型重：配色表必须存在，且序列里不能出现
//! 配色表没覆盖的字符（渲染时是 `template.get(&value).expect(...)`）。

use kuva::plot::brick::{BrickAnchor, BrickTemplate};
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::brick_template;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_brick(s: BrickSeries) -> Result<Plot, String> {
    let strigar_mode = !s.strigars.is_empty();
    if !strigar_mode && s.sequences.is_empty() {
        return Err("brick: needs `sequences` or `strigars`".into());
    }
    // 缺省补上 DNA 配色：没有它 kuva 渲染时会 `expect("... rendered without template")`。
    // 顺带一提：一旦给了 `strigars`，kuva 会用自己生成的 strigar 配色**覆盖**这个 template。
    let template = match &s.template {
        Some(spec) => brick_template(spec),
        None => BrickTemplate::new().dna().template,
    };
    // 真正画出来的行：strigar 模式下是展开后的 strigar，不是 `sequences`。
    let rendered: Vec<&str> = if strigar_mode {
        s.strigars.iter().map(|pair| pair[1].as_str()).collect()
    } else {
        s.sequences.iter().map(String::as_str).collect()
    };
    if !strigar_mode {
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
        if names.len() != rendered.len() {
            return Err(format!(
                "brick: `names` has {} entries but there are {}{} rows to label",
                names.len(),
                rendered.len(),
                if strigar_mode { " strigar" } else { " sequence" }
            ));
        }
    }
    if let Some(row) = s.consensus_row {
        if row >= rendered.len() {
            return Err(format!(
                "brick: `consensus_row` is {row} but there are only {} rows",
                rendered.len()
            ));
        }
    }

    let mut plot = BrickPlot::new()
        .with_sequences(s.sequences.clone())
        .with_template(template);
    if let Some(v) = s.names {
        plot = plot.with_names(v);
    }
    // kuva 的 `with_strigars` 会**自己生成**配色表并覆盖 `template`，所以自定义的
    // `strigar_palette` 必须在它之前给出。
    for [name, seq] in &s.strigars {
        check_strigar_runs(name, seq)?;
    }
    if let Some(v) = &s.strigar_palette {
        plot = plot.with_strigar_colors(v.clone());
    }
    if strigar_mode {
        plot = plot.with_strigars(s.strigars.iter().map(|[m, q]| (m.clone(), q.clone())));
    }
    if let Some(v) = s.x_offset {
        plot = plot.with_x_offset(v);
    }
    if let Some(v) = s.x_offsets {
        plot = plot.with_x_offsets(v);
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
    if let Some(v) = s.consensus_row {
        plot = plot.with_consensus_row(v);
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
fn check_strigar_runs(name: &str, seq: &str) -> Result<(), String> {
    let mut digits_seen = false;
    for c in seq.chars() {
        if c.is_ascii_digit() {
            digits_seen = true;
            continue;
        }
        if !digits_seen {
            return Err(format!(
                "brick: strigar `{name}` has `{c}` without a repeat count; every letter needs one, e.g. \"1A1C2G1T\" (got `{seq}`)"
            ));
        }
        digits_seen = false;
    }
    Ok(())
}
