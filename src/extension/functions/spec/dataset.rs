// ============================================================================
// 数据集参数：`kuva_render(json, data_json)` 的第二个参数
//
// 第一个参数（图表配置）描述「怎么画」，第二个参数（数据集注册表）只提供「画什么」。两者分开，
// 是因为配置常常由宿主 SDK（类似 echarts-java 那种）类型安全地构造再序列化成 JSON，而数据来自
// DuckDB 的表 —— 让 SQL 用 `to_json` / `array_agg` 直接产出数据，比把数据取回宿主再和配置拼成
// 一份嵌套 STRUCT 省事得多，也正好只用到 SQL 自己的过滤 / 转换 / 聚合能力。
//
// # 形状
//
// 注册表是一个 JSON 对象，键是**数据集名**，值是一个**行数组**：
//
// ```json
// { "sales": [ {"x": 1, "y": 2}, {"x": 3, "y": 4} ],
//   "temps": [ {"t": 0, "c": 20.5} ] }
// ```
//
// 每行是一个「字段名 -> 值」的对象；字段名就是 `encode` 里的名字。字段值可以是标量，也可以是
// 数组 / 对象（例如分组分布图型的一组观测值本身就是数组）。**行式**（数组）是刻意选的唯一形态：
// SQL 侧 `array_agg({'x': x, 'y': y})` 一句到位，且 STRUCT 的字段可以异型（DuckDB 的 `LIST`
// 是同型的，列式 `[[x, group], …]` 根本拼不出来）。
//
// # 关联
//
// 系列用 `dataset`（名字）挑数据集、用 `encode`（角色 -> 字段名）把数据集的列接到它自己的数据
// 字段上：
//
// ```json
// { "type": "scatter", "dataset": "sales", "encode": {"x": "revenue", "y": "profit"} }
// ```
//
// 规则：
//
//   - **一律用具名关联**，不出现 `0` / `1` / `2` 这样的下标（对标 ECharts 的具名数据集，但去掉
//     了 `datasetIndex` 这类下标用法）；
//   - **按名自动映射**：数据集的某个字段名恰好等于某个角色名时（例如 `x` / `y` / `value`），
//     该角色可以省略；`encode` 显式给出时覆盖它。不做位置推断（不会「第一列当 x」）；
//   - **只做选列**，不 `WHERE` / 不 `GROUP BY` / 不做数值聚合 —— 过滤、转换、聚合都交给 SQL；
//   - `dataset` 与同一字段的内联数据不能同时给（报错，而不是静默覆盖）。
//
// 绑定发生在**反序列化之前**：解析出配置的 `serde_json::Value`，把每个 series 的数据字段就地
// 填好，再走原来的强类型反序列化。所以图型的 schema / convert 一行都不用动，校验也照旧。
//
// ----------------------------------------------------------------------------
// The data argument: the second parameter of `kuva_render(json, data_json)`.
//
// The first argument (the chart config) says *how* to draw; the second (a dataset registry) only
// supplies *what* to draw. They are separate because the config is often built type-safely by a host
// SDK (an echarts-java-like one) and serialized to JSON, while the data lives in DuckDB tables — it is
// far easier to let SQL produce the data with `to_json` / `array_agg` than to pull it back into the
// host and splice it into one nested STRUCT, and it keeps filtering / transforming / aggregating in
// SQL, where it already lives.
//
// # Shape
//
// The registry is a JSON object whose keys are **dataset names** and whose values are **arrays of
// rows**: `{"sales": [{"x": 1, "y": 2}, …], "temps": [{"t": 0, "c": 20.5}]}`. Each row is a
// "field name -> value" object, and the field names are what `encode` refers to. A field value may be
// a scalar or an array/object (a grouped distribution chart's group values are themselves arrays).
// The **row form** (an array) is deliberately the only form: on the SQL side
// `array_agg({'x': x, 'y': y})` does it in one go, and STRUCT fields may be heterogeneous (DuckDB's
// `LIST` is homogeneous, so a columnar `[[x, group], …]` cannot even be built).
//
// # Association
//
// A series picks a dataset by name (`dataset`) and maps the dataset's columns onto its own data fields
// with `encode` (role -> field name). The rules: named association only (no `0`/`1`/`2` indices);
// **auto-map by name** when a dataset field is literally named like a role (an explicit `encode`
// overrides; there is no positional fallback); **column selection only** — no filtering, grouping or
// numeric aggregation, all of which stay in SQL; and a `dataset` may not be combined with inline data
// for the same field (that is an error, not a silent override).
//
// Binding happens **before deserialization**: the config's `serde_json::Value` is parsed, each
// series' data fields are filled in place, and the existing typed deserialization runs afterwards. So
// neither the per-chart schemas nor `convert` had to change, and validation is unchanged.
// ============================================================================

use std::collections::{BTreeSet, HashMap};

use serde_json::{Map, Value};

