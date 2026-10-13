plugins {
    kotlin("multiplatform") version "2.4.0"
    kotlin("plugin.serialization") version "2.4.0"
    id("de.quati.ogen") version "0.15.0"
    // AGP 9 起，KMP 模块不能再配 `com.android.library`；用 KMP 原生的 Android 插件。
    id("com.android.kotlin.multiplatform.library") version "9.4.1"
}

group = "io.github.shijianjs"
version = "0.2.0"

/** 仓库根：`sdk/kotlin/` 往上两级，用来定位共用的 OpenAPI 包装脚本。 */
val repoRoot = layout.projectDirectory.dir("../..")
val schemaDir = repoRoot.dir("sdk/json_schema").asFile
val openapiScript = repoRoot.file("sdk/openapi/openapi.mjs").asFile
val openapiSpecFile = layout.buildDirectory.file("openapi/openapi.json").get().asFile

/**
 * ogen 在**配置阶段**就要求 spec 文件存在（不是等任务执行），所以必须在配置期把它准备好。
 * 用 mtime 做一次廉价的新鲜度判断：只有 schema 或脚本比产物新（或产物缺失）时才真跑一遍共用的
 * `sdk/openapi/openapi.mjs`（与 Java SDK 同一个脚本 —— 归一化全部语言无关），平时是个 no-op。
 *
 * ogen requires the spec file to exist already at **configuration** time (not when its task runs),
 * so it has to be produced during configuration. A cheap mtime freshness check keeps the shared
 * wrapper script (the same one the Java SDK uses) from running on every build when nothing changed.
 */
run {
    val inputs = (schemaDir.listFiles()?.toList() ?: emptyList()) + openapiScript
    val newest = inputs.maxOfOrNull { it.lastModified() } ?: 0L
    if (!openapiSpecFile.exists() || openapiSpecFile.lastModified() < newest) {
        openapiSpecFile.parentFile.mkdirs()
        providers.exec {
            commandLine("node", openapiScript.absolutePath, "--out", openapiSpecFile.absolutePath)
        }.result.get()
    }
}

/** 显式重生成 spec（供 `just kotlin_sdk` / CI 使用）；日常构建由上面的配置期判断兜底。 */
val genOpenapi = tasks.register<Exec>("genOpenapi") {
    inputs.file(openapiScript)
    inputs.dir(schemaDir)
    outputs.file(openapiSpecFile)
    commandLine("node", openapiScript.absolutePath, "--out", openapiSpecFile.absolutePath)
}

kotlin {
    // ogen 生成的模型带 @ExperimentalSerializationApi（kotlinx 的判别联合 API 尚属实验性），
    // 这里统一 opt-in，免得整个构建被警告刷屏。消费方用到 `SeriesSpecDto` 时仍需自行 opt-in。
    compilerOptions {
        optIn.add("kotlinx.serialization.ExperimentalSerializationApi")
    }

    jvm()
    js {
        browser()
        nodejs()
    }
    @OptIn(org.jetbrains.kotlin.gradle.ExperimentalWasmDsl::class)
    wasmJs {
        browser()
        nodejs()
    }

    // Android：插件是 com.android.kotlin.multiplatform.library（AGP 9），配置进 `kotlin { android {} }`。
    // 只有编译 Android 产物才需要本机装 Android SDK（ANDROID_HOME），其余目标不受影响。
    android {
        namespace = "io.github.shijianjs.duckfnkuva"
        compileSdk = 36
        minSdk = 24
        // 打开 host test，Android 才会跑 commonTest 里那批用例（否则只有其它目标会跑）。
        withHostTest {}
    }

    // 下面这些目标不是随手挑的，是**依赖已发布平台变体**的交集：生成的代码依赖
    // `de.quati.ogen:core`（`OptionSerializer`）与 `de.quati:kotlin-util`（`Option`），
    // 二者都只发布了 jvm / js / wasmJs / iosX64 / iosArm64 / iosSimulatorArm64 / macosArm64
    // （kotlin-util 另有 linuxX64 / linuxArm64 / mingwX64，但 ogen:core 没有 —— 取交集）。
    // 多声明一个，KMP 的 `kmpPartiallyResolvedDependenciesChecker` 就会报 error 级失败；
    // 上游补齐变体后，把对应那行加回来即可。
    //
    // These targets are the intersection of what the dependencies publish — not a hand-picked
    // subset: the generated code needs `de.quati.ogen:core` and `de.quati:kotlin-util`, which
    // between them only ship jvm / js / wasmJs / iosX64 / iosArm64 / iosSimulatorArm64 / macosArm64.
    // Declaring one more makes kmpPartiallyResolvedDependenciesChecker fail at error level.
    macosArm64()
    iosX64()
    iosArm64()
    iosSimulatorArm64()

    sourceSets {
        commonMain.dependencies {
            // ogen 生成的模型：`Option` 来自 kotlin-util，序列化走 kotlinx-serialization-json。
            implementation("de.quati.ogen:core:0.15.0")
            implementation("de.quati:kotlin-util:2.6.1")
            implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.11.0")
        }
        commonTest.dependencies {
            implementation(kotlin("test"))
        }
        // JVM 平台的集成测试要真连 DuckDB（加载本地编好的扩展），所以 JDBC 驱动只在 jvmTest 上。
        jvmTest.dependencies {
            implementation("org.duckdb:duckdb_jdbc:1.4.2.0")
        }
    }
}

ogen {
    utilPackageName("io.github.shijianjs.duckfnkuva.util")
    add(packageName = "io.github.shijianjs.duckfnkuva") {
        specFile(openapiSpecFile.absolutePath)
        // `model {}` 是必填的（ogen 的 DSL 要求）；留空即默认。不写 client / server 块 = 只生成模型。
        model {}
    }
}

// ogen 的任务（ogenGenerate / ogenValidate）顺带把 spec 刷新一遍。
tasks.matching { it.name.startsWith("ogen") }.configureEach {
    dependsOn(genOpenapi)
}
