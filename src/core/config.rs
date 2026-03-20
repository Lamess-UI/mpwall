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
    pub schema_version: u32,
    pub wallpaper_dir: String,
    /// Raw extra flags for mpvpaper itself (not mpv options)
    pub mpvpaper_flags: String,
    pub loop_video: bool,
    pub volume: u8,
    pub speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        Self {
            schema_version: SCHEMA_VERSION,
            wallpaper_dir: format!("{}/Videos/wallpapers", home),
            mpvpaper_flags: String::new(),
            loop_video: true,
            volume: 0,
            speed: 1.0,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {}", path.display()))?;
        let config: Self = toml::from_str(&raw).with_context(|| {
            format!(
                "Failed to parse config file at {}.\nTip: delete it and mpwall will recreate defaults.",
                path.display()
            )
        })?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory at {}", parent.display())
            })?;
        }
        let raw = toml::to_string_pretty(self).context("Failed to serialize config to TOML")?;
        fs::write(&path, raw)
            .with_context(|| format!("Failed to write config file at {}", path.display()))?;
        Ok(())
    }

    /// Build the mpv options list to pass via mpvpaper's -o flag.
    /// These are mpv player options, NOT mpvpaper flags.
    pub fn build_mpvpaper_flags(&self) -> Vec<String> {
        let mut opts: Vec<String> = Vec::new();

        if self.loop_video {
            opts.push("--loop".to_string());
        }

        if self.volume == 0 {
            opts.push("--no-audio".to_string());
        } else {
            opts.push(format!("--volume={}", self.volume));
        }

        if (self.speed - 1.0).abs() > f32::EPSILON {
            opts.push(format!("--speed={}", self.speed));
        }

        opts
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
        let flags = cfg.build_mpvpaper_flags();
        assert!(flags.contains(&"--volume=50".to_string()));
    }

    #[test]
    fn loop_video_produces_loop_flag() {
        let cfg = Config::default();
        let flags = cfg.build_mpvpaper_flags();
        assert!(flags.contains(&"--loop".to_string()));
    }
}
