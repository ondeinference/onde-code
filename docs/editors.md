# Editor setup

Editors launch the agent with piped stdio, which switches it to ACP mode. `--acp` forces that mode.

## Zed

Add the agent to `settings.json`:

```json
{
  "agent_servers": {
    "Onde Code": {
      "type": "custom",
      "command": "/absolute/path/to/onde-code",
      "args": ["--acp"],
      "env": {
        "ONDE_API_KEY": "APP_ID:APP_SECRET"
      }
    }
  }
}
```

Without a global install, use `"command": "npx"` and `"args": ["-y", "@ondeinference/onde-code", "--acp"]`.
The `env` block is optional if you have already signed in with `onde-code --login`.

## Android Studio and JetBrains IDEs

Android Studio, IntelliJ IDEA and the other JetBrains IDEs run ACP agents from AI Chat (IDE
2025.3.2 or later with JetBrains AI). Once Onde Code is in the ACP registry, install it from
**Settings | Tools | AI Assistant | Agents**, or from **Install From ACP Registry** in the agent picker.

Until then, or for a local build, choose **Add Custom Agent** in the agent picker. That opens
`~/.jetbrains/acp.json`:

```json
{
  "agent_servers": {
    "Onde Code": {
      "command": "/absolute/path/to/onde-code",
      "args": ["--acp"],
      "env": {
        "ONDE_API_KEY": "APP_ID:APP_SECRET"
      }
    }
  }
}
```

The agent shows up in AI Chat right away. If it does not, check the JSON and restart the IDE. ACP
agents do not run in WSL. The `env` block is optional after `onde-code --login`.

## Xcode

Xcode runs ACP agents as custom agents in the coding assistant. This needs a release with custom
agent support (reported as Xcode 26.6, and by some tools as Xcode 27), so
check for **Add an Agent** under Intelligence in your version.

1. Open **Xcode > Settings > Intelligence**.
2. Under **Agents**, click **Add an Agent**.
3. Set **Command** to the absolute path of `onde-code` and **Arguments** to `--acp`.
4. Click **Add**, then pick Onde Code in the coding assistant.

Xcode does not expand `~` or read your shell `PATH`, so use the full path from `which onde-code`.
If you installed with npm, point Command at `npx` and use `-y @ondeinference/onde-code --acp` as the
arguments, again with the absolute path to `npx`. Sign in first with `onde-code --login`, because
Xcode has no place to set `ONDE_API_KEY`.

To let the agent use Xcode's own tools, turn on **Allow external agents to use Xcode tools** under
Model Context Protocol and register `xcrun mcpbridge` as a stdio MCP server in the agent's MCP configuration.

## marimo notebooks

marimo can chat with an ACP agent from the notebook sidebar. This is experimental in marimo, so
features and APIs may change.

marimo reaches agents over a local port, so bridge Onde Code's stdio to a WebSocket first:

```sh
npx stdio-to-ws "onde-code --acp" --port 3025
```

On Windows, put `cmd /c` before the command: `npx stdio-to-ws "cmd /c onde-code --acp" --port 3025`.
Then enable the agents feature flag under **Lab** in marimo's settings, open the agents icon in the
sidebar and select the agent. Keep the bridge running while you chat. Sign in with
`onde-code --login` beforehand or export `ONDE_API_KEY` in the same terminal.

## Signing in from the editor

The agent offers a **Sign in with Onde** method. It opens ondeinference.com in your browser, as a
URL prompt when the editor supports that. You pick an org and an app and press Connect. A key that
already works, from the environment or the stored file, passes without opening anything.

Until a key exists, the agent answers `session/new`, `session/load`, `session/resume` and
`session/prompt` with `AUTH_REQUIRED`. Once you sign in (or `onde-code --setup` succeeds), the
running agent picks up the stored key, so the open thread comes back without restarting anything.

The key is stored in the platform config dir: `~/Library/Application Support/ondecode/env` on
macOS, `~/.config/ondecode/env` on Linux and `%APPDATA%\ondecode\env` on Windows. A variable set in
the environment wins over the same one in that file.

## Logging out

Logging out from the editor deletes the stored key and the saved sign-in (`session.json`).

## What you get in the editor

- A model picker, filled from the endpoint's `GET /models` (or `ONDE_CODE_MODELS`).
- An "Auto-approve actions" toggle, in editors that support boolean config options.
- `/models` and `/setup` slash commands, answered locally without a model call.
- A thread history: sessions can be listed, resumed, loaded with full replay, closed and deleted.
  They are saved to disk, so threads reopen after the editor or the agent restarts. Permission
  choices ("always allow", auto-approve) are not saved.
