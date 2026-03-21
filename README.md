<div align="center">

<img src="public/images/mpwall Logo Transparent.png" alt="mpwall logo" width="180"/>

# mpwall

**A terminal-native animated wallpaper manager for Hyprland.**  
Built under the [Lamess UI](https://github.com/insadamt) design system.

<img src="public/images/Lamess Symbol Logo Transparent.png" alt="Lamess UI" width="70"/>

---

</div>

## Overview

`mpwall` is a lightweight CLI + TUI tool for setting and managing video wallpapers on Hyprland using `mpvpaper`. It features a full terminal UI with a file browser, wallpaper library, status monitor, and settings panel — all themed with the Lamess UI design language.

## Features

- **Browser panel** — browse your wallpaper directory, filter by name, set wallpaper with Enter
- **Library panel** — save and re-apply favourite wallpapers
- **Status panel** — monitor active wallpaper process per display
- **Settings panel** — configure directory, volume, speed, loop, autostart, and UI theme
- **3 themes** — Lamess UI (default), Cyan, Monochrome
- **Autostart** — writes `exec-once` to `hyprland.conf` automatically

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [mpvpaper](https://github.com/GhostNaN/mpvpaper)
- Hyprland

### Build from source

```bash
git clone https://github.com/insadamt/mpwall
cd mpwall
cargo build --release
sudo cp target/release/mpwall /usr/local/bin/
```

## Usage

```bash
# Launch the TUI
mpwall

# Set a wallpaper directly
mpwall set ~/Videos/wallpapers/video.mp4

# Set on a specific monitor
mpwall set ~/Videos/wallpapers/video.mp4 --monitor eDP-1

# Stop wallpaper
mpwall stop

# Enable autostart (writes to hyprland.conf)
mpwall enable

# Disable autostart
mpwall disable

# Show status
mpwall status

# List wallpapers in configured directory
mpwall list
```

## TUI Keybindings

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Switch panel |
| `↑` / `↓` or `j` / `k` | Navigate list |
| `Enter` | Set wallpaper / confirm / toggle |
| `/` | Filter files (Browser) |
| `Esc` | Cancel filter / close help |
| `a` | Add to Library |
| `d` | Remove from Library |
| `s` | Save settings |
| `?` | Toggle help overlay |
| `q` | Quit |

## Configuration

Config is stored at `~/.config/mpwall/config.toml`:

```toml
schema_version = 1
wallpaper_dir = "/home/user/Videos/wallpapers"
mpvpaper_flags = ""
loop_video = true
volume = 0
speed = 1.0
theme = "lamess_ui"
```

## Themes

| Theme | Description |
|-------|-------------|
| `lamess_ui` | Signal Yellow on pure black — Lamess UI brand (default) |
| `cyan` | Classic cyan terminal |
| `monochrome` | Pure white/gray, no color |

Cycle themes in the Settings panel → **Theme** field → `Enter`.

---

<div align="center">
<sub>Part of the Lamess UI ecosystem — precise, dark, data-driven.</sub>
</div>
