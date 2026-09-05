# Independent verification — Agent Kill-Switch Drill

## Repair verification — 2026-09-05 UTC

Implementation commit `773cf64d647fb8b8dd87aa37e4ca326ef5af5485` replaced the
failed candidate documented below. It was built, pushed, and deployed to the
same production URL. The implementation differs from these later verification
records; consult Git history for the documentation-record SHA.

| Earlier finding | Current disposition |
| --- | --- |
| Hung CLI command wrote no card | Fixed: bounded 1–30 second command timeouts produce a failed checkpoint and saved card. |
| Worksheet exposed without a license | Fixed: locked control is hidden with a real `[hidden]` rule; no public worksheet file is deployed; valid verification enables a Blob download. |
| 21 dark-mode contrast failures | Fixed: fresh light and dark desktop/phone axe scans report zero serious/critical violations. |
| Undersized navigation/footer targets | Fixed: header and footer interactions are at least 44×44 px. |
| High Vite advisory | Fixed: Vite 8.2.2 and both full and production audit commands report zero vulnerabilities. |
| Dirty oversized Cargo package | Fixed: root-anchored includes yield a clean 10-file package. |
| Missing browser policies | Fixed: live CSP, Permissions-Policy, `X-Frame-Options`, referrer, and MIME headers verified. |
| Short asset caching | Fixed: live hashed assets return `max-age=31536000, immutable`. |
| Unknown routes returned home with 200 | Fixed: live unknown route returns styled 404 with HTTP 404. |

Additional repair verification: clean setup checks, all declared claim commands,
isolated packaged-CLI demo, live browser sample/reset/disposal, HTTPS headers,
checkout redirect, and rate limit (30 × 200 then 429 with `Retry-After: 3`)
passed. Lighthouse mobile was 99/100/100/100 for performance, accessibility,
best practices, and SEO (LCP 2.3 s; CLS 0).

## Verdict

**FAIL** — candidate `78bac1779d31c4cdfef76b2bf0a1ed68cc2b28ef` is deployed at
`https://agent-kill-switch-drill.sociobot.in`, but it does not meet the
acceptance contract. The packaged CLI works for ordinary drills, while an
unresponsive control-plane command can hang indefinitely without producing an
incident card. The live site also exposes the paid worksheet without a license
and has serious dark-theme contrast failures.

Verified independently on 2026-08-28 UTC from a clean working tree at the
candidate commit. Product code was not modified.

## Defects

### High

1. **A hung command can prevent the drill from ever completing or clearly
   failing.** Neither `AllowedCommand` nor `run_allowed` has a timeout. A dry-run
   profile whose verification was `["sleep", "60"]` was still running until
   killed externally after 2 seconds (`timeout` exit 124), and no report was
   written. This violates the brief's under-five-minute success measure and is
   unsafe for the exact control-plane outage scenario the tool rehearses.

2. **The paid worksheet is visible and downloadable without a license.** On a
   clean browser profile, `#worksheet-download` has the `hidden` attribute and
   `hidden === true`, but its computed display is `inline-flex` and it is
   rendered on desktop and mobile because `.button { display:inline-flex }`
   overrides the browser's hidden rule. The link points directly to
   `/tabletop-worksheet.txt`, which returns HTTP 200 without authorization.

3. **Dark mode fails the non-negotiable accessibility baseline.** axe reports
   one serious `color-contrast` rule affecting 21 nodes at both 1440 px and
   390 px. Measured examples include 1.21:1 for drill explanatory copy, 1.32:1
   for stage details, 1.39:1 for the active stage label, and 2.68:1 for the
   primary CTA, all below 4.5:1. Visual inspection confirms that much of the
   drill card becomes nearly unreadable.

### Medium

4. **Mobile and navigation targets are undersized.** At 390 px, the primary
   navigation and footer links are 20 px high, and the home wordmark is 35 px
   high. The acceptance baseline requires 44×44 CSS px touch targets.

5. **The installed development toolchain has a high-severity vulnerability.**
   `npm audit --audit-level=high` exits 1 for Vite 7.1.7, listing multiple file
   read / deny-bypass advisories. This is dev-server exposure rather than code
   shipped in the static bundle; `npm audit --omit=dev --audit-level=high`
   passes.

