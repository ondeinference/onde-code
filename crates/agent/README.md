<p align="center">
  <img src="https://raw.githubusercontent.com/ondeinference/onde/refs/heads/main/assets/onde-inference-logo.svg" alt="Onde Inference" width="96">
</p>

<h1 align="center">onde-code-agent</h1>

<p align="center">
  Onde Code as a library, for apps that embed the agent.
</p>

<p align="center">
  <a href="https://ondeinference.com"><img src="https://img.shields.io/badge/ondeinference.com-235843?style=flat-square&labelColor=17211D" alt="Website"></a>
  <a href="https://www.npmjs.com/package/@ondeinference/onde-code"><img src="https://img.shields.io/npm/v/@ondeinference/onde-code?style=flat-square&labelColor=17211D&color=235843" alt="npm"></a>
  <a href="https://crates.io/crates/onde-code-agent"><img src="https://img.shields.io/crates/v/onde-code-agent?style=flat-square&labelColor=17211D&color=235843" alt="Crates.io"></a>
  <a href="https://github.com/ondeinference/onde-code/releases"><img src="https://img.shields.io/github/v/release/ondeinference/onde-code?style=flat-square&labelColor=17211D&color=235843" alt="Release"></a>
  <a href="https://github.com/ondeinference/onde-code/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/ondeinference/onde-code/ci.yml?style=flat-square&labelColor=17211D&color=235843&label=CI" alt="CI"></a>
  <a href="https://github.com/ondeinference/onde-code/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-235843?style=flat-square&labelColor=17211D" alt="License"></a>
</p>

<p align="center">
  <a href="https://github.com/ondeinference/onde-swift">Swift</a> · <a href="https://pub.dev/packages/onde_inference">Flutter</a> · <a href="https://www.npmjs.com/package/@ondeinference/react-native">React Native</a> · <a href="https://crates.io/crates/onde">Rust</a> · <a href="https://ondeinference.com">Website</a>
</p>

---

Onde Code as a library, for apps that embed the agent instead of launching the
[`onde-code`](https://crates.io/crates/onde-code) binary.

It is an [`ed-acp`](https://crates.io/crates/ed-acp) agent profile. A host serves it in process over
an in-memory channel and talks to it with an [Agent Client Protocol](https://agentclientprotocol.com)
client on the other end. That lets a sandboxed app (a Mac App Store build, an iOS app) run the agent
without spawning a subprocess.

## What is in the crate

- `OndeCode`, the `ed_acp::Profile`: the system prompt, the workspace tools and the `/models` and
  `/setup` commands. `OndeCode::new()` includes `run_command`. `OndeCode::without_shell()` is for a
  host that cannot start processes. It drops `run_command` and tells the model so.
- `INFO`, the agent's `AgentInfo`: name, display name, version and the `ONDE_CODE` env prefix.
- `auth::OndeAuth`, the profile's `AuthProvider`. It offers the "Sign in with Onde" method, accepts
  a key that already works from the environment or the stored file, and otherwise runs the browser
  flow.
- `browser_auth`, the flow behind it: a loopback redirect with PKCE against ondeinference.com. The
  app key comes back through a one-time code and never appears in a URL.

Inference goes to Onde Inference (`https://cloud.ondeinference.com/v1`, or `ONDE_BASE_URL`), or to
any OpenAI API compatible endpoint when `ONDE_CODE_PROVIDER=openai`.

## Serving it

```rust
use std::sync::Arc;

use agent_client_protocol::Channel;
use ed_acp::ServeOptions;
use onde_code_agent::OndeCode;

let (agent_end, client_end) = Channel::duplex();
let opts = ServeOptions { yolo: false, surface: "app" };

tokio::spawn(ed_acp::serve(Arc::new(OndeCode::without_shell()), opts, agent_end));
// Connect an ACP client to `client_end`, then initialize, authenticate and open a session.
```

`ed_acp::serve` is a future, so this runs inside the host's own async runtime. The example assumes
a `tokio` runtime, so the host needs `tokio` as a dependency. One call serves one connection: call
`serve` again with a new channel for the next client.

## Where the key lives

The stored key and the sessions live in the `ondecode` config dir (`~/Library/Application
Support/ondecode` on macOS). A sandboxed app should set `ONDE_CODE_DATA_DIR` to its own container
directory before starting the agent, so the app and the `onde-code` CLI never share or overwrite
each other's key.

## Environment

| Variable | Effect |
| --- | --- |
| `ONDE_API_KEY` | Onde Inference `app-id:app-secret`. Selects Onde Inference. |
| `ONDE_BASE_URL` | Inference endpoint. Defaults to `https://cloud.ondeinference.com/v1`. |
| `ONDE_WEB_URL` | Where the browser sign-in goes. Defaults to `https://ondeinference.com`. |
| `ONDE_CODE_PROVIDER` | `onde`, or `openai` for any OpenAI API compatible endpoint. |
| `ONDE_CODE_MODEL` | Model id. Must support tool calling. |
| `ONDE_CODE_DATA_DIR` | Where the key and sessions are stored. |

The full table is in [docs/providers.md](https://github.com/ondeinference/onde-code/blob/main/docs/providers.md).

## Get a key

Onde Code is free and Apache-2.0. Create an Onde Inference app and get its key at
[ondeinference.com](https://ondeinference.com).

## License

Apache-2.0. Copyright © 2026 Onde Inference
