use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const SCHEMA_VERSION: u32 = 1;

/// Returns the path to the config file: ~/.config/mpwall/config.toml
pub fn config_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
            PathBuf::from(home).join(".config")
        });
    base.join("mpwall").join("config.toml")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Schema version for future migration support
    pub schema_version: u32,

    /// Directory where wallpaper videos are stored
    pub wallpaper_dir: String,

    /// Default mpvpaper flags passed on every spawn
    pub mpvpaper_flags: String,

    /// Whether to loop the video (passed to mpvpaper)
    pub loop_video: bool,

    /// Audio volume (0-100). 0 means muted.
    pub volume: u8,

    /// Playback speed multiplier
    pub speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        Self {
            schema_version: SCHEMA_VERSION,
            wallpaper_dir: format!("{}/Videos/wallpapers", home),
            mpvpaper_flags: "--loop".to_string(),
            loop_video: true,
            volume: 0,
            speed: 1.0,
        }
    }
}

impl Config {
    /// Load config from disk. If the file does not exist, return defaults silently.
    /// If the file exists but is malformed, return a descriptive error.
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {}", path.display()))?;
        let config: Self = toml::from_str(&raw).with_context(|| {
            format!(
                "Failed to parse config file at {}. \nTip: delete it and mpwall will recreate it with defaults.",
                path.display()
            )
        })?;
        Ok(config)
    }

    /// Save config to disk. Creates parent directories if they don't exist.
    pub fn save(&self) -> Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory at {}", parent.display())
            })?;
        }
        let raw = toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML")?;
        fs::write(&path, raw)
            .with_context(|| format!("Failed to write config file at {}", path.display()))?;
        Ok(())
    }

    /// Build the full list of mpvpaper CLI flags from config values.
    /// These are appended to the mpvpaper command on spawn.
    pub fn build_mpvpaper_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        if self.loop_video {
            flags.push("--loop".to_string());
        }
        if self.volume == 0 {
            flags.push("--no-audio".to_string());
        } else {
            flags.push(format!("--volume={}", self.volume));
        }
        if (self.speed - 1.0).abs() > f32::EPSILON {
            flags.push(format!("--speed={}", self.speed));
        }
        // Append any raw extra flags from config
        if !self.mpvpaper_flags.is_empty() {
            // Avoid duplicating --loop if already added
            let extra: Vec<&str> = self.mpvpaper_flags.split_whitespace()
                .filter(|f| {
                    let s = f.to_string();
                    !flags.contains(&s)
                })
                .collect();
            flags.extend(extra.iter().map(|s| s.to_string()));
        }
        flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_correct_schema_version() {
        let cfg = Config::default();
        assert_eq!(cfg.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn default_config_volume_zero_produces_no_audio_flag() {
        let cfg = Config::default();
        let flags = cfg.build_mpvpaper_flags();
        assert!(flags.contains(&"--no-audio".to_string()));
    }

    #[test]
    fn nonzero_volume_produces_volume_flag() {
        let mut cfg = Config::default();
        cfg.volume = 50;
        cfg.mpvpaper_flags = String::new();
        let flags = cfg.build_mpvpaper_flags();
        assert!(flags.contains(&"--volume=50".to_string()));
    }
}
