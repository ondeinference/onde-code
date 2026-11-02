# What the agent does

Onde Code implements ACP v1, including the optional parts editors use: session
list/resume/close/delete, multi-root workspaces, MCP servers, config options and slash commands.
The [conformance audit](https://github.com/kelvin-soen/OndeCode/issues/25) lists every spec page
and where each requirement is handled.

| Area | Behavior |
|------|----------|
| Tools | `read_file`, `write_file`, `edit_file`, `list_directory` and `run_command`. Every tool takes an optional `root`, so the model can work across all roots of a multi-root workspace. Each call is reported with its name, kind, absolute locations, diff, raw input and output, and status. |
| Editor routing | File reads and writes go through the client's `fs/*` methods when it advertises them, so the agent sees unsaved buffers. Commands run in the client's `terminal/*` when available, otherwise in a local `sh -c`. On Windows, `sh` has to be on `PATH`, for example from Git for Windows. Commands time out after `ONDE_CODE_COMMAND_TIMEOUT_SECS`. |
| Permissions | Writes and commands ask through `session/request_permission` with allow once, always allow, reject once and always reject. `--yolo`, `ONDE_CODE_YOLO=1` or the auto-approve toggle skip the prompt. |
| MCP | Servers passed in `session/new`, `session/load` or `session/resume`, over stdio or streamable HTTP, are connected per session. Their tools reach the model as `mcp__<server>__<tool>`. A server that fails to start is logged and skipped, and the session still opens. |
| Prompts | Text, images (sent to the model as vision input), embedded resources and `resource_link`s. `file://` links are read, through the client's `fs/read_text_file` when advertised, and inlined. A link to a selection (`#L10:20`, as Zed sends) inlines just those lines. |
| Streaming | Answer text and reasoning (`reasoning_content` and `reasoning` deltas) stream as message and thought chunks with stable message ids, plus a `usage_update` after each model call. |
| Cancellation | `session/cancel` stops the turn with `cancelled`, drops any open permission prompt, and cancels in-flight `fs/*` and `terminal/*` requests to the client. |

## Limits

- A turn ends after 50 model calls with `max_turn_requests`.
- The agent doesn't send `plan` updates. Chat completion models don't produce plans reliably enough
  to report as structured steps.
- The model has to support tool (function) calling.
- Logs go to stderr only. In `--acp` mode stdout is the protocol.
