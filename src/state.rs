//! Shared storage for notes, backed by a JSON file on disk.
//!
//! Notes live in a `Vec` behind a `Mutex` for fast reads, and are mirrored to a
//! JSON file so they survive a restart. The file is the source of truth at
//! startup; the in-memory list is the source of truth while the service runs.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::model::Note;

/// A cheaply-cloneable handle to the shared note store.
///
/// Every request handler receives a clone via axum's `State` extractor; the
/// clones all point at the same underlying `Vec` and notes file.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Mutex<Vec<Note>>>,
    path: Arc<Option<PathBuf>>,
}

impl AppState {
    /// Create a store, optionally backed by a notes file at `path`.
    ///
    /// If the file exists and parses, its notes are loaded. If it is missing,
    /// the store starts empty. If it exists but cannot be read or parsed, the
    /// store starts empty and logs a warning rather than failing to start.
    /// Passing `None` gives a purely in-memory store (used in tests).
    pub fn new(path: Option<PathBuf>) -> Self {
        let notes = match path.as_deref() {
            Some(p) if p.exists() => Self::load(p).unwrap_or_else(|err| {
                eprintln!(
                    "warning: could not read notes file {}: {err}; starting empty",
                    p.display()
                );
                Vec::new()
            }),
            _ => Vec::new(),
        };
        Self {
            inner: Arc::new(Mutex::new(notes)),
            path: Arc::new(path),
        }
    }

    /// Return a snapshot of every note, in insertion order.
    pub fn all(&self) -> Vec<Note> {
        self.lock().clone()
    }

    /// Append a note and persist the store.
    ///
    /// Returns the created note (with its new id). If persisting fails, the
    /// in-memory list is rolled back so it never diverges from the file, and the
    /// error is returned.
    pub fn add(&self, text: String) -> Result<Note, String> {
        let mut notes = self.lock();
        let note = Note {
            id: notes.len() + 1,
            text,
        };
        notes.push(note.clone());
        if let Some(path) = self.path.as_ref() {
            if let Err(err) = Self::save(path, &notes) {
                notes.pop();
                return Err(err);
            }
        }
        Ok(note)
    }

    /// Read and parse the notes file.
    fn load(path: &Path) -> Result<Vec<Note>, String> {
        let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
        serde_json::from_slice(&bytes).map_err(|e| e.to_string())
    }

    /// Write the notes to the file as pretty JSON.
    fn save(path: &Path, notes: &[Note]) -> Result<(), String> {
        let json = serde_json::to_vec_pretty(notes).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }

    /// Lock the store, recovering the guard even if a previous holder panicked.
    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<Note>> {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
