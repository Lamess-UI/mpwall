# Getting Started

## Prerequisites

| Dependency | Purpose | Install |
|------------|---------|---------|
| Hyprland | Wayland compositor | paru -S hyprland |
| mpvpaper | Video wallpaper engine | paru -S mpvpaper |
| Nerd Fonts | TUI icons | paru -S ttf-nerd-fonts-symbols |

## Installation

### Via AUR (recommended)

    yay -S mpwall
    # or
    paru -S mpwall

No Rust or cargo required. A pre-compiled binary is downloaded directly from GitHub releases.

### Manual build from source

    git clone https://github.com/Lamess-UI/mpwall.git
    cd mpwall
    cargo build --release
    sudo install -Dm755 target/release/mpwall /usr/bin/mpwall

## First Run

### 1. Set your first wallpaper

    mpwall set ~/Videos/wallpapers/city.mp4

This immediately:
- Kills any existing wallpaper process
- Spawns mpvpaper fully detached from the terminal (survives terminal close)
- Saves the PID and file path to ~/.local/share/mpwall/state.json

### 2. Check status

    mpwall status

### 3. Enable autostart

    mpwall enable

Writes this block to ~/.config/hypr/hyprland.conf:

    # mpwall start
    exec-once = mpwall set /path/to/city.mp4 --monitor eDP-1
    # mpwall end

mpwall will never touch any lines outside this block.

### 4. Open the TUI

    mpwall

No arguments — launches the interactive TUI. Tab to navigate, q to quit, ? for help.

## Default Paths

| File | Path |
|------|------|
| Config | ~/.config/mpwall/config.toml |
| State | ~/.local/share/mpwall/state.json |
| Library | ~/.local/share/mpwall/library.json |
| Wallpaper dir | ~/Videos/wallpapers |

All paths respect XDG_CONFIG_HOME and XDG_DATA_HOME.
