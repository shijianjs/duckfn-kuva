/**
 * 把 `sdk/json_schema/` 下的两份 JSON Schema 包成一份 OpenAPI 3.1 文档，交给 openapi-generator
 * 生成 Java 模型。**不是第二份真相**：内容全部来自 json_schema，只做两处归一化：
 *
 *  1. `$defs` -> `components/schemas`、`$ref` 前缀改写、给 `SeriesSpec` 补 `discriminator`
 *     （raw JSON Schema 没有这个关键字，而 openapi-generator 靠它做多态分发）。
 *  2. **内联联合成员提成具名 `$ref`**（`hoistInlineMembers`）：openapi-generator 对 `anyOf` 里的内联
 *     数组成员会拿「类型的字符串」当类名，生成 `List<String>.class` 这种非法 Java。把内联数组成员/对象
 *     成员提成命名 schema 再引用，就能绕过。名字取 `<父名>Member<下标>`，确定性、可预期。
 *
 * OpenAPI 3.1 的 schema 方言就是 JSON Schema 2020-12 的超集，所以没有语义损失。
 *
 * Wrap the two JSON Schemas under `sdk/json_schema/` into a single OpenAPI 3.1 document for
 * openapi-generator. Not a second source of truth; two normalisations only (see above). OpenAPI 3.1's
 * schema dialect is JSON Schema 2020-12 plus extensions, so nothing is lost.
 *
 * 用法 / Usage: `node tools/openapi.mjs`（由 pom 的 exec 调用），产物 `target/openapi.json`。
 */

import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname } from 'node:path';

const schemaDir = new URL('../../json_schema/', import.meta.url);
const readSchema = (name) => JSON.parse(readFileSync(new URL(name, schemaDir), 'utf8'));

const render = readSchema('render.schema.json');
const datasets = readSchema('datasets.schema.json');

/** `#/$defs/X` -> `#/components/schemas/X`（递归）。 */
function rewriteRefs(node) {
  if (Array.isArray(node)) return node.map(rewriteRefs);
  if (node && typeof node === 'object') {
    const out = {};
    for (const [k, v] of Object.entries(node)) {
      out[k] =
        k === '$ref' && typeof v === 'string' && v.startsWith('#/$defs/')
          ? '#/components/schemas/' + v.slice('#/$defs/'.length)
          : rewriteRefs(v);
    }
    return out;
  }
  return node;
}

/** 把一份 schema 的 `$defs` 与根类型都收进 components/schemas 命名空间。 */
function collect(schemas, schema, rootName) {
  for (const [name, def] of Object.entries(schema.$defs ?? {})) {
    if (name in schemas) throw new Error(`schema name collision: ${name}`);
    schemas[name] = rewriteRefs(def);
  }
  const { $schema, $id, title, $defs, ...root } = schema;
  if (rootName in schemas) throw new Error(`schema name collision: ${rootName}`);
  schemas[rootName] = rewriteRefs(root);
}

const schemas = {};
collect(schemas, render, 'RenderSpec');
collect(schemas, datasets, 'Registry');

/**
 * 递归把 `anyOf` / `oneOf` 里的内联数组成员、内联对象成员提成具名 schema，原位换成 `$ref`。
 * `${title}`、`$ref`、`{type:null}` 以及内联标量（number/string/boolean）保持不动。
 */
