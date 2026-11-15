# Onde Code for Python

`onde-code` is the command-line interface for [Onde Code](https://ondeinference.com), a coding agent
for fine-tuning, managing and deploying models on Onde Inference. It works in any
[Agent Client Protocol](https://agentclientprotocol.com) (ACP) editor or in the terminal.

This package ships the native binary, so there is no Python code to import. Installing it puts
`onde-code` on your `PATH`.

## Install

```sh
pip install onde-code
```

or without a permanent install:

```sh
uvx onde-code
```

## Update

```sh
pip install --upgrade onde-code
```

## Run

```sh
onde-code
```

In a terminal that opens the chat UI. When an editor starts it with its streams piped, the same
binary speaks ACP over stdio instead. Get an app key at [ondeinference.com](https://ondeinference.com).

## Platform support

Wheels are published for:

- macOS `arm64`, `x64`
- Linux `arm64`, `x64` (glibc 2.28 or newer)
- Windows `x64`

There is no Windows `arm64` build yet. On a platform without a wheel, pip falls back to the source
distribution, which needs a Rust toolchain to build.

## Other installation methods

- npm: `npm install -g @ondeinference/onde-code`
- .NET: `dotnet tool install --global Onde.Code`
- winget: `winget install OndeInference.OndeCode`
- GitHub Releases: <https://github.com/ondeinference/onde-code/releases>

## Source

- Repository: <https://github.com/ondeinference/onde-code>
- Website: <https://ondeinference.com>
- Issues: <https://github.com/ondeinference/onde-code/issues>

## License

[Apache 2.0](https://github.com/ondeinference/onde-code/blob/main/LICENSE)

## Copyright

© 2026 [Splitfire AB](https://5mb.app) ([Onde Inference](https://ondeinference.com)).
