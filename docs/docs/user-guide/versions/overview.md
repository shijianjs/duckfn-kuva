---
title: Version history
sidebar_position: 1
description: Which kuva version each duckfn_kuva release wraps, and where to read what changed.
---

# Version history

Every release of the extension pins exactly one [kuva](https://crates.io/crates/kuva) release: the
plotting library is compiled into the `.duckdb_extension` file, so upgrading the extension is the only
way to move to a newer kuva. This table is that mapping.

| `duckfn_kuva` | kuva | Released | Highlights |
| --- | --- | --- | --- |
| [0.3.0](./v0.3.0.md) | 0.6.0 | 2026-10-19 | Composites, CIGAR bricks, LOESS/repel, `kuva_render_typst`, `kuva_version()` |
| [0.2.0](./v0.2.0.md) | 0.5.0 | 2026-10-13 | `dataset`/`encode` second argument, JSON Schema and the TypeScript, Java and Kotlin SDKs |
| [0.1.0](./v0.1.0.md) | 0.5.0 | 2026-10-10 | First release: all 64 plot types, terminal and file output, multi-panel figures |

## Checking the kuva version at runtime

`kuva_version()` reports the kuva release the loaded build was compiled against:

```sql
SELECT kuva_version();
-- 0.6.0
```

If it does not match this table, the file you loaded is not the release you think it is — usually an
older `.duckdb_extension` left in the extension directory, or a locally built artifact from another
branch.

## Reading a release

Each version page summarises what changed and links two places on GitHub:

- the **Release**, with one `.duckdb_extension` per platform plus the WebAssembly build;
- the **commit list** between the two version tags, i.e. the actual diff.

Only released versions get a page. The development version on `main` (`X.Y.Z-dev.N`) is not listed: it
has no tag and no release file.
