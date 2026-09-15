//! Main application state.

use super::{EditorState, FileState, ReplState};
use crate::{
    tui::theme::Theme,
    types::{DecodeOptions, Delimiter, EncodeOptions, Indent, KeyFoldingMode, PathExpansionMode},
};

/// Conversion mode (encode/decode).
#[derive(Debug, Clone, Copy, PartialEq)]
/// Describes the public `Mode` type.
pub enum Mode {
    Encode,
    Decode,
}

impl Mode {
    /// Performs the public `toggle` operation.
    pub fn toggle(&self) -> Self {
        match self {
            Mode::Encode => Mode::Decode,
            Mode::Decode => Mode::Encode,
        }
    }

    /// Performs the public `as_str` operation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Encode => "Encode (JSON → TOON)",
            Mode::Decode => "Decode (TOON → JSON)",
        }
    }

    /// Performs the public `short_name` operation.
    pub fn short_name(&self) -> &'static str {
        match self {
            Mode::Encode => "Encode",
            Mode::Decode => "Decode",
        }
    }
}

/// Statistics from the last conversion.
#[derive(Debug, Clone)]
/// Describes the public `ConversionStats` type.
pub struct ConversionStats {
    pub json_tokens: usize,
    pub toon_tokens: usize,
    pub json_bytes: usize,
    pub toon_bytes: usize,
    pub token_savings: f64,
    pub byte_savings: f64,
}

/// Central application state containing all UI and conversion state.
pub struct AppState<'a> {
    pub mode: Mode,
    pub editor: EditorState<'a>,
    pub file_state: FileState,
    pub repl: ReplState,
    pub theme: Theme,
    pub encode_options: EncodeOptions,
    pub decode_options: DecodeOptions,
    pub show_settings: bool,
    pub show_help: bool,
    pub show_file_browser: bool,
    pub show_history: bool,
    pub show_diff: bool,
    pub error_message: Option<String>,
    pub status_message: Option<String>,
    pub stats: Option<ConversionStats>,
    pub should_quit: bool,
}

impl<'a> AppState<'a> {
    /// Performs the public `new` operation.
    pub fn new() -> Self {
        Self {
            mode: Mode::Encode,
            editor: EditorState::new(),
            file_state: FileState::new(),
            repl: ReplState::new(),
            theme: Theme::default(),

            encode_options: EncodeOptions::default(),
            decode_options: DecodeOptions::default(),

            show_settings: false,
            show_help: false,
            show_file_browser: false,
            show_history: false,
            show_diff: false,

            error_message: None,
            status_message: None,
            stats: None,

            should_quit: false,
        }
    }

    /// Performs the public `toggle_mode` operation.
    pub fn toggle_mode(&mut self) {
        self.mode = self.mode.toggle();
        self.clear_error();
        self.clear_status();
    }

    /// Performs the public `toggle_theme` operation.
    pub fn toggle_theme(&mut self) {
        self.theme = self.theme.toggle();
        self.set_status("Theme toggled".to_string());
    }

    /// Performs the public `set_error` operation.
    pub fn set_error(&mut self, msg: String) {
        self.error_message = Some(msg);
        self.status_message = None;
    }

    /// Performs the public `set_status` operation.
    pub fn set_status(&mut self, msg: String) {
        self.status_message = Some(msg);
        self.error_message = None;
    }

