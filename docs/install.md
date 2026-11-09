# Install

## Release archives

Prebuilt binaries for macOS (arm64, x64), Linux (arm64, x64) and Windows (x64) are attached to
each [GitHub release](https://github.com/ondeinference/onde-code/releases), along with a
`checksums.txt`:

```sh
curl -LO https://github.com/ondeinference/onde-code/releases/latest/download/onde-code-darwin-arm64.tar.gz
curl -LO https://github.com/ondeinference/onde-code/releases/latest/download/checksums.txt
shasum -a 256 -c checksums.txt --ignore-missing
tar -xzf onde-code-darwin-arm64.tar.gz && mv onde-code ~/.local/bin/
```

The archive names are `onde-code-<os>-<arch>.tar.gz` (`.zip` on Windows), with `darwin`, `linux`
or `windows` for the OS and `arm64` or `x64` for the architecture.

## npm and npx

```sh
npm install -g @ondeinference/onde-code
```

npm installs the right binary for your platform through one of five platform packages
(`@ondeinference/onde-code-darwin-arm64` and so on). Editors can run the agent without a global
install:

```sh
npx @ondeinference/onde-code --acp
```

## NuGet (.NET tool)

```sh
dotnet tool install --global Onde.Code
```

The package bundles the binaries for macOS, Linux and Windows x64, plus macOS and Linux arm64,
so it is larger than the npm packages. There is no Windows arm64 build, and the launcher says so
when it runs there.

## PyPI

```sh
pip install onde-code
```

`uvx onde-code` runs it without a permanent install. The wheels contain only the native binary,
which lands on your `PATH`; there is no Python module. They cover macOS and Linux on arm64 and
x64 (glibc 2.28 or newer) and Windows x64. Elsewhere pip falls back to the source distribution,
which needs a Rust toolchain.

## winget

```sh
winget install OndeInference.OndeCode
```

## From source

Needs Rust 1.88 or newer.

```sh
cargo build --release   # -> target/release/onde-code
```

## ACP registry

The [ACP registry](https://agentclientprotocol.com/get-started/registry) lists agents that editors
can install for you. Onde Code's entry lives in `packaging/registry/` and offers both the binary
archives and the npx package. Once it is merged, editors that read the registry can install
onde-code without the steps above. See [packaging/registry/README.md](../packaging/registry/README.md)
for how the entry is kept current.

## First run

```sh
onde-code --version
onde-code --login
```

Sign-in is covered in [Editor setup](editors.md) and [Providers](providers.md).
