# duckfn-kuva (Kotlin Multiplatform)

`kuva_render(json, data_json)` 两个参数的 Kotlin Multiplatform 模型：把一张图的配置拼成带
kotlinx.serialization 注解的对象，序列化成 JSON 交给 DuckDB 扩展。**不是手写模型**，由
[ogen](https://github.com/goquati/ogen) 从 `../json_schema/` 生成。

```kotlin
val spec =
        RenderSpecDto(
                series =
                        Option.Some<List<SeriesSpecDto>>(
                                listOf(
                                        ScatterSeriesDto(
                                                dataset = Option.Some<String?>("sales"),
                                                encode =
                                                        Option.Some<Map<String, String>?>(
                                                                mapOf(
                                                                        "x" to "revenue",
                                                                        "y" to "profit"
                                                                )
                                                        ),
                                        )
                                )
                        )
        )

val json = Json.encodeToString(spec)
// -> {"series":[{"type":"scatter","dataset":"sales","encode":{"x":"revenue","y":"profit"}}]}
// 把它作为第一个参数交给 SQL：SELECT kuva_render(?)
```

## 为什么能得到真正的判别联合

`SeriesSpec` 是 64 个变体的判别联合（Rust 侧 `#[serde(tag = "type")]`）。kotlinx.serialization 原生支持
sealed 层级的判别联合，ogen 生成出来的就是它：

```kotlin
@Serializable
@JsonClassDiscriminator("type")
sealed interface SeriesSpecDto

@Serializable
@SerialName("scatter")
data class ScatterSeriesDto(...) : SeriesSpecDto
```

即**密封层级 + `@JsonClassDiscriminator`**，多态分发由 kotlinx 自己完成 —— 比 Java 那边的包装类
（`openapi-generator` 的 `oneOf` 产物）干净。这也是选 ogen 而不是 openapi-generator 的 Kotlin
生成器的原因：后者的 `library=multiplatform` 会把 64 个变体的字段全拍进基类（几百个 `abstract val`）
且变体不继承它，完全不可用。

## 生成链路

```text
sdk/json_schema/*.schema.json --sdk/openapi/openapi.mjs--> build/openapi/openapi.json --ogen--> Kotlin 模型
```

`sdk/openapi/openapi.mjs` 与 Java SDK **共用**（归一化全部语言无关）：`$defs` 搬进
`components/schemas`、改写 `$ref`、给 `SeriesSpec` 补 `discriminator`、提升内联联合成员、去掉
`default`。`dataset` / `encode` 这两个「反序列化前被消费」的键不在这里补 —— 它们已经由 schema 生成器
（`src/bin/json_schema.rs` 的 `inject_dataset_fields`）写进 `render.schema.json`，所以各语言拿到的是
**同样的显式字段**。

ogen 在**配置阶段**就要读 spec（不是等任务执行），所以 `build.gradle.kts` 会在配置期按 mtime 判断
新鲜度、必要时先跑一遍包装脚本。

## 目标平台

KMP 全平台：`jvm`、`js`、`wasmJs`、`linuxX64` / `linuxArm64`、`mingwX64`、`macosX64` / `macosArm64`、
`iosX64` / `iosArm64` / `iosSimulatorArm64`、`watchosArm32` / `watchosArm64` / `watchosDeviceArm64` /
`watchosSimulatorArm64`、`tvosX64` / `tvosArm64` / `tvosSimulatorArm64`。

Apple 目标只能在 macOS 上**构建**（KMP 的既定限制；在这里声明是给 macOS CI 用的，非 macOS 宿主上
对应的测试任务会被禁用）。Android 目标没开：它需要 AGP + Android SDK，会把本地构建整个卡住。

## 命令

```shell
just kotlin_sdk                       # = cd sdk/kotlin && ./gradlew ogenGenerate（仅重生成模型）
cd sdk/kotlin && ./gradlew jvmTest    # 生成 + 编译 + 跑测试
```

前置：JDK 21+（ogen 的要求）；Gradle 由 `gradlew` 自带（发行包走腾讯镜像，见
`gradle/wrapper/gradle-wrapper.properties`，想换回官方源改那一行即可）。

## 已知边界

- 生成物带 `@ExperimentalSerializationApi`（kotlinx 的判别联合 API 尚属实验性）。本模块内部已统一
  opt-in；**消费方**用到 `SeriesSpecDto` 时需要自己加 `@OptIn(ExperimentalSerializationApi::class)`。
- 字段一律是 `de.quati.kotlin.util.Option<T>`：`Option.Some(v)` 表示「给了」，`Option.Undefined`
  （默认值）表示「没给」。比裸 `null` 多一分信息 —— 「显式给了 null」与「没给」语义不同。
- 未设的字段不会被序列化出去（`Option.Undefined` 不产出键），所以不会出现 `"data":[]` 那种噪声。
- ogen 目前是 `0.15.0`（pre-1.0），版本在 `build.gradle.kts` 里钉住。
