//! File management and conversion history.

use std::path::PathBuf;

use chrono::{DateTime, Local};

/// A file or directory entry.
#[derive(Debug, Clone)]
/// Describes the public `FileEntry` type.
pub struct FileEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<DateTime<Local>>,
}

impl FileEntry {
    /// Performs the public `name` operation.
    pub fn name(&self) -> String {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string()
    }

    /// Performs the public `is_json` operation.
    pub fn is_json(&self) -> bool {
        !self.is_dir && self.path.extension().and_then(|e| e.to_str()) == Some("json")
    }

    /// Performs the public `is_toon` operation.
    pub fn is_toon(&self) -> bool {
        !self.is_dir && self.path.extension().and_then(|e| e.to_str()) == Some("toon")
    }
}

/// Record of a conversion operation.
#[derive(Debug, Clone)]
/// Describes the public `ConversionHistory` type.
pub struct ConversionHistory {
    pub timestamp: DateTime<Local>,
    pub mode: String,
    pub input_file: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
    pub token_savings: f64,
    pub byte_savings: f64,
}

/// File browser and conversion history state.
pub struct FileState {
    pub current_file: Option<PathBuf>,
    pub current_dir: PathBuf,
    pub selected_files: Vec<PathBuf>,
    pub history: Vec<ConversionHistory>,
    pub is_modified: bool,
}

impl FileState {
    /// Performs the public `new` operation.
    pub fn new() -> Self {
        Self {
            current_file: None,
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            selected_files: Vec::new(),
            history: Vec::new(),
            is_modified: false,
        }
    }

    /// Performs the public `set_current_file` operation.
    pub fn set_current_file(&mut self, path: PathBuf) {
        self.current_file = Some(path.clone());
        self.current_dir = path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        self.is_modified = false;
    }

    /// Performs the public `clear_current_file` operation.
    pub fn clear_current_file(&mut self) {
        self.current_file = None;
        self.is_modified = false;
    }

    /// Performs the public `mark_modified` operation.
    pub fn mark_modified(&mut self) {
        self.is_modified = true;
    }

    /// Performs the public `add_to_history` operation.
    pub fn add_to_history(&mut self, entry: ConversionHistory) {
        self.history.push(entry);
        if self.history.len() > 50 {
            self.history.remove(0);
        }
    }

    /// Performs the public `toggle_file_selection` operation.
    pub fn toggle_file_selection(&mut self, path: PathBuf) {
        if let Some(pos) = self.selected_files.iter().position(|p| p == &path) {
            self.selected_files.remove(pos);
        } else {
            self.selected_files.push(path);
        }
    }

    /// Performs the public `clear_selection` operation.
    pub fn clear_selection(&mut self) {
        self.selected_files.clear();
    }

    /// Performs the public `is_selected` operation.
    pub fn is_selected(&self, path: &PathBuf) -> bool {
        self.selected_files.contains(path)
    }
}

impl Default for FileState {
    fn default() -> Self {
        Self::new()
    }
}
