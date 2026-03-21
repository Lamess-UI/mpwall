# Architecture

## Overview

mpwall is structured as three independent layers that communicate in one direction only:

```
┌────────────┐  ┌────────────┐
│  CLI Layer  │  │  TUI Layer  │
│  (clap)     │  │  (ratatui)  │
└─────┼────┘  └─────┼────┘
           │            │
           │            │  both delegate to ↓
           │            │
      ┌────┴─────────┴────┐
      │       Core Layer        │
      │  config / state /       │
      │  monitor / process      │
      └──────────────────────┘
```


The CLI and TUI **never** share state directly. Both call Core functions and read the result.

## Layer Responsibilities

### CLI Layer (`src/cli/`)

- `mod.rs` — defines the `clap` command tree (subcommands, flags, help text)
- `commands.rs` — implements each command handler by calling Core functions
- Entry: `main.rs` parses args and dispatches, or launches TUI if no subcommand

### Core Layer (`src/core/`)

| Module | Responsibility |
|--------|----------------|
| `config.rs` | Load/save `config.toml`, provide hardcoded defaults, build mpvpaper flag strings |
| `state.rs` | Load/save `state.json` and `library.json`, per-monitor wallpaper tracking |
| `monitor.rs` | Query Hyprland via `hyprctl monitors -j`, validate and resolve monitor names |
| `process.rs` | Spawn mpvpaper, kill by PID, check PID liveness via `/proc` |

### TUI Layer (`src/tui/`)

| Module | Responsibility |
|--------|----------------|
| `mod.rs` | Terminal setup/teardown, main event loop, 2s state poll |
| `app.rs` | Central `App` struct — all TUI state, `refresh_state()`, panel navigation |
| `ui.rs` | Root layout, tab bar, status bar, help overlay |
| `panels/browser.rs` | File browser, filter mode, set wallpaper on Enter |
| `panels/status.rs` | Per-monitor status, enable/disable autostart |
| `panels/library.rs` | Saved wallpapers, add/remove, set from library |
| `panels/settings.rs` | Config editor with inline validation |

## Data Flow: `mpwall set video.mp4`

```
main.rs
  └→ cli::commands::cmd_set("video.mp4", None)
        └→ core::config::Config::load()          → read config.toml (or defaults)
        └→ core::state::State::load()            → read state.json
        └→ core::monitor::resolve_monitors(None) → hyprctl monitors -j
        └→ core::process::kill_pid(old_pid)      → SIGTERM existing process
        └→ core::process::spawn_mpvpaper(...)    → mpvpaper eDP-1 video.mp4 --loop
        └→ core::state::State::save()            → write state.json with new PID
```

## State File Format

`~/.local/share/mpwall/state.json`:

```json
{
  "monitors": {
    "eDP-1": {
      "wallpaper_path": "/home/user/Videos/wallpapers/city.mp4",
      "pid": 12345,
      "autostart": true
    }
  }
}
```

State is treated as a **cache**, not a source of truth. PID liveness is always verified against `/proc/<pid>` before use.

## Config File Format

`~/.config/mpwall/config.toml`:

```toml
schema_version = 1
wallpaper_dir = "/home/user/Videos/wallpapers"
mpvpaper_flags = "--loop"
loop_video = true
volume = 0
speed = 1.0
```

If this file does not exist, mpwall works perfectly using hardcoded defaults.
