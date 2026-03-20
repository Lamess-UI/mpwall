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

- [x] TASK-001: Initialize Rust project with `cargo init`, set up `Cargo.toml` with all dependencies
- [x] TASK-002: Create full project folder structure (`src/cli/`, `src/core/`, `src/tui/`, `src/tui/panels/`)
- [x] TASK-003: Add `.gitignore` (Rust-standard + editor files)
- [x] TASK-004: Create placeholder `mod.rs` files for all modules to ensure project compiles from day one
- [x] TASK-005: Add `PKGBUILD` stub in repo root

---

## Phase 2 — Foundation (Core Layer)

- [x] TASK-006: Implement `core/config.rs` — TOML config read/write with hardcoded defaults, schema version field
- [x] TASK-007: Implement `core/state.rs` — JSON state read/write for active wallpaper per monitor (path, PID, autostart)
- [x] TASK-008: Implement `core/monitor.rs` — enumerate monitors via `hyprctl monitors -j`, parse JSON output
- [x] TASK-009: Implement `core/process.rs` — spawn mpvpaper process, kill process, verify PID liveness
- [x] TASK-010: Write unit tests for `core/state.rs` (mock file I/O) — embedded in state.rs
- [x] TASK-011: Write unit tests for `core/monitor.rs` (mock monitor data for single/multi-monitor) — embedded in monitor.rs

---

## Phase 3 — CLI Commands

- [x] TASK-012: Set up `clap` command definitions in `cli/mod.rs` — root command, subcommands: `set`, `stop`, `enable`, `disable`, `status`, `list`
- [x] TASK-013: Implement `set` command — kill existing mpvpaper for target monitor, spawn new mpvpaper, write state.json
- [x] TASK-014: Implement `stop` command — kill mpvpaper process(es), update state.json
- [x] TASK-015: Implement `enable` command — write autostart entry to `hyprland.conf` inside delimited block (`# mpwall start` / `# mpwall end`)
- [x] TASK-016: Implement `disable` command — stop wallpaper + remove delimited block from `hyprland.conf`
- [x] TASK-017: Implement `status` command — read state.json, verify PID liveness, print formatted colored output
- [x] TASK-018: Implement `list` command — scan wallpaper directory, filter by video extensions, print formatted list
- [x] TASK-019: Implement `--monitor` flag support across `set`, `stop` commands
- [x] TASK-020: Validate all CLI error paths — friendly, actionable error messages for every known failure mode

---

## Phase 4 — TUI Layer

- [x] TASK-021: Implement TUI entry point `tui/mod.rs` — terminal setup, raw mode, event loop, crossterm backend for ratatui
- [x] TASK-022: Implement `tui/app.rs` — application state struct (active panel, selected file, monitor list, library, settings)
- [x] TASK-023: Implement `tui/ui.rs` — main layout rendering: split panels, tab bar, status bar, help footer
- [x] TASK-024: Implement `tui/panels/browser.rs` — scrollable file list from wallpaper directory, highlight active wallpaper, `/` filter mode, `Enter` to set
- [x] TASK-025: Implement `tui/panels/status.rs` — per-monitor status display, PID health, autostart indicator, 2-second poll interval
- [x] TASK-026: Implement `tui/panels/library.rs` — saved wallpapers list, add/remove/reorder, play playlist action, backed by `library.json`
- [x] TASK-027: Implement `tui/panels/settings.rs` — display and edit config values (wallpaper dir, mpvpaper flags), validate on save, write to `config.toml`
- [x] TASK-028: Implement global TUI keybindings — `q` quit, `Tab` switch panel, `?` help overlay, arrow keys navigation
- [x] TASK-029: Wire TUI panels to Core Layer — all actions delegate to `core/` functions, TUI never manages processes directly

---

## Phase 5 — Validation

- [x] TASK-030: State serialization test — roundtrip JSON read/write verified
- [x] TASK-031: State monitor isolation test — multi-monitor entries are fully independent
- [x] TASK-032: hyprland.conf block test — block written/removed without duplication or corruption
- [x] TASK-033: Stale PID test — `is_pid_alive` verified against PID 1 and u32::MAX
- [x] TASK-034: Multi-monitor isolation test — per-monitor state fully independent
- [x] TASK-035: Config absence test — `Config::load()` returns defaults when no file present
- [x] TASK-036: Config TOML roundtrip test — serialize/deserialize produces identical values
- [x] TASK-037: PKGBUILD finalized — correct version, depends=(mpvpaper gawk), makedepends=(cargo), release build

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
