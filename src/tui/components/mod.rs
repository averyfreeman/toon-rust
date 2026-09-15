//! UI components for the TUI.

/// Exposes the public `diff_viewer` module.
pub mod diff_viewer;
/// Exposes the public `editor` module.
pub mod editor;
/// Exposes the public `file_browser` module.
pub mod file_browser;
/// Exposes the public `help_screen` module.
pub mod help_screen;
/// Exposes the public `history_panel` module.
pub mod history_panel;
/// Exposes the public `repl_panel` module.
pub mod repl_panel;
/// Exposes the public `settings_panel` module.
pub mod settings_panel;
/// Exposes the public `stats_bar` module.
pub mod stats_bar;
/// Exposes the public `status_bar` module.
pub mod status_bar;

/// Exposes the public `item` value.
pub use diff_viewer::DiffViewer;
/// Exposes the public `item` value.
pub use editor::EditorComponent;
/// Exposes the public `item` value.
pub use file_browser::FileBrowser;
/// Exposes the public `item` value.
pub use help_screen::HelpScreen;
/// Exposes the public `item` value.
pub use history_panel::HistoryPanel;
/// Exposes the public `item` value.
pub use repl_panel::ReplPanel;
/// Exposes the public `item` value.
pub use settings_panel::SettingsPanel;
/// Exposes the public `item` value.
pub use stats_bar::StatsBar;
/// Exposes the public `item` value.
pub use status_bar::StatusBar;
