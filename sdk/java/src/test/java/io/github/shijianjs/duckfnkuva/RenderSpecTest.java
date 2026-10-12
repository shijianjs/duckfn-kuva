package io.github.shijianjs.duckfnkuva;

import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.List;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

/**
 * 端到端：用生成的模型拼一张图的 spec，交给 Jackson 序列化成 `kuva_render(json)` 的第一个参数。
 * 这是 Java 宿主「类型安全地拼配置、数据交给 SQL」用法的核心。
 */
class RenderSpecTest {

    private static final ObjectMapper MAPPER =
            new ObjectMapper().setSerializationInclusion(JsonInclude.Include.NON_NULL);

    @Test
    void buildsAndSerializesAScatterSpec() throws Exception {
        PointSpec point = new PointSpec().x(1.0).y(2.0);
        ScatterSeries scatter =
                new ScatterSeries().type("scatter").data(List.of(point)).legend("a");
        RenderSpec spec = new RenderSpec().series(List.of(new SeriesSpec(scatter)));

        String json = MAPPER.writeValueAsString(spec);

        assertTrue(json.contains("\"series\""), json);
        assertTrue(json.contains("\"type\":\"scatter\""), json);
        assertTrue(json.contains("\"x\":1.0"), json);
        assertTrue(json.contains("\"y\":2.0"), json);
        assertTrue(json.contains("\"legend\":\"a\""), json);
    }

    @Test
    void aPointCanCarryAsymmetricErrorBars() throws Exception {
        PointSpec point =
                new PointSpec()
                        .x(1.0)
                        .y(2.0)
                        .yErr(new ErrSpec(new ErrArms().neg(0.1).pos(0.3)));
        ScatterSeries scatter = new ScatterSeries().type("scatter").data(List.of(point));
        RenderSpec spec = new RenderSpec().series(List.of(new SeriesSpec(scatter)));

        String json = MAPPER.writeValueAsString(spec);

        assertTrue(json.contains("\"neg\":0.1"), json);
        assertTrue(json.contains("\"pos\":0.3"), json);
    }
}
