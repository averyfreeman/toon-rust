//! Terminal User Interface for TOON format conversion.
//!
//! Provides an interactive TUI with real-time conversion, REPL, and settings
//! panels.

/// Exposes the public `app` module.
pub mod app;
/// Exposes the public `components` module.
pub mod components;
/// Exposes the public `events` module.
pub mod events;
/// Exposes the public `keybindings` module.
pub mod keybindings;
/// Exposes the public `repl_command` module.
pub mod repl_command;
/// Exposes the public `state` module.
pub mod state;
/// Exposes the public `theme` module.
pub mod theme;
/// Exposes the public `ui` module.
pub mod ui;

use std::io;

use anyhow::Result;
/// Exposes the public `item` value.
pub use app::TuiApp;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// Initialize and run the TUI application.
///
/// Sets up terminal in raw mode, runs the app, then restores terminal state.
pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = TuiApp::new();
    let res = app.run(&mut terminal);

    // Always restore terminal, even on error
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}