/// 数据集注册表：数据集名 -> 行；每行是一个「字段名 -> 值」的对象。
///
/// The dataset registry: dataset name -> rows; each row is a "field name -> value" object.
pub(crate) type Registry = HashMap<String, Vec<Map<String, Value>>>;

/// 解析第二个参数：一个「数据集名 -> 行数组」的 JSON 对象。
///
/// 刻意手写而不是直接 `serde_json::from_str::<Registry>`：这样每一条形状错误都能点名是哪个数据集、
/// 第几行，而不是一句笼统的 `invalid type: map, expected a sequence`。
///
/// Parse the second argument: a JSON object mapping dataset names to arrays of row objects.
///
/// Written by hand rather than `serde_json::from_str::<Registry>` so that every shape error can name
/// the dataset and the row instead of a bare `invalid type: map, expected a sequence`.
pub(crate) fn parse(json: &str) -> Result<Registry, String> {
    let value: Value =
        serde_json::from_str(json).map_err(|e| format!("invalid data JSON: {e}"))?;

    let Value::Object(map) = value else {
        return Err(
            "data: expected an object mapping dataset names to arrays of row objects".into(),
        );
    };

    let mut registry = HashMap::with_capacity(map.len());
    for (name, rows) in map {
        let Value::Array(rows) = rows else {
            return Err(format!(
                "data: dataset `{name}` must be an array of row objects (`[{{\"x\": 1}}, …]`)"
            ));
        };
        let mut parsed = Vec::with_capacity(rows.len());
        for (index, row) in rows.into_iter().enumerate() {
            let Value::Object(row) = row else {
                return Err(format!(
                    "data: dataset `{name}` row {index} must be an object of field -> value"
                ));
            };
            parsed.push(row);
        }
        registry.insert(name, parsed);
    }

    Ok(registry)
}

