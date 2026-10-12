# sdk/json_schema/

`kuva_render(json, data_json)` 两个参数的 JSON Schema，**程序生成，不手改**。

| 文件 | 描述的形状 | 真相源（Rust 类型） |
| --- | --- | --- |
| `render.schema.json` | 第一个参数：图表配置 | `src/extension/functions/spec/schema/`（根类型 `RenderSpec`） |
| `datasets.schema.json` | 第二个参数：数据集注册表 | `src/extension/functions/spec/dataset.rs`（`Registry`） |

JSON Schema draft 2020-12。字段说明（`description`）来自 Rust 源码里的 `///` 文档注释 ——
schemars 生成时自动带过去，所以**改注释就是改 schema 文档**，没有第二份需要同步的文字。

## 重新生成

```bash
just json_schema
# 等价于：cargo run --bin json_schema --features json-schema
```

生成器是 `src/bin/json_schema.rs`，只做一件事：把上面两个根类型交给 schemars 的
`schema_for!`，落盘到本目录。schemars 是**可选依赖**，只有 `json-schema` feature 打开时才编
（见 `Cargo.toml`），扩展本体与 wasm 产物不带它。

改了 `spec/schema/` 或 `spec/dataset.rs` 里的类型之后，重跑一遍命令，把 diff 一起提交。
想确认手里的产物没过期：重跑后 `git diff -- sdk/` 应为空。

## 用途

给宿主 SDK 生成类型化模型（各语言的 client / builder）：两个文件各自独立、自包含
（`$ref` 全是本文件内的 `#/$defs/…`）。schema 只覆盖**形状**（类型、必填、枚举名）；
长度一致、数值范围这类校验在 Rust 的 `convert` 层做，schema 不重复那份真相。

已知边界，都不是 bug：

- `dataset` / `encode` 两个键不出现在 schema 里 —— 它们在反序列化**之前**就被
  `dataset::bind` 消费掉（见 `spec/dataset.rs`），`RenderSpec` 看不到它们。
- `color_map` 等按名解析的字段是 `{"type": "string"}` 而不是枚举：真实实现容忍大小写、
  连字符与 ColorBrewer 缩写（描述里写明了），列枚举会把合法写法误判成非法。

内部标签枚举（`#[serde(tag = "type")]` 的 `SeriesSpec`，即 64 个图型）的判别字段**放在每个变体
自己的 `$def` 里**（`properties.type` 的 `const` + `required`），`oneOf` 成员是纯 `$ref`。这是
生成器做的一步归一化（`src/bin/json_schema.rs` 的 `normalize_internal_tags`）：schemars 原样输出的
`$ref` 带兄弟节点只有 2020-12 认，draft-07 校验器会整个忽略兄弟、连标签检查一起丢，而
json-schema-to-typescript 这类生成器只读兄弟、被引用的属性全丢。归一后新旧草案都认、校验语义
不变，`../ts/` 的判别联合也才能生成。
