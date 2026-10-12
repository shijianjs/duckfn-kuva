# sdk/ts/

`kuva_render(json, data_json)` 两个参数的 TypeScript 类型，供宿主 SDK 生成类型化模型。
两个文件都是**产物**，由 [json-schema-to-typescript](https://www.npmjs.com/package/json-schema-to-typescript)
从 `../json_schema/` 下的 JSON Schema 生成。

| 文件 | 内容 | 根类型 |
| --- | --- | --- |
| `render.ts` | 第一个参数（图表配置） | `RenderSpec` |
| `datasets.ts` | 第二个参数（数据集注册表） | `Registry` |

## 重新生成

```bash
npx -y json-schema-to-typescript@15 sdk/json_schema/render.schema.json   -o sdk/ts/render.ts
npx -y json-schema-to-typescript@15 sdk/json_schema/datasets.schema.json -o sdk/ts/datasets.ts
```

`../json_schema/` 那两个文件本身由 `just json_schema` 生成；改了 Rust 类型后按
schema → TS 的顺序各跑一遍。

## 选型：为什么是 json-schema-to-typescript 而不是 quicktype

quicktype 的 JSON Schema 模式**不把 `oneOf` 生成为判别联合**：64 个 series 变体被合并成单个
接口，`type` 退化成 64 个字面量的联合、各变体字段全部可选（`{type:"scatter"}` 缺 `data` 编译期
抓不到，`switch (s.type)` 也不收窄）。用最小用例验证过：纯 `$ref` 成员、内联 `const`、
OpenAPI 风格 `discriminator` 关键字三种写法都一样，改 schema 形状救不回来。

json-schema-to-typescript 保留 `oneOf` 为联合类型，配合下面的 schema 归一化就能得到真正的
判别联合：

```ts
export type SeriesSpec = ScatterSeries | LineSeries | …;   // 64 个变体
export interface ScatterSeries { …; type: "scatter"; }     // 字面量标签 + 必填 data
```

```ts
function label(s: SeriesSpec) {
  switch (s.type) {
    case "scatter": return s.data.length;   // 收窄到 ScatterSeries
    case "violin":  return s.groups?.length; // 收窄到 ViolinSeries
  }
}
```

未知图型、变体缺必填字段都是编译错误。

## 依赖的 schema 形状：内部标签归一化

判别联合能生成，靠的是 `just json_schema` 的一步后处理（`src/bin/json_schema.rs` 的
`normalize_internal_tags`）：schemars 原样输出的内部标签成员是
`{"properties": {"type": {"const": …}}, "$ref": "#/$defs/变体", "required": ["type"]}`，
即 `$ref` 带兄弟节点 —— 只有 JSON Schema 2020-12 认这种写法：draft-07 校验器会整个忽略兄弟
（连标签检查一起丢），json2ts 则只读兄弟、被引用的属性全丢。归一把 const 搬进被引用的 `$def`
（`properties` 与 `required` 各补一项），成员只剩纯 `$ref`：新旧草案都认、校验语义不变、
生成器都吃。所以 **TS 是从提交进仓库的那份 schema 直接生成的，没有私改**。

## 已知边界

- 每个接口都带 `[k: string]: unknown`：schema 没有设 `additionalProperties: false`，与运行时
  「未知键忽略」一致（`dataset` / `encode` 这类绑定前消费的键也能通过）。代价是拼错字段名
  编译期抓不到。
- `Option<T>` 生成 `field?: T | null`（比 Rust 宽一点：`undefined` 与显式 `null` 都收）。
- 命名取自 schema 的 `title`（`RenderSpec` / `Registry`，与 Rust 类型同名）；联合成员名取自
  第一个引用它的字段（如 `secondary_series` / `series` 共用的 `SeriesSpec`）。
