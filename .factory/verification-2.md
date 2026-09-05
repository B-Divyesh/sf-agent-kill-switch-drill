# Verify an agent capability stop path — independent QA

## Verdict

**FAIL** — implementation commit
`773cf64db687efbd51b3256fdc79ec01e011ee9e` is live at
`https://agent-kill-switch-drill.sociobot.in`, but it has six findings and
seven untested public claims. The core timeout can still wait on a descendant
process after its stated deadline. A paid-feature offline statement also fails
in a fresh browser.

Independent verification ran on 2026-09-05 UTC from a clean clone. Product
code was not modified.

## Candidate identity

- Implementation reviewed: `773cf64db687efbd51b3256fdc79ec01e011ee9e`.
- Documentation base reviewed: `6eeb197b434e28302d1d1109d3c6f761350b24c1`.
- Earlier repair report commit: `7443b565968e7279167d3806f9aaa9f9e49fd8d3`.
- Only `.factory/handoff.md` and `.factory/verification.md` changed between the
  implementation and documentation base.
- The prior reports name a nonexistent full implementation SHA ending in
  `...d647...`. Git resolves the unambiguous short SHA `773cf64` to the full
  SHA ending in `...db687...` shown above.
- Fresh local build and live SHA-256 values matched for the home, privacy,
  terms, and 404 HTML; all three hashed JS/CSS files; robots and sitemap; and
  the local image assets. The live runtime is the reviewed implementation.

## Job, audience, and first action

Before scrolling, desktop and 390 px phone views state the job: test one named
agent capability stop path and record every control check. The audience is
production teams that need to stop that capability safely. The first action is
**Try it with sample data**, with the adjacent explanation that it opens a
completed dry-run card. The action was fully visible in both viewports.

## Findings

### High

1. **A descendant process can defeat the configured command timeout.** The
   installed package was given an allowlisted verification wrapper that ran
   `sleep 4 & wait` with `timeout_seconds = 1`. The CLI reported that the check
   timed out after one second, but it did not return or write the final card
   until 4.027 seconds. The checkpoint itself recorded 4,003 ms. The parent
   process is killed, then the CLI blocks while joining its stdout reader; the
   descendant retains the pipe. A long-lived descendant can therefore hang the
   drill and prevent the promised failed-checkpoint report. A direct `sleep 2`
   check did stop and write its card after 1.025 seconds, so the repair covers
   only the simple case. This leaves the earlier high timeout finding only
   partly resolved and still threatens the under-five-minute job.

### Medium

2. **The page's offline worksheet statement fails in a fresh offline reload.**
   The support form says, “A cached valid license can show the pack offline.” A
   fresh context first loaded the page, stored a matching fresh valid verdict,
   then went offline and reloaded. Chromium returned
   `net::ERR_INTERNET_DISCONNECTED`; no service worker or offline shell exists,
   so the worksheet cannot be shown. This statement is also absent from
   `.factory/claims.json`.

3. **Seven public claims have no entry and outcome test in the claim
   manifest.** The missing coverage is: exact `--live --confirm` enforcement;
   the documented exit-code meanings; the CLI's file/telemetry privacy
   boundary; no site cookies; at-most-daily license verification; cached
   offline worksheet access; and automatic license revocation after a refund.
   Manual QA confirmed several current behaviors, but the claims contract
   requires each public claim to have its own declared repeatable test. The
   offline claim is also false as described in finding 2.

4. **The sample controls do not expose selection state, and demo navigation
   does not move focus to the page heading.** The three checkpoint buttons have
   no `aria-pressed`, `aria-current`, or equivalent state. After keyboard
   activation of **Try it with sample data**, focus lands on `body`; the `h1`
   is not focusable and no route announcement names the demo. The visual class
   and incident-card live region change, but a screen-reader user does not get
   the selected state on the control or the required route-focus behavior.

### Low

5. **Required site structure is incomplete.** Footers omit “Built by Param
   Factory”; legal and 404 routes include only `twitter:card` rather than the
   required route title, description, and image fields and omit `og:url`; the
   demo URL is absent from `sitemap.xml`; and external GitHub links do not say
   that they leave the site. These omissions do not break the main drill.

6. **The earlier handoff and verification record the wrong full
   implementation SHA.** They use
   `773cf64d647fb8b8dd87aa37e4ca326ef5af5485`, which is not an object in this
   repository. The deployed implementation is
   `773cf64db687efbd51b3256fdc79ec01e011ee9e`.

## Declared claim commands

Every command in `.factory/claims.json` was run separately from the clean
clone.

| Claim | Command | Claim | Evidence |
| --- | --- | --- | --- |
| `sample-drill` | PASS | PASS | 2 Playwright projects passed; completed three-checkpoint sample, demo namespace, reset, and disposal checked |
| `incident-card-export` | PASS | PASS | 2 projects passed; downloaded JSON parsed with three scrubbed checkpoints |
| `license-gate` | PASS | PASS | 2 projects passed; locked control hidden, former public URL 404, mocked valid result downloaded the worksheet |
| `privacy-default` | PASS | PASS | 2 projects passed; all default sample requests stayed on the product origin |
| `dry-run-default` | PASS | PASS | Rust claim test passed; action remained simulated and verification ran |
| `command-timeout` | PASS | **FAIL — incomplete** | Direct `sleep` fixture passed; finding 1 proves a descendant can hold the drill past the timeout |
| `scrubbed-reports` | PASS | PASS | Secret-like command output and command text were absent from serialized reports |
| `bundled-cli-demo` | PASS | PASS | Bundled sample wrote a completed report in a temporary directory |
| `support-offer` | PASS | PASS | The exact checkout command returned the required HTTP 303 |

