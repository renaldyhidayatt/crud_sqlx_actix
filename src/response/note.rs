use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::NoteModel;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub createdAt: Option<DateTime<chrono::Utc>>,
    pub updatedAt: Option<DateTime<chrono::Utc>>,
}

impl From<NoteModel> for NoteResponse {
    fn from(note: NoteModel) -> Self {
        NoteResponse {
            id: note.id,
            title: note.title,
            content: note.content,
            createdAt: note.created_at,
            updatedAt: note.updated_at,
        }
    }
}
