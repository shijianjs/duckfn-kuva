/**
 * `kuva_render(json, data_json)` 两个参数的 TypeScript 类型 —— 包的入口。
 *
 * `render.ts` 与 `datasets.ts` 是 json-schema-to-typescript 从 `../json_schema/` 生成的**产物**
 * （`npm run gen`），不要手改；这个文件是唯一手写的源文件，只做再导出，让消费者从包根导入：
 *
 * ```ts
 * import type { RenderSpec, Registry } from "@shijianjs/duckfn-kuva";
 * ```
 *
 * The TypeScript types of `kuva_render(json, data_json)`'s two arguments — the package entry.
 *
 * `render.ts` and `datasets.ts` are **generated artifacts** (json-schema-to-typescript out of
 * `../json_schema/`, `npm run gen`); do not hand-edit them. This file is the only hand-written
 * source and merely re-exports, so consumers import from the package root.
 */

export * from "./render";
export * from "./datasets";
