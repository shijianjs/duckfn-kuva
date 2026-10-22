import {createElement} from 'react';
import type {ReactNode} from 'react';
import Link from '@docusaurus/Link';
import Translate, {translate} from '@docusaurus/Translate';
import useBaseUrl from '@docusaurus/useBaseUrl';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import CodeBlock from '@theme/CodeBlock';
import Heading from '@theme/Heading';
import Layout from '@theme/Layout';
import {
  DfkFeatures,
  DfkHero,
  DfkNextSteps,
  registerDfkElements,
  type FeatureItem,
  type HeroAction,
  type HeroBadge,
  type HeroLink,
  type NextStepItem,
} from 'duckfn-docs-kit';

import styles from './index.module.css';

// Defining the `dfk-*` custom elements is a one-time side effect (it also
// registers the official `<iconify-icon>` element). Idempotent, and a no-op
// during Docusaurus' Node prerender pass.
registerDfkElements();

/**
 * The landing page, written for people who want to *use* the extension: hero,
 * features, an install/usage SQL showcase and the "where next" cards.
 *
 * The hero, the feature grid and the next-step cards are `dfk-*` web components
 * from duckfn-docs-kit, so the whole landing layout is reusable by other
 * extension docs sites. A custom element cannot render React's `<Translate>`,
 * so the copy is resolved with the imperative `translate()` API into plain
 * strings for the active locale and handed to the components through their own
 * setters; the strings still live in `i18n/zh-Hans/code.json` under the same
 * `homepage.*` keys. The code showcase stays here because it needs the theme's
 * `CodeBlock`.
 *
 * Known trade-off (accepted): the `dfk-*` sections render client-side, so their
 * prerendered HTML is empty until hydration — the same behaviour as the
 * `<iconify-icon>` glyphs.
 */

type DfkTag = 'dfk-hero' | 'dfk-features' | 'dfk-next-steps';

/**
 * Mounts a `dfk-*` element and hands its content over through the component's
 * own setters.
 *
 * React's SSR/hydration path only reconciles string/number props onto custom
 * elements — an object payload is never serialised into the prerendered HTML,
 * so hydration would leave the component empty. A callback ref is the reliable
 * channel: React calls it with the live node after mount, where the setters are
 * invoked. The components are retained-mode, so each setter only mutates the
 * nodes it owns; nothing is re-rendered.
 */
function dfk<TContent>(
  tag: DfkTag,
  mount: (node: HTMLElement, content: TContent) => void,
  content: TContent,
): ReactNode {
  return createElement(tag, {
    ref: (node: HTMLElement | null) => {
      if (node) {
        mount(node, content);
      }
    },
  });
}

interface HeroContent {
  logoSrc: string;
  title: string;
  tagline: string;
  primary: HeroLink;
  secondary: HeroAction;
  badges: HeroBadge[];
}

function mountHero(node: HTMLElement, content: HeroContent): void {
  const hero = node as DfkHero;
  hero.setLogo(content.logoSrc);
  hero.setTitle(content.title);
  hero.setTagline(content.tagline);
  hero.setPrimaryAction(content.primary);
  hero.setSecondaryAction(content.secondary);
  hero.setBadges(content.badges);
}

interface FeaturesContent {
  sectionTitle: string;
  items: FeatureItem[];
}

function mountFeatures(node: HTMLElement, content: FeaturesContent): void {
  const features = node as DfkFeatures;
  features.setSectionTitle(content.sectionTitle);
  features.setFeatures(content.items);
}

interface NextStepsContent {
  sectionTitle: string;
  items: NextStepItem[];
}

function mountNextSteps(node: HTMLElement, content: NextStepsContent): void {
  const steps = node as DfkNextSteps;
  steps.setSectionTitle(content.sectionTitle);
  steps.setSteps(content.items);
}

/**
 * Kept out of the JSX below on purpose: a template literal written inline would
 * carry the JSX indentation into the rendered code block. This is the first SQL
 * a reader needs — the community install is a one-time step, `LOAD` is per
 * session.
 */
const INSTALL_SAMPLE = `-- once per installation, then once per session
INSTALL duckfn_kuva FROM community;
LOAD duckfn_kuva;`;

