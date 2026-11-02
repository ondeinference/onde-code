# ACP registry entry

`agent.json` is onde-code's entry in the [ACP registry](https://agentclientprotocol.com/get-started/registry)
([agentclientprotocol/registry](https://github.com/agentclientprotocol/registry)). In the registry it
lives at `onde-code/agent.json`, with a copy of `assets/icon.svg` next to it as `onde-code/icon.svg`.

The listing is submitted in a fresh PR that supersedes
[agentclientprotocol/registry#652](https://github.com/agentclientprotocol/registry/pull/652). That
one points at the old `kelvin-soen/OndeCode` v1.0.0 release, while the releases and the npm
provenance are on `ondeinference/onde-code`.

It lists two ways to run the agent:

- `binary`: the five release archives, run as `onde-code --acp`.
- `npx`: `@ondeinference/onde-code@<version>` with `--acp`. The launcher package pulls in the
  platform binary.

Once the listing is merged, this folder is not part of the release process. The registry's
`update-versions` workflow runs every hour, finds new GitHub releases of this repo, and commits the
new version to the registry's `main` itself. It swaps the version in each archive URL (`/v1.1.0/`
becomes `/v1.2.0/`), takes the `sha256` values from the digests GitHub records for the release
assets, and bumps the npx package version. That works as long as:

- releases are tagged `v<version>` and are not marked as prereleases,
- the archive names stay version-free (`onde-code-darwin-arm64.tar.gz` and so on), which
  `.github/workflows/release-github.yml` takes care of,
- all five archives are attached to the release,
- the same version is published to npm by `release-npm.yml`.

The automation only touches the version, archive URLs, hashes and npx version. Changing anything
else in the listing (description, authors, icon) still takes a PR to the registry.

What the registry checks, and where this repo covers it:

- `initialize` returns the `onde` auth method ("Sign in with Onde"). The registry's CI starts the
  binary with an empty `HOME`, so that method is what it sees, and `--auth-check` accepts it.
- `session/new` and `session/prompt` return `AUTH_REQUIRED` until a key is configured.
- `.github/workflows/release-github.yml` builds the five registry platforms on a `v*` tag and attaches
  the archives and `checksums.txt` to the release. It refuses a tag that doesn't match the
  version in `Cargo.toml`.
- `assets/icon.svg` is a 16×16 monochrome icon drawn with `currentColor`.

## Submitting a release

1. Tag the release on `main` (the tag must match `version` in `Cargo.toml`) and push it:

   ```sh
   VERSION=1.1.1
   git tag v$VERSION origin/main && git push origin v$VERSION
   ```

2. Wait for the `release` and `Code Release npm` workflows to finish, then point `agent.json` at
   the release:

   ```sh
   packaging/registry/update-agent.sh v$VERSION
   ```

   It downloads the five archives, hashes them, and writes the version, the npx package version,
   the URLs and the `sha256` values. Commit the result here.

3. In your fork of the registry, branch off `upstream/main`, copy the files and validate:

   ```sh
   git checkout -b add-onde-code-v$VERSION upstream/main
   cp ../onde-code/packaging/registry/agent.json onde-code/agent.json
   cp ../onde-code/assets/icon.svg onde-code/icon.svg
   uv run --with jsonschema .github/workflows/build_registry.py --dry-run
   ./.github/workflows/scripts/run-registry-docker.sh python3 .github/workflows/verify_agents.py --auth-check
   ```

   The build script checks that every archive URL answers with HTTP 200, so run it after the
   release is published.

4. Push the branch to your fork and open the PR against `agentclientprotocol/registry`.
