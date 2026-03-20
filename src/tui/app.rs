use anyhow::Result;
use std::path::PathBuf;

use crate::core::{
    config::Config,
    monitor::{list_monitors, Monitor},
    process::is_pid_alive,
    state::{Library, State},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ActivePanel {
    Browser,
    Status,
    Library,
    Settings,
}

impl ActivePanel {
    pub fn all() -> Vec<ActivePanel> {
        vec![
            ActivePanel::Browser,
            ActivePanel::Status,
            ActivePanel::Library,
            ActivePanel::Settings,
        ]
    }

    pub fn index(&self) -> usize {
        match self {
            ActivePanel::Browser => 0,
            ActivePanel::Status => 1,
            ActivePanel::Library => 2,
            ActivePanel::Settings => 3,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ActivePanel::Browser => " Browser ",
            ActivePanel::Status => " Status ",
            ActivePanel::Library => " Library ",
            ActivePanel::Settings => " Settings ",
        }
    }
}

/// Video file entry in the browser
#[derive(Debug, Clone)]
pub struct VideoEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
}

/// Editable settings fields
#[derive(Debug, Clone)]
pub struct SettingsEdit {
    pub wallpaper_dir: String,
    pub mpvpaper_flags: String,
    pub volume: String,
    pub speed: String,
    pub active_field: usize,
    pub editing: bool,
}

impl SettingsEdit {
    pub fn from_config(config: &Config) -> Self {
        Self {
            wallpaper_dir: config.wallpaper_dir.clone(),
            mpvpaper_flags: config.mpvpaper_flags.clone(),
            volume: config.volume.to_string(),
            speed: config.speed.to_string(),
            active_field: 0,
            editing: false,
        }
    }
}

/// Central application state for the TUI
pub struct App {
    pub active_panel: ActivePanel,
    pub show_help: bool,

    // Browser panel
    pub browser_files: Vec<VideoEntry>,
    pub browser_selected: usize,
    pub browser_filter: String,
    pub browser_filter_mode: bool,

    // Status panel
    pub state: State,
    pub monitors: Vec<Monitor>,

    // Library panel
    pub library: Library,
    pub library_selected: usize,

    // Settings panel
    pub config: Config,
    pub settings_edit: SettingsEdit,

    // Feedback message (shown in status bar)
    pub message: Option<String>,
    pub message_is_error: bool,
}

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "webm", "mov", "avi"];

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let state = State::load()?;
        let library = Library::load()?;
        let monitors = list_monitors().unwrap_or_default();
        let settings_edit = SettingsEdit::from_config(&config);
        let browser_files = Self::scan_files(&config.wallpaper_dir);

        Ok(Self {
            active_panel: ActivePanel::Browser,
            show_help: false,
            browser_files,
            browser_selected: 0,
            browser_filter: String::new(),
            browser_filter_mode: false,
            state,
            monitors,
            library,
            library_selected: 0,
            config,
            settings_edit,
            message: None,
            message_is_error: false,
        })
    }

    /// Refresh state from disk and verify PID liveness
    pub fn refresh_state(&mut self) -> Result<()> {
        self.state = State::load()?;
        // Mark stale PIDs
        for entry in self.state.monitors.values_mut() {
            if let Some(pid) = entry.pid {
                if !is_pid_alive(pid) {
                    entry.pid = None;
                }
            }
        }
        self.monitors = list_monitors().unwrap_or_default();
        Ok(())
    }

    pub fn next_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::Browser => ActivePanel::Status,
            ActivePanel::Status => ActivePanel::Library,
            ActivePanel::Library => ActivePanel::Settings,
            ActivePanel::Settings => ActivePanel::Browser,
        };
    }

    pub fn prev_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::Browser => ActivePanel::Settings,
            ActivePanel::Status => ActivePanel::Browser,
            ActivePanel::Library => ActivePanel::Status,
            ActivePanel::Settings => ActivePanel::Library,
        };
    }

    pub fn set_message(&mut self, msg: impl Into<String>, is_error: bool) {
        self.message = Some(msg.into());
        self.message_is_error = is_error;
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Scan a directory for video files
    pub fn scan_files(dir: &str) -> Vec<VideoEntry> {
        let path = std::path::Path::new(dir);
        if !path.exists() {
            return vec![];
        }
        let mut entries: Vec<VideoEntry> = std::fs::read_dir(path)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let p = e.path();
                        if p.is_file() {
                            let ext = p.extension()?.to_str()?.to_lowercase();
                            if VIDEO_EXTENSIONS.contains(&ext.as_str()) {
                                let name = p.file_name()?.to_string_lossy().to_string();
                                let size = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                                return Some(VideoEntry { path: p, name, size });
                            }
                        }
                        None
                    })
                    .collect()
            })
            .unwrap_or_default();
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }

    /// Get the filtered browser list based on current filter string
    pub fn filtered_files(&self) -> Vec<&VideoEntry> {
        if self.browser_filter.is_empty() {
            self.browser_files.iter().collect()
        } else {
            let f = self.browser_filter.to_lowercase();
            self.browser_files
                .iter()
                .filter(|e| e.name.to_lowercase().contains(&f))
                .collect()
        }
    }
}