6. **The crate is not cleanly packageable after the documented install flow.**
   After `npm ci`, `cargo package --locked` exits 101 because Cargo's broad
   `README.md`/`LICENSE` include globs select 36 ignored files under
   `node_modules`. `cargo package --locked --allow-dirty` succeeds, but the
   resulting 65,611-byte crate unnecessarily contains those third-party files.

7. **Browser policy hardening is incomplete.** The live origin sends HSTS,
   `Referrer-Policy: strict-origin-when-cross-origin`, and
   `X-Content-Type-Options: nosniff`, but no Content-Security-Policy,
   Permissions-Policy, or clickjacking policy (`frame-ancestors` or
   `X-Frame-Options`). This is material because the page stores a paid license
   token in local storage.

### Low

8. **Hashed static assets are not cached immutably.** HTML, hashed JS/CSS, and
   images all return `Cache-Control: public, must-revalidate, max-age=30`.
   Hashed assets should use a long immutable lifetime.

9. **Unknown paths return the product page with HTTP 200.** For example,
   `/does-not-exist` returns the home document instead of 404, weakening error
   semantics and crawl quality.

## Candidate and deployment identity

- Clean checkout HEAD: `78bac1779d31c4cdfef76b2bf0a1ed68cc2b28ef`.
- `origin/main` and `git ls-remote origin refs/heads/main` initially resolved to
  the same candidate.
- A fresh `npm run build` passed and produced `dist/`.
- Live and local SHA-256 hashes matched exactly for the built home document,
  hashed JS, hashed CSS, `robots.txt`, and both relay images:
  - `index.html`: `5fc6fa9bfefc9f00c02eff4c1808ec0ed2cb96730db1eb7db6afaa1c28ad68a8`
  - `main-BYrUjKjC.js`: `903d0e43e083d3fc5875a7ba25618e3fdfe32807c0241bb74ad7d4f28434c3b2`
  - `style-DXre964V.css`: `6400132bc8d581cb62355437759af2160e6fd282a101414036f0a412139c3fc3`
  - `robots.txt`: `16ceb5ee3e0dc13aa9adf31a3ebbe45a1d965b8c2b9f72eaf84e5911e140ed95`
  - 640 px image: `839cdc7828610e3768517afe899c1c3a8fa8af4e3a8cd800f348aa0f908619bc`
  - 1280 px image: `5b092d9b46a29f746b07e6da343856d8d1e6fc1f97d9a989719756dfe2778548`

The live deployment therefore matches the candidate; this is not a stale or
deployment-only failure.

## Local quality gates

| Check | Result | Evidence |
| --- | --- | --- |
| `npm ci` | PASS with warning | 21 packages installed; audit reported 1 high-severity dev dependency issue |
| `cargo test --all-targets` | PASS | 4 unit tests passed |
| `cargo test --doc` | PASS | 1 doctest passed |
| `cargo fmt --all -- --check` | PASS | no diff |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS | no warnings |
| `npm test` | PASS | 2 Node tests and 4 Playwright desktop/mobile tests passed |
| `npm run build` | PASS | exact production build wrote `dist/` |
| `npm run build:site` | PASS | documentation build completed |
| `cargo build --release --locked` | PASS | 1,760,128-byte binary |
| `cargo package --locked` | FAIL | 36 ignored `node_modules` README/LICENSE files treated as dirty package inputs |
| `cargo package --locked --allow-dirty` | PASS with defect | 65,611-byte crate, 46 files, including the unwanted `node_modules` files |
| `npm audit --audit-level=high` | FAIL | 1 high-severity Vite advisory group |
| `npm audit --omit=dev --audit-level=high` | PASS | 0 production dependency vulnerabilities |

There is no repository TypeScript typecheck or separate JavaScript lint script;
Rust formatting and clippy were run as the available lint/type-quality gates.

## Packaged CLI and public API

The `.crate` was extracted into a clean temporary directory, installed with
`cargo install --path ... --root ... --locked`, and exercised through the
installed binary. A separate clean Rust consumer depended on the extracted
package and successfully used the public `parse_config`, `validate`,
`run_drill`, and `State` API.

