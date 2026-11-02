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
