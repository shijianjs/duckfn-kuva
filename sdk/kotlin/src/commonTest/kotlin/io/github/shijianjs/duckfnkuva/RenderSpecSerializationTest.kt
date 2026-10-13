package io.github.shijianjs.duckfnkuva

import de.quati.kotlin.util.Option
import io.github.shijianjs.duckfnkuva.model.RenderSpecDto
import io.github.shijianjs.duckfnkuva.model.ScatterSeriesDto
import io.github.shijianjs.duckfnkuva.model.SeriesSpecDto
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertTrue

/**
 * 冒烟测试：生成的模型能按 kotlinx.serialization 的判别联合序列化成 `kuva_render` 要的形状 ——
 * `type` 判别标签，以及类型安全的 `dataset` / `encode` 两个键。
 *
 * Smoke tests: the generated models serialize into the shape `kuva_render` expects — a
 * discriminated union carrying the `type` tag, plus the typed `dataset` / `encode` keys.
 */
@OptIn(ExperimentalSerializationApi::class)
class RenderSpecSerializationTest {

    @Test
    fun seriesCarriesItsTypeDiscriminator() {
        val encoded = Json.encodeToString<SeriesSpecDto>(ScatterSeriesDto())
        assertTrue(encoded.contains("\"type\":\"scatter\""), "expected the type tag, got: $encoded")
    }

    @Test
    fun datasetAndEncodeAreTypedFields() {
        val scatter =
                ScatterSeriesDto(
                        dataset = Option.Some<String?>("sales"),
                        encode =
                                Option.Some<Map<String, String>?>(
                                        mapOf("x" to "revenue", "y" to "profit")
                                ),
                )
        val spec = RenderSpecDto(series = Option.Some<List<SeriesSpecDto>>(listOf(scatter)))

        val encoded = Json.encodeToString(spec)

        assertTrue(encoded.contains("\"type\":\"scatter\""), "type tag: $encoded")
        assertTrue(encoded.contains("\"dataset\":\"sales\""), "dataset: $encoded")
        assertTrue(encoded.contains("\"x\":\"revenue\""), "encode: $encoded")
    }
}
