use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    cli::commands::{cmd_enable, cmd_disable},
    core::process::is_pid_alive,
    tui::app::App,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let mut lines: Vec<Line> = Vec::new();

    if app.state.monitors.is_empty() {
        lines.push(Line::from(Span::styled(
            " No active wallpaper. Run `mpwall set <file>` to get started.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (mon, entry) in &app.state.monitors {
            lines.push(Line::from(Span::styled(
                format!(" Monitor: {}", mon),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )));

            let (status_color, status_label) = match entry.pid {
                Some(pid) if is_pid_alive(pid) => (Color::Green, format!("running  (PID {})", pid)),
                Some(pid) => (Color::Red, format!("stopped  (stale PID {})", pid)),
                None => (Color::Red, "stopped".to_string()),
            };

            lines.push(Line::from(vec![
                Span::raw("   Status   : "),
                Span::styled(status_label, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            ]));

            let file = if entry.wallpaper_path.is_empty() {
                "(none)".to_string()
            } else {
                std::path::Path::new(&entry.wallpaper_path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| entry.wallpaper_path.clone())
            };

            lines.push(Line::from(format!("   File     : {}", file)));

            let autostart_color = if entry.autostart { Color::Yellow } else { Color::DarkGray };
            let autostart_text = if entry.autostart { "enabled" } else { "disabled" };
            lines.push(Line::from(vec![
                Span::raw("   Autostart: "),
                Span::styled(autostart_text, Style::default().fg(autostart_color)),
            ]));

            lines.push(Line::from(""));
        }
    }

    // Monitor list
    lines.push(Line::from(Span::styled(
        " Connected Monitors",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));
    if app.monitors.is_empty() {
        lines.push(Line::from(Span::styled("   (none detected)", Style::default().fg(Color::DarkGray))));
    } else {
        for mon in &app.monitors {
            lines.push(Line::from(format!(
                "   {} — {}x{}  {}",
                mon.name,
                mon.width,
                mon.height,
                if mon.focused { "[focused]" } else { "" }
            )));
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Status ")
        .border_style(Style::default().fg(Color::Cyan));
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, chunks[0]);

    // Hints bar
    let hints = Paragraph::new(" e: enable autostart  |  d: disable autostart")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(hints, chunks[1]);
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('e') => {
            match cmd_enable() {
                Ok(_) => {
                    app.refresh_state()?;
                    app.set_message("Autostart enabled", false);
                }
                Err(e) => app.set_message(format!("Error: {}", e), true),
            }
        }
        KeyCode::Char('d') => {
            match cmd_disable() {
                Ok(_) => {
                    app.refresh_state()?;
                    app.set_message("Autostart disabled and wallpaper stopped", false);
                }
                Err(e) => app.set_message(format!("Error: {}", e), true),
            }
        }
        _ => {}
    }
    Ok(())
}
