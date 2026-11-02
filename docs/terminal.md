# Terminal mode

```sh
onde-code                       # asks before every edit and command
onde-code --yolo                # approves everything
onde-code --root ../shared-lib  # add another workspace root (repeatable)
```

`--root` applies to the terminal UI only. In an editor, the roots come from the editor.

## Keys

| Key | What it does |
|-----|--------------|
| Enter | Send the message |
| Esc | Cancel a running turn |
| ↑ / ↓, PgUp / PgDn | Scroll |
| Ctrl-C or `/quit` | Exit |
| `y`, `a`, `n` | At a permission prompt: allow, always allow this tool, reject |

## The TUI is an ACP client

The terminal UI starts `onde-code --acp` as a subprocess (with `ONDE_CODE_SURFACE=tui`) and speaks
the same protocol an editor does. The terminal and the editor therefore exercise the same code
paths, and anything that has to work in both lives on the ACP side.

The TUI itself comes from the [`ed-acp-tui`](https://crates.io/crates/ed-acp-tui) crate.
