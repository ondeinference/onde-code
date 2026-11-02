<p align="center">
  <img src="https://raw.githubusercontent.com/ondeinference/onde/refs/heads/development/assets/onde-inference-logo.svg" alt="Onde Inference" width="96">
</p>

<h1 align="center">Onde Code</h1>

<p align="center">
  Fine-tune, manage and deploy models without leaving your editor.
</p>

<p align="center">
  <a href="https://ondeinference.com"><img src="https://img.shields.io/badge/ondeinference.com-235843?style=flat-square&labelColor=17211D" alt="Website"></a>
  <a href="https://www.npmjs.com/package/@ondeinference/onde-code"><img src="https://img.shields.io/npm/v/@ondeinference/onde-code?style=flat-square&labelColor=17211D&color=235843" alt="npm"></a>
  <a href="https://crates.io/crates/onde-code"><img src="https://img.shields.io/crates/v/onde-code?style=flat-square&labelColor=17211D&color=235843" alt="Crates.io"></a>
  <a href="https://github.com/ondeinference/onde-code/releases"><img src="https://img.shields.io/github/v/release/ondeinference/onde-code?style=flat-square&labelColor=17211D&color=235843" alt="Release"></a>
  <a href="https://github.com/ondeinference/onde-code/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/ondeinference/onde-code/ci.yml?style=flat-square&labelColor=17211D&color=235843&label=CI" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-235843?style=flat-square&labelColor=17211D" alt="License"></a>
</p>

<p align="center">
  <a href="https://github.com/ondeinference/onde-swift">Swift</a> · <a href="https://pub.dev/packages/onde_inference">Flutter</a> · <a href="https://www.npmjs.com/package/@ondeinference/react-native">React Native</a> · <a href="https://crates.io/crates/onde">Rust</a> · <a href="https://ondeinference.com">Website</a>
</p>

---

Onde Code is a coding agent for your model work on [Onde Inference](https://ondeinference.com). It
writes and runs the fine-tuning, model management and deploy code in your repo, and it works in any
[Agent Client Protocol](https://agentclientprotocol.com) (ACP) editor or in the terminal. It is free
and Apache-2.0. Onde Inference is the hosted side: get an app key at
[ondeinference.com](https://ondeinference.com).

It is one static binary written in Rust, and it also accepts any OpenAI API compatible endpoint.

## Install

```sh
npm install -g @ondeinference/onde-code    # pulls in the binary for your platform
npx @ondeinference/onde-code --acp         # or run it without installing, from an editor
```

Prebuilt archives for macOS, Linux and Windows are on the
[releases page](https://github.com/ondeinference/onde-code/releases). Building from source and the
ACP registry are covered in [docs/install.md](docs/install.md).

## Quick start

```sh
onde-code --login        # sign in with your browser and pick an Onde Inference app
cd my-project
onde-code                # terminal UI
```

`--login` opens ondeinference.com, and the app key comes back to the agent without appearing in a
URL. Prefer to paste a key? Run `onde-code --setup`.

## Use it your way

- **In an editor:** Zed, JetBrains, Xcode, Neovim and other ACP editors run `onde-code --acp`.
  See [docs/editors.md](docs/editors.md).
- **In the terminal:** `onde-code` opens a chat UI that asks before every edit and command. See
  [docs/terminal.md](docs/terminal.md).
- **Embedded in your app:** the [`onde-code-agent`](crates/agent) crate serves the agent in
  process, for hosts that can't launch a subprocess.
- **For model operations:** pass [`onde --mcp`](docs/models.md) to the agent as an MCP server, and
  it can list, register and assign models on your Onde Inference account.

## Docs

The [docs index](docs/README.md) covers install, editors, what the agent does, the terminal UI,
model workflows, providers and configuration, and development.

## Contributing

Issues and pull requests are welcome at
[ondeinference/onde-code](https://github.com/ondeinference/onde-code). Run
`cargo fmt --check && cargo clippy --all-targets --locked -- -D warnings && cargo test` before you
open one. [docs/development.md](docs/development.md) has the layout.

## License

Apache License 2.0. See [LICENSE](LICENSE).

Copyright © 2026 Onde Inference
