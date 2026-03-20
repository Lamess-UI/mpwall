use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Check whether mpvpaper is installed and reachable in PATH.
pub fn check_mpvpaper_installed() -> Result<()> {
    which_binary("mpvpaper")
}

/// Check whether a binary exists in PATH.
pub fn which_binary(name: &str) -> Result<()> {
    let output = Command::new("which")
        .arg(name)
        .output()
        .with_context(|| format!("Failed to run 'which {}'", name))?;
    if output.status.success() {
        Ok(())
    } else {
        bail!(
            "'{}' not found in PATH.\nTip: install it with: paru -S {}",
            name,
            name
        )
    }
}

/// Spawn mpvpaper for a given monitor and video file.
/// Returns the PID of the spawned process.
///
/// Correct mpvpaper command structure:
///   mpvpaper [mpvpaper-flags] -o "<mpv options>" <monitor> <video_path>
pub fn spawn_mpvpaper(
    monitor: &str,
    video_path: &str,
    mpv_options: &[String],
) -> Result<u32> {
    check_mpvpaper_installed()?;

    if !Path::new(video_path).exists() {
        bail!(
            "Video file not found: {}\nTip: check the path and try again.",
            video_path
        );
    }

    let mut cmd = Command::new("mpvpaper");

    // Pass mpv options via -o flag (required by mpvpaper)
    if !mpv_options.is_empty() {
        let opts = mpv_options.join(" ");
        cmd.arg("-o");
        cmd.arg(opts);
    }

    // Monitor and video path come AFTER -o
    cmd.arg(monitor);
    cmd.arg(video_path);

    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    let child: Child = cmd
        .spawn()
        .with_context(|| format!("Failed to spawn mpvpaper for monitor '{}'.", monitor))?;

    let pid = child.id();
    std::mem::forget(child);

    Ok(pid)
}

/// Kill a process by PID.
/// Returns Ok(()) if the process was killed or was already dead.
pub fn kill_pid(pid: u32) -> Result<()> {
    if !is_pid_alive(pid) {
        return Ok(());
    }
    let status = Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .status()
        .with_context(|| format!("Failed to run kill on PID {}", pid))?;

    if !status.success() {
        let _ = Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .status();
    }
    Ok(())
}

/// Check whether a PID corresponds to a live process via /proc.
pub fn is_pid_alive(pid: u32) -> bool {
    Path::new(&format!("/proc/{}", pid)).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_process_is_alive() {
        let pid = std::process::id();
        assert!(is_pid_alive(pid));
    }

    #[test]
    fn nonexistent_pid_is_not_alive() {
        assert!(!is_pid_alive(0));
    }
}
