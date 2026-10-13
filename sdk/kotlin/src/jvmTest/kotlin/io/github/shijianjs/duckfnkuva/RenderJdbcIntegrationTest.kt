package io.github.shijianjs.duckfnkuva

import de.quati.kotlin.util.Option
import io.github.shijianjs.duckfnkuva.model.RenderSpecDto
import io.github.shijianjs.duckfnkuva.model.ScatterSeriesDto
import io.github.shijianjs.duckfnkuva.model.SeriesSpecDto
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.sql.Connection
import java.sql.DriverManager
import java.util.Properties
import kotlin.test.Test
import kotlin.test.assertTrue
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.Json
import org.junit.Assume

/**
 * 真·集成测试（JVM 平台）：通过 DuckDB 的 JDBC 驱动加载**本地编译好的**扩展
 * `build/debug/duckfn_kuva.duckdb_extension`，真正调一遍 `kuva_render` —— 不是 mock，也不是生成器
 * 自嗨。扩展不存在时整段 skip（与 `sdk/ts`、`sdk/java` 的集成测试同一套规则）。
 *
 * 跑之前先编扩展：`just build`。
 *
 * A real integration test on the JVM target: the locally built extension is loaded through DuckDB's
 * JDBC driver and `kuva_render` is actually called. Skipped when the extension is not built, the same
 * rule the TypeScript and Java integration tests follow.
 */
@OptIn(ExperimentalSerializationApi::class)
class RenderJdbcIntegrationTest {

    /** 仓库根下的 build/debug/；测试的工作目录是 sdk/kotlin，向上两级到仓库根。 */
    private val extension: Path =
            Paths.get("../../build/debug/duckfn_kuva.duckdb_extension")
                    .toAbsolutePath()
                    .normalize()

    private fun open(): Connection {
        val props = Properties()
        props.setProperty("allow_unsigned_extensions", "true")
        val connection = DriverManager.getConnection("jdbc:duckdb:", props)
        // 本地未签名扩展：路径里的反斜杠在 SQL 字符串里是转义符，换成正斜杠。
        connection.createStatement().use {
            it.execute("LOAD '${extension.toString().replace('\\', '/')}'")
        }
        return connection
    }

    private fun expectSvg(svg: String, label: String) {
        assertTrue(svg.startsWith("<svg"), "$label: should start with <svg")
        assertTrue(svg.trimEnd().endsWith("</svg>"), "$label: should end with </svg>")
        assertTrue(svg.length > 500, "$label: SVG looks too short")
    }

    /**
     * 两参形式：配置在 Kotlin 里类型安全地拼（`dataset` + `encode`），数据那一项交给 DuckDB 自己用
     * `to_json + array_agg` 折叠成单个 JSON 值、只把 SVG 取回 —— 百万行也不回传客户端。
     */
    @Test
    fun rendersConfigFromKotlinAndDataFromSql() {
        Assume.assumeTrue("extension not built: $extension", Files.exists(extension))

        val scatter =
                ScatterSeriesDto(
                        dataset = Option.Some<String?>("d"),
                        encode = Option.Some<Map<String, String>?>(mapOf("x" to "a", "y" to "b")),
                )
        val spec = RenderSpecDto(series = Option.Some<List<SeriesSpecDto>>(listOf(scatter)))
        val specJson = Json.encodeToString(spec)

        val sql =
                """
                SELECT kuva_render(?, to_json({'d': array_agg({'a': a, 'b': b} ORDER BY a)})) AS svg
                FROM (VALUES (1, 2), (3, 4), (5, 3)) AS t(a, b)
                """
                        .trimIndent()

        open().use { connection ->
            val svg =
                    connection.prepareStatement(sql).use { statement ->
                        statement.setString(1, specJson)
                        statement.executeQuery().use { rows ->
                            rows.next()
                            rows.getString("svg")
                        }
                    }
            expectSvg(svg, "kotlin-two-arg")
        }
    }
}
