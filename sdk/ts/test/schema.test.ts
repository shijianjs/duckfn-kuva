/**
 * 运行时测试（vitest 普通模式跑）：盯的是**被生成的那份 schema 文件本身**。
 *
 * 类型测试验证「schema -> TS」这一跳，这个文件验证 schema 这一跳没坏：
 * 结构自洽（$ref 都指向本文件内的 $defs）、内部标签是归一化后的形状（每个变体是裸 $ref，
 * 被引用的 $def 里有 const 标签且必填）—— 后者正是判别联合能生成的前提，归一化哪天退了回去，
 * 类型测试会跟着挂，但那时报的是「收窄不了」，这里报的是「schema 形状不对」，更好定位。
 *
 * Runtime test (plain vitest): watches the **generated schema file itself**.
 *
 * The type tests cover the schema -> TypeScript hop; this file covers the hop into the schema:
 * the structure is self-contained (every `$ref` points at a local `$def`) and internally tagged
 * variants are in the normalized shape (bare `$ref` members, const tag inside the referenced
 * `$def`, required). The latter is what makes the discriminated union possible — if the
 * normalization ever regresses, the type tests would fail too, but they would report "cannot
 * narrow", whereas this reports "schema shape is wrong", which is the better starting point.
 */

import { readFileSync } from 'node:fs';
import { expect, test } from 'vitest';

const schema = JSON.parse(
  readFileSync(new URL('../../json_schema/render.schema.json', import.meta.url), 'utf8'),
) as {
  $schema: string;
  $defs: Record<
    string,
    {
      oneOf?: { $ref?: string }[];
      properties?: Record<string, { const?: string }>;
      required?: string[];
    }
  >;
};

test('schema 声明 2020-12', () => {
  expect(schema.$schema).toBe('https://json-schema.org/draft/2020-12/schema');
});

test('所有 $ref 都指向本文件内的 $defs', () => {
  const refs: string[] = [];
  const walk = (node: unknown) => {
    if (Array.isArray(node)) {
      node.forEach(walk);
    } else if (node && typeof node === 'object') {
      for (const [key, value] of Object.entries(node)) {
        if (key === '$ref' && typeof value === 'string') {
          refs.push(value);
        } else {
          walk(value);
        }
      }
    }
  };
  walk(schema);

  const dangling = refs.filter((ref) => {
    const name = /^#\/\$defs\/(\w+)$/.exec(ref)?.[1];
    return !name || !(name in schema.$defs);
  });
  expect(dangling).toEqual([]);
  expect(refs.length).toBeGreaterThan(0);
});

test('SeriesSpec 的每个变体都是归一化后的形状', () => {
  const members = schema.$defs.SeriesSpec.oneOf ?? [];
  expect(members.length).toBeGreaterThan(0);

  const tags: string[] = [];
  for (const member of members) {
    // 归一化后成员只剩 $ref；带兄弟节点说明 normalize_internal_tags 没生效。
    expect(Object.keys(member)).toEqual(['$ref']);
    const name = /^#\/\$defs\/(\w+)$/.exec(member.$ref ?? '')?.[1];
    expect(name, `变体 $ref 无法解析: ${member.$ref}`).toBeTypeOf('string');

    const def = schema.$defs[name!];
    const tag = def.properties?.type?.const;
    expect(tag, `${name} 缺 type const 标签`).toBeTypeOf('string');
    expect(def.required, `${name} 的 required 里没有 type`).toContain('type');
    tags.push(tag!);
  }

  // type 标签必须互不相同，否则判别联合无法分派。
  expect(new Set(tags).size).toBe(tags.length);
});

test('datasets schema 是「数据集名 -> 行数组 -> 行对象」', () => {
  const datasets = JSON.parse(
    readFileSync(new URL('../../json_schema/datasets.schema.json', import.meta.url), 'utf8'),
  ) as {
    type: string;
    additionalProperties: { type: string; items: { type: string } };
  };

  expect(datasets.type).toBe('object');
  expect(datasets.additionalProperties.type).toBe('array');
  expect(datasets.additionalProperties.items.type).toBe('object');
});
