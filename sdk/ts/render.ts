/**
 * 一次渲染的完整规格。单图的字段直接写在顶层；出现 `figure` 就切到多面板模式。
 */
export interface Render {
    annotations?: null | AnnotationsObject;
    /**
     * 色条刻度标签的格式（热力图 / 二维直方图 / 六边形分箱图 / 等高线图）。
     * 取值同 `x_axis.tick_format`。
     */
    colorbar_tick_format?: ColorbarTickFormatEnum | number | null;
    /**
     * 多面板网格（Figure）。给了它，顶层 `panel.series` 被忽略。
     */
    figure?: null | FigureObject;
    /**
     * 文件输出的选项（只有 `kuva_render_file` 看它）。
     *
     * 与 `terminal` 一个路子：落盘目录、文件名、要不要开浏览器都写在 JSON 里，所以这个函数
     * 永远只有一个参数。
     *
     * 只有原生构建有这一项：wasm 下 `kuva_render_file` 整个不存在（见 `functions/mod.rs` 的模块
     * 声明），所以这里连字段都不编译 —— JSON 里给了 `file` 也当未知键忽略。
     *
     * The file-output options (only `kuva_render_file` looks at them). The same shape as
     * `terminal`:
     * the directory, file name and "open it" flag all live in the JSON, so that function always
     * takes
     * one argument. Native builds only: a wasm build has no `kuva_render_file` at all (see the
     * module
     * declaration in `functions/mod.rs`), so even this field is not compiled there — a `file`
     * key in
     * the JSON is ignored as unknown.
     */
    file?: null | FileObject;
    font?: null | FontObject;
    grid?: null | GridObject;
    /**
     * 画布高度（像素，即 SVG 的高）。
     */
    height?:  number | null;
    legend?:  null | LegendObject;
    palette?: string[] | PaletteEnum | null;
    /**
     * 画在**第二根 y 轴**上的 series（右侧那根）。给了它就走双 Y 轴渲染。
     *
     * 与 `series` 互不影响：两拨各自成图，共用一张画布。
     */
    secondary_series?: SecondarySeryElement[];
    /**
     * 叠加到同一套坐标轴上的 series。
     */
    series?: SecondarySeryElement[];
    /**
     * 图角上的统计框。
     */
    stats_box?: null | StatsBoxObject;
    /**
     * 终端渲染的选项（只有 `kuva_render_terminal` 看它）。
     *
     * 放在 JSON 里而不是函数签名上：这样再往里加东西 —— 是否打印、要不要上色 —— 都不用动签名。
     */
    terminal?: null | TerminalObject;
    theme?:    ThemeEnum | null | ThemeObject;
    title?:    null | TitleObject | string;
    /**
     * 画布宽度（像素，即 SVG 的宽）。
     */
    width?: number | null;
    /**
     * 第二根 x 轴（配 `secondary_series` 用；给它就切到双轴渲染）。
     */
    x2_axis?: null | X2AxisObject;
    x_axis?:  null | XAxisObject;
    /**
     * 把 x / y 轴当日期轴排刻度。
     */
    x_datetime?: null | XDatetimeObject;
    /**
     * 第二根 y 轴（右侧）。配 `secondary_series` 用。
     */
    y2_axis?:    null | X2AxisObject;
    y_axis?:     null | XAxisObject;
    y_datetime?: null | XDatetimeObject;
    [property: string]: unknown;
}

export interface AnnotationsObject {
    reference_lines?: ReferenceLineElement[];
    shaded_regions?:  ShadedRegionElement[];
    texts?:           TextElement[];
    [property: string]: unknown;
}

/**
 * 参考线。`orientation` 缺省按 `horizontal` 解释（`value` 落在 y 轴上）。
 */
export interface ReferenceLineElement {
    color?:        null | string;
    dasharray?:    null | string;
    label?:        null | string;
    orientation?:  ReferenceLineOrientation | null;
    stroke_width?: number | null;
    /**
     * 线的位置：水平线是 y 值，垂直线是 x 值。
     */
    value: number;
    [property: string]: unknown;
}

export type ReferenceLineOrientation = "horizontal" | "vertical";

export interface ShadedRegionElement {
    color?:       null | string;
    max:          number;
    min:          number;
    opacity?:     number | null;
    orientation?: ReferenceLineOrientation | null;
    [property: string]: unknown;
}

export interface TextElement {
    arrow_padding?: number | null;
    color?:         null | string;
    font_size?:     number | null;
    /**
     * 画一条箭头指向该数据坐标；`null` 就不画。
     */
    target_x?: number | null;
    target_y?: number | null;
    text:      string;
    x:         number;
    y:         number;
    [property: string]: unknown;
}

export type ColorbarTickFormatEnum = "auto" | "integer" | "sci" | "percent" | "degree";

/**
 * 多面板网格（对应 kuva 的 `Figure`）。
 */
export interface FigureObject {
    cell_height?: number | null;
    cell_width?:  number | null;
    /**
     * 逐列宽度的覆盖：`{"1": 180}`。
     */
    col_widths?:    ColWidths | null;
    cols:           number;
    figure_height?: number | null;
    figure_width?:  number | null;
    /**
     * 共享图例之外，是否保留各面板自己的图例（默认不保留）。
     */
    keep_panel_legends?: boolean | null;
    /**
     * 面板标签：`"uppercase"` / `"lowercase"` / `"numeric"` / `"none"`，自定义数组，
     * 或 `{"names": [...], "style": …, "size": …, "bold": …}`。
     */
    labels?:  string[] | LabelsEnum | null | LabelsObject;
    padding?: number | null;
    /**
     * 逐面板配置，按行优先顺序排列；长度等于 `rows * cols`，或等于 `structure` 的长度。
     */
    panels?: PanelElement[];
    /**
     * 逐行高度的覆盖：`{"2": 80}`（键是 0 起的行号，值是该行的高度）。
     */
    row_heights?: RowHeights | null;
    rows:         number;
    /**
     * 共享图例的位置（`"right_top"` / `"bottom"` / …）。不写就没有共享图例。
     */
    shared_legend?: null | string;
    /**
     * 共享图例的手工条目；给了它就不从各面板收集。
     */
    shared_legend_entries?: EntryElement[] | null;
    shared_x_all?:          boolean | null;
    /**
     * 在这些**列**内部共享 x 范围。
     */
    shared_x_cols?: number[];
    /**
     * 列内共享 x 的一段：`{"index": 列, "start": 起行, "end": 止行}`（含两端）。
     */
    shared_x_slices?: SharedXSliceElement[];
    shared_y_all?:    boolean | null;
    /**
     * 在这些**行**内部共享 y 范围（逐行一个数）。
     */
    shared_y_rows?: number[];
    /**
     * 行内共享 y 的一段：`{"index": 行, "start": 起列, "end": 止列}`（含两端）。
     */
    shared_y_slices?: SharedXSliceElement[];
    spacing?:         number | null;
    /**
     * 合并单元格：每项是一组行优先的格子下标，合成一个**矩形**面板（跨行 / 跨列）。
     * 给了它，`panels` 的长度就等于它的长度，不再是 `rows * cols`。
     */
    structure?:  Array<number[]> | null;
    title?:      null | string;
    title_size?: number | null;
    [property: string]: unknown;
}

export interface ColWidths {
}

export type LabelsEnum = "none" | "uppercase" | "lowercase" | "numeric";

export interface LabelsObject {
    bold?: boolean | null;
    /**
     * 逐个面板的标签文字。
     */
    names: string[];
    size?: number | null;
    /**
     * 缺省按 `"uppercase"` 处理（`with_labels_custom` 的样式字段只影响大小写变换，自定义文字下通常不用）。
     */
    style?: LabelsEnum | null;
    [property: string]: unknown;
}

/**
 * 一块画布：一组叠加的 series + 对布局的覆盖。
 *
 * 单图模式用它当顶层；多面板模式用它当每个 panel。
 */
export interface PanelElement {
    annotations?: null | AnnotationsObject;
    /**
     * 色条刻度标签的格式（热力图 / 二维直方图 / 六边形分箱图 / 等高线图）。
     * 取值同 `x_axis.tick_format`。
     */
    colorbar_tick_format?: ColorbarTickFormatEnum | number | null;
    font?:                 null | FontObject;
    grid?:                 null | GridObject;
    /**
     * 画布高度（像素，即 SVG 的高）。
     */
    height?:  number | null;
    legend?:  null | LegendObject;
    palette?: string[] | PaletteEnum | null;
    /**
     * 画在**第二根 y 轴**上的 series（右侧那根）。给了它就走双 Y 轴渲染。
     *
     * 与 `series` 互不影响：两拨各自成图，共用一张画布。
     */
    secondary_series?: SecondarySeryElement[];
    /**
     * 叠加到同一套坐标轴上的 series。
     */
    series?: SecondarySeryElement[];
    /**
     * 图角上的统计框。
     */
    stats_box?: null | StatsBoxObject;
    theme?:     ThemeEnum | null | ThemeObject;
    title?:     null | TitleObject | string;
    /**
     * 画布宽度（像素，即 SVG 的宽）。
     */
    width?: number | null;
    /**
     * 第二根 x 轴（配 `secondary_series` 用；给它就切到双轴渲染）。
     */
    x2_axis?: null | X2AxisObject;
    x_axis?:  null | XAxisObject;
    /**
     * 把 x / y 轴当日期轴排刻度。
     */
    x_datetime?: null | XDatetimeObject;
    /**
     * 第二根 y 轴（右侧）。配 `secondary_series` 用。
     */
    y2_axis?:    null | X2AxisObject;
    y_axis?:     null | XAxisObject;
    y_datetime?: null | XDatetimeObject;
    [property: string]: unknown;
}

export interface FontObject {
    body_size?:  number | null;
    family?:     null | string;
    label_size?: number | null;
    tick_size?:  number | null;
    title_size?: number | null;
    [property: string]: unknown;
}

/**
 * 网格 / 轴线 / 刻度等非数据部分。
 */
export interface GridObject {
    /**
     * 轴线：`"open"`（默认，只画下左）/ `"box"`（整框）。
     */
    axis_line?:       AxisLineEnum | null;
    axis_line_width?: number | null;
    /**
     * 灰度 / 可访问性模式：调色板换成灰阶 + 线型 / marker 循环。
     */
    bw_mode?: boolean | null;
    /**
     * 轴范围贴紧数据，不再向外取整。
     */
    clamp_axis?:   boolean | null;
    clamp_y_axis?: boolean | null;
    /**
     * 等比例坐标（一个数据单位在两个轴上占同样的像素）。
     */
    equal_aspect?:    boolean | null;
    grid_line_width?: number | null;
    /**
     * 注入悬停 / 点击交互的 CSS 与 JS。
     */
    interactive?: boolean | null;
    /**
     * 柱 / 饼等的值标签是否垫一层背景。
     */
    label_background?: boolean | null;
    /**
     * 主刻度之间再分几个小格（副刻度）。
     */
    minor_ticks?: number | null;
    /**
     * 整体缩放所有文字与刻度（画布尺寸不变）。
     */
    scale?:           number | null;
    show_grid?:       boolean | null;
    show_minor_grid?: boolean | null;
    /**
     * 刻度朝向：`"inside"` / `"outside"` / `"center"`。
     */
    tick_align?:  TickAlignEnum | null;
    tick_length?: number | null;
    /**
     * 刻度位置：`"primary"` / `"both"`（四边镜像）。
     */
    tick_pos?:   TickPosEnum | null;
    tick_width?: number | null;
    ticks?:      number | null;
    /**
     * 一次性给所有文字设折行宽度（标题、轴名、图例）。逐元素设的（`title.wrap`、`x_axis.wrap`、
     * `legend.wrap`）会覆盖它。
     */
    wrap?: number | null;
    [property: string]: unknown;
}

export type AxisLineEnum = "open" | "box";

export type TickAlignEnum = "inside" | "outside" | "center";

export type TickPosEnum = "primary" | "both";

/**
 * 图例。
 */
export interface LegendObject {
    /**
     * 画布绝对像素坐标 `[x, y]`。
     */
    at?: [number, number, ...number[]] | null;
    /**
     * 数据坐标 `[x, y]`。
     */
    at_data?: [number, number, ...number[]] | null;
    /**
     * `outside_bottom_columns` 布局的最大列数。
     */
    col_limit?: number | null;
    /**
     * 手工给出的图例条目，绕过自动收集。用于「颜色编码在数据里、图例得自己写」的场景
     * （例如散点带图的逐点颜色）。给了它，自动收集的条目就不再生效。
     */
    entries?: EntryElement[] | null;
    /**
     * 最多显示多少条，超出折叠成「… (+N more)」。
     */
    entry_limit?: number | null;
    /**
     * 分组图例：每项一组，组有粗体标题、各带自己的条目。**优先级最高** ——
     * 给了它，`entries` 与自动收集都不再生效（与 kuva 的 `with_legend_group` 一致）。
     */
    groups?: LegendGroup[] | null;
    height?: number | null;
    /**
     * 位置具名值，见 `convert::enums::legend_position` 支持的字符串。
     */
    position?: null | string;
    show?:     boolean | null;
    /**
     * 是否给图例画外框。
     */
    show_box?: boolean | null;
    title?:    null | string;
    width?:    number | null;
    wrap?:     number | null;
    [property: string]: unknown;
}

