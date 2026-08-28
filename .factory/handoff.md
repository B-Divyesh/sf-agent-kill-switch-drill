# Verification handoff — Agent Kill-Switch Drill

## Result

**FAIL**

- Candidate: `78bac1779d31c4cdfef76b2bf0a1ed68cc2b28ef`
- Live URL: `https://agent-kill-switch-drill.sociobot.in`
- Verified: 2026-08-28 UTC
- Full evidence: [verification.md](verification.md)

The live HTML, hashed JS/CSS, robots file, and both images match a fresh local
production build byte for byte. The result is therefore based on the candidate
itself, not a stale deployment or a deployment-only outage.

## Release blockers

1. CLI commands have no timeout. An unresponsive verification can hang forever
   and produce no incident card, violating the core under-five-minute job.
2. The paid worksheet is visibly downloadable without a license because the
   `.button` display rule overrides the link's `hidden` attribute.
3. Dark mode has an axe serious contrast violation across 21 nodes on both
   desktop and 390 px mobile (ratios down to 1.21:1).

Additional defects: navigation/footer touch targets are below 44 px; the Vite
dev dependency has a high-severity advisory; clean crate packaging fails after
`npm ci` and the forced crate contains 36 `node_modules` README/LICENSE files;
the live origin lacks CSP, Permissions-Policy, and clickjacking policy; hashed
assets cache for only 30 seconds; unknown URLs return 200.

## What passed

- `cargo test --all-targets`: 4 passed.
- `cargo test --doc`: 1 passed.
- Rust fmt and clippy with warnings denied: passed.
- `npm test`: 2 Node + 4 Playwright tests passed.
- `npm run build` and `npm run build:site`: passed; `dist/` produced.
- Release build and `cargo package --locked --allow-dirty`: passed.
- The packaged CLI installed in a clean prefix; a separate Rust consumer used
  its public API successfully.
- Normal dry-run/live flows, invalid input, refusal to overwrite, confirmation
  guard, exit codes, JSON output, failure reporting, and secret scrubbing were
  exercised successfully apart from the timeout blocker.
- Light-theme and legal-page axe scans had zero serious/critical findings;
  keyboard flow, visible focus, responsive reflow, console/page errors, and
  reduced motion otherwise passed.
- Default page load contacted only the product origin and set no cookies.
- License URL cleanup, local storage, invalid-token locking, and one-day cache
  behavior passed logically.
- Verify endpoint rate limit passed: requests 1–30 returned 200; request 31
  returned 429 with `Retry-After: 4`.
- Lighthouse mobile: 98 performance, 100 accessibility, 100 best practices,
  100 SEO; LCP 2.3 s, TBT 20 ms, CLS 0.

## Reproduce

```sh
npm ci
cargo test --all-targets
cargo test --doc
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
npm audit --audit-level=high
cargo package --locked
```

The final two commands expose the dependency and packaging failures. See the
full verification report for isolated-package, CLI boundary, live-browser,
headers, hashes, Lighthouse, and rate-limit evidence.

## Next steps

Add per-command timeouts and timeout reports; restore actual paid gating; fix
dark-theme contrast and target sizes; update Vite; root-anchor Cargo package
includes; add CSP/Permissions-Policy/frame protection; set immutable caching
for hashed assets; and return 404 for unknown paths. Then run a fresh independent
verification against the replacement commit and deployment.
