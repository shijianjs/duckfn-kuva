# sdk/ts/

`kuva_render(json, data_json)` 两个参数的 TypeScript 类型，给宿主 SDK 生成类型化模型用。
**除 `datasets.ts` 外都是产物**，由 quicktype 从 `../json_schema/` 下的 JSON Schema 生成。

| 文件 | 内容 | 来源 |
| --- | --- | --- |
| `render.ts` | 第一个参数（图表配置）的类型，根类型 `Render` | quicktype 生成 |
| `datasets.ts` | 第二个参数（数据集注册表）的类型 | 手写，原因见文件头注释 |

## 重新生成

```bash
npx -y quicktype -s schema -l ts --just-types -o sdk/ts/render.ts sdk/json_schema/render.schema.json
```

- `--just-types`：只要类型声明，不要 quicktype 的运行时转换代码（`Convert` 那套）—— SDK 侧的
  模型是用来构造 JSON 的，序列化交给 `JSON.stringify`。
- JSON Schema 改了之后重跑一遍；`../json_schema/` 那两个文件本身由 `just json_schema` 生成。

## quicktype 做到什么、不做什么

schemars 把 Rust 的 `///` 文档注释带成了 schema 里的 `description`，quicktype 再把它转成 JSDoc
挂在每个字段上 —— 注释链是通的。类型映射也对：`Option<T>` -> `field?: T | null`、单元枚举 ->
字符串字面量联合、`Vec<T>` -> `T[]`、点/三元组的 `prefixItems` -> 元组。schema 不设
`additionalProperties: false`，对应生成物里每个接口都带 `[property: string]: unknown` —— 这与
运行时的宽容度一致（未知键被忽略，`dataset` / `encode` 这类绑定前消费的键也能通过）。

两个已知限制，都是 quicktype 的 JSON Schema 模式本身的行为，不是 schema 的问题：

- **`oneOf` 不生成判别联合**。64 个 series 变体被合并成单个 `SecondarySeryElement` 接口，
  `type` 字段是全部 64 个字面量的联合，各变体的字段全部降级为可选。最小用例验证过：成员写成
  纯 `$ref`、内联 `const`、加 OpenAPI 风格 `discriminator` 关键字，三种写法都一样合并。
  后果：写 `{ type: "scatter" }` 缺 `data` 这类错误编译期抓不到，`switch (s.type)` 也不会收窄
  对象类型。要真正的判别联合得换生成器（如 json-schema-to-typescript），那是另一个决定。
- **顶层 map 类型不输出**。根对象只有 `additionalProperties` 时 quicktype 静默产出空文件
  （退出码 0），所以 `datasets.ts` 手写。

## 命名

根类型名取自 schema 的 `$id`（`render.schema.json` -> `Render`），不是 Rust 那边的
`RenderSpec`；union 成员名取自第一个用到它的字段名（`series` / `secondary_series` ->
`SecondarySeryElement`）。改名要么改 schema 的 `$id`，要么在生成后统一替换。
