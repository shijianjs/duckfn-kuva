import {fileURLToPath} from 'node:url';

import {declareDocsTests} from 'duckfn-docs-kit/sql/playwright';

declareDocsTests({
  siteDir: fileURLToPath(new URL('..', import.meta.url)),
  // `baseUrl` is what a block's `{{DFK_BASE_URL}}` expands to. On the real site it is
  // per-locale (`/duckfn-kuva/` for English, `/duckfn-kuva/zh-Hans/` for Chinese), because
  // Docusaurus copies `static/` into each locale's output. The harness has no locale and
  // runs every locale's blocks on one page, so it publishes the files under the plain
  // prefix below and takes the English one as its baseUrl.
  baseUrl: '/duckfn-kuva/',
  assets: [{url: '/duckfn-kuva/data', dir: 'static/data'}],
});