# Development

```sh
cargo test                     # unit, protocol and end-to-end ACP tests
cargo fmt --check && cargo clippy --all-targets --locked -- -D warnings
```

The tests start the real binary over stdio and point it at a mock chat completions server, so they
need no API key or network. `ONDE_CODE_DATA_DIR` isolates the stored key and sessions.
`crates/cli/tests/acp_integration.rs` drives the agent with the SDK's `Client`, and
`crates/cli/tests/acp_protocol.rs` sends raw JSON-RPC. CI runs fmt, clippy and the tests on Linux
and macOS.

Live tests run against Onde Inference and cost real tokens. They skip themselves without a key:

```sh
ONDE_API_KEY=app-id:app-secret cargo test --test onde_live -- --nocapture
```

## Where things live

| Path | |
|------|---|
| `crates/cli/src/main.rs` | command line: flags, `--login`, `--setup`, `--list-models`, and which mode to run |
| `crates/agent/src/profile.rs` | the coding agent's prompt, tools and `/models` and `/setup` commands |
| `crates/agent/src/auth.rs`, `crates/agent/src/browser_auth.rs` | "Sign in with Onde" and `--login`: loopback and PKCE against ondeinference.com/connect |
| `crates/cli/src/legacy.rs` | moves sessions saved by Onde Code 1.0 into the current store |
| `packaging/registry/` | the ACP registry entry and the script that updates it |
| `packaging/npm/` | templates and the launcher for the npm packages |

The ACP server, the chat completions client, the workspace tools, sessions on disk, MCP servers and
the terminal UI come from the Onde Agent Platform crates
[`ed-acp`](https://crates.io/crates/ed-acp) and [`ed-acp-tui`](https://crates.io/crates/ed-acp-tui).
This repo supplies what makes it a coding agent: the profile, the sign-in and the command line.

## Releases

Push a `v*` tag that matches the version in the root `Cargo.toml` (`[workspace.package]`).
`release-github.yml` builds the five targets and attaches the archives and `checksums.txt` to the
GitHub release. `release-npm.yml` publishes `@ondeinference/onde-code` and the five platform
packages with npm Trusted Publishing. After a release, the ACP registry picks up the new version
by itself.
