//! Shared, in-memory storage for notes.
//!
//! There is no database yet — notes live in a `Vec` behind a `Mutex` and are
//! lost on restart. Persisting them is the feature this series threads through
//! Parts 3–6.

use std::sync::{Arc, Mutex};

use crate::model::Note;

/// A cheaply-cloneable handle to the shared note store.
///
/// Every request handler receives a clone via axum's `State` extractor; the
/// clones all point at the same underlying `Vec`.
#[derive(Clone, Default)]
pub struct AppState {
    inner: Arc<Mutex<Vec<Note>>>,
}

impl AppState {
    /// Create an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return a snapshot of every note, in insertion order.
    pub fn all(&self) -> Vec<Note> {
        self.lock().clone()
    }

    /// Append a note with the given text and return it, with its new id.
    pub fn add(&self, text: String) -> Note {
        let mut notes = self.lock();
        let note = Note {
            id: notes.len() + 1,
            text,
        };
        notes.push(note.clone());
        note
    }

    /// Lock the store, recovering the guard even if a previous holder panicked.
    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<Note>> {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
