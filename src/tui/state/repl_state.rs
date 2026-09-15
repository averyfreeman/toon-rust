//! REPL state - separate from command mode

use std::collections::HashMap;

/// REPL session state
#[derive(Debug, Clone)]
/// Describes the public `ReplState` type.
pub struct ReplState {
    /// Whether REPL is active
    pub active: bool,
    /// Current input line
    pub input: String,
    /// Session history (output lines)
    pub output: Vec<ReplLine>,
    /// Variables stored in session
    pub variables: HashMap<String, String>,
    /// Command history
    pub history: Vec<String>,
    /// History index for navigation
    pub history_index: Option<usize>,
    /// Last result (for _ variable)
    pub last_result: Option<String>,
    /// Scroll offset for output
    pub scroll_offset: usize,
}

/// A line in the REPL output
#[derive(Debug, Clone)]
/// Describes the public `ReplLine` type.
pub struct ReplLine {
    pub kind: ReplLineKind,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
/// Describes the public `ReplLineKind` type.
pub enum ReplLineKind {
    Prompt,
    Success,
    Error,
    Info,
}

impl ReplState {
    /// Performs the public `new` operation.
    pub fn new() -> Self {
        Self {
            active: false,
            input: String::new(),
            output: vec![ReplLine {
                kind: ReplLineKind::Info,
                content: "TOON REPL - Type 'help' for commands, 'exit' to close".to_string(),
            }],
            variables: HashMap::new(),
            history: Vec::new(),
            history_index: None,
            last_result: None,
            scroll_offset: 0,
        }
    }

    /// Performs the public `activate` operation.
    pub fn activate(&mut self) {
        self.active = true;
        self.input.clear();
        self.history_index = None;
    }

    /// Performs the public `deactivate` operation.
    pub fn deactivate(&mut self) {
        self.active = false;
        self.input.clear();
        self.history_index = None;
    }

    /// Performs the public `add_prompt` operation.
    pub fn add_prompt(&mut self, cmd: &str) {
        self.output.push(ReplLine {
            kind: ReplLineKind::Prompt,
            content: format!("> {cmd}"),
        });
    }

    /// Performs the public `add_success` operation.
    pub fn add_success(&mut self, msg: String) {
        for line in msg.lines() {
            self.output.push(ReplLine {
                kind: ReplLineKind::Success,
                content: line.to_string(),
            });
        }
    }

    /// Performs the public `add_error` operation.
    pub fn add_error(&mut self, msg: String) {
        self.output.push(ReplLine {
            kind: ReplLineKind::Error,
            content: format!("✗ {msg}"),
        });
    }

    /// Performs the public `add_info` operation.
    pub fn add_info(&mut self, msg: String) {
        let content = if msg.is_empty() || msg.starts_with("  ") || msg.starts_with("📖") {
            msg
        } else {
            format!("✓ {msg}")
        };

        self.output.push(ReplLine {
            kind: ReplLineKind::Info,
            content,
        });
    }

    /// Performs the public `add_to_history` operation.
    pub fn add_to_history(&mut self, cmd: String) {
        if cmd.trim().is_empty() {
            return;
        }
        if self.history.last() == Some(&cmd) {
            return;
        }
        self.history.push(cmd);
        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }

    /// Performs the public `history_up` operation.
    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let new_index = match self.history_index {
            None => Some(self.history.len() - 1),
            Some(0) => Some(0),
            Some(i) => Some(i - 1),
        };
        if let Some(idx) = new_index {
            self.input = self.history[idx].clone();
            self.history_index = new_index;
        }
    }

    /// Performs the public `history_down` operation.
    pub fn history_down(&mut self) {
        match self.history_index {
            None => (),
            Some(i) if i >= self.history.len() - 1 => {
                self.input.clear();
                self.history_index = None;
            }
            Some(i) => {
                let new_idx = i + 1;
                self.input = self.history[new_idx].clone();
                self.history_index = Some(new_idx);
            }
        }
    }

    /// Performs the public `scroll_up` operation.
    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    /// Performs the public `scroll_down` operation.
    pub fn scroll_down(&mut self, visible_lines: usize) {
        let max_scroll = self.output.len().saturating_sub(visible_lines);
        if self.scroll_offset < max_scroll {
            self.scroll_offset += 1;
        }
    }

    /// Performs the public `scroll_to_bottom` operation.
    pub fn scroll_to_bottom(&mut self) {
        if self.output.len() <= 30 {
            self.scroll_offset = 0;
        } else {
            self.scroll_offset = self.output.len().saturating_sub(30);
        }
    }
}

impl Default for ReplState {
    fn default() -> Self {
        Self::new()
    }
}
