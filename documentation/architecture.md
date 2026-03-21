# Architecture

## Overview

mpwall is structured as three independent layers:

    +-------------+     +-------------+
    |  CLI Layer  |     |  TUI Layer  |
    |  (clap)     |     |  (ratatui)  |
    +------+------+     +------+------+
           |                   |
           +--------+----------+
                    |
           +--------+--------+
           |   Core Layer    |
           |  config / state |
           |  monitor / proc |
           +-----------------+

The CLI and TUI never share state directly. Both call Core functions and read the result.

## Core Layer (src/core/)

| Module | Responsibility |
|--------|----------------|
| config.rs | Load/save config.toml, export SCHEMA_VERSION, build mpvpaper flag strings |
| state.rs | Load/save state.json and library.json, per-monitor wallpaper tracking |
| monitor.rs | Query Hyprland via hyprctl monitors -j, validate monitor names |
| process.rs | Spawn mpvpaper detached via setsid, kill by PID, check liveness via /proc |

## TUI Layer (src/tui/)

| Module | Responsibility |
|--------|----------------|
| mod.rs | Terminal setup/teardown, main event loop, 2s state poll |
| app.rs | Central App struct, refresh_state(), panel navigation |
| ui.rs | Root layout, tab bar, status bar, help overlay |
| panels/browser.rs | File browser, filter mode, set wallpaper on Enter |
| panels/status.rs | Per-monitor status, enable/disable autostart |
| panels/library.rs | Saved wallpapers, add/remove, set from library |
| panels/settings.rs | Config editor with inline validation |

## Data Flow: mpwall set video.mp4

    main.rs
      -> cli::commands::cmd_set("video.mp4", None)
            -> core::config::Config::load()
            -> core::state::State::load()
            -> core::monitor::resolve_monitors(None)
            -> core::process::kill_pid(old_pid)
            -> core::process::spawn_mpvpaper(...)   <- setsid here
            -> core::state::State::save()

## Process Detachment

mpvpaper is spawned using libc::setsid() via pre_exec. This creates a new session
detaching the child from the terminal's process group. The wallpaper keeps running
after the terminal is closed.

## State File (~/.local/share/mpwall/state.json)

    {
      "monitors": {
        "eDP-1": {
          "wallpaper_path": "/home/user/Videos/wallpapers/city.mp4",
          "pid": 12345,
          "autostart": true
        }
      }
    }

## Config File (~/.config/mpwall/config.toml)

    schema_version = 1
    wallpaper_dir = "/home/user/Videos/wallpapers"
    mpvpaper_flags = ""
    loop_video = true
    volume = 0
    speed = 1.0

If this file does not exist, mpwall works perfectly using hardcoded defaults.
