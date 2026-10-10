---
title: File output
sidebar_position: 17
description: Writing a rendered chart to an SVG file straight from SQL, with an optional open in the browser.
---

# File output

`kuva_render_file` renders any plot and **writes it to a file**, returning the path. It is the one-statement
convenience layer over `COPY (SELECT kuva_render(…)) TO …`: instead of getting the SVG back as a string and
finding somewhere to put it, you tell the function where the file goes and it hands back the path.

This is the entry point for scripts, CI and notebooks — anywhere "render a chart and put it next to my
results" is one step and the SVG text in the middle is just noise.

## Usage

The function takes **one** argument — the same JSON `kuva_render` takes, plus a top-level `file` object
holding the output settings:

```sql
SELECT kuva_render_file('{"file":{"dir":"/tmp/charts","name":"scatter.svg","open":true},
                          "series":[{"type":"scatter","data":[[1,2],[3,4]]}]}');
-- /tmp/charts/scatter.svg
```

| Field | Default | What it sets |
| --- | --- | --- |
| `file.dir` | system temp directory | Output directory. |
| `file.name` | `kuva-<time>-<random>[-<type>-<title>].svg` | Output file name. A name with no extension gets `.svg` appended. |
| `file.open` | `false` | Open the written file in the system default browser. |

What comes back is the path that was actually written, so it is ready to feed into `read_text(…)`:

```sql
SELECT kuva_render_file('{"file":{"dir":"/tmp/charts","name":"scatter.svg"},
                          "series":[{"type":"scatter","data":[[1,2],[3,4]]}]}') AS path;
-- /tmp/charts/scatter.svg

SELECT left(content, 4) FROM read_text('/tmp/charts/scatter.svg');
-- <svg
```

## Naming and the directory

- **`file.dir`** is used as given; an empty string is an error. Without it the file lands in the system temp
  directory.
- **`file.name`** is used as given, after the file-name legality rules have run: illegal characters
  (`/ \ ? < > : * | "`), control characters, Windows reserved device names (`CON`, `NUL`, `COM1`…) and
  trailing dots and spaces are all replaced with `_`. That is also what keeps a `/` in a name from escaping
  the directory. A name with no `.svg` extension gets one, so the system renders the file as an image.
- **Without a name**, one is generated as `kuva-<time>-<random>[-<type>-<title>].svg`: the timestamp comes
  right after the prefix (so a directory sorts by time), then a random suffix, then the chart type and title
  when the spec carries them. If that name is somehow taken, another random suffix is tried, so an existing
  file is never overwritten.
- **A name you do supply overwrites** an existing file — that is the point of naming it, and it matches
  `COPY … TO`.

## Opening in a browser

`file.open: true` starts the system default browser on the file once it has been written (the write comes
first, so the file is always there when the browser looks at it). This is what makes a chart visible in one
statement:

```sql
SELECT kuva_render_file('{"file":{"dir":"/tmp/charts","open":true},
                          "series":[{"type":"line","data":[[0,1],[1,2],[2,1.5]]}]}') AS path;
```

A relative `file.dir` is resolved against the process's current directory, the same directory `std::fs`
writes into.

## Native builds only

A browser has no local file system to write to, so `kuva_render_file` **is not registered in a DuckDB-Wasm
build at all** — there is no such function to call there. On the web, use `kuva_render` and show the
returned SVG string in the page; the same JSON renders identically, only the destination differs.

## Notes

- The JSON is the same one `kuva_render` takes — including `figure`, so a multi-panel grid is written as a
  single SVG file.
- Errors behave as they do for `kuva_render`: they fail the statement rather than returning `NULL`, with a
  message that starts with `kuva_render_file: `.

## See also

- [Functions](../functions.md) — `kuva_render`, `kuva_render_terminal` and `kuva_render_file` side by side.
- [Terminal output](./terminal.md) — the other non-SVG backend.
- [kuva — Save / export](https://psy-fer.github.io/kuva/) — the library this extension wraps.
