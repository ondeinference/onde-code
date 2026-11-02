# Models

Onde Code is built for model work on Onde Inference: writing and running the code that fine-tunes,
manages and deploys your models. It does that with the same tools it uses for any repo (read and
edit files, run commands) plus the Onde CLI, [`onde`](https://github.com/ondeinference/onde-cli).

## The Onde MCP server

`onde --mcp` runs the Onde CLI as a [Model Context Protocol](https://modelcontextprotocol.io/)
server over stdio. It exposes account and model-catalog operations as tools that return structured
JSON:

| Tool | What it does |
|------|--------------|
| `login`, `me` | Sign in, and show who you are signed in as |
| `apps_list`, `app_create`, `app_rename` | Manage your Onde Inference apps |
| `models_list` | List the models in the catalog |
| `model_register` | Register a fine-tuned GGUF model that is already uploaded to Hugging Face |
| `model_assign` | Assign a model to an app |
| `hf_search` | Search Hugging Face for base models |

The tools run non-interactively and reuse the token from an `onde` sign-in. Install the CLI first,
for example `npm install -g @ondeinference/cli`, and sign in with `onde --tui`.

Any ACP client can hand it to the agent in `mcpServers`, and Onde Code connects it for the session.
In Zed, add it under `context_servers` in `settings.json`, or pass it to the agent through whatever
MCP configuration your editor uses:

```json
{
  "context_servers": {
    "onde": {
      "command": "onde",
      "args": ["--mcp"]
    }
  }
}
```

The tools then appear to the model as `mcp__onde__models_list`, `mcp__onde__model_assign` and so on.

## Fine-tuning

Fine-tuning itself is part of the `onde` CLI, not of the MCP server: a local LoRA pipeline for
Qwen2, Qwen2.5 and Qwen3 models that merges the adapter and exports GGUF. Onde Code can drive it
with `run_command` like any other tool, and ask before it does. See the
[onde-cli README](https://github.com/ondeinference/onde-cli#fine-tuning) for the pipeline.

A typical session, from a repo with your training data:

1. Ask the agent to find a base model (`hf_search`) and set up the fine-tune.
2. It writes the config and runs the pipeline through `onde`, after you approve the commands.
3. Once the GGUF is on Hugging Face, it registers it (`model_register`) and assigns it to your app
   (`model_assign`).

## Built in

Shipping the Onde tools inside the agent, with no MCP setup, is tracked in
[kelvin-soen/OndeCode#27](https://github.com/kelvin-soen/OndeCode/issues/27). Until then, the MCP
server above works today.