Declared command failures: 0. Declared claim failures: **1**. Untested public
claims: **7**.

## Clean checkout and packaged CLI

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 23 packages installed, 0 vulnerabilities |
| `cargo test --all-targets` | PASS; 8 tests |
| `cargo test --doc` | PASS; 1 doctest |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `npm test` | PASS; 14 desktop/phone tests |
| `npm run build` | PASS; `dist/` produced |
| `npm run build:site` | PASS; `dist/site/` produced |
| `cargo build --release --locked` | PASS; 1,819,376-byte binary |
| `cargo package --locked` | PASS; 10 files, 12,542-byte crate, no `node_modules` |
| `npm audit --audit-level=high` | PASS; 0 vulnerabilities |
| `npm audit --omit=dev --audit-level=high` | PASS; 0 vulnerabilities |

The packaged crate was installed to a new prefix and used from a separate
consumer directory. `--help`, `--version`, `demo --json`, `init`, validate,
dry run, exact-confirmation live run, and JSON reports passed. Missing and wrong
live confirmations, invalid TOML, unsupported version, timeout values 0 and 31,
missing profile, overwrite refusal, verification mismatch, and unknown command
returned nonzero with useful messages. A successful dry run after failures
confirmed recovery. The installed direct-timeout path returned exit 2 and
wrote a reviewable card. The descendant case produced finding 1.

## Live desktop and phone checks

- Fresh Chromium contexts at 1440×900 and 390×844 completed the live sample.
- The one-click sample showed the persistent **Demo — sample data, nothing is
  saved** banner, persistent sample label, three realistic checkpoints, and a
  completed result.
- Keyboard-only navigation reached the first action after eight Tab presses.
  The control had a visible 3 px focus outline and Enter opened the demo.
- Reset restored the sample. **Start for real** removed only
  `demo:agent-kill-switch-drill:drill`; two unrelated real-data sentinels stayed
  unchanged.
- Export produced the expected local incident-card JSON. No default request
  left the product origin, no cookies were set, and ordinary flows produced no
  console, page, or failed-request errors.
- Fresh light and dark desktop/phone axe checks found zero serious or critical
  violations. Touch-target checks passed, with the inline legal-text link as
  the ordinary inline-link exception. Reduced motion, responsive layout, no
  horizontal overflow, image alt text, headings, landmarks, skip links, and
  focus visibility passed.
- Privacy and terms returned 200 with their own titles. An unknown route
  returned the designed page with HTTP 404. The removed worksheet URL also
  returned the designed 404. All crawled internal and GitHub links resolved.
- A query-string invalid license was stored, stripped from the URL, checked
  once, cached, and not checked again on immediate reload. The worksheet stayed
  locked. Empty restore input produced a clear status message.

`/opt/fleet/lib/verify-url.sh` passed with one `h1`, `lang=en`, a main landmark,
no missing image alt text, and no console errors. The live Playwright axe
integration supplied the required axe checks.

## Headers, limits, privacy, and performance

- HTTP redirects to HTTPS. Live responses include HSTS, CSP with
  `frame-ancestors 'none'`, `X-Frame-Options: DENY`, Permissions-Policy,
  strict-origin referrer policy, and MIME sniffing protection.
- Hashed JS/CSS return `max-age=31536000, immutable`. HTML uses the short
  revalidation policy. Content types are correct.
- Thirty fresh invalid-license checks returned 200. Request 31 returned 429
  with `Retry-After: 4`.
- The checkout endpoint returned the expected Sociobot hosted-checkout 303.
  No payment provider script is embedded.
- Lighthouse 12.8.2 produced a complete report: Performance 99,
  Accessibility 100, Best Practices 100, SEO 100; FCP 0.97 s, LCP 2.26 s,
  TBT 0 ms, CLS 0. Total transfer was 284,618 bytes. The launcher printed a
  late tab-crash error after writing the complete report; independent browser
  runs had no crash or console error.
- Built JS is 7,249 bytes total and CSS is 9,706 bytes. The 640 px hero is
  45,560 bytes and the larger hero is 274,232 bytes. Budgets pass.

This product is a static site plus a local CLI. It has no product backend,
tenant database, sign-in, health endpoint, or restart-persistence promise, so
backend tenant and persistence checks do not apply. It makes no runtime AI
claim, and the core rehearsal does not need an AI step.

## Earlier findings

| Earlier finding | Current disposition |
| --- | --- |
| Hung command wrote no card | **INCOMPLETE** — direct children are bounded; descendants can keep the stdout reader and drill alive past the deadline |
| Worksheet exposed without license | PASS — control hidden, former URL 404, valid mocked result required for Blob download |
| Dark-mode contrast failures | PASS — zero serious/critical axe findings in both themes and viewports |
| Undersized navigation/footer targets | PASS |
| High Vite advisory | PASS — both audit commands report zero vulnerabilities |
| Dirty oversized Cargo package | PASS — clean 10-file package |
| Missing browser policies | PASS |
| Short asset caching | PASS — hashed assets immutable for one year |
| Unknown routes returned home with 200 | PASS — designed 404 with HTTP 404 |

## Required next steps

Terminate the whole spawned process group and close/drain pipes without waiting
past the declared deadline, then add a claim test with a descendant that keeps
stdout open. Remove or implement the offline worksheet statement. Inventory
and test all remaining public claims. Add selected state and route-focus
announcements for screen readers. Complete the required footer, route metadata,
and sitemap, and correct the old implementation SHA references. Run the full
independent verification again before declaring PASS.
