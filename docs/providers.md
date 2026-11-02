# Providers and configuration

## Onde Inference

Onde Inference is the default. Set `ONDE_API_KEY` to the `app-id:app-secret` pair from the Onde
dashboard, or store a key with `onde-code --login` (browser) or `onde-code --setup` (paste). The
agent then talks to `https://cloud.ondeinference.com/v1` and uses `onde-kkk` unless a model is set.
`onde-code --list-models` prints the ids your key can use.

## OpenAI API compatible endpoints

Point the agent at any endpoint that supports tool calling with `OPENAI_BASE_URL`,
`OPENAI_API_KEY` and `OPENAI_MODEL`, or pick that option in `--setup`.

Onde Inference wins: whenever `ONDE_API_KEY` is set, the agent uses it with the Onde Inference URL
and ignores `OPENAI_API_KEY` and `OPENAI_BASE_URL`. `ONDE_CODE_MODEL` (or `OPENAI_MODEL`) still
picks the model. To use another endpoint while `ONDE_API_KEY` is set, set
`ONDE_CODE_PROVIDER=openai`. To reach a local or staging Onde Inference deployment, set
`ONDE_BASE_URL`.

```sh
ONDE_API_KEY=APP_ID:APP_SECRET onde-code
OPENAI_BASE_URL=https://your-endpoint/v1 OPENAI_API_KEY=... OPENAI_MODEL=your-model onde-code
onde-code --list-models     # what the configured endpoint serves
```

## Environment variables

| Env var | Default | |
|---------|---------|---|
| `ONDE_CODE_PROVIDER` | detected from keys | `onde`, or `openai` for any OpenAI API compatible endpoint |
| `ONDE_API_KEY` | unset | Onde Inference `app-id:app-secret`; selects Onde Inference |
| `ONDE_WEB_URL` | `https://ondeinference.com` | where `--login` and "Sign in with Onde" send the browser, for a local or staging site |
| `ONDE_BASE_URL` | `https://cloud.ondeinference.com/v1` | Onde Inference base URL, for a local or staging deployment |
| `OPENAI_BASE_URL` | `https://api.openai.com/v1` | base URL of an OpenAI API compatible endpoint |
| `OPENAI_API_KEY` | unset | bearer token for that endpoint |
| `ONDE_CODE_MODEL` | `OPENAI_MODEL` | model id for either provider; must support tool calling |
| `OPENAI_MODEL` | `onde-kkk` (Onde Inference), `gpt-4o-mini` (other endpoints) | model id when `ONDE_CODE_MODEL` is unset |
| `ONDE_CODE_MODELS` | listed from `/models` | comma-separated models for the editor's model picker |
| `ONDE_CODE_YOLO` | unset | `1` or `true` approves every tool call |
| `ONDE_CODE_COMMAND_TIMEOUT_SECS` | `120` | how long `run_command` may run before it is killed |
| `ONDE_CODE_DATA_DIR` | the config dir | where the stored key and `sessions/` live |
| `ONDE_CODE_CONTEXT_WINDOW` | `128000` | context size reported in `usage_update` |
| `ONDE_CODE_SURFACE` | `acp` | set to `tui` by the terminal UI |
| `RUST_LOG` | unset | log filter; logs go to stderr, never stdout |

```
onde-code [--acp] [--yolo] [--list-models] [--root <path>...] [--login] [--setup] [--version]
```