/**
 * 一个图例条目。
 */
export interface EntryElement {
    color: string;
    /**
     * 虚线样式（`"line"` 形状时用得上）。
     */
    dasharray?: null | string;
    label:      string;
    /**
     * 符号形状：`"rect"` / `"line"` / `"circle"`，或 `{"marker": "triangle"}`
     * （用散点的 marker 形状）、`{"size": 6}`（大小不同的圆）。
     */
    shape?: ShapeShapeEnum | null | ShapeObject;
    [property: string]: unknown;
}

export type ShapeShapeEnum = "rect" | "line" | "circle";

/**
 * 尺寸图例：圆的大小。
 */
export interface ShapeObject {
    marker?: Marker;
    size?:   number;
    [property: string]: unknown;
}

export type Marker = "circle" | "square" | "triangle" | "diamond" | "cross" | "plus";

/**
 * 图例里的一组条目：粗体标题 + 若干条目。
 */
export interface LegendGroup {
    entries?: EntryElement[];
    title:    string;
    [property: string]: unknown;
}

export type PaletteEnum = "wong" | "okabe_ito" | "tol_bright" | "tol_muted" | "tol_light" | "ibm" | "deuteranopia" | "protanopia" | "tritanopia" | "category10" | "pastel" | "bold";

/**
 * 一张图里叠加的 series。`type` 决定用哪个变体。
 *
 * 散点图。
 *
 * 折线图。
 *
 * 柱状图。两种写法：简单（`categories` + `values`）与分组 / 堆叠（`categories` + `series`）。
 *
 * 直方图：给 `values` 自动分箱（`bins` / `range` 可选），或给 `edges` + `counts` 直接喂分箱结果。
 *
 * 箱线图。
 *
 * 饼图 / 环形图。
 *
 * 小提琴图：每个类别一条核密度曲线，宽度编码该处的密度。
 *
 * 山脊图（ridgeline）：一组分布按纵向堆叠、彼此重叠。
 *
 * 雨云图（raincloud）：半小提琴 + 箱线 + 抖动散点三件套。
 *
 * 散点带图（strip / beeswarm）：把每个观测值直接画成一个点。
 *
 * 点图（dot plot）：类别 × 类别的网格，点的大小与颜色各编码一个连续量。
 *
 * 棒棒糖图（lollipop）：从基线到每个值画一根杆 + 一个点。
 *
 * 密度曲线图：核密度估计（或直接给一条预计算的曲线）。
 *
 * ECDF 图（经验累积分布）：每个类别一条阶梯曲线。
 *
 * QQ 图（分位数-分位数）：两组分布对齐到同一条参考线上。
 *
 * 森林图（forest plot）：逐行的点估计 + 置信区间。
 *
 * PR 图（precision-recall 曲线）：按分类阈值扫出的精确率-召回率曲线。
 *
 * ROC 图：按分类阈值扫出的真阳率-假阳率曲线。
 *
 * 生存曲线（Kaplan-Meier）：时间是数值、事件是布尔（`true` = 发生了事件）。
 *
 * 火山图：x 是 log2 倍数变化，y 是 -log10(p 值)。
 *
 * 热力图：行 × 列的数值矩阵，用连续色图编码。
 *
 * 二维直方图：把散点分到 `bins_x` × `bins_y` 的网格里。
 *
 * 六边形分箱图（hexbin）：散点按六边形网格聚合，可选第三变量。
 *
 * 聚类热图（clustermap）：热图 + 行列的层次聚类树 + 注释色条。
 *
 * 等高线图：网格值 -> 等值线（或填色）。
 *
 * 三元图（ternary）：三个分量之和为 1 的散点。
 *
 * 极坐标图：角度在圆周、半径在半径上。
 *
 * 骰子图（dice plot）：类别 × 类别的网格，每格画 n 个点（点 = 计数、颜色 = 另一个量）。
 *
 * 三维散点图。
 *
 * 三维曲面图。
 *
 * 桑基图：节点 + 带权重的流向。
 *
 * 和弦图：N×N 的流矩阵画成环上的带子。
 *
 * 网络图：节点 + 边，布局可选力导向 / Kamada-Kawai / 圆周。
 *
 * 矩形树图 / 旭日图：同一套「森林 + 叶子数值」模型，只是一个铺成矩形、一个铺成圆环。
 *
 * 旭日图（旭日 / 环形树图）。数据模型与 `treemap` 完全一样。
 *
 * 韦恩图：2~4 个集合的交叠区域。
 *
 * UpSet 图：集合交集的条形图 + 点矩阵。
 *
 * 华夫图：把占比摊成固定数量的格子。
 *
 * 马赛克图（列 × 行的长表，宽度与高度正比于数值）。
 *
 * 系统发育树。树的输入（`newick` / `edges` / `dist` / `linkage` 四选一）平铺在本对象上。
 *
 * 共线性图（synteny）：两条或多条序列上的区块对应关系。
 *
 * K 线图（蜡烛图）+ 可选的成交量副图。
 *
 * 日历热力图：一年 365（或 366）天，每天一格。
 *
 * 甘特图：任务的时间条 + 里程碑 + 可选的分组表头。
 *
 * Horizon 图：多条时间序列叠成「分层」的带状图。
 *
 * 曼哈顿图：按染色体排布的基因组关联结果。
 *
 * 瀑布图：逐项的增量堆积，末了给一个累计。
 *
 * 凹凸图（bump chart）：名次随时间变化的折线。
 *
 * 帕累托图：按值降序的柱 + 累计百分比折线。
 *
 * 砖墙图（brick plot）：一串字符画成一格格「砖」，常用于展示序列 motif。
 *
 * 漏斗图：一层层的转化率；`mirror` 给的是背靠背的发散版。
 *
 * 斜率图：两列数值之间的变化，每一行一个对象。
 *
 * 人口金字塔：左右两列数值背靠背。
 *
 * 序列图：等间距的 y 值序列，画成点 / 线 / 两者。
 *
 * 雷达图（蜘蛛图）：多条闭合多边形共用一组辐射轴。
 *
 * 平行坐标：每行一个观测、每列一根垂直轴。
 *
 * 堆叠面积图：多个系列在同一批 x 上堆起来。
 *
 * 河流图（streamgraph）：堆叠面积图 + 内部排序 + 流的内联标签。
 *
 * 带状区间图：两条边界之间填色（置信带、误差范围、正常区间…）。
 *
 * 文字块图：把一段带轻量标记的正文排成一张图（`#` 标题、`**粗体**`、`---` 分隔线）。
 *
 * 独立图例图：把一组图例条目单独画出来（适合拼进报告或幻灯片）。
 *
 * 向量场图（quiver）：在网格上画箭头，方向与长度编码 (u, v)。
 *
 * 南丁格尔玫瑰图（coxcomb）：极坐标下的柱状图，扇形面积（或半径）正比于数值。
 *
 * 联合分布图（jointplot）：散点 + 顶部 / 右侧的边缘分布。
 */
