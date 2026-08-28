# Handoff — Agent Kill-Switch Drill v0.1.0

## Delivered

- Rust single-binary CLI, `agent-kill-switch-drill`, with `init`, `validate`,
  and `drill` commands; useful `--help`; JSON output; exit codes; and typed
  public Rust structures.
- TOML profiles reference only IDs from a command allowlist. Commands are argv
  arrays rather than shell strings. Dry-run is default; `--live --confirm
  <exact-profile>` is required for action execution.
- Each stage runs its declared verification command and produces a scrubbed
  incident card. It records command IDs and statuses only, never command
  strings, output, environment, or provider secrets. A failed/unavailable
  command becomes a failed checkpoint so the card shows the weak link.
- Vite static landing/docs site with an interactive safe drill, local incident
  card export, CLI instructions, responsive dithered/halftone visual system,
  `/privacy/` and `/terms/`, and optional one-time support unlock.
- Paid-unlock contract: hosted Sociobot checkout, query-token storage and URL
  cleanup, daily cached verification, offline optimistic cache, restore field,
  revocation notice, and a gated printable tabletop worksheet. Free safety and
  export tooling is not gated.
- Original generated relay illustration is responsive WebP: 1280 px / 268 KB
  and 640 px / 45 KB. Prompt and provenance are recorded in `design.md`.

## Run and verify

```sh
cargo test
npm test
npm run build       # deploy output: ./dist/index.html
npm run build:site  # documentation output: ./dist/site/index.html
cargo package --allow-dirty
```

Manual CLI smoke test run:

```sh
cargo build --release
./target/release/agent-kill-switch-drill validate --config examples/kill-switch.toml --json
./target/release/agent-kill-switch-drill drill sample --config examples/kill-switch.toml --report /tmp/incident-card.json
```

Results verified on 2026-08-28:

- `cargo test`: 4 unit tests + 1 doctest passed.
- `npm test`: 2 static tests + 4 Playwright desktop/mobile tests passed.
  The browser tests exercise keyboard-operable controls, safe-drill state, legal
  navigation, and axe with zero serious/critical findings.
- `npm run build` and `npm run build:site`: passed. Initial JS is 4.75 KB,
  CSS 7.79 KB, hero image is at/below 300 KB.
- Production static-server Lighthouse mobile: Performance 98, Accessibility
  100, Best Practices 100, SEO 100; LCP 2.4 s, CLS 0, TBT 0 ms.
- `npm audit --omit=dev --audit-level=high`: 0 vulnerabilities.
- `cargo package --allow-dirty`: passed; package is 222.5 KiB unpacked.

## Known gaps / next steps

- The included sample commands are intentionally harmless `printf` commands.
  Teams must replace them with reviewed, versioned control-plane commands and
  run a real live drill in their own environment.
- The static paid worksheet is client-side license-gated as required by the
  static product model; licensing is not an authorization system for secrets.
- Deployment cache headers and a service worker are deployment concerns; no
  external runtime services or analytics are included in this repository.
