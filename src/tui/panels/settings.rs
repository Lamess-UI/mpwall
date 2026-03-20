use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    core::config::Config,
    tui::app::{App, SettingsEdit},
};

const FIELD_LABELS: &[&str] = &[
    "Wallpaper Directory",
    "mpvpaper Flags",
    "Volume (0-100)",
    "Speed (e.g. 1.0)",
];

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(area);

    let se = &app.settings_edit;
    let values = [
        se.wallpaper_dir.as_str(),
        se.mpvpaper_flags.as_str(),
        se.volume.as_str(),
        se.speed.as_str(),
    ];

    let items: Vec<ListItem> = FIELD_LABELS
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let is_selected = i == se.active_field;
            let is_editing = is_selected && se.editing;
            let value_display = if is_editing {
                format!("{}_", values[i])
            } else {
                values[i].to_string()
            };
            let label_style = if is_selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let value_style = if is_editing {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)
            } else if is_selected {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("  {:<24} ", label), label_style),
                Span::styled(value_display, value_style),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(se.active_field));

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Settings ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // Info bar
    let config_path = crate::core::config::config_path();
    let hints = vec![
        Line::from(vec![
            Span::raw(" Config: "),
            Span::styled(
                config_path.to_string_lossy().to_string(),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(" ↑↓: navigate  |  e: edit field  |  Enter/Esc: confirm  |  s: save"),
    ];
    let info = Paragraph::new(hints)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    let se = &mut app.settings_edit;

    if se.editing {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                se.editing = false;
            }
            KeyCode::Backspace => {
                let field = active_field_mut(se);
                field.pop();
            }
            KeyCode::Char(c) => {
                let field = active_field_mut(se);
                field.push(c);
            }
            _ => {}
        }
        return Ok(());
    }

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if se.active_field > 0 {
                se.active_field -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if se.active_field < FIELD_LABELS.len() - 1 {
                se.active_field += 1;
            }
        }
        KeyCode::Char('e') | KeyCode::Enter => {
            se.editing = true;
        }
        KeyCode::Char('s') => {
            match save_settings(app) {
                Ok(_) => app.set_message("Settings saved", false),
                Err(e) => app.set_message(format!("Error: {}", e), true),
            }
        }
        _ => {}
    }
    Ok(())
}

fn active_field_mut(se: &mut SettingsEdit) -> &mut String {
    match se.active_field {
        0 => &mut se.wallpaper_dir,
        1 => &mut se.mpvpaper_flags,
        2 => &mut se.volume,
        3 => &mut se.speed,
        _ => &mut se.wallpaper_dir,
    }
}

fn save_settings(app: &mut App) -> Result<()> {
    let se = &app.settings_edit;

    let volume: u8 = se.volume.parse().map_err(|_| {
        anyhow::anyhow!("Invalid volume: '{}'. Must be a number between 0 and 100.", se.volume)
    })?;
    let speed: f32 = se.speed.parse().map_err(|_| {
        anyhow::anyhow!("Invalid speed: '{}'. Must be a decimal number like 1.0.", se.speed)
    })?;

    if volume > 100 {
        anyhow::bail!("Volume must be between 0 and 100.");
    }
    if speed <= 0.0 {
        anyhow::bail!("Speed must be greater than 0.");
    }

    let new_config = Config {
        schema_version: app.config.schema_version,
        wallpaper_dir: se.wallpaper_dir.clone(),
        mpvpaper_flags: se.mpvpaper_flags.clone(),
        loop_video: app.config.loop_video,
        volume,
        speed,
    };

    new_config.save()?;
    app.config = new_config;
    // Refresh browser with new directory
    app.browser_files = App::scan_files(&app.config.wallpaper_dir);
    app.browser_selected = 0;
    Ok(())
}
