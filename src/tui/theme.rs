//! Color themes for the TUI.

use ratatui::style::{Color, Modifier, Style};

/// Available color themes.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// Describes the public `Theme` type.
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    /// Switch between dark and light themes.
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    /// Performs the public `background` operation.
    pub fn background(&self) -> Color {
        match self {
            Theme::Dark => Color::Black,
            Theme::Light => Color::White,
        }
    }

    /// Performs the public `foreground` operation.
    pub fn foreground(&self) -> Color {
        match self {
            Theme::Dark => Color::White,
            Theme::Light => Color::Black,
        }
    }

    /// Performs the public `border` operation.
    pub fn border(&self) -> Color {
        match self {
            Theme::Dark => Color::Cyan,
            Theme::Light => Color::Blue,
        }
    }

    /// Performs the public `border_active` operation.
    pub fn border_active(&self) -> Color {
        match self {
            Theme::Dark => Color::Green,
            Theme::Light => Color::Green,
        }
    }

    /// Performs the public `title` operation.
    pub fn title(&self) -> Color {
        match self {
            Theme::Dark => Color::Yellow,
            Theme::Light => Color::Blue,
        }
    }

    /// Performs the public `success` operation.
    pub fn success(&self) -> Color {
        Color::Green
    }

    /// Performs the public `error` operation.
    pub fn error(&self) -> Color {
        Color::Red
    }

    /// Performs the public `warning` operation.
    pub fn warning(&self) -> Color {
        Color::Yellow
    }

    /// Performs the public `info` operation.
    pub fn info(&self) -> Color {
        Color::Cyan
    }

    /// Performs the public `highlight` operation.
    pub fn highlight(&self) -> Color {
        match self {
            Theme::Dark => Color::Blue,
            Theme::Light => Color::LightBlue,
        }
    }

    /// Performs the public `selection` operation.
    pub fn selection(&self) -> Color {
        match self {
            Theme::Dark => Color::DarkGray,
            Theme::Light => Color::LightYellow,
        }
    }

    /// Performs the public `line_number` operation.
    pub fn line_number(&self) -> Color {
        match self {
            Theme::Dark => Color::DarkGray,
            Theme::Light => Color::Gray,
        }
    }

    /// Performs the public `normal_style` operation.
    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.foreground()).bg(self.background())
    }

    /// Get border style, highlighted if active.
    pub fn border_style(&self, active: bool) -> Style {
        Style::default().fg(if active {
            self.border_active()
        } else {
            self.border()
        })
    }

    /// Performs the public `title_style` operation.
    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.title())
            .add_modifier(Modifier::BOLD)
    }

    /// Performs the public `highlight_style` operation.
    pub fn highlight_style(&self) -> Style {
        Style::default().fg(self.foreground()).bg(self.highlight())
    }

    /// Performs the public `selection_style` operation.
    pub fn selection_style(&self) -> Style {
        Style::default()
            .fg(self.foreground())
            .bg(self.selection())
            .add_modifier(Modifier::BOLD)
    }

    /// Performs the public `error_style` operation.
    pub fn error_style(&self) -> Style {
        Style::default()
            .fg(self.error())
            .add_modifier(Modifier::BOLD)
    }

    /// Performs the public `success_style` operation.
    pub fn success_style(&self) -> Style {
        Style::default()
            .fg(self.success())
            .add_modifier(Modifier::BOLD)
    }

    /// Performs the public `warning_style` operation.
    pub fn warning_style(&self) -> Style {
        Style::default()
            .fg(self.warning())
            .add_modifier(Modifier::BOLD)
    }

    /// Performs the public `info_style` operation.
    pub fn info_style(&self) -> Style {
        Style::default().fg(self.info())
    }

    /// Performs the public `line_number_style` operation.
    pub fn line_number_style(&self) -> Style {
        Style::default().fg(self.line_number())
    }
}