function hoistInlineMembers() {
  const queue = Object.keys(schemas);
  const used = new Set(Object.keys(schemas));

  const register = (preferred, node) => {
    let name = preferred;
    let i = 2;
    while (used.has(name)) name = `${preferred}${i++}`;
    used.add(name);
    schemas[name] = node;
    queue.push(name);
    return name;
  };

  const needsHoist = (m) =>
    m &&
    typeof m === 'object' &&
    !m.$ref && // 已经是引用
    m.type !== 'null' && // 可空成员保持内联
    (m.type === 'array' || m.type === 'object' || m.items || m.properties || m.enum);

  const transform = (node, ctx) => {
    if (Array.isArray(node)) return node.map((m) => transform(m, ctx));
    if (!node || typeof node !== 'object') return node;
    const out = {};
    for (const [k, v] of Object.entries(node)) {
      if ((k === 'anyOf' || k === 'oneOf') && Array.isArray(v)) {
        out[k] = v.map((m, i) => {
          if (needsHoist(m)) {
            const name = register(`${ctx}Member${i}`, m);
            return { $ref: `#/components/schemas/${name}` };
          }
          return transform(m, ctx);
        });
      } else if (k === 'properties' && v && typeof v === 'object') {
        out[k] = {};
        for (const [pk, pv] of Object.entries(v)) {
          const cap = pk.charAt(0).toUpperCase() + pk.slice(1);
          out[k][pk] = transform(pv, `${ctx}${cap}`);
        }
      } else {
        out[k] = transform(v, ctx);
      }
    }
    return out;
  };

  while (queue.length) {
    const name = queue.shift();
    schemas[name] = transform(schemas[name], name);
  }
}

hoistInlineMembers();

// 去掉 `default`：schemars 给带 `#[serde(default)]` 的字段写了初值（如 `data` 的 `[]`），openapi-generator
// 会照抄成 Java 字段的初值，于是「没设的字段」也会被序列化出去 —— `data: []` 与 `dataset` 冲突就是活例。
// 默认值由扩展在运行期决定，模型这边不该预填（TS 模型同样没有默认值）。
schemasStripDefaults();

/** 递归删掉 `default` 键。 */
function schemasStripDefaults() {
  const strip = (node) => {
    if (Array.isArray(node)) return node.map(strip);
    if (node && typeof node === 'object') {
      const out = {};
      for (const [k, v] of Object.entries(node)) {
        if (k === 'default') continue;
        out[k] = strip(v);
      }
      return out;
    }
    return node;
  };
  for (const name of Object.keys(schemas)) schemas[name] = strip(schemas[name]);
}

// `SeriesSpec` 的判别信息：每个 oneOf 成员的 `properties.type.const` 就是它的标签。
const seriesSpec = schemas.SeriesSpec;
if (seriesSpec && Array.isArray(seriesSpec.oneOf)) {
  const mapping = {};
  for (const member of seriesSpec.oneOf) {
    const ref = member?.$ref;
    if (typeof ref !== 'string') continue;
    const name = ref.slice(ref.lastIndexOf('/') + 1);
    const tag = schemas[name]?.properties?.type?.const;
    if (typeof tag === 'string') mapping[tag] = ref;
  }
  seriesSpec.discriminator = { propertyName: 'type', mapping };

  // 让每个图型变体都能带上扩展在反序列化**之前**消费的键（`dataset` / `encode`，见 spec/dataset.rs）：
  // 给它们开 `additionalProperties`，openapi-generator 就会生成 `@JsonAnySetter`，宿主用
  // `putAdditionalProperty(…)` 写入即可（TS 那边靠接口的索引签名达到同一目的）。
  for (const member of seriesSpec.oneOf) {
    const ref = member?.$ref;
    if (typeof ref !== 'string') continue;
    const target = schemas[ref.slice(ref.lastIndexOf('/') + 1)];
    if (target && target.type === 'object') target.additionalProperties = true;
  }
}

const doc = {
  openapi: '3.1.0',
  info: { title: 'duckfn-kuva', version: '0.1.1-dev.0' },
  paths: {},
  components: { schemas },
};

const out = new URL('../target/openapi.json', import.meta.url);
mkdirSync(dirname(fileURLToPath(out)), { recursive: true });
writeFileSync(out, JSON.stringify(doc, null, 2) + '\n');
console.log(`wrote ${Object.keys(schemas).length} schemas -> ${fileURLToPath(out)}`);