/**
 * The other half: the functions the extension registers, called exactly like
 * DuckDB's own.
 */
const USAGE_SAMPLE = `SELECT kuva_render('{"series":[{"type":"scatter","data":[[1,2],[3,4],[5,3]]}]}');
-- an SVG document
SELECT kuva_render('{"series":[]}');
-- error: kuva_render: \`series\` must not be empty: a single-figure chart needs at least one series`;

/**
 * The shields.io badges ask for `style=flat`, which is the rounded style; the
 * default `flat-square` draws square corners and would clash with the language
 * badges below, which are rounded too. The row has to look like one set, so the
 * shape is decided at the source rather than patched with CSS.
 *
 * The row answers, in order: which DuckDB versions take the extension, how
 * widely it is installed, the latest release and the licence. The Rust toolchain
 * badge the template used to carry is a build requirement and lives in the
 * development guide instead.
 *
 * The labels that are words rather than identifiers follow the site locale, so
 * the same badge reads "Weekly Downloads" in English and "周下载" in Chinese.
 *
 * 徽章：`<owner>/<repo>` 还没填时（`REPO_URL` 仍是占位符）只显示不依赖仓库地址的那些，
 * 免得首页挂着一排坏图。
 */
function badges(repoUrl: string, locale: string): HeroBadge[] {
  const repoSlug = repoUrl.replace(/^https:\/\/github\.com\//, '');
  const hasRepo = !repoSlug.includes('<');
  const weeklyLabel = locale.startsWith('zh') ? '%E5%91%A8%E4%B8%8B%E8%BD%BD' : 'Weekly%20Downloads';
  const weeklyAlt = locale.startsWith('zh') ? 'DuckDB 社区仓的周下载量' : 'Weekly downloads from the DuckDB community extensions';
  const releaseLabel = locale.startsWith('zh') ? '&label=%E7%89%88%E6%9C%AC' : '';
  const licenseLabel = locale.startsWith('zh') ? '%E8%AE%B8%E5%8F%AF%E8%AF%81' : 'license';

  return [
    {
      href: 'https://duckdb.org',
      src: 'https://img.shields.io/badge/DuckDB-1.4%2B-fff100?logo=duckdb&style=flat',
      alt: 'DuckDB 1.4 or newer',
    },
    {
      href: 'https://duckdb.org/community_extensions/extensions/duckfn_kuva',
      src: `https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fcommunity-extensions.duckdb.org%2Fdownloads-last-week.json&query=%24.duckfn_kuva&label=${weeklyLabel}&color=blue&logo=duckdb&logoColor=a899ff&style=flat`,
      alt: weeklyAlt,
    },
    ...(hasRepo
      ? [
          {
            href: `${repoUrl}/releases`,
            src: `https://img.shields.io/github/v/release/${repoSlug}?style=flat&logo=github${releaseLabel}`,
            alt: 'Latest release',
          },
        ]
      : []),
    {
      href: `${repoUrl}/blob/main/LICENSE`,
      src: `https://img.shields.io/badge/${licenseLabel}-MIT-14459b.svg?style=flat&logo=github`,
      alt: 'MIT license',
    },
  ];
}

function heroContent(
  logoSrc: string,
  getStartedHref: string,
  repoUrl: string,
  locale: string,
  title: string,
  tagline: string,
): HeroContent {
  return {
    logoSrc,
    title,
    tagline,
    primary: {
      label: translate({id: 'homepage.getStarted', message: 'Get started'}),
      href: getStartedHref,
    },
    secondary: {
      label: translate({
        id: 'homepage.github',
        description: 'Home page button linking to the repository',
        message: 'GitHub',
      }),
      href: repoUrl,
      icon: 'simple-icons:github',
    },
    badges: badges(repoUrl, locale),
  };
}

function featuresContent(): FeaturesContent {
  return {
    sectionTitle: translate({
      id: 'homepage.features.title',
      description: 'Home page section title above the feature cards',
      message: 'Built for statistical charts',
    }),
    items: [
      {
        icon: 'lucide:sigma',
        title: translate({
          id: 'homepage.features.semantics.title',
          description: 'Home page feature card title',
          message: 'Statistical semantics, not just shapes',
        }),
        details: translate({
          id: 'homepage.features.semantics.details',
          description: 'Home page feature card description',
          message:
            'The model is seaborn and ggplot2: you say what the figure means and the extension works out the axes, the bins, the scales and the layout \u2014 the part that makes a chart statistical rather than decorative.',
        }),
      },
      {
        icon: 'lucide:shapes',
        title: translate({
          id: 'homepage.features.types.title',
          description: 'Home page feature card title',
          message: '64 chart types, composable',
        }),
        details: translate({
          id: 'homepage.features.types.details',
          description: 'Home page feature card description',
          message:
            'Scatter, histogram and box through violin, sankey, candlestick and heatmap. Overlay several series on one set of axes, or lay them out as a multi-panel figure with a shared legend and a second y-axis.',
        }),
      },
      {
        icon: 'lucide:braces',
        title: translate({
          id: 'homepage.features.sql.title',
          description: 'Home page feature card title',
          message: 'One function: JSON in, SVG out',
        }),
        details: translate({
          id: 'homepage.features.sql.details',
          description: 'Home page feature card description',
          message:
            'kuva_render is an ordinary DuckDB function: hand it a JSON chart spec and it returns a complete SVG document, with nothing to import and no host plotting library in the loop.',
        }),
      },
      {
        icon: 'lucide:monitor-down',
        title: translate({
          id: 'homepage.features.platforms.title',
          description: 'Home page feature card title',
          message: 'The same chart on every platform',
        }),
        details: translate({
          id: 'homepage.features.platforms.details',
          description: 'Home page feature card description',
          message:
            'One .duckdb_extension draws it in the CLI, a Python or R session, the JVM, or the browser (DuckDB-Wasm) \u2014 no matplotlib, no ggplot2, nothing to install on the host.',
        }),
      },
      {
        icon: 'lucide:play',
        title: translate({
          id: 'homepage.features.examples.title',
          description: 'Home page feature card title',
          message: 'Examples that run here',
        }),
        details: translate({
          id: 'homepage.features.examples.details',
          description: 'Home page feature card description',
          message:
            'Every example on this site runs in your browser through DuckDB-Wasm \u2014 click Run on the gallery or the function reference and see the real chart.',
        }),
      },
      {
        icon: 'lucide:package',
        title: translate({
          id: 'homepage.features.install.title',
          description: 'Home page feature card title',
          message: 'Install in one line',
        }),
        details: translate({
          id: 'homepage.features.install.details',
          description: 'Home page feature card description',
          message:
            'INSTALL duckfn_kuva FROM community; then LOAD duckfn_kuva; \u2014 signed builds matched to your DuckDB version and platform, so no -unsigned flag is needed.',
        }),
      },
    ],
  };
}

function nextStepsContent(
  hrefs: readonly [string, string, string, string],
): NextStepsContent {
  const [intro, installation, functions, development] = hrefs;
  return {
    sectionTitle: translate({
      id: 'homepage.next.title',
      description: 'Home page section title above the link cards',
      message: 'Where to go next',
    }),
    items: [
      {
        href: intro,
        title: translate({
          id: 'homepage.next.intro.title',
          description: 'Home page link card title',
          message: 'Introduction',
        }),
        details: translate({
          id: 'homepage.next.intro.details',
          description: 'Home page link card description',
          message: 'What duckfn_kuva adds to DuckDB, in one page.',
        }),
      },
      {
        href: installation,
        title: translate({
          id: 'homepage.next.installation.title',
          description: 'Home page link card title',
          message: 'Installation',
        }),
        details: translate({
          id: 'homepage.next.installation.details',
          description: 'Home page link card description',
          message: 'The community repository, a release file, or a local build.',
        }),
      },
      {
        href: functions,
        title: translate({
          id: 'homepage.next.functions.title',
          description: 'Home page link card title',
          message: 'Functions',
        }),
        details: translate({
          id: 'homepage.next.functions.details',
          description: 'Home page link card description',
          message: 'Every function, with an example you can run right here.',
        }),
      },
      {
        href: development,
        title: translate({
          id: 'homepage.next.development.title',
          description: 'Home page link card title',
          message: 'Development guide',
        }),
        details: translate({
          id: 'homepage.next.development.details',
          description: 'Home page link card description',
          message: 'Building, testing and releasing the extension from source.',
        }),
      },
    ],
  };
}

function CodeShowcase(): ReactNode {
  return (
    <section className={styles.sectionTint}>
      <div className={styles.sectionInner}>
        <Heading as="h2" className={styles.sectionTitle}>
          <Translate
            id="homepage.showcase.title"
            description="Home page section title above the SQL code blocks">
            Install once, then call it from SQL
          </Translate>
        </Heading>
        <p className={styles.sectionLead}>
          <Translate
            id="homepage.showcase.lead"
            description="Home page paragraph introducing the SQL code blocks">
            The extension adds ordinary SQL functions, with nothing to import at
            the call site. Install it once (left), then hand any query's data to
            kuva_render and get an SVG chart back (right).
          </Translate>
        </p>
        <div className={styles.codeGrid}>
          <CodeBlock language="sql" title="duckdb">
            {INSTALL_SAMPLE}
          </CodeBlock>
          <div className={styles.codeColumn}>
            <CodeBlock language="sql" title="duckdb">
              {USAGE_SAMPLE}
            </CodeBlock>
            {/* Balances the two columns, and explains the trailing comments. */}
            <p className={styles.codeCaption}>
              <Translate
                id="homepage.showcase.caption"
                description="Home page note under the SQL code block explaining the trailing comments">
                The comments are what each call returns \u2014 the first is the
                SVG a chart renders to, the second is how a bad spec fails.
              </Translate>
            </p>
          </div>
        </div>
        <p className={styles.showcaseLinkRow}>
          <Link className={styles.showcaseLink} to="/docs/user-guide/functions">
            <Translate
              id="homepage.showcase.link"
              description="Home page link to the function reference">
              The full function reference
            </Translate>
            {/* The official Iconify web component (registered by
                registerDfkElements()); a string `icon` attribute is all it
                needs. createElement keeps it out of the JSX namespace. */}
            {createElement('iconify-icon', {
              icon: 'lucide:arrow-right',
              className: styles.showcaseLinkArrow,
              'aria-hidden': 'true',
            })}
          </Link>
        </p>
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const {siteConfig, i18n} = useDocusaurusContext();
  const repoUrl = siteConfig.customFields?.repoUrl as string;
  const locale = i18n.currentLocale;
  // Not a hard-coded "/img/...": the site is published under /<repo>/ on GitHub
  // Pages, and only useBaseUrl adds that prefix. The dfk-* components render
  // plain anchors, so every internal href is resolved here before it is passed
  // in.
  const logoUrl = useBaseUrl('img/logo.svg');
  // The hero's primary action is "Get started", so it lands on the install
  // instructions rather than the introduction.
  const getStartedUrl = useBaseUrl('/docs/user-guide/installation');
  const nextHrefs = [
    useBaseUrl('/docs/intro'),
    useBaseUrl('/docs/user-guide/installation'),
    useBaseUrl('/docs/user-guide/functions'),
    useBaseUrl('/docs/development/quick-start'),
  ] as const;

  return (
    <Layout
      title={siteConfig.title}
      description={translate({
        id: 'homepage.meta.description',
        description: 'Home page meta description for search engines and link previews',
        message:
          'Statistical plotting in SQL: install the DuckDB extension, call kuva_render, and browse all 64 chart types.',
      })}>
      {/* Layout renders no <main> of its own: this is the page's only one. */}
      <main>
        {dfk(
          'dfk-hero',
          mountHero,
          heroContent(
            logoUrl,
            getStartedUrl,
            repoUrl,
            locale,
            siteConfig.title,
            translate({id: 'homepage.tagline', message: siteConfig.tagline}),
          ),
        )}
        {dfk('dfk-features', mountFeatures, featuresContent())}
        <CodeShowcase />
        {dfk('dfk-next-steps', mountNextSteps, nextStepsContent(nextHrefs))}
      </main>
    </Layout>
  );
}