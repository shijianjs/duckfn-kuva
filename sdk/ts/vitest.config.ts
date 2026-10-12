import { defineConfig } from 'vitest/config';

// duckdb 是原生绑定（.node），放进 fork 进程池里跑比 worker 线程稳；
// 加载扩展 + 第一次渲染要几秒，放宽 hook / 用例超时。
//
// duckdb is a native binding (.node); running it in the fork pool is more stable than worker
// threads, and loading the extension plus the first render takes a few seconds, so the hook and
// test timeouts are generous.
export default defineConfig({
  test: {
    pool: 'forks',
    hookTimeout: 60_000,
    testTimeout: 30_000,
  },
});
