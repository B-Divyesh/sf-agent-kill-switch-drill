# Agent Kill-Switch Drill

Agent Kill-Switch Drill is a local Rust CLI for production teams that need to
test how they stop one agent capability and keep a record of each control
check. It is not an agent runtime, policy engine, or automatic response system.

## Run the bundled demo

Run this first. It uses the harmless sample in `examples/kill-switch.toml`,
writes a completed incident card to a new temporary directory, and prints that
path. It does not change your configuration or production controls.

```sh
cargo run -- demo
```

For machine-readable output:

```sh
cargo run -- demo --json
```

The browser sample is available at
`https://agent-kill-switch-drill.sociobot.in/?demo=1`. It shows a completed
dry-run card in an isolated `demo:agent-kill-switch-drill:*` browser-storage
namespace. Reset demo restores the sample. Start for real discards it.

## Install and run a drill

```sh
cargo install --path .
agent-kill-switch-drill init kill-switch.toml
agent-kill-switch-drill validate --config kill-switch.toml
agent-kill-switch-drill drill sample --config kill-switch.toml --report incident-card.json
```

The default is a dry run. Action commands are simulated. Declared verification
commands run and produce a confirmed or failed control-plane response. `--json`
writes the same scrubbed incident card to stdout for CI.

For a reviewed live path, name the profile twice:

```sh
agent-kill-switch-drill drill payments-write --config kill-switch.toml \
  --live --confirm payments-write --report incident-card.json
```

Exit code `0` means every declared verification confirmed. Exit code `1` means
the configuration or live confirmation was unsafe. Exit code `2` means an
action or verification failed.

## Configuration

Commands are referenced by ID. A profile can only call IDs in `[allowlist]`.
Use argv arrays, not shell strings. Review and version this configuration with
the production controls it reaches.

Every command has a timeout. It defaults to 30 seconds. Set
`timeout_seconds` from 1 to 30 for every time-sensitive rehearsal. A timeout
becomes a failed checkpoint and still produces an incident card.

```toml
version = 1

[allowlist.proxy_deny]
command = ["./ops/proxy-deny", "agent:payments:write"]
timeout_seconds = 20

[allowlist.proxy_check]
command = ["./ops/proxy-status", "agent:payments:write"]
expect_stdout = "403"
timeout_seconds = 20

[profiles.payments-write]
description = "Withdraw the payments write capability."

[[profiles.payments-write.steps]]
name = "Deny outbound tool traffic"
action = "proxy_deny"
verify = "proxy_check"
```

Reports contain command IDs, status, duration, and safe notes. They never
contain command lines, command output, environment values, or provider secrets.

## Website and support unlock

The static landing site includes the isolated sample, CLI setup, and legal
pages. Build output is `dist/`.

```sh
npm ci
npm run build
```

The free CLI, dry run, verification, and JSON export stay available. A US$39
one-time support unlock from Sociobot/Dodo adds the printable tabletop worksheet
pack. The page stores a returned license token only in browser storage and
checks it with the Sociobot license endpoint. The worksheet has no public static
URL; the browser creates the download only after a valid license result.

## Test from a clean checkout

```sh
npm ci
cargo test --all-targets
cargo test --doc
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
cargo package --locked
```

`npm test` exercises desktop and phone paths, the isolated sample, exported
incident card, license entitlement result, both color themes, legal routes,
reduced motion, and no-third-party default sample requests. The named public
claims and their commands are in `.factory/claims.json`.

## Deploy

Run `npm run build` and deploy `dist/` as the static site. The included
`staticwebapp.config.json` supplies browser security headers, immutable caching
for hashed assets, and the styled 404 response. The factory owns deployment and
billing registration; do not add payment-provider credentials to this project.

## Privacy and license

The CLI does not add telemetry. The site uses no analytics, advertising tags,
third-party fonts, or third-party scripts. Read the deployed
[/privacy](https://agent-kill-switch-drill.sociobot.in/privacy/) and
[/terms](https://agent-kill-switch-drill.sociobot.in/terms/) pages for details.

## License

MIT. See [LICENSE](LICENSE).