export interface SecondarySeryElement {
    type:  Type;
    band?: null | BandObject;
    /**
     * 主色（CSS 颜色字符串）。
     *
     * 任务条的默认颜色。
     *
     * 统一颜色。
     *
     * 没有分组时的回退色。
     *
     * 没有单独指定颜色的扇区/系列用这个颜色。
     */
    color?: null | string;
    /**
     * 逐点颜色。
     *
     * 简单模式下的逐类颜色。
     *
     * 逐组颜色。
     *
     * 逐组颜色（按位置匹配 `groups`）。
     *
     * 矩阵写法的颜色编码值，形状须与 `sizes` 一致。
     *
     * 逐点颜色，长度须与 `data` 一致。
     *
     * 逐节点颜色；不给就按调色板轮转。
     *
     * 逐集合颜色；不给就按调色板轮转。
     */
    colors?: Array<number[] | string> | null;
    /**
     * 行优先矩阵：`data[row][col]`。所有行必须等长。
     *
     * 原始散点，`[x, y]` 或 `{"x":…, "y":…}`。越界的点会被丢掉。
     *
     * 三元组 `[x, y, z]` 或 `{"x":…, "y":…, "z":…}`。
     *
     * 逐日 `(日期, 数值)`；日期格式 `"YYYY-MM-DD"`，解析不了的会被丢掉。
     */
    data?: Array<[number, number, ...number[]] | DatumObject>;
    /**
     * 交互 SVG 里的分组名（不进图例）。
     */
    group_name?: null | string;
    /**
     * 图例条目文字。给了才会在图例里出现。
     *
     * 这个图型没有统一的 `color`（颜色由 `color_map` 连续编码），所以这里不平铺 `CommonStyle`。
     * 色条图的标题。
     *
     * 色条图的标题。
     *
     * 图例标题。
     *
     * 给了就画图例（当前用作图例的标题）。
     *
     * 画图例。
     *
     * 显示图例。
     *
     * 图例文字。
     *
     * 图例标题（分组名）。
     */
    legend?: boolean | null | string;
    /**
     * marker 形状：`"circle"` / `"square"` / `"triangle"` / `"diamond"` / `"cross"` / `"plus"`。
     */
    marker?:              Marker | null;
    marker_opacity?:      number | null;
    marker_stroke_width?: number | null;
    /**
     * 统一的点半径（像素）。
     */
    size?: number | null;
    /**
     * 逐点半径（气泡图），会覆盖 `size`。
     *
     * 矩阵写法的大小编码值，行数须等于 `y_categories`，列数须等于 `x_categories`。
     *
     * 逐点半径（气泡），长度须与 `data` 一致。
     */
    sizes?: Array<number[] | number> | null;
    /**
     * 每个数据点的提示文字。
     *
     * 每个格子的提示文字，按行优先顺序。
     *
     * 每个点的提示文字。
     */
    tooltip_labels?: string[] | null;
    /**
     * 打开悬停提示。
     *
     * 这个图型没有统一的 `color` / `legend`：颜色由 `color_map` 连续编码，图例是尺寸图例 +
     * 色条图，所以这里不平铺 [`super::common::CommonStyle`]。
     * 打开悬停提示。
     *
     * 打开悬停提示（默认开）。
     */
    tooltips?: boolean | null;
    trend?:    RenderSchema | null | TrendObject;
    /**
     * 线下填充成面积。
     */
    fill?: boolean | null;
    /**
     * 点的填充不透明度（`null` 表示不填充）。
     */
    fill_opacity?: number | null;
    /**
     * 线型：`"solid"` / `"dashed"` / `"dotted"` / `"dash_dot"`，或自定义 `stroke-dasharray` 字符串。
     */
    line_style?: null | string;
    /**
     * 阶梯线（只在数据点处转折）。
     */
    step?: boolean | null;
    /**
     * 顶边线宽。
     */
    stroke_width?: number | null;
    /**
     * 各类别的占比。不给颜色就按调色板轮转。
     *
     * 逐个类目（给的是**非累计**的值，累计由 kuva 算）。
     */
    categories?:      Array<CategoryObject | string> | null;
    error_cap_width?: number | null;
    error_color?:     null | string;
    /**
     * 与柱一一对应的误差。
     */
    errors?: Array<[number, number, ...number[]] | number> | null;
    /**
     * 柱间距占比，等价于 `1 - width`。
     *
     * 组间距占比。
     *
     * 格子之间的缝（0~0.5）。
     *
     * 块之间的留白。
     *
     * 蜡烛之间的间距占比。
     *
     * 柱间距占比。
     *
     * 相邻扇区之间的角度间隙。
     */
    gap?: number | null;
    /**
     * 横向柱状图。
     *
     * 横向（值在 x 轴上）。
     *
     * 横向（类目在 y 轴上）。
     */
    horizontal?: boolean | null;
    /**
     * 分组 / 堆叠模式：每根柱一个系列。
     *
     * 一条条曲线 / 一组散点。
     *
     * 一条条小时间序列。
     *
     * 逐个时间点的名次（`null` = 该时刻缺席，图上会断开）。
     *
     * 两种模式二选一：
     * - 直接给名次（`ranks`）；
     * - 给原始数值（`values`），由 kuva 自动排名 —— 同一组里只有 `values` 的系列参与排名。
     *
     * 逐个系列（典型是不同年份）。
     *
     * 年龄组只由**第一个** series 决定，所以各 series 的 `groups` 应当对齐。
     *
     * 逐个多边形；`values` 的下标对应 `axes`，长度应一致。
     *
     * 逐个系列；`values` 的长度应与 `x` 一致（短了按 0 补）。
     *
     * 多系列写法：一个系列一条。给了它就**代替** `slices`。
     */
    series?:  SecondarySerySeries[] | null;
    stacked?: boolean | null;
    /**
     * 原始观测值（走核密度估计，至少 2 个）。
     *
     * 块里写数值。
     *
     * 纵排的 y 值；x 由下标隐式决定（`x = i`）。
     */
    values?: number[] | boolean | null;
    /**
     * 柱宽占类别槽的比例（0~1）。
     *
     * 组宽占类别槽位的比例。
     *
     * 柱宽占槽位的比例。
     */
    width?: number | null;
    /**
     * 直方图的箱数（**至少 1**；给 0 会在归一化时除零）。
     */
    bins?: number | null;
    /**
     * 预分箱的计数。
     *
     * 在区域里写数量。
     *
     * 在条上写数量。
     */
    counts?: number[] | boolean | null;
    /**
     * 预分箱的箱边界。
     *
     * 边。`source` / `target` 写节点名。
     *
     * 边表：`(父, 子, 枝长)`；根是「从没当过子」的那个节点。
     */
    edges?: Array<number | EdgeEdge> | null;
    /**
     * 叠加 KDE 曲线。
     */
    kde?:           boolean | null;
    kde_bandwidth?: number | null;
    kde_color?:     null | string;
    /**
     * 密度曲线的采样点数。
     *
     * 密度曲线采样点数。
     */
    kde_samples?: number | null;
    /**
     * 归一化成比例（y 轴到 1）。
     *
     * 归一化到同一峰值高度。
     *
     * 归一化到最大计数。
     *
     * 把每组内部归一化到和为 1。
     *
     * 每列归一到满高（关掉则各列按自己的总量）。
     *
     * 按占总人口的百分比画。
     *
     * 每根轴各自归一到 0~1。
     *
     * 每根轴各自归一到 0~1（默认开）。
     */
    normalize?: boolean | null;
    /**
     * 共享值域（不给就按数据推导）。
     */
    range?: [number, number, ...number[]] | null;
    /**
     * 每个类别一组原始观测值（kuva 自己算密度）。
     *
     * 每条山脊一组观测值；`color` 逐组生效（这个图型没有统一的 `color` 字段）。
     *
     * 一个模型一条曲线。
     *
     * 逐个数据组；x 与 y 必须等长。
     */
    groups?: SecondarySeryGroup[];
    /**
     * 缺口箱线（notch）。
     */
    notch?:         boolean | null;
    notch_depth?:   number | null;
    notch_width?:   number | null;
    overlay_color?: null | string;
    overlay_size?:  number | null;
    /**
     * 叠加抖动散点，值为抖动幅度。
     */
    strip?: number | null;
    /**
     * 叠加蜂群（swarm）散点。
     */
    swarm?: boolean | null;
    /**
     * 内半径（像素）：`> 0` 就是环形图。
     *
     * 内半径占外半径的比例，内部夹到 `[0, 0.95]`。
     */
    inner_radius?: number | null;
    /**
     * 标签位置：`"auto"` / `"inside"` / `"outside"` / `"none"`。
     */
    label_position?: LabelPositionEnum | null;
    /**
     * 小于该占比的扇区不标标签。
     */
    min_label_fraction?: number | null;
    /**
     * 标签后缀百分比。
     */
    percent?: boolean | null;
    /**
     * 单系列写法：`(标签, 数值)` 逐个扇区。
     */
    slices?: SliceElement[];
    /**
     * 核密度带宽；缺省用 Silverman 规则。
     *
     * 核密度的带宽；`marginal_type = "density"` 时建议显式给。
     */
    bandwidth?: number | null;
    /**
     * 对开小提琴：`split_groups` 给出另一侧的值，逐位置与 `groups` 配对。
     */
    split?:              boolean | null;
    split_color?:        null | string;
    split_group_colors?: string[] | null;
    split_groups?:       SplitGroupElement[] | null;
    /**
     * 对开另一半的图例文字。
     */
    split_legend?: null | string;
    /**
     * 画基线。
     *
     * 杆的起点。
     *
     * 基准线：`y` 高于它是正区、低于它是负区。
     */
    baseline?: boolean | number | BaselineEnum | null;
    /**
     * 填充曲线下方（`false` 只画轮廓线）。
     *
     * 填充曲线下方。
     *
     * 填色（只画等值线时为 `false`）。
     *
     * 填充多边形内部。
     */
    filled?: boolean | null;
    /**
     * 虚线样式，如 `"4 2"`。
     */
    line_dash?: null | string;
    /**
     * 填充不透明度。
     *
     * 填充不透明度（不能为负 —— kuva 会拿它去拼颜色串，负值直接 panic）。
     */
    opacity?: number | null;
    /**
     * 相邻山脊的重叠程度（0~1）。
     */
    overlap?: number | null;
    /**
     * 画图例（图例文字取自各组标签）。
     *
     * 显示图例（图例文字取自各点的 `group`）。
     *
     * 显示图例。
     *
     * 画图例。
     */
    show_legend?: boolean | null;
    /**
     * 带宽的额外倍率（在自动带宽上再乘）。
     */
    bandwidth_scale?: number | null;
    /**
     * 中间箱线图的宽度。
     */
    box_width?: number | null;
    /**
     * 云的不透明度。
     */
    cloud_alpha?: number | null;
    /**
     * 云相对组中心的偏移。
     */
    cloud_offset?: number | null;
    /**
     * 「云」（半小提琴）的宽度。
     */
    cloud_width?: number | null;
    /**
     * 上下翻转（把雨放到云的另一侧）。
     */
    flip?:       boolean | null;
    rain_alpha?: number | null;
    /**
     * 雨的抖动幅度。
     */
    rain_jitter?: number | null;
    /**
     * 雨相对组中心的偏移。
     */
    rain_offset?: number | null;
    /**
     * 「雨」（抖动散点）的点半径。
     */
    rain_size?: number | null;
    /**
     * 抖动与蜂群的随机种子（保证可复现）。
     *
     * 抖动的随机种子（保证可复现）。
     */
    seed?: number | null;
    /**
     * 画箱线。
     *
     * 画立方体外框。
     *
     * 给外框。
     */
    show_box?: boolean | null;
    /**
     * 画云。
     */
    show_cloud?: boolean | null;
    /**
     * 画雨。
     */
    show_rain?: boolean | null;
    /**
     * 点半径。
     */
    point_size?: number | null;
    /**
     * 摆点方式：`"strip"`（带抖动）/ `"swarm"`（蜂群）/ `"center"`（不抖动），
     * 或 `{"jitter": 0.3}` 只给抖动幅度。
     */
    style?: StyleStyleEnum | null | StyleObject;
    /**
     * 连续色图（见 `ColorMapSpec`）。
     *
     * 按模长上色（覆盖 `color`）。
     */
    color_map?: null | string;
    /**
     * 颜色编码值的取值区间。
     *
     * 模长归一区间。
     */
    color_range?: [number, number, ...number[]] | null;
    /**
     * 色条图的标题。
     */
    colorbar_label?: null | string;
    max_radius?:     number | null;
    min_radius?:     number | null;
    /**
     * 稀疏写法：每个点给 `(x 类别, y 类别, 大小编码值, 颜色编码值)`。
     * 类别表按首次出现顺序自动收集。
     *
     * 逐基因/逐项：`name` 用来标显著项。
     *
     * 散点写法：`(x, y, z)` 三元组，kuva 自己三角剖分。与 `z` 二选一。
     *
     * 逐格数据（三种输入写法之一）。
     *
     * 逐行：`(标签, 左值, 右值)`；行的先后就是 y 轴自上而下的顺序。
     */
    points?: Array<[number, number, number, ...number[]] | PointObject> | null;
    /**
     * 尺寸图例的标题。
     */
    size_label?: null | string;
    /**
     * 点半径的取值区间（编码值 -> 像素）。
     *
     * 点大小的取值区间。
     */
    size_range?: [number, number, ...number[]] | null;
    /**
     * 矩阵写法的列类别（x 轴）。
     *
     * 列类别（x 轴）。`points` 写法必填；另外两种不给我会自己收集。
     */
    x_categories?: string[] | null;
    /**
     * 矩阵写法的行类别（y 轴）。
     *
     * 行类别（y 轴）。同上。
     */
    y_categories?:   string[] | null;
    baseline_color?: null | string;
    /**
     * 基线虚线样式，如 `"4 2"`。
     */
    baseline_dash?:  null | string;
    baseline_width?: number | null;
    /**
     * 区间带的高度。
     */
    domain_height?: number | null;
    /**
     * 背景的区间带（如「正常范围」）。
     */
    domains?: DomainElement[];
    /**
     * 端点半径。
     *
     * 点半径（`0` = 按格自动）。
     */
    dot_radius?: number | null;
    /**
     * 端点描边颜色。
     */
    dot_stroke?:       null | string;
    dot_stroke_width?: number | null;
    /**
     * 画基线。
     *
     * 画随机基线（`prevalence` 水平线）。
     */
    show_baseline?: boolean | null;
    stem_width?:    number | null;
    /**
     * 预计算的曲线，给了它就跳过估计。
     */
    curve?: null | CurveObject;
    /**
     * 在图上标出拟合优度。
     */
    fit?: boolean | null;
    /**
     * 只钉上界：在 `hi` 处反射，下侧的尾巴仍然自由。
     */
    x_hi?: number | null;
    /**
     * 只钉下界：在 `lo` 处反射，上侧的尾巴仍然自由。
     */
    x_lo?: number | null;
    /**
     * 只画 `[lo, hi]` 这一段（两侧同时反射）。
     *
     * 只画这一段 x 范围（默认 `[0.5, 列数+0.5]`）。
     *
     * x 的取值区间（分箱边界），必填。
     */
    x_range?: [number, number, ...number[]] | null;
    /**
     * 置信带的不透明度。
     */
    band_alpha?: number | null;
    /**
     * 画互补累积分布（从 1 往下走）。
     */
    complementary?: boolean | null;
    /**
     * 置信带。
     */
    confidence_band?: boolean | null;
    /**
     * 各组共用的点半径。
     */
    marker_size?: number | null;
    /**
     * 在观测点上打点。
     */
    markers?: boolean | null;
    /**
     * 画这些分位数的水平参考线。给的是 **0~1 的 F 值**，如 `[0.25, 0.5, 0.75]`。
     */
    percentile_lines?: number[] | null;
    /**
     * 在轴上画每个观测的短竖线（rug）。
     */
    rug?: boolean | null;
    /**
     * rug 的高度（像素）。
     */
    rug_height?: number | null;
    /**
     * 平滑阶梯曲线。
     *
     * 平滑（默认开）；`false` 就是折线。
     */
    smooth?: boolean | null;
    /**
     * 平滑时的采样点数。
     */
    smooth_samples?: number | null;
    /**
     * 置信带的不透明度。
     */
    ci_alpha?: number | null;
    /**
     * 置信带。
     */
    ci_band?: boolean | null;
    /**
     * 标出 lambda（斜率/截距）。`genomic` 模式下默认关。
     */
    lambda?: boolean | null;
    /**
     * `"normal"`（默认）对比理论正态分布；`"genomic"` 画 -log10(p) 的 QQ。
     *
     * 多系列的摆法，默认 `stacked`。
     */
    mode?: SecondarySeryMode | null;
    /**
     * 画参考线。
     */
    reference_line?: boolean | null;
    /**
     * 区间两端的端帽宽度（0 = 不画端帽）。
     */
    cap_size?: number | null;
    /**
     * 零效应参考线的位置。
     */
    null_value?: number | null;
    /**
     * 网格行数，默认 10。
     *
     * 逐个观测行；`values` 的下标对应 `axis_names`。
     */
    rows?: RowElement[] | number | null;
    /**
     * 画零效应参考线。
     */
    show_null_line?: boolean | null;
    /**
     * 置信区间横线的粗细。
     */
    whisker_width?: number | null;
    /**
     * 基线虚线样式。
     */
    baseline_dasharray?: null | string;
    diagonal_color?:     null | string;
    /**
     * 对角线虚线样式。
     */
    diagonal_dasharray?: null | string;
    /**
     * 画随机猜测的对角线。
     */
    show_diagonal?: boolean | null;
    /**
     * 删失观测打叉。
     */
    censoring?: boolean | null;
    /**
     * 删失标记的尺寸。
     */
    censoring_size?: number | null;
    /**
     * 置信带。
     */
    ci?:         boolean | null;
    line_width?: number | null;
    /**
     * 在图上写一行文字（典型是 log-rank 的 p 值；kuva 不代算，得自己算好传进来）。
     */
    pvalue_text?: null | string;
    /**
     * 显著下调点的颜色。
     *
     * 收盘 < 开盘时的颜色。
     *
     * 下降的颜色。
     */
    color_down?: null | string;
    /**
     * 不显著点的颜色。
     */
    color_ns?: null | string;
    /**
     * 显著上调点的颜色。
     *
     * 收盘 > 开盘时的颜色。
     *
     * 上升的颜色。
     */
    color_up?: null | string;
    /**
     * 倍数变化阈值（取绝对值）。
     */
    fc_cutoff?: number | null;
    /**
     * 标签避让方式：`"nudge"`（默认）/ `"exact"` / `{"offset_x":…, "offset_y":…}`。
     */
    label_style?: LabelStyleEnum | null | LabelStyleObject;
    /**
     * 标出最显著的多少个点（`0` = 全不标）。
     */
    label_top?: number | null;
    /**
     * p 值阈值。
     */
    p_cutoff?: number | null;
    /**
     * p 值的下限（避免 `log10(0)`）。缺省取数据里最小的非零 p 值。
     *
     * p 值的下限（避免 `log10(0)`）。
     */
    pvalue_floor?: number | null;
    /**
     * 格子占槽位的比例，内部夹到 `[0.5, 1]`。
     *
     * 格子边长（像素）。
     */
    cell_size?: number | null;
    /**
     * 列标签（x 轴，从左往右）。
     *
     * 列标签，长度须等于列数。
     */
    col_labels?: string[] | null;
    /**
     * 行标签（y 轴，从下往上）。
     *
     * 行标签，长度须等于行数。
     */
    row_labels?: string[] | null;
    /**
     * 在格子里写数值。
     *
     * 在柱上标数值。
     *
     * 标出序列里的字符。
     *
     * 在层里写数值。
     *
     * 标出变化前后的数值。
     *
     * 在扇区末端标出数值。
     */
    show_values?: boolean | null;
    /**
     * 只画这一段 y 范围。
     *
     * y 的取值区间（分箱边界），必填。
     */
    y_range?: [number, number, ...number[]] | null;
    /**
     * x 方向的箱数，默认 10。
     */
    bins_x?: number | null;
    /**
     * y 方向的箱数，默认 10。
     */
    bins_y?: number | null;
    /**
     * 在图上标出相关系数 r。
     */
    correlation?: boolean | null;
    /**
     * 计数取对数（长尾时好看）。
     */
    log_count?: boolean | null;
    /**
     * 直接指定六边形边长（给了它就覆盖 `n_bins`）。
     */
    bin_size?: number | null;
    /**
     * 画色条。
     */
    colorbar?: boolean | null;
    /**
     * 平顶六边形（默认尖顶）。
     */
    flat_top?: boolean | null;
    /**
     * 颜色取对数。
     */
    log_color?: boolean | null;
    /**
     * 少于这个计数的格子不画。
     */
    min_count?: number | null;
    /**
     * 六边形的边数（分箱密度），默认 20。
     */
    n_bins?: number | null;
    /**
     * `z` 的汇总方式，默认 `count`。
     */
    reduce?: ReduceEnum | null;
    /**
     * 六边形描边颜色。
     */
    stroke?: null | string;
    /**
     * 共享的 x（通常是时间）。
     *
     * 共享的 x。
     */
    x?: number[];
    y?: number[];
    /**
     * 第三变量（长度须与 `x` 一致）。不给就用点数。
     *
     * 网格写法：`z[row][col]`，`x_coords` 给出每列的 x、`y_coords` 给出每行的 y。
     * 与 `points` 二选一。
     */
    z?: Array<number[] | number> | null;
    /**
     * 树枝颜色。
     */
    branch_color?: null | string;
    /**
     * 对列做层次聚类（默认开）。
     */
    cluster_cols?: boolean | null;
    /**
     * 对行做层次聚类（默认开）。
     */
    cluster_rows?: boolean | null;
    /**
     * 紧贴矩阵上方的注释色条。
     */
    col_annotations?: ColAnnotationElement[];
    /**
     * 列树的高度。
     */
    col_dendrogram_height?: number | null;
    /**
     * 列树：同 `row_tree`，叶子名与 `col_labels` 对应。
     */
    col_tree?: null | ColTreeObject;
    /**
     * 归一化方式，默认 `none`。
     */
    normalization?: NormalizationEnum | null;
    /**
     * 紧贴矩阵左侧的注释色条。
     */
    row_annotations?: ColAnnotationElement[];
    /**
     * 行树（ dendrogram ）的宽度。
     */
    row_dendrogram_width?: number | null;
    /**
     * 行树：给了就用它代替对行的自动聚类（这时 `cluster_rows` 不再起作用）。
     * 写法与 `phylo` 的四种树输入完全一样；叶子名必须与 `row_labels` 对得上。
     */
    row_tree?: null | ColTreeObject;
    /**
     * 显式的等值线数值（给了它就覆盖 `n_levels`）。
     */
    levels?: number[] | null;
    /**
     * 等值线颜色。
     *
     * 累计折线的颜色。
     */
    line_color?: null | string;
    /**
     * 等值线条数，默认 8。
     */
    n_levels?: number | null;
    /**
     * 每列的 x 坐标，长度须等于列数；不给就用列下标。
     */
    x_coords?: number[] | null;
    /**
     * 每行的 y 坐标，长度须等于行数；不给就用行下标。
     */
    y_coords?: number[] | null;
    /**
     * 三个角的标签，顺序是「上 / 左下 / 右下」。
     */
    corner_labels?: string[] | null;
    /**
     * 网格分几份。
     *
     * 画格子的分隔线。
     *
     * 网格线密度。
     *
     * 同心网格环数。
     *
     * 同心环的根数。
     */
    grid_lines?: boolean | number | null;
    /**
     * 画网格。
     *
     * 画同心网格环。
     */
    show_grid?: boolean | null;
    /**
     * 在网格线上标百分比。
     */
    show_percentages?: boolean | null;
    /**
     * 角度增大方向为顺时针（默认开）。
     *
     * 扇区按顺时针排（默认开）。
     */
    clockwise?: boolean | null;
    /**
     * 半径方向的网格线根数。
     */
    r_grid_lines?: number | null;
    /**
     * 半径上限（缺省按数据自动）。
     */
    r_max?: number | null;
    r_min?: number | null;
    /**
     * 标出半径刻度值。
     */
    show_r_labels?: boolean | null;
    /**
     * 圆周分几格。
     */
    theta_divisions?: number | null;
    /**
     * 0° 指向哪个方向（度）。
     */
    theta_start?: number | null;
    /**
     * 每一「点」代表什么（长度须等于 `ndots`），如 `["1","2","3","4"]`。
     */
    category_labels?: string[] | null;
    cell_height?:     number | null;
    /**
     * 格子宽 / 高占槽位的比例。
     */
    cell_width?: number | null;
    /**
     * 分类写法的颜色图例：每项 `[文字, CSS 颜色]`，长度须等于 `ndots`。
     */
    dot_legend?: Array<[string, string, ...string[]]> | null;
    /**
     * 逐点连续写法（三种输入写法之一）：一条记录一个点，各自带填充值与大小值。
     */
    dot_points?: DotPointElement[];
    /**
     * 填充色图例的标题。
     */
    fill_legend_label?: null | string;
    /**
     * 填充色的取值区间。
     */
    fill_range?: [number, number, ...number[]] | null;
    /**
     * 每个格子最多几个点（1~6，默认 4）。
     */
    ndots?: number | null;
    /**
     * 格子之间的留白。
     */
    pad?: number | null;
    /**
     * 点位置（1~n 点）图例的标题。
     */
    position_legend_label?: null | string;
    /**
     * 分类写法（三种输入写法之一）：一条记录一个点，
     * 点位由 `category` 匹配 `category_labels` 得到，颜色直接用 CSS 字符串。
     */
    records?: RecordElement[];
    /**
     * 点大小图例的标题。
     */
    size_legend_label?: null | string;
    /**
     * 方位角（度）。
     */
    azimuth?: number | null;
    /**
     * 按深度调暗（远处的点更淡）。
     */
    depth_shade?: boolean | null;
    /**
     * 仰角（度）。
     */
    elevation?: number | null;
    x_label?:   null | string;
    y_label?:   null | string;
    /**
     * 自动决定 z 轴放哪一侧。
     */
    z_axis_auto?: boolean | null;
    /**
     * z 轴放右侧。
     */
    z_axis_right?: boolean | null;
    /**
     * 按 z 值上色（覆盖逐点颜色）。
     *
     * 按高度上色。
     */
    z_colormap?: null | string;
    z_label?:    null | string;
    /**
     * 曲面不透明度。
     */
    alpha?: number | null;
    /**
     * 画网格线（默认开）。
     */
    wireframe?:       boolean | null;
    wireframe_color?: null | string;
    wireframe_width?: number | null;
    /**
     * 行优先矩阵 `z_data[row][col]`；所有行必须等长，且行列数都 >= 2。
     */
    z_data?: Array<number[]>;
    /**
     * 跨轴的流（每个元素是一条从左到右依次经过的节点名序列）。
     */
    alluvia?: AlluviaElement[];
    /**
     * 每根轴的名字，长度应与 `alluvia.nodes` 一致。
     *
     * 每根轴（列）的名字，**至少 2 个**。
     */
    axis_names?:        string[] | null;
    flow_label_format?: ColorbarTickFormatEnum | number | null;
    /**
     * 太窄的流不标字（像素）。
     */
    flow_label_min_height?: number | null;
    /**
     * 数值后面的单位，如 `"%"`。
     */
    flow_label_unit?: null | string;
    /**
     * 在流上标数值。
     */
    flow_labels?: boolean | null;
    /**
     * 在流上标百分比（优先于 `flow_labels`）。
     */
    flow_percent?: boolean | null;
    /**
     * `node_coloring = "left"` 时，多大流量算「来自左边」。
     */
    left_color_cutoff?: number | null;
    link_color?:        LinkColorEnum | null;
    link_opacity?:      number | null;
    /**
     * 连边。`source` / `target` 写节点的名字（比下标好写也好读）。
     */
    links?:         LinkElement[];
    node_coloring?: NodeColoringEnum | null;
    node_gap?:      number | null;
    node_order?:    NodeOrderEnum | null;
    /**
     * 节点排序算法的随机种子。
     */
    node_order_seed?: number | null;
    node_width?:      number | null;
    /**
     * 节点表。`links` / `alluvia` 里用**名字**引用它们，这里定义了引用的集合。
     */
    nodes?: NodeElement[];
    /**
     * 自定义配色（不给就用调色板）。
     */
    palette?: string[] | null;
    /**
     * 扇区之间的角度间隙。
     */
    gap_degrees?: number | null;
    /**
     * 节点名，长度应等于矩阵的边长；不给就用序号。
     *
     * 逐个扇区的标签（圆周上的类目名）。不给就用方位名。
     */
    labels?: string[] | null;
    /**
     * 流矩阵，必须是方阵：`matrix[i][j]` 是 i→j 的流量。
     */
    matrix?: Array<number[]>;
    /**
     * 内半径占外半径的比例。
     */
    pad_fraction?:   number | null;
    ribbon_opacity?: number | null;
    /**
     * 有向图（画箭头）。
     */
    directed?:     boolean | null;
    edge_opacity?: number | null;
    /**
     * 标签放在节点圆内。
     */
    label_inside?: boolean | null;
    label_size?:   number | null;
    /**
     * 布局算法：`"squarify"`（默认）/ `"slice_dice"` / `"binary"`。
     */
    layout?:      LayoutEnum | null;
    node_radius?: number | null;
    /**
     * 标签之间互相排斥，避免叠在一起。
     */
    repel_labels?: boolean | null;
    /**
     * 标出节点名。
     *
     * 标出叶子标签。
     *
     * 标出扇区标签。
     *
     * 标出任务名。
     *
     * 画内联标签。
     *
     * 在圆周上标出扇区标签。
     */
    show_labels?: boolean | null;
    /**
     * 边框宽度，`0` = 不画边框。
     */
    border_width?: number | null;
    color_mode?:   ColorModeEnum | null | ColorModeObject;
    /**
     * 与叶子（深度优先序）平行的颜色编码值。
     */
    color_values?: number[] | null;
    /**
     * 最多画到第几层。
     */
    max_depth?: number | null;
    /**
     * 小于这个面积（px²）就不标标签。
     */
    min_label_area?: number | null;
    /**
     * 矩形之间的留白。
     *
     * 内边距（像素）。
     */
    padding?: number | null;
    /**
     * 根矩形的外框宽度。
     */
    root_border_width?: number | null;
    /**
     * 森林的根，每个根是一棵树。
     *
     * 森林的根；多个根共用最内环。
     */
    roots?: RootElement[];
    /**
     * 标出内部节点标签。
     */
    show_parent_labels?: boolean | null;
    /**
     * 扇区角度小于该值（度）就不标标签。
     */
    min_label_angle?: number | null;
    /**
     * 圆环之间的缝（像素）。
     */
    ring_gap?: number | null;
    /**
     * 标签顺着圆周旋转。
     */
    rotate_labels?: boolean | null;
    /**
     * 起始角度（度）：`0` = 12 点方向，顺时针。
     *
     * 第一根轴的角度（度，`-90` = 从正北开始）。
     *
     * 起始角度（度，`0` = 12 点方向）。
     */
    start_angle?: number | null;
    /**
     * 集合名与圆之间画引线。
     */
    leader_lines?: boolean | null;
    /**
     * 显示 vennEuler 的布局应力。
     */
    loss?: boolean | null;
    /**
     * 预计算的交集大小；`sets` 里出现了 `elements` 的一律忽略。
     */
    overlaps?: OverlapElement[];
    /**
     * 在区域里写百分比。
     */
    percentages?: boolean | null;
    /**
     * 圆的面积正比于集合大小。
     */
    proportional?: boolean | null;
    /**
     * 在圆里画集合名的首字母。
     */
    set_indicators?: boolean | null;
    /**
     * 标出集合名。
     */
    set_labels?: boolean | null;
    /**
     * 集合。两种写法二选一，不能混用：
     * - 原始元素：`{"label": "A", "elements": ["x", "y"]}`，交叠由 kuva 算；
     * - 预计算：`{"label": "A", "size": 10}` 配 `overlaps`。
     */
    sets?:      SetElement[];
    bar_color?: null | string;
    dot_color?: null | string;
    /**
     * 「不在这个交集里」的那些点的颜色。
     */
    dot_empty_color?: null | string;
    /**
     * 非空交集：`mask` 的第 `i` 位表示 `set_names[i]` 在这个交集里。
     */
    intersections?: IntersectionElement[];
    /**
     * 最多显示多少个交集（其余折叠）。
     */
    max_visible?: number | null;
    set_names?:   string[];
    /**
     * 每个集合的元素数（左侧横条），长度须与 `set_names` 一致。
     */
    set_sizes?: number[];
    /**
     * 画左侧的集合大小条。
     */
    show_set_sizes?: boolean | null;
    sort?:           SortEnum | null;
    /**
     * 网格列数，默认 10。
     *
     * 固定列数。
     */
    cols?: number | null;
    /**
     * 空格的底色。
     */
    empty_color?: null | string;
    fill_order?:  FillOrderEnum | null;
    shape?:       SecondarySeryShape | null;
    /**
     * 在格子里写数量。
     */
    show_counts?: boolean | null;
    /**
     * 在格子里写百分比。
     *
     * 在层里写占入口的百分比。
     */
    show_percents?: boolean | null;
    /**
     * 网格下方的注记，如 `"1 格 = 10 人"`。
     */
    unit_label?: null | string;
    /**
     * 扁平长表，一行一个 `列 × 行 = 数值`；缺失的组合按 0，重复的组合会求和。
     */
    cells?: CellElement[];
    /**
     * 列的顺序（不给就按 `cells` 里首次出现的顺序）。
     */
    col_order?: string[] | null;
    /**
     * 逐行配色（按 `row_order` 的位置）。
     *
     * 逐行的颜色。
     *
     * 逐分组的颜色。
     */
    group_colors?: string[] | null;
    /**
     * 行标签的最小高度（低于它就不画）。
     *
     * 低于这个高度（像素）就不画内联标签。
     */
    min_label_height?: number | null;
    /**
     * 列标签的最小宽度。
     */
    min_label_width?: number | null;
    /**
     * 块里写百分比。
     */
    percents?: boolean | null;
    /**
     * 行的顺序。
     */
    row_order?:    string[] | null;
    branch_style?: BranchStyleEnum | null;
    /**
     * 给某个节点（及其子树）上色。
     */
    clade_colors?: Array<[number | string, number | string, ...(number | string)[]] | CladeColorObject>;
    /**
     * 距离矩阵（UPGMA 聚类）。
     */
    distance_matrix?: null | DistanceMatrixObject;
    leaf_color?:      null | string;
    /**
     * linkage 矩阵：每行 `[左下标, 右下标, 距离, 叶子数]`。
     */
    linkage?: null | LinkageObject;
    /**
     * Newick 字符串，如 `"((A:0.1,B:0.2):0.3,C:0.4);"`。
     */
    newick?:      null | string;
    orientation?: SecondarySeryOrientation | null;
    /**
     * 按累积枝长画（否则各叶子等距，即 cladogram）。
     */
    phylogram?: boolean | null;
    /**
     * 低于该值的支撑值当作噪声，不画。
     */
    support_threshold?: number | null;
    /**
     * 序列条的厚度（像素）。
     *
     * 条的厚度占行高的比例，内部夹到 `[0.1, 1]`。
     */
    bar_height?:    number | null;
    block_opacity?: number | null;
    /**
     * 对应区块。`seq1` / `seq2` 写 `sequences` 的**下标**。
     */
    blocks?: BlockElement[];
    /**
     * 逐序列配色；短于 `sequences` 的部分保持默认色。
     */
    sequence_colors?: string[] | null;
    /**
     * 序列条。
     *
     * 逐行的序列字符串，1 个字符 = 1 格。**给了 `strigars` 就不参与渲染**。
     */
    sequences?: Array<SequenceObject | string>;
    /**
     * 所有序列共用一把标尺（关掉则各自满宽）。
     */
    shared_scale?: boolean | null;
    /**
     * 蜡烛宽度占槽位的比例。
     */
    candle_width?: number | null;
    /**
     * 每根蜡烛一根 K 线，按插入顺序排在分类轴上。
     */
    candles?: CandleElement[];
    /**
     * 开盘 == 收盘（十字星）的颜色。
     */
    color_doji?: null | string;
    /**
     * 画成交量副图（每根蜡烛要带 `volume`）。
     */
    show_volume?: boolean | null;
    /**
     * 成交量副图占整幅图的高度比例。
     */
    volume_ratio?: number | null;
    /**
     * 影线粗细。
     */
    wick_width?: number | null;
    /**
     * 同日多条怎么合并，默认 `count`。
     */
    aggregation?: AggregationEnum | null;
    /**
     * 格子之间的缝（像素）。
     */
    cell_gap?: number | null;
    /**
     * 只画这个日期区间（等价于一个 period）。
     */
    date_range?: null | DateRangeObject;
    /**
     * 标出星期几。
     */
    day_labels?: boolean | null;
    /**
     * 只给日期（数值一律记 1）。
     */
    events?: string[];
    /**
     * 图例标题。
     */
    legend_label?: null | string;
    /**
     * 没有数据的格子的底色。
     */
    missing_color?: null | string;
    /**
     * 标出月份名。
     */
    month_labels?: boolean | null;
    /**
     * 显式的展示区间；给了它就覆盖 `years` / `year`。
     */
    periods?: PeriodElement[];
    /**
     * 色条的取值区间。
     */
    value_range?: [number, number, ...number[]] | null;
    week_start?:  WeekStartEnum | null;
    /**
     * 只画这一年（`years` 的单元素写法）。
     */
    year?: number | null;
    /**
     * 画哪几年（整年，1~12 月）。
     */
    years?: number[] | null;
    /**
     * 数值为 0 的格子的颜色（不给就用色图的最低端）。
     */
    zero_color?: null | string;
    /**
     * 分组表头的底色。
     */
    group_bg?: null | string;
    /**
     * 分组的显示顺序。
     */
    group_order?: string[] | null;
    /**
     * 里程碑标记的大小。
     */
    milestone_size?: number | null;
    /**
     * 在这个时间点上画一条「今天」线。
     */
    now_line?: number | null;
    /**
     * 任务 / 里程碑。
     */
    tasks?: TaskElement[];
    /**
     * 分几层带。
     */
    n_bands?: number | null;
    /**
     * 每条带的高度。
     *
     * 行高（像素）。
     */
    row_height?: number | null;
    /**
     * 正负用不同颜色（关掉就是单色）。
     */
    sign_colors?: boolean | null;
    /**
     * 在每条带上标出数值。
     */
    value_labels?: boolean | null;
    /**
     * 数值上限（用来定分带阈值）。
     */
    value_max?: number | null;
    /**
     * 给了它就用该版本的染色体长度算累积坐标。
     */
    build?: BuildEnum | null;
    /**
     * 奇数位点的颜色。
     */
    color_a?: null | string;
    /**
     * 偶数位点的颜色（默认 `color_a` 的浅一档）。
     */
    color_b?: null | string;
    /**
     * 全基因组显著性线（`-log10(p)`）。
     */
    genome_wide?: number | null;
    /**
     * 提示性显著线。
     */
    suggestive?: number | null;
    /**
     * 柱宽占槽位的比例。
     */
    bar_width?: number | null;
    /**
     * 每一根柱：`delta`（增量）/ `total`（累计）/ `difference`（从 from 变到 to）。
     */
    bars?:           BarElement[];
    color_negative?: null | string;
    color_positive?: null | string;
    color_total?:    null | string;
    /**
     * 画柱之间的连接线。
     */
    connectors?:  boolean | null;
    curve_style?: CurveStyleEnum | null;
    /**
     * 只把这一条画成强调色。
     */
    highlight?: null | string;
    /**
     * 名次小的在上（默认 1 在顶）。
     */
    rank_ascending?: boolean | null;
    /**
     * 在折线上标出每个时间点的名次。
     */
    show_rank_labels?: boolean | null;
    /**
     * 在折线末端标出系列名。
     */
    show_series_labels?: boolean | null;
    tie_break?:          TieBreakEnum | null;
    /**
     * 每个时间点一个 x 轴标签。
     */
    x_labels?: string[] | null;
    /**
     * 柱的图例文字（默认 `"Value"`）。
     */
    bar_legend_label?: null | string;
    /**
     * 在累计线上标百分比。
     */
    cumulative_labels?: boolean | null;
    /**
     * 折线的图例文字（默认 `"Cumulative %"`）。
     */
    line_legend_label?: null | string;
    /**
     * 超过这个类目数就把尾部并成一个「其他」。
     */
    max_categories?: number | null;
    /**
     * 「其他」那一组的名字。
     */
    other_label?: null | string;
    /**
     * 画阈值线。
     */
    show_threshold?: boolean | null;
    /**
     * 按值降序排（默认开；关掉就按给定的顺序）。
     */
    sorted?: boolean | null;
    /**
     * 阈值的累计百分比。
     */
    threshold?: number | null;
    /**
     * 序列的锚点。
     */
    anchor?: AnchorEnum | null;
    /**
     * 把哪一行当共识序列（`0` 起）。
     */
    consensus_row?: number | null;
    /**
     * FLANKED-STRIGAR：`(左侧翼序列, motif, strigar, 右侧翼序列)`。STR 区两侧各接一段原始
     * DNA（一个字符 = 一格，用标准 DNA 配色），省掉「用 `@` 空位把侧翼写进 motif 串」的麻烦。
     */
    flanked_strigars?: Array<[string, string, string, string, ...string[]]>;
    /**
     * 给「主」一条加个标记。
     */
    mark_primary?: boolean | null;
    /**
     * 每行的名字，长度应与生效的那一份数据（`strigars` 优先，否则 `sequences`）一致。
     */
    names?: string[] | null;
    /**
     * 逐行的注记文字。
     */
    notations?: Array<null | string> | null;
    /**
     * 逐行的参考起点（基因组坐标）；kuva 内部就是 `x_offsets` 取负，用来表达「各行从参考上的
     * 哪个位置开始」而不是「跳过多少个字符」。与 `x_offsets` 只能给一个。
     */
    start_positions?: number[] | null;
    /**
     * 结构域的配色。
     */
    strigar_palette?: string[] | null;
    /**
     * STRIGAR：`(motif, strigar)`，见模块文档。给了它就**代替** `sequences` 成为画出来的行。
     */
    strigars?: Array<[string, string, ...string[]]>;
    /**
     * 字符配色；缺省用 `dna`。**这个必须有** —— kuva 拿不到配色表时会直接 panic。
     */
    template?: TemplateEnum | { [key: string]: string } | null;
    /**
     * 所有行共用的 x 起点。
     */
    x_offset?: number | null;
    /**
     * 逐行的 x 起点（不给就用 `x_offset`）。
     */
    x_offsets?: Array<number | null> | null;
    /**
     * 坐标原点。
     */
    x_origin?:          number | null;
    connector_opacity?: number | null;
    /**
     * 发散模式两侧的标签。
     */
    left_label?: null | string;
    /**
     * 背靠背的右侧一组（发散漏斗）。
     */
    mirror?:      StageElement[] | null;
    right_label?: null | string;
    /**
     * 画层与层之间的连接带。
     */
    show_connectors?: boolean | null;
    /**
     * 在层之间写转化率。
     */
    show_conversion?: boolean | null;
    /**
     * 层与层之间的缝。
     */
    stage_gap?: number | null;
    /**
     * 逐层转化，有序（第一层是入口）。
     */
    stages?: StageElement[];
    /**
     * 右列的轴标题。
     */
    after_label?: null | string;
    /**
     * 左列的轴标题。
     */
    before_label?: null | string;
    /**
     * 按方向上色（关掉就用统一的 `color`）。
     */
    color_by_direction?: boolean | null;
    /**
     * 不变时的颜色。
     */
    color_flat?:   null | string;
    dot_opacity?:  number | null;
    line_opacity?: number | null;
    /**
     * 数值的显示格式。
     */
    value_format?: ColorbarTickFormatEnum | number | null;
    /**
     * 同一组内两根柱之间的间距。
     */
    bar_gap?: number | null;
    /**
     * 年龄组之间的间距。
     */
    group_gap?:   number | null;
    left_color?:  null | string;
    right_color?: null | string;
    /**
     * 点的半径（像素）。
     */
    point_radius?: number | null;
    /**
     * 轴名，顺时针排列，**至少 3 个**。
     */
    axes?: string[];
    /**
     * 逐轴的值域覆盖：`[轴下标, (min, max)]`。
     */
    axis_ranges?: Array<[[number, number, ...number[]] | number, [number, number, ...number[]] | number, ...([number, number, ...number[]] | number)[]]>;
    /**
     * 画轴上的刻度线。
     *
     * 每根轴的刻度数。
     */
    axis_ticks?: boolean | number | null;
    /**
     * 网格环画成圆（默认是多边形）。
     */
    circular_grid?: boolean | null;
    /**
     * 顶点圆点半径；不给就不画点。
     */
    dot_size?: number | null;
    /**
     * 逐轴反转：`[0, 2]` 表示反转第 0 与第 2 根轴。
     *
     * 逐轴反转（把大值放到下面）。
     */
    inverted_axes?: number[];
    /**
     * 虚线参考多边形（例如目标值）。
     */
    references?: ReferenceElement[];
    /**
     * 第一根轴用哪一根（配合 `axes` 轮转）。
     */
    start_axis?: number | null;
    /**
     * 在顶点标出数值。
     */
    vertex_labels?: boolean | null;
    /**
     * 用贝塞尔曲线连接（默认是直线）。
     */
    curved?:            boolean | null;
    mean_stroke_width?: number | null;
    /**
     * 在每根轴后面画一条灰带。
     */
    show_axis_bands?: boolean | null;
    /**
     * 画轴上的刻度线。
     */
    show_axis_ticks?: boolean | null;
    /**
     * 画出各组的均值线。
     */
    show_mean?: boolean | null;
    /**
     * 图例位置字符串（见 `convert::enums::legend_position`）。
     *
     * 图例位置字符串。
     */
    legend_position?: null | string;
    /**
     * 每列归一到 100%。
     */
    normalized?: boolean | null;
    /**
     * 画顶边。
     */
    show_strokes?: boolean | null;
    order?:        OrderEnum | null;
    /**
     * 画层与层之间的描边。
     */
    stroke_between?: boolean | null;
    /**
     * 下边界，长度应与 `x` 一致。
     */
    y_lower?: number[];
    /**
     * 上边界，长度应与 `x` 一致。
     */
    y_upper?: number[];
    /**
     * 背景色。
     */
    background?: null | string;
    /**
     * 正文。行级标记：`#` / `##` 标题、`**粗体**`、`---` 分隔线、空行分段。
     */
    body?: string;
    /**
     * 边框颜色（`null` / 不给 = 浅灰）。
     */
    border_color?: null | string;
    /**
     * 正文字号（不能为 0 —— kuva 用它去除字符宽度，0 会除零 panic）。
     */
    font_size?:  number | null;
    text_align?: TextAlignEnum | null;
    text_color?: null | string;
    /**
     * 图上方的标题。
     *
     * 图例标题（粗体）。
     */
    title?:   null | string;
    entries?: EntryElement[];
    /**
     * 自动布局时的列数上限。
     */
    max_cols?: number | null;
    /**
     * 最多显示几条（**至少 1** —— kuva 内部算 `n - 1`，给 0 会下溢 panic）。
     */
    max_entries?: number | null;
    /**
     * 逐支箭：尾点 `(x, y)` 与位移 `(u, v)`，都是数据坐标。
     * 分量非有限的箭会被 kuva 静默丢弃。
     */
    arrows?: ArrowElement[];
    /**
     * 自动缩放时箭头占画面的目标比例。
     */
    auto_scale_fraction?: number | null;
    /**
     * 把箭头裁在绘图区内。
     */
    clip_to_plot_area?: boolean | null;
    /**
     * 色条的标题。
     */
    color_legend_label?: null | string;
    /**
     * 箭头半宽 / 箭头长。
     */
    head_aspect?: number | null;
    /**
     * 箭头长度（像素）；不给就按 `head_ratio` 与杆长算。
     */
    head_length?: number | null;
    head_max_px?: number | null;
    /**
     * 箭头长度的上下限（像素）。
     */
    head_min_px?: number | null;
    /**
     * 箭头长 / 杆长。
     */
    head_ratio?: number | null;
    /**
     * 箭头半宽（像素）。
     */
    head_width?: number | null;
    pivot?:      PivotEnum | null;
    /**
     * `(u, v)` 的乘数；不给就按数据自动缩放。
     */
    scale?: number | null;
    /**
     * 杆的宽度（像素）。
     */
    shaft_width?: number | null;
    /**
     * 只用尾点定坐标范围（否则算上箭头末端）。
     */
    tight_bounds?: boolean | null;
    /**
     * 方位角写法：原始罗盘方位（0–360°）逐个给，由 kuva 分成 `bearings_bins` 个扇区并计数。
     * 给了它就**代替** `slices` / `series`。
     */
    bearings?: number[];
    /**
     * 方位角分成几个扇区（与 `bearings` 成对出现）。
     */
    bearings_bins?: number | null;
    /**
     * 把扇区标签换成方位名（`N` / `NE` / `E` …）。三种写法下都能用。
     */
    compass_labels?: boolean | null;
    /**
     * 半径的编码方式，默认 `area`。
     */
    encoding?: EncodingEnum | null;
    /**
     * 画辐射状分隔线。
     */
    show_spokes?: boolean | null;
    /**
     * 边缘面板填充的不透明度。
     */
    marginal_alpha?: number | null;
    /**
     * 边缘面板与主图之间的缝（像素）。
     */
    marginal_gap?: number | null;
    /**
     * 边缘面板的厚度（像素）。
     */
    marginal_size?: number | null;
    marginal_type?: MarginalTypeEnum | null;
    /**
     * 画右侧的边缘面板。
     */
    show_right?: boolean | null;
    /**
     * 画顶部的边缘面板。
     */
    show_top?: boolean | null;
    [property: string]: unknown;
}

