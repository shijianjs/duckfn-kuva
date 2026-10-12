/**
 * `kuva_render(json, data_json)` **第二个参数**（数据集注册表）的形状。
 *
 * 手写，不是生成的 —— 原因：quicktype 的 JSON Schema 模式对「根对象只带
 * `additionalProperties`、没有任何具名 `properties`」的 schema **什么都不输出**，而且是静默的
 * （退出码 0，不报错）。用最小用例复现过：根改成 `$ref` 指向 `$defs` 里的具名类型也一样。
 * 这个形状只有三行，所以直接写在这里；真相源仍然是 `src/extension/functions/spec/dataset.rs`
 * 的 `Registry`（见 `../json_schema/README.md`）。
 *
 * The shape of `kuva_render(json, data_json)`'s **second argument** (the dataset registry).
 *
 * Hand-written, not generated: quicktype's JSON Schema mode emits **nothing** for a root schema
 * that carries only `additionalProperties` and no named `properties` — silently, with exit code 0.
 * Reproduced with a minimal case; pointing the root at a named `$defs` entry via `$ref` does not
 * help either. The shape is three lines, so it is written out here; the source of truth stays
 * `Registry` in `src/extension/functions/spec/dataset.rs` (see `../json_schema/README.md`).
 */

/** 数据集注册表：数据集名 -> 行数组。 */
export type Registry = { [name: string]: Row[] };

/** 一行数据：「字段名 -> 值」。值是任意的 JSON 值（标量、数组或对象）。 */
export type Row = { [field: string]: unknown };
