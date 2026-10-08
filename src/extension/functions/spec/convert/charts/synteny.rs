//! 共线性图 -> `Plot::Synteny`。

use kuva::plot::synteny::{Strand, SyntenyBlock};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_synteny(s: SyntenySeries) -> Result<Plot, String> {
    if s.sequences.is_empty() {
        return Err("synteny: `sequences` must not be empty".into());
    }
    let n = s.sequences.len();
    for (i, b) in s.blocks.iter().enumerate() {
        // kuva 会静默跳过越界的 block，块一多就很难看出「少画了几块」。
        for (field, idx) in [("seq1", b.seq1), ("seq2", b.seq2)] {
            if idx >= n {
                return Err(format!(
                    "synteny: block {i} refers to `{field}` = {idx} but there are only {n} sequences"
                ));
            }
        }
        if b.start1 > b.end1 || b.start2 > b.end2 {
            return Err(format!(
                "synteny: block {i} has a reversed interval ({}, {})",
                b.start1, b.end1
            ));
        }
    }

    let mut plot = SyntenyPlot::new();
    for seq in &s.sequences {
        plot = plot.with_sequences([(seq.label.clone(), seq.length)]);
        if let Some(c) = &seq.color {
            if let Some(last) = plot.sequences.last_mut() {
                last.color = Some(c.clone());
            }
        }
    }
    if let Some(v) = s.sequence_colors {
        plot = plot.with_sequence_colors(v);
    }
    for b in &s.blocks {
        // 缺省方向：上在下的正向、反向由 seq 的先后决定。
        let strand = match &b.strand {
            Some(StrandKind::Forward) => Strand::Forward,
            Some(StrandKind::Reverse) => Strand::Reverse,
            None if b.seq1 <= b.seq2 => Strand::Forward,
            None => Strand::Reverse,
        };
        plot = plot.with_blocks([SyntenyBlock {
            seq1: b.seq1,
            start1: b.start1,
            end1: b.end1,
            seq2: b.seq2,
            start2: b.start2,
            end2: b.end2,
            strand,
            color: b.color.clone(),
        }]);
    }
    if let Some(v) = s.bar_height {
        plot = plot.with_bar_height(v);
    }
    if let Some(v) = s.block_opacity {
        plot = plot.with_opacity(v);
    }
    if s.shared_scale == Some(true) {
        plot = plot.with_shared_scale();
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