    /// Performs the public `clear_error` operation.
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// Performs the public `clear_status` operation.
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Performs the public `quit` operation.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Performs the public `toggle_settings` operation.
    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
        if self.show_settings {
            self.show_help = false;
            self.show_file_browser = false;
            self.show_history = false;
            self.show_diff = false;
        }
    }

    /// Performs the public `toggle_help` operation.
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        if self.show_help {
            self.show_settings = false;
            self.show_file_browser = false;
            self.show_history = false;
            self.show_diff = false;
        }
    }

    /// Performs the public `toggle_file_browser` operation.
    pub fn toggle_file_browser(&mut self) {
        self.show_file_browser = !self.show_file_browser;
        if self.show_file_browser {
            self.show_settings = false;
            self.show_help = false;
            self.show_history = false;
            self.show_diff = false;
        }
    }

    /// Performs the public `toggle_history` operation.
    pub fn toggle_history(&mut self) {
        self.show_history = !self.show_history;
        if self.show_history {
            self.show_settings = false;
            self.show_help = false;
            self.show_file_browser = false;
            self.show_diff = false;
        }
    }

    /// Performs the public `toggle_diff` operation.
    pub fn toggle_diff(&mut self) {
        self.show_diff = !self.show_diff;
        if self.show_diff {
            self.show_settings = false;
            self.show_help = false;
            self.show_file_browser = false;
            self.show_history = false;
        }
    }

    /// Performs the public `cycle_delimiter` operation.
    pub fn cycle_delimiter(&mut self) {
        self.encode_options =
            self.encode_options
                .clone()
                .with_delimiter(match self.encode_options.delimiter {
                    Delimiter::Comma => Delimiter::Tab,
                    Delimiter::Tab => Delimiter::Pipe,
                    Delimiter::Pipe => Delimiter::Comma,
                });
    }

    /// Performs the public `increase_indent` operation.
    pub fn increase_indent(&mut self) {
        let Indent::Spaces(current) = self.encode_options.indent;
        if current < 8 {
            self.encode_options = self
                .encode_options
                .clone()
                .with_indent(Indent::Spaces(current + 1));
        }
    }

    /// Performs the public `decrease_indent` operation.
    pub fn decrease_indent(&mut self) {
        let Indent::Spaces(current) = self.encode_options.indent;
        if current > 1 {
            self.encode_options = self
                .encode_options
                .clone()
                .with_indent(Indent::Spaces(current - 1));
        }
    }

    /// Performs the public `toggle_fold_keys` operation.
    pub fn toggle_fold_keys(&mut self) {
        self.encode_options =
            self.encode_options
                .clone()
                .with_key_folding(match self.encode_options.key_folding {
                    KeyFoldingMode::Off => KeyFoldingMode::Safe,
                    KeyFoldingMode::Safe => KeyFoldingMode::Off,
                });
    }

    /// Performs the public `increase_flatten_depth` operation.
    pub fn increase_flatten_depth(&mut self) {
        if self.encode_options.flatten_depth == usize::MAX {
            self.encode_options = self.encode_options.clone().with_flatten_depth(2);
        } else if self.encode_options.flatten_depth < 10 {
            self.encode_options = self
                .encode_options
                .clone()
                .with_flatten_depth(self.encode_options.flatten_depth + 1);
        }
    }

    /// Performs the public `decrease_flatten_depth` operation.
    pub fn decrease_flatten_depth(&mut self) {
        if self.encode_options.flatten_depth == 2 {
            self.encode_options = self.encode_options.clone().with_flatten_depth(usize::MAX);
        } else if self.encode_options.flatten_depth > 2
            && self.encode_options.flatten_depth != usize::MAX
        {
            self.encode_options = self
                .encode_options
                .clone()
                .with_flatten_depth(self.encode_options.flatten_depth - 1);
        }
    }

    /// Performs the public `toggle_flatten_depth` operation.
    pub fn toggle_flatten_depth(&mut self) {
        if self.encode_options.flatten_depth == usize::MAX {
            self.encode_options = self.encode_options.clone().with_flatten_depth(2);
        } else {
            self.encode_options = self.encode_options.clone().with_flatten_depth(usize::MAX);
        }
    }

    /// Performs the public `toggle_expand_paths` operation.
    pub fn toggle_expand_paths(&mut self) {
        self.decode_options =
            self.decode_options
                .clone()
                .with_expand_paths(match self.decode_options.expand_paths {
                    PathExpansionMode::Off => PathExpansionMode::Safe,
                    PathExpansionMode::Safe => PathExpansionMode::Off,
                });
    }

    /// Performs the public `toggle_strict` operation.
    pub fn toggle_strict(&mut self) {
        let strict = !self.decode_options.strict;
        self.decode_options = self.decode_options.clone().with_strict(strict);
    }

    /// Performs the public `toggle_coerce_types` operation.
    pub fn toggle_coerce_types(&mut self) {
        let coerce = !self.decode_options.coerce_types;
        self.decode_options = self.decode_options.clone().with_coerce_types(coerce);
    }
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        Self::new()
    }
}