/**
 * 数条数（`value` 被忽略）。
 */
export type AggregationEnum = "sum" | "mean" | "max" | "count";

export interface AlluviaElement {
    /**
     * 从左到右依次经过的节点名。
     */
    nodes: string[];
    value: number;
    [property: string]: unknown;
}

/**
 * 序列起点的对齐方式。
 */
export type AnchorEnum = "left" | "right";

export interface ArrowElement {
    color?: null | string;
    /**
     * x 方向分量。
     */
    u: number;
    /**
     * y 方向分量。
     */
    v: number;
    x: number;
    y: number;
    [property: string]: unknown;
}

/**
 * 置信带：给定 x 上的上下界两列。
 */
export interface BandObject {
    lower: number[];
    upper: number[];
    [property: string]: unknown;
}

export interface BarElement {
    /**
     * `difference` 的起点。
     */
    from?: number | null;
    /**
     * `"delta"` / `"total"` / `"difference"`。
     */
    kind?: KindEnum | null;
    label: string;
    /**
     * `difference` 的终点。
     */
    to?: number | null;
    /**
     * 增量；`kind` 缺省时若给了 `from`/`to` 就是 `difference`，否则是 `delta`。
     */
    value?: number | null;
    [property: string]: unknown;
}

/**
 * 用当前的累计值画一根柱，但**不**重置累加器。
 *
 * 从 `from` 画到 `to`，不影响累加器。
 */
