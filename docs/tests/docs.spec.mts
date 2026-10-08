import {fileURLToPath} from 'node:url';

import {declareDocsTests} from 'duckfn-docs-kit/sql/playwright';

declareDocsTests({
  siteDir: fileURLToPath(new URL('..', import.meta.url)),
  // The runnable blocks read the site's own data files with root-relative paths
  // (`read_csv_auto('/duckfn-kuva/data/scatter.tsv')`), which is how they will be
  // served on GitHub Pages (`baseUrl` = `/duckfn-kuva/`). The loopback harness also
  // has to serve them, otherwise every such block 404s under `playwright test`.
  assets: [{url: '/duckfn-kuva/data', dir: 'static/data'}],
});