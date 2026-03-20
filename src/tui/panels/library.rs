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
    cli::commands::cmd_set,
    tui::app::App,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let active_path = app
        .state
        .monitors
        .values()
        .next()
        .map(|e| e.wallpaper_path.clone())
        .unwrap_or_default();

    let items: Vec<ListItem> = app
        .library
        .entries
        .iter()
        .map(|path| {
            let is_active = *path == active_path;
            let name = std::path::Path::new(path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.clone());
            let exists = std::path::Path::new(path).exists();
            let (indicator, style) = if is_active {
                (" ▶", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            } else if !exists {
                (" !", Style::default().fg(Color::Red))
            } else {
                ("  ", Style::default())
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} {}", indicator, name), style),
            ]))
        })
        .collect();

    let mut list_state = ListState::default();
    let len = app.library.entries.len();
    let selected = app.library_selected.min(len.saturating_sub(1));
    if !app.library.entries.is_empty() {
        list_state.select(Some(selected));
    }

    let title = format!(" Library ({} saved) ", app.library.entries.len());
    let list = List::new(if items.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "  No saved wallpapers. Press 'a' in the Browser panel to add.",
            Style::default().fg(Color::DarkGray),
        )))]
    } else {
        items
    })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::Cyan)),
    )
    .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let hints = Paragraph::new(" Enter: set wallpaper  |  d: remove from library  |  ! = file missing")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(hints, chunks[1]);
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    let len = app.library.entries.len();

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.library_selected > 0 {
                app.library_selected -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if len > 0 && app.library_selected < len - 1 {
                app.library_selected += 1;
            }
        }
        KeyCode::Enter => {
            if let Some(path) = app.library.entries.get(app.library_selected).cloned() {
                match cmd_set(&path, None) {
                    Ok(_) => {
                        app.refresh_state()?;
                        let name = std::path::Path::new(&path)
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or(path.clone());
                        app.set_message(format!("Set wallpaper: {}", name), false);
                    }
                    Err(e) => app.set_message(format!("Error: {}", e), true),
                }
            }
        }
        KeyCode::Char('d') => {
            if let Some(path) = app.library.entries.get(app.library_selected).cloned() {
                app.library.remove(&path);
                if let Err(e) = app.library.save() {
                    app.set_message(format!("Error: {}", e), true);
                } else {
                    if app.library_selected > 0 && app.library_selected >= app.library.entries.len() {
                        app.library_selected -= 1;
                    }
                    app.set_message("Removed from library", false);
                }
            }
        }
        _ => {}
    }
    Ok(())
}