export type KindEnum = "delta" | "total" | "difference";

/**
 * wiggle（默认）：让主流尽量穿过中间。
 *
 * 对称。
 *
 * 零基线。
 */
export type BaselineEnum = "wiggle" | "symmetric" | "zero";

export interface BlockElement {
    color?: null | string;
    end1:   number;
    end2:   number;
    /**
     * 上方那条序列的下标。
     */
    seq1: number;
    /**
     * 下方那条序列的下标。
     */
    seq2:   number;
    start1: number;
    start2: number;
    /**
     * 缺省按 `seq1 <= seq2` 自动判断方向。
     */
    strand?: StrandEnum | null;
    [property: string]: unknown;
}

/**
 * 同向。
 *
 * 反向（画成弧）。
 */
export type StrandEnum = "forward" | "reverse";

/**
 * 树枝的画法。
 */
export type BranchStyleEnum = "rectangular" | "slanted" | "circular";

/**
 * 参考基因组版本（决定每条染色体的长度，用来算累积坐标）。
 */
export type BuildEnum = "hg19" | "hg38" | "t2_t";

export interface CandleElement {
    close: number;
    high:  number;
    /**
     * x 轴标签。
     */
    label: string;
    low:   number;
    open:  number;
    /**
     * 成交量，只有 `show_volume` 开着才用。
     */
    volume?: number | null;
    /**
     * 显式的数值 x；不给就按插入序号排（分类模式）。
     */
    x?: number | null;
    [property: string]: unknown;
}

