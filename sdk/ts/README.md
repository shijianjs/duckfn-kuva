# @shijianjs/duckfn-kuva

`kuva_render(json, data_json)` 两个参数的 TypeScript 类型，供宿主 SDK 生成类型化模型。
类型由 [json-schema-to-typescript](https://www.npmjs.com/package/json-schema-to-typescript)
从 `../json_schema/` 下的 JSON Schema 生成 —— **纯类型包，没有任何运行时代码**。

```ts
import type { RenderSpec, Registry, SeriesSpec } from "@shijianjs/duckfn-kuva";

const chart: RenderSpec = {
  width: 640,
  series: [{ type: "scatter", data: [[1, 2]], legend: "a" }],
};
```

64 个图型是真正的判别联合，`switch (s.type)` 收窄到对应变体，未知图型、变体缺必填字段
都是编译错误：

```ts
function label(s: SeriesSpec) {
  switch (s.type) {
    case "scatter": return s.data.length;   // 收窄到 ScatterSeries
    case "violin":  return s.groups?.length; // 收窄到 ViolinSeries
  }
}
```

## 目录结构

```text
sdk/ts/
  package.json          包描述：exports / files / scripts / 钉住的生成器与测试框架版本
  package-lock.json     锁定 json-schema-to-typescript、typescript、vitest 版本（提交入库）
  tsconfig.json         检查配置（npm run check）：src + test，严格模式，不产文件
  tsconfig.build.json   构建配置（npm run build）：只编 src，向 dist/ 产 .d.ts
  src/
    render.ts           产物：第一个参数（图表配置）的类型，根类型 RenderSpec
    datasets.ts         产物：第二个参数（数据集注册表）的类型，根类型 Registry
    index.ts            手写：再导出上面两个，让消费者从包根导入
  test/
    types.test-d.ts     vitest 类型测试：判别联合收窄、字段形状、@ts-expect-error 负例
    schema.test.ts      vitest 运行时测试：盯 schema 文件本身的结构（$ref 自洽、标签归一化）
  dist/                 构建产物（.gitignore），发布的是这里面的 .d.ts
```

`src/render.ts` 与 `src/datasets.ts` 是**产物**，不手改；`src/index.ts` 与 `test/` 是手写的。

## 命令

```bash
npm install        # 装 devDependencies（生成器 + TypeScript + vitest）
npm run gen        # 重新生成 src/render.ts 与 src/datasets.ts
npm run check      # tsc --noEmit：src + test 全量严格类型检查
npm test           # vitest run --typecheck：5 个类型测试 + 4 个 schema 结构测试
npm run build      # tsc -p tsconfig.build.json：向 dist/ 产 .d.ts
npm run prepublishOnly   # check + test + build（npm publish 时自动跑）
```

测试是 [vitest](https://vitest.dev) 的两种模式叠在一起：`*.test-d.ts` 走 `--typecheck`（tsc
驱动，断言生成出来的类型），`*.test.ts` 走普通运行时（断言 schema 文件本身）。纯类型包没有
运行时代码可测，所以「该报错的确实报错」用 `@ts-expect-error` 钉住 —— vitest 的 typecheck 会把
变成 unused 的指令算成失败，生成物哪天退化了这里先响。

改 Rust 类型后的完整链路：`just json_schema`（重新生成 schema）→ `npm run gen`。
根 Justfile 的 `just ts_sdk` 等价于后两步（`npm install` + `npm run gen`）。

## 发布到 npm

包名 `@shijianjs/duckfn-kuva`（与 GitHub 组织一致，未占用）。发布前确认三件事：

1. **版本号**：`package.json` 的 `version` 与扩展版本对齐（当前跟 `Cargo.toml` 的
   `0.1.1-dev.0`）。`just release_bump` 不碰这个文件 —— 它按 git grep 替换上一个 tag 的版本号，
   而开发中的 `-dev.N` 号不含上一个正式版本号字样。**发版时手工同步一次**。
2. `npm run check` 与 `npm run build` 通过（`prepublishOnly` 会自动跑，过不了发不出去）。
3. `npm publish --access public`（scoped 包首次发布需要显式 public）。

`dist/` 与 `node_modules/` 都已 gitignore；入库的是源（`src/`）+ 锁版本（`package-lock.json`），
CI 会用同一份锁文件重新生成并比对，防止产物与 schema 脱节。

## 选型与已知边界

- 生成器是 json-schema-to-typescript 而非 quicktype：quicktype 的 JSON Schema 模式不把 `oneOf`
  生成为判别联合（64 个变体会被合并成一个大接口、所有字段可选）。详见
  `../json_schema/README.md` 与开发指南。
- 每个接口都带 `[k: string]: unknown`：schema 没有设 `additionalProperties: false`，与运行时
  「未知键忽略」一致（`dataset` / `encode` 这类绑定前消费的键也能通过）。代价是拼错字段名
  编译期抓不到。
- `Option<T>` 生成 `field?: T | null`：`undefined` 与显式 `null` 都收，比 Rust 宽一点。