/// 把配置里每个 series 的数据字段按它的数据集填好；就地改写 `config`。
///
/// 递归遍历整份配置，所以顶层 `series` / `secondary_series` 与 `figure.panels[].series` 一视同仁。
/// 只在**同时**出现 `dataset` / `encode` 的对象上动手（那些就是 series）；其余对象继续往下走。
///
/// Fill in every series' data fields from its dataset, rewriting `config` in place.
///
/// The whole config is walked recursively, so a top-level `series` / `secondary_series` and a
/// `figure.panels[].series` are treated alike. Only objects carrying `dataset` / `encode` are touched
/// (those are the series); everything else is walked into.
pub(crate) fn bind(config: &mut Value, registry: &Registry) -> Result<(), String> {
    match config {
        Value::Object(map) => {
            if map.contains_key("dataset") || map.contains_key("encode") {
                bind_series(map, registry)
            } else {
                for value in map.values_mut() {
                    bind(value, registry)?;
                }
                Ok(())
            }
        }
        Value::Array(items) => {
            for item in items {
                bind(item, registry)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// 一个 series 的绑定：读 `dataset` / `encode`，把数据字段写回对象。
///
/// One series' binding: read `dataset` / `encode` and write the data fields back onto the object.
fn bind_series(series: &mut Map<String, Value>, registry: &Registry) -> Result<(), String> {
    let kind = match series.get("type") {
        Some(Value::String(kind)) => kind.clone(),
        _ => {
            return Err(
                "`dataset` is only valid on a series: put it next to the series' `type`".into(),
            )
        }
    };

    let Some(bindings) = bindings_for(&kind) else {
        return Err(format!(
            "{kind}: this chart type takes no `dataset` (its data has no per-row field mapping)"
        ));
    };

    let dataset = match series.get("dataset") {
        Some(Value::String(name)) => name.clone(),
        Some(_) => return Err(format!("{kind}: `dataset` must be a string naming a dataset")),
        None => {
            return Err(format!(
                "{kind}: `encode` needs `dataset` — the name of the dataset its fields come from"
            ))
        }
    };

    let Some(rows) = registry.get(&dataset) else {
        return Err(format!(
            "{kind}: unknown dataset `{dataset}` (the data argument defines: {})",
            join_sorted(registry.keys().map(String::as_str))
        ));
    };

    let encode = match series.get("encode") {
        Some(Value::Object(map)) => map.clone(),
        Some(_) => return Err(format!("{kind}: `encode` must be an object of role -> field name")),
        None => Map::new(),
    };

    let columns: BTreeSet<String> = rows.iter().flat_map(|row| row.keys().cloned()).collect();
    let roles: BTreeSet<&'static str> = bindings.iter().flat_map(|b| b.roles()).collect();

    for role in encode.keys() {
        if !roles.contains(role.as_str()) {
            return Err(format!(
                "{kind}: `encode.{role}` is not a role of this chart type (roles: {})",
                join_sorted(roles.iter().copied())
            ));
        }
    }

    // 同一个字段不能既有内联数据又由数据集填 —— 报错，不静默覆盖。
    //
    // The same field cannot be given inline and filled from a dataset — that is an error, not a
    // silent override.
    for binding in bindings {
        if series.contains_key(binding.field()) {
            return Err(format!(
                "{kind}: `dataset` fills `{}`, so `{}` cannot also be given inline",
                binding.field(),
                binding.field()
            ));
        }
    }

    let ctx = Ctx {
        kind: &kind,
        dataset: &dataset,
        columns: &columns,
        encode: &encode,
        rows,
    };

    let mut filled: Vec<&'static str> = Vec::new();
    for binding in bindings {
        if let Some(value) = binding_value(*binding, &ctx)? {
            series.insert(binding.field().to_string(), value);
            filled.push(binding.field());
        }
    }

    if filled.is_empty() {
        return Err(format!(
            "{kind}: dataset `{dataset}` (fields: {}) matched none of this chart type's roles ({}); map them with `encode`",
            join_sorted(columns.iter().map(String::as_str)),
            join_sorted(roles.iter().copied())
        ));
    }

    // 绑定完成，把这两个键去掉：schema 里没有它们，留着也只是未知键。
    //
    // Binding is done; drop these two keys — the schema has no place for them.
    series.remove("dataset");
    series.remove("encode");
    Ok(())
}

/// 一个图型的数据字段如何由数据集的行填出来。
///
/// 四种形态覆盖了全部 64 个图型的数据字段形状：一列、点列表、三维点列表、记录列表（外加 band 的
/// 上下界）。角色名 = 该形态要读的字段名，见 [`Binding::roles`]。
///
/// How one chart type's data fields are filled from a dataset's rows.
///
/// Four shapes cover the data fields of all 64 chart types — a column, a point list, a 3-D point list
/// and a record list (plus the band's two bounds). The roles are the field names that shape reads, see
/// [`Binding::roles`].
#[derive(Debug, Clone, Copy)]
enum Binding {
    /// 目标字段是一列：把数据集的某一列原样搬过去（`Vec<f64>` / `Vec<String>` / `Vec<Vec<f64>>` …）。
    ///
    /// The target field is a column: the dataset's column is moved over as it is.
    Column { field: &'static str, role: &'static str },
    /// 目标字段是二维点列表（`scatter` / `line` / `histogram2d` 的 `data`）：逐行 `x` / `y`。
    ///
    /// The target field is a 2-D point list: one point per row from `x` / `y`.
    Points { field: &'static str },
    /// 目标字段是三维点列表（`scatter3d` 的 `data`、`contour` 的 `points`）：逐行 `x` / `y` / `z`。
    ///
    /// The target field is a 3-D point list: one point per row from `x` / `y` / `z`.
    Points3 { field: &'static str },
    /// 目标字段是记录列表：逐行按这些子字段名组装一个对象（`slices` / `cells` / `edges` …）。
    ///
    /// The target field is a record list: one object per row, built from these sub-field names.
    Records { field: &'static str, fields: &'static [&'static str] },
    /// 目标字段是置信带（`{lower, upper}`）：两列。
    ///
    /// The target field is a band (`{lower, upper}`): two columns.
    Band { field: &'static str },
}

impl Binding {
    /// 这个绑定要写进 series 的哪个字段。
    ///
    /// Which series field this binding writes.
    fn field(self) -> &'static str {
        match self {
            Binding::Column { field, .. }
            | Binding::Points { field }
            | Binding::Points3 { field }
            | Binding::Records { field, .. }
            | Binding::Band { field } => field,
        }
    }

    /// 这个绑定接受的角色名（`encode` 的键、按名自动映射的候选）。
    ///
    /// The role names this binding accepts (the `encode` keys, and the auto-map candidates).
    fn roles(self) -> Vec<&'static str> {
        match self {
            Binding::Column { role, .. } => vec![role],
            Binding::Points { .. } => vec!["x", "y", "x_err", "y_err"],
            Binding::Points3 { .. } => vec!["x", "y", "z"],
            Binding::Records { fields, .. } => fields.to_vec(),
            Binding::Band { .. } => vec!["lower", "upper"],
        }
    }
}

/// 按类型名给出它的绑定表；`None` 表示这个图型不接受数据集（目前只有 `text`）。
///
/// The binding table for a type name; `None` means the type takes no dataset (only `text` today).
fn bindings_for(kind: &str) -> Option<&'static [Binding]> {
    use Binding::{Band, Column, Points, Points3, Records};

    let list: &'static [Binding] = match kind {
        // ---- 关系 / 相关 ----
        "scatter" => &[
            Points { field: "data" },
            Column { field: "sizes", role: "size" },
            Column { field: "colors", role: "color" },
            Column { field: "tooltip_labels", role: "label" },
            Band { field: "band" },
        ],
        "line" => &[Points { field: "data" }, Band { field: "band" }],
        "series" => &[Column { field: "values", role: "value" }],
        "band" => &[
            Column { field: "x", role: "x" },
            Column { field: "y_lower", role: "lower" },
            Column { field: "y_upper", role: "upper" },
        ],
        "histogram2d" => &[Points { field: "data" }],
        "contour" => &[
            Column { field: "z", role: "row" },
            Column { field: "x_coords", role: "x_coord" },
            Column { field: "y_coords", role: "y_coord" },
            Points3 { field: "points" },
        ],
        "parallel" => &[
            Column { field: "axis_names", role: "axis" },
            Records { field: "rows", fields: &["values", "group"] },
        ],
        "polar" => &[Records { field: "series", fields: &["r", "theta", "label", "color"] }],
        "ternary" => &[Records { field: "points", fields: &["a", "b", "c", "group"] }],
        "quiver" => &[Records { field: "arrows", fields: &["x", "y", "u", "v", "color"] }],
        "jointplot" => &[Records {
            field: "groups",
            fields: &["x", "y", "label", "color", "sizes", "colors", "x_err", "y_err"],
        }],
        // ---- 分类 / 比较 ----
        "bar" => &[
            Column { field: "categories", role: "name" },
            Column { field: "values", role: "value" },
            Column { field: "errors", role: "error" },
        ],
        "pie" => &[Records { field: "slices", fields: &["label", "value", "color"] }],
        "waffle" => &[Records { field: "categories", fields: &["label", "value", "color"] }],
        "funnel" => &[Records { field: "stages", fields: &["label", "value", "color"] }],
        "pareto" => &[Records { field: "categories", fields: &["label", "value"] }],
        "pyramid" => &[Records { field: "series", fields: &["label", "groups"] }],
        "lollipop" => &[Records { field: "points", fields: &["x", "y", "label", "color"] }],
        "slope" => &[Records { field: "points", fields: &["label", "before", "after"] }],
        "dot_plot" => &[Records { field: "points", fields: &["x", "y", "size", "color"] }],
        "dice_plot" => &[Records { field: "records", fields: &["x", "y", "dot", "fill", "size"] }],
        "venn" => &[Records { field: "sets", fields: &["label", "size", "elements"] }],
        "mosaic" => &[Records { field: "cells", fields: &["col", "row", "value"] }],
        "rose" => &[Records { field: "slices", fields: &["label", "value", "color"] }],
        "radar" => &[
            Column { field: "axes", role: "axis" },
            Records { field: "series", fields: &["values", "label", "color", "errors"] },
        ],
        "upset" => &[
            Column { field: "set_names", role: "set_name" },
            Column { field: "set_sizes", role: "set_size" },
            Records { field: "intersections", fields: &["mask", "count"] },
        ],
        // ---- 分布 ----
        "histogram" => &[
            Column { field: "values", role: "value" },
            Column { field: "edges", role: "edge" },
            Column { field: "counts", role: "count" },
        ],
        "density" => &[Column { field: "values", role: "value" }],
        "ridgeline" => &[Records { field: "groups", fields: &["label", "values", "color"] }],
        "ecdf" => &[Records { field: "groups", fields: &["label", "values", "color"] }],
        "qq" => &[Records { field: "groups", fields: &["label", "values", "color"] }],
        "box" => &[Records { field: "groups", fields: &["label", "values", "color"] }],
        "violin" => &[Records { field: "groups", fields: &["label", "values", "color"] }],
        "strip" => &[Records {
            field: "groups",
            fields: &["label", "values", "point_colors", "point_shapes", "kind", "jitter"],
        }],
        "raincloud" => &[Records { field: "groups", fields: &["label", "values", "color"] }],
        "hexbin" => &[
            Column { field: "x", role: "x" },
            Column { field: "y", role: "y" },
            Column { field: "z", role: "z" },
        ],
        "heatmap" => &[
            Column { field: "data", role: "row" },
            Column { field: "row_labels", role: "row_label" },
            Column { field: "col_labels", role: "col_label" },
        ],
        // ---- 统计 / 模型评估 ----
        "roc" => &[Records { field: "groups", fields: &["label", "points", "color"] }],
        "pr" => &[Records { field: "groups", fields: &["label", "points", "prevalence", "color"] }],
        "survival" => &[Records { field: "groups", fields: &["label", "times", "events", "color"] }],
        "forest" => &[Records {
            field: "rows",
            fields: &["label", "estimate", "ci_lower", "ci_upper", "weight", "color"],
        }],
        "volcano" => &[Records { field: "points", fields: &["name", "log2fc", "pvalue"] }],
        "manhattan" => &[Records {
            field: "points",
            fields: &["chromosome", "position", "pvalue", "label"],
        }],
        // ---- 层级 / 网络 ----
        "treemap" => &[
            Records { field: "roots", fields: &["label", "value", "color"] },
            Column { field: "color_values", role: "color_value" },
        ],
        "sunburst" => &[
            Records { field: "roots", fields: &["label", "value", "color"] },
            Column { field: "color_values", role: "color_value" },
        ],
        // 节点由 kuva 按 `source` / `target` 自动建出来，所以只映射边。
        "network" => &[Records {
            field: "edges",
            fields: &["source", "target", "weight", "color", "label", "curve"],
        }],
        "sankey" => &[Records {
            field: "links",
            fields: &["source", "target", "value", "color"],
        }],
        "chord" => &[
            Column { field: "matrix", role: "row" },
            Column { field: "labels", role: "label" },
        ],
        "phylo" => &[Records { field: "edges", fields: &["parent", "child", "length"] }],
        "clustermap" => &[
            Column { field: "data", role: "row" },
            Column { field: "row_labels", role: "row_label" },
            Column { field: "col_labels", role: "col_label" },
        ],
        // ---- 3D ----
        "scatter3d" => &[
            Points3 { field: "data" },
            Column { field: "sizes", role: "size" },
            Column { field: "colors", role: "color" },
        ],
        "surface3d" => &[
            Column { field: "z_data", role: "row" },
            Column { field: "x_coords", role: "x_coord" },
            Column { field: "y_coords", role: "y_coord" },
        ],
        // ---- 时间 / 金融 / 排名 ----
        "stacked_area" => &[
            Column { field: "x", role: "x" },
            Records { field: "series", fields: &["values", "label", "color"] },
        ],
        "streamgraph" => &[
            Column { field: "x", role: "x" },
            Records { field: "series", fields: &["values", "label", "color"] },
        ],
        "candlestick" => &[Records {
            field: "candles",
            fields: &["label", "x", "open", "high", "low", "close", "volume"],
        }],
        "waterfall" => &[Records {
            field: "bars",
            fields: &["label", "value", "from", "to", "kind"],
        }],
        "horizon" => &[Records {
            field: "series",
            fields: &["label", "x", "y", "pos_color", "neg_color"],
        }],
        "calendar" => &[
            Records { field: "data", fields: &["date", "value"] },
            Column { field: "events", role: "event" },
        ],
        "gantt" => &[Records {
            field: "tasks",
            fields: &["label", "start", "end", "group", "progress", "color", "milestone"],
        }],
        "bump" => &[
            Records { field: "series", fields: &["name", "ranks", "values", "color"] },
            Column { field: "x_labels", role: "x_label" },
        ],
        // ---- 复合 / 工具 ----
        "brick" => &[
            Column { field: "names", role: "name" },
            Column { field: "sequences", role: "sequence" },
            Column { field: "notations", role: "notation" },
            Column { field: "start_positions", role: "start_position" },
        ],
        "legend_plot" => &[Records {
            field: "entries",
            fields: &["label", "color", "shape", "dasharray"],
        }],
        // `text` 是一整段文字，没有逐行字段；`synteny` 的两份记录（序列与区块）共用字段名、一份
        // 数据集无法同时表达，两者都不接受 `dataset`。
        //
        // `text` is one block of prose with no per-row fields, and `synteny`'s two record lists
        // (sequences and blocks) share field names and cannot both come from one dataset; neither
        // takes a `dataset`.
        _ => return None,
    };

    Some(list)
}

/// 绑定的上下文：把「按角色取字段、取列、取数」这几件事收在一处，省得每个 helper 都带一串参数。
///
/// The binding context: keeps "resolve a role to a field, read a column, read a number" in one place
/// so the helpers do not each carry a parameter list.
struct Ctx<'a> {
    kind: &'a str,
    dataset: &'a str,
    columns: &'a BTreeSet<String>,
    encode: &'a Map<String, Value>,
    rows: &'a [Map<String, Value>],
}

impl Ctx<'_> {
    /// 角色 -> 数据集字段名：显式 `encode` 优先，否则数据集里的同名字段；都没有就是 `None`。
    ///
    /// Role -> dataset field name: an explicit `encode` wins, then a same-named dataset field; `None`
    /// when neither exists.
    fn field(&self, role: &str) -> Result<Option<String>, String> {
        if let Some(value) = self.encode.get(role) {
            let name = value.as_str().ok_or_else(|| {
                format!("{}: `encode.{role}` must be a field name (a string)", self.kind)
            })?;
            if !self.columns.contains(name) {
                return Err(format!(
                    "{}: `encode.{role}` refers to `{name}`, which dataset `{}` does not have (fields: {})",
                    self.kind,
                    self.dataset,
                    join_sorted(self.columns.iter().map(String::as_str))
                ));
            }
            return Ok(Some(name.to_string()));
        }

        Ok(self.columns.contains(role).then(|| role.to_string()))
    }

    /// 取一列（缺失的行补 `null`，让反序列化按目标类型去判）。
    ///
    /// Read one column (a missing cell becomes `null`, leaving the judgement to deserialization).
    fn column(&self, field: &str) -> Value {
        Value::Array(
            self.rows
                .iter()
                .map(|row| row.get(field).cloned().unwrap_or(Value::Null))
                .collect(),
        )
    }

    /// 取一个数值：JSON number，或能解析成数的字符串；其余一律报错并点明数据集 / 行 / 字段。
    ///
    /// Read one number: a JSON number, or a string that parses as one; anything else is an error
    /// naming the dataset, the row and the field.
    fn number(&self, field: &str, index: usize) -> Result<Value, String> {
        match self.rows[index].get(field) {
            Some(Value::Number(n)) => Ok(Value::Number(n.clone())),
            Some(Value::String(s)) => s
                .parse::<f64>()
                .ok()
                .and_then(serde_json::Number::from_f64)
                .map(Value::Number)
                .ok_or_else(|| {
                    format!(
                        "{}: dataset `{}` row {index} field `{field}` is not a number: {s:?}",
                        self.kind, self.dataset
                    )
                }),
            other => Err(format!(
                "{}: dataset `{}` row {index} field `{field}` must be a number, got {}",
                self.kind,
                self.dataset,
                describe(other)
            )),
        }
    }
}

/// 按绑定形态取值；`Ok(None)` 表示这个绑定的角色一个都没解析出来（跳过它）。
///
/// Evaluate one binding; `Ok(None)` means none of its roles resolved (so it is skipped).
fn binding_value(binding: Binding, ctx: &Ctx) -> Result<Option<Value>, String> {
    match binding {
        Binding::Column { role, .. } => {
            let Some(field) = ctx.field(role)? else {
                return Ok(None);
            };
            Ok(Some(ctx.column(&field)))
        }
        Binding::Points { .. } => points(ctx, false),
        Binding::Points3 { .. } => points(ctx, true),
        Binding::Records { fields, .. } => records(ctx, fields),
        Binding::Band { .. } => band(ctx),
    }
}

/// 点列表：二维读 `x` / `y`（可带 `x_err` / `y_err`），三维读 `x` / `y` / `z`。
///
/// A point list: 2-D reads `x` / `y` (with optional `x_err` / `y_err`), 3-D reads `x` / `y` / `z`.
fn points(ctx: &Ctx, three_d: bool) -> Result<Option<Value>, String> {
    let x = ctx.field("x")?;
    let y = ctx.field("y")?;
    let z = if three_d { ctx.field("z")? } else { None };
    let x_err = if three_d { None } else { ctx.field("x_err")? };
    let y_err = if three_d { None } else { ctx.field("y_err")? };

    // 一个都没给 -> 这个绑定不适用；给了一半 -> 报错，别让用户对着「匹配不到角色」发懵。
    //
    // None given -> not applicable; half given -> an error, rather than a puzzling "matched no role".
    if x.is_none() && y.is_none() && z.is_none() && x_err.is_none() && y_err.is_none() {
        return Ok(None);
    }
    let (Some(x), Some(y)) = (x, y) else {
        return Err(format!("{}: a point list needs both `x` and `y`", ctx.kind));
    };
    if three_d && z.is_none() {
        return Err(format!("{}: a 3-D point list needs `z` as well", ctx.kind));
    }

    let mut out = Vec::with_capacity(ctx.rows.len());
    for index in 0..ctx.rows.len() {
        let xv = ctx.number(&x, index)?;
        let yv = ctx.number(&y, index)?;

        if let Some(z) = &z {
            out.push(Value::Array(vec![xv, yv, ctx.number(z, index)?]));
        } else if x_err.is_none() && y_err.is_none() {
            // 没有误差就写成 `[x, y]`，与内联数据的短写法一致。
            out.push(Value::Array(vec![xv, yv]));
        } else {
            let mut point = Map::new();
            point.insert("x".into(), xv);
            point.insert("y".into(), yv);
            insert_cell(&mut point, "x_err", ctx, x_err.as_deref(), index);
            insert_cell(&mut point, "y_err", ctx, y_err.as_deref(), index);
            out.push(Value::Object(point));
        }
    }

    Ok(Some(Value::Array(out)))
}

/// 记录列表：逐行按子字段名组装一个对象；一个子字段都没解析出来就跳过。
///
/// A record list: one object per row built from the sub-field names; skipped when none of them
/// resolved.
fn records(ctx: &Ctx, fields: &[&'static str]) -> Result<Option<Value>, String> {
    let mut resolved: Vec<(&'static str, String)> = Vec::new();
    for field in fields {
        if let Some(source) = ctx.field(field)? {
            resolved.push((field, source));
        }
    }
    if resolved.is_empty() {
        return Ok(None);
    }

    let mut out = Vec::with_capacity(ctx.rows.len());
    for row in ctx.rows {
        let mut record = Map::new();
        for (target, source) in &resolved {
            if let Some(value) = row.get(source) {
                if !value.is_null() {
                    record.insert((*target).to_string(), value.clone());
                }
            }
        }
        out.push(Value::Object(record));
    }

    Ok(Some(Value::Array(out)))
}

/// 置信带：`{lower, upper}` 两个等长（或不等长，交给图型自己校验）的数组。
///
/// A confidence band: a `{lower, upper}` object of two arrays (equal length, or the chart type will
/// say otherwise).
fn band(ctx: &Ctx) -> Result<Option<Value>, String> {
    let lower = ctx.field("lower")?;
    let upper = ctx.field("upper")?;

    match (lower, upper) {
        (Some(lower), Some(upper)) => {
            let mut band = Map::new();
            band.insert("lower".into(), ctx.column(&lower));
            band.insert("upper".into(), ctx.column(&upper));
            Ok(Some(Value::Object(band)))
        }
        // 两边都没给 -> 这个绑定不适用；只给了一边 -> 报错。
        //
        // Neither given -> not applicable; only one given -> an error.
        (None, None) => Ok(None),
        _ => Err(format!("{}: a band needs both `lower` and `upper`", ctx.kind)),
    }
}

/// 把可选的逐点字段塞进点对象：字段没给、或这一行是 `null` 时都不写（即「这一项没给」）。
///
/// Put an optional per-point field into the point object: nothing is written when the field is absent
/// or the cell is `null` (i.e. "this item was not given").
fn insert_cell(
    point: &mut Map<String, Value>,
    target: &str,
    ctx: &Ctx,
    field: Option<&str>,
    index: usize,
) {
    if let Some(field) = field {
        if let Some(value) = ctx.rows[index].get(field) {
            if !value.is_null() {
                point.insert(target.to_string(), value.clone());
            }
        }
    }
}

/// 排序后拼成 `a, b, c`，让错误信息与测试断言稳定。
///
/// Sort and join into `a, b, c` so error messages and test assertions stay stable.
fn join_sorted<'a>(items: impl IntoIterator<Item = &'a str>) -> String {
    let mut items: Vec<&str> = items.into_iter().collect();
    items.sort_unstable();
    items.join(", ")
}

/// 把 JSON 值说成人话（错误信息里用）。
///
/// Describe a JSON value in words (for error messages).
fn describe(value: Option<&Value>) -> &'static str {
    match value {
        None | Some(Value::Null) => "null",
        Some(Value::Bool(_)) => "a boolean",
        Some(Value::Number(_)) => "a number",
        Some(Value::String(_)) => "a string",
        Some(Value::Array(_)) => "an array",
        Some(Value::Object(_)) => "an object",
    }
}

#[cfg(test)]
mod tests {
    use super::{bind, parse};

    /// 把注册表绑到配置上，成功时返回改写后的配置。
    ///
    /// Bind the registry onto the config, returning the rewritten config on success.
    fn bind_json(config: &str, data: &str) -> Result<serde_json::Value, String> {
        let registry = parse(data)?;
        let mut value: serde_json::Value = serde_json::from_str(config).expect("valid config");
        bind(&mut value, &registry)?;
        Ok(value)
    }

    #[test]
    fn explicit_encode_fills_the_point_list() {
        let out = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"d","encode":{"x":"a","y":"b"}}]}"#,
            r#"{"d":[{"a":1,"b":2},{"a":3,"b":4}]}"#,
        )
        .expect("bind");

        assert_eq!(out["series"][0]["data"], serde_json::json!([[1, 2], [3, 4]]));
        assert!(out["series"][0].get("dataset").is_none());
        assert!(out["series"][0].get("encode").is_none());
    }

    #[test]
    fn roles_are_auto_mapped_by_field_name() {
        let out = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"d"}]}"#,
            r#"{"d":[{"x":1,"y":2},{"x":3,"y":4}]}"#,
        )
        .expect("bind");

        assert_eq!(out["series"][0]["data"], serde_json::json!([[1, 2], [3, 4]]));
    }

    #[test]
    fn per_point_errors_switch_the_point_to_the_long_form() {
        let out = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"d","encode":{"x":"a","y":"b","y_err":"e"}}]}"#,
            r#"{"d":[{"a":1,"b":2,"e":0.5}]}"#,
        )
        .expect("bind");

        assert_eq!(
            out["series"][0]["data"],
            serde_json::json!([{"x": 1, "y": 2, "y_err": 0.5}])
        );
    }

    #[test]
    fn records_build_one_element_per_row() {
        let out = bind_json(
            r#"{"series":[{"type":"mosaic","dataset":"d"}]}"#,
            r#"{"d":[{"col":"a","row":"x","value":1},{"col":"b","row":"x","value":2}]}"#,
        )
        .expect("bind");

        assert_eq!(
            out["series"][0]["cells"],
            serde_json::json!([
                {"col": "a", "row": "x", "value": 1},
                {"col": "b", "row": "x", "value": 2}
            ])
        );
    }

    #[test]
    fn a_column_is_moved_over_as_it_is() {
        let out = bind_json(
            r#"{"series":[{"type":"histogram","dataset":"d","encode":{"value":"v"}}]}"#,
            r#"{"d":[{"v":1},{"v":2},{"v":2}]}"#,
        )
        .expect("bind");

        assert_eq!(out["series"][0]["values"], serde_json::json!([1, 2, 2]));
    }

    #[test]
    fn a_band_is_built_from_two_columns() {
        let out = bind_json(
            r#"{"series":[{"type":"line","dataset":"d","encode":{"x":"t","y":"v","lower":"lo","upper":"hi"}}]}"#,
            r#"{"d":[{"t":0,"v":1,"lo":0.8,"hi":1.3}]}"#,
        )
        .expect("bind");

        assert_eq!(
            out["series"][0]["band"],
            serde_json::json!({"lower": [0.8], "upper": [1.3]})
        );
    }

    #[test]
    fn panels_are_bound_too() {
        let out = bind_json(
            r#"{"figure":{"rows":1,"cols":1,"panels":[
                 {"series":[{"type":"pie","dataset":"d","encode":{"label":"k","value":"v"}}]}]}}"#,
            r#"{"d":[{"k":"a","v":3}]}"#,
        )
        .expect("bind");

        assert_eq!(
            out["figure"]["panels"][0]["series"][0]["slices"],
            serde_json::json!([{"label": "a", "value": 3}])
        );
    }

    #[test]
    fn an_unknown_dataset_is_reported_with_the_available_names() {
        let err = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"nope"}]}"#,
            r#"{"d":[{"x":1,"y":2}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("unknown dataset `nope`"), "unexpected: {err}");
        assert!(err.contains('d'), "the message should list the known names: {err}");
    }

    #[test]
    fn an_unknown_encode_role_is_reported_with_the_valid_roles() {
        let err = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"d","encode":{"z":"a"}}]}"#,
            r#"{"d":[{"x":1,"y":2}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("`encode.z`"), "unexpected: {err}");
        assert!(err.contains("x_err"), "the message should list the roles: {err}");
    }

    #[test]
    fn an_encode_field_missing_from_the_dataset_is_reported() {
        let err = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"d","encode":{"x":"nope","y":"y"}}]}"#,
            r#"{"d":[{"x":1,"y":2}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("`encode.x` refers to `nope`"), "unexpected: {err}");
    }

    #[test]
    fn inline_data_beside_a_dataset_is_reported() {
        let err = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"d","data":[[1,2]]}]}"#,
            r#"{"d":[{"x":1,"y":2}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("cannot also be given inline"), "unexpected: {err}");
    }

    #[test]
    fn a_dataset_on_a_type_without_one_is_reported() {
        let err = bind_json(
            r#"{"series":[{"type":"text","dataset":"d"}]}"#,
            r#"{"d":[{"x":1}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("takes no `dataset`"), "unexpected: {err}");
    }

    #[test]
    fn a_dataset_off_a_series_is_reported() {
        let err = bind_json(r#"{"dataset":"d"}"#, r#"{"d":[{"x":1}]}"#).unwrap_err();
        assert!(err.contains("only valid on a series"), "unexpected: {err}");
    }

    #[test]
    fn a_non_numeric_cell_is_reported_with_its_row() {
        let err = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"d","encode":{"x":"a","y":"b"}}]}"#,
            r#"{"d":[{"a":1,"b":2},{"a":"oops","b":4}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("row 1"), "unexpected: {err}");
        assert!(err.contains("field `a`"), "unexpected: {err}");
    }

    #[test]
    fn numeric_strings_are_accepted_for_coordinates() {
        let out = bind_json(
            r#"{"series":[{"type":"scatter","dataset":"d","encode":{"x":"a","y":"b"}}]}"#,
            r#"{"d":[{"a":"1.5","b":"2"}]}"#,
        )
        .expect("bind");
        assert_eq!(out["series"][0]["data"], serde_json::json!([[1.5, 2.0]]));
    }

    #[test]
    fn the_row_form_is_the_only_accepted_shape() {
        let err = parse(r#"{"d":{"x":[1,2]}}"#).unwrap_err();
        assert!(err.contains("must be an array of row objects"), "unexpected: {err}");

        let err = parse(r#"{"d":[[1,2]]}"#).unwrap_err();
        assert!(err.contains("row 0 must be an object"), "unexpected: {err}");

        let err = parse("[1,2]").unwrap_err();
        assert!(err.contains("expected an object"), "unexpected: {err}");
    }

    #[test]
    fn every_registered_type_has_a_binding_table_or_is_explicitly_unsupported() {
        // 与 `convert::charts` 的图型清单保持一致：除 `text` 外都要有绑定表。
        const TYPES: &[&str] = &[
            "scatter", "line", "bar", "histogram", "box", "pie", "violin", "ridgeline", "raincloud",
            "strip", "dot_plot", "lollipop", "density", "ecdf", "qq", "forest", "pr", "roc",
            "survival", "volcano", "heatmap", "histogram2d", "hexbin", "clustermap", "contour",
            "ternary", "polar", "dice_plot", "scatter3d", "surface3d", "sankey", "chord", "network",
            "treemap", "sunburst", "venn", "upset", "waffle", "mosaic", "phylo", "synteny",
            "candlestick", "calendar", "gantt", "horizon", "manhattan", "waterfall", "bump",
            "pareto", "brick", "funnel", "slope", "pyramid", "series", "radar", "parallel",
            "stacked_area", "streamgraph", "band", "text", "legend_plot", "quiver", "jointplot",
            "rose",
        ];

        for kind in TYPES {
            let supported = super::bindings_for(kind).is_some();
            let expected = !matches!(*kind, "text" | "synteny");
            assert_eq!(supported, expected, "unexpected dataset support for `{kind}`");
        }
    }
}
