# Onde Code for .NET

`onde-code` is the command-line interface for [Onde Code](https://ondeinference.com), a coding agent
for fine-tuning, managing and deploying models on Onde Inference. It works in any
[Agent Client Protocol](https://agentclientprotocol.com) (ACP) editor or in the terminal.

## Install

```sh
dotnet tool install --global Onde.Code
```

## Update

```sh
dotnet tool update --global Onde.Code
```

## Run

```sh
onde-code
```

In a terminal that opens the chat UI. When an editor starts it with its streams piped, the same
binary speaks ACP over stdio instead. Get an app key at [ondeinference.com](https://ondeinference.com).

## Platform support

This .NET tool bundles native `onde-code` binaries for:

- macOS `arm64`, `x64`
- Linux `arm64`, `x64` (glibc)
- Windows `x64`

There is no Windows `arm64` build yet, and the launcher says so instead of failing to start.

Because all five binaries ship in one package, the install is larger than the npm packages, which
each download only the binary for your platform.

## Other installation methods

- npm: `npm install -g @ondeinference/onde-code`
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