export interface CategoryObject {
    color?: null | string;
    label:  string;
    value:  number;
    [property: string]: unknown;
}

export interface CellElement {
    /**
     * 列名（x 轴）。
     */
    col: string;
    /**
     * 行名（y 轴）。
     */
    row:   string;
    value: number;
    [property: string]: unknown;
}

export interface CladeColorObject {
    color: string;
    node:  number;
    [property: string]: unknown;
}

/**
 * 一条注释色条：每个元素一个格子，按矩阵的行列顺序。
 */
export interface ColAnnotationElement {
    colors: string[];
    /**
     * 色条右侧的标签。
     */
    label?: null | string;
    /**
     * 色条厚度。
     */
    width?: number | null;
    [property: string]: unknown;
}

/**
 * 一棵树的输入：四种写法**四选一**。
 *
 * 单独抽出来是为了复用 —— `phylo` 直接 flatten 它，`clustermap` 的 `row_tree` / `col_tree`
 * 也是同一个东西：描述一棵树的小语言只有这一份。
 */
export interface ColTreeObject {
    /**
     * 距离矩阵（UPGMA 聚类）。
     */
    distance_matrix?: null | DistanceMatrixObject;
    /**
     * 边表：`(父, 子, 枝长)`；根是「从没当过子」的那个节点。
     */
    edges?: EdgeElement[];
    /**
     * linkage 矩阵：每行 `[左下标, 右下标, 距离, 叶子数]`。
     */
    linkage?: null | LinkageObject;
    /**
     * Newick 字符串，如 `"((A:0.1,B:0.2):0.3,C:0.4);"`。
     */
    newick?: null | string;
    [property: string]: unknown;
}

/**
 * 距离矩阵：`dist` 必须是方阵，`dist[i][j]` 是第 i 与第 j 个标签的距离。
 */
export interface DistanceMatrixObject {
    dist:   Array<number[]>;
    labels: string[];
    [property: string]: unknown;
}

/**
 * 一条枝：父节点名、子节点名、从父到子的枝长。
 */
export interface EdgeElement {
    child:  string;
    length: number;
    parent: string;
    [property: string]: unknown;
}

/**
 * linkage 矩阵：每行 `[左下标, 右下标, 距离, 叶子数]`。
 */
export interface LinkageObject {
    labels:  string[];
    linkage: Array<[number, number, number, number, ...number[]]>;
    [property: string]: unknown;
}

/**
 * 同一个颜色（默认）。
 *
 * 逐层换色。
 *
 * 按层号渐变。
 */
export type ColorModeEnum = "by_parent" | "by_value" | "explicit" | "uniform" | "by_stage" | "gradient";

export interface ColorModeObject {
    color_map?: null | string;
    [property: string]: unknown;
}

/**
 * 一条曲线：x 与 y 等长。
 */
export interface CurveObject {
    x: number[];
    y: number[];
    [property: string]: unknown;
}

/**
 * S 形（默认）。
 *
 * 直线。
 */
export type CurveStyleEnum = "sigmoid" | "straight";

/**
 * 一个点的完整写法。
 */
