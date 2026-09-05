# Handoff — Agent Kill-Switch Drill verification 2

## Result

**FAIL** for implementation commit
`773cf64db687efbd51b3256fdc79ec01e011ee9e` at
`https://agent-kill-switch-drill.sociobot.in`.

Independent QA started from documentation commit
`6eeb197b434e28302d1d1109d3c6f761350b24c1`. The earlier repair report is
`7443b565968e7279167d3806f9aaa9f9e49fd8d3`. The live build matches the
implementation byte-for-byte. The earlier documents used an incorrect full
implementation SHA; the full SHA above is the value resolved by Git.

## What was done

No product code was changed. Fresh desktop and phone browsers, keyboard and
screen-reader-relevant states, both themes, reduced motion, the sample sandbox,
reset/disposal, exports, legal pages, links, the styled 404, headers, caching,
privacy requests, checkout, rate limiting, and Lighthouse were checked.

Every declared claim command passed separately. A clean clone passed the full
Rust, Playwright, build, package, format, lint, and audit gates. The crate was
installed to a clean prefix and the installed CLI was exercised through normal,
invalid, boundary, live-confirmation, failure, and recovery paths.

## Findings

There are six findings and seven untested public claims. The release-blocking
issue is an incomplete command timeout: killing a wrapper does not kill a
descendant that holds stdout, so a one-second timeout waited 4.027 seconds and
can wait indefinitely before the report is completed. The page also makes an
offline worksheet statement that fails on offline reload. Claim coverage,
screen-reader state/focus, required footer and route metadata, and the earlier
full-SHA record need correction.

See `.factory/verification-2.md` for commands, evidence, severities, and the
disposition of every earlier finding.

## How to verify

From a clean clone:

```sh
npm ci
cargo test --all-targets
cargo test --doc
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
npm run build:site
cargo build --release --locked
cargo package --locked
npm audit --audit-level=high
npm audit --omit=dev --audit-level=high
```

Then install the unpacked crate into a new prefix and exercise `demo --json`,
validation, dry run, guarded live run, failure reports, timeout boundaries, and
recovery. The missing process-tree case must use a wrapper that spawns a child
which keeps stdout open.

## Next steps

- Make timeout enforcement cover the spawned process tree and stdout pipes.
- Remove or implement and test the offline worksheet statement.
- Add manifest entries and outcome tests for all seven public claims.
- Expose checkpoint selection state and move/announce focus on demo navigation.
- Complete footer attribution, route metadata, sitemap coverage, and SHA docs.
- Repeat independent QA. Do not deploy or publish from this verification task.
