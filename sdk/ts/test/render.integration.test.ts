/**
 * 真·集成测试：用 DuckDB 的 Node.js 绑定（`duckdb`）加载**本地编译好的**扩展
 * `build/debug/duckfn_kuva.duckdb_extension`，真正调一遍 `kuva_render`，确认 SDK 里的
 * TypeScript 类型和 Rust 端的真实行为对得上 —— 不是 mock，不是生成器自嗨。
 *
 * Real integration test: load the **locally built** extension (`build/debug/duckfn_kuva.duckdb_extension`)
 * through DuckDB's Node.js binding (`duckdb`) and actually call `kuva_render`, so the TypeScript types in
 * this SDK are checked against the real Rust behavior — no mocks, no generator talking to itself.
 *
 * 跑之前先编扩展：`just build`（产物在 `build/debug/`）。扩展不存在时整段 skip，所以 CI 里只编
 * 了 npm 包、没编扩展的情况下 `npm test` 依然绿（只是这些用例被跳过）。本地想真跑：`npm run test:integration`。
 *
 * Build the extension first (`just build`, artifact in `build/debug/`). When the binary is absent the whole
 * suite is skipped, so `npm test` stays green in CI where only the npm package — not the extension — is built.
 * To run for real locally: `npm run test:integration`.
 */

import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import * as duckdb from 'duckdb';
import { afterAll, beforeAll, describe, expect, test } from 'vitest';
import type { Registry, RenderSpec } from '../src/index';

// 仓库根下的 build/debug/；本文件在 sdk/ts/test/，向上三级到仓库根。
// Repo-root build/debug/; this file lives in sdk/ts/test/, three levels up to the repo root.
const EXT = fileURLToPath(
  new URL('../../../build/debug/duckfn_kuva.duckdb_extension', import.meta.url),
);
const haveExtension = existsSync(EXT);

let db: duckdb.Database;
let con: duckdb.Connection;

type Row = Record<string, unknown>;
type AllCb = (err: Error | null, rows: Row[]) => void;

async function openDatabase(): Promise<void> {
  db = await new Promise<duckdb.Database>((resolve, reject) => {
    const d = new duckdb.Database(
      ':memory:',
      { allow_unsigned_extensions: 'true' },
      (err) => (err ? reject(err) : resolve(d)),
    );
  });
  // LOAD 是本地的未签名扩展，必须走 SQL；路径里的反斜杠在 SQL 字符串里是转义符，换成正斜杠。
  // LOAD is a local unsigned extension and must go through SQL; backslashes in the path are SQL
  // escapes, so normalize to forward slashes.
  await new Promise<void>((resolve, reject) => {
    db.exec(`LOAD '${EXT.replace(/\\/g, '/')}'`, (err) => (err ? reject(err) : resolve()));
  });
  con = db.connect();
}

function render(spec: RenderSpec, data?: Registry): Promise<string> {
  const specJson = JSON.stringify(spec);
  const dataJson = data === undefined ? undefined : JSON.stringify(data);
  const sql = dataJson === undefined
    ? 'SELECT kuva_render(?) AS svg'
    : 'SELECT kuva_render(?, ?) AS svg';
  const all = con.all.bind(con) as unknown as (...args: unknown[]) => void;
  return new Promise<string>((resolve, reject) => {
    const cb: AllCb = (err, rows) => {
      if (err) reject(err);
      else resolve(rows[0]?.svg as string);
    };
    if (dataJson === undefined) all(sql, specJson, cb as AllCb);
    else all(sql, specJson, dataJson, cb as AllCb);
  });
}

function expectSvg(svg: string, label: string): void {
  expect(svg.startsWith('<svg'), `${label}: 应以 <svg 开头`).toBe(true);
  expect(svg.trimEnd().endsWith('</svg>'), `${label}: 应以 </svg> 结尾`).toBe(true);
  expect(svg.length, `${label}: SVG 太短，像没画出来`).toBeGreaterThan(500);
}

describe.skipIf(!haveExtension)('kuva_render —— 真实 DuckDB + 本地扩展', () => {
  beforeAll(async () => {
    await openDatabase();
  });

  afterAll(async () => {
    await new Promise<void>((resolve) => con.close(() => resolve()));
    await new Promise<void>((resolve) => db.close(() => resolve()));
  });

  test('scatter：最小 spec 产出一张 SVG', async () => {
    const svg = await render({
      series: [{ type: 'scatter', data: [[1, 2], [3, 4], [5, 3]] }],
    });
    expectSvg(svg, 'scatter');
  });

  test('line：折线图也能渲染', async () => {
    const svg = await render({
      series: [{ type: 'line', data: [[0, 1], [1, 2], [2, 1.5]] }],
    });
    expectSvg(svg, 'line');
  });

  test('bar：带类目与数值', async () => {
    const svg = await render({
      series: [{ type: 'bar', categories: ['a', 'b', 'c'], values: [3, 5, 2] }],
    });
    expectSvg(svg, 'bar');
  });

  test('histogram：自动分箱', async () => {
    const svg = await render({
      series: [{ type: 'histogram', values: [1, 2, 2, 3, 3, 3, 4], bins: 4 }],
    });
    expectSvg(svg, 'histogram');
  });

  test('两参形式：config 走 SDK 类型、data 由宿主喂', async () => {
    // 配置（series 用 dataset + encode 指到数据列）与数据（数据集注册表）分开，
    // 正是「配置类型安全、数据交给 SQL」的用法。`data` 在 dataset 形式下由 Rust 端 bind 注入，
    // 所以类型里它是可选的 —— 这里直接按 RenderSpec 写，无需任何断言转换。
    // Config (series points at data columns via dataset + encode) and data (the dataset registry)
    // are separate — exactly the "config type-safe, data from SQL" usage. `data` is optional in the
    // type because the `dataset` form has it injected by the Rust side, so this is written as a plain
    // RenderSpec with no casts.
    const spec: RenderSpec = {
      series: [
        { type: 'scatter', dataset: 'sales', encode: { x: 'revenue', y: 'profit' } },
      ],
    };
    const data: Registry = {
      sales: [
        { revenue: 10, profit: 3 },
        { revenue: 12, profit: 5 },
        { revenue: 8, profit: 2 },
      ],
    };
    const svg = await render(spec, data);
    expectSvg(svg, 'two-arg');
  });

  test('title 真的进了 SVG 文本', async () => {
    const svg = await render({
      title: { text: 'Quarterly Sales' },
      series: [{ type: 'scatter', data: [[1, 2], [3, 4]] }],
    });
    expectSvg(svg, 'title');
    expect(svg.includes('Quarterly Sales'), 'title 文本应出现在 SVG 里').toBe(true);
  });
});
