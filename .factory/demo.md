# Demo sandbox

## Browser demo

Open `https://agent-kill-switch-drill.sociobot.in/?demo=1` or choose **Try it
with sample data** from the first screen. The page immediately shows the
completed `payments-write` dry run with three realistic checks: proxy deny,
credential revoke, and queue pause.

The persistent **Demo — sample data, nothing is saved** banner identifies this
mode. **Reset demo** restores the populated sample. **Start for real** deletes
`demo:agent-kill-switch-drill:drill` and returns to the empty simulator. Demo
state uses only the `demo:agent-kill-switch-drill:*` local-storage namespace;
real drill state is never read or written by the browser simulator.

The sample incident-card export is local browser data. It makes no network
request. The worksheet download remains locked until the normal license result
is valid.

## CLI demo

Run:

```sh
cargo run -- demo
```

The command copies the shipped harmless sample into a newly created temporary
directory, runs its dry drill, writes `incident-card.json` beside it, and prints
that path. It does not use or alter a user configuration. `demo --json` keeps
the card machine-readable on stdout and prints the report path on stderr.