export interface DatumObject {
    x?:     number;
    x_err?: [number, number, ...number[]] | number | null;
    y?:     number;
    y_err?: [number, number, ...number[]] | number | null;
    z?:     number;
    /**
     * `"YYYY-MM-DD"`。
     */
    date?:  string;
    value?: number;
    [property: string]: unknown;
}

export interface DateRangeObject {
    end:   string;
    start: string;
    [property: string]: unknown;
}

export interface DomainElement {
    color:    string;
    end:      number;
    label?:   null | string;
    opacity?: number | null;
    start:    number;
    [property: string]: unknown;
}

/**
 * 逐点连续写法的一条记录。
 */
export interface DotPointElement {
    /**
     * 点位置下标（**从 0 开始**，取值须小于 `ndots`）。
     */
    dot: number;
    /**
     * 填充色的编码值。
     */
    fill?: number | null;
    /**
     * 点大小的编码值。
     */
    size?: number | null;
    x:     string;
    y:     string;
    [property: string]: unknown;
}

/**
 * 一条枝：父节点名、子节点名、从父到子的枝长。
 */
export interface EdgeEdge {
    color?: null | string;
    /**
     * 边的弯曲程度（`0` = 直线）。
     */
    curve?: number | null;
    /**
     * 边上的文字。
     */
    label?: null | string;
    /**
     * 源节点名。
     */
    source?: string;
    /**
     * 目标节点名。
     */
    target?: string;
    weight?: number;
    child?:  string;
    length?: number;
    parent?: string;
    [property: string]: unknown;
}

/**
 * 扇形面积正比于数值（默认，视觉上更准确）。
 *
 * 扇形半径正比于数值。
 */
export type EncodingEnum = "area" | "radius";

/**
 * 从左上角开始，逐行往右。
 *
 * 从左下角开始，逐行往右。
 *
 * 从左上角开始，逐列往下。
 *
 * 从左下角开始，逐列往上。
 */
export type FillOrderEnum = "row_major_top_left" | "row_major_bottom_left" | "col_major_top_left" | "col_major_bottom_left";

/**
 * 「一个标签 + 一列观测值」的分组：violin / ridgeline / raincloud / strip / ecdf / qq 都用它。
 */
export interface SecondarySeryGroup {
    label?:  null | string;
    values?: number[];
    /**
     * 缺省则整组用 series 的主色。
     */
    color?: null | string;
    /**
     * 逐点颜色；比 `values` 短时多余的点回退到组色。
     */
    point_colors?: string[] | null;
    /**
     * 逐点 marker 形状。
     */
    point_shapes?: Marker[] | null;
    /**
     * 在图上标出 AUC。
     */
    auc_label?: boolean | null;
    /**
     * 虚线样式，如 `"4 2"`。
     */
    dasharray?:  null | string;
    line_width?: number | null;
    /**
     * 标出 F1 最优的那个阈值点。
     *
     * 标出 Youden 指数最优点。
     */
    optimal_point?: boolean | null;
    /**
     * 预计算的曲线点 `(recall, precision)`；与 `predictions` 二选一，同时给时以
     * `predictions` 为准。
     *
     * 预计算的曲线点 `(fpr, tpr)`；与 `predictions` 二选一，同时给时以 `predictions` 为准。
     */
    points?: Array<[number, number, ...number[]]> | null;
    /**
     * 原始预测：`(score, is_positive)`。给了它就由 kuva 扫阈值。
     */
    predictions?: Array<[boolean | number, boolean | number, ...(boolean | number)[]] | PredictionObject> | null;
    /**
     * 类别先验 prevalence；`points` 模式下用它定基线，缺省 0.5。
     */
    prevalence?: number | null;
    /**
     * 置信带。
     */
    ci?: boolean | null;
    /**
     * 置信带的不透明度。
     */
    ci_alpha?: number | null;
    /**
     * 只在假阳率区间 `[lo, hi]` 内积分（部分 AUC）。
     */
    pauc_range?: [number, number, ...number[]] | null;
    /**
     * 每个受试者是否发生了事件，长度须与 `times` 一致。
     */
    events?: boolean[];
    /**
     * 每个受试者的随访时间，长度须与 `events` 一致。
     */
    times?: number[];
    /**
     * 逐点颜色。
     */
    colors?: string[] | null;
    /**
     * 在图上标出相关系数。
     */
    correlation?: boolean | null;
    /**
     * 在图上标出回归方程。
     */
    equation?: boolean | null;
    marker?:   Marker | null;
    /**
     * 这一组的点不透明度；不给就用顶层的 `marker_opacity`。
     */
    marker_opacity?: number | null;
    /**
     * 这一组的点半径；不给就用顶层的 `marker_size`。
     */
    marker_size?: number | null;
    /**
     * 这一组的点描边宽度。
     */
    marker_stroke_width?: number | null;
    /**
     * 逐点半径（气泡图）。
     */
    sizes?: number[] | null;
    /**
     * 画趋势线（最小二乘）。
     */
    trend?: boolean | null;
    x?:     number[];
    /**
     * 逐点 x 误差：一个数是对称的，`[下, 上]` 是不对称的。长度须与 `x` 一致。
     */
    x_err?: Array<[number, number, ...number[]] | number> | null;
    /**
     * 长度须与 `x` 一致。
     */
    y?: number[];
    /**
     * 逐点 y 误差，写法同 `x_err`。
     */
    y_err?: Array<[number, number, ...number[]] | number> | null;
    [property: string]: unknown;
}

export interface PredictionObject {
    /**
     * 是否为正类。
     */
    label: boolean;
    score: number;
    [property: string]: unknown;
}

export interface IntersectionElement {
    count: number;
    /**
     * 位掩码：第 `i` 位为 1 表示包含 `set_names[i]`。
     */
    mask: number;
    [property: string]: unknown;
}

export type LabelPositionEnum = "inside" | "outside" | "auto" | "none";

export type LabelStyleEnum = "exact" | "nudge";

/**
 * 箭头标签的偏移量，缺省给一个朝右上方的默认值。
 */
export interface LabelStyleObject {
    offset_x?: number | null;
    offset_y?: number | null;
    [property: string]: unknown;
}

/**
 * 力导向（默认）。
 *
 * 均匀排在圆周上（此时边的下标越界只是被忽略，不 panic）。
 */
export type LayoutEnum = "kamada_kawai" | "force_directed" | "circle" | "squarify" | "slice_dice" | "binary";

/**
 * 连边着色：`"source"`（取源节点色）/ `"gradient"` / `"per_link"`（用 `links[].color`）。
 */
export type LinkColorEnum = "source" | "gradient" | "per_link";

export interface LinkElement {
    color?: null | string;
    /**
     * 源节点名。
     */
    source: string;
    /**
     * 目标节点名。
     */
    target: string;
    value:  number;
    [property: string]: unknown;
}

/**
 * 直方图（默认）。
 *
 * 核密度。
 */
export type MarginalTypeEnum = "histogram" | "density";

export interface StageElement {
    color?: null | string;
    label:  string;
    value:  number;
    [property: string]: unknown;
}

/**
 * 左右各自一组柱（默认）。
 *
 * 左右重叠。
 *
 * 同一扇区里堆叠（默认）。
 *
 * 每个系列各占一个子扇区。
 */
export type SecondarySeryMode = "normal" | "genomic" | "grouped" | "overlap" | "stacked";

/**
 * 节点着色：`"label"`（同一标签同色）/ `"left"`（按最左侧的来源节点着色）。
 */
export type NodeColoringEnum = "label" | "left";

/**
 * 节点排序方式：`"input"`（按 `column` 落位）/ `"crossing_reduction"` /
 * `"neighbornet"`。
 */
export type NodeOrderEnum = "input" | "crossing_reduction" | "neighbornet";

export interface NodeElement {
    color?: null | string;
    /**
     * 固定它落在第几根轴（不给就由布局决定）。
     */
    column?: number | null;
    label:   string;
    /**
     * 分组名（进图例）。
     */
    group?: null | string;
    /**
     * 固定坐标；不给就由布局算。
     */
    position?: [number, number, ...number[]] | null;
    shape?:    NodeShape | null;
    /**
     * 点大小。
     */
    size?: number | null;
    [property: string]: unknown;
}

/**
 * 节点形状。
 */
export type NodeShape = "circle" | "square" | "triangle" | "diamond";

export type NormalizationEnum = "none" | "row_z_score" | "col_z_score";

/**
 * 由内到外（默认）。
 *
 * 按总量。
 *
 * 按输入顺序。
 */
export type OrderEnum = "inside_out" | "by_total" | "original";

/**
 * 树的方向。
 *
 * 竖着往下（默认）。
 */
export type SecondarySeryOrientation = "left" | "right" | "top" | "bottom" | "horizontal" | "vertical";

export interface OverlapElement {
    /**
     * 参与这个交集的集合名。
     */
    sets: string[];
    /**
     * **含**所有子交集的计数（kuva 用逐层减法推独占部分）。
     */
    size: number;
    [property: string]: unknown;
}

/**
 * 一个展示区间。
 */
export interface PeriodElement {
    /**
     * `"YYYY-MM-DD"`。
     */
    end: string;
    /**
     * 区间左侧的标签。
     */
    label: string;
    /**
     * `"YYYY-MM-DD"`。
     */
    start: string;
    [property: string]: unknown;
}

/**
 * 尾端（默认）。
 */
export type PivotEnum = "middle" | "tip" | "tail";

export interface PointObject {
    /**
     * 颜色编码值。
     */
    color?: number | null | string;
    /**
     * 半径编码值。
     *
     * 点大小的编码值。
     */
    size?: number | null;
    /**
     * x 轴类别名。
     */
    x?: number | string;
    /**
     * y 轴类别名。
     */
    y?: number | string;
    /**
     * 点旁的文字标签。
     *
     * 标签；不给就靠 `label_top` 自动选。
     */
    label?: null | string;
    /**
     * log2 倍数变化。
     */
    log2fc?: number;
    /**
     * 基因/项目名，出现在标签上。
     */
    name?: string;
    /**
     * 原始 p 值（不是 -log10 之后的结果）。
     */
    pvalue?: number;
    /**
     * 上角的分量。
     */
    a?: number;
    /**
     * 左下角的分量。
     */
    b?: number;
    /**
     * 右下角的分量。
     */
    c?: number;
    /**
     * 分组名（进图例）。
     */
    group?: null | string;
    /**
     * 填充色的编码值。
     */
    fill?: number | null;
    /**
     * 这一格里放了哪几个点（**从 0 开始**的下标，取值须小于 `ndots`）。
     */
    present?: number[];
    /**
     * 染色体名，`chr` 前缀可写可不写。
     */
    chromosome?: string;
    /**
     * 位置（bp）；给了才可能走累积坐标那条路。
     */
    position?: number | null;
    after?:    number;
    before?:   number;
    [property: string]: unknown;
}

/**
 * 分类写法的一条记录。
 */
export interface RecordElement {
    /**
     * 这个点落在哪个位置上：与 `category_labels` 里的名字匹配。
     */
    category: string;
    /**
     * CSS 颜色字符串。
     */
    color: string;
    x:     string;
    y:     string;
    [property: string]: unknown;
}

export type ReduceEnum = "count" | "mean" | "sum" | "median" | "min" | "max";

export interface ReferenceElement {
    color?: null | string;
    /**
     * 虚线样式，如 `"4 2"`。
     */
    dasharray?: null | string;
    /**
     * 逐点误差棒（`values` 等长）。
     */
    errors?: number[] | null;
    label?:  null | string;
    /**
     * 与 `axes` 一一对应。
     */
    values: number[];
    [property: string]: unknown;
}

/**
 * 树 / 森林的一个节点。有 `children` 就是内部节点（`value` 缺省时由子节点求和），
 * 否则是叶子（`value` 必填）。
 */
export interface RootElement {
    children?: RootElement[];
    color?:    null | string;
    label:     string;
    /**
     * 叶子的大小；内部节点不给就按子节点求和。
     */
    value?: number | null;
    [property: string]: unknown;
}

export interface RowElement {
    ci_lower?: number;
    ci_upper?: number;
    color?:    null | string;
    /**
     * 点估计。
     */
    estimate?: number;
    label?:    string;
    /**
     * 权重：按 `sqrt(weight / max_weight)` 缩放 marker。
     */
    weight?: number | null;
    /**
     * 分组名（进图例）。
     */
    group?: null | string;
    /**
     * 与 `axis_names` 一一对应。
     */
    values?: number[];
    [property: string]: unknown;
}

export interface SequenceObject {
    color?: null | string;
    label:  string;
    /**
     * 坐标上限。
     */
    length: number;
    [property: string]: unknown;
}

