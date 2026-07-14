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

impl NewNote {
    /// Validate the submitted body and return the text to store.
    ///
    /// A note has to carry something, so a body that is empty or nothing but
    /// whitespace is rejected rather than stored — otherwise the list fills up
    /// with blank rows. The text that survives is trimmed, so `"  hello  "` and
    /// `"hello"` are the same note.
    pub fn into_text(self) -> Result<String, Invalid> {
        let text = self.text.trim();
        if text.is_empty() {
            return Err(Invalid::EmptyText);
        }
        Ok(text.to_owned())
    }
}

/// Why a submitted note was rejected before it ever reached the store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invalid {
    /// The text was empty, or nothing but whitespace.
    EmptyText,
}

impl Invalid {
    /// The message sent back to the client.
    pub fn message(self) -> &'static str {
        match self {
            Self::EmptyText => "a note needs some text",
        }
    }
}

/// The JSON body returned with an error response.
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// A human-readable explanation of what went wrong.
    pub error: &'static str,
}
