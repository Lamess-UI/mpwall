# mpwall 🎬

A simple live video wallpaper manager for Hyprland/Wayland using mpvpaper.

## Dependencies

- [`mpvpaper`](https://github.com/GhostNaN/mpvpaper)
- `hyprland`
- `fish` shell
- `gawk`

## Installation

```fish
# Install dependencies
paru -S mpvpaper fish

# Install mpwall
curl -o ~/.local/bin/mpwall https://raw.githubusercontent.com/insadamt/mpwall/main/mpwall
chmod +x ~/.local/bin/mpwall
fish_add_path ~/.local/bin
```

## Usage

```fish
mpwall set ~/Videos/wallpaper.mp4   # set wallpaper now
mpwall enable                        # save current wallpaper to autostart on boot
mpwall stop                          # stop wallpaper (keep autostart)
mpwall disable                       # stop wallpaper + remove from autostart
```

## License

MIT
