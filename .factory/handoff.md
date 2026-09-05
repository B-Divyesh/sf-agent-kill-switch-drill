# Handoff — Agent Kill-Switch Drill repair

## Result

**PASS** for implementation commit `773cf64d647fb8b8dd87aa37e4ca326ef5af5485`.

- Live URL: `https://agent-kill-switch-drill.sociobot.in`
- Deployment: Azure Static Web App `sf-agent-kill-switch-drill`, production
  upload succeeded on 2026-09-05 UTC.
- Documentation and verification records are committed after the implementation
  commit. See this file’s Git history for their separate SHA.

## Job, audience, and first action

The product helps production teams test how they stop one named agent
capability and record each control check. On desktop and a 390 px phone, the
first screen states that job, identifies production teams as the audience, and
shows **Try it with sample data** before scrolling. The action opens a completed
`payments-write` dry run.

## Repaired findings

- Every allowlisted command now has `timeout_seconds`, defaulting to 30 and
  limited to 1–30 seconds. A timeout kills the child command, becomes a failed
  checkpoint, returns exit code 2, and still writes the incident card.
- The worksheet is no longer a public static URL. A locked visitor has no
  visible control and `/tabletop-worksheet.txt` returns 404. A valid license
  result enables a local Blob download.
- Dark mode now has a dedicated drill surface, readable light incident card,
  and contrast-safe status tokens. Fresh desktop and phone axe scans found zero
  serious or critical violations in light and dark themes.
- Header and footer links now meet the 44 px touch-target baseline.
- Vite is updated to 8.2.2; `npm audit --audit-level=high` reports zero
  vulnerabilities.
- Cargo package paths are anchored. `cargo package --locked` now succeeds with
  10 project files and no `node_modules` content.
- The static site now ships CSP, Permissions-Policy, frame protection,
  immutable cache policy for hashed assets, sitemap/robots, route metadata,
  generated sharing assets, and a designed 404 that returns HTTP 404.
- The landing page now has an isolated browser demo and the CLI has `demo`,
  which runs the bundled harmless sample in a temporary directory.

## Verification

From the documented clean setup:

```sh
npm ci
cargo test --all-targets
cargo test --doc
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
npm run build:site
cargo package --locked
npm audit --audit-level=high
npm audit --omit=dev --audit-level=high
```

All commands passed. `npm test` ran 14 Playwright checks across desktop and
phone. Every command in `.factory/claims.json` was run individually and passed.
The packaged crate was installed into a separate temporary prefix; its installed
`demo --json` produced a completed three-checkpoint card.

Fresh live Chromium contexts verified desktop and phone behavior: one heading,
language and landmark structure, visible focus, first-screen primary action,
sample banner and label, reset and disposal, incident-card download, locked
worksheet, no cookies, no third-party default requests, legal routes, and zero
serious/critical axe findings in both themes. Live headers include CSP,
Permissions-Policy, `X-Frame-Options: DENY`, and immutable caching on hashed
assets. The live unknown route returned the styled 404; the former worksheet URL
returned 404.

License verification allowance was checked again: 30 invalid requests returned
200 and request 31 returned 429 with `Retry-After: 3`. The existing checkout
endpoint returned its hosted-checkout 303, so billing registration is active.

Lighthouse mobile on the live origin: Performance 99, Accessibility 100, Best
Practices 100, SEO 100; FCP 1.0 s, LCP 2.3 s, TBT 0 ms, CLS 0. Lighthouse
printed a late browser-tab-crash launcher message after producing that complete
report; the independent Playwright browser pass had no console or page errors.

## Known limits and next steps

- The worksheet UI and public static path are license-gated. A static browser
  product cannot make a document delivered in its JavaScript bundle confidential
  from a determined person with browser developer tools. A truly protected paid
  asset would require a product backend; none was added because the product is
  intentionally static and local-first.
- The sample commands are harmless `printf` calls. Teams must replace them with
  reviewed, versioned control-plane commands and rehearse an approved live path.
- The factory owns registry publication and billing administration. The package
  is ready for `cargo publish`; no publish or payment action was taken here.
