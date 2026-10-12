# duckfn-kuva (Java)

`kuva_render(json, data_json)` 两个参数的 Java 模型：把一张图的配置拼成带 Jackson 注解的对象，
序列化成 JSON 交给 DuckDB 扩展。**不是手写模型**，由
[openapi-generator](https://openapi-generator.tech) 从 `../json_schema/` 生成。

```java
PointSpec p = new PointSpec().x(1.0).y(2.0);
ScatterSeries scatter = new ScatterSeries().type("scatter").data(List.of(p)).legend("a");
RenderSpec spec = new RenderSpec().series(List.of(new SeriesSpec(scatter)));

String json = new ObjectMapper().writeValueAsString(spec);
// -> {"series":[{"type":"scatter","data":[{"x":1.0,"y":2.0}],"legend":"a"}]}
// 把它作为第一个参数交给 SQL：SELECT kuva_render(?)
```

需要 `com.fasterxml.jackson.databind.ObjectMapper`（配 `Include.NON_NULL` 更干净）。

## 生成链路

```text
sdk/json_schema/*.schema.json  --tools/openapi.mjs-->  target/openapi.json  --openapi-generator-->  Java 模型
```

`tools/openapi.mjs` 做两件事，**都不是第二份真相**：

1. 把 `$defs` 搬进 `components/schemas`、改写 `$ref` 前缀，并给 `SeriesSpec` 那个 64 成员 `oneOf`
   补 `discriminator`（raw JSON Schema 没有这个关键字，openapi-generator 靠它做多态分发）。
2. 把 `anyOf` / `oneOf` 里的**内联数组成员**提成具名 `$ref` —— openapi-generator 对内联数组会拿
   类型字符串当类名，生成 `List<Double>.class` 这种非法 Java。

openapi-generator 用 `generatorName=java`、`library=native`（Jackson）。`SeriesSpec` 生成成一个
**包装类**：`class SeriesSpec extends AbstractOpenApiSchema`，`new SeriesSpec(scatter)` 包一层、
序列化时按实际类型分发（Java 没有原生 tagged union，这一层分发是必须的）；每个图型变体自己是一个
独立的 Jackson POJO。

## 目录结构

```text
sdk/java/
  pom.xml                 Maven 配置：exec（跑 tools/openapi.mjs）+ openapi-generator + Jackson 依赖
  tools/openapi.mjs       json_schema -> OpenAPI 3.1 的转换（补 discriminator、提升内联成员）
  src/test/java/…         手写测试：构造 RenderSpec + Jackson 序列化
  target/                 产物（generated-sources 在这里，已 gitignore）
```

生成的模型在构建期落进 `target/generated-sources/openapi/`，**不入库**；入库的是生成它的配置
（`pom.xml` + `tools/openapi.mjs`）与测试。

## 命令

```shell
just java_sdk            # = cd sdk/java && mvn -q generate-sources（仅重生成模型）
cd sdk/java && mvn test  # 生成 + 编译 + 跑测试
cd sdk/java && mvn package   # 打成 jar
```

前置：JDK 17+ 与 Maven。改 Rust 类型后的完整链路：
`just json_schema`（重生成 schema）→ `just java_sdk`。

## 真·集成测试（加载本地扩展）

`src/test/java/…/RenderIntegrationTest.java` 用 DuckDB 的 JDBC 驱动（`org.duckdb:duckdb_jdbc`，
test 作用域）加载**本地编译好的**扩展 `build/debug/duckfn_kuva.duckdb_extension`（配
`allow_unsigned_extensions`），真正调一遍 `kuva_render`：

- 一参形式：Java 拼好的 spec 序列化后交给 `kuva_render(?)`；
- 两参形式：配置在 Java 里类型安全地拼（`dataset` + `encode`，走 `putAdditionalProperty`），数据那一项
  交给 SQL 的 `to_json + array_agg` 折叠、只把 SVG 取回 —— 百万行也不回传客户端。

跑之前先编扩展；扩展不存在时整段 skip（与 `sdk/ts` 的集成测试同一套规则）。

```shell
just build
cd sdk/java && mvn test
```

## 已知边界

- `SeriesSpec` 是包装类而非常量联合；变体类型（`ScatterSeries` 等）是普通 POJO。
- `const` 字段（如每个变体的 `type`）openapi-generator 生成为 `Object`，设值时给字符串即可。
- `dataset` / `encode` 不在 schema 里（扩展在反序列化**之前**消费它们），图型模型因此开了
  `additionalProperties`，用 `putAdditionalProperty("dataset", …)` 写入。
- 未设的集合字段会被生成成空列表（`= new ArrayList<>()`），序列化出 `"data":[]`。扩展把「空的 inline 值」
  视同没给，所以 `dataset` / `encode` 形式不受影响；配 `NON_NULL` 的 `ObjectMapper` 能让 JSON 更干净。
- 生成物带 `@javax.annotation.Generated` / `@javax.annotation.Nonnull`，所以运行期依赖里带了
  `javax.annotation-api` 与 findbugs `jsr305`（见 `pom.xml`）。
