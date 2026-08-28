# Agent Kill-Switch Drill

Agent Kill-Switch Drill is a local-first Rust CLI for rehearsing a staged, per-capability stop sequence for production tool-calling agents. It records whether declared control planes could be reached and verified; it is not an agent runtime or policy engine.

## Install and quick start

```sh
cargo install --path .
agent-kill-switch-drill init kill-switch.toml
agent-kill-switch-drill validate --config kill-switch.toml
agent-kill-switch-drill drill sample --config kill-switch.toml --report incident-card.json
```

The default run is a **dry run**: action commands are never executed. Declared verification commands do run, showing a confirmed or failed control-plane response. `--json` writes the same scrubbed incident card to stdout for CI.

To run a reviewed live path, name the profile twice:

```sh
agent-kill-switch-drill drill payments-write --config kill-switch.toml \
  --live --confirm payments-write --report incident-card.json
```

## Configuration

Commands are referenced by ID. A profile can only call IDs in `[allowlist]`; reports contain command IDs and statuses, never command lines, output, environment, or provider secrets.

```toml
version = 1
[allowlist.proxy_deny]
command = ["./ops/proxy-deny", "agent:payments:write"]
[allowlist.proxy_check]
command = ["./ops/proxy-status", "agent:payments:write"]
expect_stdout = "403"
[profiles.payments-write]
description = "Withdraw the payments write capability."
[[profiles.payments-write.steps]]
name = "Deny outbound tool traffic"
action = "proxy_deny"
verify = "proxy_check"
```

Commands use argument arrays, never shell strings. Review and version this config with the same care as the production controls it invokes.

## Exit codes

- `0` — every requested verification confirmed.
- `1` — invalid config, missing profile, or unsafe live confirmation.
- `2` — one or more action/verification commands failed.

## Website and checks

The static landing site includes a keyboard-operable safe drill simulator, install instructions, and the optional one-time support unlock.

```sh
npm install
npm run build:site # writes dist/site
npm test
cargo test
npm run build      # writes dist/index.html
cargo package --allow-dirty
```

No telemetry is collected. The site stores a license token locally only if you choose to unlock the optional support tier.

## License

MIT. See [LICENSE](LICENSE).