- `--version` returned `0.1.0`; root and subcommand help are usable.
- `init`, `validate --json`, default dry-run, report writing, and `--json`
  completed successfully.
- The sample dry-run returned 0 with three simulated actions, three passed
  verifications, and `all_confirmed: true`.
- Exact `--live --confirm sample` returned 0 with three passed actions and
  verifications.
- Missing/wrong live confirmations were rejected with exit 1 before actions.
- Invalid TOML, unsupported version, unknown command, empty profile, missing
  profile, and init-overwrite attempts all returned exit 1 with useful errors.
- `--no-verify` and a failing verification returned exit 2 and
  `all_confirmed: false`.
- A command and output containing `PROVIDER_SECRET=ultra-secret-*` did not
  appear in stdout JSON or the saved incident card.
- A hanging verification exposed defect 1 above.

## Live browser QA

Fresh Chromium contexts were used at 1440×900 and 390×844. All interactive
flows below were driven by keyboard only where applicable.

- HTTP 200; correct title, `lang=en`, one `h1`, one `main`, and no horizontal
  overflow at either viewport.
- Theme toggle, drill execution, incident-card download, and empty-license
  validation were reachable and operable with Tab/Enter.
- Focus was visibly rendered as a solid 3 px outline.
- The downloaded card had the expected schema, `dry_run` mode, three
  checkpoints, and `all_confirmed: true`.
- Light theme: zero axe serious/critical findings on desktop and mobile.
- Dark theme: serious contrast failure on 21 nodes on both viewports.
- Privacy and terms pages: HTTP 200, correct title/landmarks, no overflow, no
  console errors, and zero axe serious/critical findings at 390 px.
- No console errors, page errors, or failed requests on the ordinary flow.
- Default load made four requests, all to the product origin, and set no
  cookies. No analytics, external fonts, or third-party scripts loaded.
- Reduced-motion emulation matched the media query, changed smooth scrolling
  to `auto`, and reduced transition/animation durations to 1 ms.
- A query-string license was saved under
  `sb_license:agent-kill-switch-drill`, stripped from the URL, verified once,
  cached as invalid, and not reverified on immediate reload. The invalid token
  kept the paid state locked logically, although defect 2 still renders the
  download link.
- Visual inspection of full-page desktop and mobile captures confirmed the
  intended responsive stacking and the dark-theme readability failure.

This is a static site, not a PWA: no service worker/offline shell test applies.
It has no sign-in, so the Entra authority requirement does not apply. It has no
product-hosted backend, so backend concurrency, persistence, and health/build
identity checks do not apply.

## Network, privacy, rate limiting, and performance

- HTTP redirects to HTTPS. The live site sends HSTS
  (`max-age=10886400; includeSubDomains; preload`), strict-origin referrer
  policy, MIME sniffing protection, and correct content types.
- The optional verify request goes only to
  `https://api.sociobot.in/api/v1/products/agent-kill-switch-drill/verify`, uses
  `Cache-Control: no-store`, and allows the exact product origin through CORS.
- Rate-limit burst: 30 rapid invalid-license GET requests returned 200; request
  31 returned 429 with `Retry-After: 4` and `X-RateLimit-After: 4`. **PASS**.
- Checkout is the required Sociobot endpoint and redirects with HTTP 303 to
  hosted Dodo checkout; no payment provider is embedded in product code.
- Lighthouse 12.8.2 mobile: Performance 98, Accessibility 100, Best Practices
  100, SEO 100; FCP 1.0 s, LCP 2.3 s, Speed Index 1.1 s, TBT 20 ms, CLS 0.
  Lighthouse tests initial light mode and therefore does not supersede the
  explicit dark-mode axe failure.
- Initial transferred resources measured 282,863 bytes, dominated by the
  274,497-byte 1280 px hero transfer. Built assets are 4,913 bytes JS, 7,790
  bytes CSS, 45,560/274,232 bytes images; JS/CSS/image budgets pass.

## Required disposition

Do not release this candidate. Add bounded command execution with a clear
failed checkpoint/report, fix the paid link's hidden styling and verify the
locked state, repair dark-theme contrast and mobile target sizes, update Vite,
tighten crate include patterns, add browser security headers, and configure
long-lived immutable caching for hashed assets. Re-run the full verification
after those changes.
