use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    cli::commands::cmd_set,
    tui::app::App,
};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let c = app.colors();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let files = app.filtered_files();
    let active_path = app
        .state
        .monitors
        .values()
        .next()
        .map(|e| e.wallpaper_path.clone())
        .unwrap_or_default();

    let total = files.len();

    let items: Vec<ListItem> = files
        .iter()
        .map(|entry| {
            let is_active = entry.path.to_string_lossy() == active_path;
            let size_str = format_size(entry.size);
            let (indicator, name_style) = if is_active {
                (
                    "▶ ",
                    Style::default().fg(c.active_item).add_modifier(Modifier::BOLD),
                )
            } else {
                ("  ", Style::default().fg(c.text_primary))
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {}{}", indicator, entry.name), name_style),
                Span::styled(
                    format!("  {}", size_str),
                    Style::default().fg(c.text_muted),
                ),
            ]))
        })
        .collect();

    let title = if app.browser_filter_mode {
        format!(" Browser  /{}_ ", app.browser_filter)
    } else if !app.browser_filter.is_empty() {
        format!(" Browser  [{}]  {}/{} ", app.browser_filter, total, app.browser_files.len())
    } else {
        format!(" Browser  {} files ", total)
    };

    let selected = app.browser_selected.min(total.saturating_sub(1));
    let mut list_state = ListState::default();
    if !files.is_empty() {
        list_state.select(Some(selected));
    }

    let empty_msg = if app.browser_filter.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "  No video files found in wallpaper directory.",
            Style::default().fg(c.text_muted),
        )))]
    } else {
        vec![ListItem::new(Line::from(Span::styled(
            format!("  No results for '{}'", app.browser_filter),
            Style::default().fg(c.text_muted),
        )))]
    };

    let display_items = if items.is_empty() { empty_msg } else { items };

    let list = List::new(display_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(title, Style::default().fg(c.title).add_modifier(Modifier::BOLD)))
                .border_style(Style::default().fg(c.border_active)),
        )
        .highlight_style(
            Style::default()
                .fg(c.highlight_fg)
                .bg(c.highlight_bg)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let hint_text = if app.browser_filter_mode {
        format!(" FILTER: {}▌  Esc: cancel  Enter: confirm", app.browser_filter)
    } else {
        " /: filter  Enter: set wallpaper  a: add to library".to_string()
    };
    let hint_style = if app.browser_filter_mode {
        Style::default().fg(c.title)
    } else {
        Style::default().fg(c.text_muted)
    };
    let filter_bar = Paragraph::new(hint_text)
        .style(hint_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(c.border_inactive)),
        );
    f.render_widget(filter_bar, chunks[1]);
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.browser_filter_mode {
        match key.code {
            KeyCode::Esc => {
                app.browser_filter_mode = false;
                app.browser_filter.clear();
                app.browser_selected = 0;
            }
            KeyCode::Enter => {
                app.browser_filter_mode = false;
            }
            KeyCode::Backspace => {
                app.browser_filter.pop();
                app.browser_selected = 0;
            }
            KeyCode::Char(c) => {
                app.browser_filter.push(c);
                app.browser_selected = 0;
            }
            _ => {}
        }
        return Ok(());
    }

    let file_count = app.filtered_files().len();

    match key.code {
        KeyCode::Char('/') => {
            app.browser_filter_mode = true;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.browser_selected > 0 {
                app.browser_selected -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if file_count > 0 && app.browser_selected < file_count - 1 {
                app.browser_selected += 1;
            }
        }
        KeyCode::Char('g') => {
            app.browser_selected = 0;
        }
        KeyCode::Char('G') => {
            if file_count > 0 { app.browser_selected = file_count - 1; }
        }
        KeyCode::Enter => {
            let entry_data = app
                .filtered_files()
                .get(app.browser_selected)
                .map(|e| (e.path.to_string_lossy().to_string(), e.name.clone()));

            if let Some((path, name)) = entry_data {
                match cmd_set(&path, None) {
                    Ok(_) => {
                        app.refresh_state()?;
                        app.set_message(format!("▶ Now playing: {}", name), false);
                    }
                    Err(e) => app.set_message(format!("Error: {}", e), true),
                }
            }
        }
        KeyCode::Char('a') => {
            let entry_data = app
                .filtered_files()
                .get(app.browser_selected)
                .map(|e| (e.path.to_string_lossy().to_string(), e.name.clone()));

            if let Some((path, name)) = entry_data {
                app.library.add(path);
                match app.library.save() {
                    Ok(_) => app.set_message(format!("+ Library: {}", name), false),
                    Err(e) => app.set_message(format!("Error saving library: {}", e), true),
                }
            }
        }
        KeyCode::Esc => {
            app.browser_filter.clear();
        }
        _ => {}
    }
    Ok(())
}

fn format_size(bytes: u64) -> String {
    const MB: u64 = 1024 * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB { format!("{:.1}G", bytes as f64 / GB as f64) }
    else if bytes >= MB { format!("{:.1}M", bytes as f64 / MB as f64) }
    else { format!("{:.1}K", bytes as f64 / 1024.0) }
}
