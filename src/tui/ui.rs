use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
    Frame,
};

use super::app::{ActivePanel, App};
use super::panels;

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Root layout: tab bar (3) + content (fill) + status bar (1)
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    draw_tabs(f, app, root[0]);
    draw_panel(f, app, root[1]);
    draw_status_bar(f, app, root[2]);

    if app.show_help {
        draw_help_overlay(f, size);
    }
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = ActivePanel::all()
        .iter()
        .map(|p| Line::from(p.label()))
        .collect();

    let tabs = Tabs::new(titles)
        .select(app.active_panel.index())
        .block(Block::default().borders(Borders::ALL).title(" mpwall "))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(tabs, area);
}

fn draw_panel(f: &mut Frame, app: &mut App, area: Rect) {
    match app.active_panel {
        ActivePanel::Browser => panels::browser::draw(f, app, area),
        ActivePanel::Status => panels::status::draw(f, app, area),
        ActivePanel::Library => panels::library::draw(f, app, area),
        ActivePanel::Settings => panels::settings::draw(f, app, area),
    }
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (text, style) = match &app.message {
        Some(msg) if app.message_is_error => (
            msg.clone(),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Some(msg) => (
            msg.clone(),
            Style::default().fg(Color::Green),
        ),
        None => (
            " Tab: switch panel  |  q: quit  |  ?: help".to_string(),
            Style::default().fg(Color::DarkGray),
        ),
    };

    let bar = Paragraph::new(text).style(style);
    f.render_widget(bar, area);
}

fn draw_help_overlay(f: &mut Frame, area: Rect) {
    let width = 48u16.min(area.width.saturating_sub(4));
    let height = 18u16.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let popup = Rect::new(x, y, width, height);

    f.render_widget(Clear, popup);

    let help_text = vec![
        Line::from(vec![Span::styled(" Keybindings", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(" Tab / Shift+Tab   Switch panel"),
        Line::from(" ↑ ↓               Navigate list"),
        Line::from(" Enter             Set wallpaper / confirm"),
        Line::from(" /                 Filter (Browser panel)"),
        Line::from(" Esc               Cancel filter / close help"),
        Line::from(" a                 Add to Library"),
        Line::from(" d                 Remove from Library"),
        Line::from(" e                 Edit field (Settings)"),
        Line::from(" s                 Save settings"),
        Line::from(""),
        Line::from(" ?                 Toggle this help"),
        Line::from(" q                 Quit"),
        Line::from(""),
        Line::from(vec![Span::styled(" Press ? or Esc to close", Style::default().fg(Color::DarkGray))]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Help ");

    let para = Paragraph::new(help_text).block(block);
    f.render_widget(para, popup);
}

/// Helper: centered rect within a parent
#[allow(dead_code)]
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(layout[1])[1]
}
