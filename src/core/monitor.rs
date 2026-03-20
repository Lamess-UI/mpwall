use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::process::Command;

/// Represents a single monitor as returned by `hyprctl monitors -j`
#[derive(Debug, Clone, Deserialize)]
pub struct Monitor {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub focused: bool,
}

/// Query all active monitors from Hyprland via hyprctl.
/// Returns a descriptive error if hyprctl is not found or Hyprland is not running.
pub fn list_monitors() -> Result<Vec<Monitor>> {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
        .with_context(|| {
            "hyprctl not found. Is Hyprland running?\nTip: mpwall requires Hyprland to be active."
                .to_string()
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "hyprctl returned an error:\n{}\nTip: ensure Hyprland is running.",
            stderr
        );
    }

    let json = String::from_utf8_lossy(&output.stdout);
    let monitors: Vec<Monitor> = serde_json::from_str(&json).with_context(|| {
        "Failed to parse hyprctl output. The Hyprland version may be unsupported.".to_string()
    })?;

    if monitors.is_empty() {
        bail!("No monitors detected by hyprctl. Is your display connected and Hyprland active?");
    }

    Ok(monitors)
}

/// Get the primary (focused) monitor, or the first one if none is focused.
pub fn primary_monitor() -> Result<Monitor> {
    let monitors = list_monitors()?;
    let primary = monitors
        .iter()
        .find(|m| m.focused)
        .or_else(|| monitors.first())
        .cloned();
    primary.ok_or_else(|| anyhow::anyhow!("No monitors found"))
}

/// Validate that a given monitor name exists in the current Hyprland session.
pub fn validate_monitor(name: &str) -> Result<()> {
    let monitors = list_monitors()?;
    if monitors.iter().any(|m| m.name == name) {
        Ok(())
    } else {
        let available: Vec<&str> = monitors.iter().map(|m| m.name.as_str()).collect();
        bail!(
            "Monitor '{}' not found.\nAvailable monitors: {}\nTip: use `mpwall status` to see active monitors.",
            name,
            available.join(", ")
        )
    }
}

/// Return monitor names to target given an optional --monitor flag.
/// If monitor is None, returns all active monitor names.
/// If monitor is Some("all"), returns all active monitor names.
/// If monitor is Some(name), validates and returns [name].
pub fn resolve_monitors(monitor: Option<&str>) -> Result<Vec<String>> {
    match monitor {
        None | Some("all") => {
            let monitors = list_monitors()?;
            Ok(monitors.into_iter().map(|m| m.name).collect())
        }
        Some(name) => {
            validate_monitor(name)?;
            Ok(vec![name.to_string()])
        }
    }
}

#[cfg(test)]
mod tests {
    /// These tests use mock data since hyprctl won't be available in CI.
    use super::*;

    fn mock_monitors() -> Vec<Monitor> {
        vec![
            Monitor {
                id: 0,
                name: "eDP-1".to_string(),
                description: "Built-in display".to_string(),
                width: 1920,
                height: 1080,
                focused: true,
            },
            Monitor {
                id: 1,
                name: "DP-1".to_string(),
                description: "External display".to_string(),
                width: 2560,
                height: 1440,
                focused: false,
            },
        ]
    }

    #[test]
    fn primary_is_focused_monitor() {
        let monitors = mock_monitors();
        let primary = monitors.iter().find(|m| m.focused).unwrap();
        assert_eq!(primary.name, "eDP-1");
    }

    #[test]
    fn mock_monitor_names() {
        let monitors = mock_monitors();
        let names: Vec<&str> = monitors.iter().map(|m| m.name.as_str()).collect();
        assert!(names.contains(&"eDP-1"));
        assert!(names.contains(&"DP-1"));
    }
}
