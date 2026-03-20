# Project Planning: mpwall

> A professional hybrid CLI/TUI live video wallpaper manager for Hyprland/Wayland.
> Version: 1.0.0 — MVP

## Status Legend
- `[ ]` Not started
- `[~]` In progress
- `[x]` Completed
- `[!]` Blocked

---

## Phase 1 — Initialization

- [ ] TASK-001: Initialize Rust project with `cargo init`, set up `Cargo.toml` with all dependencies
- [ ] TASK-002: Create full project folder structure (`src/cli/`, `src/core/`, `src/tui/`, `src/tui/panels/`)
- [ ] TASK-003: Add `.gitignore` (Rust-standard + editor files)
- [ ] TASK-004: Create placeholder `mod.rs` files for all modules to ensure project compiles from day one
- [ ] TASK-005: Add `PKGBUILD` stub in repo root

---

## Phase 2 — Foundation (Core Layer)

- [ ] TASK-006: Implement `core/config.rs` — TOML config read/write with hardcoded defaults, schema version field
- [ ] TASK-007: Implement `core/state.rs` — JSON state read/write for active wallpaper per monitor (path, PID, autostart)
- [ ] TASK-008: Implement `core/monitor.rs` — enumerate monitors via `hyprctl monitors -j`, parse JSON output
- [ ] TASK-009: Implement `core/process.rs` — spawn mpvpaper process, kill process, verify PID liveness
- [ ] TASK-010: Write unit tests for `core/state.rs` (mock file I/O)
- [ ] TASK-011: Write unit tests for `core/monitor.rs` (mock monitor data for single/multi-monitor)

---

## Phase 3 — CLI Commands

- [ ] TASK-012: Set up `clap` command definitions in `cli/mod.rs` — root command, subcommands: `set`, `stop`, `enable`, `disable`, `status`, `list`
- [ ] TASK-013: Implement `set` command — kill existing mpvpaper for target monitor, spawn new mpvpaper, write state.json
- [ ] TASK-014: Implement `stop` command — kill mpvpaper process(es), update state.json
- [ ] TASK-015: Implement `enable` command — write autostart entry to `hyprland.conf` inside delimited block (`# mpwall start` / `# mpwall end`)
- [ ] TASK-016: Implement `disable` command — stop wallpaper + remove delimited block from `hyprland.conf`
- [ ] TASK-017: Implement `status` command — read state.json, verify PID liveness, print formatted colored output
- [ ] TASK-018: Implement `list` command — scan wallpaper directory, filter by video extensions, print formatted list
- [ ] TASK-019: Implement `--monitor` flag support across `set`, `stop` commands
- [ ] TASK-020: Validate all CLI error paths — friendly, actionable error messages for every known failure mode

---

## Phase 4 — TUI Layer

- [ ] TASK-021: Implement TUI entry point `tui/mod.rs` — terminal setup, raw mode, event loop, crossterm backend for ratatui
- [ ] TASK-022: Implement `tui/app.rs` — application state struct (active panel, selected file, monitor list, library, settings)
- [ ] TASK-023: Implement `tui/ui.rs` — main layout rendering: split panels, tab bar, status bar, help footer
- [ ] TASK-024: Implement `tui/panels/browser.rs` — scrollable file list from wallpaper directory, highlight active wallpaper, `/` filter mode, `Enter` to set
- [ ] TASK-025: Implement `tui/panels/status.rs` — per-monitor status display, PID health, autostart indicator, 2-second poll interval
- [ ] TASK-026: Implement `tui/panels/library.rs` — saved wallpapers list, add/remove/reorder, play playlist action, backed by `library.json`
- [ ] TASK-027: Implement `tui/panels/settings.rs` — display and edit config values (wallpaper dir, mpvpaper flags), validate on save, write to `config.toml`
- [ ] TASK-028: Implement global TUI keybindings — `q` quit, `Tab` switch panel, `?` help overlay, arrow keys navigation
- [ ] TASK-029: Wire TUI panels to Core Layer — all actions delegate to `core/` functions, TUI never manages processes directly

---

## Phase 5 — Validation

- [ ] TASK-030: End-to-end test — `mpwall set <file>`: verify process spawns, state.json written correctly
- [ ] TASK-031: End-to-end test — `mpwall stop`: verify process killed, state.json updated
- [ ] TASK-032: End-to-end test — `mpwall enable` / `disable`: verify hyprland.conf written/cleaned without corruption
- [ ] TASK-033: Stale PID test — simulate externally killed mpvpaper, verify graceful handling in all commands
- [ ] TASK-034: Multi-monitor test — verify per-monitor isolation using mock monitor data
- [ ] TASK-035: Config absence test — verify full functionality with no `config.toml` present
- [ ] TASK-036: Final binary size and `--release` build verification
- [ ] TASK-037: Update and finalize `PKGBUILD` — correct version, checksums, `depends`, `makedepends`

---

## Phase 6 — Documentation

- [ ] TASK-038: Write `documentation/README.md` — project overview, install instructions, quick start
- [ ] TASK-039: Write `documentation/getting-started.md` — prerequisites, first run walkthrough
- [ ] TASK-040: Write `documentation/architecture.md` — layer diagram, module responsibilities, data flow
- [ ] TASK-041: Write `documentation/configuration.md` — all config.toml fields, defaults, examples
- [ ] TASK-042: Write `documentation/development-workflow.md` — build, test, release process
- [ ] TASK-043: Write `documentation/contributing.md` — contribution guidelines, code style, PR process
- [ ] TASK-044: Write `documentation/features/` — one file per MVP feature (set, stop, enable, disable, status, list, tui, multi-monitor)
- [ ] TASK-045: Update root `README.md` with final content (badges, install, usage, screenshot placeholder)
