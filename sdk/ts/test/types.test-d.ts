/**
 * 类型测试（vitest --typecheck 跑）：这个包是纯类型包，运行期什么都没有，所以「测试」就是
 * 断言生成出来的类型符合预期。
 *
 * 覆盖三件事：
 *   1. 判别联合真的能收窄 —— 64 个图型按 `type` 收到各自变体，变体自己的字段可见；
 *   2. 必填 / 可选 / `| null` 的形状符合 Rust 侧（`Option<T>` -> `field?: T | null`）；
 *   3. 该报错的确实报错 —— `@ts-expect-error` 那几行，哪条变成 unused 就是生成物退化了的信号
 *      （vitest 的 typecheck 会把 unused directive 算成失败）。
 *
 * Type tests (run by `vitest --typecheck`): this package is types only and has no runtime code,
 * so "testing" means asserting the generated types are what they should be.
 *
 * Three things are covered: (1) the discriminated union really narrows — each of the 64 chart
 * types narrows on `type` to its own variant with its own fields visible; (2) required / optional
 * / `| null` shapes match the Rust side (`Option<T>` -> `field?: T | null`); (3) the things that
 * must fail really do fail — the `@ts-expect-error` lines turn into vitest typecheck failures the
 * moment one of them becomes an unused directive, which is the signal that the generated types
 * have regressed.
 */

import { expectTypeOf, test } from 'vitest';

import type {
  PhyloSeries,
  PointSpec,
  RenderSpec,
  Registry,
  ScatterSeries,
  SeriesSpec,
  TitleFull,
  TitleSpec,
  ViolinSeries,
} from '../src/index';

test('RenderSpec 的顶层字段形状', () => {
  expectTypeOf<RenderSpec['series']>().toEqualTypeOf<SeriesSpec[] | undefined>();
  expectTypeOf<RenderSpec['secondary_series']>().toEqualTypeOf<SeriesSpec[] | undefined>();
  // Option<f64> -> number | null，再加字段可选本身带的 undefined。
  expectTypeOf<RenderSpec['width']>().toEqualTypeOf<number | null | undefined>();
  // TitleSpec 是「字符串 | 对象」的 untagged 联合，再套 Option。
  expectTypeOf<RenderSpec['title']>().toEqualTypeOf<TitleSpec | null | undefined>();
  expectTypeOf<TitleSpec>().toEqualTypeOf<string | TitleFull>();
});

test('判别联合按 type 收窄', () => {
  const s: SeriesSpec = { type: 'scatter', data: [{ x: 1, y: 2 }], legend: 'a' };

  if (s.type === 'scatter') {
    expectTypeOf(s).toEqualTypeOf<ScatterSeries>();
    // data 现在是可选的（dataset 形式由 Rust 端注入），所以是 PointSpec[] | undefined。
    // `data` is now optional (the `dataset` form has it injected by the Rust side), hence `| undefined`.
    expectTypeOf(s.data).toEqualTypeOf<PointSpec[] | undefined>();
    expectTypeOf(s.legend).toEqualTypeOf<string | null | undefined>();
  }

  const v: SeriesSpec = { type: 'violin', groups: [{ label: 'g', values: [1, 2, 3] }] };
  if (v.type === 'violin') {
    expectTypeOf(v).toEqualTypeOf<ViolinSeries>();
    // 各图型共有的样式字段来自 Rust 的 CommonStyle（flatten 进每个变体）。
    expectTypeOf(v.color).toEqualTypeOf<string | null | undefined>();
  }

  const p: SeriesSpec = { type: 'phylo', newick: '((A:0.1,B:0.2):0.3,C:0.4);' };
  if (p.type === 'phylo') {
    expectTypeOf(p).toEqualTypeOf<PhyloSeries>();
    expectTypeOf(p.newick).toEqualTypeOf<string | null | undefined>();
  }
});

test('switch 收窄后 default 分支拿到剩下的 type', () => {
  // 注意：联合必须经由函数参数进来。写成 `const s: SeriesSpec = {type:'scatter',…}` 的话，
  // 控制流分析会按初始值把 s 窄化成 ScatterSeries，后面的分支全是 never。
  //
  // Note: the union has to arrive through a parameter. `const s: SeriesSpec = {type:'scatter',…}`
  // lets control-flow analysis narrow s to ScatterSeries from the initializer, and every later
  // branch becomes never.
  const describe = (s: SeriesSpec): void => {
    switch (s.type) {
      case 'scatter':
        expectTypeOf(s).toEqualTypeOf<ScatterSeries>();
        break;
      case 'violin':
        expectTypeOf(s).toEqualTypeOf<ViolinSeries>();
        break;
      default:
        // 剩下 62 个字面量的联合（收窄掉 scatter / violin 之后）。
        expectTypeOf(s.type).toEqualTypeOf<Exclude<SeriesSpec['type'], 'scatter' | 'violin'>>();
    }
  };

  describe({ type: 'scatter', data: [] });
  describe({ type: 'violin', groups: [] });
});

test('数据集注册表的形状', () => {
  const data: Registry = {
    sales: [{ x: 1, y: 2 }],
    groups: [{ label: 'a', values: [1, 2, 2, 3] }],
  };
  // 数据集名 -> 行数组；行是「字段名 -> 任意 JSON 值」。
  expectTypeOf<Registry>().toEqualTypeOf<{ [name: string]: { [field: string]: unknown }[] }>();
  expectTypeOf(data.sales).toEqualTypeOf<{ [field: string]: unknown }[]>();
  expectTypeOf(data.groups[0].values).toEqualTypeOf<unknown>();
});

test('该报错的确实报错', () => {
  // @ts-expect-error 未知图型：字面量不在任何一个变体的 type 里
  const unknownType: SeriesSpec = { type: 'nope' };

  // @ts-expect-error data 的元素不是点（字符串不在 PointSpec 里）
  const badPoint: SeriesSpec = { type: 'scatter', data: ['x'] };

  // @ts-expect-error width 是数字，不是字符串
  const badWidth: RenderSpec = { width: '640' };

  // @ts-expect-error 注册表的值必须是行数组，不是行
  const badRegistry: Registry = { sales: { x: 1 } };

  console.log(unknownType, badPoint, badWidth, badRegistry);
});

test('dataset + encode 形式不需要 data', () => {
  // 运行时 data 由 Rust 端 bind 注入，所以类型上 scatter 可以不带 data（仅用 dataset + encode）。
  // At runtime `data` is injected by the Rust side, so a scatter can be written without `data`
  // (using only `dataset` + `encode`) — the type reflects that.
  const ds: SeriesSpec = { type: 'scatter', dataset: 'sales', encode: { x: 'revenue', y: 'profit' } };
  if (ds.type === 'scatter') {
    expectTypeOf(ds).toEqualTypeOf<ScatterSeries>();
  }
});
