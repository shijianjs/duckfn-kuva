# community-extension/ 的约定

这个目录不参与扩展运行，它是**向 [duckdb/community-extensions](https://github.com/duckdb/community-extensions)
提 PR 的暂存处**：那个 PR 只加两个文件，这里按目标路径摆好，提交时复制过去即可。

| 本目录 | 社区仓里的位置 |
| --- | --- |
| `description.yml` | `extensions/<扩展名>/description.yml` |
| `docs/function_descriptions.csv` | `extensions/<扩展名>/docs/function_descriptions.csv` |

目录名必须与 `extension.name` **逐字一致** —— 社区仓的 `scripts/build.py` 会校验，不一致直接报错。

## 提交进度与还要改什么

`description.yml` 的 `extension.description` / `maintainers` / `repo.github` 与 `docs.hello_world` /
`docs.extended_description` 都已按本扩展的真实 API（`kuva_render` / `kuva_render_terminal` /
`kuva_render_file`）写好，改 API 时跟着改 —— 改了 `hello_world` / `extended_description`，或改了函数
属性上的 description / example，都要重生成 `docs/function_descriptions.csv`（见下面「那份 CSV 是必须的」），
并**真跑一遍** `hello_world`：它渲染进社区文档页的代码块，写歪了不会有人报错。

**首次提交已完成**（2026-10-13，[#3044](https://github.com/duckdb/community-extensions/pull/3044)，已合并）：
`repo.ref` 与 `extension.version` 都指向 v0.2.0。**打开预发布腿的那一版**是 2026-10-14 的
[#3056](https://github.com/duckdb/community-extensions/pull/3056)（加 `repo.ref_next`，见下面
「`repo.ref_next`：下一个大版本那一腿」）。之后每次发版回来改这两处（见下面「每次发版都要回来改」）。
本文件下面提到的路径与用户名也照本机实际情况核对一遍。

`extension.name` / `version` / `license` / `language` / `build` / `requires_toolchains` 在模板里已经是可直接
沿用的值（下面一节解释为什么），只有 `name` 要跟着你改名后的扩展名走（`scripts/rename.sh` 会一起改）。

## `description.yml` 里**不要写注释**

这份文件会被原样复制到上游，所以刻意保持「只有字段」。需要解释的东西写在本文件里，不要写回 YAML。

`repo.ref` 写**发布那一版的提交 SHA（40 位）**（`git rev-list -n 1 v0.2.0`），**不要写 `main`、也不要写
tag 名**。三者都是合法 git ref、社区仓都能照着 clone，但上游已收录的扩展清一色用提交 SHA
（`extensions/h3`、`extensions/orc` 都是），跟着走既不用解释，也天生不可变 —— 注册项指向的东西不会随时
间漂移。写 `main` 的代价是实打实的：构建出来的二进制会自称 main 上的开发版本（`X.Y.Z-dev.N`），与这里
声明的 `version` 对不上，而且同一个注册项在不同时间构建出的是不同代码。

## `repo.ref_next`：下一个大版本那一腿

社区注册表除了正式版那条腿，还跑一条**预发布**腿（他们的 `build_next.yml`：DuckDB 用
`v2.0-cyanoptera` 分支、ci-tools 用 `main`、并带 `require_ref_next: true`）。这条腿只认描述文件里的
`repo.ref_next`：没有这个字段就直接跳过，日志里打一句
`Skipping prerelease validation: repo.ref_next is not defined` —— 也就是那条 PR 什么都没验证。

当前的值是**与 `ref` 同一个 SHA**，对应上游 `UPDATING.md` 的 state 1：代码不改就能同时喂两个引擎，
不需要开 `vx.y-codename` 分支（上游的 `duck_geoarrow`、`duck_hunt` 也是这么写的）。依据是实测：
v0.2.0 的产物（debug 与 Release 里的 `windows_amd64`）在 `v2.0.0-alpha43763` / `alpha46057` 上加载并跑通
渲染、两参数 dataset 形式、多面板 figure、终端后端与落盘，非法 spec 仍正确报错；完整 sqllogictest 在
`duckdb 2.0.0.dev…`（PyPI 预发布轮子）上 `SUCCESS`。仓库自己的
`.github/workflows/MainDistributionPipeline.yml` 里那条 `duckdb-next-build` 会继续替我们验：
它取 ci-tools 与 DuckDB 的 `v2.0-cyanoptera`，只在非 tag 时跑。

哪天预发布引擎真的要求改代码，就照 `UPDATING.md` 走：在本仓开 `vx.y-codename` 分支改到绿，把
`ref_next` 换成那个分支的最新提交 SHA（`ref` 不动），再复制进社区仓那份克隆、推到同一条 PR 分支。

## 字段为什么这么写（都不是猜的）

- `language: Rust` / `build: cargo`：本仓 Makefile 就是官方 Rust 模板那一套（include
  `extension-ci-tools/makefiles/c_api_extensions/rust.Makefile`）；社区仓里的 `rusty_quack`
  （仓库就是 `duckdb/extension-template-rs`，即该模板）用的正是 `build: cargo`。
- `requires_toolchains: "rust;python3"`：与 `.github/workflows/MainDistributionPipeline.yml` 的
  `extra_toolchains` 一致；`python3` 是 extension-ci-tools 的 configure / 测试流程要的。
- **不写 `excluded_platforms`**：没有需要排除的平台。两个 musl 平台（`linux_amd64_musl` /
  `linux_arm64_musl`）在矩阵里都是 `opt_in`，不主动点名就不会构建，所以也不必写进排除列表。
  真的跑不起来某个平台时（例如依赖在 wasm 上编不过），再回来加。
- `version`：写**要发布的那一版**，不要 `-dev.N`（开发版本是 `0.2.1-dev.0`，发版时抬成要发布的那一版，
  例如 0.2.0 那一版就是从 `0.1.1-dev.0` 抬上来的）。
  它与 `repo.ref` 配套 —— `ref: v0.2.0` ↔ `version: 0.2.0`，所以社区仓构建出的二进制自称的版本、
  文档页上的版本号、这里的字段三者一致，不存在漂移。`just release_bump` 会顺手改掉这里的 `version`，
  `repo.ref` 仍然要人工改。
- `license: MIT`：对应仓库根目录的 `LICENSE`。注意 duckdb.org 的社区扩展文档页把字段名写成 `licence`，
  那是**文档的错**，真实 schema 是 `license`（以已收录扩展的 `description.yml` 为准）。
- `docs.hello_world`：社区文档页会把它渲染进代码块，所以必须是**可直接复制跑**的真实例子 ——
  现在这两段就是 README 快速上手的两个示例。页面上「Installing and Loading」那段由社区仓的
  `layout/default.md` 自动加，`hello_world` 里不要再写 `INSTALL` / `LOAD`。

## 那份 CSV 是必须的，而且会过期

社区文档页「Added Functions」表里，函数的 description / comment / example **只有一个来源**：这个 CSV
（DuckDB 的 C 扩展 API 没有设置它们的接口）。它由 `just docs_csv` 从属性宏上的 `description` /
`comment` / `example` 生成。**改了那些属性就要重新生成、覆盖这里这份**，否则文档页停在旧文案上。

```shell
just docs_csv
cp target/function_descriptions.csv community-extension/docs/function_descriptions.csv
```

这份 CSV 由 `just docs_csv` 从 `#[duck_*]` 属性上的 `description` / `comment` / `example` 生成；改了那些
文案就重新生成一次，覆盖这里这份。

## 提交：在 fork 的克隆里做，本仓只出那两张文件

上游注册仓（`duckdb/community-extensions`）的 fork 与本地克隆：

- fork：`shijianjs/duckdb-community-extensions`
- 克隆：`S:\workspace\my\rust\duckdb\duckdb-community-extensions`（`origin` 是这个 fork，`upstream`
  指向 `duckdb/community-extensions`）—— 按本机实际路径替换。

流程是「加分支 → 复制两个文件 → 提交 → 推分支 → 开 PR」，本仓这边一行都不用改：

```powershell
$p = 'S:\workspace\my\rust\duckdb\duckdb-community-extensions'
$ext = 'duckfn_kuva'                                # 与 extension.name 逐字一致
git -C $p checkout main; git -C $p pull              # 先和 fork 的 main 同步
git -C $p checkout -b add-$ext                       # 已有同名分支就跳过这步

New-Item -ItemType Directory -Force "$p\extensions\$ext\docs" | Out-Null
Copy-Item community-extension\description.yml "$p\extensions\$ext\description.yml" -Force
Copy-Item community-extension\docs\function_descriptions.csv "$p\extensions\$ext\docs\function_descriptions.csv" -Force

git -C $p add extensions/$ext
git -C $p commit -m "Add $ext: …"                    # 上游惯例是 `Add <扩展名>` 再跟一句说明
git -C $p push -u origin add-$ext
gh pr create --repo duckdb/community-extensions --base main --head shijianjs:add-$ext `
  --title '…' --body-file …
```

- **每次发版都要回来改一处**：`repo.ref` 换成新发布提交的 SHA（`git rev-list -n 1 v0.2.0`），
  `version` 跟着改成 `0.2.0`，改完在本仓 `community-extension/` 里改，再复制进那份克隆，推到同一条 PR
  分支（PR 会自动更新）或另开一个。注册项钉的是具体提交，不会自己跟。
- PR 一开，上游会起三条构建（`Community Extension Build`、`Community Extension with latest DuckDB`、
  `Community Extension with DuckDB on Andium`）。外部贡献者的 PR 会被 GitHub 挂成 `action_required`
  —— 等维护者点 Run，属正常状态，别因此重推。
- 想把 fork 与上游同步时，那份克隆里得先补个上游远程（它只配了 `origin`）：
  `git -C $p remote add upstream https://github.com/duckdb/community-extensions.git`，之后
  `git -C $p fetch upstream && git -C $p merge upstream/main`。

## 提交前自查

```powershell
# 1. 键集要和社区仓 generate_extensions_json.py 读的一致（三组顶层键 + 各组的子键）
.\configure\venv\Scripts\python.exe -c "import yaml;d=yaml.safe_load(open('community-extension/description.yml',encoding='utf-8'));print(list(d));print(list(d['extension']));print(list(d['repo']));print(list(d['docs']))"

# 2. hello_world 原样能跑：把 docs.hello_world 抽成 .sql，前面补一行 LOAD 本地产物，再交给本机 duckdb
duckdb -unsigned -c ".read target/hello_from_yml.sql"
```

- 仓库根目录必须有 `LICENSE`（社区扩展要求开源，`license` 字段只是声明）。
- `INSTALL <扩展名> FROM community` 要等 PR 合并后才真的可用 —— 合并前 README 里那套加载方式
  只对之后的版本成立。
