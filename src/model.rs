//! The data model for the notes API.

use serde::{Deserialize, Serialize};

/// A single note held by the service.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    /// Stable identifier, assigned in insertion order (1-based).
    pub id: usize,
    /// The note's text.
    pub text: String,
}

/// The request body accepted by `POST /api/notes`.
#[derive(Debug, Deserialize)]
pub struct NewNote {
    /// The text for the new note.
    pub text: String,
}