export interface SecondarySerySeries {
    color?: null | string;
    name?:  string;
    /**
     * 原始数值，交给 kuva 排名。
     *
     * 与 `axes` 一一对应。
     *
     * 与扇区一一对应，长度应与 `labels` 一致。
     */
    values?: Array<number | null> | null;
    label?:  null | string;
    /**
     * 虚线样式，如 `"4 2"`。
     */
    line_dash?:           null | string;
    marker_opacity?:      number | null;
    marker_size?:         number | null;
    marker_stroke_width?: number | null;
    /**
     * `"scatter"`（默认）/ `"line"`。
     */
    mode?: FluffyRenderSchema | null;
    /**
     * 半径序列。
     */
    r?:            number[];
    stroke_width?: number | null;
    /**
     * 角度序列，单位是**度**；长度须与 `r` 一致。
     */
    theta?: number[];
    /**
     * 负区的颜色。
     */
    neg_color?: null | string;
    /**
     * 正区的颜色。
     */
    pos_color?: null | string;
    /**
     * x 序列（通常是时间）。
     */
    x?: number[];
    /**
     * y 序列，长度须与 `x` 一致。
     */
    y?: number[];
    /**
     * 已知的名次（允许小数名次，如 2.5 表示并列第二）。
     */
    ranks?:  Array<number | null> | null;
    groups?: PurpleRenderSchema[];
    /**
     * `mode = "overlap"` 时的填充不透明度。
     */
    opacity?: number | null;
    /**
     * 虚线样式，如 `"4 2"`。
     */
    dasharray?: null | string;
    /**
     * 逐点误差棒（`values` 等长）。
     */
    errors?: number[] | null;
    [property: string]: unknown;
}

export interface PurpleRenderSchema {
    /**
     * 年龄组标签（0-4、5-9 …）。
     */
    age: string;
    /**
     * 左边的数值。
     */
    left: number;
    /**
     * 右边的数值。
     */
    right: number;
    [property: string]: unknown;
}

export type FluffyRenderSchema = "scatter" | "line";

export interface SetElement {
    /**
     * 集合里的原始元素（原始写法，给了它就走这一套）。
     */
    elements?: string[] | null;
    label:     string;
    /**
     * 集合大小（预计算写法）。
     */
    size?: number | null;
    [property: string]: unknown;
}

/**
 * 格子形状。
 */
export type SecondarySeryShape = "square" | "circle";

export interface SliceElement {
    /**
     * 缺省则按调色板轮流取色。
     */
    color?: null | string;
    label:  string;
    value:  number;
    [property: string]: unknown;
}

/**
 * 按交集大小（默认）。
 *
 * 按参与的集合数。
 *
 * 按位掩码数值。
 */
export type SortEnum = "by_frequency" | "by_degree" | "natural";

/**
 * 对开小提琴的「另一半」：只有值。标签由 [`ViolinSeries::groups`] 按下标带过来，所以这里
 * 不收 `label` —— 写了也会被忽略。
 */
export interface SplitGroupElement {
    values: number[];
    [property: string]: unknown;
}

/**
 * 画法：`"point"`（默认）/ `"line"` / `"both"`。
 */
export type StyleStyleEnum = "strip" | "swarm" | "center" | "line" | "point" | "both";

/**
 * 只写抖动幅度时，`type` 缺省即 `strip`。
 */
export interface StyleObject {
    jitter?: number | null;
    type?:   StyleType | null;
    [property: string]: unknown;
}

export type StyleType = "strip" | "swarm" | "center";

export interface TaskElement {
    color?: null | string;
    /**
     * 结束时间；里程碑时给成与 `start` 相同。
     */
    end: number;
    /**
     * 分组名（会给它加一行表头）。
     */
    group?: null | string;
    label:  string;
    /**
     * 这是个里程碑（画成菱形而不是条）。
     */
    milestone?: boolean | null;
    /**
     * 进度 0~1，内部会夹到这个区间。
     */
    progress?: number | null;
    /**
     * 开始时间（单位自定，kuva 只当数值用）。
     */
    start: number;
    [property: string]: unknown;
}

/**
 * A 绿 / C 蓝 / G 橙 / T 红。
 *
 * A 绿 / C 蓝 / G 橙 / U 红。
 */
export type TemplateEnum = "dna" | "rna";

/**
 * 正文的对齐方式。
 */
export type TextAlignEnum = "left" | "center" | "right";

/**
 * 名次并列时怎么决出名次（取平均 / 取最小 / 取最大 / 按出现顺序稳定取）。
 */
export type TieBreakEnum = "average" | "min" | "max" | "stable";

/**
 * JSON 键是 `type`（Rust 关键字，故字段另取名）。
 */
export type RenderSchema = "linear";

export interface TrendObject {
    color?: null | string;
    /**
     * 在图上标出相关系数。
     */
    correlation?: boolean | null;
    /**
     * 在图上标出回归方程。
     */
    equation?: boolean | null;
    /**
     * JSON 键是 `type`（Rust 关键字，故字段另取名）。
     */
    type:   RenderSchema;
    width?: number | null;
    [property: string]: unknown;
}

export type Type = "scatter" | "line" | "bar" | "histogram" | "box" | "pie" | "violin" | "ridgeline" | "raincloud" | "strip" | "dot_plot" | "lollipop" | "density" | "ecdf" | "qq" | "forest" | "pr" | "roc" | "survival" | "volcano" | "heatmap" | "histogram2d" | "hexbin" | "clustermap" | "contour" | "ternary" | "polar" | "dice_plot" | "scatter3d" | "surface3d" | "sankey" | "chord" | "network" | "treemap" | "sunburst" | "venn" | "upset" | "waffle" | "mosaic" | "phylo" | "synteny" | "candlestick" | "calendar" | "gantt" | "horizon" | "manhattan" | "waterfall" | "bump" | "pareto" | "brick" | "funnel" | "slope" | "pyramid" | "series" | "radar" | "parallel" | "stacked_area" | "streamgraph" | "band" | "text" | "legend_plot" | "quiver" | "rose" | "jointplot";

/**
 * 一周从哪天开始。
 */
export type WeekStartEnum = "monday" | "sunday";

/**
 * 统计框：图角上的一小块文字（样本量、p 值、模型名…）。
 */
export interface StatsBoxObject {
    /**
     * 给外框。
     */
    border?: boolean | null;
    /**
     * 逐行文字。
     */
    entries?: string[];
    /**
     * 位置具名值，同 `legend.position`。
     */
    position?: null | string;
    /**
     * 粗体标题。
     */
    title?: null | string;
    [property: string]: unknown;
}

export type ThemeEnum = "light" | "dark" | "minimal" | "solarized";

/**
 * 用 Box 装：`ThemeCustom` 有十几个字段，直接内联会让这个枚举的每个值都带上那份体积。
 */
export interface ThemeObject {
    axis_color?:      null | string;
    background?:      null | string;
    box_median?:      null | string;
    colorbar_border?: null | string;
    font_family?:     null | string;
    grid_color?:      null | string;
    legend_bg?:       null | string;
    legend_border?:   null | string;
    pie_leader?:      null | string;
    show_grid?:       boolean | null;
    text_color?:      null | string;
    tick_color?:      null | string;
    violin_border?:   null | string;
    [property: string]: unknown;
}

export interface TitleObject {
    size?:         number | null;
    subtext?:      null | string;
    subtext_size?: number | null;
    subtext_wrap?: number | null;
    text?:         null | string;
    /**
     * 标题按字符数折行。
     */
    wrap?: number | null;
    [property: string]: unknown;
}

/**
 * 第二根轴（双 Y / 双 X 图）。字段与 [`AxisSpec`] 一一对应，只是作用在另一侧。
 */
export interface X2AxisObject {
    label_offset?: [number, number, ...number[]] | null;
    /**
     * 对数轴。
     */
    log?: boolean | null;
    max?: number | null;
    min?: number | null;
    /**
     * 轴标题。
     */
    name?: null | string;
    /**
     * 刻度格式，同 `AxisSpec.tick_format`。
     */
    tick_format?: ColorbarTickFormatEnum | number | null;
    /**
     * 轴标题折行宽度（字符）。
     */
    wrap?: number | null;
    [property: string]: unknown;
}

/**
 * 一根坐标轴。
 */
export interface XAxisObject {
    /**
     * 类别轴标签（柱状 / 箱线等）。给了就覆盖自动收集的类别。
     */
    categories?: string[] | null;
    /**
     * 轴标题相对默认位置的偏移（像素）。
     */
    label_offset?: [number, number, ...number[]] | null;
    /**
     * 标签重叠策略：`"allow"` / `"thin"` / `"stagger"`。
     */
    label_overlap?: LabelOverlapEnum | null;
    /**
     * 对数轴。
     */
    log?: boolean | null;
    max?: number | null;
    min?: number | null;
    /**
     * 轴标题。
     */
    name?: null | string;
    /**
     * 刻度格式：`"auto"` / `"integer"` / `"sci"` / `"percent"` / `"degree"`，或一个整数表示定点小数位。
     */
    tick_format?: ColorbarTickFormatEnum | number | null;
    /**
     * 刻度标签旋转角度（度）。
     */
    tick_rotate?: number | null;
    /**
     * 主刻度的间隔（给了就按它取整刻度）。
     */
    tick_step?: number | null;
    /**
     * 轴标题折行宽度（字符）。
     */
    wrap?: number | null;
    [property: string]: unknown;
}

export type LabelOverlapEnum = "allow" | "thin" | "stagger";

/**
 * 日期轴：把坐标值当时间戳（Unix 秒）来排布刻度。
 */
export interface XDatetimeObject {
    /**
     * 刻度标签的格式串（chrono 风格，如 `"%Y-%m"`）；`unit: "auto"` 时可不给。
     */
    format?: null | string;
    /**
     * 每几个单位一个刻度，默认 1。
     */
    step?: number | null;
    /**
     * 时间单位：`"auto"` / `"year"` / `"month"` / `"week"` / `"day"` / `"hour"` / `"minute"` /
     * `"second"`。给 `"auto"` 时由 kuva 按轴的范围自己选单位与格式，`format` 可以省。
     */
    unit: Unit;
    [property: string]: unknown;
}

/**
 * 时间单位：`"auto"` / `"year"` / `"month"` / `"week"` / `"day"` / `"hour"` / `"minute"` /
 * `"second"`。给 `"auto"` 时由 kuva 按轴的范围自己选单位与格式，`format` 可以省。
 *
 * 按轴的跨度自动选单位与格式。
 */
export type Unit = "year" | "month" | "week" | "day" | "hour" | "minute" | "second" | "auto";

export interface RowHeights {
}

/**
 * 共享轴的「一段」。
 */
export interface SharedXSliceElement {
    end: number;
    /**
     * 行号（`shared_y_slices`）或列号（`shared_x_slices`）。
     */
    index: number;
    start: number;
    [property: string]: unknown;
}

/**
 * 文件输出的选项，见 [`RenderSpec::file`]。
 *
 * 只有原生构建编译它（wasm 下连 `RenderSpec.file` 都不存在，见那里的说明）。
 *
 * The file-output options, see [`RenderSpec::file`]. Native builds only (a wasm build has
 * no
 * `RenderSpec.file` either, see the note there).
 */
export interface FileObject {
    /**
     * 输出目录；缺省用系统临时目录。
     *
     * The output directory; the system temp directory when unset.
     */
    dir?: null | string;
    /**
     * 输出文件名；缺省自动生成 `kuva-<随机尾缀>.svg`。
     *
     * 用户给的名字会先过一遍文件名合法性规则（见 `spec/file.rs`），没写后缀时补 `.svg`。
     *
     * The output file name; generated as `kuva-<random>.svg` when unset. A caller-supplied name
     * goes
     * through the file-name legality rules first (see `spec/file.rs`) and gets `.svg` appended
     * when it
     * has no extension.
     */
    name?: null | string;
    /**
     * 写完后用系统默认浏览器打开。缺省 `false`。
     *
     * 只有原生构建才有效：wasm 构建里 `kuva_render_file` 整个不支持（见 `spec/file.rs`）。
     *
     * Open the file in the system default browser once it has been written. Unset means
     * `false`.
     * Native builds only: `kuva_render_file` is not supported in a wasm build at all (see
     * `spec/file.rs`).
     */
    open?: boolean | null;
    [property: string]: unknown;
}

/**
 * 终端渲染的选项，见 [`RenderSpec::terminal`]。
 */
export interface TerminalObject {
    /**
     * 字符网格的列数（默认 100，和 kuva CLI 探测不到终端尺寸时的回退值一样）。
     */
    cols?: number | null;
    /**
     * 直接把结果 `print!` 到 stdout，函数返回 NULL。默认 `false`，也就是作为字符串返回。
     *
     * DuckDB 的 CLI 里，把一个字符串字段原样打到控制台并不顺手（`SELECT` 出来的是带引号、
     * 转义过的一行），而 `print!` 是随手的 —— 这个开关就是给那种场合的。
     */
    print?: boolean | null;
    /**
     * 字符网格的行数（默认 30）。
     */
    rows?: number | null;
    [property: string]: unknown;
}
