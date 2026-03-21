# mpwall

> A professional hybrid CLI/TUI live video wallpaper manager for Hyprland/Wayland.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)
[![AUR](https://img.shields.io/badge/AUR-mpwall-1793d1?logo=arch-linux)](https://aur.archlinux.org/packages/mpwall)
[![Version](https://img.shields.io/badge/version-1.0.4-green.svg)](https://github.com/Lamess-UI/mpwall/releases/tag/v1.0.4)

## What is mpwall?

mpwall is a single pre-compiled binary that replaces all manual `mpvpaper` scripting on Hyprland.
It provides:

- A clean CLI for instant wallpaper management
- A full keyboard-driven TUI for browsing, library management, and configuration
- Multi-monitor support with per-monitor state tracking
- Hyprland autostart integration with safe, delimited config edits
- Wallpaper process survives terminal close — fully detached via `setsid`
- AUR-native binary distribution — no Rust or cargo required

## Quick Install

    # Via yay
    yay -S mpwall

    # Via paru
    paru -S mpwall

No build dependencies. Installs in seconds.

## Quick Start

    mpwall set ~/Videos/wallpapers/city.mp4
    mpwall status
    mpwall enable
    mpwall

## Documentation Index

| File | Description |
|------|-------------|
| getting-started.md | Prerequisites and first run |
| architecture.md | System design and module overview |
| configuration.md | All config options with defaults |
| development-workflow.md | Build, test, release process |
| contributing.md | How to contribute |
