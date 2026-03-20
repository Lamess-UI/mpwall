use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

pub mod app;
pub mod panels;
pub mod ui;

use app::{App, ActivePanel};

const POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Entry point for the TUI. Sets up the terminal, runs the event loop, and restores the terminal on exit.
pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    // Always restore terminal even on error
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new()?;
    let mut last_poll = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Poll state every 2 seconds
        if last_poll.elapsed() >= POLL_INTERVAL {
            app.refresh_state()?;
            last_poll = Instant::now();
        }

        // Event timeout so polling still fires
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Global quit
                if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::NONE {
                    break;
                }
                // Ctrl-C also quits
                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                    break;
                }
                // Tab cycles panels
                if key.code == KeyCode::Tab {
                    app.next_panel();
                    continue;
                }
                // BackTab (Shift+Tab) goes back
                if key.code == KeyCode::BackTab {
                    app.prev_panel();
                    continue;
                }
                // Help overlay toggle
                if key.code == KeyCode::Char('?') {
                    app.show_help = !app.show_help;
                    continue;
                }
                // Delegate key to active panel
                if !app.show_help {
                    match app.active_panel {
                        ActivePanel::Browser => panels::browser::handle_key(&mut app, key)?,
                        ActivePanel::Status => panels::status::handle_key(&mut app, key)?,
                        ActivePanel::Library => panels::library::handle_key(&mut app, key)?,
                        ActivePanel::Settings => panels::settings::handle_key(&mut app, key)?,
                    }
                } else if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') {
                    app.show_help = false;
                }
            }
        }
    }
    Ok(())
}
