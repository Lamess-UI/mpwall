//! Integration tests for mpwall core layer.
//! These tests use temporary directories and mock data — no real Hyprland or mpvpaper needed.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn temp_state_file(dir: &TempDir) -> PathBuf {
    dir.path().join("state.json")
}

// ---------------------------------------------------------------------------
// TASK-030 / TASK-033: State read/write and stale PID handling
// ---------------------------------------------------------------------------

#[test]
fn state_roundtrip_serialization() {
    use mpwall::core::state::{MonitorState, State};
    use std::collections::HashMap;

    let mut state = State::default();
    state.set_monitor(
        "eDP-1".to_string(),
        MonitorState {
            wallpaper_path: "/home/user/Videos/city.mp4".to_string(),
            pid: Some(42000),
            autostart: false,
        },
    );

    let serialized = serde_json::to_string(&state).unwrap();
    let deserialized: State = serde_json::from_str(&serialized).unwrap();

    let entry = deserialized.get_monitor("eDP-1").unwrap();
    assert_eq!(entry.pid, Some(42000));
    assert_eq!(entry.wallpaper_path, "/home/user/Videos/city.mp4");
    assert!(!entry.autostart);
}

#[test]
fn stale_pid_detection() {
    use mpwall::core::process::is_pid_alive;
    // PID 1 is always alive (init/systemd), PID u32::MAX should never exist
    assert!(is_pid_alive(1));
    assert!(!is_pid_alive(u32::MAX));
}

#[test]
fn state_clear_monitor_removes_entry() {
    use mpwall::core::state::{MonitorState, State};

    let mut state = State::default();
    state.set_monitor(
        "DP-1".to_string(),
        MonitorState {
            wallpaper_path: "/tmp/test.mp4".to_string(),
            pid: Some(9999),
            autostart: true,
        },
    );
    assert!(state.get_monitor("DP-1").is_some());
    state.clear_monitor("DP-1");
    assert!(state.get_monitor("DP-1").is_none());
}

// ---------------------------------------------------------------------------
// TASK-034: Multi-monitor isolation
// ---------------------------------------------------------------------------

#[test]
fn multi_monitor_state_isolation() {
    use mpwall::core::state::{MonitorState, State};

    let mut state = State::default();
    state.set_monitor(
        "eDP-1".to_string(),
        MonitorState {
            wallpaper_path: "/tmp/a.mp4".to_string(),
            pid: Some(1001),
            autostart: false,
        },
    );
    state.set_monitor(
        "DP-1".to_string(),
        MonitorState {
            wallpaper_path: "/tmp/b.mp4".to_string(),
            pid: Some(1002),
            autostart: true,
        },
    );

    let edp = state.get_monitor("eDP-1").unwrap();
    let dp = state.get_monitor("DP-1").unwrap();

    assert_eq!(edp.wallpaper_path, "/tmp/a.mp4");
    assert_eq!(dp.wallpaper_path, "/tmp/b.mp4");
    assert_ne!(edp.pid, dp.pid);
    assert!(!edp.autostart);
    assert!(dp.autostart);
}

// ---------------------------------------------------------------------------
// TASK-032: hyprland.conf block write/remove
// ---------------------------------------------------------------------------

#[test]
fn autostart_block_not_duplicated() {
    // Simulate the remove_mpwall_block logic inline
    let existing = "exec-once = waybar\n# mpwall start\nexec-once = mpwall set old.mp4 --monitor eDP-1\n# mpwall end\nexec-once = dunst\n";

    // Remove block
    let cleaned = remove_mpwall_block(existing);
    assert!(!cleaned.contains("# mpwall start"));
    assert!(!cleaned.contains("# mpwall end"));
    assert!(!cleaned.contains("old.mp4"));
    assert!(cleaned.contains("waybar"));
    assert!(cleaned.contains("dunst"));

    // Add new block
    let new_block = format!("{}\n# mpwall start\nexec-once = mpwall set new.mp4 --monitor eDP-1\n# mpwall end\n", cleaned);
    // Must contain exactly one start marker
    assert_eq!(new_block.matches("# mpwall start").count(), 1);
    assert_eq!(new_block.matches("# mpwall end").count(), 1);
}

fn remove_mpwall_block(content: &str) -> String {
    let mut result = String::new();
    let mut inside = false;
    for line in content.lines() {
        if line.trim() == "# mpwall start" { inside = true; continue; }
        if line.trim() == "# mpwall end" { inside = false; continue; }
        if !inside {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

// ---------------------------------------------------------------------------
// TASK-035: Config absence — defaults always available
// ---------------------------------------------------------------------------

#[test]
fn config_defaults_without_file() {
    use mpwall::core::config::{Config, SCHEMA_VERSION};
    // Temporarily point XDG_CONFIG_HOME to a nonexistent path
    std::env::set_var("XDG_CONFIG_HOME", "/tmp/mpwall_test_nonexistent_xdg");
    let config = Config::load().expect("Config::load should succeed with no file present");
    assert_eq!(config.schema_version, SCHEMA_VERSION);
    assert_eq!(config.volume, 0);
    assert!(config.loop_video);
    assert!((config.speed - 1.0).abs() < f32::EPSILON);
    std::env::remove_var("XDG_CONFIG_HOME");
}

// ---------------------------------------------------------------------------
// TASK-033: Library deduplication
// ---------------------------------------------------------------------------

#[test]
fn library_no_duplicate_entries() {
    use mpwall::core::state::Library;
    let mut lib = Library::default();
    lib.add("/tmp/wallpaper.mp4".to_string());
    lib.add("/tmp/wallpaper.mp4".to_string());
    lib.add("/tmp/wallpaper.mp4".to_string());
    assert_eq!(lib.entries.len(), 1);
}

#[test]
fn library_remove_entry() {
    use mpwall::core::state::Library;
    let mut lib = Library::default();
    lib.add("/tmp/a.mp4".to_string());
    lib.add("/tmp/b.mp4".to_string());
    lib.remove("/tmp/a.mp4");
    assert_eq!(lib.entries.len(), 1);
    assert_eq!(lib.entries[0], "/tmp/b.mp4");
}

// ---------------------------------------------------------------------------
// TASK-036: Config serialization round-trip
// ---------------------------------------------------------------------------

#[test]
fn config_toml_roundtrip() {
    use mpwall::core::config::Config;
    let original = Config::default();
    let serialized = toml::to_string_pretty(&original).unwrap();
    let deserialized: Config = toml::from_str(&serialized).unwrap();
    assert_eq!(deserialized.schema_version, original.schema_version);
    assert_eq!(deserialized.volume, original.volume);
    assert_eq!(deserialized.loop_video, original.loop_video);
}
