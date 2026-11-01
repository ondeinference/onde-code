#!/usr/bin/env bash
# Bootstrap npm Trusted Publishing for the six onde-code packages.
#
# For each package: publish a 0.0.0 placeholder if it does not exist yet (a Trusted Publisher can
# only be attached to an existing package), then attach the GitHub Actions publisher for
# release-npm.yml. Safe to re-run; packages that already trust the workflow are skipped.
#
# Run locally as an @ondeinference org member, never from CI. npm prompts for an OTP per write;
# export NPM_CONFIG_OTP to pass one up front.
#
#   packaging/npm/scripts/setup-trusted-publishing.sh [--dry-run]
set -euo pipefail

# `npm trust` needs a newer npm than the one bundled with Node, so pin one through npx.
NPM="${NPM:-npx -y npm@11.21.1}"
REPO="ondeinference/onde-code"
WORKFLOW="release-npm.yml"
SCOPE="@ondeinference"
PACKAGES=(
  onde-code
  onde-code-darwin-arm64
  onde-code-darwin-x64
  onde-code-linux-x64
  onde-code-linux-arm64
  onde-code-windows-x64
)

dry_run=()
case "${1:-}" in
  "") ;;
  --dry-run) dry_run=(--dry-run) ;;
  *) echo "usage: $0 [--dry-run]" >&2; exit 2 ;;
esac

# $NPM is deliberately unquoted: it holds a command plus arguments.
# shellcheck disable=SC2086
npm_cmd() { $NPM "$@"; }

if ! npm_cmd whoami >/dev/null 2>&1; then
  echo "Not logged in to npm. Run 'npm login' first." >&2
  exit 1
fi

for name in "${PACKAGES[@]}"; do
  pkg="${SCOPE}/${name}"
  echo "== ${pkg}"

  # Only an E404 means "does not exist"; any other failure could be a registry problem.
  if view_output=$(npm_cmd view "${pkg}" version 2>&1); then
    echo "exists (latest ${view_output})"
  elif grep -q "E404" <<<"${view_output}"; then
    echo "not on npm yet, publishing a 0.0.0 placeholder"
    tmp=$(mktemp -d)
    printf '{\n  "name": "%s",\n  "version": "0.0.0",\n  "description": "Placeholder so a Trusted Publisher can be attached.",\n  "license": "MIT"\n}\n' "${pkg}" > "${tmp}/package.json"
    (cd "${tmp}" && npm_cmd publish --access public --tag placeholder ${dry_run[@]+"${dry_run[@]}"})
    rm -rf "${tmp}"
  else
    echo "npm view failed for ${pkg}:" >&2
    echo "${view_output}" >&2
    exit 1
  fi

  trust_output=$(npm_cmd trust list "${pkg}" --json 2>/dev/null || true)
  if grep -q "${REPO}" <<<"${trust_output}" && grep -q "${WORKFLOW}" <<<"${trust_output}"; then
    echo "already trusts ${REPO} ${WORKFLOW}, skipping"
    continue
  fi

  npm_cmd trust github "${pkg}" --repo "${REPO}" --file "${WORKFLOW}" --allow-publish --yes \
    ${dry_run[@]+"${dry_run[@]}"}
done

echo "Done."
