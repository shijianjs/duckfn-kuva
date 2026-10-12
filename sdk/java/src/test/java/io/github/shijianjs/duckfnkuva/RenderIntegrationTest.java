package io.github.shijianjs.duckfnkuva;

import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.Statement;
import java.util.List;
import java.util.Map;
import java.util.Properties;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

/**
 * 真·集成测试：通过 DuckDB 的 JDBC 驱动加载**本地编译好的**扩展
 * `build/debug/duckfn_kuva.duckdb_extension`，真正调一遍 `kuva_render` —— 不是 mock，也不是生成器自嗨。
 *
 * 跑之前先编扩展：`just build`（产物在 `build/debug/`）。扩展不存在时整段 skip（与 `sdk/ts` 的集成
 * 测试同一套规则）。
 */
class RenderIntegrationTest {

    /**
     * 仓库根下的 build/debug/；本测试的工作目录是 sdk/java（Maven 的 basedir），向上两级到仓库根。
     */
    private static final Path EXT =
            Paths.get("../../build/debug/duckfn_kuva.duckdb_extension").toAbsolutePath().normalize();

    private static final ObjectMapper MAPPER =
            new ObjectMapper().setSerializationInclusion(JsonInclude.Include.NON_NULL);

    private static Connection open() throws Exception {
        Properties props = new Properties();
        props.setProperty("allow_unsigned_extensions", "true");
        Connection con = DriverManager.getConnection("jdbc:duckdb:", props);
        // 本地未签名扩展：路径里的反斜杠在 SQL 字符串里是转义符，换成正斜杠。
        try (Statement st = con.createStatement()) {
            st.execute("LOAD '" + EXT.toString().replace('\\', '/') + "'");
        }
        return con;
    }

    /** 一参形式：把 Java 拼好的 spec 序列化后交给 `kuva_render(?)`。 */
    private static String render(Connection con, RenderSpec spec) throws Exception {
        try (PreparedStatement ps = con.prepareStatement("SELECT kuva_render(?) AS svg")) {
            ps.setString(1, MAPPER.writeValueAsString(spec));
            try (ResultSet rs = ps.executeQuery()) {
                rs.next();
                return rs.getString("svg");
            }
        }
    }

    private static void expectSvg(String svg, String label) {
        assertNotNull(svg, label + ": no SVG");
        assertTrue(svg.startsWith("<svg"), label + ": should start with <svg");
        assertTrue(svg.trim().endsWith("</svg>"), label + ": should end with </svg>");
        assertTrue(svg.length() > 500, label + ": SVG looks too short");
    }

    @Test
    void rendersAScatterSpec() throws Exception {
        Assumptions.assumeTrue(Files.exists(EXT), "extension not built: " + EXT);

        ScatterSeries scatter =
                new ScatterSeries()
                        .type("scatter")
                        .data(List.of(
                                new PointSpec().x(1.0).y(2.0),
                                new PointSpec().x(3.0).y(4.0),
                                new PointSpec().x(5.0).y(3.0)));
        RenderSpec spec = new RenderSpec().series(List.of(new SeriesSpec(scatter)));

        try (Connection con = open()) {
            expectSvg(render(con, spec), "scatter");
        }
    }

    /**
     * 两参形式：配置在 Java 里用类型安全地拼（`dataset` + `encode`），数据那一项交给 DuckDB 自己用
     * `to_json + array_agg` 折叠成单个 JSON 值、只把 SVG 取回 —— 百万行也不回传客户端。这正是
     * `kuva_render(json, data)` 做两个参数的用意，也是 Java 宿主该走的路。
     */
    @Test
    void rendersConfigFromJavaAndDataFromSql() throws Exception {
        Assumptions.assumeTrue(Files.exists(EXT), "extension not built: " + EXT);

        // `dataset` / `encode` 不在 schema 里（扩展在反序列化之前消费它们），走 additionalProperties。
        ScatterSeries scatter =
                new ScatterSeries()
                        .type("scatter")
                        .putAdditionalProperty("dataset", "d")
                        .putAdditionalProperty("encode", Map.of("x", "a", "y", "b"));
        RenderSpec spec = new RenderSpec().series(List.of(new SeriesSpec(scatter)));

        String sql =
                "SELECT kuva_render(?, to_json({'d': array_agg({'a': a, 'b': b} ORDER BY a)})) AS svg "
                        + "FROM (VALUES (1, 2), (3, 4), (5, 3)) AS t(a, b)";

        try (Connection con = open();
                PreparedStatement ps = con.prepareStatement(sql)) {
            ps.setString(1, MAPPER.writeValueAsString(spec));
            try (ResultSet rs = ps.executeQuery()) {
                rs.next();
                expectSvg(rs.getString("svg"), "two-arg");
            }
        }
    }
}
