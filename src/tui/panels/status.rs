use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
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
    let c = app.colors();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    if app.state.monitors.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No active wallpaper.",
            Style::default().fg(c.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            "  → Browse files and press Enter to set a wallpaper.",
            Style::default().fg(c.text_muted),
        )));
    } else {
        for (mon, entry) in &app.state.monitors {
            // Monitor header
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(
                    format!("MONITOR  {}", mon.to_uppercase()),
                    Style::default().fg(c.title).add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(Span::styled(
                format!("  {}", "─".repeat(36)),
                Style::default().fg(c.border_inactive),
            )));

            // Status
            let (status_color, status_label, status_icon) = match entry.pid {
                Some(pid) if is_pid_alive(pid) =>
                    (c.success, format!("RUNNING   PID {}", pid), "▶"),
                Some(pid) =>
                    (c.danger,  format!("STALE     PID {} (dead)", pid), "!"),
                None =>
                    (c.danger,  "STOPPED".to_string(), "■"),
            };
            lines.push(Line::from(vec![
                Span::styled("  STATUS     ", Style::default().fg(c.text_muted)),
                Span::styled(format!("{} {}", status_icon, status_label),
                    Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            ]));

            // File
            let file = if entry.wallpaper_path.is_empty() {
                "(none)".to_string()
            } else {
                std::path::Path::new(&entry.wallpaper_path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| entry.wallpaper_path.clone())
            };
            lines.push(Line::from(vec![
                Span::styled("  FILE       ", Style::default().fg(c.text_muted)),
                Span::styled(file, Style::default().fg(c.text_primary)),
            ]));

            // Autostart
            let (at_color, at_text) = if entry.autostart {
                (c.success, "ENABLED")
            } else {
                (c.text_muted, "DISABLED")
            };
            lines.push(Line::from(vec![
                Span::styled("  AUTOSTART  ", Style::default().fg(c.text_muted)),
                Span::styled(at_text, Style::default().fg(at_color).add_modifier(Modifier::BOLD)),
            ]));

            lines.push(Line::from(""));
        }
    }

    // Connected monitors section
    lines.push(Line::from(Span::styled(
        "  CONNECTED MONITORS",
        Style::default().fg(c.title).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!("  {}", "─".repeat(36)),
        Style::default().fg(c.border_inactive),
    )));
    if app.monitors.is_empty() {
        lines.push(Line::from(Span::styled("   None detected", Style::default().fg(c.text_muted))));
    } else {
        for mon in &app.monitors {
            let focused = if mon.focused {
                Span::styled(" [focused]", Style::default().fg(c.active_item))
            } else {
                Span::raw("")
            };
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {}   {}×{}", mon.name, mon.width, mon.height),
                    Style::default().fg(c.text_primary),
                ),
                focused,
            ]));
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Status ",
            Style::default().fg(c.title).add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(c.border_active));
    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, chunks[0]);

    let hints = Paragraph::new(" e: enable autostart  |  d: disable autostart  |  r: refresh")
        .style(Style::default().fg(c.text_muted))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(c.border_inactive)),
        );
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
                    app.set_message("Autostart disabled", false);
                }
                Err(e) => app.set_message(format!("Error: {}", e), true),
            }
        }
        KeyCode::Char('r') => {
            app.refresh_state()?;
            app.set_message("Status refreshed", false);
        }
        _ => {}
    }
    Ok(())
}
